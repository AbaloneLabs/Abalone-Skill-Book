---
name: supply-chain-disruption-and-contingency.md
description: Use when the agent is planning logistics disruption response, contingency routing, carrier or port failure, weather events, strikes, geopolitical risk, supplier interruption, transportation delays, capacity shortages, or supply-chain resilience actions.
---

# Supply Chain Disruption And Contingency

Logistics disruption planning is not a generic backup plan. It is the discipline of deciding which flows matter most, which risks are plausible, what early warnings to monitor, what alternatives are actually executable, and who can authorize tradeoffs under pressure. Agents often respond to disruption by suggesting expedited freight or alternate suppliers without checking capacity, contracts, cost exposure, regulatory constraints, or downstream consequences. This skill helps the agent design contingency thinking that is specific enough to use when the network is stressed.

## Core Rules

### Identify The Disruption Type And Affected Flow

Start by naming the disruption: weather, port congestion, labor strike, carrier bankruptcy, cyber outage, customs hold, supplier shutdown, production delay, road closure, fuel shortage, geopolitical event, pandemic control, warehouse outage, system failure, or sudden demand spike. Then identify the affected lanes, SKUs, customers, facilities, modes, carriers, suppliers, and time windows.

Avoid treating all disruptions as transportation delays. A port strike affects containers, chassis, demurrage, inventory positioning, and production schedules. A WMS outage affects picking and inventory accuracy. A supplier shutdown affects inbound replenishment, production planning, and allocation. The response should match the disruption's mechanism rather than its symptom.

### Segment By Criticality Before Spending Money

When disruption hits, not every shipment deserves the same recovery effort. Segment orders and materials by customer commitment, revenue, margin, contractual penalty, production-line risk, patient or safety impact, shelf life, event deadline, replacement availability, and strategic relationship. Define what is must-save, what can be delayed, what can be substituted, and what should be cancelled or backordered.

Expediting everything is usually a sign of no prioritization. Air freight, team drivers, premium parcel, charters, and spot capacity should be reserved for flows where the cost is justified. A strong plan explains the business reason for spending or not spending, rather than using urgency as the only criterion.

### Verify That Alternatives Are Executable

Backup routes and modes must be real. Check carrier capacity, equipment availability, lane permits, customs feasibility, transit time, cutoff times, warehouse labor, packaging suitability, temperature control, hazmat restrictions, product dimensions, insurance, customer receiving hours, and system ability to process the alternate route. A theoretically available mode can fail if the freight is not packaged, documented, or approved for it.

For international flows, alternate ports, airports, border crossings, or countries may change duties, documentation, licenses, sanctions exposure, transit permits, and importer requirements. For domestic flows, alternate carriers may require onboarding, EDI setup, labels, account numbers, routing-guide exceptions, or insurance review. Confirm the operational details before promising recovery.

### Monitor Leading Indicators

Contingency is stronger when activated before failure. Monitor weather forecasts, port dwell, carrier tender rejection, on-time performance, inventory cover, supplier production updates, customs exams, terminal appointments, labor news, road closures, geopolitical advisories, fuel constraints, and system incident reports. Use thresholds to trigger action: days of supply below a level, tender rejection above a level, dwell time exceeding a limit, or a critical customer deadline at risk.

Do not wait for a missed delivery to declare disruption. Early action may allow mode shift, pre-pull, alternate sourcing, customer communication, or inventory allocation. Late action often leaves only expensive and unreliable choices.

### Coordinate Decisions Across Functions

Disruption response affects sales, customer service, procurement, planning, finance, legal, trade compliance, operations, and leadership. Decide who can approve premium freight, customer allocation, substitution, late delivery communication, carrier changes, supplier changes, inventory reservation, and contract exceptions. If authority is unclear, teams may take conflicting actions.

Communication should separate facts, assumptions, decisions needed, and next update time. Customers and internal stakeholders need realistic commitments, not optimism. If the root cause or ETA is uncertain, state uncertainty and what is being done to reduce it. Overpromising during disruption damages trust more than a clear delay.

### Capture Cost And Learning

Disruption creates extraordinary costs: premium freight, storage, demurrage, detention, labor overtime, packaging changes, chargebacks, lost sales, scrap, and customer credits. Track costs by cause and decision. This helps finance understand impact and helps leadership decide whether resilience investments are justified.

After the disruption, review what worked. Did safety stock cover the risk? Were alternate carriers ready? Did master data support rerouting? Did customer communication happen early? Did contracts allow flexibility? A contingency plan should improve after each real event.

## Common Traps

- Calling every late shipment a disruption without identifying the underlying mechanism and affected flow.
- Expediting all orders instead of segmenting by criticality, penalty, revenue, safety, shelf life, and customer impact.
- Promising alternate routes without confirming capacity, documentation, packaging, system setup, receiving windows, and regulatory feasibility.
- Waiting for failure instead of using leading indicators and thresholds.
- Letting sales promise one recovery plan while logistics, procurement, and finance make different assumptions.
- Ignoring duties, licenses, customs, hazmat, temperature, or insurance changes when rerouting internationally.
- Tracking premium freight cost but not the root cause, decision owner, or avoided business impact.
- Treating the disruption as over when the delayed shipment delivers, without fixing the resilience gap; communicating only best-case ETAs during uncertainty

## Self-Check

- Is the disruption type, mechanism, affected lane, SKU, customer, facility, supplier, and time window clearly defined?
- Are shipments and materials segmented by business criticality before premium recovery spend is recommended?
- Are substitute, delay, cancel, allocate, expedite, and reroute options evaluated explicitly?
- Are alternate carriers, modes, ports, routes, facilities, packaging, documents, and system setup actually executable?
- Are regulatory, customs, hazmat, temperature, insurance, and customer receiving constraints checked for alternate plans?
- Are leading indicators and activation thresholds identified?
- Are decision rights clear for premium freight, allocation, substitution, customer communication, and contract exceptions?
- Does stakeholder communication separate facts, assumptions, actions, owners, and next update time?
- Are extraordinary costs and avoided impacts tracked by cause and decision?
- Is there a post-disruption review path to update contracts, inventory, routing guides, supplier plans, and data?
