---
name: demand-capacity-forecasting-model-risk.md
description: Use when the agent is applying AI or analytics to logistics demand forecasting, capacity planning, volume prediction, labor forecasting, carrier capacity, warehouse throughput forecasts, or model risk in supply chain operations.
---

# Demand Capacity Forecasting Model Risk

AI forecasting in logistics can improve planning, but it can also create confident errors that cascade into inventory, labor, transportation, service, and cost. Agents often discuss forecast accuracy abstractly and miss the operational question: what decision will this forecast change, how wrong can it be, and who catches it before capacity is committed? This skill helps treat demand and capacity models as decision systems with risk controls.

## Core Rules

### Define the decision the forecast supports

Clarify whether the model drives labor scheduling, carrier procurement, dock appointments, inventory deployment, warehouse slots, fleet sizing, replenishment, production handoff, or customer promise dates. Forecast granularity, horizon, and accuracy metric should match that decision.

A weekly national forecast may be useless for daily dock labor. A SKU forecast may not help if the operational constraint is cube, pallets, appointments, or driver hours. Start with the decision, not the model.

### Forecast operational units, not only sales units

Logistics capacity is consumed by orders, lines, cases, pallets, cube, weight, stops, miles, temperature class, hazmat class, labor minutes, dock doors, trailers, and returns. Translate demand into the units that actually constrain work.

Revenue or unit volume can hide the true load. A small number of bulky orders may consume more capacity than many small items, and returns can disrupt labor even when sales are flat.

### Identify data drift and structural breaks

Demand and capacity patterns change after promotions, product launches, channel shifts, weather events, labor disruptions, customer onboarding, network redesign, pricing changes, system conversions, or policy changes. Models trained on old patterns may fail precisely when decisions matter most.

Set alerts for drift, missing data, unusual input values, and changes in business process. Do not treat model output as stable just because the dashboard refreshed.

### Compare model error to operational tolerance

Measure not only statistical error but the cost of being wrong. Under-forecasting may cause missed service, overtime, premium freight, stockouts, and labor stress. Over-forecasting may create idle labor, excess capacity, inventory misplacement, and detention.

Different lanes, customers, products, and days have different tolerances. A 10 percent error may be acceptable in one warehouse and severe during a launch or holiday peak.

### Preserve human context and override rules

Planners, warehouse leaders, sales, procurement, carriers, and customer teams may know about events that data has not captured. Define when humans can override the model, what evidence is required, and how overrides are recorded and reviewed.

Human override should not be random opinion, but neither should the model suppress fresh operational knowledge. Track both model output and human adjustments.

### Plan scenario and stress testing

Use scenarios for demand spikes, capacity losses, weather disruption, supplier delay, labor shortage, carrier failure, and customer acceleration. Forecasting should support decisions under uncertainty, not only produce a single expected number.

Stress tests reveal whether the network has buffers, flexible labor, alternate carriers, inventory repositioning options, and escalation triggers before the disruption arrives.

### Monitor downstream behavior

A forecast changes behavior. Teams may overbook labor, reduce safety stock, delay carrier procurement, or promise service based on the model. Monitor whether users understand confidence intervals, assumptions, and the intended use of the forecast.

Model risk includes misuse. A good forecast can create bad outcomes when applied to the wrong decision or treated as a guarantee.

### Close the learning loop

After each planning cycle, compare forecast, human overrides, actual demand, capacity used, service results, premium costs, and root causes. Feed learning back into model features, business rules, and planning cadence.

Do not evaluate only whether the model was numerically accurate. Evaluate whether it improved operational decisions.

## Common Traps

- Building an impressive forecast without identifying the capacity decision it supports.
- Forecasting sales units while labor, cube, stops, dock doors, or driver hours are the real constraints.
- Ignoring data drift after launches, promotions, network changes, weather, or customer behavior shifts.
- Reporting average accuracy while hiding high-impact errors on critical lanes or peak days.
- Treating model output as a promise rather than a planning input with uncertainty.
- Allowing human overrides without evidence, recording, or post-review.
- Blocking human overrides when teams know about events not present in the data; skipping stress tests for demand spikes, capacity losses, and operational disruptions
- Letting downstream teams misuse forecasts for decisions outside the model's scope; failing to compare forecast output to service, cost, labor, and customer outcomes

## Self-Check

- Is the exact logistics decision supported by the forecast defined?
- Are forecast horizon, granularity, and metric aligned to that decision?
- Does the model forecast operational constraints such as orders, lines, pallets, cube, stops, miles, labor minutes, dock doors, and returns?
- Are drift, missing data, unusual inputs, and structural breaks monitored?
- Is forecast error translated into operational service, cost, labor, inventory, and capacity risk?
- Are human override rules, evidence standards, and review processes defined?
- Are scenarios and stress tests run for spikes, capacity loss, weather, supplier delay, labor shortage, and carrier failure?
- Do users understand confidence, assumptions, and prohibited uses of the forecast?
- Does the learning loop compare model, overrides, actuals, decisions, service, premium cost, and root causes?
