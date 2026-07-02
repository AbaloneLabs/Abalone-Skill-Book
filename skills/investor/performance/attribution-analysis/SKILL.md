---
name: attribution_analysis.md
description: Use when the agent is decomposing portfolio performance into sources such as asset allocation, security selection, and interaction effects, applying Brinson attribution, multi-factor or multi-level attribution, explaining alpha versus residual, or judging whether returns came from skillful decisions or from market beta, factor exposures, and luck.
---

# Attribution Analysis

Attribution analysis decomposes a portfolio's return into the decisions that produced it: how much came from asset allocation, how much from security selection, how much from factors, and how much is unexplained residual. The judgment problem is that without attribution, a manager or investor cannot tell whether returns came from skill, from taking compensated risk, or from luck. Agents often stop at "the portfolio beat the benchmark" and never ask why, which is exactly the question that matters for predicting whether the edge will persist.

This skill is for decomposing performance to separate genuine decision skill from risk exposure and noise.

## Core Rules

### Separate Allocation, Selection, And Interaction Effects

The classic Brinson framework splits active return into three pieces, and each tells a different story.

Decompose:

- allocation effect, the value from overweighting or underweighting asset classes that performed as expected;
- selection effect, the value from picking securities that beat their segment benchmark;
- interaction effect, the combined value when allocation and selection decisions reinforce or offset each other.

A portfolio can "beat the benchmark" through lucky allocation while losing on every selection decision, or vice versa. Attribution reveals which decisions actually drove the result.

### Use The Right Level Of Granularity

Attribution is only useful if the buckets are meaningful. Too coarse hides the story; too fine drowns it in noise.

Choose:

- asset-class or sleeve buckets for top-down allocation stories;
- sector, country, or factor buckets for more granular selection stories;
- a depth that matches the strategy's actual decision process;
- consistent buckets across periods so effects are comparable.

The decomposition should mirror how decisions are actually made. A factor-tilt strategy needs factor buckets; a country rotator needs country buckets.

### Extend To Multi-Factor And Multi-Level Attribution

Single-level Brinson misses factor exposures that drive returns invisibly. Modern attribution layers factor analysis on top.

Apply:

- a factor model to decompose returns into known factor exposures (market, size, value, momentum, quality, volatility);
- the residual after factors as the candidate "true alpha";
- multi-level attribution that nests asset class, sector, and security effects;
- fixed-income attribution (curve, sector, security selection) for bond portfolios.

If a strategy's "alpha" disappears after controlling for factors, the edge was factor exposure, not skill. This is one of the most important findings attribution can produce.

### Distinguish Skill From Beta And Factor Exposure

Most active return is beta or factor exposure, not skill. Attribution's main job is to make this visible.

Check:

- how much active return is explained by market beta;
- how much by documented factor tilts;
- how much remains as residual after these are removed;
- whether the residual is consistent or sporadic.

Persistent, positive residual after factor control is the strongest evidence of skill. Sporadic residual is usually luck. A large gross "alpha" that evaporates after factor decomposition is just packaged risk.

### Require Enough History And Significance

Attribution over a short period is dominated by noise. Effects must be measured over enough time and tested for significance.

Assess:

- the length of the track record and whether it spans cycles;
- the volatility of each effect over time;
- whether the allocation or selection effect is consistently positive or erratic;
- statistical significance versus random chance.

A one-year allocation "win" is nearly meaningless. Consistent effects over a full cycle, net of costs, are the minimum bar for claiming skill.

### Account For Costs And Trading

Attribution that ignores costs flatters active management. Costs are a real, persistent drag that must be assigned to the decisions that caused them.

Include:

- management fees as a separate negative effect;
- transaction costs assigned to the turnover that caused them;
- the cost of maintaining factor or style exposures;
- tax drag for taxable investors.

Net attribution often shows that the gross skill is real but consumed by costs. That distinction matters for the investor's net outcome.

### Connect Attribution To Forward Expectations

The point of attribution is not to praise or blame the past but to update expectations. An effect that was lucky should not be expected to repeat.

Ask:

- which effects have an economic or process reason to persist;
- which were regime-driven and may reverse;
- whether the strategy is still positioned to produce the same effects;
- what would have to be true for the edge to continue.

Attribution feeds forward-looking judgment. A skill that depended on a specific regime may not survive the next one.

## Common Traps

### Stopping At "Beat The Benchmark"

Total return comparison hides whether the win came from allocation, selection, or luck. Without decomposition, no lesson can be drawn.

### Ignoring Factor Exposures

A strategy can show "alpha" that is entirely explained by value, momentum, or size tilts. Without factor attribution, packaged risk is mistaken for skill.

### One-Period Attribution

A single period's attribution is noisy. Effects must be assessed for consistency and significance over time.

### Gross-Of-Cost Attribution

Ignoring fees and transaction costs flatters active management. Net attribution often erases the apparent edge.

### Misleading Bucket Choice

Buckets that do not match the decision process produce meaningless effects. The decomposition must mirror how decisions are made.

### Confusing Interaction With Skill

The interaction effect is often misattributed to allocation or selection. It should be reported separately and not used to inflate either.

### Survivorship And Selection In The Benchmark

If the benchmark is biased, the attribution is biased. Benchmark validity is a precondition for honest attribution.

### Over-Interpreting Residual

Residual after factors is candidate alpha, not proof of skill. It must be persistent and significant before skill is claimed.

## Self-Check

- [ ] Active return is decomposed into allocation, selection, and interaction effects using a framework that matches the decision process.
- [ ] The level of granularity (asset class, sector, country, factor) reflects how decisions are actually made.
- [ ] Multi-factor attribution is applied to separate documented factor exposure from residual candidate alpha.
- [ ] The analysis distinguishes skill from beta and factor exposure, and tests whether residual is persistent.
- [ ] Effects are measured over enough history spanning cycles, with consistency and significance assessed.
- [ ] Fees, transaction costs, and tax drag are included as separate negative effects.
- [ ] Attribution findings are connected to forward expectations, distinguishing persistent skill from regime-driven luck.
- [ ] The recommendation avoids claiming skill from short, gross, or factor-unexplained periods, and notes that attribution is a diagnostic requiring professional judgment for real strategy evaluation.
