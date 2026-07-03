---
name: drayage-carrier-dispatch-and-terminal-appointment-control.md
description: Use when the agent is coordinating drayage carrier dispatch, port pickup appointments, terminal appointment systems, chassis assignment, delivery scheduling, pre-pulls, live unloads, yard storage, driver wait time, or day-of-execution control for import and export container moves.
---

# Drayage Carrier Dispatch And Terminal Appointment Control

Drayage execution fails when dispatch is planned as a truck order rather than a synchronized chain of terminal access, driver capacity, chassis availability, release status, delivery appointment, yard handling, and empty return. The agent should treat appointments and dispatch instructions as operational controls. A carrier can accept a load and still be unable to execute if the appointment is wrong, the terminal rejects the driver, the chassis pool is empty, the warehouse cannot receive, or the container is blocked by release status.

## Core Rules

### Make the dispatch packet complete and unambiguous

Every drayage dispatch should include container number, size/type, steamship line, terminal, bill of lading, booking or delivery order reference, pickup number if required, release status, customs status, hazardous or reefer details, seal requirements, weight, overweight permits if needed, chassis requirement, delivery address, receiving hours, appointment time, contact numbers, access instructions, and empty return instructions if known. Missing details create dry runs, driver delays, or wrong-terminal attempts.

Differentiate live unload, drop, pre-pull, transload, bonded move, exam move, export load-in, and empty return. Each move has different time windows, equipment needs, and cost implications. If the customer uses shorthand like "just grab the box," convert it into dispatchable instructions.

### Treat terminal appointments as scarce operational assets

Terminal appointment systems are not clerical steps. They determine whether a driver can enter, when the container can be picked up or returned, whether dual transactions are possible, and how much recovery time exists if a driver is delayed. Confirm the correct terminal, appointment type, transaction, container availability, driver or trucker registration, trouble ticket process, cutoff times, and appointment modification rules.

Do not book appointments without checking downstream feasibility. A pickup appointment that conflicts with warehouse receiving hours, chassis availability, driver hours, or empty return windows may simply move the failure from the terminal to the consignee. When slots are scarce, prioritize containers by last free day, customer criticality, appointment flexibility, delivery readiness, and charge exposure.

### Align driver, chassis, and container constraints

Dispatch plans must account for equipment realities. Some moves require tri-axle chassis, gensets, reefer fuel checks, hazardous placards, overweight permits, bonded carrier authority, TWIC access, port-specific registration, or scale stops. Chassis shortages, pool restrictions, roadability problems, and tire or brake issues can disrupt a move even when the container is ready.

The agent should confirm whether the dray carrier supplies chassis, uses a pool, requires customer-owned equipment, or needs a specific provider. If a container is overweight, out-of-gauge, hazardous, refrigerated, or high value, the dispatch must include special handling and escalation instructions. Do not let cost pressure push an unsuitable carrier or equipment choice.

### Coordinate delivery appointment and unloading capacity

The warehouse or consignee is part of the drayage move. Verify receiving hours, dock constraints, labor availability, live unload time, drop yard rules, container parking, lift equipment, seal inspection, pallet exchange, lumper requirements, and paperwork. If the facility has strict appointments, the pickup plan must protect that time. If the facility unloads slowly, negotiate drop time, yard storage, or detention approval before the driver arrives.

For pre-pulls, confirm where the loaded container will sit, who pays yard storage and chassis, whether the yard is secure, when the final delivery appointment occurs, and whether the empty return clock is still acceptable. Pre-pulls can prevent demurrage but create detention, chassis, and yard costs if not managed.

### Use day-of-execution monitoring, not passive status waits

On execution day, monitor driver assignment, gate status, appointment check-in, trouble tickets, wait time, pickup confirmation, seal, outgate time, delivery arrival, unload completion, empty return appointment, and ingate confirmation. Escalate quickly when a driver is stuck because terminal trouble windows can close. Capture evidence for detention, wait time, dry run, and demurrage disputes.

Communication should be exception-focused. Stakeholders need to know whether the move is still on plan, what blocked it, what decision is needed, and what cost clock is affected. Avoid vague updates like "driver at port" if the driver has not outgated the container.

## Common Traps

- Sending a carrier an incomplete dispatch and expecting them to infer terminal, release, appointment, or delivery details.
- Booking terminal appointments before confirming warehouse receiving or chassis availability.
- Assuming a carrier acceptance means the move is executable. Driver, equipment, appointment, and terminal status still matter.
- Ignoring dual transaction opportunities. A coordinated empty return and import pickup can save time, but only if both transactions are accepted.
- Using pre-pull to avoid demurrage without managing yard, chassis, security, and detention exposure.
- Discovering overweight, hazmat, reefer, or bonded requirements after dispatch, causing refused pickup or compliance risk.
- Failing to capture check-in, outgate, arrival, unload, and ingate timestamps. Cost disputes and performance reviews need evidence.
- Letting a terminal trouble ticket sit unresolved until the appointment window expires.

## Self-Check

- Does the dispatch packet include all container, terminal, release, appointment, equipment, delivery, contact, and empty return details needed for execution?
- Have I confirmed the terminal appointment type, window, transaction, registration requirements, modification rules, and trouble process?
- Are driver capacity, chassis source, special equipment, permits, TWIC or bonded authority, hazmat, reefer, and weight constraints addressed?
- Does the pickup appointment align with warehouse receiving hours, unload capacity, drop rules, and customer readiness?
- If pre-pull or yard storage is used, are cost ownership, security, chassis, delivery timing, and empty return implications approved?
- On execution day, am I tracking assigned driver, gate status, outgate, delivery arrival, unload, empty return, and exceptions with timestamps?
- Have I escalated blocked appointments or terminal trouble with a decision and deadline, not merely a status note?
- Can stakeholders distinguish container picked up, driver waiting, terminal blocked, delivered loaded, unloaded, and empty returned?
