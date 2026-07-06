---
name: sod-conflict-identification.md
description: Use when the agent is identifying segregation of duties conflicts, mapping incompatible role combinations, analysing access rights across the initiate-authorise-record-report cycle, using role-based conflict matrices, or evaluating whether detected SoD conflicts create unacceptable risk in a financial process.
---

# Segregation of Duties Conflict Identification

Segregation of duties (SoD) is the principle that no single individual should control all stages of a transaction — initiation, authorisation, recording, and access to assets. When duties are not segregated, one person can both commit and conceal an error or fraud. Identifying SoD conflicts is therefore central to fraud risk assessment and to scoping controls testing. The recurring failure is not missing the obvious conflicts but accepting role descriptions at face value instead of mapping the actual access rights a person holds across systems, and treating a single system's clean SoD report as evidence that the environment is clean when the real risk spans multiple systems and manual workarounds.

## Core Rules

### Map conflicts against the full transaction lifecycle, not just one stage

A complete SoD analysis covers four functional responsibilities for each significant transaction class:

1. **Initiation** — starting or requesting a transaction (raising a purchase order, entering a timesheet, creating a vendor).
2. **Authorisation** — approving the transaction before it commits the entity (approving the PO, signing off the invoice, releasing the payment).
3. **Recording** — entering, posting, or adjusting the accounting record (posting to the GL, adjusting a balance, running depreciation).
4. **Asset custody / reporting** — handling the underlying asset or producing the report used to monitor it (handling cash, accessing the warehouse, running the bank reconciliation).

A conflict exists whenever one person holds two or more of these across a transaction class. Map each significant process (revenue, procurement, payroll, treasury, GL/close) against all four stages; conflicts often hide at the boundaries between stages that different teams own.

### Analyse actual access rights, not job titles or role names

Job titles and role descriptions routinely diverge from the access a person actually holds, especially in entities with long tenures, accumulated access, or weak access governance. Build the conflict analysis from:

- system-assigned roles and permissions (the authoritative source);
- any direct, ad-hoc, or "temporary" access granted outside the standard role model;
- shared or generic accounts that effectively grant access to whoever knows the password;
- manual capabilities outside the system (physical access to cheque stock, signature stamps, ability to initiate wire requests by phone).

A person whose title suggests only recording duties may, through accumulated access, also be able to initiate and authorise. The system role report is the starting point; the actual effective access is the answer.

### Use a conflict matrix tailored to the entity's processes and risks

Generic SoD matrices (vendor-master-vs-payment, JE-entry-vs-approval) are useful starting points but must be tailored. Different entities have different risk concentrations:

- a services firm's risk concentrates in time recording and revenue recognition;
- a manufacturer's concentrates in inventory and procurement;
- a bank's concentrates in treasury and account opening.

Build the matrix from the entity's significant accounts and processes, not from a generic checklist, and weight conflicts by the materiality and fraud susceptibility of the underlying flow.

### Test for cross-system conflicts, not just within-system conflicts

The most dangerous conflicts often span systems: a person who can create a vendor in the procurement system, approve invoices in the AP system, and reconcile the bank account in the treasury system has a complete conflict that no single system's SoD report would flag. Where transaction flows cross systems (common in entities with multiple ERPs, acquired entities on different platforms, or bolt-on specialist systems), consolidate access across all in-scope systems before assessing conflicts. A clean SoD report from each individual system is not evidence of a clean environment.

### Identify both inherent conflicts (by design) and circumvention conflicts (by access)

Two distinct categories of conflict exist:

- **Inherent conflicts** — built into the process design because the entity is too small or too centralised to separate duties (the classic small-entity problem where the bookkeeper does everything).
- **Circumvention conflicts** — the design separates duties, but a privileged user (often an IT administrator or super-user) can bypass the separation by posting outside workflow, force-approving, or accessing functions outside their role.

Both matter. Inherent conflicts are usually known and may be mitigated by compensating controls (owner review of all activity by the bookkeeper). Circumvention conflicts are often invisible to management and are a common fraud route, because they defeat controls that everyone assumes are working.

### Assess the risk significance of each conflict, not just its existence

Not every conflict is equally serious. Evaluate each identified conflict by:

- the materiality and liquidity of the assets exposed (cash and near-cash rank highest);
- whether the conflict enables concealment as well as commission (a conflict over both posting and reconciliation lets the person hide their own misdeed);
- the volume and value of transactions in the conflicted flow;
- the presence and strength of compensating controls;
- the integrity and oversight of the individual holding the conflict (relevant but never sufficient on its own).

Rank conflicts and focus mitigation and testing on the highest-risk combinations, while documenting why lower-risk conflicts are accepted or covered by compensating controls.

### Re-assess SoD after organisational change

Conflicts are not static. Reorganisations, acquisitions, staff departures, system migrations, and role re-designs all shift the access landscape. A conflict analysis that was clean at the start of the period may be stale by year-end if, for example, a key reviewer left and their duties were absorbed by someone who now holds incompatible access. Confirm the analysis reflects access as of (or throughout) the period under audit, not a prior point in time.

### Connect SoD findings to fraud risk and control deficiencies

SoD conflicts are not just control design observations; they are direct inputs to fraud risk assessment. A conflict that lets one person commit and conceal fraud in a material flow raises fraud risk and warrants targeted procedures (focused substantive testing, surprise procedures, review of override). Document each significant conflict's effect on assessed risk and on the planned response, rather than leaving it as a standalone access-management finding.

## Common Traps

- **Accepting role descriptions or titles instead of actual effective access.** Accumulated, temporary, or ad-hoc access routinely creates conflicts the role model does not show.
- **Treating a clean within-system SoD report as evidence of a clean environment.** Cross-system conflicts are the most dangerous and the most missed.
- **Using a generic SoD matrix without tailoring** to the entity's significant accounts and fraud-susceptible flows.
- **Missing circumvention conflicts** held by privileged or super-users who can bypass designed separation.
- **Listing conflicts without assessing their risk significance**, leading either to alarm fatigue or to serious conflicts being under-prioritised.
- **Assuming a known, long-standing inherent conflict is acceptable** without testing the compensating control that is supposed to mitigate it.
- **Letting a stale SoD analysis stand** after reorganisations, departures, or system changes that have shifted access.
- **Treating SoD as an access-management issue only**, missing its direct connection to fraud risk and the need for targeted fraud procedures; **Overlooking shared, generic, or service accounts** that effectively grant conflicting access to whoever uses them, often without individual accountability

## Self-Check

- For each significant transaction class, did I map conflicts across all four stages — initiation, authorisation, recording, and asset custody/reporting?
- Did I build the conflict analysis from actual effective access (system roles plus ad-hoc, temporary, shared, and privileged access), not from job titles?
- Is the conflict matrix tailored to this entity's significant accounts and fraud-susceptible flows, not a generic checklist?
- Did I consolidate access across all in-scope systems to catch cross-system conflicts that no single system's report would show?
- Did I identify both inherent conflicts (by design) and circumvention conflicts (via privileged access), and treat the latter as a fraud risk?
- For each significant conflict, did I assess risk significance based on asset materiality, concealment potential, volume, and compensating controls — and rank them?
- Does my SoD analysis reflect access throughout the audit period, accounting for reorganisations, departures, and system changes?
- Have I connected each significant conflict to fraud risk assessment and designed targeted procedures, rather than treating SoD as a standalone access finding?
