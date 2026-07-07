---
name: time_weighted_vs_money_weighted.md
description: Use when the agent is choosing between time-weighted and money-weighted returns, calculating TWR and IRR, interpreting the return gap and timing drag, deciding which method measures manager skill versus investor experience, handling external cash flows and sub-period linking, or reviewing how the choice of return method changes the reported result and what each method does and does not reveal.
---

# Time-Weighted Versus Money-Weighted Returns

The choice between time-weighted return (TWR) and money-weighted return (MWR, often the internal rate of return or IRR) is one of the most consequential and most misunderstood decisions in performance measurement. The two methods can produce dramatically different numbers for the same portfolio, and they answer fundamentally different questions. Time-weighted return measures the performance of the investments themselves, neutralizing the effect of when money was added or withdrawn; it is the right measure of manager skill. Money-weighted return measures the actual experience of the investor, including the effect of their cash-flow timing; it is the right measure of the investor's outcome. The judgment problem is that agents routinely use the wrong method, report TWR as if it were the investor's return, or fail to recognize that the gap between the two reveals the large, often negative, effect of cash-flow timing. The skill is choosing and interpreting each method correctly and understanding what the gap between them signifies.

This skill is for selecting and interpreting return methods with awareness of what each reveals and conceals.

## Core Rules

### Match The Method To The Question Being Asked

The foundational rule is that the method must fit the question. The two methods answer different questions, and using the wrong one produces a misleading number.

Time-weighted return (TWR):

- answers "how did the investments perform, independent of cash-flow timing?";
- neutralizes the effect of external contributions and withdrawals;
- is the correct measure of manager or strategy skill, where the client controls cash flows;
- is required by GIPS for composites for this reason.

Money-weighted return (MWR / IRR):

- answers "what return did the investor actually experience, given their cash flows?";
- reflects the timing and size of every contribution and withdrawal;
- is the correct measure of the investor's realized outcome;
- is sensitive to whether money was added at highs or lows.

If the question is about the manager, use TWR. If the question is about the investor, use MWR. Reporting one as if it were the other is the most common and most damaging error.

### Understand Why The Two Methods Diverge

The divergence between TWR and MWR is not an error; it is information. The gap is driven entirely by cash-flow timing relative to performance.

The dynamics:

- if an investor adds money just before a rise, MWR exceeds TWR (good timing amplified the gain);
- if an investor adds money just before a decline, MWR is below TWR (bad timing amplified the loss);
- if there are no external cash flows, TWR and MWR are identical;
- the larger and more poorly timed the cash flows, the wider the gap.

Empirically, investor MWR is usually below TWR, because investors tend to add money after rallies (near highs) and withdraw after declines (near lows). This systematic timing drag is real money lost, and it is invisible if only TWR is reported.

### Calculate Each Method Correctly

Each method has a specific calculation that must be applied properly, especially around cash flows.

Time-weighted return:

- value the portfolio at every external cash flow, breaking the period into sub-periods;
- compute the return for each sub-period (neutralizing the cash flow within it);
- geometrically link the sub-period returns to get the period TWR;
- frequent valuation (daily) improves accuracy by reducing sub-period distortion.

Money-weighted return (IRR):

- solve for the discount rate that equates the present value of all cash flows (contributions, withdrawals, ending value) to zero;
- weight each period's return by the capital deployed in it;
- a single number that reflects both investment performance and cash-flow timing.

Errors in cash-flow handling (wrong dates, missed flows, wrong sub-period breaks) corrupt both methods. Precision in recording the timing and size of every external cash flow is essential.

### Interpret The Return Gap As Timing Skill Or Drag

The gap between TWR and MWR measures the effect of cash-flow timing. It is often called the return gap or timing drag, and it is one of the most informative numbers in performance analysis.

Use the gap to:

- quantify how much investor behavior (adding and withdrawing) helped or hurt versus a buy-and-hold;
- identify systematic timing errors (consistent negative gaps suggest poor cash-flow discipline);
- illustrate to investors the cost of their own behavior, which is often larger than fees;
- compare across investors or periods to find where timing helped or hurt most.

A persistent, large negative gap is a behavioral cost that no manager skill can overcome. Revealing it to the investor is often more valuable than reporting the manager's TWR, because the investor controls the cash flows.

### Handle The Practical Complications

Real-world return calculation involves complications that, if mishandled, distort both methods.

Address:

- large external cash flows: break the sub-period at the flow to keep TWR accurate;
- intra-day cash flows: define a consistent convention (beginning or end of day);
- income reinvestment: treat reinvested dividends as internal (no flow) or external consistently;
- fees and taxes: decide whether to compute gross or net, and apply consistently to both methods;
- partial periods: annualize carefully, recognizing the limits for short records.

Inconsistency in any of these produces numbers that cannot be compared across periods, accounts, or managers. Document the conventions and apply them uniformly.

### Use Both Methods Together For A Complete Picture

Neither method alone tells the whole story. The strongest analysis reports both and explains the gap.

Report together:

- TWR to evaluate the manager or strategy fairly, isolating investment skill from cash-flow luck;
- MWR to show the investor's actual outcome, including their cash-flow timing;
- the gap to quantify the effect of timing, which is often the largest single driver of investor outcomes;
- the context (market conditions, cash-flow pattern) that explains the gap.

A report showing only TWR flatters the manager and hides the investor's real experience. A report showing only MWR conflates manager skill with timing luck. Both, with the gap explained, give an honest and complete picture.

### Recognize What Neither Method Captures

Even both methods together do not capture everything relevant to performance evaluation.

Remember:

- neither method predicts the future; past returns, however calculated, do not guarantee future results;
- neither captures risk-adjusted return (volatility, drawdown, beta) without supplementary metrics;
- TWR can mask high volatility behind a smooth average; MWR can be dominated by a single large poorly timed flow;
- taxes, for taxable investors, affect the realized outcome and are often excluded from both methods.

Supplement TWR and MWR with risk measures, after-tax returns, and qualitative context. The return method is one part of a complete performance picture, not the whole of it.

## Common Traps

### Using TWR As The Investor's Return

TWR neutralizes cash flows and measures manager skill. The investor's actual return is money-weighted and often lower due to timing.

### Using MWR To Evaluate Manager Skill

MWR conflates investment performance with cash-flow timing. A manager can look bad because the client timed flows poorly, or good because the client timed them well.

### Ignoring The Return Gap

The gap between TWR and MWR quantifies the cost or benefit of cash-flow timing, often the largest driver of investor outcomes. Hiding it hides the real story.

### Mishandling Cash Flows In Calculation

Wrong dates, missed flows, or inconsistent sub-period breaks corrupt both TWR and MWR. Precision in cash-flow recording is essential.

### Annualizing Short Or Distorted Periods

A short period or one dominated by a single large flow produces an annualized figure that is not meaningful. Flag the limitations.

### Comparing Methods Across Different Cash-Flow Patterns

TWR and MWR are only directly comparable for the same portfolio and cash flows. Comparing a manager's TWR to an investor's MWR is apples to oranges.

### Forgetting Taxes And Risk

Neither method captures risk-adjusted return or after-tax outcome without supplementary metrics. Returns alone are incomplete.

### Treating Either Number As Predictive

Both TWR and MWR describe the past. Neither predicts future performance, regardless of how favorable the history looks.

## Self-Check

- [ ] The return method (TWR or MWR) is matched to the question: TWR for manager or strategy skill, MWR for the investor's actual experience.
- [ ] The reason the two methods diverge (cash-flow timing relative to performance) is understood, and the gap is recognized as information, not error.
- [ ] Each method is calculated correctly, with proper sub-period breaks for TWR and correct IRR solving for MWR, and cash flows are recorded precisely.
- [ ] The return gap between TWR and MWR is computed and interpreted as a measure of timing skill or drag, often revealing behavioral cost larger than fees.
- [ ] Practical complications (large flows, intra-day conventions, reinvested income, fees, taxes, partial periods) are handled consistently and documented.
- [ ] Both methods are reported together with the gap explained, to give a complete picture of manager skill and investor outcome.
- [ ] The analysis acknowledges what neither method captures (future prediction, risk-adjusted return, after-tax outcome) and supplements with risk measures and context.
- [ ] The recommendation states that return methods describe the past and do not predict future results, that cash-flow timing materially affects investor returns, that calculation errors can distort both methods, and that this is not investment advice and professional performance-measurement expertise may be warranted for formal reporting.