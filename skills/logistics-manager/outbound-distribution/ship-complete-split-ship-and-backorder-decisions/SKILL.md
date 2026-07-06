---
name: ship-complete-split-ship-and-backorder-decisions.md
description: Use when the agent is deciding ship-complete versus split shipment, partial fulfillment, backorder handling, order allocation, customer promise changes, inventory shortages, delivery consolidation, or service-cost tradeoffs in outbound logistics.
---

# Ship Complete Split Ship And Backorder Decisions

When inventory is short or orders contain mixed availability, logistics must decide whether to hold for a complete order, split ship, substitute, cancel, or backorder. These choices affect customer experience, freight cost, warehouse labor, revenue recognition, inventory allocation, and service metrics. Agents often choose the fastest shipment path without considering customer preference, cost-to-serve, penalties, and downstream communication. This skill helps make partial-fulfillment decisions deliberately.

## Core Rules

### Start with customer and contract rules

Check whether the customer requires ship-complete, allows partials, accepts substitutions, has appointment limits, charges penalties, or needs advance notice. E-commerce, retail, wholesale, industrial, and healthcare customers may have very different rules.

Do not split ship just because inventory is available. Some customers reject partials or penalize them heavily.

### Evaluate the service and cost tradeoff

Split shipments can improve speed but increase freight cost, packaging, labor, carbon, customer contacts, and claims risk. Ship-complete can reduce cost but delay revenue and customer use. Backorders can preserve promise if communicated well.

The right answer depends on item criticality, customer value, freight economics, and delay impact.

### Define item criticality inside the order

Some items are usable alone; others are kits, accessories, installation sets, regulated components, or parts of a promotion. Shipping incomplete sets can create no value or even customer frustration.

Do not treat every line as independently shippable. Understand how the customer will use the order.

### Align with inventory allocation policy

When stock is constrained, define which customers, channels, orders, or items receive allocation. Avoid letting split-ship rules consume scarce inventory in a way that hurts higher-priority commitments.

Partial fulfillment decisions should not bypass allocation governance. The first shippable order is not always the right order.

### Coordinate warehouse execution

Split shipments and backorders create extra pick waves, pack tasks, labels, documents, cartons, holds, and exception queues. Ensure WMS and OMS status reflect each partial shipment and remaining balance accurately.

Operational complexity must be considered. A decision that looks customer-friendly may create fulfillment errors if systems cannot manage it cleanly.

### Communicate promise changes clearly

Customers need to know what shipped, what did not, why, expected backorder timing, whether additional freight charges apply, and what choices they have. Internal teams need consistent status to avoid conflicting answers.

Poor communication makes partial shipments feel like mistakes. Clear communication can preserve trust.

### Track backorders actively

Backorders need owners, expected supply date, allocation, aging, customer priority, cancellation risk, and release rules. Avoid letting aged backorders remain open after demand, product status, or customer need has changed.

Backorder is a promise, not a storage status. Manage it actively.

### Define cancellation and substitution authority

When backorders age or supply changes, decide who can cancel, substitute, downgrade, upgrade, or ask the customer to choose. Record approvals and customer preference so teams do not make inconsistent decisions.

### Measure outcome, not only ship speed and simulate policy effects before changing defaults

Track split frequency, incremental freight, customer satisfaction, cancellations, backorder age, fill rate, contacts, claims, and margin impact. Use data to tune policies.

Fast partial shipments are not always better if they create cost, confusion, and repeat contacts.

Before changing split-ship or ship-complete rules, model expected impact on freight cost, packages per order, delivery speed, backorder age, warehouse labor, packaging use, customer contacts, and allocation fairness. Test by segment if possible.

Policy defaults affect thousands of orders. A small rule change can shift cost and workload across the network.

## Common Traps

- Splitting shipments without checking customer contract, routing guide, penalty, and appointment rules.
- Holding complete orders without understanding item criticality and customer urgency.
- Shipping accessories or partial kits that the customer cannot use.
- Letting partial shipments consume constrained inventory outside allocation policy.
- Creating WMS/OMS status confusion between shipped lines and remaining balances.
- Communicating shipment status without explaining backorder timing and customer options; leaving aged backorders open without owner, priority, and cancellation review
- Canceling, substituting, upgrading, or downgrading backorders without authority and customer preference records; measuring success by ship speed while ignoring cost, contacts, claims, and satisfaction
- Changing split-ship defaults without modeling freight, labor, packaging, contact, and allocation impact; applying one split-ship rule to every customer, channel, and product type

## Self-Check

- Are customer rules known for ship-complete, partials, substitutions, appointments, penalties, and notice?
- Is the tradeoff among speed, freight cost, labor, packaging, carbon, revenue, and customer impact evaluated?
- Are items classified by whether they are independently useful, kit-linked, accessory-linked, regulated, or promotion-linked?
- Do partial decisions respect inventory allocation, customer priority, channel rules, and scarce-stock governance?
- Can WMS and OMS accurately track partials, remaining balances, holds, labels, documents, and release status?
- Are customers told what shipped, what is backordered, why, when, cost impact, and available choices?
- Are backorders owned, aged, prioritized, allocated, and reviewed for cancellation or promise changes?
- Are cancellation, substitution, downgrade, upgrade, and customer-choice authorities defined and recorded?
- Are split frequency, incremental freight, contacts, claims, cancellations, satisfaction, and margin measured?
- Are policy changes modeled for freight cost, package count, labor, backorder aging, contacts, and allocation fairness?; does the policy balance customer service with operational complexity and cost-to-serve?
