---
name: anonymization_and_pseudonymization.md
description: Use when the agent is anonymizing or pseudonymizing a dataset before sharing, publishing, using it for analytics or model training, copying production data to a test environment, or building a feature that replaces direct identifiers with tokens. Covers the distinction between anonymization and pseudonymization, re-identification risk from quasi-identifiers, k-anonymity and l-diversity and t-closeness, differential privacy and noise, aggregation pitfalls, data combination and linkage attacks, pseudonymous key management, the limits of anonymization, and the rule against labeling stripped data as anonymous when re-identification remains possible. Also use when releasing datasets, exporting data to partners, training ML on user data, or reviewing whether a dataset is still personal data under privacy regimes.
---

# Anonymization And Pseudonymization

Anonymization is the claim that a dataset no longer relates to identifiable individuals, and it is one of the most overclaimed properties in software. The intuition is "remove the names and it is anonymous." The reality is that almost any dataset with multiple attributes per person is re-identifiable through combination: the well-known finding that roughly 87% of the U.S. population is unique by ZIP code plus gender plus date of birth means a "de-identified" medical dataset with those fields is, for most rows, a lookup table back to a named person. The recurring failure is not a weak algorithm; it is the belief that stripping direct identifiers produces anonymity, when it produces pseudonymity at best — data that is still personal data, still under obligation, and re-identifiable by anyone with an auxiliary dataset.

Agents tend to under-invest here because anonymization feels like a one-step transform and the output "looks anonymous." The defects live in the fields that remain: quasi-identifiers that combine to uniqueness, rare attribute values that single out an individual, timestamps and locations that trace a person, free-text fields that contain names, and join keys that link back to an identified dataset. The judgment problem is treating anonymization as a measured property of the dataset against a defined adversary and auxiliary data — not as a label applied after removing obvious names. If you cannot state what an attacker with reasonable auxiliary data could learn, you have not anonymized; you have relabeled.

This skill is about anonymizing and pseudonymizing data honestly. It complements the PII skill (which covers minimizing what is collected and classifying identifiers) and the retention skill (which covers deleting data). Here the question is: when you must keep or share data but want it not to identify individuals, what actually achieves that, and what only appears to.

## Core Rules

### Distinguish Anonymization From Pseudonymization, And Label Honestly

These terms are not interchangeable, and confusing them produces false confidence and legal exposure. The distinction is whether re-identification is possible.

- **Pseudonymization** replaces direct identifiers with tokens or keys, but a mapping exists (held somewhere) that can re-identify. The data still relates to identifiable individuals, so it remains personal data under most regimes, and obligations largely remain. Useful for limiting exposure within a system (e.g., analytics keyed by a pseudonymous id), not for "anonymizing" a published dataset.
- **Anonymization** irreversibly removes or transforms identifying information so that no one — including you — can re-identify individuals, by any means reasonably likely to be used, including combination with other data. If it succeeds, the result is no longer personal data and most obligations fall away. If it does not succeed, you have pseudonymous data you are wrongly treating as free of obligation.

The decisive question: could anyone, with reasonable effort and auxiliary data, re-identify an individual in this dataset? If yes, it is pseudonymous (or just poorly protected personal data), not anonymous. Do not label data "anonymous" unless the re-identification risk has been genuinely assessed and is low against a realistic adversary.

### Assess Re-Identification Risk From Quasi-Identifiers, Not Just Direct Identifiers

Direct identifiers (name, email, government id) are the obvious removal targets, but re-identification is usually driven by combinations of quasi-identifiers — fields that are not identifying alone but unique in combination. The classic example (ZIP + DOB + gender) generalizes: almost any set of demographic, temporal, and spatial attributes becomes unique at modest granularity.

- **Enumerate quasi-identifiers in the dataset**: date of birth or birth year, ZIP or postal code, gender, race/ethnicity, occupation, employer, education, precise timestamps, geolocation, device type, rare diagnoses or purchases.
- **Measure uniqueness.** For the combination of quasi-identifiers present, what fraction of records are unique (and thus directly re-identifiable against an external dataset like voter rolls)? If a meaningful fraction is unique, the dataset is not anonymous.
- **Consider the adversary's auxiliary data.** Re-identification is a join against data the attacker already has. A dataset that is "anonymous" against no auxiliary data is re-identifiable against public records, breach corpora, or the attacker's own customer data.
- **Beware rare values.** A single unusual attribute (a rare diagnosis, an extreme age, a niche occupation) can single out an individual even without a full quasi-identifier set — the "the only person in X county with condition Y" problem.

If you have not measured uniqueness against realistic quasi-identifier combinations, you have not assessed anonymity; you have assumed it.

### Apply Formal Anonymization Models Where Re-Identification Risk Is Real

When a dataset must be shared or published and re-identification is a concern, informal "we removed the names" is insufficient. Formal models provide measurable guarantees:

- **k-anonymity.** Each record is indistinguishable from at least k-1 others on its quasi-identifiers (generalizing or suppressing values until groups of size k exist). Defends against linkage on quasi-identifiers, but does not protect attribute disclosure (if all k share a sensitive value, the attacker learns it).
- **l-diversity.** Extends k-anonymity so each group of k has at least l "well-represented" values for sensitive attributes, defending against attribute homogeneity within a group.
- **t-closeness.** Further requires that the distribution of a sensitive attribute within each group is close to the overall distribution, defending against skew.
- **Differential privacy.** A formal, mathematical guarantee that the output's distribution is nearly the same whether any one individual's data is included or not, achieved by adding calibrated noise. The strongest guarantee against auxiliary-information attacks, at the cost of utility and implementation complexity, and requiring careful accounting of a privacy budget across queries.

Each model has tradeoffs in utility loss, computational cost, and the attacks it defends against. Choose based on the sensitivity of the data and the strength of the adversary, and understand what each does not guarantee (k-anonymity does not stop attribute disclosure; naive differential privacy with an unbounded query budget "runs out" of guarantee). Apply the model, measure the resulting k or epsilon, and document it — do not claim a guarantee you did not compute.

### Manage Pseudonymization Keys As A Security-Critical Secret

Pseudonymization reduces risk only if the mapping back to real identities is protected. If the mapping is compromised or co-located with the pseudonymous data, the pseudonymization is undone.

- **Store the mapping separately from the pseudonymous data**, under separate access control, ideally in a different system or trust boundary. The analytics warehouse holds pseudonymous ids; a tightly controlled identity vault holds the mapping.
- **Apply least privilege to the mapping.** Only the components that must re-identify (and there should be few, with strong justification) can access it. Analytics, ML training, and reporting use the pseudonymous data and never need the mapping.
- **Rotate or destroy the mapping when no longer needed.** A mapping kept forever is a permanent re-identification capability; destroying it converts pseudonymous data toward anonymous (subject to other re-identification vectors).
- **Treat the pseudonymous data as personal data.** Because re-identification is possible via the mapping, the data is still under privacy obligations. Pseudonymization is risk reduction, not a release from obligation.

### Beware Aggregation That Still Leaks

Aggregation ("report only counts and statistics") feels safe, but leaks in specific ways:

- **Small cell counts.** A report showing "1 user in region X with condition Y" identifies that individual. Suppress or threshold small cells (e.g., suppress counts below 5 or 10).
- **Averaging over tiny groups** can be inverted if the attacker knows group membership.
- **Time-series or longitudinal aggregates** can trace an individual if the group is small or the individual dominates it.
- **Repeated queries with overlapping groups** allow reconstruction (differencing attacks): querying "count of A" and "count of A minus this one person" reveals the person.

Aggregation reduces but does not eliminate re-identification risk. Apply thresholding, and consider differential privacy for releases where reconstruction attacks are feasible.

### Treat Free-Text, Unstructured, And Derived Data As Re-Identification Vectors

Structured fields are the easy part. The hard re-identification vectors are the fields that contain identifying information incidentally:

- **Free-text fields** (comments, notes, descriptions, support tickets) may contain names, emails, phone numbers, or enough context to identify. Strip or redact with NER-based tooling, and accept that redaction is imperfect.
- **Unstructured content** (documents, images, audio) may contain faces, license plates, screen content, or metadata (EXIF) that identifies. Stripping obvious metadata is necessary but not sufficient.
- **Derived data** (embeddings, model outputs, segment memberships) can encode identity, especially if the model memorized individual records. A model trained on too few examples per user may regurgitate identifying data; assess memorization before sharing models.
- **Metadata and join keys.** A pseudonymous id that joins to an identified dataset re-identifies; a timestamp precise enough to match an external event log re-identifies.

Audit the full content of the dataset, not just the structured columns, for identifying information.

### Assume Combination Attacks; Minimize What You Share

The dominant re-identification threat is combination: the attacker joins your "anonymous" release against another dataset to recover identity. The defense is to share less and share coarser.

- **Share the minimum needed.** If the recipient needs aggregate statistics, share aggregates, not row-level data. If they need a sample, share a sample, not the full set.
- **Reduce granularity.** Coarser ZIP (first 3 digits), age bands instead of DOB, date instead of timestamp, region instead of precise location — each generalization shrinks uniqueness.
- **Reduce dimensionality.** Fewer attributes per record means fewer quasi-identifier combinations. Drop columns the recipient does not need.
- **Prefer synthetic data for non-production.** Where the goal is realistic test data, generate synthetic data that preserves statistical properties without real individuals, rather than anonymizing production data (which carries residual re-identification risk into test environments).

The strongest anonymization is the field you never shared. Every reduction in what is released is a reduction in re-identification surface.

## Common Traps

### Calling Stripped Data "Anonymous" When Quasi-Identifiers Remain

Removing names but keeping ZIP, DOB, gender, and timestamps, and labeling the dataset anonymized. Combinations of quasi-identifiers re-identify most individuals; this is pseudonymous data still under obligation. Measure uniqueness before claiming anonymity.

### Pseudonymization Treated As Anonymization

Replacing user ids with tokens and believing the data is no longer personal. Because the mapping can re-identify, the data remains personal data. Pseudonymization reduces risk; it does not remove obligations, and the mapping must be protected.

### Releasing Small Cell Counts

An aggregate report that includes "1 person in region X with condition Y," singling out that individual. Suppress or threshold small cells; do not report counts below a minimum group size.

### Pseudonymous Id That Joins Back To Identified Data

A pseudonymous id in a shared dataset that is also present in an identified internal dataset, so anyone with access to both re-identifies. Ensure shared datasets cannot be joined to identified data the recipient may hold, or accept the data is not anonymous.

### Free-Text Or Unstructured Content Left Intact

Sharing notes, comments, documents, or images that contain names, contact details, or identifying context, while the structured fields were carefully anonymized. Audit and redact unstructured content; strip metadata (EXIF, document properties).

### Differential Privacy With An Unbounded Query Budget

Applying differential privacy noise per query but allowing unlimited queries, so the cumulative privacy loss (epsilon) grows until the guarantee is meaningless. Track and cap the privacy budget across all releases.

### Sharing Row-Level Data When Aggregates Would Do

Releasing row-level records because "they're anonymized," when the recipient only needed aggregate statistics. Share the minimum granularity required; row-level data has far higher re-identification risk than aggregates.

### Assuming A Model Or Embedding Cannot Leak Identity

Sharing a model or embeddings trained on user data without assessing memorization, when the model may regurgitate identifying records. Assess memorization (especially for under-represented users) before sharing models trained on personal data.

### Mapping Kept With The Pseudonymous Data

Storing the pseudonymization mapping in the same database or with the same access as the pseudonymous data, so one compromise undoes the pseudonymization. Separate the mapping into a distinct trust boundary with least-privilege access.

## Self-Check

- [ ] Anonymization and pseudonymization are distinguished honestly: nothing is labeled "anonymous" unless re-identification risk was assessed against a realistic adversary with auxiliary data and found to be low; pseudonymous data is treated as still under privacy obligations.
- [ ] Re-identification risk was measured from quasi-identifier combinations (ZIP/DOB/gender, timestamps, location, rare values), not assumed from the removal of direct identifiers; the fraction of unique records is known, not guessed.
- [ ] Where re-identification risk is real, a formal model (k-anonymity, l-diversity, t-closeness) or differential privacy is applied, the resulting k or epsilon is computed and documented, and the model's limitations (e.g., k-anonymity does not stop attribute disclosure) are understood.
- [ ] Pseudonymization mappings are stored separately from the pseudonymous data under least-privilege access, treated as security-critical secrets, and rotated or destroyed when no longer needed; the pseudonymous data is treated as personal data.
- [ ] Aggregate releases suppress or threshold small cell counts, and repeated-query / differencing attacks are considered (with differential privacy budget accounting where applicable).
- [ ] Free-text, unstructured content, metadata (EXIF, document properties), embeddings, and model outputs were audited for identifying information and redacted or assessed for memorization before release.
- [ ] The minimum necessary data is shared at the coarsest viable granularity (aggregates over rows, generalized quasi-identifiers, fewer columns, samples over full sets), and synthetic data is preferred for non-production use over anonymized production data.
- [ ] Shared datasets were checked for join keys that could re-identify against data the recipient may hold; the release does not enable combination attacks that recover identity.
- [ ] For binding obligations, the anonymization approach was reviewed against the applicable regime's standard for anonymization (which is strict) and a qualified reviewer, not relied on as legal advice.
