---
name: dimensions-weight-hazmat-and-country-of-origin-data.md
description: Use when the agent is managing logistics master data for item dimensions, weight, cube, cartonization, hazmat classification, lithium batteries, country of origin, customs data, tariff attributes, or compliance-critical product data.
---

# Dimensions Weight Hazmat And Country Of Origin Data

Product logistics data determines how items move, rate, pack, comply, clear customs, and fit automation. Wrong dimensions, weight, hazmat flags, battery status, country of origin, or tariff attributes can cause freight overcharges, carrier rejections, customs delays, safety violations, and warehouse failures. Agents often treat these fields as static attributes and miss how they drive operational decisions. This skill helps govern high-impact product data.

## Core Rules

### Measure physical data from controlled sources

Dimensions, weight, cube, case pack, pallet pattern, stackability, conveyability, and packaging levels should come from measured product, approved supplier data, engineering data, or certified item setup. Distinguish eaches, inner packs, cases, pallets, and shipping cartons.

Do not copy dimensions casually from marketing pages. Logistics needs the physical unit that systems will handle.

### Use data for the correct packaging level

An item may have selling unit, each, inner, master carton, pallet, kit, and shipping configuration. Confirm which level drives storage, cartonization, carrier rating, customs, automation, and customer display.

Many errors come from putting case dimensions in each fields or selling-unit weight in shipping carton fields. Define levels clearly.

### Control hazmat and battery classification

Hazmat, dangerous goods, lithium batteries, aerosols, chemicals, magnets, flammables, and regulated materials require classification, UN numbers, packing group, quantity limits, mode restrictions, labels, SDS, and carrier eligibility. Use qualified compliance sources.

Do not let operations guess whether a product is regulated. Wrong flags can create safety incidents or carrier violations.

### Maintain country of origin and customs attributes

Country of origin, HTS/HS code, ECCN or export controls, trade preference eligibility, valuation attributes, and product descriptions affect customs clearance and duty. Changes in sourcing or bill of material may change origin.

Origin data is not a one-time setup. It must follow sourcing, supplier, and product changes.

### Validate data against operational outcomes

Use freight audit, carrier dimensional weight, cartonization failures, automation jams, pick exceptions, customs holds, and chargebacks to identify bad data. Data quality should be measured by process performance.

If a field is rarely checked but frequently causes exceptions, it needs a governance control.

### Manage supplier data onboarding

Suppliers should provide data in required format with evidence, effective dates, packaging levels, compliance documents, and change notification rules. Validate samples and reject incomplete records before item launch.

Supplier-provided data should not flow automatically into production systems without review where risk is high.

### Audit high-risk data periodically

Use physical remeasurement, carrier invoice sampling, hazmat document checks, customs entry review, and warehouse exception sampling for high-volume, high-cost, regulated, or frequently disputed items. Prioritize items with repeated adjustments or unexplained charges.

Data quality degrades as packaging, suppliers, and regulations change. Periodic audits catch errors that normal transaction processing may work around silently.

### Handle changes and replacements carefully

New packaging, bundle changes, kit changes, supplier changes, battery changes, formulation changes, or country-of-origin changes may require data updates and downstream retesting. Link product change control to logistics data review.

The system may continue using old data long after the physical product changed unless ownership is explicit.

### Monitor financial and compliance exposure

Bad dimensions cause parcel surcharges, LTL reclasses, storage errors, and carton waste. Bad hazmat or origin data creates fines, holds, denied shipments, and customer issues. Quantify exposure to prioritize cleanup.

Data cleanup competes with daily work. Use risk and cost to focus effort.

## Common Traps

- Copying dimensions and weights from marketing or supplier text without verifying packaging level.
- Mixing each, inner, case, pallet, kit, and shipping carton attributes.
- Treating hazmat, battery, aerosol, chemical, magnet, and regulated flags as optional.
- Letting country of origin and customs codes remain unchanged after sourcing changes.
- Fixing one shipment manually while leaving bad item data in the master.
- Accepting supplier data without evidence, format controls, or change notification.
- Failing to re-audit high-risk dimensions, weights, hazmat flags, and origin data after changes and disputes.
- Ignoring carrier dimensional-weight charges, cartonization failures, and automation jams as data-quality signals.
- Updating physical packaging without retesting rating, storage, automation, and compliance workflows.
- Prioritizing easy data cleanup instead of high-risk financial and compliance exposure.

## Self-Check

- Are dimensions, weight, cube, case pack, pallet, stackability, and conveyability measured or source-validated?
- Are each, inner, case, pallet, kit, selling unit, and shipping carton levels clearly separated?
- Are hazmat, dangerous goods, lithium battery, aerosol, chemical, magnet, UN number, packing, and carrier rules controlled?
- Are country of origin, HS/HTS, export control, trade preference, valuation, and customs descriptions current?
- Are freight charges, cartonization failures, automation jams, customs holds, and chargebacks used to detect bad data?
- Are supplier data requirements, evidence, format, effective dates, and change notification defined?
- Are high-volume, regulated, costly, or disputed items periodically remeasured and audited against operational evidence?
- Do product, supplier, packaging, formulation, battery, bundle, and kit changes trigger logistics data review?
- Is financial and compliance exposure quantified to prioritize cleanup?
- Can operations trust product data for rating, packing, automation, storage, compliance, and customs?
