---
name: unit_economics.md
description: Use when the agent is evaluating unit economics, customer acquisition cost, lifetime value, contribution margin, payback period, scalability, break-even thresholds, or validating the underlying economics of a business model before forming a view on growth quality, profitability, or the sustainability of a subscription, marketplace, or direct-to-consumer business.
---

# Unit Economics

Unit economics break a business down to the profitability of a single unit of activity, one customer, one order, one transaction, one store, one subscriber, and ask whether that single unit, repeated many times, creates or destroys value. This matters because aggregate growth and aggregate losses can hide fundamentally broken economics: a company can grow revenue spectacularly while losing money on every customer, and if each new customer destroys value, then growth itself is the problem rather than the solution. Investing agents frequently accept headline growth and a vague promise of future margins without ever checking whether the underlying unit is profitable, how long payback takes, whether margins improve with scale, and what assumptions about retention and churn the whole model depends on. Unit economics are where the optimism of a growth story meets the arithmetic of value creation.

Use this skill before answering questions such as "are this company's unit economics good", "is the growth sustainable", "when will it turn profitable", "how do I evaluate a subscription or DTC business", or "does scale fix the margins". The goal is to prevent the agent from treating growth as self-evidently good, to force it to compute or estimate the key unit metrics, to test the sensitivity of the model to churn and acquisition cost, and to distinguish businesses that burn cash to build durable economics from those that burn cash to mask broken ones.

Unit economics analysis supports business model assessment, not investment recommendation. Conclusions should reflect uncertainty and the investor's own objectives and risk tolerance.

## Core Rules

### Define The Unit Clearly Before Measuring It

Unit economics are only meaningful once the unit is defined, and the right unit depends on the business model. A mismatched unit produces misleading metrics.

Common unit definitions:

- Per-customer, for subscription, SaaS, telecom, and DTC businesses, where one customer generates recurring revenue over time.
- Per-order or per-transaction, for marketplaces, delivery, and e-commerce, where each transaction has its own contribution margin.
- Per-store, per-location, or per-vehicle, for retail, restaurants, and asset-based services, where each physical unit is a profit center.
- Per-unit-of-product, for manufacturing and commodities, where each item has a cost and a price.

Define the unit, then measure revenue, variable cost, and contribution margin for that unit. A business can have strong per-customer economics but weak per-transaction economics, or vice versa, so the choice of unit shapes the entire analysis.

### Compute Customer Acquisition Cost And Lifetime Value Honestly

For customer-based businesses, the central pair of metrics is customer acquisition cost, CAC, and lifetime value, LTV, and both are routinely miscalculated to flatter the picture.

CAC should include:

- All marketing and sales spend, not just digital ad spend, including sales headcount, content, partnerships, and brand.
- Spend attributable to acquiring new customers, separated from spend that retains or reactivates existing ones.
- The full cost including overheads and tools, not just media spend.

LTV should reflect:

- Average revenue per customer per period.
- Gross margin on that revenue, not revenue itself, because the customer must be profitable after variable costs.
- The realistic customer lifetime derived from observed retention or churn, not an optimistic assumption.
- Cohort behavior, because retention often declines with cohort age and blended averages hide deterioration.

The LTV-to-CAC ratio is a common summary, but it is only as good as the inputs. A 3:1 ratio built on inflated LTV or understated CAC is meaningless.

### Measure Payback Period And Cash Conversion

Payback period, how many months or years of customer contribution are needed to recover the acquisition cost, is often more informative than the LTV-to-CAC ratio because it directly addresses the cash dynamics of growth.

Assess:

- The CAC payback in months, using contribution margin, not gross revenue.
- Whether payback is shortening or lengthening as the business scales, lengthening payback is a warning that acquisition is getting harder or more expensive.
- The implication for cash flow, if payback is long and the business is growing fast, it must fund an ever-larger working investment in new customers, which can consume cash even when unit economics are positive.
- The reinvestment rate and whether the business can self-fund growth or requires continuous external capital.

A business with positive unit economics but long payback can still burn cash aggressively while growing. Payback period links unit profitability to the cash flow statement.

### Test The Scalability And Margin Trajectory

The promise of many growth businesses is that margins improve with scale, as fixed costs spread, unit costs fall, and the mix shifts toward higher-margin products. The agent must test whether this is actually happening rather than assume it.

Examine:

- Whether contribution margin per unit is rising, stable, or falling as volume grows.
- Whether CAC is stable or rising, because in many markets acquisition gets harder and more expensive as the easy customers are saturated.
- Whether gross margin improves with scale or is structurally capped by the cost structure.
- The mix of growth, is it coming from more units, higher prices, better retention, or lower cost, and which of those are sustainable?
- Operating leverage, the gap between contribution margin and operating margin, and whether fixed costs are truly fixed or rise with scale.

Scalability is proven by a margin trajectory, not asserted by a plan. A business whose unit economics have not improved despite years of scale may never achieve the promised leverage.

### Stress-Test Churn And Retention Assumptions

For subscription and recurring-revenue businesses, the entire model rests on retention, and small changes in churn produce large changes in LTV. The agent should never accept stated retention without examining cohort data.

Stress-test:

- Gross and net revenue retention by cohort, and whether retention is improving, stable, or decaying with cohort age.
- The sensitivity of LTV to a 1, 2, or 5 percentage point change in churn.
- The difference between logo churn, losing customers, and revenue churn, losing small versus large customers.
- Expansion revenue from existing customers, which can mask high logo churn and is not guaranteed to persist.
- The maturity of the customer base, early cohorts often retain better than later, broader cohorts.

A model that looks attractive at 5% annual churn can be value-destroying at 12% churn. Churn sensitivity is the single most important stress test for recurring-revenue unit economics.

### Distinguish Contribution Profitability From Accounting Profitability

A unit can be contribution-positive, covering its variable costs, while the company still loses money because fixed costs, overhead, and growth investment exceed total contribution. Both views matter.

Reconcile:

- Sum the contribution across all units and compare to fixed costs and overhead to see the path to operating profit.
- Identify the break-even scale, the number of units or customers at which total contribution covers fixed costs.
- Separate the economics of the existing book of business from the economics of growth, a mature business can be profitable while growth investment burns the difference.

A business that is contribution-positive but cash-negative because it is acquiring customers faster than payback allows is in a different position than one that is contribution-negative on every unit. The first may be funding durable value; the second is destroying it with each sale.

## Common Traps

### Accepting Headline Growth Without Unit Profitability

Revenue growth that comes from acquiring value-destroying customers accelerates losses. Growth is only valuable if the unit creates value.

### Using Flattered CAC And LTV

Understated CAC by excluding sales headcount, or overstated LTV by using revenue instead of margin or optimistic retention, produces a misleading ratio. The inputs determine the truth.

### Assuming Scale Will Fix The Margins

Many businesses show no margin improvement despite years of scale. Scalability must be observed in the data, not assumed from the plan.

### Ignoring Payback Period And Cash Dynamics

Positive unit economics with long payback can still consume vast cash during growth. Payback links units to the cash flow statement.

### Blending Cohorts And Hiding Churn

Blended retention averages can hide decaying later cohorts. Cohort-level retention is the only reliable view of churn.

### Confusing Contribution Profit With Operating Profit

A unit can be contribution-positive while the company loses money due to fixed costs and growth investment. Both layers must be reconciled.

### Treating Expansion Revenue As Guaranteed

Net revenue retention can look strong because of expansion from existing customers while logo churn is high. Expansion is not guaranteed to persist and can mask underlying churn.

## Self-Check

- [ ] The unit was defined clearly, per customer, per order, per location, per product, and revenue, variable cost, and contribution margin were measured for that unit.
- [ ] CAC was computed including all sales and marketing spend, and LTV was computed using gross margin and realistic, cohort-derived retention.
- [ ] Payback period was measured in months using contribution margin, and its trend and cash flow implications were assessed.
- [ ] Scalability was tested against actual margin and CAC trajectory, not assumed from the business plan.
- [ ] Churn and retention assumptions were stress-tested at the cohort level, including sensitivity of LTV to churn changes.
- [ ] Contribution profitability was reconciled to operating profitability, and the break-even scale was estimated.
- [ ] Expansion revenue was distinguished from logo retention, and net retention was not treated as guaranteed.
- [ ] The path from current losses to profitability was grounded in unit and scale economics, not asserted.
- [ ] The analysis distinguishes a business funding durable value from one destroying value with each unit.
- [ ] The conclusion frames unit economics as business model assessment, notes uncertainty and sensitivity, and accounts for investor-specific objectives and risk tolerance.