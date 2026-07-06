---
name: telemetry-and-data-interpretation.md
description: Use when the agent is instrumenting a game for analytics, defining events and metrics, interpreting retention or funnel drop-off dashboards, evaluating A/B test results, distinguishing correlation from causation in player behavior data, or deciding whether a telemetry signal justifies a design or balance change.
---

# Telemetry and Data Interpretation

Telemetry is the instrumentation and analysis of what players actually do at scale, as opposed to what they say in playtests or reviews. The judgment problem is that telemetry feels objective — the numbers came from real players, the dashboard is authoritative — so teams treat any clear-looking metric as proof. But telemetry measures behavior under the specific conditions the game created, and those conditions are full of confounds: a feature looks unpopular because the tutorial never taught it, a level looks too hard because a bug spawned extra enemies, retention looks fine because a cohort effect is masking a collapse in a segment. Agents tend to miss the important issues because the data is abundant and the confounds are invisible; you can always find a number that supports the decision you already wanted. The harm this prevents is shipping balance or monetization changes based on misread dashboards, "fixing" a feature whose poor numbers were caused by something upstream, or declaring an experiment a success when it was never actually measuring what the team thought. Worse, telemetry without interpretation discipline leads to local optimization — chasing whatever is easy to measure — while the unmeasured parts of the experience quietly rot. The agent has freedom to choose metrics and dashboards, but the validity safeguards — defining events precisely, pre-registering hypotheses, and never conflating correlation with causation — are mandatory. This skill covers the decisions that determine whether telemetry informs design or misleads it.

## Core Rules

### Define Events and Metrics Precisely Before Shipping Instrumentation

Every telemetry event must have an unambiguous definition: what triggers it, what parameters it carries, and what question it answers. "Level complete" is not a definition — does it fire on the victory screen, the moment the win condition is met, or when rewards are granted? Ambiguous definitions produce metrics that look consistent but measure different things across builds, and the drift is invisible until someone tries to reason across versions. Write a data dictionary that maps each event to the decision it supports, and audit it whenever the feature changes. The decision criterion is that if a metric cannot be traced from the dashboard tile back to a single documented event trigger, it cannot be trusted to drive a change. When teams instrument "everything just in case," they get noise and storage cost without the discipline of knowing what each signal means.

### Tie Every Metric to a Decision, Not to Curiosity

Dashboards accumulate metrics because they are cheap to add, and soon the team is watching forty numbers that nobody acts on. The trap is that curiosity-driven metrics create the illusion of insight while consuming attention, and they invite post-hoc storytelling where any movement is explained after the fact. For each metric, name the decision it would change and the threshold at which it would change it. If you cannot, the metric is decoration. The rule is to prefer a small set of decision-linked metrics over a large set of ambient ones, because the small set forces you to define what success and failure actually look like before the data arrives, rather than rationalizing whatever arrives.

### Pre-Register the Hypothesis Before Reading the Result

The single most important discipline in data interpretation is to state what you expect and why before you look at the numbers. Without a pre-stated hypothesis, the human brain will find a plausible story in any dataset — this is confirmation bias wearing a lab coat, and it is especially dangerous with rich telemetry where dozens of segments and cuts are available to mine for a flattering result. Write down the metric, the predicted direction, and the reasoning before unblinding the data. The decision rule is that a finding consistent with a pre-registered hypothesis is strong evidence; a finding discovered by slicing the data after the fact is a hypothesis to test next time, not a basis for shipping a change. When the team wants to act on an unexpected pattern, convert it into a pre-registered follow-up rather than treating the surprise as proven.

### Separate Correlation From Causation Ruthlessly

Telemetry shows what co-occurs, not what causes what. Players who engage with the crafting system retain better — but did crafting cause retention, or do retained players simply have more time to discover crafting? The confound is selection: the players who reach any optional feature are already the more committed ones, so every optional feature will appear correlated with retention. Establishing causation requires either an experiment (random assignment, A/B test) or a causal argument grounded in the design, not a correlation coefficient. The decision criterion is that a correlational finding can justify investigating or hypothesizing, but only an experiment or a strong mechanistic argument justifies a change. When an A/B test is impossible, lower the confidence of causal claims and say so.

### Design Experiments That Actually Isolate the Variable

A/B tests are powerful but easy to corrupt. Common failures include: too few players to detect the real effect size (the test "shows no difference" because it was underpowered), peeking at results early and stopping when they look good (inflates false positives), testing changes so small no player could notice them, or testing changes bundled with other changes so the cause of any movement is unattributable. Before running a test, define the minimum effect size worth detecting, compute the required sample, decide the stopping rule in advance, and change only one variable. The decision rule is that a test which violates these constraints produces a number that feels decisive but is not — and an underpowered "no effect" result is often more dangerous than no test at all, because it is used to kill ideas that were never fairly evaluated.

### Segment Data to Reveal What Aggregates Hide

Aggregate metrics average over populations that behave differently, and the average can be stable while important segments collapse or surge. Retention looks fine overall, but new players are churning while veterans are sticking — a net-neutral number masking a worsening problem. Always segment by the variables that matter for the question: player tenure, acquisition source, platform, region, spend tier, and prior behavior. The decision criterion is that an aggregate finding should be cross-checked against key segments before it drives a decision, because a change that helps the average may harm the segment the game most needs to grow. Beware the opposite trap: slicing into too many segments produces small samples and spurious differences, so segment deliberately, not exhaustively.

## Common Traps

### The Goodhart's Law Collapse

Once a metric becomes a target, players and teams optimize for the metric rather than the underlying experience it was meant to represent. Session length is a target, so designers pad with grinding; a daily-active metric is a target, so the game adds friction that forces logins. The trap is that the metric keeps rising while the actual experience degrades, and because the dashboard looks healthy, nobody investigates. The false signal is the improving number; the harm is a game that measures well and feels worse, until retention eventually collapses for reasons the metric never captured. The defense is to track multiple independent metrics and watch for divergence, especially between engagement metrics and satisfaction or retention quality.

### Survivorship Bias in Funnel Analysis

A funnel shows that players who reach level 10 retain at 80%, so the team concludes the mid-game is great. The trap is that only the already-committed players reached level 10 — the ones who would have churned already did, earlier. The 80% is a measure of the survivors, not of the level design. The false signal is the high number deep in the funnel; the real question is how many reached that point and what the cumulative drop-off actually was. This trap causes teams to celebrate late-game metrics while ignoring the upstream bleeding that determines whether anyone gets there.

### Mistaking Bug-Driven Behavior for Player Preference

Telemetry shows players avoid a zone, so the team concludes the zone is unpopular and deprioritizes it. In reality, a collision bug made the zone frustrating, or a quest marker pointed the wrong way, or the entrance was genuinely hard to find. The trap is that the data accurately reports avoidance but misattributes the cause to design preference. The false signal is the clean behavioral pattern; the hidden cause is a technical or UX failure that, if fixed, would reverse the conclusion. This trap causes permanent design changes for problems that were temporary bugs, and it is why telemetry findings should be cross-checked against known issues before acting.

### Reading Noise as Signal

Dashboards update in real time, and every wiggle invites a story. A 2% dip in day-1 retention triggers an emergency meeting, but the dip is within normal variance and means nothing. The trap is that humans see patterns in randomness and feel compelled to explain every movement. The false signal is the visible change on the chart; the reality is statistical noise. This trap causes constant low-value investigation and reactive changes that themselves introduce variance, making the data harder to read over time. The defense is to know the normal variance band of each metric and ignore movements within it.

### Ignoring Cohort Effects

A retention metric looks stable quarter over quarter, so the team believes the game is healthy. But each quarter's new players are churning faster than the last, masked because older, stickier cohorts still dominate the average. The trap is that aggregate stability hides a deteriorating acquisition experience or a changing player population. The false signal is the flat headline number; the real signal is visible only in cohort-by-cohort analysis. This trap causes teams to discover decline only when the legacy cohorts finally age out and the average collapses, far later than the data could have warned them.

## Self-Check

- Does every event and metric trace to a documented definition and a specific decision it would change, with curiosity-only metrics removed from decision dashboards?
- Did I pre-register the hypothesis — metric, predicted direction, and reasoning — before reading the result, and am I treating post-hoc discoveries as hypotheses for next time rather than proven findings?
- Have I distinguished correlation from causation, and where causation is claimed, is it backed by an experiment or a mechanistic argument rather than a coefficient?
- For any A/B test, did I define the minimum detectable effect, compute required sample size, set a pre-committed stopping rule, and isolate a single variable?
- Did I cross-check aggregate findings against meaningful segments (tenure, platform, cohort, spend tier) to ensure the average is not hiding a collapsing or surging subgroup?
- Am I watching for divergence between target metrics and quality indicators (Goodhart), and have I confirmed that behavioral patterns are not caused by upstream bugs before attributing them to preference?
- Are movements I am reacting to outside the normal variance band of the metric, and am I using cohort analysis rather than aggregates to detect slow-moving decline?
