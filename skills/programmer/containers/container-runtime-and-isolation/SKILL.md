---
name: container_runtime_and_isolation.md
description: Use when the agent is hardening container runtime isolation, choosing between runc, gVisor, Kata Containers, or Firecracker microVMs for a workload, deciding whether stronger isolation than a standard container is warranted, dropping Linux capabilities, writing or selecting a seccomp profile, applying AppArmor or SELinux, evaluating the shared-kernel threat model, running untrusted or third-party code in containers, reviewing a security context, or auditing why privileged mode is in use. Use when the agent is configuring pod security, reasoning about namespaces and cgroups as isolation primitives, or deciding whether a container is a sufficient security boundary for multi-tenant, regulated, or hostile workloads. Risk surfaces include kernel CVEs reachable from inside a container, container escape via privileged mode or excess capabilities, noisy-neighbor resource exhaustion, and the false assumption that containers isolate as strongly as virtual machines.
---

# Container Runtime And Isolation

A container is not a virtual machine, and the difference is the entire threat model. A VM virtualizes hardware: each guest runs its own kernel, and compromise of a guest process does not directly yield access to the hypervisor or other guests. A container shares the host kernel with every other container on the node. Its isolation is built from two Linux kernel facilities — *namespaces*, which make a process see its own private view of resources (mounts, process IDs, network, users, IPC, UTS), and *cgroups*, which limit how much CPU, memory, and I/O a process may consume. Together they produce *soft* isolation: strong enough to keep well-behaved processes apart, but fundamentally dependent on the correctness of one shared kernel and on the runtime configuration that mediates access to it.

Agents tend to treat "it runs in a container" as if it implied a security guarantee equivalent to a VM, and then act on that assumption by running untrusted code, granting privileged mode to "make it work," or leaving default capabilities in place. Each of these collapses the isolation back toward zero: privileged mode removes nearly every boundary the kernel offers; an excess capability like `CAP_SYS_ADMIN` or `CAP_SYS_PTRACE` is an escape primitive; and a kernel CVE reachable from inside the container's syscall surface is a single bug away from every container on the node. The judgment problem is to size the isolation to the actual threat — who can put code into the container, what they gain if they escape, and what else shares that kernel — and to treat namespaces and cgroups as the *floor* of isolation, not the ceiling.

This skill covers the runtime-isolation decision and the hardening controls layered on top. It complements `image-security-and-scanning`, which covers the *image* program (scanning, signing, secrets) and the runtime security context at a high level; here the focus is the *runtime isolation model* — which runtime to choose, how far the kernel boundary holds, and when to step up to stronger isolation. The Dockerfile-level non-root user and the graceful-shutdown behavior belong to `dockerfile-best-practices` and `orchestration-and-scaling`.

## Core Rules

### Understand The Shared-Kernel Threat Model Before Trusting Isolation

The default container (runc/containerd) isolates via namespaces and cgroups, but every container on a node runs syscalls against the *same* host kernel. This means two things an agent must internalize before sizing any control. First, a kernel vulnerability reachable from inside a container's syscall surface is a potential escape to the host and to every co-resident container — the kernel is the trusted computing base, and its CVEs are shared across the whole node. Second, namespaces hide resources from a process's *view* but do not, by themselves, prevent a privileged process from reaching them; the enforcement comes from capabilities, seccomp, and MAC policies layered on top.

Size the threat by three questions: *Who controls the code inside the container?* (your own reviewed service vs. a third-party plugin vs. an arbitrary user's submitted code); *What does an attacker gain by escaping?* (access to one disposable node vs. a path to secrets, other tenants, or the control plane); and *What else shares this kernel?* (nothing sensitive vs. other tenants vs. the cluster's system components). The answers decide whether default runc isolation is adequate or whether a stronger boundary is required. A single-tenant cluster running only your own reviewed services is a different problem from a multi-tenant platform running customer-supplied code, and the two must not be configured identically.

### Drop Capabilities Aggressively And Treat Privileged Mode As A Defect

Linux capabilities partition the powers historically bundled into root. A container starts with a small default set that still includes several dangerous ones (`CAP_NET_RAW` for raw socket crafting, `CAP_SYS_PTRACE` for inspecting other processes, `CAP_DAC_READ_SEARCH` for bypassing file permissions). The correct default is to drop *all* capabilities and add back only the specific ones the workload demonstrably needs — and most services need none. A workload that "needs" `CAP_SYS_ADMIN` almost always needs a narrower fix (a bind to a low port via `sysctl net.ipv4.ip_unprivileged_port_start`, a `SYS_TIME` grant for NTP, a volume mount instead of raw block access) rather than the blanket grant.

Privileged mode (`privileged: true`, or `docker run --privileged`) is the strongest red flag in a container configuration: it disables namespace isolation, grants every capability, disables seccomp and AppArmor/SELinux, and makes the container effectively the host. It exists for debugging and a narrow set of container-in-container or hardware-access cases, not for making an application run. When you encounter it in a manifest, treat it as a defect to be justified or removed: identify the underlying need (binding a port, mounting a device, running Docker-in-Docker in CI), and satisfy that need with the least-privilege mechanism. A CI runner that needs to build images should use a rootless or Kanigma-style setup, not a privileged pod in production.

### Add Seccomp And MAC Profiles To Restrict The Syscall Surface

Capabilities limit *powers*; seccomp limits *which syscalls* a process may issue at all. The default Docker seccomp profile already blocks dozens of historically-abused syscalls (kernel module loading, certain `keyctl` and `bpf` operations, `pivot_root` in unsafe contexts), and applying it (it is on by default in Docker and containerd, and selectable in Kubernetes) is a meaningful baseline. For higher-value workloads, a custom profile generated by tracing the application's actual syscalls (e.g., via `oci-seccomp-bpf-hook` or running under `strace`/`bpftrace`) restricts the surface to only what the app uses, turning a broad attack surface into a narrow allow-list. Verify the profile is actually applied — a seccomp annotation that the runtime ignores is theater.

Mandatory Access Control (AppArmor on Debian/Ubuntu, SELinux on RHEL-family) confines what a process may do *even after* a bug grants it unexpected privileges: which files it can read or execute, which capabilities it can exercise, which network operations are permitted. The runtime's default profile is a reasonable baseline; high-value services warrant a custom profile that denies everything except the app's real access pattern. These MAC systems are complementary to seccomp and to dropped capabilities — each covers a different escape or escalation path, and the combination is what makes "the app process was compromised" a contained event rather than a path to the host.

### Know When To Step Up To Stronger Isolation Than A Standard Container

For some workloads, the shared-kernel model is not an acceptable boundary regardless of how well it is hardened. Recognize these cases and choose a runtime that provides a real additional boundary. The common options, in increasing order of isolation (and overhead):

- **gVisor** (runsc) — implements a userspace "guest kernel" that intercepts the container's syscalls and re-implements them against a restricted surface, so most syscalls never reach the host kernel directly. Strong against kernel-syscall-based escapes; adds latency on syscall-heavy workloads. Good default for untrusted-but-similar-architecture workloads.
- **Kata Containers** — runs each container (or pod) inside a lightweight VM using a standard hypervisor (QEMU, cloud-hypervisor), so the workload gets its own guest kernel while remaining OCI-compatible. Strong isolation with near-native compatibility; the right choice when you need VM-grade boundaries for workloads that still want the container ergonomics.
- **Firecracker** (microVM) — a purpose-built minimal VMM (originally from AWS Lambda/Fargate) that boots a tiny VM in milliseconds for high-density, strongly-isolated workloads. Used when you spin up large numbers of short-lived, mutually-untrusted sandboxes (serverless, code-execution platforms).

The decision is a tradeoff between isolation strength and overhead (boot time, memory, syscall latency), and it should be driven by the threat: your own reviewed services on a dedicated node rarely need more than hardened runc; multi-tenant or user-supplied code usually does. Do not adopt a heavier runtime for performance or fashion — adopt it because the threat model demands a boundary the shared kernel cannot provide.

### Enforce Resource Limits So A Noisy Or Hostile Neighbor Cannot Exhaust The Node

Cgroups are the other half of isolation: they bound how much CPU, memory, and I/O a container may consume. Without limits, a single container that leaks memory, loops on CPU, or saturates disk I/O can starve every co-resident workload and even destabilize the node — a denial of service that is just as real as a security escape. Set memory limits (`resources.limits.memory` and the lower `requests.memory`), CPU limits or quotas, and where relevant ephemeral-storage and I/O limits, sized to the workload's real needs with headroom.

The trap is setting limits without requests (causing scheduling problems) or setting them so tight that the workload is OOM-killed under normal load. Measure the real steady-state and peak consumption first, then set limits with a margin, and set requests high enough that the scheduler places the pod on a node with real capacity. Pair limits with a graceful-shutdown and eviction policy so that pressure leads to controlled degradation, not a cascade of kills. Resource limits are isolation against accidental and malicious exhaustion, and omitting them leaves the node's availability dependent on every container's good behavior.

## Common Traps

### Assuming Containers Isolate As Strongly As VMs

Treating "containerized" as equivalent to "isolated like a VM" and then running untrusted or multi-tenant code on a shared kernel. The shared kernel means a reachable kernel CVE is a potential escape to the host and all co-resident containers. Size the isolation to the threat; use gVisor/Kata/Firecracker when the workload is untrusted.

### Granting Privileged Mode To "Make It Work"

Reaching for `privileged: true` when a container fails to bind a port, mount a device, or run Docker-in-Docker, instead of identifying the minimal grant. Privileged mode disables nearly every isolation boundary and is effectively host access. Replace it with the specific capability, sysctl, or volume mount the workload actually needs.

### Leaving Default Capabilities In Place

Shipping with the default capability set, which includes `CAP_NET_RAW`, `CAP_SYS_PTRACE`, and others that few services need and that are direct escalation primitives. Drop ALL and add back only the verified minimum; most services need none.

### A Seccomp Or MAC Profile That Is Not Actually Applied

Annotating a seccomp profile or AppArmor profile in the manifest but running on a runtime or node configuration that ignores it, so the workload has no syscall or MAC restriction while the team believes it does. Verify the profile is enforced (check the process's actual seccomp mode and the loaded AppArmor/SELinux status), not just declared.

### No Resource Limits, Enabling Noisy-Neighbor Exhaustion

Running containers without memory, CPU, and storage limits, so one leaking or hostile container can OOM-kill neighbors or saturate the node. Cgroup limits are isolation against resource exhaustion; set them with measured headroom, and pair with requests so scheduling reflects real capacity.

### Choosing A Heavier Runtime Without A Threat Justification

Adopting gVisor or Kata for performance, trendiness, or "extra safety" on workloads that are your own reviewed code on a dedicated node, paying latency and operational overhead for isolation the threat model does not require. Step up to a stronger runtime only when the workload is untrusted or multi-tenant and the shared kernel is an unacceptable boundary.

### Trusting The Image's Non-Root User As Sufficient Isolation

Conflating "runs as non-root" with "isolation is complete." A non-root process can still exercise granted capabilities, reach the network, and exploit a kernel bug. Non-root is necessary and is the floor; pair it with dropped capabilities, seccomp, MAC, read-only rootfs, and resource limits for a real boundary.

## Self-Check

- [ ] The shared-kernel threat model has been explicitly considered: who controls the code in the container, what an escape yields, and what else shares the kernel, and the isolation is sized to those answers rather than assumed sufficient.
- [ ] No container runs in privileged mode; any historical use of `privileged: true` has been replaced by the minimal capability, sysctl, or volume mount that satisfies the underlying need.
- [ ] Capabilities are dropped to ALL and only the specific capabilities the workload demonstrably requires are added back; the default capability set (including `CAP_NET_RAW`, `CAP_SYS_PTRACE`) is not in use.
- [ ] A seccomp profile (default or custom-generated from the app's real syscalls) is applied and *verified enforced* on the runtime, not merely annotated; an AppArmor or SELinux profile confines the process beyond capabilities.
- [ ] For untrusted, third-party, or multi-tenant workloads, a stronger runtime (gVisor, Kata Containers, or Firecracker microVMs) is selected where the shared kernel is an unacceptable boundary, and the choice is justified by the threat model rather than adopted for its own sake.
- [ ] Every container has memory, CPU, and (where relevant) ephemeral-storage and I/O limits set with measured headroom, paired with resource requests high enough for correct scheduling, so a leaking or hostile container cannot exhaust the node.
- [ ] The runtime reads only the root filesystem it needs (read-only rootfs with explicit writable volumes) and sets `no-new-privileges`, so the process cannot escalate via setuid or other paths even if compromised.
- [ ] No workload's isolation is being trusted solely on "it runs as non-root"; non-root is paired with the layered controls above, and the combination is what is claimed as the boundary.
- [ ] If kernel CVEs are a concern for co-resident workloads, the node's kernel is kept current and the exposure of the syscall surface is minimized (seccomp), recognizing that the kernel is the shared trusted computing base for standard containers.
