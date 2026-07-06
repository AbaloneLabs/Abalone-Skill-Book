---
name: leading_indicators_and_input_metrics.md
description: Use when the agent is defining leading indicators that predict the North Star, choosing input metrics the team can directly influence, building a metric tree from inputs to outcomes, or distinguishing metrics that predict success from those that only describe it.
---

# Leading Indicators And Input Metrics

A North Star metric describes success, but it usually moves slowly and is influenced by many factors, so it cannot guide day-to-day work. To make the North Star actionable, the team needs leading indicators and input metrics: measures that predict the North Star, that respond quickly to changes, and that the team can directly influence through its work. Done well, a metric tree connects daily effort to the North Star through a chain of cause and effect, so that every team understands how their work moves the ultimate outcome. Done poorly, the team tracks metrics that correlate with success but do not cause it, or metrics that move freely without affecting the North Star at all. Agents often build metric trees by listing every measurable number rather than by establishing which metrics actually drive the outcome.

The harm this skill prevents is effort spent moving numbers that do not matter. A team optimizing a vanity metric or a correlated-but-non-causal metric will hit its targets while the North Star stays flat, because the metric it pursued had no real leverage on the outcome. The discipline of distinguishing causes from correlates is what makes a metric tree useful rather than decorative.

Use this skill before answering questions such as "what leading indicators should we track", "how do we build a metric tree", "what input metrics predict our North Star", or "why do our metrics move but our outcomes do not". The goal is to prevent the agent from building a metric tree on correlation rather than causation.

## Core Rules

### Distinguish Metrics That Cause The Outcome From Metrics That Merely Correlate

The central error in metric trees is confusing correlation with causation. Many metrics move alongside the North Star without driving it, because they share a common cause or because the relationship is coincidental. Optimizing a correlated metric produces no movement in the outcome. The useful leading indicators are those that, when deliberately moved, cause the North Star to move. Establishing causation requires more than observing that two metrics rise together; it requires a theory of how one produces the other and, ideally, evidence from experiments.

For each candidate leading indicator, state the causal mechanism: how does changing this metric change the North Star? If the mechanism is vague or absent, the metric is probably a correlate, not a cause. Prioritize metrics with clear causal stories, and treat the rest as diagnostic signals rather than optimization targets.

### Choose Input Metrics The Team Can Directly Influence

A leading indicator is only useful for guiding work if the team can move it through its actions. Metrics that describe conditions the team cannot change, however predictive, are not actionable and tend to create frustration or gaming. The best input metrics sit close enough to the team's work that effort produces visible movement, creating a tight feedback loop between action and signal.

Map each input metric to the specific work that moves it. If no clear work maps to the metric, either the metric is not an input or the team lacks the lever to influence it. In the latter case, the gap between work and metric is itself diagnostic: it reveals where the team lacks leverage on the outcome.

### Build A Metric Tree That Connects Inputs To The North Star

A metric tree traces the chain from controllable inputs through intermediate outcomes to the North Star, making explicit how daily work is supposed to produce long-term success. Each link in the tree should represent a causal relationship, not merely a correlation. The tree's value is that it lets every team locate its work on the path to the outcome and understand which lever it owns. Without the tree, teams optimize local metrics with no clear connection to the ultimate goal.

Draw the tree and test each link. If a link is assumed rather than established, mark it as a hypothesis and plan to validate it. A metric tree full of untested assumptions is a story, not a model, and optimizing against it may move nothing that matters.

### Favor Leading Indicators Over Lagging Descriptions

Lagging metrics describe what already happened; they are satisfying to report but useless for guiding action, because by the time they move, the work that caused the movement is long past. Leading indicators change before the outcome does, giving the team time to adjust. A retention rate is lagging; the behaviors that predict retention, such as activation or early engagement patterns, are leading. Build the metric set around leading indicators, with lagging metrics reserved for confirming that the leading ones are working.

The tradeoff is that leading indicators are often less certain than lagging ones, because they predict rather than describe. Accept that uncertainty, because the alternative is steering by the rearview mirror. A slightly uncertain leading indicator that lets the team adjust is more valuable than a certain lagging metric that arrives too late to act on.

### Validate The Causal Links With Experiments

Because the value of a metric tree depends on causation, the links should be validated wherever possible through experiments. When the team deliberately moves an input metric, does the North Star respond as the tree predicts? If not, the link is wrong, and continuing to optimize the input wastes effort. Experiments turn the metric tree from an assumption into a tested model.

Where experiments are not feasible, validate links through natural variation and historical analysis: when the input metric moved in the past, did the outcome follow? Triangulate evidence from multiple sources, because single-source validation is vulnerable to confounding. Treat the metric tree as a living model that is refined as evidence accumulates.

### Avoid Metric Proliferation That Dilutes Focus

A metric tree can grow to include dozens of metrics, at which point it stops focusing effort and starts diffusing it. Every additional tracked metric competes for attention and implies its own optimization. Keep the tree lean: a small number of input metrics per team, each clearly causal and actionable, connected through a few intermediate outcomes to the North Star. More metrics do not mean more insight; they mean less focus.

When stakeholders request additional metrics, distinguish between metrics worth optimizing and metrics worth merely monitoring. Not every measurable number deserves to be a target. Reserve optimization status for the few metrics with clear causal leverage.

## Common Traps

### Correlation Mistaken For Causation

Building a tree from metrics that move with the North Star without driving it. The trap is optimizing numbers that have no leverage on the outcome.

### Lagging Metrics Treated As Actionable

Tracking retention or revenue as a guide to work. The trap is steering by outcomes that have already been determined, too late to change them.

### Input Metrics The Team Cannot Influence

Choosing predictive metrics that no team action can move. The trap is frustration, gaming, or abandonment of the metric as a guide.

### An Untested Metric Tree

Drawing causal links on assumption alone. The trap is a plausible-looking model that breaks the moment it is acted on.

### Metric Proliferation

Tracking so many metrics that focus dilutes. The trap is every team optimizing its own number with no coherent effect on the outcome.

### Optimizing Local Metrics With No Line Of Sight To The North Star

Each team pursuing its metric without understanding the connection. The trap is locally rational work that collectively fails to move the outcome.

## Self-Check

- [ ] Each leading indicator has a stated causal mechanism explaining how it drives the North Star, not merely a correlation.
- [ ] Input metrics are ones the team can directly influence through its work, with clear levers mapped to each.
- [ ] A metric tree connects inputs through intermediate outcomes to the North Star, with each link representing causation.
- [ ] The metric set favors leading indicators over lagging descriptions, accepting prediction uncertainty for earlier signal.
- [ ] Causal links are validated through experiments or historical analysis, and untested links are marked as hypotheses.
- [ ] The number of optimization-target metrics is kept lean to preserve focus.
- [ ] Every team can locate its work on the tree and explain how it contributes to the North Star.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
