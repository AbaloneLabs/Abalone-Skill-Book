---
name: freight_routing_tendering_and_dispatch.md
description: Use when the agent is routing freight, tendering loads to carriers, dispatching shipments, managing routing guides, load acceptance, appointment timing, carrier fallbacks, shipment visibility, and daily transportation execution across truckload, LTL, parcel, intermodal, drayage, or final-mile operations.
---

# Freight Routing Tendering And Dispatch

Freight routing and tendering turn transportation plans into live commitments. This is where rates, service promises, carrier capacity, appointment windows, inventory readiness, and customer expectations meet operational reality. Agents often describe routing as choosing a carrier, but daily execution requires checking load readiness, tender sequence, carrier acceptance, pickup timing, documentation, tracking, fallback rules, and exception response. Weak tendering creates late pickups, premium freight, detention, missed appointments, and customer service noise. Strong dispatch discipline makes freight move predictably and exposes exceptions early.

Use this skill when planning transportation execution, creating routing rules, tendering shipments, dispatching loads, managing carrier fallbacks, or reviewing shipment status.

## Core Rules

### Confirm Shipment Readiness Before Tender

Do not tender freight that is not operationally ready or realistically ready by pickup. Confirm inventory availability, order release, pick and pack status, pallet count, weight, cube, dimensions, temperature or hazmat requirements, paperwork, labels, customer routing guide, appointment needs, and ship-from dock capacity.

Tendering too early can create carrier delays, detention, and distrust. Tendering too late reduces carrier options. The planner should understand the readiness signal and cutoff for each mode. For dynamic operations, use status gates so transportation does not book capacity against phantom freight.

### Use A Governed Routing Guide

A routing guide should define carrier sequence by lane, mode, service level, cost, capacity, customer requirement, and performance. It should also define when to skip a carrier, when to use spot market, when to expedite, and who approves exceptions. Without governance, dispatchers may choose familiar carriers, cheapest rates, or premium options inconsistently.

Keep routing guides current with rates, service days, tender acceptance, claims, accessorials, insurance, customer restrictions, and carrier capacity. A stale guide creates false confidence and slows tendering when carriers reject loads.

### Tender With Enough Lead Time And Complete Information

Carriers need accurate pickup and delivery locations, dates, appointment windows, freight description, weight, pieces, dimensions, equipment, temperature, hazmat details, value, special services, contact information, and reference numbers. Missing or wrong information causes rework, rejected tenders, accessorials, and service failures.

Lead time matters by mode and lane. Truckload, intermodal, air, drayage, and final-mile appointments may require different booking horizons. For tight deadlines, communicate urgency and constraints clearly rather than assuming the carrier can infer them.

### Monitor Acceptance And Rejection Patterns

Tender acceptance is an early capacity signal. Rejections may indicate rate issues, lane imbalance, seasonality, weather, carrier dissatisfaction, poor facility performance, or unrealistic lead time. Track rejection reasons and time to cover.

Do not treat each rejection as isolated. If primary carriers reject a lane repeatedly, review rates, tender lead time, pickup experience, dwell, payment, forecast accuracy, or routing guide design. Chronic rejection is a network issue, not only a dispatcher problem.

### Protect Appointment Discipline

Many shipments depend on pickup, delivery, port, rail, warehouse, retail, or job-site appointments. Dispatch should verify appointment requirements, booking responsibility, confirmation numbers, time zones, check-in rules, detention terms, and rescheduling process. Missed appointments can trigger chargebacks, storage, production delays, or customer penalties.

Coordinate appointment timing with warehouse readiness and transit reality. A delivery appointment that assumes perfect transit and no loading delay is fragile. Build buffers for high-risk lanes and communicate proactively when appointments are at risk.

### Define Fallback And Escalation Rules

When a carrier rejects, a driver is late, freight is not ready, weather disrupts a lane, or an appointment is missed, dispatchers need a rule set. Define fallback carriers, spot market thresholds, expedite approval, customer notification triggers, mode changes, hold decisions, and escalation owners.

Fallback rules should protect service and cost. Not every late shipment deserves premium freight. Prioritize by customer tier, contractual penalty, product criticality, promised date, margin, and recovery options. Document why premium choices were made so root causes can be addressed later.

### Maintain Shipment Visibility And Documentation

Dispatch execution requires visibility: tender status, carrier assignment, driver details, pickup confirmation, in-transit updates, exception notes, proof of delivery, temperature logs, seal numbers, customs documents, bills of lading, and delivery receipts. The level of tracking should match shipment risk and customer expectation.

Visibility gaps become customer service problems. If a carrier cannot provide required tracking or documents, that limitation should affect carrier selection for high-risk freight. Ensure systems, emails, portals, and documents align so teams do not chase conflicting status.

### Close The Loop After Execution

Transportation execution should feed continuous improvement. Review on-time pickup, on-time delivery, tender acceptance, cost variance, accessorials, dwell, claims, missed appointments, and exception reasons. Tie issues back to root causes: warehouse readiness, routing guide, carrier performance, customer receiving, forecast accuracy, or documentation quality.

Daily dispatch is not only firefighting. It is an evidence source for procurement, network design, warehouse operations, and customer policy.

## Common Traps

### Tendering Freight That Is Not Ready

Booking carriers before freight is picked, packed, staged, or documented creates detention and missed pickups. Readiness gates protect execution.

### Using A Stale Routing Guide

Rates, carriers, service days, insurance, and lane capacity change. A stale guide causes repeated rejection and hidden spot-market cost.

### Treating Rejections As Random

Repeated carrier rejections usually have a cause: price, timing, dwell, lane imbalance, facility reputation, or poor forecast. Investigate patterns.

### Missing Appointment Details

Wrong time zones, confirmation numbers, receiving rules, or rescheduling requirements can derail an otherwise covered load.

### Escalating Too Quickly To Premium Freight

Expedite can protect service, but unmanaged premium freight hides planning failures and drains margin. Use approval rules and reason codes.

### Relying On Verbal Status

Verbal updates without system status, pickup proof, or delivery documentation create disputes and poor visibility. Capture evidence.

### Ignoring Facility Experience

Carriers avoid facilities with long dwell, poor communication, or difficult check-in. Dispatch performance depends partly on warehouse discipline.

### Failing To Learn From Exceptions

If exceptions are resolved but not categorized, the same problems repeat. Close the loop into routing, carrier management, and operations.

## Self-Check

- [ ] Is shipment readiness confirmed before tendering, including inventory, packing, staging, documents, and dock capacity?
- [ ] Is the routing guide current and governed by lane, mode, service, carrier performance, cost, and exception rules?
- [ ] Are tenders sent with complete freight, equipment, appointment, contact, reference, and special handling details?
- [ ] Are tender acceptance, rejection reasons, and time-to-cover monitored for patterns?
- [ ] Are appointment requirements, confirmation numbers, time zones, and rescheduling rules managed?
- [ ] Are fallback carriers, spot thresholds, expedite approvals, and customer notification triggers defined?
- [ ] Is shipment visibility appropriate for customer promise and shipment risk?
- [ ] Are proofs of pickup, delivery, temperature, seal, BOL, and other documents captured where required?
- [ ] Are premium freight decisions documented with reason codes?
- [ ] Are execution exceptions reviewed for root causes beyond the immediate shipment?
