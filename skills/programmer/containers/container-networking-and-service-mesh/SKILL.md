---
name: container_networking_and_service_mesh.md
description: Use when the agent is designing or reviewing container networking, reasoning about the CNI model, configuring pod networking and overlay or underlay networks, exposing a service via ClusterIP, NodePort, LoadBalancer, or an ingress controller, setting up service discovery, defining network policies as the in-cluster firewall, deciding whether to adopt a service mesh for mTLS and traffic control, or debugging why two workloads cannot or should not reach each other. Use when the agent is exposing a service to the internet, restricting east-west traffic between namespaces, enabling zero-trust mutual TLS, implementing canary or weighted routing, or reviewing whether the cluster's networking defaults to allow-all. Risk surfaces include default-allow networking where any pod can reach any pod, exposed NodePorts that bypass ingress controls, missing network policies that leave databases reachable from every workload, service-mesh complexity that exceeds the team's ability to operate it, and the assumption that ClusterIP provides isolation rather than just reachability.
---

# Container Networking And Service Mesh

Container networking has two halves that agents routinely conflate. The first is *reachability* — how a packet gets from one pod to another, from a pod to the outside world, or from an external client to a service. The second is *policy* — which of those packets *should* be allowed. A default Kubernetes cluster answers the first question thoroughly (every pod can reach every other pod and most external endpoints) and the second question barely at all: in the absence of network policies, the network is flat and permissive, so a compromised or misconfigured pod can probe and reach every other workload in the cluster, including databases and internal-only services. Treating "the network works" as equivalent to "the network is secure" is the central mistake this skill exists to prevent.

Agents tend to focus on making connectivity *work* and treat the controls that restrict it as optional hardening to add later. That sequencing produces clusters where a frontend pod can talk directly to a database in a different namespace, where a NodePort exposes an internal service to the whole internet, where services discover and call each other over plaintext with no authentication, and where the addition of a service mesh adds mTLS and traffic control but also a layer of operational complexity the team cannot debug. The judgment problem is to design networking as two deliberate layers — a connectivity model (CNI, pod networking, service types, ingress) and a policy model (network policies, and where justified, a service mesh) — and to make the default posture restrictive rather than permissive.

This skill covers container networking and the service-mesh decision. It is distinct from `orchestration-and-scaling`, which covers replica counts, readiness, and graceful shutdown; here the focus is the *network* — the CNI model, service exposure, discovery, network policies, and when a service mesh earns its complexity.

## Core Rules

### Understand The CNI Model And How Pods Actually Reach Each Other

The Container Network Interface (CNI) is the plugin contract that assigns each pod an IP address and wires it into a network where pods can communicate without NAT. The common patterns are *overlay networks* (VXLAN, Geneve, WireGuard-based), which tunnel pod-to-pod traffic over the node network and work on any substrate but add encapsulation overhead; *underlay or routable* models (BGP-based plugins like Calico in routed mode, or cloud VPC-native networking like AWS VPC CNI), which assign pods routable IPs directly and avoid overlay overhead but require subnet capacity and routing support; and hybrid approaches. The choice affects performance, IP consumption, and whether pods are reachable from outside the cluster without translation.

The judgment is rarely "which CNI is best in the abstract"; it is "which model fits this environment's constraints." Cloud-native CNI gives pods routable VPC IPs and avoids overlay overhead but consumes IP space and couples pod addressing to the VPC; an overlay works anywhere and isolates pod traffic but adds latency and complexity. Whatever the choice, the agent should understand whether pod IPs are routable outside the cluster, how the plugin handles network policy (some CNI plugins include policy enforcement; others require a separate component), and what the failure mode looks like when the CNI mis-assigns an IP. Networking bugs that manifest as "pod cannot reach the database" are very often CNI or routing issues, not application issues.

### Choose The Right Service Type For How The Workload Should Be Exposed

A Kubernetes Service provides a stable virtual IP (ClusterIP) and DNS name that load-balances over a set of pods, decoupling callers from the ephemeral pod IPs. The *type* of the Service decides who can reach it:

- **ClusterIP** (default) — reachable only from inside the cluster. Correct for internal services, databases, and anything that should never be directly exposed. Note that ClusterIP provides *reachability*, not *isolation* — any pod in the cluster can still reach it unless a network policy restricts it.
- **NodePort** — exposes the service on a static port on every node's IP, reachable from outside the cluster. Convenient for quick exposure but exposes the port broadly (often to the whole internet if nodes are public), bypasses ingress-level controls, and consumes a limited port range. Rarely the right choice for production internet-facing services.
- **LoadBalancer** — provisions a cloud load balancer that fronts the service. Appropriate for exposing a single service to the internet or to a private network, but each service gets its own balancer (cost) and each is a separate public endpoint to manage.
- **Ingress** (via an ingress controller — NGINX, Traefik, Envoy-based, cloud ingress) — the standard way to expose many HTTP services through one or a few load balancers, using host- and path-based routing and terminating TLS at the edge. This is the correct pattern for most internet-facing HTTP workloads.

The decision should be deliberate: internal services stay ClusterIP and are restricted by network policy; internet-facing HTTP goes through an ingress controller that centralizes TLS, routing, and rate limiting; non-HTTP or TCP services use a LoadBalancer when they must be external. Reaching for NodePort to "just expose it" is a common shortcut that creates an unmanaged public endpoint.

### Use Ingress Controllers To Centralize External Entry And TLS

An ingress controller is the single, controlled front door for external HTTP traffic: it terminates TLS, routes by host and path to backend services, applies rate limits and authentication, and produces unified access logs. Concentrating external entry in one place is both a convenience and a control — it means TLS certificates, security headers, and access policy are managed in one location rather than scattered across per-service load balancers. Pair it with a certificate management automation tool (cert-manager requesting Let's Encrypt or an internal CA) so certificates are issued and rotated automatically rather than hand-managed into expiry.

The traps at the ingress layer are exposing services that should be internal (an ingress rule that routes to a database or admin panel without authentication), trusting the ingress alone for security (it handles north-south traffic but does not restrict east-west pod-to-pod traffic), and letting ingress rules drift unmanaged across teams. Treat the ingress configuration as part of the security perimeter: require authentication for sensitive routes, use TLS for everything external, and review the full set of routes as a deliberate exposure inventory.

### Treat Network Policies As The In-Cluster Firewall And Default To Deny

The single most important security control in container networking is the network policy. Without it, the cluster network is flat: every pod can reach every other pod and every service, so a single compromised pod can enumerate and attack the entire cluster's internal surface, including databases, caches, and internal-only APIs. Network policies are the in-cluster firewall — they specify which pods (by label selector) may talk to which other pods on which ports, and they are enforced by the CNI or a dedicated policy engine. The correct posture is *default deny*: start by denying all ingress and egress to a namespace, then add explicit allow rules for the traffic that should exist.

Default-deny is a posture, not a one-time configuration, and it requires knowing the real traffic flows. The practical approach is to observe the actual communication (via mesh telemetry, flow logs, or tools that infer policy from observed connections), generate a baseline policy, then enforce it. The common failure is writing policies that are too permissive (broad namespace selectors that amount to allow-all) or too narrow (breaking legitimate traffic that was not observed during baseline). Validate policies in a staging environment before enforcing in production, and treat the set of network policies as the cluster's segmentation design — the artifact that expresses "which workloads may talk to which," reviewed and version-controlled like any other security control.

### Decide Whether A Service Mesh Earns Its Complexity

A service mesh (Istio, Linkerd, Consul Connect, Cilium service mesh) inserts a layer — typically a per-pod sidecar proxy, or an eBPF-based node component — that mediates service-to-service traffic. It provides mutual TLS (every internal call is encrypted and authenticated, without application changes), fine-grained traffic control (weighted routing for canaries, retries, timeouts, circuit breaking), and deep observability (per-service latency, success rates, and topology from L7 telemetry). For a cluster running many internal services with real east-west security and traffic-control needs, these capabilities are valuable and hard to replicate per-application.

The trap is adopting a mesh for capabilities the team does not need, or without reckoning the operational cost. A sidecar mesh adds latency and resource overhead to every pod, a new control plane to operate and upgrade, a new failure mode (a misconfigured sidecar breaks traffic in ways that are hard to debug), and a steep learning curve for on-call engineers. The decision should be explicit: adopt a mesh when you genuinely need zero-trust mTLS across many services, L7 traffic control that the application cannot easily provide, or unified observability you cannot get otherwise — and when the team can operate it. If the need is only mTLS between a few services, a lighter solution (Cilium's native encryption, application-level TLS, or a CNI with built-in mTLS) may deliver the value without the sidecar tax. Do not install Istio because it is mentioned in a blog post; install it because the problem in front of you is a mesh problem.

## Common Traps

### Default-Allow Networking Where Any Pod Can Reach Any Pod

Running a cluster with no network policies, so the network is flat and a compromised or misconfigured pod can reach every other workload including databases and internal-only services. Adopt a default-deny posture with explicit allow rules for the traffic that should exist; ClusterIP provides reachability, not isolation.

### Exposing An Internal Service Via NodePort

Using a NodePort to "just expose" a service, creating a broad public endpoint on every node that bypasses ingress-level TLS, routing, and access controls. Use an ingress controller for HTTP, a LoadBalancer for TCP, and keep internal services ClusterIP plus network policy.

### Confusing Reachability With Isolation

Believing a ClusterIP service is "isolated" because it has no external endpoint, when in fact any pod in the cluster can reach it. Isolation comes from network policies, not from the service type; restrict internal services explicitly.

### Network Policies That Are Too Broad Or Too Narrow

Writing policies with wide namespace selectors that amount to allow-all, or policies so narrow they break legitimate traffic not captured in the baseline. Derive policies from observed real flows, validate in staging, and treat the policy set as the cluster's reviewed segmentation design.

### Adopting A Service Mesh Without Justifying The Complexity

Installing Istio or a sidecar mesh for mTLS or canary routing that the workload does not need, taking on latency, resource, control-plane, and debugging costs the team is not ready for. Adopt a mesh when zero-trust mTLS, L7 traffic control, or unified observability is a real requirement and the team can operate it; otherwise use a lighter solution.

### Trusting The Ingress Alone For Cluster Security

Relying on the ingress controller to secure the cluster, when it controls only north-south (external) traffic and does nothing to restrict east-west (pod-to-pod) communication. Pair ingress controls with network policies for internal segmentation.

### Unmanaged TLS And Drifting Ingress Routes

Hand-managing certificates that expire and break services, or letting ingress routes accumulate across teams so that an admin panel or database is exposed without authentication. Automate certificates with cert-manager, require TLS for external traffic, and review the full route set as a deliberate exposure inventory.

## Self-Check

- [ ] The CNI model is understood for this environment — whether pod IPs are routable outside the cluster, whether overlay or underlay is used, what the overhead and IP-consumption tradeoffs are, and what the failure mode looks like when the CNI mis-assigns an IP.
- [ ] Each service's exposure type is deliberate: ClusterIP for internal services, an ingress controller for internet-facing HTTP, a LoadBalancer for external TCP, and NodePort avoided as a production exposure mechanism.
- [ ] An ingress controller centralizes external entry, terminates TLS (with certificates automated via cert-manager or equivalent), applies routing and access controls, and the full set of routes is reviewed as a deliberate exposure inventory.
- [ ] A default-deny network policy posture is in place (ingress and egress denied by default, explicit allows for legitimate traffic), derived from observed real flows and validated in staging before production enforcement.
- [ ] No internal service (database, cache, internal API) is reachable from every pod in the cluster; each is restricted by network policy to the specific workloads and ports that need it, and ClusterIP is not mistaken for isolation.
- [ ] The service-mesh decision is justified: a mesh is adopted only when zero-trust mTLS, L7 traffic control, or unified observability is a real requirement the team can operate, and lighter alternatives (native encryption, application TLS) are used when they suffice.
- [ ] If a service mesh is in use, its overhead (latency, resources), control-plane operation, and failure modes are understood, and the team can debug a sidecar or routing misconfiguration rather than being blindsided by it.
- [ ] No NodePort exposes an internal or sensitive service broadly; any NodePort in use is intentional, scoped, and documented as a deliberate exception rather than a convenience shortcut.
