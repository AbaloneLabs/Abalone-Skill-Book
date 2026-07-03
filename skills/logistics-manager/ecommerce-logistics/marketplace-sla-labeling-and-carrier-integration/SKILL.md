---
name: marketplace_sla_labeling_and_carrier_integration.md
description: Use when the agent is managing marketplace fulfillment, retailer or marketplace SLAs, shipping labels, carrier integrations, scan compliance, routing rules, marketplace penalties, EDI/API order flow, label service codes, cancellations, late shipment defects, or marketplace customer promise risk.
---

# Marketplace SLA Labeling And Carrier Integration

Marketplace logistics is governed by platform rules as much as warehouse capability. Amazon, Walmart, eBay, retail marketplaces, delivery apps, and seller portals may define ship-by times, valid tracking, cancellation rules, label formats, carrier eligibility, delivery promises, defect metrics, and penalties. Agents often plan marketplace fulfillment like ordinary ecommerce and miss that one wrong service code, late scan, or API failure can damage account health. This skill helps manage marketplace shipping as an integration and compliance surface.

## Core Rules

### Read The Marketplace Promise As A Contract

Marketplace orders come with rules: ship-by date, deliver-by date, valid tracking upload, approved carriers, cancellation reason codes, response deadlines, late shipment metrics, on-time delivery, pre-fulfillment cancel rate, seller-fulfilled prime standards, routing guides, or chargeback terms. These rules may change.

Do not rely on generic ecommerce assumptions. Identify the current marketplace policy, seller account settings, carrier mapping, holiday handling, and cutoff logic. When exact rules affect risk, verify current platform documentation or seller portal information.

### Align Warehouse Cutoffs With Platform Clocks

Marketplace clocks may be based on local time zone, customer promise, order import time, business days, weekends, holidays, or seller handling time. The warehouse may see orders after payment, fraud, tax, or API processing. If platform and WMS clocks differ, the operation can be late before work starts.

Map order import, acknowledgment, pick release, label purchase, tracking upload, carrier scan, and shipment confirmation. Build alerts before SLA breach. Avoid batching marketplace orders in a way that misses platform deadlines.

### Control Label Service Codes And Carrier Eligibility

Marketplace labels and carrier integrations must use correct service codes, package dimensions, weight, ship-from address, return address, hazmat flags, signature requirements, marketplace order IDs, and tracking upload. Wrong service mapping can produce invalid tracking, late delivery, extra charges, or platform defects.

Test carrier integrations after rate changes, software updates, warehouse moves, and new services. Confirm that labels scan, manifests close, tracking uploads, and carrier events feed back to the marketplace. Do not assume a printed label means platform compliance.

### Maintain Tracking And First-Scan Discipline

Marketplaces often care about valid tracking and carrier acceptance scans. If the warehouse marks shipped but the package misses pickup or the carrier fails to scan, the order can appear late or fraudulent. Trailer consolidation, end-of-day pickup, and weekend handling can create scan gaps.

Track label creation, manifest, physical handoff, first carrier scan, and marketplace upload. For high-risk windows, reconcile unscanned labels before cutoff. Escalate repeated carrier scan misses with evidence.

### Handle Cancellations, Substitutions, And Stockouts Carefully

Marketplace cancellation reasons can affect metrics. Canceling due to out of stock, address issue, buyer request, fraud, or restricted item may have different consequences. Substituting products without buyer and platform compliance can create claims and account risk.

Define who can cancel, which reason codes apply, when to contact the buyer, and how inventory errors are corrected. Do not hide stockouts with fake shipment confirmations or unapproved substitutions. Short-term metric protection can create larger account risk.

### Protect Data And Integration Reliability

Marketplace fulfillment depends on APIs, EDI, order import jobs, label services, carrier rating, inventory feeds, and tracking uploads. Integration outages can create silent backlog. Duplicate imports, missing orders, bad addresses, stale inventory, and failed confirmations must be detected quickly.

Monitor integration queues, errors, retry behavior, and reconciliation between marketplace, OMS, WMS, TMS, and carrier systems. Define manual fallback for order export, label generation, and shipment confirmation if systems fail. Manual fallback should protect data accuracy, not create duplicate shipments.

### Segment Marketplace Orders Operationally

Marketplace orders may need special handling: branded packing restrictions, marketplace inserts rules, neutral packaging, serial capture, hazmat restrictions, gift options, return labels, customer messaging limits, and shipping method constraints. Mixing them with DTC orders without controls can violate platform rules.

Use channel-specific rules for packing, labeling, documentation, communication, and returns. Train packers on differences that matter. If the marketplace prohibits marketing inserts or requires specific return process, the pack station should not rely on memory.

### Review Account Health As An Operations Metric

Late shipment, valid tracking, cancellation, return, refund, complaint, and on-time delivery metrics are operational signals. They should be reviewed by SKU, warehouse, carrier, software, and cutoff window. Marketplace penalties can include listing suppression, buy box loss, seller suspension, and chargebacks.

Treat account-health risk as a logistics KPI. If one carrier or service produces defects, adjust mapping or promise settings. If one SKU causes cancellations, fix inventory data or allocation.

## Common Traps

### Assuming Marketplace Orders Follow DTC Rules

Platform clocks, tracking, cancellation, and messaging rules can be stricter.

### Printing Labels Without Checking Service Mapping

Wrong carrier service codes can create invalid tracking or late delivery.

### Marking Orders Shipped Before Physical Handoff

Scan gaps can damage account health even if the package leaves later.

### Letting API Failures Hide Backlog

Order import or tracking upload failures need monitoring and reconciliation.

### Using Wrong Cancellation Reasons

Reason codes affect metrics and can misrepresent operational problems.

### Mixing Inserts And Branding Across Channels

Marketplace rules may prohibit marketing materials or require neutral packaging.

### Treating Account Health As A Sales Problem Only

Many defects are logistics process failures.

### Ignoring Marketplace Rule Changes

Handling-time, carrier, tracking, and holiday rules can change and need current verification.

## Self-Check

- [ ] Are current marketplace SLAs, handling time, tracking, carrier, cancellation, and penalty rules identified?
- [ ] Are platform clocks aligned with order import, release, pick, pack, label, confirmation, and pickup timing?
- [ ] Are label service codes, dimensions, weights, hazmat flags, ship-from, return address, and order IDs correct?
- [ ] Are manifest, first scan, tracking upload, and marketplace confirmation reconciled?
- [ ] Are cancellations, stockouts, substitutions, and buyer communications handled by approved rules?
- [ ] Are API/EDI/order import, label, inventory, and tracking failures monitored with fallback procedures?
- [ ] Are marketplace packing, inserts, branding, return, serial, and messaging rules separated from DTC rules?
- [ ] Are account-health metrics reviewed by warehouse, carrier, SKU, service, software, and cutoff window?
- [ ] Are current platform rules verified when exact compliance affects risk?
- [ ] Would a software outage, carrier scan miss, or stockout produce a compliant response?
