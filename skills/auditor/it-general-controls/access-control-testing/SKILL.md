---
name: access-control-testing.md
description: Use when the agent is testing logical access controls over financial systems, evaluating user provisioning and de-provisioning, reviewing access reviews, assessing privileged or super-user access, testing authentication and password controls, or deciding whether access control weaknesses undermine reliance on automated controls and the integrity of data.
---

# Access Control Testing

Logical access control is the foundation of IT general controls: if the wrong people can get into the financial systems, every control that depends on those systems is compromised. Access control testing is also where over-confidence is most dangerous, because entities routinely produce clean-looking access reports that hide privileged accounts, orphaned access from departed users, shared credentials, and super-user capabilities that bypass the application's controls entirely. The discipline is to test access as it actually operates — including the back doors — not as the access policy describes it.

## Core Rules

### Test the full user lifecycle: joiner, mover, leaver

Access risk concentrates at the points where access changes. Test all three lifecycle events:

- **Joiner**: how is access granted to new users? Is it based on an approved request, tied to a role appropriate to the job, and restricted to what is needed?
- **Mover**: when a user changes role, is their old access removed and new access granted? This is the most common leakage point — accumulated access from prior roles that is never revoked.
- **Leaver**: when a user departs, is access disabled promptly (same day for high-risk systems), and is contractor/vendor access time-boxed and revoked on expiry?

For each, confirm the trigger (HR action, contract end), the authorisation, and the timeliness. Orphaned accounts of departed users are both a finding and a fraud risk; accumulated "mover" access is the most common source of unintended segregation-of-duties conflicts.

### Test periodic access reviews for genuine operation

Most entities have a periodic access recertification process. Confirm it actually operates rather than rubber-stamps:

- Is the review performed by someone who understands the access (the user's manager or a business owner), not just IT?
- Does the reviewer see meaningful detail (which roles, which transactions), or only a list of usernames?
- Are inappropriate accesses actually removed after the review, or just noted?
- Is the review performed at the stated frequency, with evidence?

A recertification that produces no removals across multiple cycles may indicate a clean environment or a cosmetic review; test by tracing a sample of users to confirm their access is appropriate, independent of the review.

### Identify and test privileged and super-user access specifically

Standard user access reports do not show privileged access, which is where the real risk lies. Identify:

- **System administrators** with broad access across the application and database;
- **Super-users** with application functions that bypass workflow (force-post, force-approve, mass-update);
- **Emergency / firefighter access** granted temporarily for support, and whether it is logged, reviewed, and revoked;
- **Database-level access** that can change data outside the application, bypassing its controls and audit trail.

Privileged access is not inherently wrong — administrators need it — but it must be tightly controlled, logged, and reviewed, and privileged users must not also hold business roles that create segregation-of-duties conflicts. Test whether privileged access is segregated from business transaction capability.

### Test authentication strength and shared-account controls

Authentication controls who can use an access right. Confirm:

- password or passphrase complexity, length, and rotation appropriate to the system's risk;
- multi-factor authentication for remote access, privileged access, and high-risk systems;
- account lockout after failed attempts;
- that shared, generic, or service accounts are either eliminated or tightly controlled, with their use attributable to individuals.

Shared accounts destroy individual accountability and are a frequent fraud route because actions cannot be traced to a person. Where shared accounts are unavoidable (some service accounts), confirm their credentials are vaulted, rotated, and their use logged.

### Test physical and environmental access where relevant

Logical access rests on physical access: anyone who can reach a server console or a finance user's unlocked workstation can often bypass logical controls. Confirm:

- physical security over data centres and server rooms;
- clean-desk and screen-lock discipline for finance users with access to sensitive systems;
- controls over removable media and local data extracts.

Physical access matters most where systems are on-premise and where finance users handle sensitive data; in fully cloud-hosted environments the physical risk shifts to the provider and is covered by the service organisation report.

### Connect access weaknesses to their effect on reliance

An access finding is not just an IT observation; it directly affects what reliance is possible:

- Weak access control over a system undermines reliance on every automated control and every report generated from that system, because the data and configuration cannot be trusted to be unchanged.
- A privileged user with business transaction capability can bypass the application controls, so those controls cannot be relied upon for the periods and populations the user could affect.
- Orphaned or excessive access raises fraud and override risk, requiring targeted substantive procedures.

State, for each significant access finding, which automated controls or data sources it affects and how the audit response changes. An access finding with no effect on the audit plan has not been thought through.

### Scope access testing to the systems that matter

Not every system needs the same depth of access testing. Focus on:

- the core financial applications (ERP, GL, revenue, procurement, treasury, payroll);
- systems that feed material data into the financial systems (time and billing, inventory, consolidation);
- end-user computing tools (spreadsheets, databases) that perform material calculations;
- interfaces and the systems that control them.

Apply lighter procedures to systems with no material financial impact. Depth should follow materiality and the extent of reliance on automated controls and system-generated data.

### Re-test across the period, not just at a point in time

Access at year-end may be clean while access during the year was not (a temporary grant that gave someone conflicting access for a quarter, then was removed before year-end). Test access changes across the period — a sample of new grants, role changes, and de-provisionings dated throughout the year — not only the year-end user list. Period-wide testing catches temporary exposures that point-in-time testing misses.

## Common Traps

- **Accepting the standard user access report as the complete picture**, missing privileged, super-user, emergency, and database-level access.
- **Testing access only at year-end**, missing temporary grants or exposures that existed during the year and were removed before the snapshot.
- **Treating periodic recertification as effective because it occurred**, without confirming it produced removals or that reviewers examined meaningful detail.
- **Missing the "mover" leakage**, where role changes accumulate access and create SoD conflicts that no one notices.
- **Allowing privileged users to hold business roles**, creating circumvention conflicts that silently defeat application controls.
- **Overlooking shared, generic, or service accounts** that destroy individual accountability.
- **Treating access findings as IT observations only**, without connecting them to the reliability of automated controls, system-generated data, and the need for targeted substantive procedures.
- **Testing every system to the same depth**, wasting effort on immaterial systems while under-testing the core financial applications.
- **Forgetting end-user computing** (critical spreadsheets) where access and version control are often weakest and most material.

## Self-Check

- Did I test the full user lifecycle — joiner, mover, leaver — including timeliness of provisioning and de-provisioning?
- Did I confirm periodic access reviews operate genuinely, with meaningful detail and actual removals, not just sign-offs?
- Did I identify and test privileged, super-user, emergency, and database-level access, including whether privileged users hold business roles?
- Did I test authentication (complexity, multi-factor for remote and privileged access, lockout) and the control of shared and service accounts?
- Did I consider physical and environmental access where systems are on-premise or users handle sensitive data?
- For each significant access finding, did I state which automated controls or data sources it affects and how the audit response changes?
- Did I scope access testing depth to materiality and reliance, focusing on core financial systems, feeds, and critical end-user computing?
- Did I test access changes across the period, not only the year-end snapshot, to catch temporary exposures?
