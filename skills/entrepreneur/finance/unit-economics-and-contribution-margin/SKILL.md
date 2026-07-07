---
name: unit_economics_and_contribution_margin.md
description: Use when the agent is calculating or interpreting unit economics for a startup, computing gross margin and contribution margin per customer, modeling customer lifetime value against acquisition cost, diagnosing whether a business gets more profitable or less as it scales, or deciding whether the economics support growth or require repricing and cost restructuring.
---

# Unit Economics And Contribution Margin

Revenue growth that hides negative unit economics is not progress; it is the controlled acceleration of losses. A startup can hit every growth target, raise on the strength of a hockey-stuck chart, and still be building a business that loses more money the bigger it gets. Unit economics is the discipline of asking, for each customer or transaction, whether the business earns more than it costs to acquire and serve. When the answer is yes, growth compounds profit. When the answer is no, growth compounds loss, and no amount of volume fixes a broken per-unit model. The judgment problem is not the arithmetic of margin but the honesty of what is included in the unit, because founders routinely exclude costs that scale with customers, such as support, infrastructure, payment processing, and onboarding, producing a flattering margin that dissolves at scale.

Use this skill when modeling or evaluating a startup's per-customer economics, when growth is being funded before unit economics are proven, when deciding whether to scale sales, or when diagnosing why a growing company is burning more rather than less. The goal is to prevent the agent from declaring a model profitable by omitting the costs that scale, and to keep it from freezing growth unnecessarily when the economics are genuinely sound.

## Core Rules

### Define The Unit Before Computing Economics

Unit economics means nothing if the unit is ambiguous. A unit can be a transaction, a subscription month, a customer lifetime, or a contract. Different choices answer different questions, and mixing them produces nonsense.

Define explicitly:

- the unit: one transaction, one month of subscription, one customer over their lifetime;
- the period: monthly contribution versus lifetime contribution;
- the cohort: new customers only, or blended with existing;
- the segment: all customers, or a specific ICP where economics differ.

Monthly contribution answers whether each active customer is profitable to serve. Lifetime contribution, incorporating churn, answers whether the customer is profitable to acquire. Both matter, and they can disagree.

### Include All Costs That Scale With Customers

The most common unit-economics error is computing margin using only cost of goods sold while excluding the variable costs that grow with the customer base. A margin that ignores support tickets, cloud usage, payment fees, onboarding labor, and churn-prevention effort is not the real margin; it is a best case that the business will never realize.

Include in per-unit variable cost:

- infrastructure and hosting that scales with usage or accounts;
- payment processing and transaction fees;
- customer support and success effort attributable per customer;
- onboarding and implementation labor for new customers;
- software, data, and third-party API costs consumed per customer;
- variable sales commissions if computing contribution after acquisition.

The test is whether the cost would grow if the customer base grew. If yes, it belongs in the unit. Fixed costs that do not scale with customers, such as the core engineering team, belong below the line, not in the per-unit margin.

### Separate Contribution Margin From Gross Margin

Gross margin is an accounting construct defined by what sits above the cost-of-revenue line. Contribution margin is an economic construct: revenue minus all variable costs to acquire and serve the customer. The two often differ, and investors and founders talk past each other by using the same word for different numbers.

Clarify which is being discussed:

- gross margin: revenue minus cost of revenue as defined by accounting standards;
- contribution margin: revenue minus all variable costs of serving the customer;
- fully-loaded contribution: contribution minus fully attributed acquisition cost;
- lifetime contribution: contribution accumulated over the customer's expected life.

A startup can show a healthy gross margin and a negative contribution margin if significant variable costs sit below the gross-margin line. Decisions about scaling should be made on contribution, not gross margin alone.

### Model Lifetime Value With Honest Churn And Gross Margin

Customer lifetime value is one of the most manipulated numbers in startup finance. It is computed by dividing average revenue by churn rate and multiplying by gross margin, and small changes in either input produce wildly different outputs. An optimistic churn assumption and a flattering margin turn a weak business into a seemingly attractive one.

Compute LTV conservatively by:

- using observed churn from real cohorts, not target churn;
- using contribution margin, not gross margin, if variable costs are significant;
- accounting for revenue contraction within accounts, not just full churn;
- separating cohorts, because early customers often churn differently than later ones;
- acknowledging that LTV is a forecast, not a measurement, and stating its sensitivity.

An LTV number presented without its churn and margin assumptions is not informative.

### Judge The Business By Whether Margin Improves Or Deteriorates With Scale

Some businesses have improving unit economics as they grow: infrastructure costs amortize, support becomes more efficient, pricing power increases. Others have deteriorating economics: each marginal customer costs more to acquire, support burden grows faster than revenue, or pricing erodes as the market saturates. The direction matters more than the current level.

Assess the trajectory by asking:

- does customer acquisition cost rise or fall as the company scales;
- does per-customer support cost fall with productization and self-service;
- does gross margin improve with scale or compress with competition;
- does churn improve as the product matures or worsen as the base broadens.

A business with modest but improving unit economics may be worth scaling. A business with strong but deteriorating economics is worth understanding before pouring in capital.

### Use Unit Economics To Decide Whether To Scale Sales

The decision to fund aggressive growth should follow, not precede, proven unit economics. Scaling customer acquisition when contribution per customer is negative burns capital to acquire customers who destroy value. The right sequence is to fix the model first, then pour fuel on it.

Before scaling acquisition, confirm:

- contribution margin per customer is positive after all variable serve costs;
- LTV exceeds CAC by a defensible multiple, commonly three or more for subscription businesses, with a payback period the balance sheet can sustain;
- the channels being scaled have been tested at smaller volume and hold at larger volume;
- the economics are not dependent on a temporary channel arbitrage that will close.

If unit economics are negative, the answers are repricing, cost reduction, or a different segment, not more growth.

### Segment Unit Economics, Do Not Average

Blended unit economics across segments can hide a healthy segment subsidizing a broken one. The startup may be pouring acquisition spend into a segment with poor retention while a smaller segment has excellent economics. Segment-level unit economics reveal where to concentrate.

Compute economics per segment by:

- separating CAC and LTV by acquisition channel and customer type;
- identifying which segments have the shortest payback period;
- identifying which segments drain support and churn early;
- shifting acquisition spend toward segments with better per-unit economics.

A blended CAC:LTV ratio that looks acceptable may conceal a segment worth abandoning and a segment worth dominating.

## Common Traps

### Omitting Variable Costs That Scale With Customers

Excluding support, infrastructure, and onboarding from the unit produces a margin the business will never achieve. Include every cost that grows with the customer base.

### Using Gross Margin When Contribution Margin Is The Decision Input

Gross margin is an accounting figure. Scaling decisions should be made on contribution margin, which includes all variable serve and acquire costs.

### Inflating LTV With Optimistic Churn And Margin

Small changes in churn and margin inputs swing LVO dramatically. Use observed cohort data and report sensitivity, not a single flattering number.

### Scaling Acquisition Before Unit Economics Are Proven

Funding growth on a negative-contribution model accelerates losses. Fix the model, then scale.

### Averaging Across Segments

Blended unit economics hide a strong segment and a weak one. Compute per segment and shift spend toward the stronger.

### Assuming Margin Improves Automatically With Scale

Scale improves some costs and worsens others. Test whether acquisition cost, support burden, and pricing power actually move favorably before assuming they will.

### Treating A Temporary Channel Arbitrage As Durable Economics

Cheap acquisition from a channel that will saturate or close is not a sustainable unit-economics advantage. Distinguish durable efficiency from temporary arbitrage.

## Self-Check

- [ ] The unit of analysis, transaction, month, lifetime, is explicitly defined and used consistently.
- [ ] All costs that scale with customers, including support, infrastructure, fees, and onboarding, are included in the per-unit margin.
- [ ] Contribution margin is distinguished from gross margin, and scaling decisions use the former.
- [ ] Lifetime value is computed from observed cohort churn and contribution margin, with sensitivity reported.
- [ ] The direction of unit economics with scale, improving or deteriorating, has been assessed, not just the current level.
- [ ] Acquisition is being scaled only after per-customer contribution is positive and LTV exceeds CAC by a defensible multiple with an acceptable payback period.
- [ ] Unit economics are computed per segment, not blended, and acquisition spend is shifting toward stronger segments.
- [ ] No cost has been excluded on the grounds that it is "fixed" when it actually grows with the customer base.
- [ ] The CAC:LTV ratio is not dependent on a channel arbitrage likely to close as volume scales.
- [ ] The model honestly reflects whether the business becomes more or less profitable as it grows.