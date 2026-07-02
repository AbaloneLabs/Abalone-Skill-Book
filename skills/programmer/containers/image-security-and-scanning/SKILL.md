---
name: image_security_and_scanning.md
description: Use when the agent is building a container image security program, deciding when and how to scan images (build, push, periodic), choosing between Trivy, Grype, and Snyk, triaging CVEs by severity versus CVSS versus EPSS versus exploitability, defining a rebuild and base-image patching cadence, setting up image signing with cosign and Sigstore, generating SBOMs and in-toto provenance attestations, enforcing admission policy on signed images, hardening runtime with read-only rootfs, dropped capabilities, seccomp, AppArmor, SELinux, and no-new-privileges, injecting secrets at runtime rather than baking them in, adding runtime threat detection with Falco, or mapping image security to CIS, PCI-DSS, or HIPAA controls. Use when a scan is failing the pipeline, when a signed image is being rejected at admission, when hardening a deployment's security context, or when an audit demands evidence of vulnerability and supply-chain controls. The base scanning-and-signing philosophy is in container-image-design; this skill is the full security program around it.
---

# Image Security And Scanning

A container image is the most concentrated security artifact most teams ship. It bundles an operating system, language runtimes, hundreds of third-party libraries, and your own code into one immutable unit that is replicated to registries, pulled by CI and developers, and run as dozens or hundreds of identical replicas — each one a copy of every vulnerability and every misconfiguration baked in at build time. An image security program is the system that makes that artifact knowable (you know what is in it and what is wrong with it), verifiable (you can prove where it came from), constrained (it cannot do more at runtime than it must), and monitored (you can tell when it misbehaves). Without that program, "we run containers" is a statement of unknown risk repeated across the fleet.

Agents tend to reduce image security to a single act — turn on a scanner, block on critical CVEs — and treat the rest as future work. That single act produces false confidence: a scanner that runs once at build but never again misses the CVEs published next week against the unchanged image already in production; a "block on critical" policy that paralyzes releases on unexploitable findings while ignoring a high-severity bug on the actual request path; a signed image that nobody verifies at admission, so the signature is decoration. The judgment problem is treating image security as a layered program — scanning with triage, a current base, signing with enforced verification, least-privilege runtime, runtime secrets, runtime detection, and compliance evidence — and recognizing that each layer covers a failure mode the others miss.

This skill is the full security program. It overlaps with `container-image-design`, which covers scanning and signing as part of image philosophy; here those topics go deeper into program design (scan cadence, triage by exploitability, attestation, admission enforcement) and are joined by runtime hardening, secret injection, runtime detection, and compliance mapping. The Dockerfile-level patterns (non-root user, multi-stage, layer hygiene) are in `dockerfile-best-practices`; the cluster scaling and graceful-shutdown behavior is in `orchestration-and-scaling`.

## Core Rules

### Scan At Build, On Push, And Periodically Once Deployed

A single scan at build time is necessary but not sufficient, because vulnerabilities are discovered against *unchanged* images. An image scanned clean on Monday can be full of criticals by Friday without anything in your pipeline changing. Scan at three points:

- **At build (developer/CI).** Fast feedback before an image is promoted. Catches new deps and base-image regressions before they leave the developer's machine or the PR.
- **On push to the registry / at promotion.** The gate that decides whether an image may enter a trusted registry or be promoted to staging/prod. This is where policy is enforced (block on untriaged criticals, require SBOM, require signature).
- **Periodically, against images already deployed.** Re-scan the registry on a schedule (daily) and alert on newly-published CVEs affecting deployed images. This is the control that catches the "clean at build, vulnerable next month" gap, and it is the one most teams omit.

Each point has a different audience and policy: build is fast and developer-facing, push is the gate, periodic is surveillance of the running fleet. All three are required for a real program; build-only is theater that feels like coverage.

### Choose The Scanner By Coverage And Ecosystem Fit, Not Habit

Trivy, Grype (Anchore), and Snyk are the common choices, and they differ in ways that matter:

- **Trivy** — broad coverage (OS packages, language deps, IaC, secrets, misconfigurations), fast, good for CI gates and registry scanning. Strong general-purpose default.
- **Grype** — focused vulnerability scanning against SBOMs, integrates well with Syft-generated SBOMs and the Anchore/supply-chain tooling. Strong when your program is SBOM-centric.
- **Snyk / commercial scanners** — developer-facing prioritization, fix-version guidance, deeper language-ecosystem databases, and often better triage UX at the cost of licensing. Strong when triage volume is high and you want fix recommendations.

The choice should be driven by your stack (does it cover your language ecosystems and base distros?), your workflow (CI gate vs. registry scan vs. developer portal), and whether you need SBOM-driven or direct-scan workflows. Do not pick by familiarity and then discover it does not cover Alpine's advisory feed or your private registry. Run more than one where coverage gaps matter (e.g., Trivy for OS + Snyk for language deps), accepting the duplicate-finding cost.

### Triage By Exploitability, Not By Severity Alone

A scanner reports CVEs with a severity (Critical/High/Medium/Low) usually derived from CVSS base score. Severity is a poor sole sorting key, because CVSS measures *theoretical* exploitability of the vulnerability in isolation, not whether *your* application can actually reach the vulnerable code. A "Critical" CVE in a build-time tool that never runs in production, or in a library loaded only on a code path your app never executes, is a lower real risk than a "High" CVE on your hot request path. Triage by:

- **Reachability / exploitability.** Is the vulnerable function on a path an attacker can reach? Tools and SBOM-plus-call-graph analysis can estimate this; manual judgment applies for the rest. A CVE in an installed-but-unreachable package is documentation, not an incident.
- **EPSS (Exploit Prediction Scoring System).** The probability that a CVE is actually being exploited in the wild in the near term. EPSS often separates "Critical in theory" from "being wormed right now," and is a far better prioritization signal than CVSS alone for what to fix first.
- **Whether a fix exists.** A critical with an available patch and a critical with no upstream fix are very different operational problems. The former is a rebuild; the latter is a risk-acceptance or a compensating control.
- **The component's role.** A CVE in `openssl` on the TLS path is existential; the same severity in a test-only dependency is noise.

Define a policy that blocks releases on exploitable, high-EPSS, fixable criticals on reachable paths, and tracks (rather than blocks on) theoretical or unreachable findings. A policy that blocks on every critical — including unexploitable ones — paralyzes releases and trains the team to ignore the scanner. A policy that ignores everything leaves real risk unaddressed. The goal is a known, declining, triaged surface, not zero findings.

### Keep The Base Image Current Rather Than Chasing Individual CVEs

The durable fix for the long tail of base-image CVEs is not patching them one at a time; it is rebuilding regularly against a current, patched base. Distributions publish updated base images frequently (Debian point releases, Alpine revisions, distroless rebuilds), and a current base absorbs hundreds of CVEs at once. A program that rebuilds weekly against the latest base and pins by digest stays far ahead of one that hand-patches individual CVEs.

Balance two pressures:

- **Stay current enough to absorb patches** — rebuild on a cadence (weekly is common), and on the publication of a critical that affects you. Automation (Dependabot/Renovate for base images, scheduled rebuild pipelines) makes this cheap.
- **Do not chase every CVE the day it drops.** A rebuild cadence plus triage handles the long tail; reserve immediate action for high-EPSS, reachable criticals. Chasing every advisory burns the team and destabilizes the build.

The signal that the program is working is that the deployed fleet's average CVE age and count trend down over time, not that any single image is ever at zero.

### Sign Images And Generate SBOMs And Attestations As Part Of The Build

Signing proves the image came from your build system; an SBOM records what is in it; a provenance attestation records how and from what it was built. Together they let a deployment verify not just "is this image authentic" but "does this image correspond to reviewed source, built on our trusted pipeline, from these pinned bases." Build the program with Sigstore/cosign as the common stack:

- **Sign every promoted image** with a key (or keyless, tied to the build's OIDC identity). The signature is what admission checks.
- **Generate an SBOM** (Syft, or the build tool's native SBOM) and attach it as an attestation. The SBOM is the inventory you query when a new CVE drops ("which of our images contain log4j 2.14?") and the evidence you hand to compliance.
- **Attest provenance** (SLSA/in-toto) recording the source commit, the base images and their digests, the build parameters, and the builder identity. This is what lets you trust that the running artifact came from reviewed source and not a substituted build.

The attestations are only valuable if they are *verified*; generating them without enforcement is documentation, not a control. Pair generation with admission enforcement (next rule).

### Enforce Signatures And Attestations At Admission, Not Just At The Registry

A signed image that nothing verifies is decoration. The control that makes signing meaningful is admission policy — the cluster (or runtime) refuses to start an image that is not signed by the trusted identity, or that lacks a required attestation. Enforce at the point of deployment:

- **Policy controllers** (cosign policy, Kyverno, OPA/Gatekeeper, Sigstore Policy Controller) evaluate each image pull against signature and attestation requirements and reject non-conforming images before they run.
- **Require the provenance attestation**, not just the signature, when you want to assert the image came from the trusted pipeline (not merely that *someone* signed it).
- **Scope policy by environment.** Prod may require signed + attested + from the prod registry; dev may be looser. The policy is where "trusted" is defined concretely.

This is the container equivalent of verifying package signatures on dependencies: the build produces trust evidence, and the runtime consumes it. Without the consumption side, the production side is busywork.

### Harden The Runtime Beyond Non-Root

Running as non-root is necessary but not sufficient. A non-root process can still bind privileged ports it was granted, reach the network it should not, write to the filesystem it should not, and escalate through a kernel or runtime bug. Layer additional least-privilege controls in the security context / runtime profile:

- **Read-only root filesystem.** The container's rootfs is mounted read-only; the process writes only to explicitly mounted writable volumes (tmp, cache, data). This stops a wide class of persistence and tampering.
- **Drop ALL capabilities, add back only what is needed.** Default capabilities include several (`CAP_NET_RAW`, `CAP_SYS_PTRACE`, etc.) that few services need and that are escalation primitives. `drop: ALL` then add the one or two required.
- **`no-new-privileges`.** Prevents the process from gaining more privileges via setuid binaries or other escalation paths. Cheap and high-value.
- **Seccomp profile.** Default Docker seccomp already blocks many syscalls; a custom profile can restrict further to only the syscalls the app uses. Apply when the default is not strict enough.
- **AppArmor / SELinux.** MAC policies that confine what the process can do even after compromise. Use the runtime's default profile at minimum; custom profiles for high-value services.

Each control covers a different escape or escalation path; together they make "the app process was compromised" a contained event rather than a path to the host. The Dockerfile establishes that the app does not *need* root; these controls ensure it *cannot* escalate even if a bug exists.

### Inject Secrets At Runtime, Never Bake Them In

A secret baked into an image is a security incident: the image is replicated, pulled by developers and CI, retained in registries long after rotation, and recoverable by anyone with read access including from intermediate layers. The rule is absolute for production secrets — they are injected at runtime, not embedded at build:

- **Mounted secret stores.** Kubernetes Secrets mounted as files, cloud secret managers (AWS/GCP/Azure), HashiCorp Vault. The application reads the secret from a file path or an env var populated at runtime.
- **External Secrets Operator / CSI secrets store.** Sync secrets from a real secrets manager into the cluster as mountable artifacts, so the source of truth is the manager and rotation does not require a rebuild.
- **Build-time secrets use BuildKit secret mounts** (`--mount=type=secret`), which make a secret available to a single `RUN` without writing it to any layer. This is the correct pattern for a token needed only to fetch private deps during build.

If a secret was ever baked in (via `ARG`, `ENV`, a copied file, even one later removed), treat it as compromised and rotate it — it is in the layer history and recoverable. `container-image-design` states the philosophy; the operational discipline is that production secrets live only in runtime-injected stores.

### Add Runtime Threat Detection For Behavior You Cannot Predict At Build

Static controls (scanning, signing, hardening) catch what you can enumerate at build time. Runtime detection catches what you cannot — the process that suddenly opens a reverse shell, reads `/etc/shadow`, makes an unexpected outbound connection, or spawns a tool not in the image. Tools like Falco (rules-based on syscalls and container events), eBPF-based detection, and cloud runtime security products watch the running container for behavior that deviates from the expected:

- **Start with the rule packs** (Falco's default rules) and tune to your environment to cut noise.
- **Wire detections into alerting and (selectively) response** — an alert no one sees is useless; an automated isolation of a confirmed-compromised container is the goal for high-value services.
- **Correlate with the image identity** so a runtime finding points back to the specific image, version, and SBOM, closing the loop between detection and the build that produced it.

Runtime detection is the layer that catches the zero-day and the compromised insider that static controls cannot. It is not a replacement for them; it is the assumption that static controls will, eventually, be bypassed.

### Map Controls To The Compliance Frameworks That Apply To You

For regulated environments, image security is also an evidence problem: an auditor asks "show me you scan, you patch, you restrict access, you can trace what ran." Map your controls to the framework's requirements so the program produces evidence as a byproduct:

- **CIS Kubernetes / Docker Bench** — the baseline hardening benchmarks. Run the benchmarks and remediate; they are the most cited baseline.
- **PCI-DSS** — vulnerability scanning, patching cadence, least-privilege access, change control. Your scan cadence, rebuild cadence, and admission policy map directly.
- **HIPAA / FedRAMP / ISO 27001 / SOC 2** — access controls, change management, monitoring, incident response. The SBOMs, signing logs, scan reports, and runtime detections are the evidence.

The discipline is to build the program once and let it satisfy the frameworks, rather than running a separate "compliance scan" for each audit. If your scan reports, SBOMs, admission decisions, and runtime alerts are retained and queryable, most evidence requests are a query, not a project.

## Common Traps

### Build-Time-Only Scanning

Scanning at build and never again, so an image that was clean on Monday carries a critical published Tuesday for weeks in production. Add periodic re-scanning of the registry and deployed fleet; the "clean at build, vulnerable later" gap is the most common hole.

### Blocking On Every Critical, Including Unexploitable Ones

A gate that fails the build on any Critical CVSS, including CVEs in unreachable packages or build-only tools, paralyzes releases until the team either disables the gate or learns to ignore it. Triage by exploitability and EPSS; block on reachable, high-EPSS, fixable criticals and track the rest.

### Signing Without Admission Enforcement

Generating cosign signatures and SBOM attestations but configuring nothing to verify them, so a substituted or malicious image runs unhindered. The signature is a control only when admission policy rejects unsigned or unattested images.

### Chasing Individual CVEs Instead Of Rebuilding The Base

Hand-patching CVEs one at a time while the base image drifts months behind, accumulating a backlog. The durable fix is a regular rebuild cadence against a current base; reserve individual action for high-EPSS reachable criticals.

### Trusting CVSS And Ignoring EPSS

Prioritizing by CVSS base score alone, so a theoretically-critical but never-exploited CVE jumps the queue over a high-severity bug being actively wormed. Use EPSS to separate theoretical from in-the-wild exploitability.

### Hardening Non-Root And Stopping There

Switching to a non-root user and considering runtime hardening done, leaving a writable rootfs, default capabilities, and no seccomp/AppArmor. Layer read-only rootfs, dropped capabilities, no-new-privileges, and MAC profiles; non-root is the floor, not the ceiling.

### A Secret In A Build Arg "Because It Is Only For Build"

Passing a token through `ARG` to clone private deps, believing it is safe because it is not in the final stage. The token is in the image history and recoverable. Use BuildKit secret mounts for build-time secrets, and rotate anything that was ever passed as an `ARG`.

### SBOMs Generated But Never Queryable

Producing SBOMs to check a compliance box but storing them where no one can query them, so when a new CVE drops you still cannot answer "which images are affected." Store SBOMs in a queryable system (an SBOM registry/service) and rehearse the query before you need it under pressure.

### Runtime Detection That Only Alerts Into A Void

Standing up Falco with default rules, wiring it to a noisy alert channel, and letting it be ignored. Tune rules to your environment, route high-confidence detections to a watched channel, and (for high-value services) automate isolation. An alert no one acts on is worse than nothing because it implies coverage that does not exist.

## Self-Check

- [ ] Images are scanned at build, on push/promotion (where policy gates entry to trusted registries), and periodically against the deployed fleet, so a CVE published after build is still caught in production.
- [ ] The scanner was chosen by coverage of the stack's language ecosystems and base distros and by workflow fit (CI gate vs. registry vs. developer portal), not by habit, and covers any coverage gaps with a second tool where needed.
- [ ] Triage policy sorts by exploitability/reachability and EPSS in addition to CVSS severity, blocks releases on reachable high-EPSS fixable criticals, and tracks (rather than blocks on) theoretical or unreachable findings.
- [ ] There is a regular rebuild cadence against a current patched base (pinned by digest) so the long tail of CVEs is absorbed by base updates, with immediate action reserved for high-EPSS reachable criticals, and the fleet's average CVE age/count trends down.
- [ ] Every promoted image is signed (cosign/Sigstore), has an SBOM generated and attached, and has a SLSA/in-toto provenance attestation recording source commit, base digests, build params, and builder identity.
- [ ] Admission policy (cosign policy / Kyverno / Gatekeeper / Sigstore Policy Controller) rejects images that are unsigned, unattested, or from untrusted sources, scoped appropriately per environment, so signing is enforced not decorative.
- [ ] The runtime security context sets a read-only root filesystem with explicit writable volumes, drops ALL capabilities and adds back only what is needed, sets no-new-privileges, and applies a seccomp profile and AppArmor/SELinux appropriate to the service.
- [ ] No production secret is baked into any image; secrets are injected at runtime from mounted stores, cloud secret managers, Vault, or External Secrets Operator, and build-time secrets use BuildKit secret mounts — any previously-baked secret has been rotated.
- [ ] Runtime threat detection (Falco/eBPF) runs with tuned rules, routes high-confidence detections to a watched channel (and automated isolation for high-value services), and correlates findings back to image identity, SBOM, and version.
- [ ] Controls map to applicable compliance frameworks (CIS Benchmarks, PCI-DSS, HIPAA, etc.) and produce retained, queryable evidence (scan reports, SBOMs, admission decisions, runtime alerts) so audit requests are queries, not projects.
