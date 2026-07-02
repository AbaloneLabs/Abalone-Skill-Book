---
name: max_drawdown_and_recovery.md
description: Use when the agent is measuring or interpreting maximum drawdown and recovery, assessing drawdown distribution and depth, evaluating the asymmetry of losses and the return needed to recover, sequence-of-returns risk in decumulation, or judging whether a strategy's drawdown profile is survivable and whether historical recovery times are a reliable guide.
---

# Max Drawdown And Recovery

Maximum drawdown (MDD) is the largest peak-to-trough decline a portfolio has experienced; recovery is the time and return needed to climb back to the prior peak. These are the metrics that determine whether an investor survives a strategy in practice, because investors abandon strategies in drawdowns, not on average. The judgment problem is that MDD is a single historical number that understates future risk, recovery is not guaranteed, and the mathematics of recovery is brutally asymmetric: a 50% loss requires a 100% gain to recover.

This skill is for measuring and interpreting drawdown and recovery honestly, not as reassurance.

## Core Rules

### Treat MDD As A Lower Bound, Not A Forecast

Historical maximum drawdown is the worst observed in the sample. The future worst can be deeper, because the sample may not include the next regime's crisis.

Use MDD by:

- reporting it as the worst seen, not the worst possible;
- examining drawdowns across multiple regimes and full cycles;
- stress-testing beyond the historical worst with hypothetical shocks;
- acknowledging that a strategy's first real crisis often exceeds its short track record's MDD.

A strategy with a 15% MDD over a five-year calm period can draw down 40% in its first real bear market. The short-sample MDD is a floor on expectation, not a ceiling.

### Understand The Asymmetry Of Loss And Recovery

Losses are asymmetric with the gains needed to recover. This is the most important and most underappreciated drawdown fact.

Know the math:

- a 20% loss needs a 25% gain to recover;
- a 33% loss needs a 50% gain;
- a 50% loss needs a 100% gain;
- a 75% loss needs a 300% gain.

The deeper the drawdown, the more disproportionately difficult the recovery. This is why deep drawdowns are not just painful but wealth-destroying: the return required to recover may take years or may never come, especially for older investors or those withdrawing.

### Measure Recovery In Time And In Probability

Recovery has two dimensions: how long it took historically, and how likely a similar recovery is going forward.

Assess:

- historical recovery times across crises (they vary enormously);
- whether the recovery was driven by factors that may not repeat (falling rates, policy response);
- the probability and time horizon of recovery given current valuations and regime;
- the path of recovery, including whether the investor could have held through it.

A recovery that took three years after 2008 relied on extraordinary policy support and falling rates. The next recovery may be slower if those levers are unavailable. Historical recovery time is a guide, not a guarantee.

### Account For Sequence Risk In Decumulation

Drawdowns are far more dangerous when the investor is withdrawing. Losses early in decumulation, combined with withdrawals, can make recovery impossible.

Model:

- the effect of withdrawals during a drawdown (selling more shares at lower prices);
- the probability of portfolio depletion under bad sequences;
- the "danger zone" near retirement where sequence risk peaks;
- dynamic spending rules that reduce withdrawals in drawdowns.

A 30% drawdown with 4% withdrawals can deplete a portfolio far faster than the same drawdown in accumulation. Sequence risk transforms a survivable drawdown into a permanent one.

### Examine The Drawdown Distribution, Not Just The Maximum

A single MDD number hides the frequency and duration of all drawdowns. The distribution tells the real story.

Report:

- the frequency of drawdowns of various depths (5%, 10%, 20%+);
- the average and distribution of underwater periods;
- the clustering of drawdowns (do they come in waves?);
- the calm versus crisis drawdown behavior.

A strategy with one deep drawdown and otherwise shallow ones differs greatly from one with frequent moderate drawdowns. The distribution reveals the lived experience of holding the strategy.

### Separate Temporary Drawdown From Permanent Loss

A drawdown is a decline from which the asset can recover. A permanent loss (bankruptcy, fraud, dilution, obsolescence) never recovers. Confusing them is dangerous.

Distinguish:

- cyclical drawdowns in sound assets that recover with the cycle;
- structural or permanent impairments where recovery is unlikely;
- the role of leverage, which can turn a temporary drawdown into a permanent loss via margin call or forced sale.

A diversified equity index drawdown is usually temporary; a single stock that goes bankrupt is permanent. Leverage and concentration convert the former into the latter.

### Connect Drawdown To Investor Behavior

The relevant drawdown is not the historical maximum; it is the deepest drawdown the investor will actually hold through. Behavior is the binding constraint.

Estimate:

- the dollar and percentage drawdown at which the investor is likely to sell;
- whether the strategy's drawdown profile fits within that tolerance;
- the behavioral cost of abandoning at the bottom (locking in the loss, missing the recovery);
- whether drawdown control (de-risking, hedging) is warranted to keep the investor invested.

A strategy whose MDD exceeds the investor's behavioral limit is unsuitable regardless of its long-run return, because the investor will not capture that return.

## Common Traps

### Treating Historical MDD As The Worst Possible

The next crisis can exceed the sample's worst. Short or calm-sample MDDs systematically understate risk.

### Ignoring Recovery Asymmetry

A 50% loss needs a 100% gain. Deep drawdowns are disproportionately hard to recover from, yet agents often discuss them as if recovery were symmetric.

### Assuming Recovery Is Guaranteed

Some drawdowns become permanent (single-name ruin, structural impairment). Even for sound assets, recovery depends on factors that may not recur.

### Ignoring Sequence Risk In Decumulation

The same drawdown is far more dangerous when withdrawing. Sequence risk can make a survivable drawdown permanent.

### Single-Number MDD Without Distribution

One MDD hides the frequency, duration, and clustering of drawdowns that define the lived experience.

### Confusing Temporary Decline With Permanent Loss

A cyclical drawdown in a sound asset recovers; a permanent impairment does not. Leverage and concentration convert one into the other.

### Optimizing Average Return, Ignoring Drawdown Path

A strategy with a good average return but a drawdown path the investor cannot hold delivers none of its average benefit. Path dominates average for real outcomes.

### Trusting Recovery Times From A Favorable Regime

Recoveries after 2008 and 2020 were unusually fast due to policy support. Future recoveries may be slower; historical speed is not a law.

## Self-Check

- [ ] MDD is reported as the worst observed in the sample, with stress tests beyond it, not as the worst possible.
- [ ] The asymmetry of loss and required recovery gain is made explicit for the relevant drawdown depths.
- [ ] Recovery is assessed in both time and probability, with the drivers of past recoveries (rates, policy) examined for repeatability.
- [ ] Sequence-of-returns risk in decumulation is modeled, including depletion probability and dynamic spending.
- [ ] The drawdown distribution (frequency, depth, duration, underwater periods, clustering) is reported, not only the single maximum.
- [ ] Temporary drawdown is distinguished from permanent loss, with leverage and concentration flagged as converters.
- [ ] The drawdown profile is connected to the investor's behavioral limit, and drawdown control is considered where the profile exceeds tolerance.
- [ ] The recommendation flags that drawdowns can exceed history, that recovery is not guaranteed and can be slow or impossible, and that professional advice may be warranted for decumulation and concentrated portfolios.
