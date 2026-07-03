---
name: incompatible-duties.md
description: Use when the agent is evaluating specific incompatible duty combinations beyond classical segregation of duties, assessing conflicts in master data maintenance, reconciliation versus transaction processing, custody versus recording, IT administration versus business functions, or deciding which duty combinations are genuinely incompatible versus merely suboptimal.
---

# Incompatible Duties

Beyond the classical segregation-of-duties framing (initiate, authorise, record, custody), real processes contain specific incompatible duty combinations that are easy to overlook because each individual duty looks legitimate. A person who can both maintain the vendor master and approve payments, or who can both post journal entries and reconcile the account, holds a combination that lets them create and conceal misstatement. The discipline is to identify these combinations by name, evaluate whether the combination is genuinely incompatible or merely suboptimal, and avoid both over-flagging (treating every adjacency as a conflict) and under-flagging (missing the combinations that actually enable concealment).

## Core Rules

### Identify incompatible combinations by their concealment potential, not just their adjacency

A duty combination is genuinely incompatible when holding both duties lets one person both commit an error or fraud *and* conceal it. The defining feature is concealment. Test each candidate combination with the question: if this person made an unauthorised or erroneous entry using duty A, could they use duty B to hide it from the normal monitoring? If yes, the combination is incompatible. If the two duties are merely adjacent but neither enables concealment of the other, the combination may be suboptimal but is not a true SoD conflict. Concentrate on concealment-enabling combinations.

### Evaluate the high-frequency incompatible combinations explicitly

Certain combinations recur across entities and are almost always incompatible. Confirm each is assessed:

- **Master data maintenance + transaction processing**: creating or editing a vendor, customer, employee, or bank account and then transacting with it. This is the single most exploited combination in payment and revenue fraud.
- **Transaction initiation + authorisation**: raising and approving the same request (the classic approval conflict).
- **Recording + reconciliation**: posting entries to an account and then reconciling that account, which lets the poster hide their own errors or fabrications.
- **Custody + recording**: handling an asset (cash, inventory, securities) and recording its movement, which enables direct theft matched by false records.
- **IT administration + business function**: an IT administrator who also holds a business role (e.g., can both administer the ERP and post journal entries) can bypass every control in the system.
- **Account opening / onboarding + transaction processing**: common in financial services; lets staff create accounts and transact with them.

Name each combination present in the entity and assess it, rather than relying on a generic matrix to surface them.

### Distinguish genuine incompatibility from operational adjacency

Not every pair of duties held by one person is a control problem. A person may legitimately hold adjacent duties where:

- neither duty enables concealment of the other (e.g., preparing two unrelated reports);
- the combination is necessary for a small entity and is mitigated by a strong, independent compensating control;
- the duties are performed at different times with independent review between them.

Over-flagging adjacency as incompatibility dilutes focus and generates noise that hides the serious conflicts. Apply the concealment test to keep the assessment sharp.

### Assess master data conflicts as a distinct, high-risk category

Master data — vendor, customer, employee, bank, product, chart of accounts — is the foundation of transaction processing. Whoever can change master data can redirect payments, create ghost vendors, alter pricing, or reclassify transactions. Treat the combination of master data maintenance with any transaction duty over the same data type as high-risk, and confirm:

- who can create, edit, and deactivate master records;
- whether changes are reviewed or approved by someone independent;
- whether a periodic independent review of master data changes exists as a compensating control;
- whether the system enforces any separation, or whether it relies entirely on procedure.

Master data conflicts are frequently underweighted because the duties look administrative rather than transactional.

### Evaluate the IT administrator / business user combination specifically

The IT administrator who also holds business privileges is a uniquely dangerous combination because administrative rights can bypass every application control. A DBA who can also post journal entries, or a system admin who can also approve payments, can operate outside the application's workflow and audit trail. Confirm:

- whether IT administrators hold any business-role privileges in the applications they administer;
- whether emergency or "firefighter" access is logged, reviewed, and revoked promptly;
- whether production data can be changed outside the application (direct database updates) and who can perform them.

This combination often defeats controls that everyone assumes are working and is a recurring fraud route.

### Consider the temporal dimension: duties held at different times

A combination may be acceptable if the two duties are never held simultaneously — for example, a person who prepares a reconciliation in one period and a different person who reviews it, with no overlap. But many "temporal" separations are illusory: the same person prepares and reviews because the reviewer is absent, or the separation exists on the schedule but not in practice. Confirm temporal separation is real and enforced, not just scheduled.

### Weigh the combination against compensating controls and entity size

In smaller entities, complete separation is often impossible, and some incompatible combinations are accepted as a business reality. This is defensible only where:

- a strong, independent compensating control operates (e.g., the owner reviews every transaction by the person holding the combination);
- the combination is documented and known, not hidden;
- the risk significance is assessed and accepted at the right level (owner, board).

Acceptance of an incompatible combination without a documented, tested compensating control is a control deficiency, regardless of entity size.

### Connect incompatible duty findings to specific fraud schemes

Each incompatible combination maps to specific fraud schemes (ghost vendor payments, skimming, kiting, fraudulent disbursements, revenue cut-off manipulation). When documenting a finding, name the scheme it enables. This makes the risk concrete, focuses the response, and connects the access finding to the fraud risk assessment and to any targeted substantive procedures.

## Common Traps

- **Flagging every adjacent duty pair as incompatible**, generating noise that obscures the genuinely dangerous concealment-enabling combinations.
- **Missing master data conflicts** because master data maintenance looks administrative rather than transactional.
- **Overlooking the IT administrator / business user combination**, which silently defeats every application control and is a common fraud route.
- **Accepting "temporal" separation that exists only on the schedule**, not in practice when the reviewer is absent.
- **Treating small-entity acceptance of a combination as adequate** without a documented, tested compensating control.
- **Documenting incompatible duties as access findings only**, without naming the specific fraud scheme each enables or connecting to fraud risk procedures.
- **Assuming a combination is safe because it has existed for years without incident** — the absence of detected fraud is not evidence of effective control when the combination enables concealment.
- **Forgetting that shared or generic accounts create incompatible combinations** for whoever holds the credentials, often without individual accountability.

## Self-Check

- For each incompatible combination I identified, did I confirm it enables concealment (the defining test), not merely adjacency?
- Did I explicitly assess the high-frequency combinations — master data + transactions, recording + reconciliation, custody + recording, IT admin + business role, account opening + transactions?
- Did I distinguish genuine incompatibility from operational adjacency, and avoid over-flagging that dilutes focus?
- Did I treat master data conflicts as a distinct high-risk category and confirm whether changes are independently reviewed?
- Did I specifically test whether IT administrators hold business privileges or can change production data outside the application?
- For any combination accepted due to entity size, is there a documented, tested compensating control — and is the acceptance recorded at the right level?
- Did I confirm temporal separations are real and enforced in practice, not just on the schedule?
- For each significant finding, did I name the specific fraud scheme it enables and connect it to the fraud risk assessment and targeted procedures?
