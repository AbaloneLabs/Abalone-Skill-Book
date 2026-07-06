---
name: change-management-testing.md
description: Use when the agent is testing change management controls over financial systems, evaluating the process for requesting, approving, testing, and migrating changes, assessing emergency changes, testing segregation between development and production, or deciding whether change management weaknesses undermine reliance on automated controls that depend on stable configuration.
---

# Change Management Testing

Change management is the IT general control that protects the stability of financial systems. Every automated control, every validation rule, every workflow, and every report depends on a configuration that someone can change. If changes are not properly requested, tested, approved, and migrated, then a configuration that was correct when tested can silently become incorrect — disabling a control, altering a calculation, or opening an access gap — and the auditor's prior testing becomes stale. The discipline is to confirm that the configurations underlying relied-upon automated controls were protected from unmanaged change throughout the period, not only at the point of testing.

## Core Rules

### Test the full change lifecycle, not just the approval step

A credible change management process moves through identifiable stages. Confirm each is present and operating:

1. **Request / justification**: is there a documented request stating what changes and why?
2. **Impact assessment and testing**: is the change assessed for impact on financial processes, controls, and interfaces, and tested before migration?
3. **Approval**: is the change approved by an appropriate business and IT authority, with evidence?
4. **Migration**: is the change moved to production in a controlled way, by someone other than the developer?
5. **Post-implementation review**: is the change confirmed to have worked as intended after go-live?

Testing only the approval signature misses the most common failure modes: untested changes that break controls, and developers migrating their own code without independent oversight.

### Confirm segregation between development and production

The single most important structural control is that the people who develop or configure changes are not the people who migrate them to production. Without this segregation, a developer can push untested or unapproved changes directly into the live financial system. Confirm:

- developers do not have write access to production environments;
- migration to production is performed by a separate role (release management, IT operations);
- emergency override paths that bypass this segregation are logged and reviewed.

Where developers hold production access (common in smaller or legacy environments), the structural control is absent and every change carries elevated risk; compensate with stronger detective controls and post-change review.

### Test emergency changes specifically

Emergency (or "hotfix") changes are where change management most often breaks down, because urgency is used to justify bypassing the normal process. For emergency changes, confirm:

- the emergency route is used genuinely for emergencies, not routinely for convenience;
- each emergency change has after-the-fact documentation, testing, and approval;
- emergency changes are reviewed promptly after go-live to confirm correctness and to fold any permanent fix back into the formal process;
- the volume of emergency changes is monitored — a high proportion signals a broken normal process.

A process where a large share of changes are "emergencies" is effectively an uncontrolled process, regardless of what the policy says.

### Link change management testing to the automated controls being relied upon

Change management is not tested in the abstract; it is tested to support reliance on specific automated controls and configurations. For each significant automated control or configuration relied upon:

- identify the configuration or code that enforces it;
- confirm it was not changed during the period, or that any change was properly managed and the control re-tested after the change;
- if it was changed, confirm the change did not alter the control's operation in a way that undermines reliance.

This linkage is what makes change management relevant. Without it, change management testing is a generic IT exercise disconnected from the audit strategy.

### Test that changes to financial reporting configuration are specifically controlled

Beyond application code, financial systems contain configuration that directly affects the financial statements: chart of accounts mappings, depreciation rules, revenue recognition schedules, allocation rules, report definitions. Changes to these are financial reporting changes and warrant business (not just IT) approval. Confirm:

- who can change financial reporting configuration;
- whether changes require finance approval, not only IT approval;
- whether changes are tested with finance involvement before migration;
- whether a log of configuration changes is reviewed by finance.

Unmanaged changes to financial reporting configuration can silently alter the numbers, and are often less controlled than code changes because they are treated as "just configuration."

### Test user acceptance testing (UAT) for financial relevance

Where UAT is performed before migration, confirm it is meaningful for financial controls:

- does UAT include scenarios that exercise the financial controls and validations, not just whether the system "works"?
- is UAT performed or reviewed by finance users who understand the financial impact, not only IT?
- are UAT failures resolved and re-tested before migration?

Cosmetic UAT that confirms the system runs without testing the specific controls and calculations provides false assurance that the change is safe for financial reporting.

### Assess the change pipeline for the period between testing and year-end

Changes can occur after the auditor's testing but before period-end. Confirm what changes occurred in the gap between the last control test and the balance sheet date, and whether any of them affected the relied-upon controls. A change in that window that altered a relied-upon control invalidates the prior test and requires re-testing or substitution. Build the change management procedures to cover the period through (or near) year-end, not just the fieldwork window.

### Treat change management weaknesses as risk amplifiers

Where change management is weak — uncontrolled migrations, developers with production access, routine emergency changes, no UAT — the reliability of every automated control and every system-generated report is in question. This is not just an IT finding; it forces a strategy shift toward more substantive testing, less reliance on automated controls, and more independent evidence about the data. Record the weakness and its effect on the audit response explicitly.

## Common Traps

- **Testing only the approval step** of the change lifecycle, missing untested changes and self-migration by developers.
- **Accepting emergency change processes at face value** without checking the volume and the after-the-fact documentation.
- **Treating change management as a generic IT test** without linking it to the specific automated controls and configurations being relied upon.
- **Overlooking changes to financial reporting configuration** (account mappings, depreciation rules, report definitions) that are often less controlled than code.
- **Accepting cosmetic UAT** that confirms the system runs without exercising the financial controls and calculations.
- **Stopping change management testing at the fieldwork date**, missing changes in the window before period-end that could affect relied-upon controls.
- **Allowing developers to hold production access** without compensating detective controls and post-change review.
- **Treating change management weaknesses as IT observations only**, without reflecting them in reduced reliance on automated controls and expanded substantive testing; **Missing interface changes** — changes to data feeds between systems can alter downstream reporting even when each individual system is unchanged

## Self-Check

- Did I test all stages of the change lifecycle — request, impact assessment and testing, approval, migration, post-implementation review — not just approval?
- Did I confirm segregation between development and production, including who has write access to production?
- Did I test emergency changes for genuine emergency use, after-the-fact documentation, and proportion of total changes?
- For each significant automated control or configuration relied upon, did I confirm it was unchanged in the period or that any change was properly managed and re-tested?
- Did I specifically test changes to financial reporting configuration (account mappings, depreciation, revenue schedules, report definitions) for finance approval?
- Did I assess UAT for financial relevance — scenarios exercising the controls, finance involvement — not just whether the system runs?
- Did I extend change management coverage through or near year-end to catch changes after fieldwork that could affect relied-upon controls?
- Where change management is weak, did I reflect it in reduced reliance on automated controls and expanded substantive testing, not only in an IT finding?
