---
name: data-classification-and-handling.md
description: Use when the agent is designing a data classification scheme, assigning sensitivity tiers to data assets, defining handling rules for confidential restricted or personal data, mapping classification to access controls and encryption, or evaluating whether data handling meets the sensitivity tier requirements.
---

# Data Classification and Handling

Data classification is the backbone of information governance. Without knowing what data the organisation holds and how sensitive it is, no downstream control — access, encryption, retention, sharing, transfer — can be correctly applied. Classification is not a one-time labelling exercise; it is an ongoing discipline of identifying data sensitivity, assigning handling rules proportionate to that sensitivity, and enforcing those rules across systems and people. A classification scheme that is too granular collapses into non-use; one that is too coarse lumps critical secrets with trivia and protects neither well. This skill addresses the judgment involved in designing and operating a data classification and handling framework.

## Core Rules

### Design a classification scheme that is simple enough to use and granular enough to protect

A workable classification scheme has a small number of tiers (typically three to five) that correspond to meaningfully different handling requirements. A common structure:

- **Public**: approved for public release; no restriction on disclosure;
- **Internal**: for internal use; disclosure causes limited harm;
- **Confidential**: disclosure causes significant harm; restricted access;
- **Restricted / Secret**: disclosure causes severe harm (trade secrets, regulated data, security credentials); tightly controlled.

Resist adding tiers for every nuance. Each tier must have a distinct handling rule; if two tiers have identical rules, merge them. The scheme's value is in the consistent application, not in taxonomic precision.

### Define explicit handling rules for each tier

Each classification tier must have defined handling requirements covering:

- **Access**: who may access (role-based, need-to-know, approval required);
- **Storage**: where it may reside (approved systems, encryption at rest);
- **Transmission**: how it may be sent (encrypted in transit, no unencrypted email for restricted);
- **Sharing**: with whom it may be shared externally and under what terms;
- **Retention**: how long it is kept and how it is disposed;
- **Marking**: how it is labelled so users and systems recognise the tier.

Handling rules that are vague ("handle appropriately") are unenforceable. Be specific enough that a user or system can determine the correct handling from the tier alone.

### Map personal and regulated data into the scheme, not as a parallel system

Personal data, special category data, financial data, health data, and other regulated categories must be accommodated within the classification scheme, not handled by a separate and conflicting system. Typically, regulated data falls into the higher tiers (confidential or restricted) and inherits the corresponding handling rules, with additional specific obligations (consent records, retention limits, breach notification) layered on. Ensure the classification and the regulatory regime are reconciled so that users apply one consistent framework.

### Assign classification at creation or ingestion, not retrospectively

Classification is most accurate when assigned at the point of creation or ingestion, when the context is known. Require the creator or ingesting system to classify data as it enters the environment. Retrospective classification of accumulated data is expensive, error-prone, and often deferred indefinitely. Build classification into the creation workflow (templates, forms, ingestion pipelines) as a mandatory field.

### Enforce classification through technical controls, not just policy

Policy that says "restricted data must be encrypted" is ineffective if systems do not enforce it. Map classification to technical controls:

- **Data loss prevention (DLP)**: detect and block movement of restricted data to unapproved channels;
- **Access controls**: restrict access by classification tier and role;
- **Encryption**: enforce encryption at rest and in transit for confidential and restricted tiers;
- **Labeling and metadata**: tag data so controls can read and enforce the tier;
- **System provisioning**: ensure new systems are configured to the highest tier they will hold.

Technical enforcement closes the gap between policy and practice.

### Handle the default-to-highest-reasonable-tier problem correctly

When the classification of data is uncertain, the safe default is to treat it as the higher tier until determined. However, this must be paired with a prompt classification review, or the organisation ends up over-classifying everything (which is costly and breeds non-compliance as users ignore impractical rules). Establish a process to resolve uncertain classifications quickly rather than leaving data perpetually over-classified.

### Maintain a data inventory and map classification to assets

A data inventory (data map or register) records what data the organisation holds, where, in what system, at what classification, and who is responsible. The inventory is the foundation for access reviews, retention enforcement, breach response, and regulatory inquiries. Maintain it as a living document, updated as systems and data flows change. An organisation that cannot enumerate its data cannot govern it.

### Train users and audit classification in practice

A classification scheme fails if users do not understand it or do not apply it. Train all users on the scheme, the handling rules, and their responsibilities. Audit classification in practice: sample documents and systems and verify the assigned tier matches the actual sensitivity. Misclassification is common and must be detected and corrected.

## Common Traps

### Over-classifying to be safe, then ignoring the rules

When everything is "restricted," the tier loses meaning and users stop applying the handling rules. Over-classification creates a false sense of protection while breeding non-compliance. Classify accurately.

### Designing too many tiers that users cannot distinguish

A seven-tier scheme sounds precise but is unusable. Users default to the middle tier regardless of actual sensitivity. Keep tiers few and distinct.

### Handling rules that are policy-only with no technical enforcement

"Confidential data must not be emailed externally" is ineffective if email systems do not enforce it. Without technical controls, policy is aspirational.

### Classifying data without defining retention and disposal

Classification that covers access and encryption but not retention leads to indefinite accumulation. Each tier needs a retention and disposal rule.

### Failing to reconcile classification with regulated data categories

Personal data governed by GDPR, health data by HIPAA, financial data by sector rules — if each has its own parallel labelling, users face conflicting signals. Integrate regulated categories into the unified scheme.

### Never auditing whether classification matches actual sensitivity

Data is classified at creation and never reviewed. Over time, sensitivity changes (a draft becomes public; a confidential list becomes stale) and the classification drifts. Audit periodically.

## Self-Check

- Does the classification scheme have a small number of tiers (three to five), each with distinct, specific handling rules?
- Are personal and regulated data categories integrated into the scheme rather than handled by conflicting parallel systems?
- Is classification assigned at creation or ingestion as a mandatory step, not retrospectively?
- Are handling rules enforced through technical controls (DLP, access controls, encryption, labeling) rather than policy alone?
- Is uncertain data defaulted to the higher tier with a prompt classification review process?
- Is a data inventory maintained mapping data assets to classification, location, and ownership?
- Are users trained on the scheme, and is classification audited in practice with corrections applied?
- Does each tier have a defined retention and disposal rule, not just access and encryption?
