---
name: container-availability-free-time-and-per-diem-risk.md
description: Use when the agent is managing port container availability, last free day, demurrage, detention, per diem, free time, empty return windows, chassis constraints, ocean carrier charges, terminal storage, or cost exposure from delayed import pickup and container return.
---

# Container Availability Free Time And Per Diem Risk

Import drayage is often treated as a simple pickup and delivery move, but the expensive failures happen in the timing rules around container availability, last free day, demurrage, detention, per diem, terminal appointments, chassis supply, and empty return acceptance. A container can be physically at the port but not legally, operationally, or commercially available. The agent should manage the chain of dates, permissions, payment status, appointments, and equipment return options before charges become unavoidable.

## Core Rules

### Define availability before promising pickup

Do not equate vessel arrival, discharge, customs release, freight release, and terminal availability. A container may arrive on a vessel, but still be unavailable because it has not discharged, is in a closed area, has a customs or exam hold, lacks ocean carrier freight release, has unpaid charges, needs a delivery order, requires appointment access, or is blocked by terminal status. Confirm the specific terminal status fields and the party responsible for each release.

The pickup promise should be tied to a verified state: container number, terminal, vessel/voyage, discharge status, customs release, carrier release, payment status, available date, last free day, appointment requirement, chassis requirement, hazardous or reefer status, overweight risk, and whether the bill of lading covers multiple containers with different dates. If the customer wants a delivery appointment before these facts are known, label it tentative.

### Build the free-time calendar as a control tool

Last free day is not just a date on a report. It is a deadline that depends on terminal free time, ocean carrier free time, rail or port rules, weekends, holidays, holds, split releases, and whether extensions or clock stoppage apply. Track demurrage, detention, and per diem separately because different parties may assess them for different behaviors: container remaining at terminal, container out too long, chassis use, or failure to return empty equipment.

Create a calendar that includes vessel ETA, discharge, availability, customs release, last free day, pickup appointment, warehouse delivery appointment, unloading time, empty return cutoff, and charge start. Work backward from the last free day and the empty return obligation. A plan that picks up on time but cannot unload or return the empty can still create detention or per diem exposure.

### Manage empty return as part of the import move

Empty return constraints are one of the most common hidden cost drivers. The destination warehouse may need several days to unload, the consignee may reject delivery, the steamship line may restrict empty returns to specific terminals, the terminal may not accept the equipment type, the appointment window may be unavailable, or chassis pools may be constrained. Confirm where and when the empty can be returned before dispatching the loaded container.

For live unloads, check whether the consignee can unload within free time and whether detention starts during the live service. For drop-and-pick, confirm yard space, lift equipment, seal process, container accessibility, and whether the trucker can retrieve the empty promptly. If the consignee asks to hold a container as storage, price and approve the detention exposure rather than treating it as a free extension.

### Assign charge ownership before charges accrue

Demurrage, detention, per diem, storage, pre-pull, yard storage, chassis, dry run, wait time, and exam fees create disputes when ownership is not clear. Determine whether the forwarder, customs broker, consignee, shipper, ocean carrier, drayage carrier, warehouse, or terminal caused the delay. Document timestamps, releases, appointment attempts, terminal closures, hold notices, delivery appointment requests, and customer instructions.

Charge prevention is better than charge recovery. If a container is at risk, escalate before the last free day with a specific action: approve pre-pull, change delivery appointment, request free-time extension, pay pending charges, prioritize customs clearance, split a delivery, use a bonded move, or authorize storage. Do not merely report that charges may occur.

### Treat data quality as operational risk

Small errors in container numbers, bill of lading, SCAC, terminal, steamship line, consignee, customs entry, or delivery order can make systems show conflicting statuses. Track source of truth and timestamp for each update. Avoid relying on a single stale portal screenshot. When statuses conflict, verify with the terminal, carrier, broker, forwarder, or dray provider according to urgency.

Exception communication should be precise. Say which container, which charge clock, which date, which party must act, and what the cost consequence is. General statements such as "container is not available" or "free time is running out" do not create accountable action.

## Common Traps

- Treating vessel arrival as container availability. Discharge, release, payment, holds, and appointment access can all block pickup.
- Tracking only demurrage and forgetting detention, per diem, chassis, yard storage, wait time, and dry run charges.
- Focusing on loaded pickup while ignoring empty return. Charges often continue after the consignee receives the goods.
- Assuming free time stops automatically during customs holds, terminal closures, or exams. Rules vary and may require disputes or extensions.
- Letting the customer use the container as warehouse storage without pricing detention exposure and return risk.
- Waiting until the last free day to book an appointment. Terminal slots, chassis supply, and warehouse capacity may already be gone.
- Failing to document release timestamps and appointment attempts. Disputes are weak without evidence.
- Blaming the dray carrier for charges caused by late paperwork, unpaid freight release, unavailable empty returns, or consignee delay.

## Self-Check

- Have I verified container availability through discharge, customs release, carrier release, payment, terminal status, and appointment access rather than vessel ETA alone?
- Did I build a calendar covering last free day, pickup, delivery, unloading, empty return, and each charge clock?
- Have I checked empty return location, appointment rules, equipment acceptance, chassis needs, and consignee unload timing before dispatch?
- Is charge ownership documented with timestamps, release status, appointment attempts, holds, closures, and customer instructions?
- Have I escalated at-risk containers with a concrete decision such as pre-pull, extension, payment, delivery change, or storage approval?
- Did I separate demurrage, detention, per diem, storage, chassis, wait time, and dry run exposure in communication?
- Are data conflicts between terminal, carrier, broker, and dray provider resolved or clearly marked with source and timestamp?
- Can the customer see the exact action needed, deadline, cost consequence, and accountable party?
