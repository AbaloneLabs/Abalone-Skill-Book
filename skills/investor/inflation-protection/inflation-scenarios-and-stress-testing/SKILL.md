---
name: inflation_scenarios_and_stress_testing.md
description: Use when the agent is stress testing a portfolio against inflation scenarios, distinguishing expected from unexpected inflation, modeling regime change, or evaluating how assets behave across deflation, mild inflation, and hyperinflation. Covers inflation stress testing, expected versus unexpected inflation, regime change, stagflation, hyperinflation tail risk, correlation breakdowns during inflation, and building scenarios that reveal hidden portfolio vulnerabilities.
---

# Inflation Scenarios And Stress Testing

Inflation does not arrive as a single number; it arrives as a regime with a cause, a persistence, and a set of cross-asset effects that break the correlations a portfolio relied on. The judgment problem is that investors stress-test for "inflation goes up 3 percent" as a uniform shock, when the damage depends on whether inflation is expected or surprise, demand-driven or supply-driven, transient or persistent, and whether it arrives with growth or recession. A portfolio that survives mild expected inflation can be destroyed by stagflation, and the assets that hedge a demand-pull inflation can fail in a supply-shock. Agents and investors run one inflation scenario, conclude the portfolio is fine, and miss that a different inflation regime — often the more dangerous one — would break it. The skill is constructing distinct, internally coherent inflation scenarios and testing the portfolio against the ones that actually threaten it.

This skill is for stress-testing portfolios against inflation scenarios. It is not personalized financial advice; scenarios are illustrative, and past performance does not guarantee future results.

## Core Rules

### Build Scenarios Around Causes And Regimes, Not A Single Inflation Number

A uniform "inflation rises" shock tells you almost nothing, because assets respond differently depending on why inflation is rising and what is happening to growth.

Construct distinct scenarios:

- **demand-pull inflation** — strong growth, rising wages, capacity constraints; equities and real estate often hold up because growth supports cash flows;
- **cost-push or supply-shock inflation** — rising input costs, disrupted supply; commodities may rise while equities suffer margin compression;
- **stagflation** — inflation with recession or stagnation; the cruelest regime, where both growth assets and nominal bonds suffer;
- **persistent regime change** — a shift from a low-inflation to a high-inflation era that reprices all long-duration assets;
- **hyperinflation tail** — a loss of confidence in currency, extreme but low-probability, requiring different protections.

Each scenario has internally consistent growth, inflation, rate, and cross-asset implications. Test the portfolio against several, not one.

### Separate Expected From Unexpected Inflation In Every Scenario

Expected inflation is already priced into asset yields and valuations. The portfolio risk comes from inflation that deviates from expectations — the surprise.

Decompose each scenario:

- what inflation level is currently priced into breakevens, nominal yields, and market expectations;
- does the scenario assume inflation matching, exceeding, or falling short of expectations;
- which assets are sensitive to the surprise component rather than the level.

Assets that hedge expected inflation (like nominal growth assets over long horizons) differ from those that hedge unexpected inflation (like TIPS and commodities). Map each holding to the surprise it addresses.

### Model Correlation Breakdowns During Inflation Regimes

The diversification a portfolio relies on in normal times often fails precisely during inflation stress, when correlations converge. Historical average correlations understate the risk.

Stress the correlations:

- in stagflation, equities and nominal bonds can fall together, destroying the classic 60/40 diversification;
- in a regime shift to high inflation, long-duration bonds and long-duration growth equities both suffer as discount rates rise;
- commodities may decorrelate positively, but only certain commodity exposures (spot-linked, not futures-roll-eroded) provide it;
- real assets' reported low correlations can be artifacts of stale pricing that break down when marks catch up.

Use regime-conditional correlations, not full-sample averages, when stress-testing. The bad scenarios are where diversification matters most and fails most often.

### Identify The Long-Duration Vulnerabilities Explicitly

Inflation and rising rates punish long-duration assets — whose cash flows are far in the future — by raising the discount rate and compressing valuations. Many portfolios carry hidden duration they do not recognize.

Find the duration:

- long-duration nominal bonds and bond funds carry the most direct interest-rate risk;
- long-duration growth equities (unprofitable, high-multiple, future-cash-flow-dependent) carry equity duration that reprices sharply when rates rise;
- real estate and infrastructure held through long-lease or regulated structures may have implicit duration;
- pension and insurance liabilities behave as long-duration short positions that inflation can inflate.

Map the portfolio's duration exposure and test how each long-duration position behaves when the discount rate rises with inflation.

### Test The Spending And Liability Side, Not Just The Asset Side

For investors with spending needs or liabilities, inflation stress must run through the obligation side, not only the asset side. An asset portfolio that survives inflation nominally can still fail if the real spending need grows faster.

Stress the full balance:

- for retirees, model real spending rising with inflation while assets may lag;
- for pension and insurance, model the liability growing with inflation and rates while assets reprice;
- for goals-based investors, test whether the funded ratio or goal-achievement probability survives an inflation regime;
- for borrowers, model how inflation interacts with debt (fixed-rate debt can benefit from inflation; floating-rate debt suffers).

The relevant measure is the surplus or goal achievement after inflation, not the nominal asset return.

### Include Path Dependence And Sequence Risk

Inflation regimes unfold over years, and the path matters as much as the endpoint. A portfolio that is solvent at the end of an inflation episode can fail along the way if forced to sell assets during the worst of it.

Model the path:

- sequence risk for decumulating investors — an early inflation spike with falling asset values can permanently impair a portfolio even if later years recover;
- liquidity needs during the stress — forced selling during the worst correlation breakdown locks in losses;
- the time lag between inflation arriving and hedges (commodities, real asset repricing, TIPS adjustment) taking effect;
- the behavioral risk that investors abandon hedges that have underperformed just before they are needed.

Path-aware stress testing reveals vulnerabilities that endpoint-only analysis hides.

### Define What Survival Means Before Running The Scenario

A stress test is only useful if it has a clear pass/fail definition tied to the investor's objectives. Define survival in advance.

Define:

- the maximum acceptable drawdown in real terms;
- the minimum funded ratio or goal-achievement probability;
- the liquidity headroom needed to avoid forced selling through the scenario;
- the conditions under which the portfolio would need to be restructured mid-scenario.

Without a definition of survival, a stress test becomes a narrative exercise rather than a decision tool.

## Common Traps

### Running One Uniform Inflation Shock

Assets respond differently to different inflation regimes. A single shock hides the scenario that would break the portfolio.

### Ignoring Expected Versus Unexpected Inflation

Expected inflation is already priced. The risk is the surprise, and assets hedge the two differently.

### Using Full-Sample Average Correlations

Correlations converge in inflation stress. Regime-conditional correlations reveal the diversification breakdown that averages hide.

### Missing Hidden Long-Duration Exposure

Long-duration bonds, growth equities, and certain real assets all carry duration that reprices sharply when inflation and rates rise.

### Stressing Only The Asset Side

For investors with spending needs or liabilities, inflation must run through the obligation side. Nominal asset survival can mask real failure.

### Endpoint-Only Analysis Ignoring Path And Sequence

A portfolio solvent at the endpoint can fail along the path if forced selling or sequence risk bites during the worst of the regime.

### No Definition Of Survival

Without a clear pass/fail tied to objectives, a stress test is a narrative, not a decision tool.

## Self-Check

- [ ] Multiple distinct inflation scenarios (demand-pull, cost-push, stagflation, regime change, hyperinflation tail) were constructed with internally consistent growth, rate, and cross-asset implications.
- [ ] Each scenario decomposed expected versus unexpected inflation, and assets were mapped to the surprise they hedge.
- [ ] Regime-conditional correlations were used, exposing diversification breakdowns (equities and bonds falling together) that full-sample averages hide.
- [ ] Long-duration vulnerabilities (nominal bonds, growth equities, long-lease real assets, liabilities) were identified and tested against rising discount rates.
- [ ] The stress ran through the spending and liability side, not only the asset side, measuring real surplus or goal achievement.
- [ ] Path dependence and sequence risk were modeled, including forced-selling and liquidity needs during the worst of the regime.
- [ ] A clear definition of survival (maximum real drawdown, minimum funded ratio, liquidity headroom, restructuring triggers) was set before running the scenarios.
- [ ] The conclusion flags that scenarios are illustrative not predictive, that regime outcomes are uncertain, that past performance does not guarantee future results, and that this is not personalized financial advice.
