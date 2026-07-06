---
name: data-driven-balance-and-iteration.md
description: Use when the agent is using telemetry to balance and iterate the game, interpreting player behavior data to tune difficulty and economy, running A/B tests, or evaluating whether data-driven decisions improve the game or produce metric-optimization that harms the player experience, local optimization, and the false confidence of acting on misread data.
---

# Data-Driven Balance and Iteration

Data-driven design — using telemetry to balance and iterate the game — promises objectivity and continuous improvement, and it is also where design most commonly degrades through metric-optimization that harms the experience, local optimization that misses the global picture, and the false confidence of acting on misread data. The judgment problem is that data must inform (not replace) design judgment, that the metrics that are easy to measure (sessions, engagement) are often not the metrics that reflect player wellbeing or long-term health, and that A/B testing can optimize for the wrong outcome. Agents tend to miss this because the data feels objective (the numbers say), while the data reflects only what was measured (not what matters), and because optimizing a visible metric (engagement) can degrade an invisible one (player wellbeing, long-term retention). The harm is games that are optimized into engagement-extracting grind, balance changes that fix one problem while creating another, and the false confidence of data-driven decisions that were actually misread. This skill covers how to use data to inform design without degrading it, interpret data with appropriate humility, and avoid the metric-optimization traps. The agent has latitude in the analysis, but the obligation to weigh data against design judgment and player wellbeing is not optional.

## Core Rules

### Use Data to Inform, Not Replace, Design Judgment

Data should inform design decisions — providing evidence about player behavior — but not replace design judgment, because data shows what is happening but not what should be happening, and the interpretation requires design understanding of what the data means for the experience. The decision rule: use data as evidence to inform decisions, apply design judgment to interpret what the data means, and avoid deferring to data without judgment. Data-without-judgment produces misinformed decisions, because the numbers were acted on without understanding.

### Beware Optimizing Visible Metrics at the Expense of Invisible Ones

The metrics that are easy to measure (session length, daily engagement) are visible, while the metrics that reflect player wellbeing and long-term health (satisfaction, burnout, sentiment) are harder to measure, and optimizing the visible can degrade the invisible. The decision rule: before optimizing a visible metric, identify the invisible metrics it may affect (wellbeing, sentiment, long-term retention), and avoid optimizations that improve visible at the expense of invisible. Visible-only optimization degrades invisible health, because the cost to wellbeing was not measured.

### Avoid Local Optimization That Misses the Global Picture

Optimizing a local metric (a single encounter's completion rate, a single feature's engagement) can degrade the global experience (the overall pacing, the overall engagement), and data-driven iteration must consider the global picture, not just the local metric being optimized. The decision rule: before optimizing a local metric, assess the global impact, and avoid local optimizations that harm the global experience. Local-only optimization degrades the global game, because the local fix created a global problem.

### Design A/B Tests With Clear Hypotheses and Correct Metrics

A/B testing requires clear hypotheses (what change, what expected outcome) and correct metrics (measuring what matters, not what is easy), because A/B tests without clear hypotheses or with wrong metrics produce misleading results that drive wrong decisions. The decision rule: for each A/B test, define the hypothesis and the metric that reflects the intended outcome, ensure the metric is correct, and avoid testing without clear design. Hypothesis-less A/B testing misleads, because the test was not designed to answer a question.

### Interpret Data With Humility About Causation and Confounds

Data shows correlation, not necessarily causation, and confounds (other factors affecting the metric) can produce misleading interpretations, so data must be interpreted with humility about what it actually proves, not with false confidence. The decision rule: when interpreting data, consider alternative explanations and confounds, avoid claiming causation from correlation, and seek confirming evidence before acting. Overconfident interpretation produces wrong decisions, because correlation was mistaken for causation.

### Balance Quantitative Data With Qualitative Player Feedback

Quantitative data (what players do) must be balanced with qualitative feedback (what players say and feel), because the two reveal different things — data shows behavior, feedback shows experience — and either alone is incomplete. The decision rule: combine quantitative telemetry with qualitative feedback (surveys, forums, reviews), cross-check the two, and avoid relying on either alone. Quantitative-only data misses the experience, because behavior does not reveal feeling.

## Common Traps

### Deferring to Data Without Design Judgment

The team, trusting the objectivity of data, defers to the numbers without applying design judgment, and the data-driven decisions miss what the data means for the experience. The trap is that data feels objective. The false signal is that the decisions are evidence-based. The harm is that the data shows what is happening but not what should be, the decisions act on the numbers without understanding, the experience is degraded by changes that the data "justified," and the design judgment that should interpret the data is absent, because the team deferred rather than interpreted.

### Optimizing Visible Metrics, Degrading Invisible Health

The team optimizes a visible metric (session length, engagement) without considering the invisible metrics (wellbeing, sentiment), and the optimization degrades the invisible health. The trap is that the visible metric improves. The false signal is that the data shows improvement. The harm is that the engagement optimization may extract engagement through compulsion, the player wellbeing degrades, the long-term retention and sentiment that are harder to measure decline, and the game's health is traded for the visible metric, because the invisible cost was not measured.

### Local Optimization Harming the Global Game

The team optimizes a local metric (one encounter, one feature) without assessing global impact, and the local optimization degrades the global experience. The trap is that the local metric improves. The false signal is that the data shows the change worked. The harm is that the local fix creates a global problem (the pacing shifts, the overall engagement changes), the global experience degrades, the local optimization that looked successful actually harmed the game, and the team acts on the local success while the global decline accumulates, because the global impact was not assessed.

### Hypothesis-Less A/B Testing Producing Misleading Results

The team runs A/B tests without clear hypotheses or correct metrics, and the tests produce misleading results that drive wrong decisions. The trap is that A/B testing feels rigorous. The false signal is that the tests provide data. The harm is that the tests without hypotheses measure the wrong thing, the results are misinterpreted, the decisions based on the misleading results are wrong, and the rigor of A/B testing is undermined by poor design, because the tests were not designed to answer clear questions.

### Overconfident Causation Claims From Correlation

The team interprets correlation as causation, claims the data proves a relationship, and acts on the overconfident interpretation. The trap is that the correlation is visible. The false signal is that the data shows a relationship. The harm is that the correlation may not be causal, a confound may explain the relationship, the decision based on the false causation is wrong, and the team acts with false confidence on a misread, because correlation was mistaken for proof.

### Quantitative-Only Data Missing the Experience

The team relies on quantitative telemetry without qualitative feedback, and the data shows behavior but not experience, and the decisions miss how players feel. The trap is that quantitative data is precise. The false signal is that the data shows what players do. The harm is that the behavior does not reveal the experience, the decisions act on behavior without understanding feeling, the changes that improve behavioral metrics may degrade the felt experience, and the player sentiment that determines long-term health is missed, because qualitative feedback was not combined with the quantitative.

## Self-Check

- Am I using data to inform design decisions while applying design judgment to interpret it, not deferring to numbers alone?
- Before optimizing a visible metric, have I identified and assessed the invisible metrics (wellbeing, sentiment) it may affect?
- Before optimizing a local metric, have I assessed the global impact to avoid harming the overall experience?
- Are A/B tests designed with clear hypotheses and correct metrics that reflect the intended outcome?
- Am I interpreting data with humility about causation and confounds, seeking confirming evidence before acting?
- Am I balancing quantitative telemetry with qualitative feedback to understand both behavior and experience?
- Did I confirm data-driven iteration improves the game rather than optimizing metrics at the expense of player experience?
