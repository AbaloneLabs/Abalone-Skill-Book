---
name: intermodal-delay-exception-and-drayage-coordination.md
description: Use when the agent is managing intermodal delays, drayage coordination, rail exceptions, missed rail cutoffs, terminal congestion, customer ETA changes, container holds, transload risk, or multimodal exception response.
---

# Intermodal Delay Exception And Drayage Coordination

Intermodal delays are often caused by the interfaces: rail cutoffs, ramp congestion, drayage capacity, customer receiving, customs holds, weather, chassis shortages, and documentation. Agents often report a delayed ETA without coordinating the ground recovery plan. This skill helps manage intermodal exceptions across rail, dray, terminal, customer, and inventory owners.

## Core Rules

### Identify the exception type precisely

Classify whether the issue is rail linehaul delay, missed cutoff, train annulment, ramp congestion, chassis shortage, driver shortage, customs hold, line hold, documentation issue, customer appointment miss, weather, derailment, or equipment problem.

Different exceptions have different owners and recovery options. A generic "rail delay" message is not enough.

### Protect customer commitments with updated ETAs

Recalculate ETA based on rail status, ramp unload, availability, dray appointment, customer receiving window, and empty return. Communicate confidence level and next update time.

Intermodal ETA should include the final mile. Train arrival ETA alone can mislead customers.

### Coordinate drayage recovery options

Options may include alternate dray carrier, priority pickup, offsite yard, transload, truck recovery, appointment change, weekend work, or holding until free time. Evaluate cost, service, and equipment effects.

Do not assume the dray carrier can absorb changes. Drayage is often the limiting factor in recovery.

### Manage rail cutoffs and ingates

Outbound intermodal requires correct ingate, equipment, documentation, weight, hazmat, and cutoff timing. Missed cutoffs can delay by days depending on schedule. Track tender timing and terminal acceptance.

A load is not safely moving by intermodal until it has actually ingated and made the train.

### Control transload and mode-shift decisions

If intermodal delay threatens service, transload to truck, divert, or mode shift may be considered. Check product handling, seals, claims, cost, customer approval, and inventory visibility before moving.

Recovery should not create damage, custody, or system problems worse than the delay.

### Set decision deadlines before options expire

Every exception has a point where choices disappear: rail cutoff, last free day, customer appointment release, warehouse labor cutoff, weekend terminal closure, driver hours, or truck recovery capacity. Set a decision deadline and name who must approve spend or service changes before that deadline.

Do not wait for perfect certainty if waiting removes the practical recovery option. A good exception plan states what will happen if the next rail update, hold release, or dray appointment is not confirmed by a specific time.

### Preserve custody, seal, and documentation control

When recovery changes the physical path, verify seal status, transfer paperwork, bill of lading updates, hazardous documentation, temperature records, customs status, and customer authorization. If a container is opened for transload or inspection, define who records condition and who accepts claim risk.

Intermodal exceptions can create ambiguous custody between rail, dray, terminal, warehouse, broker, and customer. That ambiguity becomes expensive when cargo is short, damaged, late, or misbilled.

### Monitor free time during exceptions

Delays often consume free time at destination or origin. Track last free day, hold reason, appointment availability, and who owns charges. Request relief where justified by rail or terminal fault.

Cost recovery depends on evidence. Capture timeline and cause.

### Keep internal owners aligned and review recurring delay patterns

Customer service, inventory, sales, procurement, finance, and warehouses may need to know delays and recovery choices. Provide factual status, risk, decision needed, and cost implication.

Intermodal exceptions create downstream planning decisions, not just transportation updates.

Analyze delays by lane, ramp, rail provider, dray carrier, customer, day of week, commodity, and season. Use patterns to adjust lead time, carrier choice, cutoff discipline, or safety stock.

Repeated exceptions should change the network design or promise, not only create daily firefighting.

## Common Traps

- Calling every exception a rail delay without identifying the true cause and owner.
- Reporting train ETA without ramp availability, drayage, and customer receiving timing.
- Assuming dray carriers can recover missed appointments without capacity impact.
- Missing origin ingate or rail cutoff and discovering the load did not depart.
- Choosing transload or truck recovery without checking product handling and custody risk.
- Waiting for another status update until cutoffs, appointment windows, or recovery capacity are no longer usable; changing route or mode without controlling seals, documents, customs status, hazmat records, or claim ownership
- Letting demurrage accrue during holds without cause evidence and charge ownership; failing to communicate cost and service tradeoffs to sales, service, inventory, and finance
- Treating recurring ramp or dray failures as isolated incidents; updating customers once and then letting ETAs age

## Self-Check

- Is the exception classified by rail, ramp, chassis, dray, customs, documentation, weather, customer, or equipment cause?
- Does ETA include rail movement, unload, availability, dray pickup, customer window, and confidence level?
- Are drayage recovery options, capacity, cost, appointment changes, and offsite yard needs evaluated?
- Are origin ingate, equipment, documentation, weight, hazmat, and rail cutoff status confirmed?
- Are transload, truck recovery, diversion, and mode-shift decisions checked for handling, seal, cost, claims, and visibility?
- Is there a decision deadline for each recovery option before cutoffs, free time, appointments, or capacity expire?
- Are custody, seal, customs, hazmat, temperature, BOL, and customer-approval records controlled after any path change?
- Are free time, last free day, holds, appointment availability, charge ownership, and relief evidence tracked?
- Are customer service, inventory, sales, procurement, finance, and warehouse owners aligned on risk and decisions?
- Are recurring delay patterns used to adjust lead times, lanes, providers, cutoffs, promises, and buffers?; can the exception response protect both service and cost without losing custody or visibility?
