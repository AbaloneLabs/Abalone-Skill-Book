---
name: launch_readiness_and_checklist.md
description: Use when the agent is preparing for a product launch, building a launch readiness checklist, coordinating cross-functional launch dependencies, or deciding whether a feature is ready to release to users.
---

# Launch Readiness And Checklist

A launch is not the moment code reaches production. It is a coordinated event across product, engineering, design, support, sales, marketing, legal, security, and data, where each function has work that must be complete before customers are exposed. Launch readiness is the discipline of making that coordination explicit instead of assumed.

Agents miss this because shipping feels like the finish line. They confirm the feature is built, the happy path works, and a launch date exists, then declare readiness. The harm is predictable: support is flooded by questions they were never briefed on, sales pitches capability that does not exist yet, marketing overclaims, legal reviews happen after the announcement, and a late-breaking integration failure turns a planned launch into an incident. The opposite failure is over-process, where a small improvement is gated behind a major-release checklist and momentum dies.

Use this skill before answering broad questions such as "is this ready to launch", "what should be on our launch checklist", "who needs to be ready before we release", "what are our go/no-go criteria", or "do we need a dry run for this launch". The goal is to prevent the agent from treating readiness as a vibe check and instead make dependencies, gates, and risks explicit.

## Core Rules

### Size The Launch To A Tier

Not every release deserves the same ceremony. A launch tier defines how much coordination, communication, and review a change requires, and it should be decided before work begins so the right functions are pulled in early.

A typical tiering:

- Tier 1, major release: new product, major repositioning, breaking change, or anything customer-facing and strategic. Full cross-functional readiness, executive review, GTM bill of materials, dry run.
- Tier 2, significant feature: meaningful new capability for existing users. Support and sales enablement, help docs, announcement, targeted monitoring.
- Tier 3, minor improvement: small feature or enhancement. Light documentation, internal note, standard rollout.
- Tier 4, maintenance or fix: bug fix, perf, polish. No external launch motion.

The trap is either over-investing ceremony in a Tier 4 change or under-investing coordination in a Tier 1. Decide the tier explicitly and let it drive the checklist.

### Build A Cross-Functional Readiness Checklist

Readiness is owned by product but executed by many functions. The checklist should name, per function, what done looks like and who owns it. A checklist that says "support ready" is useless; a checklist that says "support macros drafted, FAQ published, escalation path tested, on-call briefed by [date]" is actionable.

Cover at minimum:

- product: scope locked, acceptance criteria met, release notes drafted;
- engineering: deployed, feature-flagged, rollback path verified, on-call rotation aware;
- design: final states shipped, empty and error states reviewed, accessibility checked;
- support: macros, FAQ, known-issues list, escalation contacts, staffing for launch volume;
- sales: enablement deck, objection handling, pricing and packaging confirmed;
- marketing: announcement, blog, demo, assets approved and scheduled;
- legal and compliance: terms, privacy, claims, and regulatory review complete;
- security: threat model and review done for sensitive changes;
- data: instrumentation verified, dashboards live, guardrails defined.

Each item needs an owner and a due date, not just a checkbox.

### Define Go/No-Go Criteria And Launch Gates

A launch should have explicit gates that must pass before exposure. Without gates, a launch proceeds on optimism and social pressure, and a single late slip cascades into a confused release.

Define criteria such as:

- acceptance criteria met and QA signed off;
- performance and load tested to expected launch volume;
- security and privacy review complete for relevant changes;
- rollback verified and time-bound (how long to revert, how long to disable);
- monitoring and alerting in place with thresholds defined;
- support and sales enablement complete;
- documentation published or staged;
- data instrumentation confirmed sending correct events.

Decide in advance what blocks the launch versus what is an acceptable known issue. A gate that nobody is willing to enforce is not a gate.

### Identify The Critical Path And Late-Breaking Risks

The critical path is the chain of dependencies that determines whether the launch happens on time. Product should know which items, if they slip, move the date, and which can be absorbed.

Ask:

- What is the longest dependency chain leading to launch?
- Which external teams or vendors are on the path?
- What is the latest point at which we can still safely delay?
- What are the top three risks most likely to surface late?
- For each risk, what is the mitigation or contingency?

Late-breaking risks are normal; being surprised by them is not. A standing risk review in the final weeks catches integration failures, data issues, and dependency slips while there is still time to react.

### Rehearse High-Stakes Launches

For Tier 1 launches, a dry run or rehearsal is not optional theater. It surfaces the gaps that checklists miss: the demo that breaks on the real environment, the support agent who cannot find the FAQ, the speaker who cannot answer the obvious question, the runbook that assumes access nobody has.

Rehearse the customer-facing flows end to end, the support escalation path, the incident response, and the announcement timing. The cost of a rehearsal is hours; the cost of a public launch failure is weeks of cleanup and lost trust.

### Confirm Rollback And Incident Response Readiness

Every launch should be reversible in a defined way, or the team should have explicitly accepted that it is not and why. Reversibility is what makes a launch safe to attempt.

Confirm:

- how the feature is disabled (flag, config, deploy);
- how long rollback takes and who can execute it;
- what the incident response looks like if something breaks at 2am;
- who is on call during and after the launch window;
- what severity thresholds trigger rollback versus investigation.

If the answer to "how do we turn this off" is uncertain, the launch is not ready.

### Match Documentation And Enablement To The Tier

Documentation and enablement are part of the product on launch day, not a follow-up. Users judge a feature by whether they can understand and use it, and support judges it by whether they can answer for it.

Match the depth to the tier: a major release needs help center articles, in-product guidance, release notes, and a demo; a minor improvement may need only a release note and an internal heads-up. The failure is launching capability that no one can explain.

## Common Traps

### Treating Readiness As A Vibe Check

Saying "we feel good about it" is not readiness. Without explicit gates and owners, readiness becomes whoever is most optimistic in the room, and the pessimists get blamed later for not speaking up.

### Over-Processing Small Changes

Applying a Tier 1 checklist to a Tier 4 fix burns goodwill and slows the team. Process should scale to risk. A checklist that cannot shrink is a sign the tiering is not real.

### Forgetting Internal Teams Until Launch Day

Support, sales, and customer success are the people who face customers when something breaks or confuses. If they learn about the launch from the public announcement, they cannot defend it, and the launch fails in the channel you cannot see.

### Assuming Marketing Claims Match Reality

Marketing copy often drifts ahead of what shipped. If GTM assets are not reviewed against the actual product behavior, the company announces capability that does not exist, and trust erodes the moment a customer tries it.

### Skipping The Dry Run Because The Team Is Busy

Busyness is exactly when the dry run matters most. The teams that skip rehearsal are the ones who discover, on launch day, that the demo environment is stale and the runbook is wrong.

### Leaving Rollback As A Theoretical Option

Rollback that has never been tested is a hope, not a plan. Many teams assume they can revert, then discover a migration or a flag dependency makes it painful or impossible under pressure.

### Launching To Everyone At Once Without Gates

A big-bang launch maximizes blast radius for any defect. Even when a full release is the goal, gating exposure behind a brief internal or beta stage catches issues a checklist cannot.

## Self-Check

- [ ] The launch has an explicitly assigned tier, and the ceremony matches that tier rather than defaulting to maximum or minimum process.
- [ ] A cross-functional checklist exists with a named owner and due date for each item across product, engineering, design, support, sales, marketing, legal, security, and data.
- [ ] Go/no-go gates are defined in advance, and the team has agreed which are hard blockers versus acceptable known issues.
- [ ] The critical path is identified, including the latest safe delay point and the top late-breaking risks with mitigations.
- [ ] High-stakes launches have a rehearsed dry run covering customer flows, support escalation, and incident response.
- [ ] Rollback is concrete and time-bound: the team knows how to disable the feature, how long it takes, and who can execute it.
- [ ] Monitoring, alerting, guardrail thresholds, and on-call coverage are in place for the launch window.
- [ ] Documentation, help center content, release notes, and in-product guidance are published or staged before exposure.
- [ ] Support and sales enablement is complete, with FAQ, macros, objection handling, and escalation paths tested.
- [ ] GTM and marketing claims have been verified against what the product actually does on day one.
