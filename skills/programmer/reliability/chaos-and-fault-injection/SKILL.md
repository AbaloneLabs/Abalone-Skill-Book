---
name: chaos_and_fault_injection.md
description: Use when the agent is planning chaos engineering experiments or fault injection, controlling blast radius, forming and validating resilience hypotheses, automating experiments, running safe experiments in production, or verifying recovery and rollback procedures.
---

# Chaos and Fault Injection

Resilience that is never tested is theoretical. A system may have retries, circuit breakers, fallbacks, and failover, but if those mechanisms have never been exercised against a real failure, you do not know whether they work, whether they were configured correctly, or whether they compose safely under a real outage. Chaos engineering and fault injection are the practices of deliberately injecting failures to validate resilience hypotheses before an uncontrolled outage does it for you. Done well, they surface hidden coupling, misconfigured failover, and missing alerts while the team is watching. Done badly, they cause the very outages they were meant to prevent.

The judgment problem is forming a meaningful hypothesis, choosing a fault that tests it, bounding the blast radius so an experiment cannot take down production, automating for repeatability, and knowing when to run in production versus staging. The agent should not equate "break things randomly" with chaos engineering; disciplined chaos is hypothesis-driven, bounded, and reversible.

This skill applies whenever you are designing resilience validation, planning a game day or chaos experiment, or deciding how to verify that failure mechanisms actually work.

## Core Rules

### Start from a hypothesis, not from "let's break something"

A chaos experiment without a hypothesis is just vandalism. Begin by stating what you expect to happen when a specific failure occurs, and what would constitute a violation of that expectation:

- "If the primary database fails over, the application should recover within 30 seconds with no data loss and no user-visible errors beyond brief latency."
- "If the recommendation service is unavailable, the page should render with popular-item fallback within 500ms, and no request should fail."

The hypothesis defines the steady-state (the metrics/SLOs you expect to hold), the fault you inject, and the pass/fail criteria. An experiment that does not have a defined steady-state and pass/fail criteria cannot tell you anything useful.

### Define and control the blast radius

The single most important safety property is that an experiment cannot cause more damage than intended. Bound the blast radius explicitly:

- **Scope**: limit the experiment to a subset of traffic, a single availability zone, a single service instance, or a fraction of users. Use canary segments or service-level targeting.
- **Abort conditions**: define in advance the conditions under which you immediately stop the experiment (error rate exceeds X%, latency exceeds Y, a critical metric degrades). These should be automated, not dependent on a human noticing.
- **Reversibility**: ensure the fault can be removed quickly (kill the injection process, restore the instance, flip the flag). Prefer faults with automatic time bounds (the injection stops after N minutes regardless).
- **Game-day vs automated**: start with a scheduled, staffed game day where humans monitor and can abort, before moving to automated experiments.

Weak choice: injecting a fault across the whole fleet with no abort condition. Strong choice: injecting into one instance or one canary segment with automated abort thresholds and a hard time bound.

### Choose faults that test the specific hypothesis

Match the fault to the resilience property you are validating:

- **Network faults**: latency injection, packet loss, DNS failures, connection drops. Tests timeouts, retries, and circuit breakers.
- **Dependency failures**: kill a dependency, return errors from a mock, or make it slow. Tests fallback and degradation.
- **Resource exhaustion**: fill disk, exhaust memory, saturate CPU, exhaust connection pools. Tests backpressure, load shedding, and graceful degradation.
- **Instance/zone failure**: terminate instances, fail over a database, blackhole an AZ. Tests failover, redundancy, and recovery procedures.
- **Time/clock skew**: shift a clock, simulate leap seconds. Tests time-dependent logic and certificate validity.

Injecting a fault that does not test the hypothesis wastes the experiment and the risk.

### Validate the full recovery, not just the failure response

The experiment is not over when the fault is injected and the system absorbs it. You must also verify recovery:

- Does the system return to steady-state after the fault is removed?
- Do queues drain, circuit breakers close, and fallbacks disengage?
- Are there lingering effects (stale caches, half-open connections, accumulated state) that degrade behavior after recovery?
- Did the alerts fire correctly, and did the runbook/on-call respond appropriately?

A system that survives the fault but never recovers cleanly is still broken.

### Move from staging to production deliberately

Staging-only chaos has limited value because staging rarely matches production's topology, load, and failure modes. But production chaos carries real risk. A typical progression:

1. **Staging/game days**: validate the experiment mechanics and the hypothesis in a safe environment first.
2. **Off-peak production, single instance**: minimal blast radius, staffed.
3. **Business-hours production, larger scope**: increase scope and confidence as the system proves resilient.
4. **Automated, continuous chaos**: small, frequent, bounded experiments run automatically as part of CI/operations, catching regressions in resilience over time.

Do not skip to automated production chaos before the team has confidence in abort mechanisms and recovery.

### Automate for repeatability and regression detection

Manual one-off experiments are forgotten; the resilience they validated erodes as the system changes. Automate experiments so they run repeatedly:

- Encode the steady-state checks, fault injection, abort conditions, and recovery validation as code.
- Run small experiments continuously (or on a schedule) so resilience regressions are caught early.
- Version experiments and track results over time, so a previously-passing experiment that starts failing signals a real regression.

### Close the loop on findings

Chaos experiments that surface a weakness are valuable only if the weakness is fixed:

- Every experiment that reveals unexpected behavior produces an action item (fix the code, fix the config, fix the alert, update the runbook).
- Track action items to completion. An unfixed finding means the known weakness persists in production.
- Re-run the experiment after the fix to confirm the weakness is resolved.

## Common Traps

### Breaking things without a hypothesis

Random fault injection without a defined steady-state and pass/fail criteria produces noise and risk without knowledge. It is not chaos engineering; it is carelessness.

### Unbounded blast radius

Injecting a fault across the whole system with no abort condition can cause a real outage. The blast radius must be explicitly bounded and automatically reversible.

### Chaos only in staging

Staging does not match production topology, load, or failure modes, so staging-only chaos gives false confidence. Production chaos (with proper bounding) is where real resilience is proven.

### Validating the failure but not the recovery

A system that absorbs a fault but never returns to steady-state (stuck circuit breakers, drained-then-not-refilled pools, lingering stale caches) is still broken. Recovery is part of the experiment.

### No automated abort conditions

Relying on a human to notice degradation and stop the experiment is unreliable under stress. Abort thresholds must be automated and fast.

### One-off experiments that are never repeated

Resilience validated once erodes as the system evolves. Without automation and repetition, the experiment's value decays to zero.

### Findings that are never fixed

Surfacing a weakness and then not fixing it means the known vulnerability persists. The experiment is only valuable if the loop is closed.

### Ignoring alerting and operational response

Chaos validates not just the system's automatic resilience but the operational response. If the alerts did not fire or the on-call did not know what to do, the operational resilience is broken even if the system self-healed.

## Self-Check

- Does every experiment start with a clear hypothesis, a defined steady-state (metrics/SLOs), and explicit pass/fail criteria?
- Is the blast radius explicitly bounded (subset of traffic, single instance/zone/canary) with automated abort conditions and a hard time bound?
- Was the chosen fault matched to the specific resilience property being tested, rather than injected arbitrarily?
- Does the experiment validate full recovery (return to steady-state, circuit breakers closing, fallbacks disengaging, no lingering effects), not just failure absorption?
- Has the team progressed deliberately from staging to off-peak production to business-hours to automated chaos, rather than jumping to automated production experiments?
- Are experiments automated and repeated so resilience regressions are caught over time?
- Did the experiment verify that alerts fired and the operational runbook/response was adequate, not just that the system self-healed?
- Is every finding from an experiment tracked to a completed fix, with a re-run confirming the fix?
- Are abort mechanisms tested themselves (can you actually stop the injection quickly)?
- Before running in production, have you confirmed the experiment is reversible and that a worst-case outcome is acceptable given the blast radius?
