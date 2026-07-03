---
name: funnel_construction_and_steps.md
description: Use when the agent is building a conversion funnel, choosing which steps to include, defining funnel boundaries and conversion windows, or deciding whether a step represents a real user decision versus a mere screen transition.
---

# Funnel Construction And Steps

A funnel is a model of a journey, and like any model it is only as good as the choices baked into it. The tool computes the rates in seconds, which hides the fact that every decision about steps, boundaries, and windows determines whether the resulting number means anything. Two analysts building a funnel for the same product can produce wildly different drop-off stories, not because the data differs, but because one counted screens and the other counted decisions. Funnel construction is a judgment problem disguised as a query.

The harm this skill prevents is a funnel that confidently misleads. A funnel built from every screen in a flow shows drop-off everywhere and locates the real problem nowhere. A funnel with the wrong conversion window manufactures drop-off that does not exist, or erases drop-off that does. A funnel whose steps are not real user decisions produces a rate that moves for reasons unrelated to user intent. The construction errors, not the arithmetic, are what lead teams to fix the wrong step.

Use this skill before building a conversion funnel, choosing which steps to include, setting the conversion window, or defining where the funnel begins and ends. The work is to make the funnel a faithful model of a genuine user journey, where each step represents a real decision and each rate reflects real behavior.

## Core Rules

### Define Each Step As A Behavior, Not A Screen

A funnel step should represent a meaningful user decision or action, not merely a page the user passed through. Listing every screen in the flow produces a long funnel where every transition looks like failure, because users naturally pause, backtrack, and resume. The real drop-off is usually concentrated in one or two genuine decisions, and a screen-by-screen funnel obscures it beneath noise.

Choose steps that mark commitment: started, entered key information, submitted, completed verification, finished. A good test is whether a user could skip the step and still succeed; if yes, the step is a screen, not a decision, and belongs in a diagnostic drill-down rather than the primary funnel.

### Keep The Funnel Short Enough To Interpret

Every additional step adds a conversion rate that multiplies into the final number and dilutes interpretability. A ten-step funnel where each step converts at ninety percent yields a final rate of thirty-nine percent, which looks like collapse but is just compounding. Long funnels make every step look like a problem and hide which one actually matters.

Prefer a short primary funnel of three to six genuine decisions, and push detailed investigation into separate drill-downs. If a step moves the overall rate by less than a percent, ask whether it belongs in the primary view or is better understood as context.

### Match The Conversion Window To Observed Behavior

The conversion window, the time allowed to move from one step to the next, changes the rate more than almost any other setting. A same-session window punishes products with considered purchases, because users who return the next day count as drop-off. A thirty-day window flatters impulsive flows by counting eventual conversions that had nothing to do with the original session. There is no universally correct window, only the one that matches how users actually behave.

Estimate the window from historical time-to-convert data: plot when users who eventually convert actually do so, and choose a window that captures the bulk without waiting indefinitely. State the window alongside every rate, because a conversion rate without a window is meaningless and a different window tells a different story.

### Define Clear Boundaries For Entry And Exit

Where the funnel begins and ends determines what the rate measures. An entry defined too broadly, such as anyone who loaded the homepage, dilutes the funnel with low-intent users and depresses every downstream rate. An entry defined too narrowly, such as only users who clicked a specific button, hides how many never reach the funnel at all. Both choices are valid for different questions, but they must be deliberate.

Define the entry as the point where the user has shown intent relevant to the outcome, and the exit as the completed outcome. Document what counts as entry and completion, including exclusions for internal users, test accounts, and duplicate events, so the rate is reproducible and not silently redefined.

### Distinguish Sequential From Non-Sequential Flows

Classic funnels assume users move through steps in order, but real journeys branch, loop, and skip. A user may complete step three before step two, or revisit step one after step four. A strictly sequential funnel mislabels these users as drop-off even when they eventually convert, which understates the real rate.

Check whether the journey is genuinely sequential before using a strict funnel. Where users move non-linearly, consider a milestone-based view that tracks completion of key actions regardless of order, or segment users by their actual path. Forcing a branching journey into a linear funnel produces a number that does not match reality.

### Align Step Granularity With The Decision Being Modeled

Step granularity should match the decision the funnel is meant to inform. A coarse funnel with few steps is good for spotting where the biggest loss occurs but hides the detail needed to fix it. A fine-grained funnel localizes the problem but risks counting screens as decisions. Choose granularity based on whether the goal is to locate loss or to diagnose it.

A practical approach is to start coarse, identify the step with the largest drop, then build a detailed drill-down for that step alone. This keeps the primary funnel interpretable while still enabling diagnosis where it matters.

### Validate The Funnel Against Known Outcomes

Before trusting a funnel, check it against ground truth you already know. If the funnel says a thousand users completed purchase but the billing system recorded nine hundred, the discrepancy points to a construction error such as double-counting, missing events, or a window that captures cancellations. A funnel that does not reconcile with operational data is hiding a definition problem.

Reconcile the funnel output against independent counts at entry, at completion, and at one or two intermediate steps. Discrepancies are diagnostic, not embarrassing; they reveal where the model diverges from reality before the team acts on the model.

## Common Traps

### Counting Screens Instead Of Decisions

A funnel built from page views treats every transition as a step and produces drop-off everywhere. Users naturally pause and backtrack, so screen-level funnels overstate loss and hide the real decision points. Build steps from genuine commitments, not from screens.

### Using The Tool's Default Conversion Window

The default window in the analytics tool is not a justification, and the same funnel at one day versus thirty days tells different stories. Pick the window from observed time-to-convert behavior and state it. A window chosen for convenience produces a number chosen for convenience.

### Making The Funnel So Long Every Step Looks Like Failure

Each additional step multiplies its conversion rate into the final number, so long funnels compound small losses into apparent collapse. Keep the primary funnel short and push detail into drill-downs, or every step will look like a problem.

### Defining Entry Too Broadly Or Too Narrowly

A broad entry dilutes the funnel with low-intent users and depresses rates; a narrow entry hides how many never reach the funnel. Choose the entry deliberately based on the question, and document it so the rate is reproducible.

### Forcing A Branching Journey Into A Linear Funnel

Real journeys loop, skip, and reorder, and a strictly sequential funnel mislabels these users as drop-off. Check whether the flow is genuinely sequential, and use milestone-based views where users move non-linearly.

### Trusting The Funnel Without Reconciling To Ground Truth

A funnel that does not match operational counts is hiding a definition error such as double-counting or a window that captures cancellations. Reconcile against known outcomes before acting on the rates.

## Self-Check

- [ ] Each funnel step represents a genuine user decision or commitment, not merely a screen transition.
- [ ] The primary funnel is short enough that each step is interpretable, with detail pushed into drill-downs.
- [ ] The conversion window is chosen from observed time-to-convert behavior and is stated alongside every rate.
- [ ] Funnel entry and exit are deliberately defined and documented, including exclusions for internal and duplicate events.
- [ ] The journey was checked for sequentiality, and non-linear flows use milestone-based or segmented views rather than a strict funnel.
- [ ] Step granularity matches the decision being informed, coarse for locating loss and detailed for diagnosing it.
- [ ] The funnel output is reconciled against independent operational counts at entry, completion, and intermediate steps.
- [ ] No step is included merely because the event exists in the log; each earns its place as a real decision.
- [ ] The window, boundaries, and exclusions are documented so the rate is reproducible by another analyst.
- [ ] A drill-down exists for the step with the largest drop, so the primary funnel can locate loss and the detail can diagnose it.
