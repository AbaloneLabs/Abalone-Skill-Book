---
name: spousal_and_survivor_benefits.md
description: Use when the agent is coordinating Social Security claiming for a married couple, evaluating spousal benefits, restricted application strategy, survivor benefits after a spouse's death, divorced or widowed benefit eligibility, or optimizing the combined lifetime household benefit across two earners with different earning histories.
---

# Spousal And Survivor Benefits

For a married couple, Social Security is not two independent decisions. Each spouse may be entitled to a benefit based on their own earnings record, a spousal benefit based on their partner's record, and eventually a survivor benefit based on the deceased spouse's record. The rules that govern how these interact, including which benefit is paid, when restricted application applies, and how the survivor benefit is determined, create claiming strategies whose combined household value can differ by tens of thousands of dollars. The judgment problem is optimizing the couple as a unit rather than each spouse individually.

The harm this skill prevents is each spouse claiming in isolation and missing the coordinated strategy that maximizes household lifetime benefits, particularly the survivor benefit, which is the largest and most commonly squandered source of value. Social Security rules have changed over time, restricted application eligibility depends on birth year, and divorced and widowed claimants face different rules. No output here is personalized advice. The agent should frame the coordination logic and direct the user to their earnings records and a qualified advisor for the binding decision.

## Core Rules

### Optimize The Household, Not Each Spouse

The claiming decision that is best for one spouse in isolation may be worst for the household. The lower earner might claim early to provide income while the higher earner delays, or both might coordinate to maximize the survivor benefit. The agent should model the combined household benefit across both lifetimes, including the period after the first death, rather than optimizing each spouse separately. The household lens is the single most important shift in couples claiming analysis.

### Understand How Spousal Benefits Are Calculated

A spousal benefit is based on the higher-earning spouse's primary insurance amount and is paid to the lower-earning spouse, up to roughly half of the higher earner's full benefit at the lower earner's full retirement age. The lower earner receives their own benefit first, and a spousal top-up brings the total to the spousal amount if that is higher. Claiming the spousal benefit before full retirement age reduces it permanently. The agent should distinguish the spousal benefit from the worker's own benefit and should note that the spousal maximum is tied to the higher earner's full benefit, not their delayed benefit.

### Maximize The Survivor Benefit By Delaying The Higher Earner

The survivor benefit is the largest and most valuable interaction. When one spouse dies, the survivor receives the larger of their own benefit or the deceased spouse's benefit, not both. Therefore, the larger of the two benefits, typically the higher earner's, determines the survivor's income for the rest of their life. Delaying the higher earner's claim to grow their benefit directly increases the survivor's lifetime income. This is often the highest-value claiming move available to a couple, and the agent should prioritize it. The lower earner's claim age has no effect on the survivor benefit, so the lower earner can claim earlier to provide household income.

### Apply Restricted Application Rules Correctly By Birth Year

Restricted application, the ability to claim only a spousal benefit while letting one's own benefit grow, was a powerful strategy that let one spouse collect spousal income while delaying their own. The law changed so that this option is available only to those who attained a certain age before the change; younger claimants are deemed to file for all benefits they are eligible for and receive the higher combined amount, with no ability to restrict. The agent must check the claimant's birth year before invoking restricted application, because recommending it to an ineligible claimant is a factual error that invalidates the strategy.

### Handle Divorced And Widowed Claimants Differently

Divorced claimants may claim spousal benefits based on a living ex-spouse's record if the marriage lasted long enough and other conditions are met, without the ex-spouse having filed, in some cases. Widowed claimants may claim survivor benefits, sometimes as early as a young age, and may switch to their own benefit later if it grows larger. The agent should not assume married-couple rules apply to divorced or widowed claimants, should check the specific eligibility conditions, and should consider the sequencing of switching between survivor and own benefits.

### Sequence Claims To Bridge The Income Gap

A common coordinated strategy is for the lower earner to claim at or before full retirement age to provide household income, while the higher earner delays to maximize both their own benefit and the eventual survivor benefit. The agent should model the household cash flow during the delay period and ensure the early claim bridges the gap without forcing asset sales. The sequence matters because the lower earner's claim funds the household while the higher earner's delay builds the floor.

### Re-Check Strategy Against Changing Rules And Records

Social Security rules change, earnings records can contain errors, and the optimal strategy depends on both spouses' full earnings histories. The agent should verify earnings records through the official statement, should not rely on rules that may have changed, and should treat any strategy as needing confirmation against current law before execution.

## Common Traps

### Optimizing Each Spouse Separately

The household optimum is not the sum of two individual optima. The agent should model the combined benefit, especially the survivor period.

### Missing The Survivor Benefit Maximization

Delaying the higher earner maximizes the survivor's lifetime income. Ignoring this leaves the largest source of value on the table.

### Recommending Restricted Application To Ineligible Claimants

Restricted application is available only to those born before the law's cutoff. Recommending it to younger claimants is an error.

### Confusing Spousal Benefit With Own Benefit

The spousal benefit is a top-up to the lower earner's own benefit, capped at roughly half the higher earner's full benefit. The agent should not treat it as an independent payment.

### Applying Married-Couple Rules To Divorced Or Widowed Claimants

Divorced and widowed claimants have distinct eligibility and sequencing rules. The agent should verify the specific conditions rather than generalizing.

### Claiming The Spousal Benefit Early Without Understanding The Reduction

Claiming a spousal benefit before full retirement age permanently reduces it. The agent should account for the reduction in the comparison.

### Ignoring Earnings Record Errors

Errors in earnings records reduce benefits. The agent should recommend verifying records before finalizing a strategy.

## Self-Check

- [ ] The claiming strategy optimizes combined household lifetime benefit, including the survivor period, not each spouse separately.
- [ ] The survivor benefit is maximized by delaying the higher earner, and this is treated as a priority.
- [ ] Spousal benefit calculation, including the top-up and the cap at roughly half the higher earner's full benefit, is explained correctly.
- [ ] Restricted application is recommended only to claimants eligible by birth year, with the eligibility check explicit.
- [ ] Divorced and widowed claimant rules are applied correctly, with eligibility conditions verified.
- [ ] The claim sequence bridges the household income gap during the higher earner's delay without forcing harmful asset sales.
- [ ] Earnings records and current rules are verified before the strategy is treated as final.
- [ ] The reduction for claiming a spousal benefit before full retirement age is included in the comparison.
- [ ] The output includes a disclaimer that it is not personalized advice and that Social Security rules change over time.
- [ ] No strategy is presented as universally optimal without reference to the couple's earnings histories, ages, and longevity.