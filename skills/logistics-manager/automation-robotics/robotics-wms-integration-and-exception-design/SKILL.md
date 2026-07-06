---
name: robotics-wms-integration-and-exception-design.md
description: Use when the agent is integrating robotics with WMS, WES, WCS, warehouse systems, inventory control, order orchestration, exception handling, robot tasking, automated picking, or robotic fulfillment workflows.
---

# Robotics WMS Integration And Exception Design

Robotics integration succeeds when system decisions, physical movements, inventory state, and exception handling stay synchronized. A robot can move exactly as instructed and still break the operation if WMS tasks, inventory status, order priority, replenishment, or exception queues are wrong. Agents often focus on API connectivity and miss the operational contract between robotics and warehouse systems. This skill helps design integration around real warehouse control.

## Core Rules

### Define system ownership of each decision

Clarify whether WMS, WES, WCS, robotics software, labor management, or human supervisors own order release, task sequencing, inventory allocation, robot dispatch, replenishment, putaway, exceptions, and completion confirmation.

If two systems think they are the decision owner, tasks conflict. If no system owns a decision, work stalls in hidden queues.

### Synchronize inventory state tightly

Robots rely on accurate location, quantity, item, container, license plate, and status data. Define when inventory changes state: picked, staged, moved, shorted, damaged, quarantined, replenished, counted, or packed. Handle latency and failed confirmations explicitly.

Inventory state should match physical reality and customer promise. A robot movement not reflected in WMS can create phantom stock or lost orders.

### Design exceptions before go-live

Plan what happens when robots cannot reach a location, a tote is missing, inventory is short, a barcode fails, product is damaged, a task times out, a path is blocked, a battery is low, or a user cancels an order. Exceptions need queues, owners, alerts, and resolution rules.

Exception design is not an afterthought. Automated systems expose thousands of small failures that manual operators used to resolve silently.

### Protect order priority and service logic

Robotics tasking should understand cutoffs, promised delivery dates, customer priority, carrier departure, wave planning, hot orders, hazardous or temperature items, and partial shipment rules. Otherwise the system may optimize motion while missing service commitments.

Do not let robot efficiency override customer promise unless leadership explicitly accepts that tradeoff.

### Test end-to-end workflows, not only interfaces

Integration testing should cover order release, replenishment, pick, short, cancel, change, pack, ship, return, inventory count, maintenance downtime, network loss, and recovery. Include realistic volume, peak concurrency, and bad data.

An API test that passes one clean transaction is not proof that the operation can run.

### Prepare manual recovery and reconciliation

When robotics or WMS integration fails, define how orders are found, inventory is reconciled, robots are stopped or drained, work is shifted to manual mode, and customer promises are protected. Track what happened during outage windows.

Manual recovery should not create a second inventory truth. Reconciliation steps need to be documented and tested.

### Control master data and physical fit

Robotics depends on dimensions, weight, stackability, conveyability, graspability, barcode placement, tote compatibility, location geometry, and item restrictions. Bad master data creates failed picks, jams, damage, and misrouted tasks.

Integration quality is limited by physical data quality. Validate master data for the items and containers the robot will actually handle.

### Monitor integration health after launch

Track task latency, failed messages, exception rates, robot idle time, inventory discrepancies, blocked paths, short picks, cancel failures, and manual interventions. Set alert thresholds and owners.

Do not wait for missed shipments to notice integration degradation. Small error queues can grow into service failure quickly.

### Govern changes across systems

WMS configuration changes, robotics software updates, item master changes, layout changes, carrier cutoffs, and process changes can break integration assumptions. Require change review, testing, and release notes across all affected systems.

Warehouse integration is a living system. Uncoordinated changes are a common cause of automation instability.

## Common Traps

- Treating API connectivity as proof that robotic fulfillment is ready.
- Leaving WMS, WES, WCS, and robotics software with overlapping decision ownership.
- Allowing inventory moves to occur physically without timely system confirmation.
- Designing exceptions only after robots start failing tasks in production.
- Optimizing robot travel while missing cutoffs, carrier departures, and customer priorities.
- Testing only happy-path transactions instead of shorts, cancels, downtime, bad data, and recovery.
- Creating manual recovery that breaks inventory integrity; ignoring dimensions, weights, barcodes, tote fit, and item handling restrictions
- Letting error queues grow without owners and thresholds; changing WMS, item master, layout, or robot software without integration retesting

## Self-Check

- Is decision ownership clear across WMS, WES, WCS, robotics software, labor systems, and supervisors?
- Are inventory state transitions defined for picks, moves, shorts, damage, quarantine, replenishment, counts, and packing?
- Are exception queues, owners, alerts, and resolution rules designed for robot, inventory, task, path, and data failures?
- Does robot tasking respect order priority, cutoffs, carrier departures, hot orders, and partial shipment rules?
- Has end-to-end testing covered realistic volume, bad data, cancellations, shorts, returns, downtime, and recovery?
- Is manual recovery documented without creating conflicting inventory truth?
- Are dimensions, weight, stackability, graspability, barcode placement, tote fit, and location geometry validated?
- Are integration health metrics monitored with alert thresholds and owners?
- Are system, layout, process, and master-data changes governed through retesting?
