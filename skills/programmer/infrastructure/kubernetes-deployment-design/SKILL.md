---
name: kubernetes_deployment_design.md
description: Use when the agent is designing Kubernetes workloads for production, writing Deployment, Pod, Service, Ingress, or HorizontalPodAutoscaler manifests, setting resource requests and limits and QoS classes, configuring liveness readiness and startup probes, scheduling with affinity anti-affinity taints tolerations and topology spread constraints, wiring autoscaling, mounting ConfigMaps and Secrets, or planning namespace isolation and rolling update strategy. Covers the runtime configuration of pods and services, distinct from the container image itself and from how the manifests are delivered through IaC.
---

# Kubernetes Deployment Design

A Kubernetes Deployment looks deceptively simple to write: a container image, a port, a replica count, and `kubectl apply`. The judgment problem is that the defaults are wrong for production, and the harm they cause is invisible until load, a deploy, or a node failure exposes it. A pod with no resource requests gets evicted under pressure and never rescheduled fairly; a liveness probe set a little too aggressive kills the pod in a loop during a slow startup; a Service with no readiness gate routes traffic to a container that is up but not ready; a Deployment with three replicas all on one node loses an availability zone and calls itself highly available. None of these fail in dev. All of them fail in prod, at the worst moment.

Agents tend to copy a working Deployment and change the image, inheriting whatever defaults the original had, because the manifest applies cleanly and the pod becomes Ready. But "Ready" only means the kubelet could start the container; it says nothing about whether the workload will survive a rolling update, a node loss, a traffic spike, or a misbehaving dependency. The discipline of Kubernetes workload design is to make the failure behavior explicit in the manifest: how much CPU and memory the pod is guaranteed, how the cluster knows it is healthy versus merely alive, where it is allowed to schedule, how many replicas exist and where, and how a new version rolls out without taking the old one down until the new one is proven.

This skill is about the runtime configuration of the workload — the Pod, Deployment, Service, Ingress, autoscaler, and namespace objects. It is distinct from the container image (its base, layers, user, and entrypoint) and from how the manifest is delivered (GitOps, Helm, Kustomize, or raw IaC). Those matter, but here the question is what the manifest must say so the cluster runs the workload safely.

## Core Rules

### Set Resource Requests Always, And Choose Limits Deliberately

Resource requests are the scheduling and eviction guarantee; limits are the enforcement ceiling. They are not the same setting, and confusing them is the most common Kubernetes mistake.

- **Requests are mandatory for any production pod.** The scheduler uses requests to place pods, and the kubelet uses them to decide what to evict under memory pressure. A pod with no memory request is treated as having requested zero, so it is the first thing killed when a node is starved, and it can be scheduled onto a node that has no room for it. Always set CPU and memory requests.
- **Decide limits as a separate, deliberate choice.** A CPU limit caps how much CPU the container can use even when the node is idle; setting one too low causes CPU throttling that silently destroys latency even though the pod reports healthy. A memory limit, once exceeded, gets the container OOM-killed. The strong patterns are: set memory limits (to bound a leak), and be cautious with CPU limits — many teams run with CPU requests but no CPU limits, or with limits well above requests, specifically to avoid throttling.
- **Aim for the QoS class you actually want.** Guaranteed (requests equal limits, for both CPU and memory, on every container) gives the pod the lowest eviction priority and predictable scheduling. Burstable (requests below limits, or only requests) is the common case for variable workloads. BestEffort (no requests or limits) should not exist in production because it is evicted first and scheduled onto nodes that may not have the resources. Choose the class deliberately; do not arrive at BestEffort by omission.
- **Size from measurement, then revisit.** Requests that are too high waste cluster capacity and block scheduling; too low invites eviction and throttling. Start from observed usage (the 95th percentile plus headroom) and adjust. Requests set once and never revisited are usually wrong within months.

### Use The Right Probe For The Right Question, And Never Make Liveness Aggressive

Probes are how the cluster learns about container health, and the three kinds answer different questions. Misusing them — especially making liveness aggressive — is a leading cause of self-inflicted outages.

- **Readiness probe answers "can this pod receive traffic?"** A failing readiness probe removes the pod from the Service's endpoints but does not restart it. Use it for any dependency-driven unavailability: the pod is up but the database is unreachable, so it should stop receiving requests. This is the probe that protects callers.
- **Liveness probe answers "is this pod so broken it must be restarted?"** A failing liveness probe kills and restarts the container. It must be lenient: a slow request, a GC pause, or a cold start must not trip it, or the pod enters a kill loop that never recovers. Liveness is for deadlocks and stuck states the application cannot recover from itself, not for transient slowness.
- **Startup probe answers "has the application finished initializing?"** It exists so that a slow-starting app is not killed by the liveness probe during boot. For any app with non-trivial startup (JVM warmup, migration, cache fill), set a startup probe with a generous failure threshold and a long-enough window, and let liveness take over only after startup succeeds. Without a startup probe, the common "fix" is to weaken liveness so much it never catches real deadlocks.
- **Prefer the probe type that reflects real readiness.** An HTTP readiness probe on a real health endpoint is stronger than a TCP probe (which only proves the port is open) and far stronger than no probe. But the endpoint must actually check dependencies — a health endpoint that always returns 200 protects nothing.

### Spread Replicas For Real Availability, Not Just Replica Count

Three replicas is not high availability if they all land on the same node or the same zone. Availability is a property of where the pods are, not how many exist.

- **Use topology spread constraints to distribute across zones.** `topologySpreadConstraints` with `maxSkew` and `whenUnsatisfiable: DoNotSchedule` (for hard requirements) or `ScheduleAnyway` (for soft) is the modern way to spread pods across topology domains like zones. This is what makes a single-zone failure survivable.
- **Use pod anti-affinity to keep replicas off the same node.** For stateless services, prefer `podAntiAffinity` so two replicas do not co-locate on one node. For stricter needs, combine anti-affinity with topology spread.
- **Account for node affinity, taints, and tolerations.** Where a pod can run is also constrained by node affinity (which nodes it wants), taints (which nodes repel it), and tolerations (which taints it tolerates). A workload that tolerates only one node pool is not spreadable no matter what the constraints say. Make the scheduling surface match the availability goal.
- **Set replica count above the failure capacity.** If you need 2 to handle load and you run exactly 2 across 2 zones, losing a zone drops you to 1 with no margin. Run 3 (or more) so that losing one still leaves enough capacity. The count must exceed the largest single failure domain you can lose.

### Make The Rolling Update Strategy Match Your Tolerance For Downtime And Bad Versions

The Deployment's update strategy decides what happens when the image tag changes. The defaults are usually too loose for stateful or latency-sensitive services.

- **`RollingUpdate` with `maxUnavailable` and `maxSurge` tuned to the workload.** `maxUnavailable: 0` with a positive `maxSurge` guarantees the old version stays fully available while the new one comes up — the safe choice for anything user-facing. `maxUnavailable` greater than zero speeds the rollout but briefly reduces capacity. Choose based on whether you can tolerate a capacity dip.
- **Readiness gates the rollout.** A new pod only receives traffic (and only counts toward rollout progress) once it is Ready, which means its readiness probe passes. If readiness is weak, a broken version rolls out fully before anyone notices. Strong readiness is what makes a rolling update safe.
- **Understand rollback.** `kubectl rollout undo` returns to the previous ReplicaSet, but it does not undo side effects (a database migration the new version ran, schema changes, written data). A rollback is a code-level revert of the pod spec, not a system-level time machine. For stateful workloads, plan the forward path because rollback may not be safe.
- **Prefer immutable, version-tagged images over `latest`.** A Deployment that references `latest` is non-reproducible and makes rollout history meaningless. Pin to a digest or a version tag so a rollback actually means something.

### Separate Configuration From Secrets, And Handle Updates Deliberately

ConfigMaps and Secrets are how non-image configuration reaches the pod, and they have specific update semantics that bite when ignored.

- **Mount ConfigMaps and Secrets as volumes or env vars with the update tradeoff in mind.** Volume-mounted ConfigMaps are updated in the pod (after a short delay) without a restart; environment variables are baked at pod start and require a restart to change. If your app does not hot-reload config, env vars with an explicit rollout on change are safer than a volume mount the app never re-reads.
- **Trigger a rollout when config changes, or use immutable config.** The classic trap is updating a ConfigMap and assuming the pods picked it up. Either trigger a rollout (e.g., via a checksum annotation on the pod template that changes with the config) or mark the ConfigMap `immutable` and create a new one on change, forcing a referential update.
- **Prefer external secret stores for real secrets.** Mounting a raw Kubernetes Secret couples secret rotation to pod restarts and puts the secret value in etcd. For anything that rotates, use an external secrets operator (External Secrets, a cloud secrets manager) that syncs secrets in and can rotate them. Kubernetes Secrets are fine for static, cluster-scoped values; they are the wrong tool for credentials that change.
- **Restrict Secret access with RBAC and namespace boundaries.** A Secret is readable by anything with permission in its namespace. Do not put high-value secrets in a namespace shared by many workloads; scope them narrowly.

### Choose An Autoscaling Strategy That Matches The Workload's Signal

Autoscaling is not one mechanism. The right one depends on what signal predicts the need for more capacity.

- **HPA scales on a metric (CPU, memory, or custom).** It is the default for stateless services. CPU-based HPA assumes CPU correlates with load, which is true for compute-bound services and false for I/O-bound ones; for the latter, use a custom metric (requests per second, queue depth). HPA also requires that requests are set, because it scales relative to requested CPU.
- **VPA adjusts requests automatically.** It is useful for right-sizing but conflicts with HPA on the same CPU/memory metrics, and it restarts pods to apply new sizes. Use VPA for workloads HPA cannot serve, not alongside CPU-based HPA.
- **Cluster autoscaler or Karpenter adds nodes when pods are pending.** Pod-level autoscaling (HPA) is useless if there is no node to schedule the new pod on. A node autoscaler must be in the loop for HPA to actually deliver capacity. Know the delay: nodes take minutes to provision, so HPA must scale early enough that pending pods do not stall users.
- **Set scale-to-zero carefully.** It saves cost but introduces cold-start latency and breaks long-lived connections. Only stateless, latency-tolerant services should scale to zero; anything user-facing usually should not.

### Use Namespaces And Network Policies To Contain Blast Radius

Namespaces are the unit of quota, network policy, and RBAC scope. A namespace strategy is a segmentation strategy.

- **One namespace per team, tenant, or environment-tier.** Namespaces let you apply resource quotas (so one team cannot starve the cluster), network policies (so one namespace cannot reach another's databases), and distinct RBAC. Treat the namespace as a soft trust boundary.
- **Resource quotas and LimitRanges make sharing safe.** Without a quota, one runaway pod can consume the cluster. A `ResourceQuota` caps total requests per namespace; a `LimitRange` sets default and bounds on individual pods. Both belong in any multi-tenant cluster.
- **Network policies enforce the segmentation the namespace implies.** A namespace boundary alone does not prevent network access — by default pods can reach any pod. Add a default-deny network policy and then allow only the required flows, or the segmentation is conceptual, not real.
- **Decide soft versus hard isolation explicitly.** Soft isolation (namespaces plus policies plus quotas) suits teams in one cluster. Hard isolation (separate clusters) is required when tenants are mutually distrustful or have different compliance regimes. Do not pretend soft isolation is hard isolation.

## Common Traps

### Setting A Limit Without A Request, Or Omitting Both

A limit with no request leaves scheduling undefined (the scheduler may place the pod anywhere), and omitting both yields BestEffort QoS that is evicted first. Always set requests; set limits as a deliberate ceiling, not as a default.

### Aggressive Liveness That Kills Pods During Slow Starts Or Transient Load

A liveness probe with a short timeout and low failure threshold trips during a GC pause, a slow dependency, or a cold start, and restarts the pod — which then takes longer to start, trips again, and loops. Liveness must be lenient; use a startup probe for slow initialization and reserve liveness for genuine deadlocks.

### Using A TCP Or No Probe And Calling The Pod Ready

A TCP readiness probe only proves the port is open, not that the application can serve. A pod that passes a TCP probe but cannot reach its database receives traffic and returns errors. Use an HTTP probe against an endpoint that checks real readiness, including critical dependencies.

### Three Replicas On One Node Or One Zone And Calling It HA

Replica count without topology spread is not availability. A node or zone failure takes down all replicas. Spread with topology spread constraints and pod anti-affinity, and keep the count above the failure capacity of the largest domain you can lose.

### `maxUnavailable` Too High For A User-Facing Service

A rolling update with `maxUnavailable: 25%` on a 4-replica service briefly removes one pod, which may be fine — or may exceed your latency budget under load. For user-facing services, set `maxUnavailable: 0` and let `maxSurge` carry the rollout so capacity never dips.

### Updating A ConfigMap And Assuming Pods Picked It Up

Volume-mounted ConfigMaps update eventually; env-var config does not update at all without a restart. If the app does not watch for changes, a config update silently does nothing. Trigger a rollout via a checksum annotation, or use immutable ConfigMaps that force a new object and a referential change.

### Putting Rotating Credentials In A Raw Kubernetes Secret

A Secret mounted from a literal value requires a pod restart to rotate and stores the value in etcd. For credentials that change, use an external secrets operator that syncs from a manager and can rotate independently of the pod lifecycle.

### HPA Without A Node Autoscaler

HPA scales the pod count, but if no node can run the new pods, they stay Pending and users see no improvement. HPA must be paired with cluster autoscaler or Karpenter, and the scale-up delay of node provisioning must be factored into how early HPA triggers.

### Default-Deny Missing, So Namespaces Are Not Actually Isolated

A namespace with quotas and RBAC but no network policy still allows any pod to reach any pod. Segmentation requires a default-deny network policy plus explicit allows; without it, the namespace boundary is administrative, not network.

### Referencing `latest` So Rollout History Is Meaningless

A Deployment tagged `latest` cannot be rolled back meaningfully, because the previous ReplicaSet points at the same moving tag. Pin images to a digest or version tag so rollout undo returns to a known-good artifact.

## Self-Check

- [ ] Every container has explicit CPU and memory requests (so it schedules and evicts predictably), limits are set as a deliberate ceiling rather than copied defaults, and the resulting QoS class (Guaranteed, Burstable, BestEffort) was chosen on purpose with no production pod left as BestEffort.
- [ ] A readiness probe checks real application readiness including critical dependencies (not just an open port), a liveness probe is lenient enough to survive GC pauses and transient load, and any slow-starting app has a startup probe so liveness does not kill it during initialization.
- [ ] Replicas are spread across failure domains with topology spread constraints and pod anti-affinity, the scheduling surface (node affinity, taints, tolerations) actually permits that spread, and the replica count exceeds the capacity lost if the largest single failure domain fails.
- [ ] The rolling update strategy has `maxUnavailable` and `maxSurge` tuned to the service's tolerance (zero unavailable for user-facing services), rollout is gated by strong readiness, images are pinned to a digest or version tag (not `latest`), and the cases where rollback is unsafe (migrations, written data) were identified.
- [ ] Configuration reaches pods through a mechanism whose update semantics are understood (volume mount versus env var), config changes trigger a rollout or use immutable ConfigMaps so updates are not silently ignored, and rotating secrets come from an external secrets operator rather than raw Kubernetes Secrets.
- [ ] An autoscaling strategy was chosen to match the workload's predictive signal (CPU for compute-bound, custom metrics for I/O-bound), HPA is paired with a node autoscaler so scaled pods can actually schedule, and scale-to-zero is used only for latency-tolerant stateless services.
- [ ] Pods and deployments are configured for graceful shutdown — a `terminationGracePeriodSeconds` long enough for in-flight requests to drain, a readiness probe that stops routing new traffic immediately on SIGTERM, and preStop hooks where the app does not handle SIGTERM — so rolling updates and node drains do not drop in-flight requests.
- [ ] Namespaces are used as segmentation boundaries with resource quotas and LimitRanges for safe sharing, network policies enforce a default-deny with explicit allows so the segmentation is real and not just administrative, and the soft-versus-hard isolation choice matches the trust model between tenants.
