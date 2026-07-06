---
name: failure_mode_analysis.md
description: Use when the agent is analyzing how a system or feature can fail, mapping failure modes to their causes and effects, deciding how the system should detect respond and recover from each failure, or ensuring designs degrade gracefully rather than catastrophically when components and dependencies break.
---

# Failure Mode Analysis

Failure mode analysis is the practice of systematically identifying how a system can fail, the causes and effects of each failure, and the design responses that make failure tolerable. Every system fails: components break, dependencies become unavailable, errors occur, and resources are exhausted. The question is not whether failure will happen but how the system behaves when it does. Done well, failure mode analysis produces designs that fail gracefully, degrading in ways that users can tolerate and recovering without harm. Done poorly, failure is an afterthought, and the system fails catastrophically, corrupting data, losing work, or becoming unavailable in ways that harm users and the business. Agents often design for the happy path and treat failure as an exception to handle in code, missing that failure behavior is a design decision that shapes the user experience.

The harm this skill prevents is the catastrophic failure that harms users. A system that loses user input when a save fails, that corrupts data when a dependency times out, or that becomes entirely unavailable when a single component breaks is a system that fails badly, because no one designed how it should fail. Failure mode analysis brings failure into the design, so that the system's failure behavior is intentional, tolerable, and recoverable, rather than accidental and harmful.

Use this skill before answering questions such as "how do we handle failures in this feature", "what happens when dependencies fail", "how do we design for graceful degradation", or "how should the system behave when things go wrong". The goal is to prevent the agent from treating failure as an implementation detail rather than a design decision.

## Core Rules

### Identify Failure Modes For Every Component And Dependency

Failure mode analysis begins with identifying how each component and dependency can fail. For each external dependency, an API, a database, a third-party service, consider the ways it can fail: it can be unavailable, slow, return errors, return unexpected data, or return partial data. For each internal component, consider how it can fail: it can throw exceptions, return incorrect results, exhaust resources, or enter bad state. Enumerate these failure modes systematically, because each represents a situation the design must address.

This enumeration should be comprehensive, because missed failure modes are unhandled failure modes. Use a checklist of failure types: unavailability, slowness, errors, unexpected data, partial data, resource exhaustion, and corruption. For each component and dependency, consider which failure types apply and what each looks like concretely. This systematic approach surfaces failure modes that ad hoc consideration misses, ensuring that the design addresses the full range of ways the system can break.

### Analyze Effects To Understand The Impact Of Each Failure

For each failure mode, analyze its effects: what happens to the system and to the user if this failure occurs? Effects range from minor, a slight delay, to severe, data loss or system unavailability. Understanding the effects is essential, because it determines how much design effort the failure warrants and what response is appropriate. A failure with minor effects may need only basic handling; a failure with severe effects demands robust design, including detection, response, and recovery.

Effect analysis should trace the failure through the system, considering not just the immediate effect but the propagated effects. A dependency failure that causes an error in one component may propagate to affect related components, user-facing features, and data integrity. Trace these propagation paths to understand the full impact, because the immediate effect may be minor while the propagated effect is severe. Design responses often target the propagation path, containing the failure before its effects spread.

### Design Detection So Failures Are Noticed, Not Silent

A failure that is not detected is a failure that persists and harms users. Design detection for each failure mode, specifying how the system recognizes that the failure has occurred. Detection may be explicit, through error codes and exceptions, or implicit, through timeouts, health checks, and anomaly detection. The detection mechanism should be reliable, noticing the failure promptly, and specific enough to distinguish the failure from normal variation, so that responses are triggered appropriately.

Silent failures are a particular danger. A dependency that returns stale data, a component that produces incorrect results without erroring, or a process that hangs without failing produces silent failure, where the system continues as if everything is fine while providing wrong or degraded results. Design detection for these cases specifically, through validation, freshness checks, and consistency verification, because they are the failures that harm users without anyone noticing. Assume that failures will be silent unless detection is designed for them.

### Design Response That Matches The Failure's Severity

The system's response to a failure should match the failure's severity and the user's needs in the situation. For minor failures, the response may be transparent: retry, fall back to a default, or degrade a non-essential feature. For severe failures, the response may need to be explicit: inform the user, preserve their work, and provide an alternative path. The response should be designed, not left to whatever the code happens to do, because the response shapes the user's experience of the failure and determines whether it is tolerable.

Key response patterns include: retry, for transient failures, with limits to avoid retry storms; fallback, for dependencies that can be substituted or defaulted; graceful degradation, for features that can be partially disabled; fail-fast, for failures where continuing would cause harm; and user notification, for failures the user must know about. Choose the pattern that fits the failure and the user's needs, and specify it in the design. The goal is that the user experiences a tolerable degradation, not a catastrophic failure.

### Design Recovery So The System Returns To A Good State

Detection and response handle the failure as it occurs; recovery returns the system to a good state afterward. For transient failures, recovery may be automatic: the system retries and succeeds, or the dependency recovers and the system resumes. For persistent failures, recovery may require intervention: a manual fix, a data reconciliation, or a process restart. Design the recovery path for each failure mode, specifying how the system returns to normal operation and what happens to any state that was affected during the failure.

Recovery design must account for the state the failure leaves behind. A failure mid-operation may leave partial side effects, inconsistent data, or orphaned resources. The recovery path must address these, restoring consistency and cleaning up, so that the system does not carry the failure's effects forward. This is especially important for operations with multiple steps or side effects, where a failure between steps can leave the system in a state that no step was designed to handle. Design recovery for these cases explicitly, because they are where failures cause lasting harm.

### Design For Graceful Degradation, Not Just All-Or-Nothing

Many systems are designed as all-or-nothing: either everything works or the system is unavailable. Graceful degradation is the alternative, where the system continues to provide value, reduced or partial, when components fail. A feature that depends on a recommendation service can show popular items when the service is unavailable. A feature that depends on real-time data can show cached data with a freshness indicator. Designing for graceful degradation means identifying which features are essential and which can be degraded, and specifying how the system behaves when each dependency fails.

Graceful degradation requires knowing what is essential. Identify the core features that must always work, and design them to have minimal dependencies, so that they remain available when other components fail. Identify the features that can be degraded, and specify the degraded behavior for each. This prioritization ensures that failure reduces capability rather than eliminating it, which is the difference between a system that users can still use during failures and one they cannot.

## Common Traps

### Happy Path Design With Failure As Afterthought

Designing only for success and handling failure in code. The trap is failure behavior that is accidental and often harmful.

### Missed Failure Modes

Not enumerating all the ways components and dependencies can fail. The trap is unhandled failures that surprise the team in production.

### Silent Failures Without Detection

Failures that persist without being noticed. The trap is wrong or degraded results that harm users without anyone knowing.

### Response That Does Not Match Severity

Over-reacting or under-reacting to failures. The trap is user experiences that are either alarming or harmfully silent.

### No Recovery Design

Handling the failure but not the aftermath. The trap is inconsistent state and lasting effects from failures that were detected but not recovered.

### All-Or-Nothing Instead Of Graceful Degradation

Treating any failure as total. The trap is systems that become entirely unavailable when a single component breaks.

## Self-Check

- [ ] Failure modes are identified for every component and dependency, using a systematic checklist of failure types.
- [ ] The effects of each failure are analyzed, tracing propagation to understand full impact.
- [ ] Detection is designed for each failure mode, including specific design for silent failures.
- [ ] The response matches the failure's severity and the user's needs, using appropriate patterns like retry, fallback, and degradation.
- [ ] Recovery is designed, addressing the state the failure leaves behind and restoring consistency.
- [ ] The system is designed for graceful degradation, with essential features protected and non-essential features degrading tolerably.
- [ ] Failure behavior is a deliberate design decision, not an accidental implementation outcome.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
