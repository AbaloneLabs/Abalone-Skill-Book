---
name: item-location-carrier-and-rate-master-data-governance.md
description: Use when the agent is managing item master, location master, carrier master, service codes, routing data, freight rates, calendars, lead times, facility setup, or logistics master data governance across systems.
---

# Item Location Carrier And Rate Master Data Governance

Item, location, carrier, and rate master data controls the daily behavior of logistics systems. It decides where inventory can live, which carrier is selected, what service is available, what cost is charged, which calendar applies, and what promise is made. Agents often talk about data quality generically and miss the specific master-data objects that drive failures. This skill helps govern the records that logistics systems depend on.

## Core Rules

### Define critical master-data objects

Identify which objects drive logistics execution: item, SKU, UOM, location, zone, facility, customer, supplier, carrier, service code, route, rate, calendar, lead time, cutoff, dock, equipment, and handling rule. Map which systems use each object.

Governance is weak when teams do not know which records are critical. Start by naming the operational objects.

### Establish one source of truth where possible

Decide which system owns each data element and which systems consume it: ERP, WMS, TMS, OMS, rate engine, carrier portal, yard system, customs system, or data warehouse. Define synchronization rules and error handling.

Conflicting records create unpredictable execution. A carrier service active in TMS but inactive in ERP can break billing or shipping.

### Control item and unit-of-measure setup

Items need SKU, description, UOM conversion, pack hierarchy, dimensions, weight, handling flags, temperature, hazmat, expiration, serialization, lot control, and replenishment attributes. UOM errors can multiply inventory and freight mistakes.

Do not launch items until logistics fields are complete enough for receiving, storage, picking, packing, shipping, and returns.

### Govern location and facility data

Locations need address, geocode, timezone, operating hours, calendar, dock rules, carrier access, delivery restrictions, contact, storage type, capacity, and system status. Facility records should distinguish ship-from, ship-to, return, cross-dock, and billing roles.

Bad location data creates failed appointments, wrong routing, late deliveries, and tax or freight errors.

### Govern carrier, service, and rate setup

Carrier records need legal name, SCAC or account, service codes, mode, lane eligibility, contract terms, fuel, accessorials, insurance, payment terms, EDI/API setup, and effective dates. Rates need version control and expiration.

Do not let old rates, duplicate carrier records, or inactive services remain available for planning and payment.

### Manage calendars, cutoffs, and lead times

Shipping calendars, receiving calendars, holidays, carrier pickup times, customer delivery windows, transit times, and order cutoffs directly affect customer promises. Keep them current and tied to operational reality.

Outdated calendars are a common cause of impossible promises. They are master data, not scheduling trivia.

### Audit data quality by exception patterns

Monitor failed labels, rating errors, tender rejects, appointment failures, ASN rejects, inventory discrepancies, address corrections, carrier invoices, and customer complaints. These errors often point to master-data defects.

Data governance should prioritize records that create execution failures, not only fields with blanks.

### Deactivate and archive deliberately

Old locations, carriers, services, items, rates, and customer records should be deactivated without breaking history, returns, claims, and reporting. Define when records can no longer be used for new activity.

Leaving obsolete records active creates wrong choices; deleting records carelessly breaks traceability.

### Use stewardship routines and data health metrics

Create recurring reviews for duplicate records, inactive-but-used records, blank critical fields, failed synchronizations, expiring rates, invalid addresses, missing calendars, and records with repeated exceptions. Assign data stewards and due dates for cleanup.

Master data improves when health is visible. Ad hoc cleanup after outages is less effective than a steady governance rhythm that catches deterioration early.

## Common Traps

- Discussing data quality without naming the logistics records that drive execution.
- Letting ERP, WMS, TMS, OMS, rate engine, and carrier portal disagree on active values.
- Launching items without complete UOM, dimensions, handling, hazmat, lot, and returns attributes.
- Using inaccurate geocodes, time zones, dock rules, contacts, and facility roles.
- Allowing duplicate carriers, expired rates, inactive services, and wrong fuel tables to remain selectable.
- Treating calendars, holidays, transit times, and cutoffs as informal planner knowledge.
- Cleaning blank fields while ignoring exception patterns that reveal high-impact bad data.
- Deleting obsolete records instead of deactivating them with history preserved.
- Waiting for outages before reviewing duplicates, blanks, failed syncs, expiring rates, and invalid addresses.
- Failing to test downstream impacts after master-data updates.

## Self-Check

- Are critical item, location, carrier, service, rate, calendar, lead time, cutoff, and handling records identified?
- Is source-of-truth ownership defined across ERP, WMS, TMS, OMS, rate engine, carrier, yard, customs, and reporting systems?
- Are item SKU, UOM, pack, dimensions, handling, temperature, hazmat, serialization, lot, and return attributes complete?
- Are location addresses, geocodes, time zones, hours, dock rules, contacts, capacity, roles, and restrictions current?
- Are carrier legal name, account, SCAC, mode, service, lane, rates, fuel, accessorials, terms, EDI/API, and effective dates governed?
- Are calendars, holidays, pickup times, delivery windows, transit times, and order cutoffs maintained as master data?
- Are label failures, rating errors, tender rejects, appointment failures, ASN rejects, invoice issues, and complaints used for audit?
- Are obsolete records deactivated without breaking history, returns, claims, and reporting?
- Are data stewards reviewing duplicates, blanks, failed syncs, expiring rates, invalid addresses, missing calendars, and repeated exceptions?
- Can logistics systems execute consistently from shared, current, and tested master data?
