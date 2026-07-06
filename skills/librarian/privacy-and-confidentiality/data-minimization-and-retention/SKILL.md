---
name: data_minimization_and_retention.md
description: Use when the agent is deciding what patron or operational data a library service should collect, setting or reviewing retention periods for circulation records, logs, computer sessions, and analytics, configuring systems to minimize collection and automate deletion, auditing existing data stores for over-collection, aligning retention with privacy policy and law, or evaluating whether collected data is justified by a genuine operational need.
---

# Data Minimization And Retention

Every piece of data a library holds is a liability. It can be breached, subpoenaed, misused, or accidentally exposed, and in a library it can also reveal what someone read, searched, or studied, which is exactly the information the profession is ethically bound to protect. Data minimization is the practice of collecting only what a service genuinely needs, and retention is the practice of keeping it only as long as that need exists. Together they are the most powerful privacy protections available, because data that was never collected cannot leak, and data that has been deleted cannot be disclosed. Technical security matters, but minimization and retention do the work of making the target smaller and the exposure window shorter.

The judgment problem is that library systems collect data by default, and the path of least resistance is to keep everything indefinitely. Vendors log searches and sessions because they can. Analytics tools track behavior because tracking is the default. Backups retain deleted records because no one planned otherwise. Staff hesitate to delete because the data might be useful someday. The result is libraries that accumulate detailed records of patron activity far beyond any operational need, turning routine systems into rich surveillance archives waiting for a breach or a legal demand. The agent's job is to challenge default collection, justify every data type against a real operational need, set and enforce short retention, and recognize that over-retention is itself a privacy harm that may require escalation when it conflicts with legal obligations or institutional pressure.

Use this skill when deciding what to collect, setting or reviewing retention periods, configuring systems to minimize and delete, auditing data stores, or aligning retention with policy and law. The goal is to prevent the agent from collecting by default, retaining indefinitely, treating deletion as risky, or allowing retention to drift away from the privacy policy.

## Core Rules

### Treat Every Data Point As A Liability

The starting point of minimization is recognizing that data is not an asset to be hoarded but a risk to be justified.

Liarity mindset:

- each record is a potential breach, subpoena target, or chilling record of reading;
- data that is never collected cannot leak;
- data that is deleted cannot be disclosed;
- the safest data is the data that does not exist;
- collection must be justified by need, not by possibility.

When the default question becomes "why do we need this?" rather than "why not keep it?", retention decisions change.

### Justify Each Data Type Against A Genuine Operational Need

Collect only what the service requires to function. Require a purpose for every data type.

Justification test:

- what specific service or function does this data enable?
- is the function legitimate and necessary?
- is there a less identifying way to achieve the same function?
- is the data needed in identifiable form, or could it be aggregated or anonymized?
- if no clear need exists, do not collect.

Apply this test to registration fields, circulation records, computer logs, search logs, analytics, and facility access data. Eliminate collection that survives no legitimate purpose.

### Minimize Collection At The Source

The most effective protection is not collecting data in the first place. Configure systems to minimize at the point of generation.

Source minimization:

- do not log searches or browsing beyond technical necessity;
- avoid retaining circulation history after items are returned and fines cleared;
- limit registration to fields genuinely needed;
- configure analytics to aggregate and avoid individual tracking;
- prefer systems designed for minimal collection;
- avoid collecting data for unspecified future uses.

Minimization at the source eliminates entire categories of risk before they arise.

### Set Short, Enforced Retention For Each Data Type

Retention should be as short as the service allows. Define and enforce limits per data type.

Retention guidance by type:

- circulation and borrowing records: purge once the transaction is complete and fines resolved;
- computer session and login logs: hours to a few days at most;
- search and browsing logs: minimize or avoid, with very short retention if kept;
- facility access and camera footage: only as long as needed for security, typically days to weeks;
- analytics: aggregated and short-retained, avoiding individual profiles;
- registration data: retain while the account is active, purge after inactivity.

Indefinite retention is the default of many systems and is a privacy failure. Set explicit, short limits.

### Automate Deletion So It Actually Happens

Manual deletion is unreliable. Automate retention enforcement.

Automation:

- configure systems to purge records on schedule;
- use scripts or system features to enforce retention;
- monitor that automated deletion is running and succeeding;
- alert on deletion failures so they are corrected;
- document the automation so it survives staff turnover.

Retention limits that depend on someone remembering to delete are not real limits. Automate and monitor.

### Align Backups With Retention

Backups often retain deleted data indefinitely, undermining retention policy. Plan for this explicitly.

Backup alignment:

- define how retention applies to backup copies;
- ensure deleted data is removed from backups within a defined period where feasible;
- secure backups with the same access controls as production;
- avoid backup schemes that preserve deleted records indefinitely;
- document the relationship between primary retention and backup retention.

A retention policy that primary systems honor but backups ignore is incomplete. Align both.

### Audit Existing Data Stores For Over-Collection

Libraries accumulate data stores over time. Audit them to find and reduce over-collection.

Audit practices:

- inventory all systems and the data they hold;
- identify data collected without a current purpose;
- find data retained beyond policy limits;
- locate orphaned or forgotten data stores;
- delete or reduce what is not justified;
- document findings and remediation.

Over-collection often hides in forgotten logs, old systems, and vendor exports. Periodic audits surface and correct it.

### Prefer Aggregation And Anonymization Where Possible

Where a function needs insight but not individual data, aggregate or anonymize.

Aggregation and anonymization:

- use aggregate statistics instead of individual records for reporting;
- anonymize or pseudonymize data where individual identity is not needed;
- recognize that pseudonymized data may still be re-identifiable and treat it carefully;
- avoid building individual behavioral profiles;
- prefer trends and counts over individual tracking.

Aggregation provides operational insight without building a surveillance record of individuals.

### Align Retention With Policy, Law, And Ethics

Retention must be consistent with the privacy policy, applicable law, and professional ethics.

Alignment:

- ensure retention settings match the published privacy policy;
- comply with legal retention obligations where they exist, but do not exceed them;
- recognize professional ethics may require shorter retention than law allows;
- reconcile conflicts between legal mandates to retain and ethical duties to minimize;
- document the basis for each retention decision.

Inconsistency between policy, system settings, and law creates confusion and risk. Reconcile them deliberately.

### Document And Communicate Retention Decisions

Retention decisions should be transparent and traceable.

Documentation:

- record the rationale for each retention period;
- document the systems and automation that enforce it;
- communicate retention to patrons in the privacy policy;
- maintain records of audits and remediation;
- review and update retention as needs and technology change.

Documented decisions survive staff turnover and provide accountability.

## Escalation Guidance

Minimization and retention decisions can conflict with legal mandates, institutional pressure, or vendor defaults. Escalate in these situations:

- Any legal mandate to retain data longer than policy or ethics would prefer: escalate to legal counsel and leadership to reconcile the conflict and document the basis.
- Any institutional pressure to collect or retain more data for analytics, marketing, or security: escalate to weigh the privacy cost against the claimed benefit.
- Any discovery of significant over-collection or indefinite retention of patron reading data: escalate to leadership for prompt remediation, as this is an active privacy harm.
- Any vendor system that cannot be configured to meet retention policy: escalate to procurement and leadership to renegotiate or replace.
- Any conflict between backup retention and deletion obligations: escalate to systems and legal to align the two.
- Any uncertainty about whether a data type may be retained: escalate to privacy officer or legal counsel before keeping it.

When retention and minimization are in doubt, the default should be less data and shorter retention, with escalation to confirm.

## Common Traps

### Collecting By Default

Systems collect more than needed. Configure for minimal collection and justify each data type.

### Indefinite Retention

Keeping data forever is the easy default and a privacy failure. Set and enforce short retention.

### Treating Deletion As Risky

Hesitation to delete leads to over-retention. The greater risk is keeping data that serves no purpose.

### Backups That Ignore Retention

Backups retaining deleted data indefinitely undermine the policy. Align backup retention with primary retention.

### Over-Collection Hidden In Old Systems

Forgotten logs and stores accumulate data. Audit periodically to find and reduce them.

### Building Individual Behavioral Profiles

Tracking individuals for analytics creates surveillance records. Prefer aggregation and anonymization.

### Retention Inconsistent With Policy And Law

Gaps between policy, system settings, and legal obligations create confusion and risk. Reconcile them.

### Manual Deletion That Never Happens

Retention limits that depend on memory are not real. Automate and monitor deletion.

## Self-Check

- Is every collected data type justified against a genuine operational need, with less-identifying alternatives considered?
- Is collection minimized at the source, with systems configured to avoid logging searches, sessions, and history beyond necessity?
- Are retention periods short, explicit, and defined per data type, with no indefinite retention?
- Is deletion automated, monitored for success, and alerted on failure so retention is actually enforced?
- Is backup retention aligned with primary retention so deleted data is removed from all copies within a defined period?
- Have existing data stores been audited for over-collection, orphaned data, and retention violations, with remediation documented?
- Is aggregation or anonymization preferred over individual tracking wherever operational insight does not require identity?
- Are retention decisions consistent with the privacy policy, applicable law, and professional ethics, with conflicts reconciled and documented?
- Are retention rationale, automation, and audits documented and communicated to patrons?
- Are escalation paths defined for legal retention conflicts, pressure to over-collect, discovered over-retention, and vendor limitations?
