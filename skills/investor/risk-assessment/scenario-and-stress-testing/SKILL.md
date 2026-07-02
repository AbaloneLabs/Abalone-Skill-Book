---
name: scenario_and_stress_testing.md
description: Use when the agent is stress-testing a portfolio using historical scenarios, hypothetical shocks, factor exposures, Monte Carlo simulation, or tail events, estimating drawdowns and recovery under crisis conditions, or judging portfolio resilience, hidden concentrations, and the adequacy of risk controls before a real stress event.
---

# Scenario And Stress Testing

Stress testing asks "what could break this portfolio" before it breaks. It is the discipline of imagining adverse conditions, historical and hypothetical, and measuring the damage. The judgment problem is that the most dangerous scenarios are often the ones not in the historical sample, and most stress tests are built from the crises already survived, which by definition the system weathered. Agents run a few familiar historical replays, declare the portfolio "resilient," and miss the next crisis whose shape is different.

This skill is for designing stress tests that genuinely probe vulnerability rather than confirm safety.

## Core Rules

### Use Multiple Categories Of Stress

No single stress type is sufficient. Combine several families.

Use:

- historical replays (2008, 2020, the 1970s inflation, the 1994 bond crash, regional crises);
- hypothetical factor shocks (equity down, rates up, credit wide, dollar strong, volatility spike);
- reverse stress tests (what scenario would cause an unacceptable loss, then ask if it is plausible);
- Monte Carlo simulation (thousands of random paths, including fat-tailed ones);
- correlation-breakdown scenarios where diversifiers fail together.

Each family catches different blind spots. Historical replays test known crises; hypothetical shocks test factor sensitivities; reverse tests find the breaking point; Monte Carlo maps the distribution.

### Stress The Factors, Not Just The Labels

A portfolio's vulnerability lives in its factor exposures, not its asset labels. Two portfolios labeled "diversified" can have very different factor stress results.

Decompose and shock:

- equity beta;
- duration and rate sensitivity;
- credit spread sensitivity;
- currency exposures;
- liquidity and volatility exposures;
- factor tilts (value, momentum, quality, size).

A shock to rates may hurt a "diversified" portfolio loaded with bonds far more than an equity-only shock. Factor-level stress reveals what label-level stress hides.

### Model Correlation Breakdown

The defining feature of crises is that correlations change. Diversifiers that held in calm fail together in stress.

Test:

- correlation-to-one scenarios where diversification partially or fully collapses;
- joint shocks where equities, credit, and diversifiers fall together;
- funding and liquidity stress where previously uncorrelated assets sell off together;
- regime-specific correlations (growth shock versus inflation shock).

A portfolio that survives an isolated equity shock but fails a correlation-breakdown shock is not crisis-resilient.

### Include Path And Liquidity Stress

A single-point loss number misses how the portfolio behaves over time and whether it can be funded or sold.

Model:

- the path of drawdown and recovery, not just the trough;
- margin and collateral calls on leveraged or derivatives positions;
- liquidity gaps where illiquid sleeves cannot be sold or are marked down sharply;
- funding-cost spikes that force deleveraging at the worst time;
- withdrawal or redemption pressure in decumulation.

Path and liquidity stress separate a portfolio that "recovers on paper" from one that survives in practice.

### Run Reverse Stress Tests

Standard stress asks "what happens if X." Reverse stress asks "what would it take to cause unacceptable loss Y," then judges whether that scenario is plausible.

Apply:

- define the unacceptable loss (goal failure, ruin, plan abandonment);
- find the factor moves or scenario that produces it;
- assess the plausibility of that scenario given current conditions;
- identify the specific exposures that drive the break.

Reverse stress finds vulnerabilities that forward tests miss because no one thought to shock that combination.

### Acknowledge Model Limits And Tail Uncertainty

Every stress test rests on assumptions about distributions, correlations, and relationships that may fail in the very crisis being modeled. Treat outputs as rough, not precise.

Be explicit about:

- the distributional assumption and its breakdown in extreme tails;
- the correlation assumption and its instability;
- the liquidity and pricing assumptions;
- what the model cannot capture (regime change, structural breaks, contagion channels).

A stress test is a disciplined imagination exercise, not a guarantee. The goal is to surface and reduce vulnerability, not to certify safety.

### Connect Results To Action

A stress test that does not change behavior is theater. Tie findings to decisions.

For each material vulnerability, decide:

- reduce the exposure that drives the break;
- add diversification or hedging that helps in that scenario;
- increase reserves or liquidity buffers;
- change the plan (de-risk, extend horizon, adjust goals);
- accept the risk consciously with eyes open.

Document the residual risks the investor chooses to bear, so that when the scenario arrives, the response is planned, not panicked.

## Common Traps

### Only Replaying Survived Historical Crises

Historical replays test crises the system already weathered. The next crisis often has a different shape.

### Stressing Labels Instead Of Factors

Asset-label stress misses factor vulnerabilities. A rate shock to a bond-heavy "diversified" portfolio is invisible at the label level.

### Assuming Stable Correlations

Correlations break in crises. Stress tests that hold correlations fixed overstate diversification exactly when it matters.

### Single-Point Loss Without Path

A trough-to-recovery average hides margin calls, liquidity gaps, and forced sales that happen along the path.

### Over-Trusting The Model

Stress outputs are model-dependent and fragile in the tails. Treating them as precise invites false confidence.

### No Reverse Stress

Without asking what would break the portfolio, agents miss the scenario no one thought to shock.

### Stress Tests That Change Nothing

A test that surfaces a vulnerability but leads to no action is wasted. Findings must connect to exposure, hedging, or plan changes.

## Self-Check

- [ ] Multiple stress families are used (historical, hypothetical factor shocks, reverse stress, Monte Carlo, correlation breakdown).
- [ ] Stress is applied at the factor level (equity, duration, credit, currency, liquidity, volatility), not only at asset labels.
- [ ] Correlation-breakdown and joint-shock scenarios are modeled, not just isolated single-factor shocks.
- [ ] Path of drawdown, margin and collateral calls, liquidity gaps, funding stress, and redemption pressure are included.
- [ ] Reverse stress tests identify the scenario that would cause unacceptable loss and assess its plausibility.
- [ ] Model limits, distributional assumptions, and tail uncertainty are acknowledged, and outputs are treated as rough.
- [ ] Each material vulnerability is connected to a concrete action (reduce exposure, hedge, buffer, change plan) or a conscious acceptance of residual risk.
- [ ] The recommendation frames stress testing as vulnerability discovery, not safety certification, and notes that extreme outcomes can exceed any model and that professional advice may be warranted for complex or leveraged portfolios.
