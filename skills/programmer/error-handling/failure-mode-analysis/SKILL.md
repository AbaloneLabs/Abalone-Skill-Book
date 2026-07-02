---
name: failure_mode_analysis.md
description: Use when the agent is performing FMEA or failure mode and effects analysis, identifying single points of failure, assessing failure impact and severity, designing mitigation strategies, analyzing cascade and common-cause failures, or evaluating overall system resilience and redundancy.
---

# Failure Mode Analysis

Most outages are not caused by a single mysterious bug; they are caused by failure modes that were always present but never identified and mitigated. A database that is the sole write path, a cache that is a hard dependency, a certificate that expires, a DNS record that is single-homed—these are latent failures waiting for the right trigger. Failure Mode and Effects Analysis (FMEA) is the discipline of systematically enumerating how a system can fail, what the impact of each failure is, and what mitigation reduces the risk. It turns "we did not think of that" into "we considered it and here is what we do."

The judgment problem is identifying the failure modes that matter (including the non-obvious ones like common-cause and cascade failures), assessing their impact honestly rather than optimistically, choosing mitigations that actually reduce risk rather than adding complexity that creates new failure modes, and recognizing where redundancy is illusory. The agent should not treat resilience as a feature to add; it should treat it as a property to analyze.

This skill applies whenever you are designing or reviewing a system for resilience, planning capacity and redundancy, or preparing for incidents.

## Core Rules

### Enumerate failure modes systematically, component by component

Walk the architecture and, for each component, ask how it can fail and what happens when it does. Cover the full failure space:

- **Hardware/instance failure**: a server, disk, or network interface dies.
- **Network failure**: latency, packet loss, partition, DNS failure, a misconfigured firewall.
- **Dependency failure**: a downstream service, database, cache, queue, or third-party API is slow, erroring, or unavailable.
- **Resource exhaustion**: disk full, memory exhausted, connection pool depleted, thread pool saturated, certificate or quota expired.
- **Data failure**: corruption, replication lag, loss of a replica, split-brain.
- **Human/config failure**: a bad deploy, a misconfigured flag, a typo in a runbook, an expired credential.
- **Time/clock failure**: clock skew, leap second, expired certificate tied to time.

Do not stop at "the server crashes." The non-crash failures (slow, partially available, returning wrong data) are often more dangerous because they are less visible.

### Identify single points of failure (SPOFs)

A single point of failure is any component whose failure causes the overall system to fail. For each component, ask: if this fails, does the system continue serving? Common SPOFs:

- A single database instance with no replica or failover.
- A single availability zone or data center for a service that claims high availability.
- A single load balancer, DNS provider, or CDN.
- A single cache that is a hard dependency (not a fallback).
- A single human or team that holds unique knowledge or access ("bus factor").
- A single credential, certificate, or quota whose expiry halts the system.

Eliminate SPOFs by adding redundancy (replicas, multi-AZ, multi-provider), or by removing the hard dependency (make the cache a fallback, not a requirement).

### Assess impact along severity, likelihood, and detectability

For each failure mode, assess three dimensions (the classic FMEA risk-priority approach):

- **Severity**: how bad is the impact? Data loss and safety harm are worst; full outage is severe; partial degradation is moderate; cosmetic is minor. Be honest, not optimistic—a "read-only mode" for a write-heavy service is effectively an outage.
- **Likelihood**: how often is this expected to happen? Hardware failures are routine at scale; a region outage is rare; a specific software bug may be one-time.
- **Detectability**: how quickly will you know? A failure with no alert may persist for hours; one with good monitoring is caught in minutes.

Prioritize mitigations by the combination (high severity × high likelihood × low detectability is the worst). A high-severity, low-likelihood risk still warrants mitigation if the impact is catastrophic.

### Analyze cascade and common-cause failures

The most damaging outages are not single-component failures but cascades, where one failure triggers others:

- **Cascade**: a dependency fails, callers retry and overload the next tier, which fails, and so on. Retries, timeouts, and shared resource pools are common cascade vectors.
- **Common-cause**: a single underlying event causes multiple "redundant" components to fail together. Two replicas in the same zone fail when the zone fails; two services sharing a dependency fail together when it fails; replicas with the same software bug fail together on the same input.

Redundancy protects against independent failures, not common-cause failures. Two replicas in different zones with the same software version share the software-bug common cause. Analyze what your redundant components actually share (zone, provider, software, config, credentials, time) and diversify where the shared cause is significant.

### Choose mitigations that reduce risk without adding new failure modes

Mitigations themselves can introduce risk:

- Adding a retry can cause retry storms (a new failure mode) unless paired with backoff and jitter.
- Adding a cache can turn a soft dependency into a hard one if the system cannot function without it.
- Adding automated failover can cause split-brain or data loss if the failover logic is buggy.
- Adding complexity can make the system harder to operate, increasing human-error failures.

For each mitigation, ask: what new failure mode does this introduce, and is the net risk lower? Prefer simple, well-understood mitigations over clever ones.

### Validate redundancy is real, not illusory

Redundancy that has never been tested may not actually work:

- A replica that has never been promoted may fail on promotion (corruption, schema drift, missing permissions).
- A multi-AZ deployment where traffic never actually fails over may have a broken failover path.
- A backup that has never been restored may be corrupt or incomplete.

Test failover and restore regularly (see chaos-and-fault-injection). Untested redundancy is a comfort, not a guarantee.

### Include the operational and human failure modes

Resilience is not only about software:

- **Deploy risk**: can a bad deploy take down the system? Mitigate with canaries, rollbacks, and progressive delivery.
- **Config/secret rotation**: will a credential or certificate expiry cause an outage? Track expiries and rotate before they lapse.
- **Runbook and on-call readiness**: when the failure occurs, will the on-call know what to do? An undocumented failure mode with no runbook extends every outage.
- **Capacity headroom**: does the system have margin for traffic spikes or the loss of a replica? A system running at 95% capacity has no resilience.

## Common Traps

### Stopping at crash failures

Analysis that only considers "the server is down" misses the more common and more dangerous slow/partial/wrong-data failures. Enumerate the full failure space, including degraded modes.

### Optimistic severity assessment

Calling a failure "minor" because the system has a manual workaround ignores that the workaround may be infeasible at scale or under time pressure. Assess severity at the impact to users and the business, not the convenience of the fix.

### Redundancy that shares a common cause

Two replicas in the same zone, or two services sharing a dependency, fail together. Redundancy protects only against independent failures. Identify and diversify shared causes.

### Mitigations that add new failure modes

A retry without backoff, a cache that becomes a hard dependency, or a failover mechanism that can split-brain each trade one failure mode for another. Evaluate the net risk.

### Untested failover and backup

A replica never promoted, a backup never restored, a multi-AZ setup that never fails over—these provide false confidence. Test them.

### Ignoring cascade vectors

A single dependency failure that triggers retry storms or saturates shared pools turns a local failure into a system-wide outage. Analyze how failures propagate, not just where they originate.

### Forgetting operational and human failures

A bad deploy, an expired certificate, or an undocumented failure mode can cause outages as severe as any hardware failure. Include them in the analysis.

### Analysis that is done once and never revisited

Systems change; new dependencies, new code, and new traffic patterns introduce new failure modes. Treat failure mode analysis as a living practice, revisited on architectural changes and after incidents.

## Self-Check

- Have you enumerated failure modes for each component across the full space (crash, slow, partial, wrong-data, resource exhaustion, dependency, network, config, human, time)?
- Are all single points of failure identified, with each either eliminated via redundancy or explicitly accepted with documented rationale?
- For each significant failure mode, have you assessed severity, likelihood, and detectability, and prioritized mitigations by the combined risk?
- Have you analyzed cascade paths (retry storms, shared-pool saturation) and common-cause failures (shared zone, provider, software, credentials)?
- Does each chosen mitigation reduce net risk, with new failure modes it introduces explicitly considered?
- Has redundancy (replicas, failover, backups) been tested under realistic conditions, not just assumed to work?
- Are operational and human failure modes (deploy, config, secret/cert expiry, runbook readiness, capacity headroom) included in the analysis?
- Is there capacity headroom to absorb the loss of a replica or a traffic spike without cascading?
- Are failure modes that lack runbooks or alerts identified, with documentation and monitoring added?
- Is the analysis revisited on architectural changes and after incidents, rather than treated as one-time?
