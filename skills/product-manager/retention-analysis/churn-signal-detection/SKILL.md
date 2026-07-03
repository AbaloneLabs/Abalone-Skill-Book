---
name: churn_signal_detection.md
description: Use when the agent is identifying churn signals and leading indicators of disengagement, building early warning from usage decline, distinguishing noise from real churn risk, or deciding which users are at risk before they actually leave.
---

# Churn Signal Detection

Churn is the moment a user stops being a customer, but by the time churn is observed it is almost always too late to act on it. The useful work is detecting the signals that precede churn, early enough that intervention has a chance, and separating those signals from the ordinary noise of variable human behavior. This is a prediction and judgment problem, not a reporting problem.

Agents miss this because churn feels like a clean outcome to measure. They build a list of users who cancelled or went inactive, call it the churn report, and treat detection as done. The harm is that the team only ever reacts after the loss, when save offers are expensive and often pointless. The opposite failure is over-eager detection: flagging every dip in usage as churn risk, overwhelming customer success with false positives until the warnings are ignored even when they are right.

Use this skill before answering broad questions such as "which users are about to churn", "what are our leading indicators of churn", "how do we build an early warning system", "is this usage drop real or noise", or "who should customer success reach out to this week". The goal is to prevent the agent from reporting churn as a backward-looking fact while ignoring the forward-looking signals that could change the outcome.

## Core Rules

### Separate Leading From Lagging Signals

Lagging signals confirm churn after it has happened: cancellation, non-renewal, account deletion, login cessation. Leading signals precede it and are where intervention is possible: declining session frequency, shrinking breadth of features used, drop in the primary value action, rising support tickets, failed payments, or a key contact changing roles.

Build the detection system around leading signals, and use lagging signals only to validate that the prediction was correct. A churn report built entirely on lagging signals is an obituary, not a warning. For each candidate signal, ask how far in advance it tends to appear and whether the lead time is long enough to act.

### Define The Value Action And Watch Its Decline

The strongest leading signal is decay in the action that represents core product value, not overall activity. A user who still logs in but has stopped performing the primary value action is more at risk than one whose session count dipped during a busy week.

Identify the value action the same way you would for retention: the behavior that proves the user got the product's benefit. Then track not just whether it happens but its trend, frequency, and recency. A user whose value-action frequency has dropped by half over three weeks is a stronger signal than one who had one quiet week.

### Combine Multiple Signals Into A Risk Picture

No single signal is reliable enough to act on. A usage dip might be a vacation, a billing failure might be a stale card, a support spike might be a single frustrating bug. Combining weak signals into a composite risk score is more robust than reacting to any one of them.

A useful composite considers recency (how long since the last value action), frequency trend (is usage declining over time), breadth (is the user narrowing to fewer features), and friction events (support tickets, failed payments, errors encountered). Weight signals by how strongly they have historically predicted churn in this product, and validate the weights against actual outcomes rather than intuition.

### Separate Signal From Noise With Thresholds And Trends

Human usage is noisy. A week of low activity is normal; a sustained decline is not. The detection rule must distinguish a real trend from ordinary variance, or the system will cry wolf constantly.

Use trend over multiple periods rather than a single snapshot, and set thresholds based on the distribution of normal behavior for each segment. A power user dropping to average activity is a signal; a casual user staying casual is not. Consider statistical or rolling baselines so the threshold adapts to each user's own pattern instead of a one-size-fits-all cutoff.

### Segment At-Risk Users By Saveability And Value

Not every at-risk user is worth saving, and not every save is possible. A detection system that flags everyone equally wastes effort on accounts that were never viable or that no intervention can rescue.

Layer value and saveability onto the risk score. A high-value, high-risk, saveable account is the top priority for outreach. A low-value at-risk account may not warrant manual effort. An account whose churn is driven by factors outside the product (budget cuts, company shutdown, role change) may not be saveable regardless of signal strength. Rank the list, do not just produce it.

### Validate Predictions Against Actual Outcomes

A churn detection system is a hypothesis generator. Its quality is measured by precision (of those flagged, how many actually churned), recall (of those who churned, how many were flagged), and lead time (how far in advance the flag appeared). Without measuring these, the team has no idea whether the signals are real.

Track flagged users through to their actual outcome, and refine the model as data accumulates. A signal that fires constantly but rarely predicts churn should be down-weighted or dropped; a signal that fires rarely but predicts churn reliably should be elevated. Treat the detection rules as a product that improves with feedback, not a one-time configuration.

### Distinguish Voluntary, Involuntary, And Seasonal Churn

Churn has different causes and different signals. Voluntary churn (the user chose to leave) is preceded by engagement decline and dissatisfaction signals. Involuntary churn (failed payment, expired card, access revoked) is often a billing or operations problem, not an engagement one. Seasonal churn reflects the product's natural cycle and is not always worth intervening on.

Classify churn by type, because the detection signals and the right response differ for each. Treating involuntary churn with engagement outreach wastes effort; treating seasonal churn as a crisis burns the team out. Match the signal to the cause.

## Common Traps

### Building A Backward-Looking Churn Report

Reporting who already churned feels rigorous but is useless for prevention. The team celebrates the accuracy of the report while continuing to lose users it could have saved.

### Flagging Every Usage Dip As Risk

Reacting to single-period dips without considering normal variance floods customer success with false positives. Within weeks the warnings are ignored, including the ones that mattered.

### Watching The Wrong Event

Tracking login or session decline instead of decline in the value action produces noisy signals. A user can keep logging in while having effectively disengaged from the product's core benefit.

### Treating All At-Risk Users Equally

A flat list of at-risk accounts ignores value and saveability. Effort gets spent on low-value or unsaveable accounts while the high-value saveable ones do not get enough attention.

### Trusting The Model Without Validation

Composite risk scores feel authoritative, but without measuring precision, recall, and lead time against real outcomes, the team is acting on untested assumptions dressed up as data.

### Confusing Involuntary Churn With Disengagement

Failed payments and expired cards churn users who were otherwise engaged. Treating these as engagement problems and sending save campaigns misses the actual fix, which is usually billing operations.

### Ignoring External Causes Of Churn

Role changes, company budget cuts, mergers, and project endings cause churn that no product improvement can prevent. Flagging these accounts as saveable burns effort and sets up the team to feel they failed when the cause was outside their control.

## Self-Check

- [ ] The detection system is built on leading signals, not only lagging outcomes like cancellation or deletion.
- [ ] The primary value action is defined, and its decline is tracked rather than generic activity or login.
- [ ] Multiple weak signals are combined into a composite risk score rather than acting on any single signal.
- [ ] Thresholds distinguish real trends from normal usage variance, using multi-period trends or per-user baselines.
- [ ] At-risk users are ranked by value and saveability, not presented as an undifferentiated list.
- [ ] Predictions are validated against actual outcomes for precision, recall, and lead time, and the model is refined from that feedback.
- [ ] Churn is classified as voluntary, involuntary, or seasonal, with detection signals and responses matched to each type.
- [ ] External causes of churn (role change, budget, company events) are recognized and excluded from save efforts where appropriate.
- [ ] The system produces few enough false positives that customer success still trusts and acts on the warnings.
- [ ] Lead time between signal and churn is long enough that intervention is actually possible.
