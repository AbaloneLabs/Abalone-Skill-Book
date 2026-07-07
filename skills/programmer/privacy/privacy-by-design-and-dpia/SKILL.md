---
name: privacy_by_design_and_dpia.md
description: Use when the agent is building a feature or system that processes personal data and must apply privacy-by-design (data minimization, purpose limitation, privacy as a default, end-to-end security), conducting or supporting a Data Protection Impact Assessment (DPIA) for high-risk processing, deciding on privacy controls early in design (rather than retrofitting), mapping data flows of personal data, identifying and mitigating privacy risks, or ensuring a feature meets principles like lawfulness, fairness, and transparency under GDPR or similar regimes. Also covers the failure modes of bolting privacy on after launch, collecting data "just in case" without a purpose, missing a required DPIA for high-risk processing, controls that protect storage but not inference or re-identification, and the recurring mistake of treating privacy as a compliance formality rather than a set of design decisions that determine what the system is allowed to do with people's data.
---

# Privacy By Design And DPIA

Privacy by design is the principle that privacy protections must be built into systems from the start — as default settings, as minimized data collection, as purpose-limited use, as end-to-end security — rather than retrofitted after the system has already collected and exposed data. A Data Protection Impact Assessment (DPIA) is the structured process for identifying and mitigating privacy risks in high-risk processing before it begins. The judgment problem is that privacy is determined by design decisions made early: what data is collected, why, how long it is kept, who can access it, what it can be combined with, and what the user consented to. Once a system has collected data broadly and built features on it, restricting the data threatens the features, and retrofitting privacy is expensive and incomplete. Worse, privacy harms are often invisible to the system's designers: a dataset that is "anonymized" can be re-identified when combined with other data; a feature that seems benign can enable inference about sensitive attributes; a retention period that seems reasonable can accumulate a dossier that becomes a liability and a target. The discipline is to treat privacy as a set of design constraints applied before data is collected, to map the data flows and risks deliberately, and to conduct a DPIA where processing is high-risk — not to treat privacy as a compliance formality filled in after launch.

Agents tend to under-invest here because the feature works without privacy work, and privacy feels like a compliance overlay rather than an engineering concern. The harm appears when the data is collected and the risk materializes. A feature collects data "just in case" with no defined purpose, and the purpose limitation principle is violated (the data cannot be used, or is used unlawfully). A high-risk processing (large-scale, sensitive, or systematic monitoring) launches without a DPIA, and the regulator requires it halted and assessed. A dataset is shared or published as "anonymized" but is re-identifiable, exposing individuals. A retention period is never enforced, and the accumulated data becomes a breach liability and a regulatory violation. The judgment problem is to apply privacy-by-design principles as early design constraints, to map personal data flows and assess their risks, to conduct DPIAs for high-risk processing, and to treat privacy as engineering, not paperwork.

This skill covers privacy-by-design principles, data flow mapping, DPIA process and triggers, risk identification and mitigation, and the limits of common controls (anonymization, consent). It complements the pii-handling skill (handling PII operationally), the consent-and-data-rights skill (consent and user rights), and the anonymization skill (techniques and their limits). Here the focus is designing privacy in from the start and assessing high-risk processing.

## Core Rules

### Apply Privacy-By-Design Principles As Early Design Constraints

Privacy by design is a set of principles that, applied at design time, shape what the system is allowed to do with personal data. The core principles:

- **Data minimization: collect the least data needed for the purpose.** Every field collected is a liability (storage, breach surface, regulatory scope). Collect only what the defined purpose requires, not what might "someday be useful." If you cannot state the purpose a field serves, do not collect it.
- **Purpose limitation: use data only for the stated, compatible purpose.** Data collected for one purpose cannot be repurposed without a lawful basis. "We collected it for billing, so we can use it for marketing" is a purpose-limitation violation. Define the purpose at collection and bound the use to it.
- **Privacy as the default: the most protective setting is the default.** Users who do nothing should get the most privacy-protective configuration; sharing or additional processing should require an opt-in, not be the default that must be opted out of.
- **End-to-end security and retention.** Privacy requires that data is protected across its whole lifecycle (collection, transit, storage, access, deletion) and retained only as long as needed (see the data-retention skill). Security and retention are privacy controls, not separate concerns.
- **Privacy embedded into design, not bolted on.** Privacy controls designed into the architecture (minimized collection, access controls, retention enforcement) are effective; those retrofitted after data is collected and features built on it are expensive and incomplete.

### Map Personal Data Flows Before Building

You cannot protect data flows you have not identified. Mapping how personal data moves through the system is the foundation of privacy design:

- **Map the full lifecycle of personal data.** For each category of personal data: where is it collected, where is it stored, who can access it, where is it transmitted, what is it combined with, how long is it retained, and how is it deleted. A data flow map makes the privacy surface visible.
- **Identify all recipients and processors.** Personal data shared with third parties, subprocessors, or other internal teams extends the privacy boundary; each recipient is a trust and compliance decision. Map them, and ensure each has a lawful basis and adequate safeguards.
- **Identify combinations and inferences.** Data that is benign alone may be sensitive when combined or may enable inference about sensitive attributes (health, beliefs, orientation). Map not only what is collected but what can be derived, and treat derivable sensitive data as sensitive.
- **Keep the map current.** Systems evolve; new data flows appear. A data flow map that is not maintained becomes a stale document that misses the flows that matter.

### Conduct A DPIA For High-Risk Processing

A DPIA is a structured assessment of privacy risks and mitigations, required (under GDPR and similar regimes) for processing likely to result in high risk to individuals. The judgment is knowing when one is required and conducting it substantively:

- **Recognize the triggers for high-risk processing.** Large-scale processing of sensitive data, systematic monitoring, profiling with significant effects, processing of vulnerable populations, and innovative technologies (new forms of identification, tracking) typically trigger a DPIA. When in doubt, assess.
- **Conduct the DPIA before the processing begins.** A DPIA conducted after launch cannot shape the design; its findings arrive too late. Conduct it during design, so its mitigations are built in.
- **Make the DPIA substantive, not a formality.** A real DPIA identifies concrete risks (re-identification, function creep, unauthorized access, discriminatory effects), assesses their likelihood and severity, and specifies mitigations that reduce risk to acceptable levels. A DPIA that lists no real risks or mitigations is paperwork, not protection.
- **Consult stakeholders and, where required, the regulator.** For genuinely high-risk processing, consult the data protection officer, affected users (where feasible), and the regulator (GDPR requires prior consultation for residual high risk). Do not treat the DPIA as a solo engineering exercise.

### Identify And Mitigate The Real Privacy Risks

Privacy risks are specific and varied, and mitigation must target the actual risk, not a generic "we encrypted it":

- **Re-identification risk.** "Anonymized" data that can be linked to other data to re-identify individuals is not anonymous (see the anonymization skill). Assess re-identification risk realistically, including auxiliary data an attacker might have, and apply robust techniques (k-anonymity, differential privacy) where the risk is real.
- **Inference and derived sensitive data.** Data that enables inferring sensitive attributes (predicting health from purchases, beliefs from reading) carries the risks of sensitive data even if never collected directly. Assess what can be inferred and govern it as sensitive where appropriate.
- **Function creep.** Data collected for one purpose gradually used for others. Mitigate with purpose limitation, access controls scoped to the purpose, and governance that catches repurposing.
- **Unauthorized access and breach.** The data's confidentiality depends on access controls, encryption, and security (see the pii-handling skill). A breach of over-collected or over-retained data is a larger harm; minimization and retention reduce the blast radius.
- **Discriminatory or unfair effects.** Processing that produces discriminatory outcomes (biased profiling, differential pricing) is a privacy and fairness harm. Assess for these effects, especially in profiling and automated decisions.

### Understand The Limits Of Consent And Anonymization As Controls

Consent and anonymization are common privacy controls, but both have limits that are easy to over-rely on:

- **Consent must be informed, specific, freely given, and withdrawable.** A buried clause or a bundled "agree to everything" is not valid consent; consent must be granular per purpose and revocable. Over-relying on consent that is not valid provides no lawful basis (see the consent-and-data-rights skill).
- **Consent is not a substitute for minimization.** Collecting everything because the user "consented" violates minimization; consent legitimizes a specific purpose, not unlimited collection.
- **Anonymization must be robust against realistic attackers.** Removing direct identifiers is rarely sufficient; quasi-identifiers and combinations enable re-identification. Treat data as anonymous only if re-identification is genuinely infeasible against a realistic attacker, and document the assessment.
- **Pseudonymization is not anonymization.** Pseudonymized data (keys replaced with identifiers) is still personal data; it reduces risk but does not remove the data from privacy scope. Do not treat pseudonymized data as exempt from privacy obligations.

## Common Traps

### Bolting Privacy On After Launch

Building the feature and collecting the data first, then adding privacy controls, when the controls are now expensive, incomplete, and threaten the features built on the data. Apply privacy-by-design principles as early design constraints.

### Collecting Data "Just In Case" Without A Purpose

Collecting fields with no defined purpose, violating data minimization and purpose limitation, and accumulating liability. Collect only what a stated purpose requires; if you cannot state the purpose, do not collect it.

### Missing A Required DPIA For High-Risk Processing

Launching large-scale, sensitive, or systematic processing without a DPIA, and the regulator requiring it halted. Recognize high-risk triggers and conduct a substantive DPIA before processing begins.

### Treating Anonymization As Removing Identifiers

Assuming data is anonymous because direct identifiers were removed, when quasi-identifiers and combinations enable re-identification. Assess re-identification realistically and apply robust techniques where the risk is real.

### Over-Relying On Invalid Consent

A bundled or buried consent that is not informed, specific, or freely given, providing no lawful basis, or treating consent as a substitute for minimization. Make consent granular and revocable, and minimize regardless of consent.

### Ignoring Inference And Derived Sensitive Data

Dismissing privacy risk because sensitive attributes are "never collected," when they can be inferred from other data. Assess what can be derived and govern derivable sensitive data as sensitive.

### No Retention Enforcement, Accumulating A Dossier

Collecting personal data with no enforced retention, so it accumulates indefinitely into a breach liability and regulatory violation. Define and enforce retention (see the data-retention skill); retention is a privacy control.

## Self-Check

- [ ] Privacy-by-design principles are applied as early design constraints: data minimization (only data a stated purpose requires), purpose limitation (use bounded to the collected purpose), privacy as the default (most protective setting without action), end-to-end security and retention, and privacy embedded in design rather than retrofitted.
- [ ] A current data flow map identifies the full lifecycle of personal data (collection, storage, access, transmission, combination, retention, deletion), all recipients and processors, and derivable combinations/inferences — and is maintained as the system evolves.
- [ ] A DPIA is conducted before high-risk processing (large-scale sensitive data, systematic monitoring, significant-effect profiling, vulnerable populations, innovative technologies), it is substantive (concrete risks, likelihood/severity, real mitigations), and stakeholders/DPO/regulator are consulted where required.
- [ ] Concrete privacy risks are identified and mitigated: re-identification (robust anonymization where risk is real), inference of sensitive attributes (governed as sensitive), function creep (purpose limitation and scoped access), unauthorized access/breach (minimization and retention reduce blast radius), and discriminatory effects (assessed in profiling).
- [ ] Consent and anonymization are understood in their limits: consent is informed, specific, freely given, and withdrawable (not bundled or buried), minimization applies regardless of consent, anonymization is robust against realistic re-identification, and pseudonymized data is treated as personal data.
- [ ] Retention is defined and enforced (data is deleted on schedule, not accumulated indefinitely), as a core privacy control.
- [ ] The highest-risk cases were verified — a re-identification of "anonymized" data, an inference of sensitive attributes, a purpose-limitation violation from repurposing, a high-risk processing launched without a DPIA, and unenforced retention accumulating a dossier — not only the clean privacy-policy document.
