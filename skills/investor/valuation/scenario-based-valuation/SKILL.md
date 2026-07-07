---
name: scenario_based_valuation.md
description: Use when the agent is valuing a company or asset across multiple distinct future scenarios (bull, base, bear, disruption, regulatory change), assigning probabilities to scenarios, blending scenario values into an expected value, deciding how wide the scenario range should be, or reviewing how scenario valuation handles deep uncertainty, non-linear outcomes, and the risk that probability weights create false precision.
---

# Scenario-Based Valuation

Scenario-based valuation appraises an asset by constructing several distinct, internally consistent pictures of the future, valuing the asset under each, and combining the results (often as a probability-weighted expected value) rather than relying on a single point estimate. The strength is that it forces explicit treatment of uncertainty and non-linear outcomes: a company's value in a disruption scenario may be a fraction of its base case, and ignoring that possibility is a form of analysis malpractice. The judgment problem is that scenario valuation is easy to do badly. Agents construct scenarios that are too similar to be informative, assign probability weights that imply false precision, or blend disparate outcomes into a tidy expected value that hides the wide dispersion and the path-dependence that actually determine investment outcomes. The skill is building scenarios that genuinely span the plausible range, weighting them honestly, and presenting the dispersion rather than collapsing it into a misleading average.

This skill is for valuing under uncertainty through scenarios with awareness of where the method adds insight and where it manufactures false confidence.

## Core Rules

### Build Scenarios That Genuinely Span The Plausible Range

The value of scenario analysis comes from the spread between scenarios. If all scenarios converge to similar values, the exercise adds nothing. The scenarios must be distinct enough to matter.

Construct scenarios that vary:

- the macro environment (growth, inflation, rates, regime);
- industry structure (competitive intensity, disruption, consolidation);
- company-specific outcomes (execution, product success, margin trajectory);
- regulatory, legal, and technological shifts;
- the tail outcomes, including severe downside and surprising upside.

Each scenario should be a coherent, internally consistent story, not a random variation of one input. A bear case that is just "10% lower growth" is not a scenario; it is a sensitivity. Real scenarios change the story.

### Make Each Scenario Internally Consistent

A scenario is a self-contained world. Its assumptions must fit together; mixing a recession macro with record margins and aggressive expansion produces an incoherent picture.

For each scenario, ensure:

- the macro backdrop is consistent with the industry and company outcomes;
- the competitive response is realistic (rivals react to a company's success or distress);
- the financial drivers (revenue growth, margin, capex, working capital) tell one story;
- the balance sheet and cash flow consequences follow from the operating assumptions.

Inconsistency between the narrative and the numbers is where scenario valuations silently break. Read each scenario as a whole and check that the financials embody the story.

### Assign Probabilities Honestly, Not Precisely

Probability weights let you compute an expected value, but they are the most over-precise part of the method. Few analysts can distinguish a 22% probability from a 28% probability, and the implied precision is illusory.

Approach weights by:

- acknowledging they are subjective judgments, not measurements;
- using broad bands (say 20-30%) rather than precise point weights where possible;
- testing the expected value's sensitivity to the weights;
- being explicit that the weights reflect your beliefs and could be wrong.

The expected value is a useful summary, but the dispersion across scenarios is often more informative than the weighted average. Do not let a tidy expected value obscure a wide and consequential range.

### Value Each Scenario With Appropriate Methods And Inputs

Each scenario may warrant different valuation inputs, because risk, growth, and capital costs differ across futures.

Adjust by scenario:

- discount rates that reflect the risk of that world (a disruption scenario may carry a higher discount rate);
- terminal value assumptions that fit the scenario's long-run structure;
- multiples drawn from peers in analogous conditions, not a single sector average;
- capital structure and balance sheet outcomes, including distress in downside cases.

Applying one discount rate and one terminal multiple to all scenarios undoes much of the benefit. The valuation inputs should embody the scenario's economics.

### Present The Full Distribution, Not Just The Expected Value

The expected value hides the risk. Two investments with identical expected values can have radically different downside, and the downside is what ruins investors. Show the shape of the outcome distribution.

Present:

- the value under each named scenario, clearly labeled;
- the probability-weighted expected value as a summary, with its limitations stated;
- the downside scenario value and its probability, since this drives most risk decisions;
- the upside and its probability, since optionality matters;
- the breakeven scenario (what must be true for the current price to be fair).

An investor who sees only the expected value cannot judge whether the upside compensates for a real chance of severe loss. The distribution is the analysis.

### Include Disruption And Regime-Change Scenarios

The most valuable scenarios are often the ones that feel unlikely: disruption, regulatory upheaval, technological obsolescence, or a shift in the industry's economics. These low-probability, high-impact outcomes drive most of the risk and most of the opportunity.

Ensure scenarios include:

- a disruptive competitor or technology that changes the economics;
- regulatory or legal action that constrains or enables the business;
- a macro regime shift (persistent inflation, deflation, rate shock);
- management, governance, or balance-sheet stress events.

Excluding these because they are "low probability" ignores the outcomes that matter most. The probability may be small, but the value impact is large, and the product of the two is what the expected value must capture.

### Connect Scenarios To Decision Rules And Catalysts

Scenario valuation is most useful when it drives a decision. Connect the scenario set to what an investor should do and what would change the view.

Link to:

- the margin of safety implied by the downside scenario versus the current price;
- the catalysts that would move the probability toward one scenario (earnings, regulatory decisions, product launches);
- the monitoring signals that would trigger updating the weights;
- the position sizing appropriate to the dispersion (wide dispersion warrants smaller size).

A scenario analysis that produces a number but no decision rule is incomplete. The output should inform what to own, how much, and what to watch.

## Common Traps

### Scenarios That Are Too Similar To Matter

If bull, base, and bear cases converge to near-identical values, the analysis adds nothing. Scenarios must span a meaningful range.

### Inconsistent Scenario Assumptions

Mixing a recession narrative with boom-era margins produces incoherent scenarios. Each scenario must be internally consistent.

### False Precision In Probability Weights

Distinguishing 22% from 28% is beyond honest judgment. Weights are beliefs, not measurements; use bands and test sensitivity.

### Collapsing To An Expected Value That Hides Risk

The weighted average hides the downside that drives most risk. Present the full distribution, especially the severe scenarios.

### Omitting Low-Probability High-Impact Scenarios

Excluding disruption or regime change because it is "unlikely" ignores the outcomes that matter most. Their value impact is large even at low probability.

### Using Identical Valuation Inputs Across Scenarios

One discount rate and terminal multiple for all scenarios undoes the benefit. Inputs should embody each scenario's risk and economics.

### Anchoring Scenarios To The Current Price

Constructing scenarios that conveniently justify the market price is confirmation, not analysis. Build scenarios from fundamentals first, then compare to price.

### Treating Scenario Outputs As Predictions

Scenarios are tools for thinking about uncertainty, not forecasts. The future will rarely match any single scenario exactly.

## Self-Check

- [ ] Scenarios are distinct and span a meaningful range of macro, industry, company, regulatory, and tail outcomes, not minor variations of one base case.
- [ ] Each scenario is internally consistent, with macro, competitive, and financial assumptions that tell one coherent story.
- [ ] Probability weights are treated as subjective beliefs in broad bands, with sensitivity of the expected value to the weights tested and disclosed.
- [ ] Each scenario is valued with inputs (discount rate, terminal assumptions, multiples, capital structure) appropriate to that scenario's risk and economics.
- [ ] The full distribution of scenario values is presented, including downside, upside, expected value, and breakeven, not only the weighted average.
- [ ] Low-probability high-impact scenarios (disruption, regime change, distress) are explicitly included despite their unlikelihood.
- [ ] Scenarios are connected to decision rules (margin of safety, position sizing), catalysts that would shift weights, and monitoring signals.
- [ ] The recommendation states that scenario valuation depends on subjective assumptions, that probability weights are estimates not measurements, that no scenario will unfold exactly as described, and that this is not investment advice and professional valuation expertise may be warranted for complex or high-uncertainty situations.