---
name: regression_and_statistical_significance.md
description: Use when the agent is running regressions on financial data, interpreting betas and p-values, testing whether a relationship is real, assessing correlation significance, worrying about spurious correlation, p-hacking, multiple testing, or evaluating predictive models on returns. Covers statistical significance, spurious correlation, non-stationarity, multiple testing corrections, and avoiding false discoveries in finance.
---

# Regression And Statistical Significance

Financial data is noisy, non-stationary, and finite. Almost any two time series that trend upward over a long period will appear correlated. A regression run on such data will almost always produce a beta and a p-value, and those outputs will look authoritative. But a statistically significant result in finance is often an artifact of shared trends, small samples, data mining, or a model that violated its own assumptions. The danger is treating a regression output as proof of a real relationship when it is coincidence wearing a p-value.

Agents tend to over-trust regression statistics because they are precise and quantitative. A p-value below 0.05 feels like evidence. But in financial returns, with low signal-to-noise ratio, non-normal distributions, and pervasive common trends, standard significance tests are easily misled. The judgment problem is deciding when a regression result reflects a genuine economic relationship, when it is spurious, and how to demand robustness before acting on a finding.

This skill applies to running regressions, interpreting correlations and betas, testing factor or strategy relationships, evaluating predictive models, and guarding against data mining. It is not investment advice; statistical relationships in markets are unstable, significance does not imply causation, and historical fit does not guarantee future predictive power.

## Core Rules

### Test For And Remove Common Trends Before Inferring Relationship

Most financial time series are non-stationary: prices, earnings, and indices drift upward over time. Regressing two trending series produces spuriously high R-squared and significant t-statistics even when no real relationship exists. This is the classic spurious regression problem.

Before interpreting a regression, check for non-stationarity (unit root tests) and difference the data (use returns or changes rather than levels) or use cointegration analysis if a long-run equilibrium is theorized. A regression on price levels that looks great may be entirely spurious; the same regression on returns often shows no relationship. Always ask whether the significance survives transforming levels to changes.

### Require Economic Logic, Not Just Statistical Fit

A statistically significant relationship without an economic mechanism is suspect. In finance, where hundreds of variables can be tested against returns, some will correlate by chance. A plausible causal story (why would X predict returns, and through what mechanism) is necessary to elevate a correlation above coincidence.

Demand an explanation of the economic driver: risk compensation, behavioral bias, institutional constraint, informational friction, or structural feature. If the only evidence is the regression itself, the finding is fragile. Strong research combines statistical significance with economic rationale and out-of-sample evidence.

### Correct For Multiple Testing

When many variables, factors, or strategies are tested, some will appear significant by chance alone. Testing 100 random predictors against returns at a 5% significance level yields about 5 "significant" findings purely from luck. Without correcting for the number of tests, false discoveries dominate.

Apply multiple-testing corrections (Bonferroni, Holm, or false discovery rate methods) when many hypotheses are tested. Report how many tests were run, not just the ones that worked. Be especially skeptical of "discovered" factors or signals from large-scale data mining, because the reported significance ignores the unreported failures.

### Use Sufficient Sample Length Relative To Noise

Financial returns have a very low signal-to-noise ratio. Detecting a real but small effect requires a long sample. A regression on a few years of monthly data has low power to distinguish a true relationship from noise; a "significant" result from such a sample may be a fluke that will not replicate.

Assess statistical power: given the expected effect size and return volatility, how long a sample is needed to detect it reliably? If the available sample is shorter than that, even a significant result is weak evidence. Prefer longer histories and higher-frequency data where appropriate, while watching for non-stationarity over long samples.

### Check Regression Assumptions And Diagnose Violations

Standard regression p-values assume normally distributed, homoskedastic, independent residuals. Financial return residuals routinely violate all three: they are fat-tailed, volatility-clustered, and autocorrelated. Violations inflate apparent significance and underestimate uncertainty.

Run diagnostics: residual plots, tests for heteroskedasticity (and use robust standard errors), tests for autocorrelation, and checks for normality. Report robust or Newey-West standard errors that account for serial correlation and heteroskedasticity. A "significant" result under naive standard errors may become insignificant under correct ones.

### Validate Out Of Sample, Not Just In Sample

In-sample fit is almost always optimistic. The real test is whether the relationship holds on data not used to estimate it. Split the sample, estimate on one portion, and test on the held-out portion. Better, use walk-forward or rolling estimation. A relationship that fails out of sample was likely overfit.

Report out-of-sample performance prominently. A model with great in-sample R-squared but poor out-of-sample prediction is not useful; it is a description of the past, not a tool for the future. Treat any finding validated only in sample as a hypothesis, not a conclusion.

### Distinguish Significance From Effect Size And Economic Value

A relationship can be statistically significant yet economically trivial. With enough data, even tiny effects become significant. A factor that "significantly" predicts returns but explains 0.1% of variance and adds 2 basis points of return net of costs is not worth trading. Conversely, a relationship with modest statistical significance but large effect size and strong economic logic may be valuable.

Report effect sizes, confidence intervals, and the economic magnitude (basis points of return, impact net of costs), not just p-values. Judge whether the relationship is large enough to exploit after transaction costs, fees, and taxes.

## Common Traps

### Spurious Regression On Non-Stationary Trending Series

Regressing two price level series that both trend up produces high R-squared and significant t-stats with no real relationship. The trap is reporting this as a discovered link. Always difference to returns or test for cointegration first.

### P-Hacking And Selective Reporting

Running many specifications and reporting only the significant one manufactures false discoveries. The trap is presenting a clean significant result that emerged from dozens of unreported tests. Require disclosure of all tests and multiple-testing correction.

### Confusing Correlation With Causation

A significant correlation does not establish that one variable causes another. Both may be driven by a third factor, or the relationship may be coincidental. The trap is implying causation from a regression. Demand a causal mechanism and consider confounders.

### Ignoring Fat Tails And Volatility Clustering

Financial returns are not normal; they have fat tails and clustered volatility. Standard errors that assume normality and homoskedasticity understate true uncertainty. The trap is over-confident significance claims. Use robust standard errors and acknowledge tail risk.

### Overfitting With Too Many Regressors

Adding many variables to a regression inflates in-sample fit while reducing degrees of freedom and out-of-sample reliability. The trap is a model that fits history beautifully but predicts poorly. Prefer parsimony and penalized regression where appropriate.

### Treating A Short Significant Sample As Proof

A couple of years of significant results in noisy returns is weak evidence. The trap is generalizing from a short favorable window. Demand longer samples, out-of-sample validation, and awareness of the regime during the sample.

### Significance Without Economic Magnitude

A significant but tiny effect is not actionable. The trap is treating statistical significance as practical importance. Always translate to basis points net of costs and judge economic value.

## Self-Check

- [ ] The series were tested for non-stationarity; regressions use returns or changes, or cointegration is established, not raw trending levels.
- [ ] An economic mechanism explains the relationship, not just statistical fit.
- [ ] The number of tests run is disclosed, and multiple-testing correction is applied when many hypotheses were examined.
- [ ] Sample length is assessed against the power needed to detect the effect; short samples are treated as weak evidence.
- [ ] Regression diagnostics were run; robust or autocorrelation-consistent standard errors are used given fat tails and volatility clustering.
- [ ] Out-of-sample or walk-forward validation is reported, not only in-sample fit.
- [ ] Effect size and economic magnitude (basis points net of costs) are reported alongside significance, not just p-values.
- [ ] Causation is not implied from correlation; confounders are considered.
- [ ] The model is parsimonious; overfitting from too many regressors is avoided.
- [ ] The conclusion is probabilistic and notes market relationships are unstable and historical significance does not guarantee future predictive power; it is not personalized advice.
