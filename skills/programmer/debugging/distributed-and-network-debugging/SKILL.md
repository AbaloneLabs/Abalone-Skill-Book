---
name: distributed_and_network_debugging.md
description: Use when the agent is debugging a distributed system — a request failing across multiple services, a network timeout, a partial failure where some nodes are reachable and others not, a consistency anomaly, a mysterious latency spike, or a failure that appears intermittent or environment-dependent; using distributed tracing, request correlation, network diagnostics, or service-level isolation to locate the failing component. Covers distributed tracing and request correlation, the "is it my code, my dependency, or the network" triage, partial failure and network asynchrony, intermittent and environment-dependent failures, reproducing distributed failures locally, and the discipline of isolating the failing component before diving into its internals.
---

# Distributed And Network Debugging

Debugging a single-process program is hard; debugging a distributed system is harder by an order of magnitude, because the failure may be in any of several components, the evidence is spread across them, and the system's partial-failure modes (some nodes up, some down; the network slow but not down; messages delayed, reordered, or dropped) do not occur in single-process programs. A request that fails in a distributed system may have failed in the client, in any of the services it traversed, in a dependency of one of those services, or in the network between them — and the error the client sees (a timeout, a 500, a corrupted response) often does not indicate where the failure occurred. Without distributed tracing and request correlation, the failure is a mystery spread across logs and metrics on multiple machines, with no thread connecting them. The discipline of distributed debugging is using correlation (trace IDs, request IDs) to follow a request across components, isolating the failing component (is it my code, my dependency, or the network?) before diving into its internals, and accounting for the partial-failure and asynchrony modes that single-process intuition does not cover.

Agents tend to debug distributed failures with single-process intuition (assuming the error's location is obvious, that a response means success, that the network is reliable), to search logs across services without correlation (unable to connect events), and to overlook intermittent or environment-dependent failures that do not reproduce locally. The judgment problem is recognizing that distributed failures require correlation to follow, that the triage (which component) precedes the diagnosis (why), and that partial failure, network asynchrony, and intermittency are the characteristic failure modes. This skill covers the discipline of distributed and network debugging: correlation and tracing, component triage, partial failure, intermittency, reproduction, and network diagnostics.

## Core Rules

### Correlate The Request Across Components With Tracing

The foundational tool for distributed debugging is request correlation: a trace that follows a single request across all components, connecting the logs, metrics, and errors each component emitted.

- **Use distributed tracing to follow the request across services.** A trace (with a trace ID propagated across service calls — see telemetry-pipeline-and-collection) shows the request's journey: each service it entered, each operation within, the time spent, and where it failed. A trace that stops at a service, or shows an error span, identifies the failing component.
- **Correlate logs and metrics by trace ID and request ID.** Logs across services, tagged with the trace ID, can be filtered to a single request, showing what each component did. Without the trace ID, logs across services cannot be connected, and the failure is a mystery.
- **Include the trace ID in error reports and user-facing error references.** An error reported to the user (or to an error system) should include the trace ID, so the support team or engineer can immediately pull the trace and logs for that request. See error-reporting-and-crash-aggregation.
- **Propagate context (user ID, tenant ID, operation) alongside the trace ID.** Additional context allows filtering by user or operation, narrowing to the affected requests when a trace ID is not available (e.g., a user reports a problem without a trace ID).

### Triage: Is It My Code, My Dependency, Or The Network?

Before diving into a component's internals, triage which component failed. The error the client sees rarely indicates the location.

- **Follow the trace to find where the request failed or slowed.** The trace shows the last successful span and the failing (or slow) span, identifying the component. A trace that ends at service X (no further spans) suggests X failed or could not reach its dependency.
- **Distinguish "my code failed" from "my dependency failed" from "the network failed."** A 500 from a service is its code failing; a timeout calling a dependency is the dependency or the network; a connection refused is the dependency down or unreachable. Each points to a different component and a different investigation.
- **Check the dependency's health and metrics, not just the caller's error.** A caller's timeout to a dependency may be the dependency being slow (check its latency metrics), down (check its health), or the network being slow or partitioned (check network metrics and connectivity). Do not assume the caller's error tells the whole story.
- **Use the dependency's own tracing and logs to continue the investigation.** If the trace shows the request reached the dependency, the dependency's spans and logs show what it did and where it failed. Follow the trace across the boundary.

### Account For Partial Failure And Network Asynchrony

Distributed systems exhibit partial failure (some nodes up, some down) and network asynchrony (delays, reordering, drops) that single-process programs do not. These cause characteristic failure modes.

- **Partial failure: some instances of a service work, others do not.** A request may succeed or fail depending on which instance it hits (one instance is wedged, misconfigured, or on a bad node). A failure that affects only some requests (intermittent, or correlated with a specific instance) suggests partial failure. Check per-instance metrics and health.
- **Network partitions and degradations: the network is slow, lossy, or partitioned, not down.** A network that is slow (high latency) or lossy (dropping packets, causing retransmits) causes timeouts and retries without being "down." A partition (some nodes cannot reach others) causes split-brain or unavailable subsets. Network issues often manifest as intermittent timeouts or latency spikes.
- **Message delay, reordering, and duplication.** Network messages may be delayed (arriving late, causing timeouts), reordered (arriving in a different order than sent), or duplicated (arriving twice). Code that assumes timely, ordered, unique delivery fails under these conditions. See data-replication-and-consistency and async-and-concurrent-error-propagation.
- **Clock skew across nodes.** Timestamps from different nodes are not directly comparable (clocks skew), causing anomalies in time-based logic (TTL expiration, last-write-wins, scheduling). Verify timestamps are not the issue; use logical clocks or a time service where ordering matters. See distributed-algorithm-selection.

### Investigate Intermittent And Environment-Dependent Failures

Distributed failures are often intermittent (sometimes they occur, sometimes not) or environment-dependent (occur in production but not locally), which makes them hard to reproduce and diagnose.

- **Gather data on the conditions of the failures: when, how often, under what load, for which users or requests.** A failure that occurs only under high load, only for certain request types, only at certain times, or only for certain users has a pattern that points to the cause. Collect enough instances to identify the pattern.
- **Compare failing and succeeding requests.** What is different about a request that fails vs one that succeeds? Different instance (partial failure), different data (a specific input triggering a bug), different timing (a race condition), different load (a capacity issue). The diff reveals the trigger.
- **Reproduce in a test environment with the production conditions.** Intermittent failures often require the production conditions (load, data, concurrency, network characteristics) to reproduce. A test environment that lacks these may not reproduce the failure; add realism (load testing, production-like data, chaos testing).
- **Use chaos engineering to inject failures and verify behavior.** Deliberately injecting failures (killing instances, adding latency, partitioning the network) reveals how the system behaves under partial failure and can reproduce intermittent issues. A system that fails under injected failure has a resilience gap to fix.

### Use Network Diagnostics To Confirm Or Rule Out Network Causes

When the network is a suspect (timeouts, connection issues, partitions), network diagnostics confirm or rule it out.

- **Check connectivity and latency between components (ping, traceroute, tcpdump).** Verify the components can reach each other and measure the latency. A high latency or packet loss confirms a network issue; normal latency rules it out (pointing to the component).
- **Check DNS resolution.** A failure to resolve a service's name (DNS issue, stale cache, misconfigured record) causes connection failures that look like the service is down. Verify DNS resolves correctly from the failing component.
- **Check firewall, security group, and network policy rules.** A blocked port or IP (a firewall rule, a security group change, a network policy update) causes connection failures. Verify the network policy allows the expected traffic.
- **Check for connection exhaustion and resource limits.** A component that cannot open new connections (connection pool exhausted, file descriptor limit reached, port range exhausted) fails to reach dependencies, looking like a network issue but rooted in resource exhaustion.

### Reproduce The Failure In A Controlled Setting

To diagnose and verify a fix, reproduce the failure in a controlled setting (local or test environment).

- **Reproduce with the failing request's trace and inputs.** The trace and inputs of a failing request provide the recipe to reproduce it: the same data, the same parameters, the same path. Replay the request in a test environment.
- **Reproduce the distributed topology, not just one service.** A failure that occurs across services may not reproduce with one service alone; reproduce the relevant topology (the caller and the dependency, or the full path) to trigger the failure.
- **Inject the failure conditions (load, latency, partial failure) if needed.** If the failure requires specific conditions (high load, network latency, a specific instance being down), inject them in the test environment to reproduce.
- **Accept that some distributed failures cannot be fully reproduced locally.** A failure dependent on production scale, data, or network characteristics may not reproduce in a test environment. In these cases, rely on production observability (tracing, metrics, logs) and targeted experiments (chaos injection, canary changes) to diagnose.

## Common Traps

### Single-Process Intuition Applied To Distributed Failures

Assuming the error's location is obvious, a response means success, or the network is reliable — none of which hold distributed. Use correlation; triage the component; account for partial failure.

### Debugging Without Correlation (Searching Logs Across Services)

Searching logs across services without a trace ID or request ID, unable to connect events across components. Correlate by trace ID; use distributed tracing.

### Assuming The Caller's Error Tells The Whole Story

A caller's timeout or 500 treated as the full picture, without checking the dependency's health and metrics. Check the dependency; follow the trace across the boundary.

### Overlooking Partial Failure

A failure that affects only some requests (intermittent, instance-correlated) not recognized as partial failure, investigated as a global issue. Check per-instance metrics and health.

### Intermittent Failure Treated As Unreproducible (And Ignored)

An intermittent failure that is hard to reproduce dismissed rather than investigated via pattern gathering and condition reproduction. Gather failure conditions; reproduce with realism.

### Network Assumed Reliable

Network issues (latency, loss, partition) not considered, with failures attributed to components. Use network diagnostics; consider the network as a suspect.

### Clock Skew Ignored

Timestamps from different nodes compared directly, causing anomalies in time-based logic. Account for clock skew; use logical clocks or a time service.

### Reproduction Attempted Without Distributed Conditions

A failure reproduced with one service or under light load, missing the conditions (topology, load, data) that trigger it. Reproduce the topology and conditions; use chaos injection.

## Self-Check

- [ ] Requests are correlated across components via distributed tracing (trace ID propagated across service calls), logs and metrics are filterable by trace ID and request ID, errors include the trace ID for immediate investigation, and additional context (user, tenant, operation) is propagated for filtering.
- [ ] The failing component is triaged before diving into internals — the trace is followed to find where the request failed or slowed, "my code vs my dependency vs the network" is distinguished, the dependency's health and metrics are checked (not just the caller's error), and the trace is followed across the service boundary.
- [ ] Partial failure (some instances failing, causing intermittent or instance-correlated failures) and network asynchrony (delay, reordering, duplication, partitions, degradations) are accounted for, per-instance metrics are checked, and clock skew is considered for time-based anomalies.
- [ ] Intermittent and environment-dependent failures are investigated via pattern gathering (when, how often, under what conditions), comparison of failing vs succeeding requests, reproduction with production conditions (load, data, concurrency, network), and chaos engineering to inject failures and reveal resilience gaps.
- [ ] Network causes are confirmed or ruled out with network diagnostics (connectivity, latency, DNS, firewall/security group rules, connection exhaustion), rather than assuming components are at fault or the network is reliable.
- [ ] The failure is reproduced in a controlled setting (the failing request's trace and inputs, the distributed topology, injected failure conditions) to diagnose and verify the fix, with the understanding that some distributed failures require production observability and targeted experiments when local reproduction is infeasible.
- [ ] The investigation distinguishes the symptom (what the client observes) from the root cause (what actually failed), following the causal chain across components rather than stopping at the first observed error.
- [ ] Findings are documented (the failure, its location, its cause, the fix) and shared, because distributed failures often reveal systemic issues (a missing trace propagation, a resilience gap, a monitoring blind spot) that, once known, prevent future failures across the system.
