---
name: benchmark_selection.md
description: Use when the agent is choosing or evaluating a benchmark for a portfolio or strategy, comparing to a market index or custom blended benchmark, assessing benchmark appropriateness, detecting style drift or return cyclicality, or judging whether performance comparisons are fair, unbiased, and free of survivorship or selection effects.
---

# Benchmark Selection

A benchmark is the standard against which performance and risk are judged. It sounds administrative, but the choice of benchmark determines whether "outperformance" is real skill, hidden beta, or a misleading comparison. Agents often pick a convenient index, discover the portfolio "beat" it, and declare success, when the benchmark was simply wrong for the strategy. The judgment problem is that an inappropriate benchmark can make a bad strategy look good and a good strategy look bad, and the error is invisible until examined.

This skill is for selecting and validating a benchmark before drawing performance conclusions.

## Core Rules

### Match The Benchmark To The Strategy's Risk Profile

A valid benchmark must reflect the same market, sector, style, capitalization, currency, and risk exposures as the portfolio. Comparing a small-cap value strategy to a large-cap growth index is meaningless.

Verify alignment on:

- asset class and geography;
- market capitalization range;
- style and factor exposures (value, growth, quality, momentum);
- currency exposure;
- duration and credit quality for fixed income;
- liquidity and investability.

If the portfolio and benchmark have different risk profiles, any return difference is partly structural, not skill. Choose the benchmark that best mirrors the strategy's intended exposures.

### Prefer A Custom Blend For Multi-Style Portfolios

Few real portfolios fit a single index. A blend of indices weighted to the strategy's policy mix usually fits better.

Build:

- a composite benchmark from indices matching each sleeve;
- weights that reflect the policy allocation;
- a rebalancing rule for the blend that matches the portfolio's rebalancing;
- currency and fee adjustments where relevant.

A custom blended benchmark is more work but far more honest than forcing a complex portfolio into a single mismatched index.

### Check For Style Drift Against The Benchmark

A strategy that drifts from its stated style will diverge from its benchmark for reasons unrelated to skill. Drift can be accidental or deliberate ("chasing the market").

Monitor:

- the portfolio's factor exposures over time versus the benchmark;
- sector and country tilts that appear and disappear;
- cash levels and their effect on relative return;
- whether the manager is staying in style or migrating toward whatever is working.

Style drift against the benchmark invalidates the comparison and often signals risk the investor did not sign up for.

### Account For Cyclicality And Regime Bias

A benchmark comparison over one regime can flatter or punish a strategy unfairly. Value underperforms in growth regimes; momentum crashes in reversals; defensive strategies lag in rallies.

Assess:

- the market regime during the measurement period;
- whether the strategy's style was in or out of favor;
- performance across multiple full cycles, not one favorable stretch;
- the strategy's expected behavior in the current regime.

A strategy that "beat its benchmark" during a regime tailor-made for its style has proven little. Look through the cycle.

### Beware Of Survivorship, Selection, And Look-Ahead Bias

Benchmark and peer comparisons can be distorted by biases baked into the data.

Check:

- whether the index or peer group includes only survivors (survivorship bias);
- whether the benchmark was selected after seeing that it made performance look good (selection bias);
- whether the comparison uses point-in-time data or includes information not available at the time (look-ahead bias);
- whether fees, trading costs, and taxes are applied consistently.

Biased comparisons manufacture outperformance that evaporates under honest measurement.

### Use Net-Of-Fee, Net-Of-Cost Returns

Gross returns flatter active strategies. The investor only keeps the net.

Compare:

- net-of-fee returns for both portfolio and benchmark (or benchmark minus a comparable fee);
- transaction costs and drag;
- tax efficiency for the investor's situation;
- the compounding effect of fees over the full period.

A strategy that beats gross but loses net has no value to the investor.

### State The Time Horizon And Significance

Short periods produce noisy, meaningless comparisons. Significance matters.

Report:

- the measurement period and whether it spans a full cycle;
- the volatility of the return difference (tracking error);
- whether the outperformance is statistically meaningful or within noise;
- rolling-period hit rates, not just one cumulative number.

A one-year "beat" is often noise. Demand enough history and the right significance lens before concluding skill.

## Common Traps

### Using A Convenient But Mismatched Index

Comparing a strategy to a well-known index that does not match its style is the most common benchmark error and the easiest way to manufacture false outperformance.

### Benchmark Selection After The Fact

Choosing the benchmark that makes the period look best is selection bias. The benchmark should be specified in advance.

### Ignoring Style Drift

A drifting strategy diverges from its benchmark for non-skill reasons. Without monitoring exposures, the comparison is invalid.

### One-Regime Comparisons

A strategy that outperformed in a favorable regime has proven little. Conclusions drawn from one cycle often reverse in the next.

### Gross Returns And Hidden Costs

Gross-of-fee comparisons flatter active management. Net comparisons often erase the apparent edge.

### Survivorship In Peer Groups

Peer-group medians that include only surviving funds overstate typical performance and make an average strategy look below-average.

### Treating Noise As Skill

A return difference smaller than the tracking error is statistically indistinguishable from luck. Concluding skill from noise leads to overconfidence.

## Self-Check

- [ ] The benchmark matches the strategy's asset class, style, cap range, currency, duration, and liquidity.
- [ ] A custom blended benchmark is used where the portfolio spans multiple styles, with weights and rebalancing matching the policy.
- [ ] Factor exposures and style drift were monitored against the benchmark over time.
- [ ] Performance was assessed across multiple regimes and full cycles, not one favorable period.
- [ ] Survivorship, selection, and look-ahead biases in the benchmark or peer group were checked.
- [ ] Returns are compared net of fees, costs, and taxes where relevant.
- [ ] The time horizon, tracking error, statistical significance, and rolling hit rates are reported, not just one cumulative number.
- [ ] The recommendation avoids declaring skill from short or biased comparisons, and notes that benchmark choice materially changes conclusions and that professional analysis may be warranted for complex strategies.
