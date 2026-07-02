---
name: data-minimization-and-purpose-limitation.md
description: Use when the agent is deciding what personal data to collect or retain, applying data minimization principles, enforcing purpose limitation, evaluating whether a new use is compatible with the original purpose, designing retention schedules, or conducting collection necessity assessments for a processing operation.
---

# Data Minimization and Purpose Limitation

Two of the most foundational principles of data protection are data minimization (collect and retain only what is necessary) and purpose limitation (use data only for specified, explicit, legitimate purposes). These principles are deceptively simple in statement and profoundly difficult in practice. Organisations collect more data than they need "just in case," repurpose data for new analytics or marketing without assessing compatibility, and retain data indefinitely because deletion is operationally hard. Each of these is a violation that compounds over time: a dataset that grew without discipline becomes a liability that is difficult to unwind. This skill addresses the judgment involved in deciding what to collect, what to retain, and when a new use is permissible.

## Core Rules

### Apply data minimization at the point of collection, not retrospectively

Data minimization requires that personal data collected is adequate, relevant, and limited to what is necessary for the stated purpose. This is determined at design time. For each data field requested:

- Is it necessary for the specific purpose? (If the purpose is order fulfillment, the shipping address is necessary; the date of birth is not.)
- Is it adequate? (Is enough collected to achieve the purpose?)
- Is it relevant and not excessive? (Are there fields collected "because we might need them someday"?)

Challenge every field. A field with no current, specific purpose should not be collected. "Nice to have" is not a lawful basis for collection.

### Define and document the specific purpose before collection

Purpose limitation requires that data is collected for specified, explicit, and legitimate purposes and not further processed incompatibly. "Specified and explicit" means the purpose is articulated concretely, not vaguely. "To improve our services" is not a specified purpose; "to analyse checkout abandonment and reduce friction in the payment flow" is. Document the purpose for each collection, link it to the lawful basis, and communicate it to the data subject in the privacy notice.

### Test new uses for compatibility before proceeding

When data collected for one purpose is to be used for another, the new use is lawful only if the new purpose is compatible with the original. The compatibility assessment considers:

- Any link between the original and new purposes;
- The context of collection and the data subject's reasonable expectations;
- The nature of the data (especially if special category);
- The consequences of the new use for the data subject;
- The existence of safeguards (encryption, pseudonymisation).

If the new purpose is incompatible, it requires a fresh lawful basis (often consent) and fresh notice. Do not silently repurpose data. A common failure is using customer data collected for service delivery for unrelated marketing or profiling without consent.

### Distinguish further processing that is compatible from incompatible repurposing

Some further processing is clearly compatible and does not require a new basis:

- Using data for internal research and development consistent with the data subject's expectations;
- Anonymisation or pseudonymisation for security research;
- Archiving in the public interest or scientific research (with specific conditions).

Other repurposing is clearly incompatible:

- Using service data to build profiles sold to third parties;
- Combining datasets to infer special category data the subject did not provide;
- Using employment data for unrelated monitoring or surveillance.

When in doubt, treat the new use as incompatible and seek consent or a specific basis.

### Design and enforce retention schedules

Data minimization extends to retention: data must not be kept longer than necessary for the purpose. For each data category, define:

- The retention period tied to the purpose (for example, transaction data kept for the legal limitation period plus a defined margin);
- The trigger for deletion (a date, an event, a status change);
- The deletion or anonymisation method;
- Exceptions for legal hold or legal obligation retention.

Retention without a schedule is indefinite retention, which is a violation. Enforce schedules through automated deletion where possible; manual deletion is rarely reliable at scale.

### Anonymise or pseudonymise where feasible

Where the purpose can be achieved without direct identifiers, use pseudonymisation (replacing identifiers with tokens, with a separately held re-identification key) or anonymisation (irreversibly preventing re-identification). Pseudonymisation reduces risk while allowing re-identification for legitimate purposes; anonymisation removes the data from the scope of data protection law entirely (if genuinely anonymous). Note that pseudonymised data is still personal data; only true anonymisation changes the legal status.

### Periodically review and purge accumulated data

Data tends to accumulate in systems, backups, logs, and analytics stores beyond any defined purpose. Conduct periodic data inventories and purges:

- Identify data that no longer serves a current purpose;
- Delete or anonymise it unless subject to legal hold;
- Update the inventory to reflect the purge.

A dataset that has not been reviewed in years almost certainly contains data retained beyond necessity.

### Prevent function creep through governance

Function creep — the gradual expansion of data use beyond the original purpose — is often incremental and unplanned. Prevent it through:

- Change control that flags new uses of existing data for compatibility assessment;
- Access controls that limit use to the defined purpose;
- Periodic audits of actual data use against documented purposes.

## Common Traps

### Collecting data "in case we need it later"

Anticipatory collection without a current, specific purpose violates minimization. The cost of not having a field later is far lower than the cost of holding data unlawfully collected. Collect when the purpose arises.

### Vague purpose statements that enable function creep

A purpose like "to provide and improve services" is so broad it justifies almost any use. Specify purposes concretely so that compatibility can be assessed and function creep detected.

### Silently repurposing data for marketing or analytics

Using service data for marketing without consent, or for profiling the data subject did not expect, is incompatible further processing. Assess compatibility and obtain consent where required.

### Indefinite retention because deletion is hard

Operational difficulty is not a legal justification for retaining data beyond its purpose. Build deletion into systems and processes. "We have not gotten around to deleting it" is a violation.

### Treating pseudonymised data as anonymous

Pseudonymised data is still personal data because re-identification is possible with the key. Only irreversibly anonymised data falls outside the regime. Do not claim anonymisation where re-identification remains feasible.

### Combining datasets to infer sensitive data

Merging datasets can reveal special category data (health, political views) the subject never provided. Inferring sensitive data triggers the heightened Article 9 regime and is often incompatible with the original purposes.

## Self-Check

- For each data field collected, is there a documented, specific, current purpose that makes it necessary, adequate, and not excessive?
- Is the purpose for each collection specified explicitly and communicated to the data subject in the privacy notice?
- Before using data for a new purpose, has a compatibility assessment been conducted considering the link, context, data nature, consequences, and safeguards?
- Is incompatible repurposing stopped or routed through fresh consent and notice?
- Are retention schedules defined for each data category, with deletion triggers, methods, and legal-hold exceptions?
- Are anonymisation or pseudonymisation applied where the purpose can be achieved without direct identifiers?
- Are periodic data inventories and purges conducted to remove data no longer serving a current purpose?
- Is function creep prevented through change control, access controls, and periodic use audits?
