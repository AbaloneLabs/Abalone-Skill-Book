---
name: exception_handling_and_expedites.md
description: Use when the agent is managing fulfillment exceptions, urgent orders, expedites, late shipments, inventory shorts, address problems, carrier failures, damaged goods, customer escalations, premium freight, service recovery, and root-cause control.
---

# Exception Handling And Expedites

Exceptions are where fulfillment systems reveal their weak points. Inventory shorts, damaged goods, late orders, failed labels, carrier caps, wrong addresses, fraud holds, customer escalations, and missed cutoffs all require decisions outside normal flow. Agents often solve exceptions one at a time by expediting, manually overriding, or asking someone to "just ship it." That can protect a single order while hiding root causes and creating cost leakage. Strong exception management restores service when justified, protects controls, and feeds learning back into the operation.

Use this skill when handling fulfillment exceptions, designing escalation paths, approving premium freight, or reviewing service recovery.

## Core Rules

### Classify The Exception Before Acting

Different exceptions need different responses. Classify by cause and impact: inventory short, damaged item, mispick, pack failure, address issue, payment or fraud hold, customer change, carrier failure, system outage, labor backlog, weather, compliance hold, or priority escalation. Then determine whether the order can still meet the promise and what recovery options exist.

Do not treat every exception as urgent. Some need customer communication, some need inventory correction, some need replacement, some need cancellation, and some justify premium freight. Classification prevents overreaction and helps root-cause analysis.

### Protect Control Points During Urgency

Urgent orders still need compliance, payment, inventory, safety, hazmat, export, temperature, and customer requirements. Skipping controls may create worse failures than a late shipment. Expedite should not mean bypassing scan, label, lot, serial, restricted-party, or packing requirements unless an authorized exception process exists.

Define which controls can never be bypassed and which can be handled through alternate approval. For regulated, high-value, dangerous, or customer-compliance freight, control integrity matters as much as speed.

### Use A Decision Framework For Expedites

Premium freight and manual handling should be justified by business value. Consider customer tier, promised date, contractual penalty, product criticality, safety impact, margin, order value, replacement cost, root cause, and whether the expedite will actually recover service. If the order will still miss the customer need, premium freight may waste money.

Require approval thresholds and reason codes. Expedite categories might include warehouse error, carrier failure, customer-paid upgrade, sales escalation, inventory recovery, medical criticality, or launch commitment. This makes cost visible and prevents expedite from becoming routine.

### Communicate Early And Specifically

When an exception affects the customer promise, customer service, sales, or the customer may need timely information. Communication should state what happened, what is being done, revised timing, options, and any action required from the customer. Vague messages create repeat contacts.

Coordinate internal and external communication. The warehouse should not promise what transportation cannot deliver, and customer service should not offer replacements that inventory cannot supply. Use a single current status.

### Maintain Exception Ownership

Exceptions often cross functions: warehouse, inventory, order management, customer service, transportation, finance, fraud, and sales. Without ownership, issues stall. Assign an owner for each exception who is responsible for next action, status, and closure.

Use queues or dashboards that show age, priority, reason, owner, customer impact, and deadline. Aging exceptions should escalate before the promise is missed, not after.

### Preserve Evidence And System Accuracy

Exception handling can distort inventory and financial records if not documented. Damaged goods, shorts, substitutions, cancellations, reships, credits, and returns need system transactions, photos, reason codes, and approvals. Otherwise the operation may ship replacements while inventory remains wrong.

For carrier failures, capture tracking, pickup proof, delivery exception, claim evidence, and customer impact. For warehouse errors, capture pick, pack, scan, and audit data. Evidence supports claims and improvement.

### Separate Recovery From Root Cause

Immediate recovery solves the customer issue; root cause prevents recurrence. After closure, classify whether the cause was inventory accuracy, replenishment, slotting, training, system logic, carrier capacity, demand planning, supplier quality, address validation, customer policy, or unrealistic SLA.

Do not let the team celebrate a heroic recovery while the same failure repeats. Review high-cost, high-frequency, and high-impact exceptions regularly.

### Design Exception Paths That Scale

Manual heroics may work for five exceptions and fail for five hundred. Build scalable paths: address validation queues, short-pick rules, substitution policies, damaged-item workflows, carrier fallback rules, premium freight approval, customer notification templates, and automated status triggers.

During peak, simplify exception handling where possible and prioritize by customer impact. A complex manual process can become the bottleneck.

## Common Traps

### Expediting Every Late Order

Premium freight should be a governed recovery tool, not the default answer. Use value and recoverability criteria.

### Bypassing Compliance Controls

Urgency does not justify illegal, unsafe, or contract-breaking shipment. Define non-negotiable controls.

### Leaving Exceptions Ownerless

If everyone is copied but no one owns the next action, the exception ages until the customer complains.

### Failing To Update Inventory

Reships, substitutions, damaged goods, and shorts must be transacted correctly or the next order repeats the failure.

### Communicating Too Late

Customers and internal teams need time to choose alternatives. Late communication removes options.

### Treating Manual Fixes As Success

Manual recovery may hide a process defect. Capture root cause and recurrence.

### Letting Sales Escalations Skip The Queue

Some escalations are valid, but uncontrolled sales pressure can distort priorities and cost. Use rules.

### Measuring Only Exception Count

Cost, age, customer impact, root cause, and recurrence matter more than raw count alone.

## Self-Check

- [ ] Is each exception classified by cause, customer impact, deadline, and recoverability?
- [ ] Are non-bypassable controls defined for compliance, payment, inventory, safety, hazmat, export, temperature, and customer requirements?
- [ ] Are expedite approvals governed by value, criticality, penalty, margin, root cause, and reason code?
- [ ] Are customer and internal communications specific, current, and coordinated?
- [ ] Is each exception assigned to an owner with next action and deadline?
- [ ] Are system transactions, photos, approvals, tracking, and evidence captured?
- [ ] Are recovery actions separated from root-cause analysis?
- [ ] Are recurring causes fed back to inventory, warehouse, carrier, system, supplier, or SLA owners?
- [ ] Are exception workflows scalable for peak volume?
- [ ] Could the operation explain why premium freight was used and what will prevent repeat use?
