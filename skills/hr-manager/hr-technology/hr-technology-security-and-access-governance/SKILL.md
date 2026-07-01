---
name: hr-technology-security-and-access-governance.md
description: Use when the agent is configuring access controls for HR systems, managing sensitive employee data permissions, conducting HR system access reviews, responding to HR data breaches, or designing security protocols for people data platforms.
---

# HR Technology Security and Access Governance

HR systems concentrate the most sensitive data in the organization: compensation, performance ratings, medical and accommodation records, disciplinary history, social security numbers, and background check results. A breach or misuse of this data carries legal liability (GDPR, CCPA, HIPAA-intersecting records), regulatory exposure, and severe reputational damage. Yet access governance is often treated as an IT concern rather than an HR accountability. HR must own the question of who should see what people data, because only HR understands the sensitivity gradients and the business justification for access. The discipline is to apply least-privilege consistently, to audit access regularly, and to design for the reality that the greatest data risk is often insider misuse, not external attack.

## Core Rules

### Apply Least-Privilege as the Default, Justify Exceptions

Every user should have the minimum access required to perform their role — no more. A manager needs to see their direct reports' data, not the entire department's. An HRBP needs access to their assigned business unit, not the whole company. A recruiter needs applicant data, not current employee compensation. Define role-based access templates that encode least-privilege by default, and require documented business justification for any exception. Review exception grants periodically and revoke them when the justification expires. The instinct to grant broad access to avoid access-request friction is the single greatest source of over-privileged users and insider data risk.

### Classify Data by Sensitivity and Apply Tiered Controls

Not all people data carries the same risk. Create a data classification scheme that tiers sensitivity: public (job postings), internal (org chart), confidential (compensation, performance), and restricted (medical/accommodation records, SSNs, background checks, investigation files). Apply controls that scale with sensitivity: restricted data may require named-individual access, audit logging of every view, and encryption at rest. Critically, segregate the most sensitive categories — medical and accommodation records, investigation files — into separate access domains so that routine HR access does not incidentally expose them. An HRBP who can see performance and compensation should not automatically see medical accommodation documentation.

### Conduct Regular Access Reviews with Business Owners

Access creep is inevitable: employees change roles, managers transfer, contractors' projects end, and access granted for a temporary need persists indefinitely. Conduct quarterly or semi-annual access reviews where business owners (HR leadership, HRBP managers, functional leaders) formally certify that each user's access is still appropriate. Make these reviews meaningful — not a rubber-stamp — by providing reviewers with access patterns (what the user actually viewed) and flagging anomalies. Remove access promptly upon role change or termination; delayed deprovisioning is a leading cause of data exfiltration by departing employees.

### Design Audit Logging for Accountability, Not Just Compliance

Every access to and change in sensitive employee data should be logged with user, timestamp, action, and record accessed. But logs are useless if no one reviews them. Design active monitoring for high-risk patterns: a manager viewing records outside their team, bulk exports of compensation data, access to a colleague's medical record, after-hours access spikes. Configure alerts for these patterns and assign someone to investigate them. Audit logs serve accountability (deterrence and investigation) only when their content is actively monitored, not merely stored for post-incident forensics.

### Manage Vendor and Third-Party Access

HR technology vendors, implementation consultants, and benefits carriers often require system access, and this access is a significant and undermanaged risk. Grant third-party access only for defined periods and scoped to specific data, with expiration dates enforced. Require vendors to demonstrate their own security practices (SOC 2, data processing agreements) before granting access. Monitor third-party access patterns and revoke promptly when the engagement ends. A consultant who retains HRIS access after implementation completion is a breach waiting to happen.

### Plan for Breach Response Before the Breach

When sensitive employee data is exposed, the first hours determine whether the incident becomes a contained event or a public crisis. Develop and rehearse an incident response plan specific to HR data: who leads, how the scope is assessed, when legal and communications engage, what regulatory notification obligations apply (state breach laws, GDPR 72-hour window), and how affected employees are informed. Tabletop the plan annually. The organizations that handle breaches well are those that rehearsed; those that handle them poorly assumed it would not happen to them.

### Address Insider Risk Deliberately

The greatest threat to HR data is not the external hacker but the trusted insider: the HR coordinator browsing a colleague's compensation, the manager reviewing a former report's medical records, the departing employee exporting a contact list. Design controls that assume insider curiosity and malice: segregation of duties (no single person can both create and approve a compensation change), monitoring for anomalous access patterns, and a culture where inappropriate access has consequences. Train all HR staff that access to data is not permission to view it — accessing data without business need is a policy violation even if the system permits it.

## Common Traps

### Broad Access Granted for Convenience

Granting company-wide or department-wide access to avoid the friction of individual access requests creates a population of over-privileged users. The cost of a single breach dwarfs the cumulative friction of proper access management. Enforce least-privilege even when it is inconvenient.

### Co-Mingling Sensitive Data Categories

When medical, accommodation, investigation, and compensation data share the same access domain, anyone with routine HR access can view the most sensitive categories. Segregate by sensitivity tier. An employee's medical accommodation documentation should not be visible to the same people who can view their performance rating.

### Access Reviews as Rubber Stamps

Quarterly access reviews that managers approve without examination provide false assurance. Make reviews meaningful by surfacing access patterns, flagging anomalies, and requiring specific recertification of sensitive access. Track review quality and hold reviewers accountable for oversights.

### Delayed Deprovisioning Upon Termination

When an employee departs, their system access should be revoked concurrent with their departure — not days later. Build deprovisioning into the offboarding workflow with automated triggers. The window between departure and deprovisioning is when exfiltration occurs.

### Assuming the Vendor's Security Is Sufficient

A vendor's SOC 2 report demonstrates their controls, not yours. You remain accountable for the data you entrust to them. Review vendor security practices, negotiate data protection terms, and monitor their access. Do not outsource accountability.

### No Monitoring of Audit Logs Until an Incident Occurs

Logs collected but never reviewed provide only post-incident forensics, not prevention. Configure active monitoring and alerting for high-risk access patterns, and assign ownership for investigation. The value of logging is in the watching, not the storing.

## Self-Check

- Does every user have least-privilege access by default, with documented justification and periodic review for exceptions?
- Have I classified people data by sensitivity tier and applied controls that scale, with the most sensitive categories segregated into separate access domains?
- Do I conduct meaningful access reviews where business owners recertify access with access-pattern context, not rubber-stamp approvals?
- Are audit logs actively monitored for high-risk patterns (out-of-team access, bulk exports, after-hours spikes), with assigned investigation ownership?
- Is third-party and vendor access time-bound, scoped, monitored, and promptly revoked upon engagement end?
- Do I have a rehearsed HR-data-specific incident response plan covering scope assessment, legal/communications engagement, regulatory notification, and employee communication?
- Have I designed controls (segregation of duties, anomaly monitoring, consequence culture) that address insider risk, not just external threats?
- Is deprovisioning automated and concurrent with termination, with no access window post-departure?
