---
name: data-lifecycle-and-retention-management.md
description: Use when the agent is designing data retention schedules, managing the data lifecycle from creation through disposal, implementing legal hold and preservation, defining deletion and anonymisation methods, or resolving conflicts between retention obligations and data minimization or deletion requests.
---

# Data Lifecycle and Retention Management

Data has a lifecycle: it is created or collected, used, stored, possibly archived, and eventually deleted or anonymised. Governing that lifecycle — deciding how long each category of data is retained, when and how it is disposed, and how retention interacts with legal obligations, data subject rights, and business needs — is a core governance discipline. The dominant failure mode is indefinite retention: data accumulates because deletion is hard, because "we might need it," or because no one owns the decision. Indefinite retention violates minimization principles, inflates breach exposure, and complicates every regulatory inquiry. This skill addresses the judgment involved in designing and enforcing a defensible data lifecycle and retention program.

## Core Rules

### Build retention schedules from legal obligations, business needs, and risk

A retention schedule assigns a retention period to each data category based on three inputs:

- **Legal obligations**: laws or regulations that mandate retention for a minimum period (tax records, employment records, AML records, clinical trial data) or prohibit retention beyond a maximum;
- **Legitimate business needs**: operational requirements that justify retention (customer relationship management, warranty support, analytics), balanced against minimization;
- **Risk**: the cost and exposure of retaining data longer than necessary (breach surface, regulatory scrutiny, eDiscovery burden).

Document the basis for each retention period. A period with no documented rationale is arbitrary and indefensible.

### Define the retention trigger, not just the period

A retention period is meaningless without a trigger — the event or date from which the period runs. Define the trigger explicitly:

- For transaction data: the transaction date or the relationship-end date;
- For employment data: the termination date;
- For contract data: the contract-end or final-performance date;
- For AML records: the transaction date or the relationship-end date (with the legally specified period).

A retention period that runs from an undefined or inconsistently recorded trigger is unenforceable in practice.

### Specify the disposal method: deletion, anonymisation, or archival

At the end of the retention period, define what happens:

- **Secure deletion**: removal from all live systems, backups (within backup cycles), and logs;
- **Anonymisation**: irreversible transformation so the data no longer identifies individuals, after which it may be retained for analytics;
- **Archival**: transfer to long-term storage for a defined further period (for legal hold or historical purposes), with restricted access.

Specify the method and the technical process. "Delete" that leaves data in backups or archived copies is not deletion. Document the disposal and retain a disposal record.

### Reconcile retention with data subject rights to erasure

Retention schedules and the right to erasure can conflict. The right to erasure is not absolute: data may be retained despite an erasure request where retention is necessary for legal obligations, legal claims, or other specified grounds. When an erasure request arrives:

- Assess whether a retention obligation or legal hold requires continued retention;
- If no obligation applies, delete the data across systems;
- If an obligation applies, restrict processing but retain, and document the basis.

Do not use retention schedules as a blanket excuse to deny erasure; apply the specific exemption.

### Manage legal hold as a suspension of routine disposal

When litigation, investigation, or regulatory inquiry is anticipated or underway, a legal hold suspends routine deletion for relevant data. Manage legal hold through:

- A defined issuance process (counsel authorises, scope defined);
- Custodian notification and acknowledgement;
- Technical suspension of deletion for the scoped data;
- Release when the hold is no longer needed, after which normal retention resumes.

Failure to implement legal hold leads to spoliation (destruction of evidence) with serious legal consequences. Conversely, indefinite "perma-holds" that are never released defeat the retention program.

### Enforce retention through automation, not manual goodwill

Manual deletion at scale is unreliable. Enforce retention through:

- Automated deletion or archival at the end of the retention period;
- System-level retention policies (email, collaboration platforms, databases);
- Periodic bulk purges of data past retention;
- Backup rotation that does not defeat deletion.

Where automation is not feasible for a data category, define a periodic manual purge process with owner accountability and verification.

### Address backups, archives, and shadow copies

Data exists in backups, archives, snapshots, logs, analytics extracts, and shadow systems. A retention program that deletes from the live system but leaves data in backups is incomplete. Address:

- Backup retention cycles (so deleted data ages out of backups within a defined period);
- Archive policies (so archived data is also subject to retention);
- Analytics and data warehouse copies (so derived data is also purged or anonymised);
- Shadow systems and local copies (through DLP and governance).

True disposal requires addressing all copies.

### Review and update retention schedules periodically

Retention schedules are not static. Laws change, business needs evolve, and new data categories emerge. Assign ownership for periodic review (at least annually) of the retention schedule, and update periods, triggers, and methods as required. A schedule written five years ago and never reviewed is stale.

## Common Traps

### Indefinite retention because deletion is operationally difficult

The most common failure. Data accumulates because no one implemented deletion. Operational difficulty is not a legal basis; it is a governance gap to be remediated.

### Retention periods with no documented basis

A period chosen by convention or guess, with no legal, business, or risk rationale, is arbitrary. Document the basis so it can be defended and updated.

### Deleting from live systems but leaving backups and shadow copies

"Deleted" data that persists in backups, archives, or analytics stores is not truly deleted. Address all copies in the disposal method.

### Using retention schedules to deny erasure requests blanketly

Retention obligations are specific. Applying them as a blanket excuse to refuse erasure ignores the specific exemptions and violates the data subject's rights where no obligation applies.

### Legal holds that are never released

A hold issued for a matter that concludes but is never released becomes a permanent retention override, defeating the program. Manage hold release as actively as hold issuance.

### Retention schedules that are never reviewed or updated

A schedule that does not reflect current law, business needs, or data categories drifts into inaccuracy. Assign review ownership and cadence.

## Self-Check

- Is each data category assigned a retention period based on documented legal obligation, business need, and risk assessment?
- Is the retention trigger (the event or date from which the period runs) explicitly defined and consistently recorded?
- Is the disposal method (deletion, anonymisation, archival) specified with the technical process, including handling of backups and derived copies?
- Are erasure requests reconciled with retention obligations, applying specific exemptions rather than blanket refusal?
- Is legal hold managed through issuance, custodian notification, technical suspension, and active release?
- Is retention enforced through automation (system policies, bulk purges, backup rotation) rather than relying on manual goodwill?
- Are backups, archives, analytics extracts, and shadow copies addressed in the disposal process?
- Is the retention schedule reviewed and updated periodically with assigned ownership?
