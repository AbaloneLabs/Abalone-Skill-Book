---
name: demand-supply-balancing-and-sop-input.md
description: Use when the agent is balancing demand and supply, preparing operations input for S&OP or S&OE, translating forecasts into capacity and supply actions, reconciling demand plans with inventory and supplier constraints, prioritizing constrained supply, or reviewing planning assumptions where risks include optimistic forecasts, hidden capacity gaps, double counting buffers, weak scenario planning, or decisions that improve one metric while damaging service, cost, or resilience.
---

# Demand Supply Balancing And SOP Input

Demand-supply balancing is where forecasts meet physical and operational constraints. It is not the same as demand planning, procurement, or inventory control, though it depends on all three. Agents often accept a forecast as the plan and then suggest generic capacity or purchasing actions without checking lead time, supplier constraints, inventory quality, service promises, workforce capacity, and financial tradeoffs. This skill helps the agent prepare operational input that makes planning decisions realistic and accountable.

Use this skill when preparing S&OP or S&OE input, balancing supply with demand, reviewing constrained plans, translating demand scenarios into operations requirements, or deciding how to prioritize limited supply. The agent should make assumptions visible and avoid presenting a single optimistic plan as operational truth.

## Core Rules

### Separate unconstrained demand from feasible supply

Start by distinguishing what customers, sales, or forecasts want from what the operation can realistically supply. Unconstrained demand may include market upside, promotions, sales targets, backlog, seasonality, or strategic commitments. Feasible supply depends on inventory, supplier capacity, lead time, labor, equipment, logistics, quality, cash, and service constraints.

Do not collapse these into one number. A plan that hides constraints will fail late, when fewer recovery options remain. Show the gap, timing, assumptions, and decision needed.

### Convert forecasts into operational requirements

Forecast units must translate into materials, labor hours, machine time, storage space, transportation, supplier orders, quality checks, customer support load, and cash needs. If the forecast is at a high level, identify the mix assumptions that matter: product, region, channel, customer tier, configuration, size, perishability, or service level.

Volume without mix can mislead. The same unit count may create very different capacity needs if the mix shifts toward complex, regulated, bulky, or labor-intensive items.

### Check buffers and constraints explicitly

Organizations often add buffers in several places: sales forecast, planning forecast, safety stock, supplier commitment, warehouse capacity, expedited transport, and staffing assumptions. Double-counted buffers create excess cost; missing buffers create fragile plans. Identify where protection exists and what risk it is protecting.

Also name hard constraints: supplier minimums, production batch size, storage limit, shelf life, customs lead time, equipment capacity, labor availability, regulatory approval, or budget ceiling. The balancing decision depends on which constraints can be flexed and which cannot.

### Use scenario planning for material uncertainty

When demand, supply, or lead time is uncertain, build a small set of scenarios: base, upside, downside, and disruption. For each scenario, show service impact, inventory position, labor need, supplier action, cost, and trigger points. Avoid pretending the base case is the only plan.

Scenarios should drive decisions. If the upside scenario requires supplier reservation by a certain date, that decision must be surfaced. If the downside scenario creates excess inventory, the cancellation or redeployment path should be known.

### Make allocation principles visible

When supply is constrained, decide how to allocate. Options include first-come first-served, strategic customer priority, margin, contractual obligation, safety or regulatory need, geographic fairness, launch commitments, backorder age, or customer vulnerability. Each rule has tradeoffs.

Do not let allocation happen through ad hoc escalation. A transparent rule prevents hidden favoritism and reduces conflict between sales, support, finance, and operations.

### Reconcile plan with financial and service tradeoffs

Supply actions affect cost and cash: overtime, premium freight, supplier expedite fees, inventory carrying cost, obsolescence, write-offs, service penalties, and lost sales. They also affect service: fill rate, cycle time, customer promise, cancellation, substitution, and backlog.

Present tradeoffs together. A plan that preserves service by using emergency freight may be justified, but the decision should be intentional. A plan that protects cost while increasing backlog should also be explicit.

### Keep planning cadence and ownership tight

Demand-supply balancing needs a rhythm. Define who updates demand, supply, inventory, constraints, decisions, and exceptions; when the plan freezes; how changes are approved; and which issues go to executive review. Without cadence, the plan becomes stale quickly.

S&OP is not complete when a meeting happens. It is complete when decisions are captured, owners are assigned, and downstream teams use the same plan.

### Track plan quality and learning

After execution, compare forecast, supply plan, actual demand, actual supply, service level, expedites, shortages, excess, and assumptions. Identify whether misses came from forecast error, supplier failure, internal capacity, data quality, decision delay, or execution failure.

Use the learning to improve planning rules, buffers, supplier commitments, and escalation thresholds. Repeating the same miss each cycle means the process is not learning.

## Common Traps

- Treating the forecast as feasible supply without checking constraints.
- Planning at total volume while ignoring mix changes that drive capacity and material needs.
- Adding buffers in multiple places without knowing the total protection or cost.
- Building scenarios that do not trigger decisions or actions.
- Allocating constrained supply through whoever escalates most loudly.
- Optimizing cost, service, or inventory in isolation.
- Holding S&OP meetings without decision ownership and downstream plan updates.
- Explaining misses as forecast error when supplier, data, capacity, or decision timing caused the failure.

## Self-Check

- Are unconstrained demand and feasible supply shown separately?
- Are forecast assumptions translated into materials, labor, equipment, storage, transport, quality, and cash needs?
- Are product mix, region, channel, customer tier, and service level assumptions explicit where they matter?
- Are buffers identified without double counting?
- Are hard constraints and flex options named?
- Do scenarios include triggers, decisions, owners, cost, and service impact?
- Are constrained-supply allocation principles visible and approved?
- Are service, cost, inventory, and cash tradeoffs shown together?
- Are cadence, freeze rules, decision rights, and downstream plan updates clear?
- Is plan accuracy reviewed against actuals with learning fed into the next cycle?
