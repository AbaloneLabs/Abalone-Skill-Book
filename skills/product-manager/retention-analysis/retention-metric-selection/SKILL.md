---
name: retention_metric_selection.md
description: Use when the agent is choosing a retention metric, defining what counts as active or retained, selecting the right time horizon, or deciding between broad and narrow retention definitions for the product and question at hand.
---

# Retention Metric Selection

Retention is the most quoted and most misdefined number in product analytics. The phrase "30-day retention is 40%" means almost nothing until someone states what counts as active, which cohort is being measured, what the comparison baseline is, and whether the window resets on each return. Selecting the metric is the real decision; computing it is the easy part.

Agents miss this because retention feels like a single fact waiting to be read off a dashboard. They grab whatever the analytics tool labels "retention", report it, and build strategy on a number whose definition quietly changes between teams. The harm is that a product can look healthy under a loose definition while real engagement decays, or look broken under a strict definition while power users are deepening. The opposite failure is overfitting the definition to flatter the team, choosing the horizon and active-event that produce the prettiest curve.

Use this skill before answering broad questions such as "what is our retention", "how should we define active users", "what retention window should we use", "is rolling or bounded retention better", or "how do we compare retention to competitors". The goal is to prevent the agent from reporting a retention number without first justifying the definition that produced it.

## Core Rules

### Define Active Explicitly And Match It To Value

Retention is only as honest as its definition of "active". A login is not engagement. A session is not value. An active user should have performed an action that reflects the core value of the product, not the lowest-friction event the analytics tool happens to capture.

Ask what event proves the user received value. For a messaging app that may be sending or reading a message; for a productivity tool, completing a task; for a marketplace, a successful transaction. A broad definition (any visit) inflates the number and hides decay; a narrow definition (only the primary value action) is more honest but smaller and noisier. State the chosen event, why it represents value, and what a looser or stricter alternative would change.

### Choose Bounded Versus Rolling Retention Deliberately

Bounded (cohort) retention measures the percentage of users from a starting cohort who are active in a specific later window, such as day 7 or day 30. Rolling (N-day) retention measures whether a user was active at any point on or after day N. The two answer different questions and produce different curves even on the same data.

Bounded retention is the right default for understanding the shape of the drop-off and comparing cohorts fairly, because each cohort is measured against the same fixed windows. Rolling retention is better when the product expects irregular but persistent use, such as a tax tool or a travel booking app, where activity clusters seasonally. State which is in use, because mixing the two across reports produces comparisons that look precise and are meaningless.

### Match The Time Horizon To The Product's Natural Usage Cycle

A 7-day horizon is meaningless for a product people use monthly, and a 90-day horizon hides everything for a daily-use product. The retention window should reflect how often a user would naturally return if the product were working, not the cadence the team wishes were true.

Map the horizon to the job. A social feed or communication tool may warrant daily or weekly retention. A budgeting or expense tool may warrant monthly. An annual compliance or tax product may warrant quarterly or yearly. Choosing a horizon shorter than the natural cycle makes healthy retention look catastrophic; choosing one longer smooths over real early drop-off. When in doubt, report multiple horizons (day 1, day 7, day 30) so the curve shape is visible rather than a single flattering point.

### Segment By Cohort Before Averaging

A single blended retention number hides the cohorts that matter. New-user retention, resurrected-user retention, and retained-user retention behave differently, and a blended metric can rise while new-user retention falls, masking a acquisition-quality or onboarding problem.

Decide which cohorts to track separately: acquisition channel, signup month, plan tier, geography, device, or first-use behavior. The most diagnostic segmentation is usually the one tied to how the user was acquired or what they did in their first session, because early experience predicts later retention. Report at least new-user cohort retention separately, since that is where most product levers act.

### Separate Retention From Frequency And Breadth

Retention asks whether a user came back. It does not capture how often, how deeply, or across how many features. Two products with identical 30-day retention can be radically different in health if one user returns once and the other returns daily across multiple workflows.

Pair retention with complementary metrics: frequency (sessions per active user per period), breadth (number of features or surfaces touched), and depth (time or actions per session). Decide which combination defines a "healthy" user for this product, and avoid treating retention in isolation as a complete picture of engagement.

### Pick A Comparison Baseline Before Judging The Number

A retention number without a baseline is uninterpretable. Forty percent day-30 retention is excellent for a B2B SaaS tool and alarming for a daily consumer app. The judgment depends entirely on the reference point.

Useful baselines include the product's own historical trend, a target derived from unit economics or break-even, a benchmark for the product category, or the retention of a comparison cohort (such as a control group in an experiment). State the baseline alongside the number. Without it, the team will either celebrate mediocrity or panic over a healthy curve.

### Reconcile The Definition With What Stakeholders Expect

Retention is reported upward to leadership, outward to investors, and sideways to peer products. If each audience uses a different definition, every meeting becomes an argument about the number rather than the decision. Worse, definitions drift toward whichever is most flattering for the current audience.

Document the chosen definition in one place, note where it differs from finance, marketing, or investor reporting, and decide deliberately whether to align them. A product team that silently uses a stricter definition than the one sold to investors is storing up a credibility problem.

## Common Traps

### Defaulting To Login Or Session As Active

Using login or any-page-view as the active event is the most common and most flattering mistake. It counts users who opened a notification, bounced, and left, treating them as retained. This hides real disengagement behind a low bar.

### Choosing The Horizon That Makes The Curve Look Best

When day-7 retention looks bad but day-30 looks fine, teams quietly switch the headline metric. Selecting the window after seeing the data is p-hacking applied to retention, and it produces numbers no one should make decisions from.

### Comparing Bounded To Rolling Across Reports

One dashboard reports bounded day-30 retention, another reports rolling, and a slide blends them. The numbers are not comparable, but they are presented as if they are, leading to confident conclusions from incompatible definitions.

### Averaging Away The Cohort That Matters

Blending new and existing users into one retention number can show stability while new-user retention collapses. The aggregate looks fine precisely because the problem is hidden inside it.

### Treating One Retention Number As The Whole Story

Reporting retention without frequency, breadth, or depth lets a product look healthy while users become shallow and intermittent. The headline stays green until it suddenly does not.

### Benchmarking Against An Undefined "Industry Average"

Citing an industry retention benchmark without naming the source, definition, or product category gives false confidence. Benchmarks are only useful when their definition matches yours, which is rare.

### Changing The Definition Without Versioning The History

When the active event or window changes, the historical series breaks. Teams compare the new definition to the old history as if continuous, producing trends that are artifacts of the definition change rather than real behavior.

## Self-Check

- [ ] The active event is defined explicitly and reflects core product value, not merely login or any session.
- [ ] The choice between bounded and rolling retention is stated, and reports do not silently mix the two.
- [ ] The time horizon matches the product's natural usage cycle, and multiple horizons are shown when the cycle is ambiguous.
- [ ] Retention is segmented by cohort, with new-user retention reported separately rather than blended away.
- [ ] Retention is paired with frequency, breadth, or depth so a single number is not mistaken for the whole engagement picture.
- [ ] A comparison baseline (historical trend, target, category benchmark, or control cohort) is named alongside the number.
- [ ] The definition is documented, and differences from finance, marketing, or investor reporting are reconciled deliberately.
- [ ] The horizon and active event were chosen before looking at the data, not selected to flatter the curve.
- [ ] Any change to the definition is versioned, and historical series are not compared across incompatible definitions.
- [ ] Benchmark comparisons name the source, definition, and product category rather than citing an undefined industry average.
