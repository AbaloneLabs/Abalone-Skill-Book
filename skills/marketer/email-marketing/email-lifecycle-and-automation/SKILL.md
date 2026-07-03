---
name: email_lifecycle_and_automation.md
description: Use when the agent is designing lifecycle email programs, building triggered and automated email flows, planning welcome onboarding abandonment re-engagement sequences, mapping email to customer journey stages, or reviewing whether automated email responds to real behavior and need.
---

# Email Lifecycle And Automation

Lifecycle email responds to where a customer actually is and what they just did, rather than broadcasting on a schedule. It is usually the highest-performing part of an email program because it is timely and relevant. But it fails when triggers fire on the wrong events, when sequences ignore the recipient's actual state, when automation sends email to people who should be suppressed, or when flows are built once and never maintained. Bad automation sends the wrong message at scale, faster than any human could.

Use this skill before designing lifecycle flows, building triggered sequences, mapping email to journey stages, or auditing automated programs. The goal is to prevent the agent from building automation that is technically impressive but behaviorally wrong.

## Core Rules

### Map Email To Real Journey Stages And Triggers

Lifecycle email only works when it responds to a real moment in the customer's actual experience. Start from the journey, not the email.

Map:

- the real stages a customer moves through, from signup to activation to retention to risk;
- the events or states that signal each stage, such as signup, first use, inactivity, or renewal;
- the question or need the customer has at that moment;
- the email that would genuinely help at that point;
- the trigger that should fire it, based on behavior not guesswork.

An email that arrives at the wrong moment, no matter how well written, is irrelevant or annoying.

### Choose Triggers That Reflect Real Intent

The trigger determines relevance. Choose events that genuinely signal the recipient's situation or need.

Use strong triggers:

- a signup or download that signals interest;
- a first action or milestone that signals activation;
- inactivity that signals disengagement risk;
- a purchase or upgrade that signals a new stage;
- a renewal or contract event that signals decision time.

Avoid weak triggers like time alone or arbitrary segments that do not reflect what the recipient just did or needs.

### Respect The Recipient's Current State

Automation that ignores what the recipient has already done sends embarrassing, irrelevant messages. Build state awareness.

Account for:

- whether the recipient already completed the action the email pushes;
- whether they are in another flow that should take precedence;
- their current plan, status, or lifecycle stage;
- their consent and preferences;
- suppression of converted, churned, or opted-out users.

Sending an onboarding email to someone who already activated, or a trial email to someone who already bought, signals broken automation and erodes trust.

### Sequence For The Right Outcome, Not Maximum Sends

A flow should move the recipient toward a specific outcome, not maximize touches. Each email must earn its place.

Design sequences:

- with a clear goal for the flow, such as activation, conversion, or retention;
- where each email advances toward that goal;
- with conditional branches based on behavior;
- with an end or escalation, not infinite loops;
- with frequency that respects the recipient's tolerance.

A flow that sends daily emails for weeks to drive one activation burns the relationship for a marginal conversion gain.

### Write For The Moment, Not The Brand

Lifecycle email copy should fit the specific moment and need, not read like a generic broadcast.

Write:

- in the context of what the recipient just did or did not do;
- with the next step that genuinely helps them;
- with proof and reassurance appropriate to their stage;
- without assuming knowledge they do not have;
- with a single clear call to action.

Copy that ignores the trigger event feels robotic. Copy that references the recipient's actual behavior feels human.

### Test, Maintain, And Audit Automation

Automation decays as products, segments, and behaviors change. Build maintenance into the program.

Maintain:

- regular audits of active flows for broken triggers and stale content;
- testing of subject lines, timing, and content within flows;
- review of suppression logic as segments and products evolve;
- monitoring of flow performance and fatigue signals;
- sunset or rebuild of flows that no longer match the product.

A flow built two years ago may now reference features that no longer exist or send to segments that should be excluded.

### Connect Automation To Business Outcomes

Lifecycle flows should be measured by the outcomes they move, not the emails they send.

Measure:

- the conversion or activation rate of flow recipients versus a comparable non-received group;
- the incremental lift the flow provides, ideally via holdout;
- the revenue or retention influenced;
- guardrails such as unsubscribes, complaints, or fatigue within the flow;
- drop-off points where recipients disengage.

Without a holdout or comparison, you cannot tell whether the flow caused the outcome or merely correlated with recipients who were going to convert anyway.

## Common Traps

### Time-Based Instead Of Behavior-Based Triggers

Sending on a schedule rather than in response to real behavior produces irrelevant timing.

### Ignoring Recipient State

Automation that fires regardless of what the recipient already did sends embarrassing, contradictory messages.

### Infinite Or Excessive Sequences

Flows that never end or send too often exhaust recipients for marginal gains.

### Stale, Unmaintained Flows

Flows built long ago reference outdated features, segments, or offers and quietly damage the experience.

### No Suppression Logic

Failing to suppress converted, churned, or opted-out users sends irrelevant or violating email.

### Generic Copy Ignoring The Trigger

Copy that could appear in any email ignores the specific moment that justified sending.

### No Holdout Or Incrementality

Without a comparison group, the flow's true contribution to outcomes is unknown.

## Self-Check

- [ ] Each flow is mapped to a real journey stage, trigger event, and recipient need.
- [ ] Triggers reflect genuine behavior or intent, not arbitrary time or segment guesses.
- [ ] Recipient state is respected, with suppression of those who completed, converted, churned, or opted out.
- [ ] Sequences have a clear goal, conditional branches, an end, and frequency that respects tolerance.
- [ ] Copy is written for the specific moment and trigger, not as generic broadcast.
- [ ] Flows are regularly audited, tested, and maintained as products and segments change.
- [ ] Suppression and precedence logic prevents conflicting or redundant flows.
- [ ] Performance is measured by incremental lift, ideally via holdout, not just flow activity.
- [ ] Drop-off points and fatigue signals within flows are monitored and addressed.
- [ ] No flow references outdated features, offers, or segments that no longer exist.
