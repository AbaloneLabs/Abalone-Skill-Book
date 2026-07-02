---
name: data_minimization_and_purpose_limitation.md
description: Use when the agent is deciding what personal data to collect, evaluating whether a new use is compatible with the original purpose, detecting scope creep, or assessing repurposing risk for analytics, AI training, or secondary sharing.
---

# Data Minimization And Purpose Limitation

Data minimization and purpose limitation are the principles most often named in enforcement and most often violated by default. Systems are built to collect everything because storage is cheap, and data collected for one purpose is quietly reused for another because the data is already there. Both habits create compounding risk: more data means more breach exposure, more retention burden, and more basis problems, while repurposing quietly undermines the lawful basis under which the data was originally collected.

Use this skill before adding a data field to a form, approving a new analytics or AI use, sharing data with a new recipient, or reviewing whether existing data can support a new feature. The goal is to make the agent justify collection and secondary use explicitly, rather than defaulting to collecting and reusing whatever is available.

## Core Rules

### Collect Only What Is Necessary For The Specified Purpose

Data minimization is not "collect a little less." It is a requirement that each data element be necessary for a specified, explicit purpose. Necessity must be demonstrated, not assumed.

For each field collected, answer:

- what specific purpose does this field serve;
- is the purpose specified and documented;
- is the field actually necessary to achieve that purpose, or merely useful;
- could the purpose be achieved with less data, with aggregated data, or with pseudonymized data;
- is the field collected by default or only when needed.

Fields collected "just in case," "for future analytics," or "because the form template had them" fail minimization. Default form fields should be the minimum set, with optional fields clearly marked and justified.

### Specify Purposes Explicitly At Collection

Purpose limitation depends on purposes being specified at the time of collection. Vague purposes such as "to improve our services" or "for business purposes" are too broad to bind future use.

Specify each purpose so that:

- it is concrete enough to test compatibility against later uses;
- it is disclosed to individuals in the notice;
- it is tied to a lawful basis;
- it is recorded in the data inventory and RoPA.

A purpose described after the fact cannot bind processing that has already occurred. The notice and the inventory should agree on the purposes.

### Test Compatibility Before Secondary Use

Purpose limitation does not forbid all secondary use, but it requires that secondary uses be compatible with the original purpose. Compatibility is a reasoned decision, not a default.

Assess compatibility by considering:

- the link between the original and new purpose;
- the context in which the data was collected and the individual's reasonable expectations;
- the nature of the data, with special category and children's data demanding stricter limits;
- the consequences of the new use for the individual;
- whether safeguards such as encryption, pseudonymization, or opt-out reduce risk.

Compatible secondary uses might include fraud detection, security monitoring, or internal quality improvement closely tied to the service. Incompatible uses typically include advertising, selling data, training shared AI models, or unrelated product features. When in doubt, treat the use as incompatible and obtain a fresh basis.

### Detect And Control Scope Creep

Scope creep is the gradual expansion of data collection and use beyond the original design. It is rarely a single decision; it accumulates through small additions.

Watch for scope creep signals:

- form fields added without a documented purpose;
- logs and events captured at increasing verbosity;
- new analytics events added by product teams without review;
- data exported to new tools or spreadsheets for exploration;
- enrichment with third-party data that expands the profile;
- indefinite retention justified as "we might need it."

Implement a review gate for new data collection and new uses, with a documented minimization and compatibility assessment. The gate should be lightweight enough to be used and strict enough to matter.

### Apply Minimization To Analytics, ML, And AI

Modern data use is where minimization most often fails. Analytics dashboards, ML feature stores, and AI training datasets tend to absorb all available data because more data feels safer.

Apply minimization to advanced uses:

- prefer aggregated or anonymized data where the use does not require identifiability;
- pseudonymize data before analytics where direct identifiers are not needed;
- limit training datasets to fields necessary for the model's purpose;
- avoid training shared or third-party models on personal data without a compatible basis;
- document the minimization choices in the model or dataset card.

Training a general-purpose model on customer support transcripts, chat inputs, or behavioral data without a compatible purpose is a high-risk repurposing that needs explicit review.

### Limit Sharing And Recipients

Minimization applies to who receives data, not only what is collected. Sharing more data than a recipient needs expands your risk and theirs.

For each recipient:

- share only the fields necessary for the recipient's role;
- use aggregation or pseudonymization where the recipient does not need identifiers;
- define the recipient's permitted purposes contractually;
- avoid broad data dumps to vendors for exploratory work;
- review whether a recipient still needs the data on a recurring basis.

Vendor data sharing should be scoped to the minimum necessary for the service, not to "everything we have."

### Tie Minimization To Retention

Minimization is not only about what you collect but how long you keep it. Retaining data beyond its purpose is itself a minimization failure.

Connect minimization to retention by:

- defining retention based on the purpose, not on storage convenience;
- deleting or anonymizing data when the purpose expires;
- avoiding indefinite retention disguised as "we might need it";
- reviewing retention when purposes change or end.

### Make Minimization Decisions Auditable

Minimization decisions should be documented so they can be reviewed and challenged. An undocumented minimization decision is indistinguishable from no decision.

Record:

- the purpose for each data category;
- the necessity rationale for each field;
- compatibility assessments for secondary uses;
- the safeguards applied;
- the approver and date.

## Common Traps

### Collecting Everything Because Storage Is Cheap

The cost of storage is irrelevant to minimization. The cost of a breach, a rights request, or an enforcement action scales with the volume and sensitivity of data held.

### Vague Purposes That Permit Anything

"To improve our services" or "for analytics" are not purposes that bind future use. They leave every secondary use uncontrolled.

### Quiet Repurposing For AI Training

Using support transcripts, chat logs, or behavioral data to train shared models without a compatibility assessment is a recurring and high-profile enforcement theme.

### Treating Optional Fields As Harmless

Optional fields are still collected data. If they are not necessary, they expand exposure and should be removed or justified.

### Sharing Full Records With Vendors

Sending complete user records to a vendor that needs only a subset is a minimization failure at the recipient boundary.

### Confusing Pseudonymization With Anonymization

Pseudonymized data is still personal data and still subject to minimization and purpose limits. Re-identification risk must be assessed, not assumed away.

### Letting Scope Creep Accumulate

Small, unreviewed additions compound into a data collection footprint far larger than the original design, with no single point of accountability.

## Self-Check

- Is each data field collected necessary for a specified, documented purpose, with the necessity rationale recorded?
- Are purposes concrete enough to test compatibility against future uses, and disclosed consistently in notices and the inventory?
- Before any secondary use, has a compatibility assessment considered the link between purposes, expectations, data nature, consequences, and safeguards?
- Is there a review gate for new data collection and new uses that catches scope creep?
- Are analytics, ML feature stores, and AI training datasets minimized, pseudonymized, or aggregated where identifiability is not required?
- Is data shared with recipients limited to the fields necessary for their role, with purposes defined contractually?
- Is retention tied to purpose, with deletion or anonymization when the purpose expires?
- Are minimization decisions documented with purpose, necessity, safeguards, approver, and date?
- Are special category and children's data subject to stricter minimization?
- Can the organization show that optional fields and "just in case" collection have been challenged rather than accepted?
