---
name: customer_profitability_analysis.md
description: Use when the agent is measuring profitability by customer, assigning revenue and service costs to customers, identifying which customers erode margin, or recommending actions on unprofitable accounts based on a customer profitability model.
---

# Customer Profitability Analysis

Not all customers are equally profitable, and the difference is often hidden. Revenue reporting shows which customers buy the most, but it says nothing about what it costs to serve them. A high-revenue customer who demands constant support, custom terms, small shipments, and extended payment may destroy value, while a smaller customer who orders predictably and pays promptly may generate most of the profit. Customer profitability analysis exposes this, but only if the cost assignment reflects real service consumption rather than an arbitrary spread.

Use this skill before building a customer profitability model, assigning costs to customers, ranking accounts by profit, recommending pricing or service changes, or deciding which customers to grow, constrain, or exit. The goal is to prevent the agent from equating revenue with value, from allocating service costs by an arbitrary base, or from recommending account actions on a model that hides the real drivers of profitability.

## Core Rules

### Define The Customer And The Scope Of Analysis

Customer profitability means different things at different levels. Define what is being measured before assigning any cost.

Clarify:

- the customer entity, parent, sold-to, ship-to, or contract level;
- the time period, monthly, quarterly, or annual;
- whether the analysis includes only product margin or full service and support cost;
- whether related parties, distributors, or end-users are treated separately;
- the currency and any intercompany elimination required.

A profitability ranking is only meaningful if every customer is measured on the same basis. Mixing levels or periods produces non-comparable results.

### Start With Revenue Net Of True Customer-Specific Concessions

Gross revenue rarely reflects what the customer actually contributes. Adjust for the concessions that vary by customer.

Capture:

- net revenue after returns, allowances, and rebates;
- volume discounts and pricing tiers specific to the customer;
- payment terms and the cost of extended credit;
- promotional or co-marketing funding;
- chargebacks, penalties, and service credits;
- the effect of payment delinquency or bad debt risk.

Do not allocate general discounts or corporate promotions to individual customers unless they are truly customer-specific. Over-allocation distorts the ranking.

### Assign Direct Service Costs By Actual Consumption

The core of customer profitability is assigning the costs of serving each customer based on what they actually consume, not by a revenue spread.

Assign by consumption where possible:

- order processing by number of orders or order lines;
- fulfillment and freight by shipments, weight, or special handling;
- customer service by contacts, tickets, or time spent;
- technical support by cases or engineering hours;
- account management by dedicated headcount or reported time;
- returns and reverse logistics by return volume;
- credit and collection effort by days outstanding and collection activity.

Where direct tracing is not possible, use the strongest available driver and document it. Spreading service cost by revenue reproduces the distortion the analysis is meant to fix.

### Handle Shared And Fixed Costs Deliberately

Some costs cannot be traced to a customer. Facilities, executive oversight, and shared technology benefit the whole portfolio. How these are treated changes the conclusions.

Decide and document:

- which costs are directly traceable to customers;
- which shared costs are allocated using a defensible driver;
- which corporate or fixed costs are held as unallocated portfolio cost;
- the effect of each layer on the profitability ranking.

Reporting multiple levels, from contribution margin up to fully allocated profit, is often more useful than a single number. It lets management see where value is created before arbitrary allocations obscure it.

### Validate The Model Against Operational Reality

A customer profitability model must be reconciled to the ledger and sense-checked with the people who manage the accounts.

Validate by:

- reconciling total revenue and assigned cost to the ledger;
- reviewing surprising results with sales and service managers;
- checking that known difficult customers show low profitability;
- checking that known easy customers show high profitability;
- testing sensitivity to the allocation bases used.

When the model contradicts the lived experience of account managers, either the model is wrong or the managers' intuition is wrong. Investigate before acting.

### Use The Output To Inform, Not Mechanically Enforce, Decisions

Customer profitability is decision support, not an autopilot. A customer who looks unprofitable may be strategic for reasons the model cannot see: reference value, entry into a new segment, a pipeline to larger accounts, or cross-sell potential.

Use the analysis to:

- identify candidates for pricing, term, or service-level conversations;
- highlight process improvements that reduce service cost;
- inform account segmentation and resource allocation;
- support exit decisions only after strategic review.

Present results with the assumptions and limitations explicit. A ranking presented as definitive truth invites bad decisions and erodes trust in the model.

### Respect Confidentiality And Data Sensitivity

Customer profitability data is sensitive. It combines revenue, cost, payment behavior, and sometimes individual account-manager information. Restrict access to those who need it for decisions, and avoid embedding it in broadly shared reports.

### Acknowledge Management Accounting Limits

Customer profitability is a management accounting tool. It does not govern external segment reporting, transfer pricing, or tax. Where profitability analysis informs regulated pricing, government contracts, or intercompany arrangements, specific rules may constrain the methods and allocations permitted. Confirm the applicable framework with a qualified professional before using the outputs in external or regulated contexts.

## Common Traps

### Equating Revenue With Profit

Ranking customers by revenue hides the cost to serve and almost always overstates the value of demanding accounts.

### Allocating Service Cost By Revenue

Spreading service cost by revenue makes high-revenue customers look cheaper to serve than they are and entrenches the distortion the analysis was meant to expose.

### Presenting A Single Fully-Allocated Number

A single bottom-line number hides where value is created. Layered reporting, from contribution to fully allocated, is more useful and more honest.

### Ignoring Payment Behavior

A customer who pays slowly or defaults carries a real cost of capital and risk that pure margin analysis misses.

### Acting Mechanically On The Ranking

Exiting an unprofitable customer without strategic review can destroy future value, relationships, or market position.

### Stale Models

Customer behavior and service processes change. A model built once and never refreshed produces confident-looking numbers that no longer reflect reality.

### Confusing Management Analysis With Reporting

Customer profitability is internal decision support. Using it in external segment reporting, transfer pricing, or tax without framework review can create errors.

## Self-Check

- Is the customer entity, period, scope, and currency defined consistently across all accounts analyzed?
- Is revenue adjusted for customer-specific concessions, discounts, payment terms, and credit risk?
- Are direct service costs assigned by actual consumption rather than by a revenue spread?
- Are shared and fixed costs handled deliberately, with traceable, allocated, and unallocated layers reported separately?
- Does the model reconcile to the ledger and validate against the operational experience of account managers?
- Is the output used to inform decisions rather than mechanically enforce account actions?
- Is access to sensitive customer profitability data restricted appropriately?
- Does the analysis acknowledge management accounting limits and flag where regulated or external use requires framework confirmation and professional review?
