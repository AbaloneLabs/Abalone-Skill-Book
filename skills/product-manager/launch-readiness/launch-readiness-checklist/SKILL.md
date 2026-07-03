---
name: launch_readiness_checklist.md
description: Use when the agent is building a launch readiness checklist, verifying cross-functional completeness before release, checking that support sales marketing docs and ops are ready, or gating a launch on true readiness rather than a date.
---

# Launch Readiness Checklist

A launch is not the moment code reaches production. It is a coordinated event across product, engineering, design, support, sales, marketing, legal, security, and data, where each function has work that must be complete before customers are exposed. A readiness checklist is the tool that makes that coordination explicit instead of assumed, and gating a launch on the checklist rather than on a date is what separates a controlled release from a hope.

Agents miss this because shipping feels like the finish line. They confirm the feature is built, the happy path works, and a launch date is on the calendar, then declare readiness. The harm is predictable: support is flooded by questions they were never briefed on, sales pitches capability that does not exist yet, marketing overclaims, legal reviews happen after the announcement, and a late integration failure turns a planned launch into an incident. The opposite failure is over-process, where a minor improvement is gated behind a major-release checklist and momentum dies.

Use this skill before answering broad questions such as "is this ready to launch", "what should be on our readiness checklist", "who needs to be ready before we release", "what are our go or no-go criteria", or "do we need a dry run". The goal is to prevent the agent from treating readiness as a vibe check and instead make completeness, ownership, and gates explicit.

## Core Rules

### Size The Checklist To The Launch Tier

Not every release deserves the same ceremony. A launch tier defines how much coordination, communication, and review a change requires, and it should be decided before work begins so the right functions are pulled in early and the checklist scales to the risk.

A typical tiering ranges from a major release, which needs full cross-functional readiness, executive review, and a dry run, down through significant features, minor improvements, and maintenance fixes, each with progressively lighter process. The failure is either over-investing ceremony in a small fix or under-investing coordination in a strategic release. Decide the tier explicitly and let it drive the checklist.

### Make Every Item Specific, Owned, And Dated

A checklist that says "support ready" is useless. A checklist that says "support macros drafted, FAQ published, escalation path tested, on-call briefed by Friday, owner named" is actionable. Each item must state what done looks like, who is responsible, and when it is due.

Generic items get checked off without the underlying work being done, because no one knows what the check actually required. Force specificity: name the artifact, the owner, the due date, and the verification. An item without an owner is an item no one will do.

### Cover Every Function That Faces The Customer Or The System

Readiness is owned by product but executed by many functions, and the checklist must reflect that. Omitting a function means that function discovers the launch from the public announcement and cannot defend it.

Cover at minimum product (scope locked, acceptance met, release notes), engineering (deployed, flagged, rollback verified, on-call aware), design (final and edge states shipped, accessibility checked), support (macros, FAQ, known issues, staffing), sales (enablement, objection handling, pricing confirmed), marketing (announcement, assets approved and scheduled), legal and compliance (terms, privacy, claims reviewed), security (threat model for sensitive changes), and data (instrumentation verified, dashboards live). Each function's readiness is a gate, not a nice-to-have.

### Define Hard Gates Versus Acceptable Known Issues

Not every incomplete item should block a launch, and not every incomplete item should be tolerated. The team must decide in advance which checklist items are hard blockers and which are acceptable known issues the launch can proceed with.

Classify each gate as blocking or non-blocking. A blocking gate that no one is willing to enforce is not a gate; it is theater. A non-blocking issue that surprises a customer on launch day is a failure of classification. Make the distinction explicit, document the accepted known issues, and communicate them so no one is blindsided.

### Verify Marketing And Sales Claims Against Reality

Marketing copy and sales decks drift ahead of what shipped. If go-to-market assets are not reviewed against actual product behavior, the company announces capability that does not exist, and trust erodes the moment a customer tries it.

Build a verification step into the checklist: every external claim, screenshot, demo, and capability statement is checked against the shipped product. This is where launches most commonly overclaim, because marketing works from intent and engineering works from reality, and the two diverge in the final weeks.

### Confirm Rollback And Operational Readiness

Every launch should be reversible in a defined way, or the team should have explicitly accepted that it is not and why. Reversibility is what makes a launch safe to attempt, and operational readiness is what makes it survivable if something breaks.

Confirm how the feature is disabled, how long rollback takes and who can execute it, what the incident response looks like if something breaks overnight, who is on call during and after the launch window, and what severity thresholds trigger rollback. If the answer to "how do we turn this off" is uncertain, the launch is not ready.

### Rehearse High-Stakes Launches

For major releases, a dry run is not optional theater. It surfaces the gaps that checklists miss: the demo that breaks on the real environment, the support agent who cannot find the FAQ, the speaker who cannot answer the obvious question, the runbook that assumes access nobody has.

Rehearse the customer-facing flows end to end, the support escalation path, the incident response, and the announcement timing. The cost of a rehearsal is hours; the cost of a public launch failure is weeks of cleanup and lost trust. Skipping rehearsal because the team is busy is exactly when rehearsal matters most.

## Common Traps

### Treating Readiness As A Vibe Check

Saying "we feel good about it" is not readiness. Without explicit gates and owners, readiness becomes whoever is most optimistic in the room, and the pessimists get blamed later for not speaking up.

### Over-Processing Small Changes

Applying a major-release checklist to a minor fix burns goodwill and slows the team. Process should scale to risk, and a checklist that cannot shrink is a sign the tiering is not real.

### Forgetting Internal Teams Until Launch Day

Support, sales, and customer success face customers when something breaks or confuses. If they learn about the launch from the public announcement, they cannot defend it, and the launch fails in the channel you cannot see.

### Generic Checklist Items

Items like "docs ready" or "support ready" get checked off without the work being done, because no one defined what the check required. Specificity is what makes a checklist trustworthy.

### Gates No One Will Enforce

A blocking criterion that the team will not actually hold the launch on is theater. It creates the illusion of rigor while letting the launch proceed regardless.

### Marketing Claims Outrunning The Product

GTM assets built from intent rather than shipped reality announce capability that does not exist, and trust erodes the moment a customer tries the feature.

### Skipping The Dry Run Because The Team Is Busy

Busyness is exactly when the dry run matters most. The teams that skip rehearsal are the ones who discover, on launch day, that the demo environment is stale and the runbook is wrong.

## Self-Check

- [ ] The launch has an explicitly assigned tier, and the ceremony matches that tier rather than defaulting to maximum or minimum process.
- [ ] Every checklist item states a specific artifact, a named owner, and a due date, not a vague "ready".
- [ ] The checklist covers product, engineering, design, support, sales, marketing, legal, security, and data, with each function's readiness treated as a gate.
- [ ] Each gate is classified as a hard blocker or an acceptable known issue, and the team has agreed which is which.
- [ ] Blocking gates are ones the team will actually enforce, not theater that yields to launch pressure.
- [ ] Marketing and sales claims, screenshots, and demos have been verified against what the product actually does on day one.
- [ ] Rollback is concrete and time-bound: the team knows how to disable the feature, how long it takes, and who can execute it.
- [ ] Monitoring, alerting, guardrail thresholds, and on-call coverage are in place for the launch window.
- [ ] Documentation, help center content, release notes, and in-product guidance are published or staged before exposure.
- [ ] High-stakes launches have a rehearsed dry run covering customer flows, support escalation, and incident response.
