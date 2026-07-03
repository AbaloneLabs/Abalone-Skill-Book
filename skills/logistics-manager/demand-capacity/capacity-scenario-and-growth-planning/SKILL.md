---
name: capacity_scenario_and_growth_planning.md
description: Use when the agent is forecasting logistics capacity needs, planning for growth, peak volume, new channels, promotions, facility throughput, transportation capacity, labor requirements, or scenario planning across warehouses, fleets, carriers, fulfillment operations, and distribution networks.
---

# Capacity Scenario And Growth Planning

Capacity planning connects commercial ambition to operational reality. It determines whether warehouses, labor, carriers, docks, yards, systems, equipment, and inventory flow can absorb demand without service collapse or wasteful overbuilding. Agents often treat capacity as a simple volume forecast and miss the shape of demand: order mix, cube, weight, handling time, cutoffs, seasonality, service promises, labor productivity, carrier pickup windows, system limits, and exception rates. A strong capacity plan uses scenarios to show what the operation can handle, what breaks first, and which decisions are needed before growth arrives.

Use this skill when planning for volume growth, peak season, channel launches, promotions, facility expansion, labor plans, carrier capacity, or operational readiness. The goal is to turn uncertain demand into actionable capacity decisions.

## Core Rules

### Define The Capacity Unit That Actually Constrains The Operation

Capacity is not only orders per day. The true constraint may be order lines, units, pallets, cube, weight, picks, eaches, cases, cold-chain space, hazmat storage, dock doors, trailers, labor hours, pack stations, sort capacity, carrier pickups, system transactions, customer appointment slots, or returns processing. Different products and channels consume capacity differently.

Identify the capacity unit for each process: receiving, putaway, replenishment, picking, packing, staging, loading, transport, delivery, returns, and customer service. A promotion that doubles small parcel orders may overload pack stations while leaving pallet storage available. A B2B launch may overload dock appointments and paperwork while order count appears modest.

### Build Scenarios, Not A Single Forecast

Forecasts are uncertain, especially when growth comes from new products, new customers, promotions, marketplace changes, acquisitions, weather, or macroeconomic shifts. Build scenarios such as conservative, expected, high, and stress cases. Include timing, not just annual totals. A warehouse may handle annual volume but fail during a two-week spike.

Scenarios should show when capacity is exceeded and by what driver. If the high case requires more labor by September, new equipment by October, or carrier commitments by July, those decision dates belong in the plan. The purpose is not to predict perfectly; it is to prevent surprise.

### Translate Demand Into Workload

Convert commercial forecasts into operational workload. Sales dollars are not enough. Translate demand into orders, lines per order, units per line, cube, weight, storage days, pick method, pack material, temperature zone, hazardous classification, delivery service, returns probability, and support contacts.

Ask what changes in mix. Growth in bulky items affects trailers and storage more than order count. Growth in high-SKU variety affects slotting and pick path. Growth in direct-to-consumer orders affects small parcel, packaging, and customer service. Growth in wholesale affects pallets, appointments, and chargeback compliance. Capacity planning must follow work content.

### Identify Bottlenecks And Dependencies

Every operation has bottlenecks. Adding labor will not help if pack stations, conveyors, dock doors, carrier pickup times, yard space, WMS wave capacity, or replenishment cannot support the additional flow. Likewise, adding carrier capacity will not help if orders cannot be picked before cutoff.

Map process dependencies and find the first constraint under each scenario. Then identify the second constraint that appears after the first is relieved. This prevents solving one visible problem while another immediately blocks throughput. Include upstream and downstream dependencies such as supplier delivery reliability, inbound appointment schedules, inventory availability, customer routing guides, and finance approval cycles.

### Account For Productivity, Learning Curves, And Absence

Labor capacity depends on trained productivity, not headcount alone. New hires, temporary labor, overtime fatigue, absenteeism, turnover, supervisor span, safety incidents, and process changes affect output. Automation may also have ramp-up periods, maintenance downtime, and exception handling needs.

Use realistic productivity assumptions and document them. If the plan assumes temporary labor reaches full productivity in one week, test whether training, quality, and supervision support that. If overtime is used, consider error rates, injury risk, morale, and sustainability. Capacity that exists only on paper will fail in execution.

### Time Decisions To Lead Times

Capacity decisions have lead times. Hiring, training, carrier procurement, warehouse leases, racking, automation, packaging supply, system configuration, dock scheduling, permits, and inventory repositioning may require weeks or months. A capacity plan must show when decisions must be made, not only when capacity is needed.

Create decision gates tied to forecast signals. For example, if confirmed orders exceed a threshold by a date, release temporary labor hiring, reserve overflow storage, add weekend shifts, or secure carrier drop trailers. Without gates, teams either overcommit too early or react too late.

### Balance Buffer Against Waste

Capacity buffers protect service but cost money. Too little buffer creates missed cutoffs, overtime, detention, stockouts, backlog, and customer penalties. Too much buffer creates idle labor, unused space, excess inventory, and unnecessary contracts. The right buffer depends on demand volatility, service criticality, replenishment speed, and cost of failure.

Explain the chosen buffer. Critical healthcare, spare-parts, or high-penalty operations may justify larger reserves. Low-margin, flexible-service operations may accept tighter capacity with clear customer communication. Do not hide the service-risk tradeoff.

### Link Capacity Planning To Commercial Commitments

Sales, marketing, product, and customer teams can create capacity demand through promotions, launches, contract promises, onboarding, and service-level commitments. Logistics must be involved before those promises are made. A capacity plan should state what commercial commitments are supportable and under what assumptions.

If a promotion, new retailer, or service promise exceeds capacity, present alternatives: stagger launch, cap volume, adjust cutoff times, change service level, outsource a lane, pre-build inventory, simplify packaging, or add temporary capacity. The planner's role is to make feasibility clear before customers are affected.

## Common Traps

### Planning Against Average Volume

Averages hide peaks, day-of-week patterns, order cutoff surges, and seasonal spikes. An operation can handle average daily volume and still fail every Monday, every month-end, or during a promotion. Plan against peak profiles and recovery time.

### Measuring The Wrong Unit

Orders, pallets, lines, cube, weight, and labor hours tell different stories. If the plan uses the wrong unit, it may miss the real constraint. A small rise in order count can create a large rise in pick lines or packing complexity.

### Assuming Labor Is Instantly Productive

Temporary or new labor requires recruiting, onboarding, training, supervision, safety orientation, and quality control. Counting heads without productivity ramp creates false capacity and can increase errors.

### Ignoring Carrier And Dock Constraints

Warehouse output is useless if carriers cannot pick up, trailers are unavailable, dock doors are full, appointment windows are missed, or parcel pickups cap out. Transportation capacity must be part of the plan.

### Treating Systems As Unlimited

WMS, TMS, order management, label generation, API calls, scanners, printers, and customer portals can become capacity constraints. System slowdown during peak can reduce throughput even with enough labor.

### Overbuilding For A One-Time Spike

Peak capacity solutions should match the durability of demand. Permanent space, automation, or headcount may be inappropriate for a temporary surge. Consider temporary shifts, overflow, 3PL support, or service-level adjustment.

### Ignoring Recovery Capacity

If the operation falls behind, it needs capacity to recover while new orders continue arriving. Plans that only meet daily demand leave no room to clear backlog after a disruption.

### Disconnecting From Sales Promises

Commercial teams may sell service levels, launch dates, or volumes that operations cannot support. Capacity planning must feed back into what can be promised, priced, or scheduled.

## Self-Check

- [ ] Are the true capacity units identified for receiving, storage, picking, packing, docks, transport, delivery, returns, and support?
- [ ] Are conservative, expected, high, and stress scenarios modeled with timing and peak profiles?
- [ ] Is demand translated into operational workload by order mix, lines, units, cube, weight, service level, and handling requirements?
- [ ] Are first and second bottlenecks identified under each scenario?
- [ ] Are labor productivity, training, absence, turnover, overtime, and supervision assumptions realistic?
- [ ] Are lead times and decision gates shown for labor, carriers, space, equipment, systems, and suppliers?
- [ ] Is the chosen capacity buffer justified by service risk and cost?
- [ ] Are carrier, dock, yard, and system constraints included?
- [ ] Does the plan include recovery capacity after backlog or disruption?
- [ ] Are commercial commitments checked against operational capacity before promises are made?
