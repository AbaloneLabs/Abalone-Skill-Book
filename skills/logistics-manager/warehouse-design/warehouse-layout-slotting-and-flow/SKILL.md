---
name: warehouse_layout_slotting_and_flow.md
description: Use when the agent is designing warehouse layout, slotting strategy, pick paths, travel reduction, product placement, ABC velocity zones, replenishment flow, packing adjacency, congestion reduction, safety, or operational flow across receiving, storage, picking, packing, and shipping.
---

# Warehouse Layout Slotting And Flow

Warehouse layout is a flow design problem, not a floor-plan decoration exercise. Slotting, aisle direction, pick path, replenishment, packing adjacency, and staging determine labor productivity, accuracy, congestion, and safety. Agents often recommend "put fast movers near shipping" without considering replenishment conflicts, product compatibility, order affinity, ergonomics, seasonality, or how work actually moves through the building. This skill helps design layout and slotting around real operational behavior.

## Core Rules

### Map End-To-End Flow Before Moving Slots

Layout decisions should support the full path: receiving, quality, putaway, reserve, forward pick, replenishment, value-added services, packing, sortation, shipping, returns, and exceptions. Moving one product closer to packing may create congestion in replenishment or receiving. A short pick path can still fail if packing is overwhelmed.

Use process maps, travel observations, order data, and congestion points. Identify where labor walks, where forklifts travel, where product waits, and where errors occur. Do not optimize one zone in isolation.

### Slot By Velocity, Cube Movement, And Order Affinity

ABC velocity is useful but incomplete. Slotting should consider pick frequency, cube movement, unit weight, dimensions, handling difficulty, order affinity, seasonality, replenishment frequency, cartonization, and customer requirements. A fast but bulky SKU may need more space, not just a prime slot. Items often ordered together may belong near each other if it reduces travel and packing complexity.

Use the right measure for the operation: lines picked, eaches, cases, pallets, cube, weight, revenue, margin, or service priority. Revisit slots as demand changes. Static slotting becomes stale quickly in seasonal or promotional businesses.

### Separate Pick Flow From Replenishment Flow Where Possible

Picking and replenishment can conflict. Forklifts replenishing forward pick faces during peak picking create safety risk and congestion. If replenishment travels through the same narrow aisles as pickers, productivity and injury risk suffer.

Design replenishment paths, timing, drop zones, and triggers deliberately. Consider off-shift replenishment, one-way aisles, separate equipment, forward pick sizing, and reserve adjacency. A slot that is easy to pick but impossible to replenish is not a good slot.

### Design For Ergonomics And Product Risk

Heavy, fragile, high-value, hazardous, liquid, awkward, sharp, temperature-sensitive, or regulated items need placement that protects people and product. Heavy fast movers should not require frequent overhead lifts. Fragile goods should not sit where forklifts or heavy cartons damage them. High-value goods may need controlled access.

Slotting should reduce bends, reaches, lifts, and unsafe handling. Put heavy items in ergonomic zones, separate incompatible products, and place special handling near appropriate packing or control areas. Safety and accuracy are layout outcomes.

### Control Congestion At Choke Points

Congestion often appears at aisle intersections, fast-pick zones, packing stations, replenishment drop points, shipping staging, returns, and equipment charging. A layout can have enough total space but fail at narrow choke points. During peak, small design flaws magnify.

Observe peak flow or simulate it. Add lane markings, one-way paths, widened intersections, separate fast movers, staggered waves, buffer zones, and clear parking for equipment. Do not let pallets, totes, or exceptions occupy travel paths.

### Align Packing And Shipping With Pick Strategy

Picking strategy affects packing. Batch, wave, zone, cluster, pick-to-cart, pick-to-tote, pallet pick, and goods-to-person all create different downstream needs. Packing needs space, materials, scales, printers, quality checks, cartonization, carrier sort, hazardous labels, and exception handling.

Do not design pick paths without deciding how orders consolidate, verify, pack, and ship. Fast pick performance is not enough if packing becomes the bottleneck or mis-sorts increase.

### Use Visual And System Controls Together

Slotting only works if people and systems agree. Locations need clear IDs, barcodes, signage, aisle markers, pick sequence, replenishment logic, and WMS accuracy. Visual controls can reduce confusion, but they must match system data. Re-slotting requires updates to location master, labels, replenishment rules, and training.

Avoid informal slot changes that are not reflected in WMS. A product "temporarily moved" can cause inventory errors, shorts, and wasted search time. Control temporary overflow with status and expiration.

### Review Layout Against Growth And Change

Layout should support current needs while allowing change. New channels, SKU growth, returns, automation, value-added services, customer packaging rules, and peak volume can alter flow. A layout that uses every inch today may have no place for tomorrow's exception.

Document assumptions about order mix, volume, SKU count, labor model, and service levels. Set review triggers: peak season, new customer, SKU growth, pick productivity drop, replenishment misses, or congestion incidents.

## Common Traps

### Putting Fast Movers Near Shipping Without Replenishment Review

High-velocity slots need enough pick face and safe replenishment, not only proximity.

### Using ABC Alone

Weight, cube, dimensions, affinity, seasonality, and handling risk can outweigh simple velocity rank.

### Ignoring Packing Bottlenecks

Picking faster can only move the bottleneck downstream if packing and shipping are not designed.

### Letting Temporary Overflow Become Permanent

Uncontrolled overflow breaks WMS accuracy and pick discipline.

### Creating Forklift And Picker Conflicts

Shared aisles and peak replenishment create safety and congestion problems.

### Forgetting Ergonomics

Poor slot heights and heavy lifts create injury risk and slow work.

### Treating Returns As A Side Corner

Returns can clog flow and contaminate inventory if not intentionally placed.

### Not Re-Slotting After Demand Changes

Seasonality and promotions can make last quarter's slotting wrong.

## Self-Check

- [ ] Is the layout based on end-to-end flow from receiving through shipping and returns?
- [ ] Does slotting use velocity, cube, weight, dimensions, handling, affinity, seasonality, and service priority?
- [ ] Are pick and replenishment paths, timing, equipment, and drop zones designed to avoid conflict?
- [ ] Are heavy, fragile, high-value, hazardous, liquid, and awkward items placed safely?
- [ ] Are congestion points at aisles, packing, staging, returns, and intersections identified and controlled?
- [ ] Does packing and shipping capacity match the chosen pick strategy?
- [ ] Are location IDs, barcodes, signage, WMS rules, and replenishment logic synchronized?
- [ ] Are temporary overflow and exceptions controlled with status and ownership?
- [ ] Are growth, SKU changes, new channels, and peak season included in layout assumptions?
- [ ] Would the flow still work during peak volume, a promotion spike, or a replenishment miss?
