---
name: critical-process-and-single-point-failure-review.md
description: Use when the agent is identifying critical operational processes, reviewing single points of failure, mapping dependencies, prioritizing what must continue during disruption, assessing key-person or vendor dependency, or deciding which operations require continuity plans and backup capability.
---

# Critical Process And Single Point Failure Review

Business continuity starts by deciding what must not fail. Many operations look resilient until a single approver, tool, vendor, site, report, credential, or informal expert becomes unavailable. Agents often ask for a continuity plan before identifying the few processes and dependencies that would actually stop service or create high harm. This skill helps the agent find critical processes and single points of failure before designing continuity actions.

## Core Rules

### Define Criticality By Impact

Classify critical processes by consequences of interruption: safety, customer harm, legal or compliance breach, financial loss, service outage, operational blockage, employee access, vendor dependency, reputational risk, and recovery difficulty. Do not rely only on process volume.

A low-volume process can be critical if it handles regulated deadlines, payroll, security access, refunds, incident command, or safety checks.

### Map The End-To-End Process

Identify the steps, owners, systems, inputs, approvals, handoffs, records, communications, outputs, and downstream dependencies. Include work that happens outside official workflow, such as spreadsheets, personal inboxes, local trackers, and verbal approvals.

Continuity gaps often live in informal work. If the process map ignores them, the plan will fail under disruption.

### Identify Single Points Of Failure

Look for one person, site, vendor, credential, approval, report, system, data feed, physical asset, carrier, bank, integration, or knowledge source that can stop the process. Include hidden single points such as one person who knows how to reconcile a report or one vendor contact who can unlock an account.

Rank each single point by impact, likelihood, time to recover, and existing backup.

### Check Time Sensitivity

Define maximum tolerable downtime, recovery time objective, recovery point objective where data matters, and deadline sensitivity. Some work can pause for a day; other work must continue within minutes or hours.

Time sensitivity drives backup design. A process with a two-hour tolerance needs prepared access and trained backups, not a plan to start training during the disruption.

Check whether time sensitivity changes by calendar. Payroll, month-end, regulatory cutoffs, peak season, or launch windows can make an otherwise deferrable process critical for a short period.

### Review People And Skill Dependencies

Identify certifications, licenses, permissions, languages, location access, supervisor authority, and tribal knowledge needed to perform the process. Check whether backups are trained, current, available, and authorized.

Do not count a backup who has no current access, no recent practice, or the same availability risk as the primary.

### Review Vendor And Third-Party Dependencies

Map vendors, platforms, outsourced teams, logistics providers, payment processors, telecom, facilities, and data providers. Check service commitments, escalation contacts, alternate routes, manual workarounds, and contract obligations.

If the vendor fails, the operation still owns the customer or internal impact. Vendor dependency should be visible in continuity planning.

### Identify Minimum Viable Operation

For each critical process, define what must continue at minimum: essential transactions, customer communication, safety steps, regulatory deadlines, decision approvals, or data capture. Separate must-do work from work that can pause or degrade.

This prevents teams from trying to maintain full normal service during disruption when the real goal is harm reduction.

Name the customer or internal expectation during minimum viable operation. If normal service is unavailable, stakeholders need to know what remains available, what is delayed, and what requires escalation.

### Prioritize Mitigation

Mitigate the most severe and least recoverable single points first. Options include cross-training, alternate vendor, manual fallback, redundant access, documented procedure, emergency approval path, inventory buffer, system backup, or service-scope reduction.

Do not spread effort thinly across every dependency while the most critical one remains exposed.

### Keep Ownership Clear

Each critical process and failure point needs an owner responsible for readiness, testing, and updates. Ownership should include authority to fix gaps or escalate investment needs.

If ownership is split across functions, name the coordinator and decision path.

## Common Traps

- Treating high-volume processes as automatically most critical while missing low-volume high-harm work.
- Mapping only official workflow and ignoring side spreadsheets, personal inboxes, and informal approvals.
- Counting backup people who lack access, practice, authority, or availability.
- Ignoring third-party and physical dependencies because they sit outside the operations team.
- Defining continuity as full normal service instead of minimum viable operation.
- Focusing on likely disruptions while ignoring low-likelihood severe failure points.
- Treating vendor SLAs as continuity plans; leaving single points of failure known but unowned
- Designing mitigation before defining recovery time and downtime tolerance; assuming a process is resilient because it has never failed before

## Self-Check

- Are critical processes ranked by operational harm, not only volume?
- Is the end-to-end process mapped, including informal work and hidden handoffs?
- Are single points of failure identified across people, tools, vendors, sites, data, approvals, and assets?
- Are downtime tolerance and recovery time needs defined?
- Are backup people verified for training, access, authority, and availability?
- Are vendor and third-party dependencies included with escalation and fallback options?
- Is minimum viable operation defined separately from normal service?
- Are mitigation priorities based on severity, recoverability, and readiness gap?
- Does each critical process and failure point have an accountable readiness owner?
- Are unresolved ownership or investment gaps escalated rather than left as known risks?
