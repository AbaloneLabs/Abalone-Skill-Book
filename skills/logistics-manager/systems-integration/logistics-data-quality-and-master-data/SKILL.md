---
name: logistics-data-quality-and-master-data.md
description: Use when the agent is assessing logistics master data, item dimensions and weights, packaging data, carrier service codes, addresses, lead times, routing guides, location records, supplier data, data governance, or operational errors caused by poor logistics data.
---

# Logistics Data Quality And Master Data

Logistics data is physical reality written into systems. If dimensions, weights, addresses, service codes, handling flags, packaging rules, lead times, or location records are wrong, the operation pays through misrated freight, failed labels, bad slotting, stockouts, chargebacks, damaged goods, customs holds, and missed promises. Agents often treat data cleanup as clerical work instead of an operational control. This skill helps the agent identify which data matters, who owns it, and how to prevent bad data from becoming daily firefighting.

## Core Rules

### Prioritize Data By Operational Consequence

Not all data fields carry equal risk. Start with fields that drive physical flow, cost, compliance, and customer promises: SKU, description, barcode, unit of measure, case pack, each/case/pallet hierarchy, length, width, height, weight, cube, stackability, storage temperature, hazmat flag, lot or serial control, expiration, country of origin, tariff code, packaging type, supplier pack, lead time, ship-from, ship-to, carrier service, routing guide, delivery window, and customer compliance rules.

Rank data issues by impact. A typo in a long description may be tolerable; wrong weight can misrate freight, wrong barcode can stop picking, wrong origin can affect customs, and wrong case pack can corrupt inventory. Use risk-based cleanup rather than trying to perfect every field equally.

### Tie Data Ownership To Process Ownership

Master data should have named owners and change controls. Product teams may know item composition, procurement may know supplier pack, warehouse engineering may know storage media, transportation may know carrier service codes, trade compliance may own tariff and origin, finance may own cost and tax data, and customer service may own promise language. If no one owns a field, it will drift.

Define who can create, approve, change, and retire records. New-item setup, supplier change, packaging change, customer onboarding, carrier onboarding, facility move, and system migration all need data gates. A logistics team should not discover a new SKU's dimensions during the first wave of live orders.

### Measure Physical Data, Do Not Guess It

Dimensions, weights, cartonization rules, pallet patterns, and packaging instructions should come from measured or validated sources. Supplier-provided data can be wrong, especially after packaging changes. System defaults are dangerous. For freight rating, storage design, automation, parcel surcharges, and truckload utilization, small measurement errors can create large cost and service impacts.

Establish measurement standards: each versus inner pack versus case, net versus gross weight, maximum versus nominal dimension, packaging included or excluded, pallet overhang, dimensional rounding, and unit conversions. Without standards, two teams can both be "accurate" in incompatible ways.

### Validate Addresses, Locations, And Routing Rules

Address quality affects labels, carrier acceptance, delivery success, taxes, routing, transit time, accessorials, and customer communication. Validate ship-to, bill-to, return, supplier, facility, dock, store, and cross-dock addresses. Include contact names, phone numbers, delivery windows, appointment requirements, liftgate needs, residential/commercial status, security gate procedures, and receiving constraints where relevant.

Location master data inside facilities also matters: bins, zones, temperature areas, hazmat areas, pick faces, replenishment locations, dock doors, staging lanes, yard slots, and virtual locations. If location data does not match physical layout, WMS behavior will create travel waste, inventory errors, and unsafe workarounds.

### Connect Data Quality To Cost And Service Metrics

Data defects should be visible in operational metrics: freight re-rate, dimensional surcharge, address correction fee, retailer chargeback, pick exception, cycle count adjustment, stockout, misship, customs hold, label failure, delivery exception, rejected ASN, or manual order touch. Treat these as data-quality signals, not isolated operational mistakes.

Build feedback loops. When a carrier invoice reveals a weight variance, when a warehouse corrects dimensions, when customs queries origin, or when a customer rejects an ASN, feed the correction into master data with approval and audit trail. Repeated manual fixes are evidence that governance is weak.

### Govern Change Without Freezing The Operation

Change controls should prevent bad changes, not make needed corrections impossible. Define emergency correction paths for shipment-blocking data, and normal review paths for lower-risk changes. Test changes that affect integrations, labels, rates, routing, replenishment, automation, or compliance before they hit production.

Retire obsolete data. Old carrier services, closed facilities, inactive customers, outdated routing guides, discontinued SKUs, and duplicate addresses create selection errors. Data governance includes cleanup as well as creation.

## Common Traps

- Treating logistics master data as administrative housekeeping rather than a driver of cost, compliance, and service.
- Cleaning all fields equally instead of prioritizing fields with physical, financial, or regulatory impact.
- Guessing dimensions and weights from similar items or supplier catalogs without measurement standards.
- Mixing each, case, inner pack, pallet, net weight, gross weight, and dimensional weight.
- Allowing product, procurement, warehouse, transportation, trade, and finance teams to change related fields without ownership rules.
- Validating street addresses but ignoring delivery windows, dock constraints, appointment rules, liftgate needs, and contacts.
- Fixing shipment exceptions manually without correcting the master data that caused them.
- Letting obsolete service codes, locations, SKUs, and routing rules remain selectable; migrating data into a new system without cleansing and field definition alignment

## Self-Check

- Are high-impact logistics fields identified and prioritized by cost, service, compliance, and physical flow risk?
- Does each critical field have an owner, approver, change process, and audit trail?
- Are dimensions, weights, packaging hierarchy, and pallet data measured or validated with clear standards?
- Are addresses and delivery profiles validated for carrier acceptance, appointments, accessorials, contacts, and receiving constraints?
- Does facility location data match the physical layout and operational rules?
- Are trade, hazmat, temperature, lot, serial, expiration, and compliance fields owned by qualified teams?
- Are data defects linked to metrics such as re-rates, chargebacks, pick exceptions, customs holds, label failures, and delivery exceptions?
- Is there a feedback loop from invoice variances, warehouse corrections, customer rejects, and border queries into master data?
- Are emergency corrections possible without bypassing governance entirely?
- Are obsolete records, duplicate addresses, inactive services, and outdated routing rules retired?
