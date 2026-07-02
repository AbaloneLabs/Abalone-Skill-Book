---
name: alpha_beta_and_risk_adjusted.md
description: Use when the agent is measuring risk-adjusted performance using Sharpe, Sortino, Treynor, information ratio, alpha, beta, or M2, comparing returns per unit of risk, evaluating whether outperformance reflects skill or risk-taking, or judging the reliability and limitations of these metrics across strategies, regimes, and time periods.
---

# Alpha, Beta, And Risk-Adjusted Return

Raw returns are a poor measure of skill because they ignore risk. A strategy that returns more by taking more risk has not demonstrated skill; it has simply bought more exposure. Risk-adjusted metrics (Sharpe, Sortino, Treynor, information ratio, alpha, beta, M2) attempt to separate skill from risk-taking by expressing return relative to the risk taken. The judgment problem is that each metric captures a different notion of risk, each has specific blind spots, and all are noisy and regime-dependent. Agents routinely cite a single ratio, declare skill, and miss that the metric may be misleading or statistically meaningless.

This skill is for choosing and interpreting risk-adjusted metrics honestly.

## Core Rules

### Match The Metric To The Risk Question

Different metrics answer different questions. Using the wrong one gives a misleading answer.

Know the metrics:

- Sharpe ratio, excess return per unit of total volatility; rewards smoothness, penalizes upside and downside equally;
- Sortino ratio, excess return per unit of downside deviation; ignores upside volatility, better for strategies where only downside is harmful;
- Treynor ratio, excess return per unit of beta; rewards return for systematic risk only, suited to well-diversified portfolios;
- Information ratio, active return per unit of tracking error; measures the efficiency of active bets against a benchmark;
- alpha (Jensen's), return above what beta predicts; the candidate skill measure after market exposure is removed;
- M2 (Modigliani), Sharpe expressed in return units at a benchmark volatility; intuitive, comparable across portfolios.

Choose the metric that matches the question. Sharpe for absolute risk efficiency, Sortino for downside focus, Treynor for diversified-portfolio beta, information ratio for active management, alpha for skill after beta.

### Remove Beta Before Claiming Alpha

High returns from high beta are not skill. Alpha is the residual after beta is accounted for, and it is the candidate measure of genuine edge.

Check:

- the portfolio's beta to the relevant market;
- whether the "alpha" survives after controlling for size, value, momentum, and other factors;
- whether the alpha is from a single favorable period or persistent;
- the statistical significance of the alpha estimate.

Much apparent alpha evaporates after factor control. What remains, if persistent and significant, is the strongest evidence of skill.

### Account For Non-Normality And Tail Risk

Sharpe and Treynor assume symmetric, well-behaved distributions. Real returns have fat tails, skew, and occasional crashes that these metrics hide.

Consider:

- Sortino for downside-specific risk;
- the worst drawdown and its recovery alongside Sharpe;
- the effect of a single outlier month on the ratio;
- measures that incorporate higher moments or tail risk (Calmar, Omega, conditional Sharpe).

A strategy can have a high Sharpe and a catastrophic tail. A high ratio from selling volatility, for instance, hides disaster risk that surfaces rarely but ruinously.

### Require Statistical Significance And Enough History

Risk-adjusted ratios are estimates with wide confidence intervals. A high ratio over a short period is often noise.

Assess:

- the length of the track record relative to the strategy's cycle;
- the standard error of the ratio estimate;
- rolling ratios over sub-periods for consistency;
- whether the ratio survives excluding the best single month.

A Sharpe difference of 0.2 between two strategies over three years is frequently indistinguishable from chance. Demand enough data and test robustness before concluding one strategy is better.

### Separate Skill From Regime And Factor Luck

A high ratio in one regime may reflect favorable factor exposure, not skill. The metric cannot tell the difference on its own.

Investigate:

- which factors were in favor during the period;
- whether the strategy's exposures happened to match the regime;
- performance across multiple regimes;
- whether the edge is structurally explained or coincidental.

A momentum strategy's high Sharpe in a trending market proves little about its behavior in a reversal. Look through regimes.

### Adjust For Leverage And Frequency

Sharpe and similar ratios are sensitive to the return measurement frequency and to leverage. Comparisons across strategies require consistency.

Standardize:

- the return frequency (monthly, quarterly) for comparability;
- annualization conventions;
- the effect of leverage, which scales return and volatility together, leaving Sharpe roughly unchanged but changing path risk;
- the effect of smoothing or stale pricing, which inflates ratios artificially.

A leveraged strategy and an unleveraged one can have the same Sharpe but very different blow-up risk. Frequency and leverage must be controlled before comparing.

### Include Net-Of-Cost And After-Tax Perspectives

Ratios computed on gross returns flatter active strategies. The investor experiences net and after-tax returns.

Report:

- net-of-fee and net-of-cost ratios;
- after-tax ratios for the investor's tax situation;
- the drag from turnover and taxes on the ratio over time.

A strategy with a strong gross Sharpe can have a weak net Sharpe once costs and taxes are included. Net is what the investor keeps.

## Common Traps

### Citing A Single Ratio As Proof Of Skill

One ratio over one period is usually noise. Skill requires persistence, significance, and factor control.

### Confusing Beta With Alpha

High returns from high beta are risk, not skill. Without removing beta and factors, "alpha" is just repackaged exposure.

### Ignoring Non-Normality

Sharpe and Treynor hide tail risk. Volatility-selling strategies show high ratios until they crash.

### Comparing Ratios Across Frequencies Or Leverage

Monthly and daily Sharpe ratios, or leveraged and unleveraged strategies, are not directly comparable without adjustment.

### Stale Pricing Inflating Ratios

Infrequently priced assets show artificially low volatility and inflated ratios. The true risk appears only in a forced sale.

### Gross-Of-Cost Ratios

Gross ratios flatter active management. Net ratios often erase the apparent edge.

### Survivorship And Selection In The Sample

Ratios computed only on surviving strategies or favorable periods overstate typical performance.

### Annualizing Short-Period Ratios

Annualizing a three-month Sharpe multiplies noise into a misleadingly precise number.

## Self-Check

- [ ] The metric chosen matches the risk question (Sharpe for total risk, Sortino for downside, Treynor for beta, information ratio for active, alpha for skill after beta).
- [ ] Beta and factor exposures were removed before claiming alpha, and the residual was tested for persistence and significance.
- [ ] Non-normality, tail risk, skew, and worst drawdown were considered alongside the chosen ratio.
- [ ] Statistical significance, standard error, rolling consistency, and the effect of the best month were assessed.
- [ ] Performance was examined across regimes, separating skill from factor and regime luck.
- [ ] Return frequency, leverage, and stale-pricing effects were controlled before comparing ratios.
- [ ] Ratios are reported net of fees, costs, and taxes where relevant.
- [ ] The recommendation avoids declaring skill from a single short-period ratio, acknowledges that risk-adjusted metrics are noisy and regime-dependent, and notes that professional analysis may be warranted for strategy evaluation.
