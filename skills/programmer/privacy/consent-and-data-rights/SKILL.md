---
name: consent_and_data_rights.md
description: Use when the agent is building features for consent capture and management, data subject access requests, data portability, deletion or rectification requests, consent withdrawal, purpose limitation enforcement, preference centers, or rights workflows. Covers consent as a verifiable record with purpose, versioning, and revocation; the rights to access, rectify, erase, restrict, port, and object; purpose limitation and secondary-use gating; consent withdrawal cascades; regional regimes (GDPR, CCPA/CPRA, LGPD, PIPL) and their differing triggers; privacy-by-design defaults; and the systems needed to actually fulfill a rights request across primary stores, caches, backups, and downstream consumers. Also use when reviewing whether a feature respects the purposes for which data was collected or whether consent is genuinely informed and freely given.
---

# Consent And Data Rights

Consent and data rights are usually implemented as a form to click and an email address for "privacy requests," and then treated as solved. They are not. Consent is a legal and ethical state that must be captured with enough context to be provable years later, tied to the specific purposes for which it was given, versioned as those purposes or the law change, and revocable at any time with effects that cascade to every downstream system using the data. Data rights (access, deletion, portability, rectification) are not a customer-support ticket; they are a programmatic obligation to find a person's data across every store, cache, backup, and downstream consumer, act on it within a legal deadline, and prove you did. The recurring failure is building the front-end (the consent banner, the request form) and assuming the back-end exists, when the back-end — the record of what was consented to, the map of where data lives, the machinery to act on it — is the hard and legally consequential part.

Agents tend to under-deliver here because the requirements look like UI work. The defects live in the data layer: consent stored without purpose or version so you cannot tell what was agreed to, deletion that removes a row from the primary table but leaves the person in the analytics warehouse, the recommendation model, the email platform, and last night's backup; access requests answered from the obvious store while forgotten exports and log archives still hold the data; consent withdrawal that updates a flag but keeps sending marketing email from a synced system that never re-reads the flag. The judgment problem is treating consent and rights as cross-system data-consistency problems with legal deadlines and audit requirements, not as single-record updates.

This skill is about implementing consent and data subject rights as real, fulfillable, auditable systems. It complements the PII skill (which covers classifying and minimizing data) and the retention skill (which covers scheduled deletion); here the question is how to honor individual choices and rights over data that spans the whole system.

## Core Rules

### Capture Consent As A Verifiable, Purpose-Bound, Versioned Record

Consent is not a boolean. A defensible consent record captures what a person agreed to, in what form, when, and with what they were told.

- **Tie consent to specific purposes.** "Marketing" is not a purpose; "promotional email for product X" is. Each purpose has its own consent state, so a user can consent to transactional email but not marketing, or to analytics but not personalization.
- **Record the version of the notice presented.** When the privacy notice or the consent text changes, existing consent remains tied to the version the user saw. A material change in purpose or practice may require fresh consent, not a silent update.
- **Capture proof.** Timestamp, the identity of the user (or their authenticated session), the exact text or a reference to it, the mechanism (checkbox, toggle, account setting). For higher-stakes processing, this record is the evidence that consent was validly obtained.
- **Default to no consent, require an active opt-in where required.** Pre-ticked boxes, bundled consents ("consent to everything in one click"), or consent required to use a service that does not need the data are not valid consent under strict regimes. Separate consents for separate purposes.
- **Make consent revocable at any time, as easily as it was given.** Withdrawal must be at least as easy as giving consent, and must actually take effect (see the cascade rule below).

### Make Consent Withdrawal Cascade To Every Consumer Of The Data

A withdrawn consent that updates a flag but does not stop processing is a violation. Withdrawal must propagate to every system that acts on the consent.

- **Map the consumers of each consent.** Before relying on withdrawal, know which systems read it: the email platform, the recommendation engine, the ad-tech integrations, the analytics pipeline, the CRM. If you do not know who reads a consent state, you cannot withdraw it.
- **Propagate withdrawal, do not wait for re-read.** Systems that cache consent (a synced audience, an exported segment) will keep processing until they refresh. Push withdrawal to consumers or remove the affected records from downstream audiences, rather than assuming a flag change will be noticed.
- **Decide withdrawal vs deletion.** Withdrawing consent for a purpose stops processing for that purpose; it does not necessarily delete the data. Be explicit about which happens: does withdrawing marketing consent keep the account but stop email, or must the data be deleted? Different purposes and regimes differ.
- **Handle the transition for in-flight processing.** A withdrawal that arrives mid-campaign should stop future sends but cannot un-send already-delivered email. Define the effective boundary and honor it.

### Build Rights Workflows That Span Every Store, Not Just The Primary Database

Data subject rights (access, rectification, erasure, restriction, portability, objection) are obligations over all of a person's data, wherever it lives. A request fulfilled from the users table while the person remains in the warehouse, the logs, the backups, and downstream integrations is not fulfilled.

- **Maintain a data map.** Know, per data category, where it is stored and where it flows: primary databases, caches, analytics warehouses, log archives, search indexes, message queues, third-party processors, exports shared with partners. You cannot fulfill a request for data you have forgotten you hold.
- **Cover derived and replicated data.** Aggregates, embeddings, segments, and model features derived from a person's data may be in scope for access or deletion; pseudonymous ids that re-identify are in scope. Decide and document how each derived store is handled.
- **Handle backups and logs honestly.** Erasure does not require rewriting immutable backups, but it does require that restored-from-backup data does not resurrect the deleted person in production. Define the backup-window exception and ensure restore procedures apply outstanding deletions.
- **Meet the deadline.** Most regimes require response within a fixed period (commonly one month, extendable under conditions). The workflow must be operational, not aspirational; if fulfilling a request requires manual work per store, you will miss the deadline at volume.
- **Verify identity without creating new exposure.** Confirming the requester is the data subject must not require collecting excessive new data or create an enumeration vector (e.g., revealing whether an account exists). Use existing authentication where possible; for unauthenticated requests, verify proportionally to the sensitivity of the data.

### Enforce Purpose Limitation As A System Property, Not A Hope

Data collected for one purpose cannot be freely used for another. Purpose limitation is a legal principle that becomes an engineering problem: how do you prevent a column collected for billing from being used for ad targeting?

- **Tag data with its collection purpose.** At ingest, record why each data element was collected and under what consent or legal basis. A field without a purpose tag is a field that can be misused without anyone noticing.
- **Gate secondary use on a compatible purpose or fresh consent.** Before using data for a new purpose, determine whether the new purpose is compatible with the original (some regimes define compatibility narrowly) or whether new consent is required. Do not silently repurpose data.
- **Make incompatible use fail loudly.** Access controls or data-use policies that block a join between "billing data" and "marketing audience" turn purpose limitation from a policy into a property. Where feasible, enforce it in the system rather than relying on people to remember.

### Design For Privacy By Default, Not Privacy By Configuration

Privacy-by-design means the default state protects the user, and sharing or processing requires a positive act. The trap is the inverse: maximal data collection and sharing by default, with the user responsible for finding and disabling each one.

- **Default to least collection and least sharing.** Collect only what the feature needs; share with third parties only when the user opts in. The burden of action should be on the party that benefits from more data, not on the user to protect themselves.
- **Make choices granular and persistent.** A single "accept all" with a hidden "manage options" is not privacy by design. Offer meaningful per-purpose choices that persist across sessions and devices.
- **Decouple consent from service where the law requires it.** If consent is required for a purpose that is not necessary for the service, refusing consent must not break the service. Bundling is not valid consent.

### Account For Regional Differences Without Building A Separate System Per Regime

GDPR, CCPA/CPRA, LGPD, PIPL, and others overlap but differ in who is covered, what rights exist, what triggers them, and what the deadlines are. The trap is either ignoring the differences (applying one regime globally and under-protecting) or forking the system per regime (unmaintainable).

- **Model the most protective regime as the baseline.** Building to GDPR-grade generally satisfies or exceeds other regimes, then layer regional specifics (e.g., CCPA's "sale" and "share" concepts, opt-out vs opt-in models) as configuration.
- **Track residency and applicability.** Which regime applies can depend on the user's location, your establishment, or where the data is processed. Record enough about each user to determine applicability, and apply the right rules.
- **Map regime-specific rights to a common workflow.** "Right to delete" (GDPR), "right to delete" (CCPA, with exceptions), and "right to erasure" are close enough to share machinery, with regime-specific exclusions as configuration. Do not fork the deletion engine.

## Common Traps

### Consent Stored As A Single Boolean

One `marketing_consent` flag with no purpose, version, timestamp, or proof. When the notice changes or a request arrives, there is no record of what was actually agreed to. Capture purpose, version, timestamp, and mechanism.

### Withdrawal That Updates A Flag But Does Not Stop Processing

Withdrawing marketing consent sets a flag, but the email platform, synced from a nightly export, keeps sending for weeks. Propagate withdrawal to every consumer; do not assume downstream systems will re-read the flag.

### Deletion From The Primary Table Only

A deletion request removes the user row, but the person persists in the analytics warehouse, the recommendation features, the CRM, the logs, and the next backup restore. Fulfill across every store on the data map, including derived data and the backup-restore path.

### Access Request Answered From The Obvious Store

Fulfilling an access request from the users table while forgetting exports, log archives, and third-party processors that also hold the data. Enumerate all stores from the data map; an incomplete access response is a failed response.

### Bundled Or Pre-Ticked Consent

A single "I agree to everything" checkbox, or pre-ticked boxes, treated as valid consent for multiple purposes. Separate consents for separate purposes; require active opt-in where the regime demands it.

### Silent Repurposing Of Data

Using data collected for billing to train an ad-targeting model, on the assumption that "we have the data so we can use it." Gate secondary use on purpose compatibility or fresh consent; tag data with collection purpose.

### Default-On Sharing The User Must Find And Disable

Third-party sharing or non-essential processing enabled by default, with the user responsible for opting out across buried settings. Default to least sharing; make opt-in the path for non-essential processing.

### Treating Rights Requests As Customer Support Tickets

No defined workflow, deadline tracking, or identity verification; requests handled ad hoc by whoever sees the email. Operationalize the workflow with identity verification, deadline tracking, and a data map that makes fulfillment routine.

### Forking The System Per Regime

A separate consent and rights implementation for each region, diverging and rotting. Model the most protective regime as a baseline and express regional differences as configuration over shared machinery.

## Self-Check

- [ ] Consent is captured as a purpose-bound, versioned record with timestamp, identity, the notice version presented, and mechanism — not a single boolean — and each purpose has its own consent state.
- [ ] Consent is not bundled or pre-ticked where valid consent requires granularity and active opt-in; refusal of non-essential consent does not break the essential service.
- [ ] Consent withdrawal cascades to every consumer of the data (email platform, recommendation, ad-tech, analytics, CRM), with downstream audiences updated or purged rather than relying on a flag re-read.
- [ ] A current data map exists covering primary stores, caches, warehouses, logs, backups, search indexes, queues, third-party processors, and partner exports, so rights requests can be fulfilled across all of them.
- [ ] Rights workflows (access, rectification, erasure, restriction, portability, objection) cover derived and replicated data, handle the backup/restore path so deleted people are not resurrected, verify requester identity proportionally, and meet the legal deadline operationally at volume.
- [ ] Purpose limitation is enforced: data is tagged with collection purpose at ingest, secondary use is gated on compatible purpose or fresh consent, and incompatible joins are blocked where feasible rather than left to human memory.
- [ ] Privacy-by-design defaults are in place: least collection and least sharing by default, granular and persistent per-purpose choices, and opt-in (not opt-out) for non-essential processing where required.
- [ ] Regional regimes are handled by modeling the most protective regime as a baseline and expressing differences (CCPA sale/share, opt-out models, applicability triggers) as configuration over shared machinery, with residency/applicability recorded to determine which rules apply.
- [ ] For binding obligations, the consent model and rights workflows were reviewed against the specific applicable regime(s) and a qualified reviewer; this skill is engineering guidance, not legal advice.
