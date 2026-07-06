---
name: robustness_and_defensive_design.md
description: Use when the agent is designing for robustness, deciding how a system should handle unexpected invalid and adversarial input, building defensive behaviors that prevent cascading failures, or ensuring designs tolerate real-world conditions without breaking corrupting data or compromising safety.
---

# Robustness And Defensive Design

Robustness is the property that a system continues to function acceptably under conditions that were not fully anticipated: unexpected input, invalid states, component failures, and adversarial behavior. Defensive design is the practice of building robustness into the system, by validating input, containing failures, avoiding assumptions, and designing conservatively for the conditions the system will actually face. Done well, defensive design produces systems that tolerate real-world conditions without breaking, corrupting data, or compromising safety. Done poorly, systems are fragile, failing catastrophically when conditions deviate from the happy path. Agents often design for the expected, because it is what they can see, leaving the system vulnerable to the unexpected conditions that real users and real environments produce.

The harm this skill prevents is the fragile system that breaks under real conditions. A system that assumes input is always valid crashes on malformed input. A system that assumes dependencies are always available fails when they are not. A system that assumes users behave cooperatively is exploited by those who do not. These fragilities produce outages, data corruption, security breaches, and user harm, all because the design did not account for the conditions the system actually faces. Defensive design builds in the tolerance that prevents these failures.

Use this skill before answering questions such as "how do we make this system robust", "how do we handle invalid input", "how do we prevent cascading failures", or "how do we design for real-world conditions". The goal is to prevent the agent from designing for the happy path and leaving robustness to be added later, when it is expensive or impossible.

## Core Rules

### Validate Input At Trust Boundaries

Input validation is the first line of defense against unexpected conditions, and it should occur at every trust boundary, where data crosses from an untrusted source to a trusted system. External input, from users, from APIs, from files, from network messages, should be validated before it is processed, to ensure it conforms to expected types, ranges, formats, and constraints. Validation at trust boundaries prevents invalid input from propagating into the system, where it can cause failures, corruption, or security vulnerabilities.

Validation should be comprehensive, checking type, range, format, length, and semantic constraints. It should fail closed, rejecting input that does not validate rather than attempting to process it, because processing invalid input is where failures and vulnerabilities arise. And it should provide useful feedback, so that legitimate users can correct their input, while not revealing information that helps adversaries. Treat all external input as untrusted until validated, because the cost of validating is far lower than the cost of processing invalid input.

### Avoid Assumptions About State And Preconditions

Fragile systems are full of implicit assumptions: that a referenced entity exists, that a field is populated, that a precondition was checked elsewhere, that the system is in the expected state. These assumptions hold in the happy path and fail in edge cases, producing null pointer errors, inconsistent state, and unexpected behavior. Defensive design makes assumptions explicit and checks them, verifying preconditions before acting and handling the cases where assumptions do not hold, rather than allowing them to produce failures.

This does not mean checking every possible condition everywhere, which is impractical and produces bloated code. It means identifying the assumptions that, if wrong, would cause significant failure, and checking those. Focus on assumptions whose violation is plausible, because the system interacts with untrusted input or unreliable dependencies, and whose consequences are severe, because failure would corrupt data, breach security, or cause outages. Check these assumptions defensively, and design the behavior for when they fail, so that the system degrades rather than breaks.

### Contain Failures To Prevent Cascading

Robust systems contain failures, preventing a failure in one component from cascading to affect others. Failure containment is achieved through isolation: components are isolated so that a failure's effects are limited to the component and do not propagate. This isolation may be process-level, through separate processes that can fail and restart independently; it may be circuit-breaker-level, through mechanisms that stop calling a failing dependency; or it may be design-level, through boundaries that prevent one component's bad state from affecting another. Design for containment, so that failures are local rather than system-wide.

Cascading failures are among the most dangerous, because a single failure triggers others, bringing down the entire system. A slow dependency causes threads to pile up, exhausting resources and bringing down the system. A failing component causes retries that overload other components, spreading the failure. Design specifically against cascading, through timeouts that prevent waiting indefinitely, circuit breakers that stop calling failing dependencies, bulkheads that isolate resources so one failure cannot exhaust them all, and backpressure that prevents overload. These patterns, applied at the right boundaries, contain failures and preserve overall system function.

### Design Conservatively For Real-World Conditions

Defensive design means designing for the conditions the system will actually face, not for idealized conditions. Real-world conditions include unreliable networks, slow dependencies, partial failures, heavy load, malformed input, and adversarial behavior. Design conservatively, assuming that these conditions will occur and ensuring the system tolerates them. This conservatism may mean lower limits, more validation, more redundancy, or more conservative defaults than a happy-path design would choose, and it is the price of robustness.

Conservatism should be calibrated to the stakes. A system whose failure would cause significant harm, to safety, to data integrity, to revenue, deserves more conservative design than a system whose failure is easily corrected. Apply defensive measures where the stakes warrant them, and accept more risk where they do not. This calibration prevents both over-engineering, which wastes effort on robustness that is not needed, and under-engineering, which leaves critical systems fragile. The calibration is a judgment call, informed by the consequences of failure.

### Protect Data Integrity Under All Conditions

Data integrity is among the most important properties to defend, because data corruption persists and propagates, causing harm long after the failure that caused it. Design to protect data integrity under all conditions, including failures, concurrency, and partial operations. This means using transactions or equivalent mechanisms to ensure operations are atomic, either completing fully or not at all; using constraints and validation to prevent invalid data from being stored; and designing recovery to restore consistency after failures, rather than leaving the data in a corrupted state.

Concurrency is a particular threat to data integrity, because concurrent operations can interleave in ways that produce inconsistent state. Design for the concurrency the system will face, using appropriate locking, versioning, or conflict resolution to ensure that concurrent operations do not corrupt data. Test specifically for concurrency edge cases, because they are hard to reproduce and often missed. Data integrity, once lost, is expensive to restore, so defending it through design is far more cost-effective than recovering it after corruption.

### Consider Adversarial Conditions, Not Just Accidental Ones

Robustness includes tolerance of adversarial conditions, where users or external actors deliberately attempt to break, exploit, or overload the system. Accidental conditions, malformed input from a legitimate user, are one concern; adversarial conditions, crafted input designed to exploit a vulnerability, are another. Defensive design considers both, validating input to reject not just accidentally invalid data but deliberately malicious data, rate-limiting to prevent abuse, and designing authentication and authorization to resist bypass.

Adversarial thinking requires considering what an attacker might do, not just what a normal user would do. This includes injection attacks, where input is crafted to execute unauthorized logic; overflow attacks, where input is designed to exhaust resources; replay attacks, where valid messages are captured and replayed; and escalation attacks, where a user attempts to access more than they are authorized for. Design defenses for these, treating the system as operating in an environment that includes adversaries, because most real systems do. Ignoring adversarial conditions leaves the system vulnerable to exploitation that can cause severe harm.

## Common Traps

### Input Validation Only At The UI, Not At Trust Boundaries

Validating in the interface but trusting internal calls. The trap is invalid input reaching trusted code through other paths.

### Implicit Assumptions That Hold Until They Do Not

Assuming entities exist, fields are populated, preconditions hold. The trap is failures when the assumptions are violated in edge cases.

### No Failure Containment, Enabling Cascades

Components tightly coupled so one failure spreads. The trap is single failures bringing down the entire system.

### Designing For Idealized Conditions

Assuming reliable networks, fast dependencies, well-behaved input. The trap is fragility under the real conditions the system faces.

### Data Integrity Assumed Rather Than Designed

Assuming operations are atomic and concurrency is safe. The trap is corruption under failures and concurrent access.

### Ignoring Adversarial Conditions

Designing only for cooperative users. The trap is vulnerabilities that adversaries exploit to break or abuse the system.

## Self-Check

- [ ] Input is validated at every trust boundary, failing closed and providing appropriate feedback.
- [ ] Critical assumptions about state and preconditions are made explicit and checked defensively.
- [ ] Failures are contained through isolation, circuit breakers, bulkheads, and backpressure to prevent cascading.
- [ ] The system is designed conservatively for real-world conditions, calibrated to the stakes of failure.
- [ ] Data integrity is protected through atomic operations, constraints, and recovery design, including under concurrency.
- [ ] Adversarial conditions are considered, with defenses against injection, overflow, replay, and escalation attacks.
- [ ] The system tolerates unexpected, invalid, and adversarial input without breaking, corrupting data, or compromising safety.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
