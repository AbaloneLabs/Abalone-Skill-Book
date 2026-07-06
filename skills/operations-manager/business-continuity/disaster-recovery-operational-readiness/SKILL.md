---
name: disaster-recovery-operational-readiness.md
description: Use when the agent is assessing disaster recovery readiness from an operations perspective, coordinating with technology or facilities recovery, validating restore procedures, preparing operational data recovery, aligning RTO and RPO to service needs, or checking whether people can resume work after major outage or site disruption.
---

# Disaster Recovery Operational Readiness

Disaster recovery is not only a technical restore. Operations must know what data, access, people, facilities, vendors, queues, and communication are needed to resume service after a major disruption. Agents often assume that if systems are restored, operations are recovered. This skill helps the agent check the operational readiness needed to use restored systems and resume controlled work.

## Core Rules

### Align Recovery Targets To Operations

Confirm recovery time objective and recovery point objective for each critical process. RTO defines how quickly service must resume. RPO defines how much data loss or re-entry is tolerable. These targets should reflect customer, regulatory, financial, safety, and internal dependency needs.

Do not accept generic technology targets without checking whether they support operational promises.

### Identify Operational Data Needs

Determine what data must be available after recovery: open cases, orders, approvals, customer commitments, payment records, inventory, schedules, access lists, vendor tickets, audit evidence, and manual fallback records. Some data may live outside core systems.

If data is restored to an earlier point, define reconciliation and duplicate prevention.

Prioritize data by restart sequence. Staff may need enough current data to resume critical work before every historical record is restored, but the plan must state what can safely wait.

### Validate Access And Credentials

After disruption, staff may need VPN, multi-factor authentication, physical badges, admin rights, vendor portal access, shared mailbox access, emergency devices, or temporary permissions. Confirm who can restore or approve access if normal identity systems are impaired.

Access recovery should preserve security. Emergency access needs logging, expiration, and review.

### Coordinate Technology And Operational Sequence

Technology teams may restore systems in an order that does not match operational dependencies. Identify which systems, integrations, reports, printers, scanners, phone lines, payment tools, and vendor portals must come back first for critical processes.

State the sequence needed for minimum viable operation and what can wait.

Define what operations will do if a restored dependency fails validation. Teams need a fallback decision before they restart work, not after customers receive incorrect or duplicate outcomes.

### Prepare Work Resumption Procedures

Staff need instructions for what to do when systems return: verify queue status, compare manual records, identify duplicates, re-enter work, clear held items, validate reports, update customers, and report anomalies. Without resumption procedures, restored systems can create confusion and data errors.

Define who announces that work can resume and which controls apply.

Include a hold period if needed. For some processes, teams should validate data, access, integrations, and control reports before releasing all held work or restarting customer-facing promises.

### Include Facilities And Physical Constraints

Disaster recovery may involve site access, power, equipment, inventory, paper records, mail, shipping, safety checks, workspace, and transport. Remote work may not solve physical operations or regulated record handling.

Check whether alternate sites or remote work can support the required work volume and privacy needs.

### Test Vendor And Third-Party Recovery

Critical vendors may have their own disaster recovery plans, but operations needs evidence of what service will be available, how to escalate, and what workaround exists. Include payment processors, logistics, telecom, cloud tools, outsourced teams, and facilities vendors.

Vendor recovery promises should be tested or reviewed, not assumed.

### Plan Communication During Recovery

Define who tells staff, customers, leaders, vendors, and internal partners what is restored, what remains degraded, what work should resume, and when the next update will come. Communicate residual limitations clearly.

Avoid saying "all systems restored" if operations still cannot process key work safely.

Provide specific language for degraded recovery. Staff should know whether to say work is resumed, partially resumed, manually handled, delayed, or still under validation.

### Review After Exercises And Real Events

Update readiness after tests, outages, system changes, staffing changes, vendor changes, and process redesign. Track gaps and owners. A disaster recovery plan that is not maintained becomes an artifact of old systems.

Operational readiness should be tested alongside technical recovery.

## Common Traps

- Assuming technical restore equals operational recovery.
- Accepting RTO and RPO targets without validating operational service impact.
- Forgetting data outside core systems, such as manual trackers, shared inboxes, and vendor portals.
- Restoring access quickly but without expiration, logging, or review.
- Letting technology restore sequence ignore operational dependencies.
- Resuming work without reconciliation, duplicate prevention, and queue validation.
- Ignoring physical site, equipment, inventory, mail, shipping, or privacy constraints; trusting vendor disaster recovery claims without evidence or escalation paths
- Communicating full recovery while work remains degraded or controlled manually; failing to update DR readiness after system, vendor, staffing, or process changes

## Self-Check

- Are RTO and RPO aligned to critical operational processes and service promises?
- Are operational data needs identified, including records outside core systems?
- Are reconciliation and duplicate prevention defined for restored or manually captured data?
- Are emergency access and credentials available with security controls?
- Is system restore sequence aligned to minimum viable operation?
- Are work resumption procedures clear for staff?
- Are facilities, equipment, physical records, and remote-work constraints included?
- Are vendor and third-party recovery dependencies reviewed with evidence and contacts?
- Does communication distinguish restored systems from restored operational capability?
- Are readiness gaps updated after tests, incidents, and operating changes?
