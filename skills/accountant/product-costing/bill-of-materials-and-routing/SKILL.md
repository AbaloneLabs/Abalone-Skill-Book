---
name: bill_of_materials_and_routing.md
description: Use when the agent is building or reviewing a bill of materials, defining manufacturing routings, linking components and operations to a product, setting up standard costs, or checking whether product costing master data reflects how a product is actually made.
---

# Bill Of Materials And Routing

A bill of materials (BOM) and a routing are the master data that tell a costing system what a product is made of and how it is made. They look like simple lists, but they drive inventory valuation, standard costs, variance analysis, cost of goods sold, and margin reporting. When a BOM is missing a component, when a routing omits an operation, or when quantities and times are wrong, the resulting product cost is wrong in a way that is hard to detect because every report ties back to the same flawed master data.

Use this skill before creating or revising a BOM, defining routings, setting up manufactured item master records, validating standard cost rolls, or troubleshooting why a product cost looks incorrect. The goal is to prevent the agent from building costing master data that is internally consistent but disconnected from how the product is actually produced.

## Core Rules

### Treat The BOM As A Costing And Operational Contract

A BOM is not a suggestion. It defines the components, quantities, and structure that the costing system will assume every unit consumes. Treat it as a contract between engineering, production, and accounting.

Confirm for each manufactured item:

- every material component is listed with the correct quantity per unit;
- units of measure are consistent between the component and the parent item;
- the BOM structure, single-level or multi-level, matches the production process;
- phantom, subassembly, and co-product or by-product relationships are modeled correctly;
- scrap, yield, and shrinkage factors are applied where the process warrants them;
- effective dates reflect engineering changes and version control.

A BOM that omits a real component understates cost. A BOM that includes obsolete components overstates it. Both errors propagate into inventory and margin reporting.

### Define Routings That Reflect The Real Sequence Of Operations

A routing defines the operations, work centers, sequence, and time required to make a product. It drives labor and overhead absorption into the standard cost.

For each routing verify:

- every operation performed is represented, in the correct sequence;
- work centers are mapped to the correct cost centers and rates;
- setup time, run time, and queue or move time are captured where material;
- standard times reflect current process capability, not historical guesses;
- alternate routings are documented and their use controlled;
- operations that are outsourced are flagged as subcontracted with correct cost treatment.

A routing that skips an operation understates conversion cost. A routing with inflated times overstates cost and hides inefficiency as favorable variances.

### Reconcile BOM And Routing To Physical Reality

Master data drifts. Engineering changes, process improvements, supplier substitutions, and equipment changes all make yesterday's BOM and routing wrong today.

Reconcile periodically by:

- walking the production floor and comparing actual process to the routing;
- reviewing engineering change orders and confirming they updated the BOM and routing;
- comparing issued material quantities to BOM quantities and investigating large gaps;
- reviewing standard versus actual operation times;
- confirming subcontracted operations and their pricing.

When the costing system says a product costs more or less than expected, the first place to look is usually the BOM and routing, not the accounting entries.

### Handle Scrap, Yield, And By-Products Defensibly

Production processes generate scrap, yield loss, and sometimes by-products or co-products. How these are modeled changes product cost.

Address:

- planned scrap or yield loss built into component quantities;
- the difference between planned and abnormal scrap, with abnormal scrap treated as period cost under the applicable framework;
- by-product revenue credited against product cost or recognized separately, consistent with policy;
- co-product cost allocation using a defensible basis such as sales value or physical measure.

Document the method. Changing scrap or yield assumptions alters reported cost even when nothing operational changed, so disclose and quantify such changes.

### Maintain Effective Dating And Version Control

Products evolve. A BOM or routing without effective dates applies the same structure to all periods, which corrupts historical cost analysis and variance reporting.

Maintain:

- effective start and end dates for each BOM and routing version;
- engineering change order linkage so each version is traceable to its authorization;
- archived versions so historical costs can be reproduced;
- controlled change procedures so production and accounting update together.

Uncontrolled BOM and routing changes are a common source of unexplained cost swings and audit findings.

### Reconcile Standard Cost To The General Ledger

The standard cost built from BOM and routing must reconcile to the inventory and cost of goods sold balances in the ledger. A disconnect means the master data and the accounting system disagree.

Reconcile:

- the standard cost roll output to inventory valuation;
- material price and usage variances to purchasing and production activity;
- labor and overhead rate and efficiency variances to payroll and cost center activity;
- revaluation of inventory when standards change.

If variances are persistently large or one-directional, the BOM, routing, or standard rates are likely wrong and should be investigated, not absorbed silently.

### Respect Framework And Jurisdictional Limits

Inventory costing rules differ by reporting framework and jurisdiction. What counts as an inventoriable cost, how overhead is absorbed, how scrap and idle capacity are treated, and whether standard costing is permitted for external reporting all depend on the applicable standard. Confirm the BOM and routing design, and the resulting standard cost, comply with the relevant GAAP, IFRS, tax, or regulatory requirements before relying on them externally, and escalate uncertain treatments to a qualified professional.

## Common Traps

### Omitting Low-Value Or Support Components

Small components, packaging, adhesives, and consumables are easy to leave out of a BOM. Aggregated across volume, they materially understate cost.

### Using Stale Standard Times

Routings built years ago no longer reflect current cycle times. Stale times produce meaningless variances and hide real efficiency gains or losses.

### Ignoring Engineering Changes

When engineering changes are not reflected in the BOM or routing, the costing system prices a product that no longer exists.

### Misapplying Scrap And Yield Factors

Applying scrap at the wrong level of the BOM, or double-counting yield loss, distorts component quantities and product cost.

### Confusing Phantom And Real Subassemblies

Modeling a subassembly as a real item when it is a phantom creates unnecessary inventory layers and revaluation complexity. Modeling a real item as a phantom loses inventory visibility.

### Changing Standards Without Disclosing The Effect

Updating BOM, routing, or rates changes standard cost and inventory valuation. Failing to quantify and disclose the effect distorts period-over-period comparisons.

### Assuming Standard Cost Equals Reporting Cost

Standard costing is a management and inventory tool. Whether the resulting cost is acceptable for external reporting, tax, or regulated purposes depends on the framework and must be confirmed.

## Self-Check

- Does the BOM list every material component with correct quantities, units, and structure?
- Does the routing represent every operation, work center, sequence, and material time, including setup and subcontracted steps?
- Have the BOM and routing been reconciled to the physical process through floor walks, engineering change review, and material issuance comparison?
- Are scrap, yield, by-products, and co-products modeled with a documented and defensible method?
- Are effective dates and version control in place so historical costs can be reproduced?
- Does the standard cost built from BOM and routing reconcile to inventory and cost of goods sold in the general ledger?
- Are persistent or one-directional variances investigated rather than absorbed?
- Does the design acknowledge framework and jurisdictional limits, with uncertain external-reporting treatments escalated to a qualified professional?
