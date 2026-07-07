---
name: audit_logging_and_compliance_data.md
description: Use when the agent is building audit logs, compliance trails, tamper-evident records, access-history features, change-tracking, or evidence-collection for security review, legal hold, eDiscovery, or regulatory compliance (GDPR, HIPAA, SOC 2, PCI DSS); deciding what security-relevant events to record, how to store them so they cannot be silently altered, how long to retain them, how to separate audit data from operational logs, or how to record who did what without leaking secrets or excessive personal data. Covers audit event design, append-only and WORM storage, integrity protection, retention and legal hold, privacy obligations on audit data, and the difference between operational logs and audit trails.
---

# Audit Logging And Compliance Data

An audit trail is the record that proves what happened, who did it, and when. It exists for the moment something goes wrong — a breach, a dispute, a regulatory inquiry, an insider threat investigation — and its value is realized only under pressure, long after the events it records. The defining failure of audit logging is that it is built casually when nothing is wrong and found inadequate when it is needed most. A team logs some events to the application log stream, mixes them with debug output, overwrites them on deploy, expires them after a week, and discovers during an incident that the record of who accessed the compromised records is gone, incomplete, or was never written. Compliance data has the same property: it is collected for a future obligation, and the cost of collecting it incorrectly is paid when that obligation is enforced and the data cannot answer the question.

Agents tend to conflate audit logging with operational logging, treating an audit event as just another log line. They are different in purpose, in storage, in integrity requirements, and in retention. An operational log is for debugging a running system; it is mutable, short-lived, and disposable. An audit trail is for accountability; it must be complete for the events that matter, protected against alteration, retained for the period the obligation requires, and queryable by the dimensions an investigation will use. The judgment problem is designing audit data as evidence: deciding what counts as an auditable event, recording it with the fields an investigator needs, storing it where it cannot be silently changed, retaining it as long as required, and protecting the privacy of the individuals it records. This skill covers the discipline of building audit and compliance data that survives to be useful when it is needed.

## Core Rules

### Separate Audit Trails From Operational Logs

Operational logs and audit trails serve different purposes and must not be the same stream. Mixing them guarantees that audit events are lost when logs roll over, diluted by noise, or overwritten by a deploy.

- **Audit data goes to a dedicated, append-only store.** A separate table, a separate log stream, a WORM (write-once-read-many) bucket, or a dedicated audit service. It is not a debug log that rotates hourly.
- **Operational logs are for the running system; audit trails are for accountability.** An operational log can be deleted freely; an audit record may be legally required to survive for years. Conflating them means either over-retaining operational noise or under-retaining audit evidence.
- **Route audit events to the audit store directly, not via the general logger.** A `logger.info("user deleted record")` is an operational log line, not an audit record. Audit events have a defined schema, a dedicated sink, and integrity protection.

### Define What Counts As An Auditable Event

Not every action needs an audit record, but the security- and compliance-relevant ones must be identified and consistently captured. Under-logging leaves gaps; over-logging drowns the signal and inflates storage. Define the auditable event set deliberately:

- **Authentication and authorization events:** login success and failure, logout, MFA challenge and result, password reset, privilege escalation, role or permission grant and revoke, assumption of another identity (impersonation, support access).
- **Access to sensitive data:** reads, exports, and bulk queries of protected records (PII, PHI, financial data, credentials), especially by privileged users or support staff.
- **State-changing operations on protected resources:** create, update, delete of records, configurations, access policies, integrations, or billing settings — particularly destructive or irreversible actions.
- **Configuration and security-setting changes:** changes to authentication settings, feature flags affecting security, IP allow-lists, API key creation and revocation, webhook endpoint changes.
- **Administrative and system actions:** data restores, manual database edits, feature-flag overrides, access to production by an operator.

For each, decide whether the audit need is "did this happen" (an event log suffices) or "exactly what changed" (a before/after diff is required).

### Capture The Fields An Investigation Will Need

An audit record is useful only if it answers the questions an investigation will ask. Design the schema around those questions, and capture the context at the time of the event — it cannot be reconstructed later.

- **Who:** the authenticated actor (user id, service account, API key id), and, where relevant, the effective actor (impersonating admin, delegated scope). Do not record only a username that can be renamed; record the stable id.
- **What:** the action, the resource type, and the specific resource identifier. Use stable, enumerable action names (`record.export`, `policy.update`), not free text.
- **When:** a precise, monotonic timestamp, ideally with a sequence number to order concurrent events. Wall-clock time alone is insufficient under clock skew or concurrent writes.
- **Outcome:** success or failure, and for failures, the reason (denied, not found, error). A failed access attempt is often more interesting to an investigation than a successful one.
- **Source:** the origin of the request — IP address, client, request id, session id — enough to correlate with other systems and to detect anomalous access patterns.
- **Before/after for changes:** for mutations, capture the previous and new values of the changed fields, so the record shows what was altered, not just that something was.

### Protect Audit Data Integrity Against Tampering

An audit trail that an attacker (or a malicious insider) can alter silently is not evidence. Integrity protection must be designed in, because the threat model for audit data includes privileged actors who can access the database.

- **Make audit records append-only and immutable.** The application must not have an update or delete path for audit records. Enforce this at the storage layer (WORM storage, append-only tables, database permissions that deny UPDATE/DELETE on the audit table to the application role).
- **Chain or sign records to detect tampering.** A hash chain (each record includes the hash of the previous record) or a periodic signature over a batch of records lets a reviewer detect that a record was inserted, removed, or altered after the fact. This raises the bar from "edit one row" to "recompute the entire chain undetected."
- **Separate write and read paths.** The service that writes audit records should not be the same role that can read or administer them, where feasible. A compromised application should not be able to read or alter its own audit trail.
- **Replicate audit data to a separate, hardened location.** A copy in a separate account, region, or system that the production environment cannot write to provides recovery if the primary is compromised.

### Retain For The Required Period, And Honor Legal Hold

Retention is where audit data meets legal obligation. Retain too little and you cannot answer a regulatory inquiry; retain too long and you may violate data-minimization or privacy law. Retention must be deliberate, not whatever the default log rotation happens to be.

- **Know the retention obligations that apply.** Regulations and frameworks specify minimums: PCI DSS requires one year of audit logs (and recommends three); HIPAA requires six years; SOC 2 expects retention consistent with the stated policy; GDPR imposes minimization that caps indefinite retention of personal data. State the applicable requirement explicitly.
- **Set retention by record type, not globally.** Security audit events may need years; operational debug logs may need weeks. A single global retention setting will over-retain one and under-retain the other.
- **Implement legal hold.** When litigation, investigation, or regulatory inquiry is anticipated, the relevant audit data must be preserved beyond its normal retention. A legal-hold mechanism that prevents deletion of held records is required for compliance.
- **Plan for secure deletion at end of retention.** Data that is no longer required and not under hold should be deleted on schedule, both to honor minimization and to reduce the volume of data exposed in a future breach.

### Respect Privacy Obligations On Audit Data

Audit data often contains personal data — the very data that makes it useful also subjects it to privacy law. Recording who accessed what can itself create a privacy obligation.

- **Do not record secrets or unnecessary sensitive data in audit events.** Passwords, tokens, full request bodies, payment card numbers, and contents of protected documents should not appear in audit records. Record that access occurred and to which resource, not the sensitive content itself.
- **Apply data-subject rights to audit data where required.** Some regulations grant individuals the right to know what data is held about them, including in logs. Determine whether audit data is in scope and how to honor access or erasure requests without destroying the audit integrity the record exists to provide.
- **Minimize and pseudonymize where possible.** Where the actor can be referenced by a stable internal id rather than a directly identifying value, prefer the id. The mapping lives in a controlled directory; the audit record references it.

## Common Traps

### Treating Audit Events As Ordinary Log Lines

Writing audit events to the application log stream, so they rotate, get overwritten, or are drowned in debug noise, and are gone when needed. Use a dedicated, append-only audit store.

### Recording The Action But Not The Outcome Or Context

Logging "user accessed record 42" without success/failure, source IP, or the before/after of a change, leaving an investigation unable to reconstruct what actually happened. Capture the full investigative context at event time.

### Mutable Audit Storage

Storing audit records in a table the application can UPDATE or DELETE, so a compromised or malicious insider can alter the trail undetected. Make audit storage append-only and integrity-protected (hash chain or signatures).

### Retention Set By Default Log Rotation

Letting audit data expire after a week or a month because that is the operational log default, then being unable to answer a regulatory inquiry covering a longer period. Set retention deliberately per the applicable obligation.

### Over-Recording Sensitive Content

Dumping full request bodies, secrets, or document contents into audit records "for completeness," creating a second sensitive data store that expands breach scope and violates minimization. Record access metadata, not sensitive content.

### No Legal Hold Mechanism

Having no way to preserve records beyond normal retention when litigation or inquiry is anticipated, leading to spoliation. Implement legal hold before it is needed.

### Trusting Wall-Clock Timestamps Alone

Ordering concurrent events by wall-clock time, which is unreliable under clock skew, so the audit trail shows an impossible or misleading sequence. Use a monotonic sequence number or a central sequencer where ordering matters.

### Audit Records That Cannot Be Queried Investigatively

Storing audit data in a format optimized for writing but impossible to query by actor, resource, or time range, so an investigation must scan everything. Design the store and indexes around the investigative queries.

## Self-Check

- [ ] Audit trails are stored separately from operational logs, in a dedicated append-only store (separate table, WORM bucket, or audit service), and are not subject to ordinary log rotation or deploy-time overwrite.
- [ ] The set of auditable events is defined deliberately and covers authentication/authorization, access to sensitive data, state-changing operations on protected resources, security-setting changes, and administrative actions, with a decision per event on whether before/after detail is required.
- [ ] Each audit record captures who (stable actor id, effective/delegated actor where relevant), what (enumerable action, resource type and id), when (precise timestamp plus sequence number for ordering), outcome (success/failure with reason), source (IP, client, request/session id for correlation), and before/after values for mutations.
- [ ] Audit storage is append-only and immutable at the storage layer (WORM, deny UPDATE/DELETE to the application role), and tamper-evidence is provided by a hash chain or periodic signatures so insertion, deletion, or alteration is detectable.
- [ ] Retention is set per record type based on the applicable regulatory or contractual obligation (e.g., PCI one year, HIPAA six years), a legal-hold mechanism preserves records beyond normal retention when required, and secure deletion occurs at end of retention.
- [ ] No secrets, tokens, full request bodies, payment data, or protected document contents are recorded in audit events; audit records contain access metadata and resource identifiers, not sensitive content.
- [ ] Privacy obligations on audit data (data-subject access/erasure rights, minimization, pseudonymization) are assessed and handled, with stable internal ids preferred over directly identifying values where feasible.
- [ ] Audit data is queryable by the investigative dimensions (actor, resource, time range, action), with indexes or a store design that supports the queries an investigation or compliance review will run.
