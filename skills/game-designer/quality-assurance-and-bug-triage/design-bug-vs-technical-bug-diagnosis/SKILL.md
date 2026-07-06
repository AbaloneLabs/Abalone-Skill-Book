---
name: design-bug-vs-technical-bug-diagnosis.md
description: Use when the agent is diagnosing whether a game issue is a design flaw or a technical bug, routing issues between design and engineering, distinguishing intended-but-bad behavior from unintended behavior, or resolving disputes over whether observed behavior is working as designed.
---

# Design Bug vs Technical Bug Diagnosis

When something feels wrong in a game, the first and most consequential question is whether the behavior is a technical bug, the system failing to do what it was designed to do, or a design bug, the system doing exactly what it was designed to do but the design itself being wrong. The judgment problem is that these two categories feel identical from the player's perspective, they are fixed by entirely different disciplines, and misdiagnosing one as the other sends the issue to the wrong team who either cannot fix it or fixes the wrong thing. Agents miss the distinction because the symptom is the same, the spec is often ambiguous enough to support either reading, and the political pressure to label a problem a technical bug, which feels like an accident, rather than a design bug, which feels like a mistake, biases the diagnosis. This skill covers how to determine what the system was supposed to do, how to distinguish a failure of implementation from a failure of intent, and how to route the issue so it reaches the discipline that can actually resolve it. The designer has authority over the design-bug determination but must diagnose jointly with engineering against the spec and the code.

## Core Rules

### First Determine What the System Was Designed to Do, Then Compare

The diagnosis cannot begin from the symptom; it must begin from the intended behavior. The rule is that before classifying an issue, the diagnoser must locate the design intent for the behavior in question, whether in the spec, the GDD, or a recorded decision, and compare the observed behavior to that intent. If the observed behavior matches the intent and the intent produces a bad experience, it is a design bug: the system is working as designed and the design is wrong. If the observed behavior does not match the intent, it is a technical bug: the system is failing to implement the design. The trap is diagnosing from the symptom alone, which collapses both categories into "feels wrong" and routes the issue arbitrarily. When the intent is undocumented or ambiguous, the first task is to reconstruct or decide the intent, because without it no diagnosis is possible.

### A Design Bug Is Correct Behavior Under a Wrong Design

The defining feature of a design bug is that the code is doing exactly what it was asked to do, and the problem is that what it was asked to do produces a bad player experience. An overpowered weapon is a design bug if the damage values match the spec; a frustrating encounter is a design bug if the spawn logic matches the design. The rule is to verify that the implementation matches the spec before concluding the issue is design, because a technical bug that happens to match a bad-feeling outcome will be mislabeled as design and sent to a team that cannot fix it. The discipline is to confirm the implementation is correct first, then to evaluate whether the design itself needs to change. When the implementation is correct and the experience is bad, the fix is a design change, not a code change.

### A Technical Bug Is the System Failing to Match Its Design

The defining feature of a technical bug is that the code is not doing what the design specified. A weapon that deals the wrong damage because of a calculation error, a trigger that fails to fire, a state that transitions incorrectly, an animation that desyncs from the logic. The rule is to confirm the spec, confirm the observed behavior diverges from it, and route to engineering with the divergence documented. The trap is assuming any divergence is trivially technical when some divergences are actually underspecified behavior the engineer filled in ad hoc, in which case the root issue is a design gap that produced an arbitrary implementation. Distinguish a true implementation failure from an underspecification that led to a reasonable but wrong guess.

### Check Whether the Spec Is Underspecified Before Assigning Blame

A large fraction of issues are neither clean design bugs nor clean technical bugs but the result of a spec that did not cover the case, leaving the engineer to implement something reasonable that turned out wrong. The rule is that when the behavior is bad and the spec is silent on the case, the root cause is a design gap, and the fix is to decide the intended behavior and update both the spec and the implementation. The trap is routing these to engineering as technical bugs, when engineering implemented the only reasonable interpretation of an incomplete spec, or routing them to design as design bugs, when the design was simply never asked to decide. Name underspecification as its own category so the fix addresses the actual gap rather than patching a symptom.

### Route by Root Cause, Not by Which Team Is Less Busy

Under pressure, issues get routed to whichever discipline has capacity or whichever is less likely to push back, rather than to the discipline that can fix the root cause. The rule is that routing must follow the diagnosis: design bugs to design, technical bugs to engineering, underspecification to design for a decision then to engineering for implementation. The trap is convenience routing, which sends the issue to a team that can only treat the symptom, producing a patch that does not hold or a fix that masks the real problem. When the root cause is misrouted, the issue recurs or mutates, because the discipline that received it could not address what actually caused it.

### Verify the Fix Addresses the Root Cause, Not the Symptom

A common failure is fixing the most visible symptom of an issue without addressing the underlying cause, so the symptom disappears but the cause produces a different symptom elsewhere. The rule is that every fix should be evaluated against the root cause identified in diagnosis: did the fix change the design if it was a design bug, the implementation if it was a technical bug, the spec if it was underspecification? The trap is the fix that makes the reported case go away, which closes the ticket but leaves the cause intact to generate the next report. When a fix only addresses the symptom, the issue will recur in a form the team does not immediately recognize as the same root cause.

### Document the Diagnosis So the Same Confusion Does Not Recur

When an issue's classification is non-obvious or contested, the reasoning that resolved it must be recorded, so that the next similar issue is diagnosed consistently rather than re-litigated. The rule is that for any issue where the design-versus-technical distinction was unclear, the ticket should note what the intent was determined to be, why the behavior was classified as it was, and what the root cause was. The trap is resolving the issue privately and leaving no trace, so the team re-argues the same distinction on the next similar bug. Documentation turns a one-off judgment into a reusable precedent that speeds future diagnosis.

## Common Traps

### Labeling Everything a Technical Bug to Avoid Design Accountability

A feature feels bad, and the team calls it a technical bug because bugs feel like accidents no one is responsible for, whereas a design bug feels like a mistake someone made. The trap is that this misroutes the issue to engineering, who will investigate the code, find it matches the spec, and bounce it back, wasting a cycle. The false signal is the comfort of the bug label. The harm is delayed resolution, because the issue traveled to the wrong discipline and back, and the erosion of the diagnostic discipline that lets future issues be routed correctly.

### Calling an Underspecified Case a Design Bug When It Was Never Designed

An engineer implemented a reasonable behavior for a case the spec never covered, the behavior feels wrong, and the team labels it a design bug on the theory that design must have intended something. The trap is that design never intended anything because the case was never considered, so calling it a design bug implies a decision that was never made. The false signal is that the behavior is bad, which feels like it must contradict some intent. The harm is that the fix is treated as changing an existing design decision when it is actually making one for the first time, and the spec gap that caused the problem goes unaddressed.

### Fixing the Symptom and Declaring the Bug Resolved

The reported case stops reproducing after a fix, the ticket closes, and the root cause continues to generate the same class of problem in other forms. The trap is that symptom-level fixes look like resolution because the specific report disappears, while the cause remains active. The false signal is the closed ticket and the non-reproducing case. The harm is recurring issues that the team does not recognize as related, each treated as new, each consuming diagnosis effort, while the underlying cause is never addressed.

### Assuming the Spec Is Correct Because It Exists

The diagnoser finds a spec that covers the behavior, sees the implementation matches it, and concludes the behavior is correct by definition. The trap is that the spec itself may encode a bad design decision, and matching a bad spec is still a design bug, because the design is wrong even if it was documented. The false signal is the existence of documentation, which feels authoritative. The harm is that bad-but-specified behavior is shielded from redesign because it is mistaken for correct behavior, and the player experience suffers from a decision nobody is willing to revisit.

### Routing by Capacity Rather Than Root Cause

The issue is assigned to whichever discipline has free cycles, regardless of where the root cause lives, because the goal appears to be keeping everyone busy and the backlog moving. The trap is that the receiving discipline can only treat the symptom, so the issue is partially addressed or bounced, and the real cause waits. The false signal is the moving backlog. The harm is wasted effort on misrouted issues, delayed resolution of the actual cause, and frustration on the team that received work it could not properly complete.

### Treating the First Plausible Diagnosis as Final

The diagnoser forms a quick hypothesis, it seems to fit, and the issue is routed and fixed without verifying the hypothesis against the spec and the code. The trap is that the first plausible explanation is often a symptom-level story that feels satisfying but misses the root cause, and acting on it produces a fix that does not hold. The false signal is the coherence of the initial explanation. The harm is a fix that closes the ticket but not the problem, leading to recurrence and to misplaced confidence that the issue was resolved.

## Self-Check

- Before classifying the issue, did I locate the design intent in the spec or a recorded decision and compare the observed behavior to that intent?
- For issues classified as design bugs, have I confirmed the implementation matches the spec, so the fix is a design change rather than a misrouted code fix?
- For issues classified as technical bugs, have I confirmed the behavior diverges from a clear spec, rather than reflecting an engineer's reasonable guess at an underspecified case?
- Where the spec is silent on the observed case, have I named underspecification as the root cause and routed to design for a decision before implementation?
- Is each issue routed to the discipline that owns the root cause, rather than to whichever team has capacity or is less likely to push back?
- Does the fix address the identified root cause, changing the design for design bugs, the implementation for technical bugs, and the spec for underspecification, rather than only suppressing the reported symptom?
- For non-obvious or contested classifications, is the diagnostic reasoning documented so future similar issues are diagnosed consistently without re-litigation?
