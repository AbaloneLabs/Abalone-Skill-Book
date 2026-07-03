---
name: assumption_validation_and_confirmation_tracking.md
description: Use when the agent is validating project assumptions, planning confirmation activities, tracking which assumptions remain unproven, deciding when an assumption must become a verified fact, or reviewing the evidence status of premises a delivery plan depends on.
---

# Assumption Validation and Confirmation Tracking

Logging an assumption is only the first move. The assumption continues to shape the plan until it is either confirmed as a fact or invalidated and converted into a risk, issue, or replanning trigger. Validation is the discipline of actively proving or disproving each premise before the plan depends on it for too long, and confirmation tracking is the discipline of knowing, at any moment, which assumptions are still unproven and how much of the plan rests on them. The judgment problem is that assumptions quietly age: a premise that was reasonable at week one can become dangerous by week six if no one has checked it, because the cost of being wrong compounds the later it is discovered.

Agents tend to treat logging as completion. Once an assumption is written down, it feels managed, so validation drifts and confirmation becomes occasional rather than systematic. The result is a register full of entries that are months old and still marked "open," each one a hidden liability. The skill is to drive assumptions toward closure on a schedule that reflects how much damage their falsity would cause, and to treat unproven load-bearing assumptions as active exposure, not as background information.

## Core Rules

### Drive Every Assumption Toward a Closed State

An assumption is open, confirmed, invalidated, or obsolete. The default should be movement toward closure. For each open assumption, define the specific evidence or event that will close it: a test result, a signed approval, a measured performance figure, a delivered artifact. "We will monitor" is not validation; it is deferred attention. The validation plan is the set of activities that will produce the closing evidence, owned by a named person with a target date.

### Sequence Validation by Consequence and Time-Criticality

Not all assumptions need confirmation at the same speed. Prioritize by the combination of how much breaks if the assumption is false and how soon the plan commits to depending on it. An assumption that underpins a decision next week must be validated before that decision; an assumption that affects a milestone six months out can be validated on a slower track. Build a validation calendar that front-loads the premises whose falsity would force the most expensive replanning.

### Match the Validation Method to the Type of Assumption

Different assumptions require different proof. A technical performance assumption needs a test, benchmark, or proof of concept. A regulatory assumption needs written confirmation from legal or the authority. A resource availability assumption needs a commitment from the resource owner. A market or adoption assumption needs evidence from pilots, surveys, or comparable data. Asking the wrong kind of question produces false confidence: a verbal "yes, that should be fine" is not confirmation of a load-bearing technical premise. Specify the method that would actually settle the question.

### Treat Unproven Load-Bearing Assumptions as Active Exposure

A confirmed assumption reduces plan risk; an open load-bearing assumption is an unresolved exposure that should be visible in status reporting. Quantify the exposure: which milestones, estimates, or decisions rest on still-open assumptions, and what is the fallback if one fails. Hiding open assumptions behind green status creates a false sense of security. Make the count and consequence of unproven premises part of the health picture, not a footnote.

### Define Explicit Confirmation Criteria Before Validation Begins

Before launching a validation activity, state what result will count as confirmation and what will count as invalidation. Ambiguous criteria lead to arguments after the evidence arrives, with stakeholders interpreting the same result differently. For "the API meets our latency target," define the target value, the test conditions, and the pass threshold. Pre-defined criteria turn validation from a judgment call into a clean decision, and they make it harder to rationalize a marginal result as a pass.

### Convert Invalidated Assumptions Promptly

When validation shows an assumption is false, do not leave it lingering as "open" while hoping for a workaround. Convert it immediately into a risk, an issue, or a replanning trigger, and assess which plan elements it undermines. The value of validation is realized only when invalidation changes the plan. A confirmed-false assumption that stays in the register unchanged is worse than no validation, because it creates the illusion that the problem is known and handled.

### Track Confirmation Status With a Clear Lifecycle

Maintain a small, consistent status set for each assumption: proposed, validation in progress, confirmed, invalidated, obsolete. Add the date of last status change and the evidence reference. This lifecycle makes it possible to answer, at any time, how many assumptions are still open, how long they have been open, and which are overdue their validation target. Aged open assumptions are a leading indicator of unmanaged exposure and should be flagged in reviews.

### Re-Validate Assumptions When Conditions Change

An assumption confirmed months ago can become invalid if the context shifts: a vendor changes ownership, a regulation is updated, a key person leaves, a dependency is re-architected. Treat major changes as a trigger to re-examine prior confirmations, especially those tied to critical path decisions. Confirmation is not permanent; it holds only as long as the conditions that made it true still hold.

## Common Traps

### Treating Logging as Completion

Writing an assumption in the register feels like managing it, so validation never gets scheduled. The trap is that the artifact exists and looks responsible, while the actual exposure is untouched. The cost appears late, when an unvalidated assumption fails and forces expensive replanning. Distinguish capture from validation and require a validation plan for every load-bearing assumption.

### Accepting Verbal Assurance as Confirmation

A stakeholder saying "yes, that should be fine" is recorded as confirmation, but it carries no evidence and no accountability. The trap is that verbal agreement feels like progress and avoids confrontation, yet it dissolves the moment the premise is tested. Require evidence appropriate to the assumption's type and consequence, and treat unsupported assurances as still-open.

### Validating the Easy Assumptions and Deferring the Hard Ones

Teams confirm the premises that are cheap and comfortable to check and quietly postpone the politically difficult or technically expensive ones. The trap is that the deferred assumptions are usually the most load-bearing, so the register shows high closure while the real exposure remains. Track validation by consequence, not by convenience, and escalate overdue validation of high-impact assumptions.

### Failing to Define What Counts as Confirmation

Without pre-agreed criteria, a validation activity produces a result that everyone interprets differently, and the assumption stays open through endless debate. The trap is that ambiguous criteria feel flexible but actually prevent closure. Define the pass and fail conditions before the activity starts, so the evidence can settle the question cleanly.

### Leaving Invalidated Assumptions as Open

When validation shows a premise is false, the team may avoid the uncomfortable conclusion by leaving the status ambiguous or rewording the assumption. The trap is that avoidance protects feelings while leaving the plan built on a known falsehood. Force a clean conversion: invalidated assumptions must become risks, issues, or replanning actions immediately.

### Letting Confirmed Assumptions Expire Silently

A confirmation from three months ago is treated as permanently settled, even though the vendor, regulation, or team composition has since changed. The trap is that stale confirmations give false confidence exactly when the risk has re-emerged. Tie confirmation validity to conditions and re-examine prior confirmations whenever a material change occurs.

### Reporting Green While Load-Bearing Assumptions Stay Open

Status reports show the project as healthy because tasks are moving, while critical decisions still rest on unproven premises. The trap is that the open assumptions are invisible in the health picture until they fail. Make the count and consequence of open load-bearing assumptions part of status reporting so the exposure is visible before it becomes a crisis.

## Self-Check

- [ ] Does every open assumption have a defined validation activity, a named owner, and a target date, rather than just "monitor"?
- [ ] Is validation sequenced by consequence and time-criticality, front-loading assumptions whose falsity forces the most expensive replanning?
- [ ] Is the validation method matched to the assumption type (test, written approval, commitment, pilot data) rather than defaulted to verbal assurance?
- [ ] Are unproven load-bearing assumptions reported as active exposure, not hidden behind green status?
- [ ] Are confirmation criteria, including pass and fail thresholds, defined before validation activities begin?
- [ ] Are invalidated assumptions promptly converted to risks, issues, or replanning triggers instead of left open?
- [ ] Does each assumption carry a clear lifecycle status with a last-updated date and evidence reference?
- [ ] Are aged open assumptions flagged and escalated rather than allowed to drift indefinitely?
- [ ] Are previously confirmed assumptions re-examined when material conditions change?
- [ ] Is validation tracked by consequence, ensuring the hard high-impact assumptions are not deferred in favor of easy ones?
