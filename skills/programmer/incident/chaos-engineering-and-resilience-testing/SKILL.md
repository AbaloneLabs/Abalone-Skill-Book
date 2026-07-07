---
name: chaos_engineering_and_resilience_testing.md
description: Use when the agent is designing or running chaos engineering experiments, game days, or resilience testing (fault injection, dependency failure simulation, network partition and latency injection, killing instances or pods), defining steady-state hypotheses, scoping blast radius and abort conditions, choosing what failure modes to validate, or building confidence in a distributed system's resilience before a real outage forces the question. Also covers the failure modes of chaos experiments that are unsafe or unscoped (causing real customer impact), experiments with no steady-state hypothesis (so results are uninterpretable), testing only the easy failures and ignoring correlated or cascading ones, treating chaos as a one-off event rather than a continuous practice, and the recurring mistake of assuming redundancy means resilience without ever validating recovery under failure.
---

# Chaos Engineering And Resilience Testing

Redundancy on paper is not resilience in practice. A system can have multiple replicas, automated failover, retries, and circuit breakers in its design, and still fail catastrophically when a real dependency goes down — because the failover was never exercised, the retry storm amplified the load, the circuit breaker's threshold was wrong, or a hidden coupling surfaced only under failure. Chaos engineering is the discipline of deliberately injecting failure into a running system to discover, before a real outage does, how it actually behaves when things break. The judgment problem is that resilience cannot be inferred from the architecture diagram; it must be validated empirically, and validating it safely requires scoping the experiment, defining what "healthy" means (the steady-state hypothesis), controlling the blast radius, and being honest that the most important failures are the correlated, cascading ones, not the isolated single-point failures that are easy to test.

Agents tend to under-invest here because the architecture looks resilient (it has replicas and retries) and because chaos experiments feel risky (what if we break production?). The harm is misplaced confidence. A team believes failover is automatic and discovers, during a real outage, that it had never been triggered and the DNS switch takes 20 minutes. A retry policy that looked sensible amplifies a downstream failure into a retry storm. A dependency marked "non-critical" turns out to be on the critical path for login. A single-zone failure cascades because capacity was sized for steady state, not for the loss of a zone. The judgment problem is to treat resilience as an empirical property to be validated, to choose experiments that probe the failure modes that actually matter (correlated and cascading), to scope them so they cannot cause unbounded customer impact, and to close the loop by fixing the weaknesses found rather than treating the experiment as a checkbox.

This skill covers steady-state hypotheses, experiment design and scoping, failure-mode selection, blast-radius control, and operationalizing chaos as a continuous practice. It complements the incident-response skill (handling real outages), the recovery-and-failover skill (failover design), and the slo-and-error-budget skill (the reliability target chaos helps validate). Here the focus is proactively validating resilience through controlled failure injection.

## Core Rules

### Define A Steady-State Hypothesis Before Injecting Failure

An experiment without a hypothesis is uninterpretable: you inject a failure, something happens, and you cannot say whether the system "passed." The steady-state hypothesis defines what healthy looks like and what the experiment tests:

- **Define steady state as measurable, customer-facing signals.** Steady state is not "the servers are up"; it is the SLOs, error rates, latency, throughput, and user-facing availability that define acceptable behavior. These are the signals the experiment will perturb and observe.
- **State the hypothesis explicitly.** "If we kill one replica of service X, the error rate will stay below the SLO and recover within N seconds, because of automated health checks and failover." The hypothesis names the expected behavior and the mechanism expected to produce it.
- **The experiment tests the hypothesis, not just "does it break."** A well-designed experiment either confirms the system behaved as designed (resilience validated) or reveals it did not (a weakness found). Both outcomes are valuable; an undefined experiment yields neither.
- **Abort if steady state is breached beyond the abort threshold.** Define, in advance, the error-rate or latency threshold at which the experiment is rolled back, so a finding does not become a customer incident.

### Scope The Blast Radius And Control The Experiment

Chaos experiments perturb real systems, and uncontrolled experiments cause real outages. Scoping the blast radius is non-negotiable:

- **Start small and expand.** Begin with a single instance, a small fraction of traffic, or a staging environment, and expand only as the system demonstrates resilience at each scale. Do not start by killing a third of production.
- **Target a bounded slice, not the whole fleet.** Inject failure into a specific instance, availability zone, or a small percentage of requests, so the impact is bounded and observable, not fleet-wide.
- **Define and automate abort conditions and rollback.** Before starting, define the conditions that trigger immediate rollback (error rate exceeds X, latency exceeds Y, a critical downstream degrades), and ensure rollback is automated and tested. A manual rollback discovered mid-experiment to be broken turns a finding into an incident.
- **Schedule and staff experiments.** Run during business hours with engineers present who can diagnose and roll back, not during off-hours unattended. Have a game-day structure: a facilitator, observers, and a clear stop condition.
- **Communicate to avoid false alarms.** An experiment that spikes error rates looks like a real incident to on-call and to monitoring; coordinate with on-call and clearly mark experiment traffic so it does not trigger a confused incident response.

### Choose Failure Modes That Probe What Actually Matters

The easy experiments (kill one pod) test the obvious. The valuable experiments probe the correlated, cascading, and hidden-coupling failures that cause real outages:

- **Test dependency failures, not just component failures.** Real outages are often a dependency (a database, a downstream API, an identity provider, a third-party service) failing or degrading. Inject latency, errors, or unavailability into dependencies to validate timeouts, circuit breakers, fallbacks, and graceful degradation.
- **Test correlated failures, not only independent ones.** A single instance failing is easy; a zone failing (taking instances, a slice of a dependency, and a load balancer node together) probes correlated failure that reveals hidden coupling and capacity shortfalls. Design experiments for the failure correlations that actually occur.
- **Test cascading and amplification paths.** Inject a downstream latency and observe whether retries amplify load, whether queues back up, whether timeouts cascade. The most damaging outages are cascades; test the mechanisms (retries, backpressure, shared resource exhaustion) that turn a small failure into a large one.
- **Test the "non-critical" dependencies.** A dependency assumed non-critical may sit on the critical path; inject its failure and find out. Assumptions about criticality are exactly what chaos validates.
- **Include network-level failures.** Packet loss, partitions, DNS failures, and clock skew are real and rarely tested. They reveal assumptions about connectivity and timing that component-level tests miss.

### Validate Recovery, Not Just Survival

Surviving a failure (staying up) is only half of resilience; recovering to full health (restoring capacity, clearing backlog, re-establishing steady state) is the other half:

- **Observe the recovery, not only the immediate impact.** After injecting and removing a failure, does the system return to steady state? How long does recovery take? Does it need manual intervention, or is it automated?
- **Check for stuck or degraded states after recovery.** A system may "survive" but remain degraded (reduced capacity, a backlog that drains slowly, a circuit breaker that never closes). Recovery to full health, not merely survival, is the goal.
- **Validate failover and failback.** Failover to a standby is tested; failback (returning to primary) is often not, and is where latent issues surface. Test the full failover-and-return cycle, not only the switch away.
- **Test data consistency after recovery.** After a failover or a partition, is data consistent across replicas and stores? A system that survives but splits-brain or loses committed data has not truly recovered.

### Treat Chaos As A Continuous Practice, Not A One-Off Event

A single game day finds some weaknesses; resilience is sustained by continuously exercising failure, so the system's behavior under failure is always known and always improving:

- **Automate experiments and run them regularly.** Move from manual game days to automated, scheduled experiments (in bounded scope) that run continuously, so regressions in resilience are caught as the system evolves.
- **Integrate findings into the system and its design.** A chaos finding that is not fixed is a known weakness left in production. Track findings as work, fix the root cause, and re-experiment to confirm the fix.
- **Evolve experiments as the system changes.** New dependencies, new couplings, and new scaling points introduce new failure modes; the experiment suite must evolve to probe them, or it validates yesterday's system.
- **Build resilience into the deployment pipeline.** Use the lessons to make resilience testable in pre-production (fault injection in staging, CI-level resilience checks), so weaknesses are caught before production.

### Be Honest About What Chaos Cannot Replace

Chaos engineering validates resilience under injected failure; it does not replace other reliability practices:

- **Chaos does not replace good design.** Redundancy, isolation, timeouts, bulkheads, and graceful degradation must be designed in first; chaos validates whether they work.
- **Chaos does not replace monitoring and incident response.** Detecting and responding to real, unanticipated failures still requires strong observability and incident process (see the incident-response and observability skills).
- **Chaos findings are conditional on the experiment.** An experiment that passes validates resilience for the specific failure injected, under the conditions tested; it does not certify the system against all failures. Communicate the scope of what was validated.

## Common Traps

### An Experiment With No Steady-State Hypothesis

Injecting a failure and observing "something happened," with no defined healthy state or expected behavior, so the result is uninterpretable. Define the steady-state hypothesis and the mechanism expected to preserve it before injecting.

### Unscoped Experiments Causing Real Customer Impact

Running a fault injection without blast-radius limits, abort conditions, or tested rollback, turning a finding into an outage. Start small, bound the target, automate abort, staff the experiment, and coordinate with on-call.

### Testing Only Easy, Independent Failures

Killing single pods repeatedly while never testing correlated zone failures, dependency degradations, or cascading retry storms — the failures that cause real outages. Probe correlated, cascading, and hidden-coupling failure modes.

### Assuming Redundancy Means Resilience Without Validating Recovery

Believing replicas and failover guarantee resilience, without ever exercising failover or checking recovery to full health. Validate failover, failback, backlog clearing, and data consistency, not just survival.

### A Finding That Is Never Fixed

A chaos experiment reveals a weakness (a missing timeout, a wrong circuit-breaker threshold) and the finding is logged but not remediated, leaving a known weakness in production. Track findings as work, fix root causes, and re-experiment to confirm.

### Chaos As A One-Off Event

A single game day that is run once and never repeated, so resilience regresses silently as the system evolves. Automate and schedule experiments continuously and evolve the suite with the system.

### False Confidence From Passing The Wrong Experiments

Treating a passed experiment as certification that the system is resilient against all failures, when it validated only a specific failure under specific conditions. Communicate the scope and keep testing the failures that matter.

## Self-Check

- [ ] Each experiment has a defined steady-state hypothesis (measurable, customer-facing SLOs/error/latency/availability signals) and a stated expected behavior and mechanism, so the result is interpretable as pass or weakness-found.
- [ ] The blast radius is scoped: experiments start small (single instance, small traffic slice, staging) and expand only as resilience is demonstrated, with automated abort conditions and tested rollback defined in advance.
- [ ] Experiments are scheduled, staffed (facilitator, observers, stop condition), run during business hours, and coordinated with on-call and clearly marked, so they do not trigger a confused real incident response.
- [ ] Failure modes probe what matters: dependency failures and degradations, correlated (zone-level) failures, cascading/amplification paths (retry storms, backpressure), "non-critical" dependencies on the critical path, and network-level failures (partitions, DNS, clock skew) — not only easy independent component failures.
- [ ] Recovery is validated, not only survival: return to steady state, time to recover, need for manual intervention, failover and failback, backlog clearing, and data consistency across replicas after a partition or failover.
- [ ] Chaos is a continuous practice: experiments are automated and scheduled regularly, findings are tracked and remediated (root cause fixed, re-experimented to confirm), the suite evolves with the system, and resilience checks are integrated into pre-production and CI.
- [ ] The limits are understood: chaos validates injected-failure resilience and does not replace good design, monitoring, or incident response; findings are communicated as conditional on the specific failure and conditions tested.
- [ ] The highest-risk cases were verified — a correlated zone failure, a dependency degradation that triggered a retry cascade, a failover that had never been exercised, and a recovery that left the system degraded — not only the easy single-pod kill.
