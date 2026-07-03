---
name: attribution_and_incrementality.md
description: Use when the agent is designing or evaluating marketing attribution, choosing between last-click first-click and multi-touch models, running incrementality or geo holdout tests, deciding how to credit channels for conversions, or reviewing whether reported channel performance reflects real contribution.
---

# Attribution And Incrementality

Attribution is how marketing decides which efforts earned credit for a result. It is one of the most contested and most error-prone areas of marketing analytics because every model is an approximation and each one rewards different behavior. Get it wrong and budget flows to the wrong channels, awareness and brand work gets starved, and teams optimize for the appearance of performance rather than real contribution. Incrementality testing is the discipline that keeps attribution honest.

Use this skill before choosing an attribution model, interpreting channel performance, allocating budget by attribution, or designing incrementality tests. The goal is to prevent the agent from treating any single attribution model as truth and from starving channels that actually drive results.

## Core Rules

### Treat Every Attribution Model As A Biased Lens

No attribution model is correct. Each one is a lens with known distortion.

Know the bias of each:

- last-click credits the final touch and starves upper-funnel work;
- first-click credits discovery and ignores what drove conversion;
- linear and time-decay spread credit but can dilute real contribution;
- position-based weights the ends but assumes a shape that may not hold;
- data-driven models reflect your data's patterns but need volume and cleanliness.

State which model you use, why, and what behavior it rewards. Never present one model's numbers as the objective truth about a channel's value.

### Separate Attribution From Incrementality

Attribution credits touches that occurred. Incrementality measures what would not have happened without the effort. They answer different questions.

Distinguish:

- attribution asks who was present when conversion happened;
- incrementality asks what would have happened without the spend;
- a channel can be present on every conversion yet add no incremental value;
- a channel can touch few conversions yet drive many it never gets credit for.

Channels that score well on attribution can score poorly on incrementality, especially retargeting and branded search. Test both views before reallocating budget.

### Use Holdouts And Geo Tests For High-Stakes Decisions

When the budget decision is large, rely on experiments, not model output.

Design:

- geo holdouts that withhold spend in matched markets;
- holdout audiences that exclude a control group from a campaign;
- pre-post measurement with matched controls;
- sufficient sample and duration to detect a real effect;
- clear definition of the metric being measured.

A clean holdout is the closest marketing gets to proof. Use it for decisions that move meaningful money.

### Watch For Self-Attributing Networks

Ad platforms report their own performance, and they have incentives to look good. Treat platform-reported attribution with skepticism.

Guard against:

- platforms using view-through or engagement windows that inflate credit;
- platforms crediting conversions that would have happened anyway;
- overlapping platforms both claiming the same conversion;
- branded search and retargeting capturing demand created elsewhere;
- reported ROAS that ignores incrementality.

Reconcile platform totals against your own source of truth, and validate with incrementality where it matters.

### Account For Cross-Device And Identity Gaps

Real journeys span devices, cookies expire, and identity breaks down. Attribution built on fragile identity under-credits some paths.

Acknowledge:

- cross-device journeys that fragment a single user;
- cookie loss and consent changes that break tracking;
- anonymous upper-funnel touches that never get credited;
- B2B committee journeys that span multiple people;
- the gap between tracked and actual paths.

The tracked path is always a subset of the real one. Design attribution to be robust to this gap, and do not over-interpret precision.

### Reconcile Attribution With Finance

Attribution numbers must reconcile with the business's financial reality, or they will not be trusted.

Reconcile:

- attributed pipeline and revenue against finance's actuals;
- attributed CAC against blended finance CAC;
- the gap between attributed and blended results, and explain it;
- the unattributed portion rather than ignoring it.

If attributed revenue plus unattributed does not plausibly equal total, the model is misleading decision-makers.

### Revisit The Model As Funnel And Mix Change

The best attribution model for one stage of growth is wrong for another. Revisit it.

Revisit when:

- the channel mix changes substantially;
- the funnel length or committee size changes;
- tracking and identity capabilities shift;
- the business enters new segments or markets;
- incrementality results contradict the model.

Attribution is not set-and-forget. It degrades as conditions drift from when it was calibrated.

## Common Traps

### Last-Click As Default Truth

Defaulting to last-click starves the awareness and consideration work that made the last click possible.

### Platform Self-Attribution

Trusting ad platforms' own ROAS without reconciliation or incrementality inflates perceived performance.

### Confusing Attribution With Incrementality

Crediting a channel for conversions it was present on does not prove it caused them.

### Ignoring Branded Search And Retargeting Capture

Treating branded search and retargeting conversion rates as proof of those channels' value ignores demand created elsewhere.

### Over-Precise Multi-Touch

Believing a complex model's fractional credit is precise truth hides that it is still an approximation.

### No Reconciliation With Finance

Attribution that does not reconcile with actuals loses credibility and misleads budget decisions.

### Set-And-Forget Model

Failing to revisit attribution as the funnel, mix, and tracking change lets the model silently mislead.

## Self-Check

- [ ] The attribution model in use is named, its bias is stated, and the behavior it rewards is understood.
- [ ] Attribution is distinguished from incrementality, and both views are considered for budget decisions.
- [ ] High-stakes budget decisions are supported by holdout or geo experiments, not model output alone.
- [ ] Platform-reported performance is reconciled against an independent source of truth.
- [ ] Branded search and retargeting are evaluated for incrementality, not just attributed conversion rate.
- [ ] Cross-device, cookie loss, and identity gaps are acknowledged, and precision is not over-interpreted.
- [ ] Attributed results reconcile plausibly with finance's actuals, and the unattributed portion is explained.
- [ ] The model is revisited when the channel mix, funnel, or tracking capabilities change.
- [ ] More than one attribution model is reviewed for important decisions.
- [ ] The team understands that attribution guides judgment, it does not replace it.
