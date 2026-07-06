---
name: pii_handling_and_data_minimization.md
description: Use when the agent is collecting, storing, logging, transmitting, displaying, or deleting personal data such as names, emails, phone numbers, addresses, identifiers, location, IP addresses, biometric or health or financial data; deciding what fields to capture or persist; designing a form, an API payload, a database schema, a log line, an analytics event, or an export that may carry personal data; applying data minimization, purpose limitation, retention limits, or deletion; masking, redacting, anonymizing, or pseudonymizing data; controlling third-party or subprocessor sharing; building privacy by design; or reviewing compliance with GDPR, CCPA/CPRA, HIPAA, or similar regimes. Also covers the distinction between direct and indirect identifiers, re-identification risk, audit and access controls, and the rule against collecting or logging data "just in case."
---

# PII Handling And Data Minimization

Personal data is a liability as well as an asset. Every field you collect, store, log, or transmit is a record about a person that you are now responsible for: it can be breached, subpoenaed, misused, aggregated to infer more than you intended, or retained long after its purpose is gone. The recurring failure in privacy engineering is not weak encryption — it is over-collection: storing data "because we might need it," logging full request bodies to debug, copying production data into a test database, or passing identifiers to a third party without asking why. Each addition feels harmless and expands the blast radius of a future incident. The judgment problem is deciding, for every piece of personal data, whether you truly need it, for what specific purpose, for how long, and who must not see it.

Agents tend to under-invest here because collecting data is the path of least resistance. A form with every field, a log line with the whole payload, a database column "in case marketing wants it" — these are easy to write and produce working software immediately. The harm is deferred and invisible until a breach, a regulator inquiry, or a user's deletion request reveals that the data was never needed, was never deletable, and was sitting in four places you had forgotten about. This skill is deliberately conservative: when uncertain, collect less, retain less, share less, and treat privacy obligations as hard constraints, not preferences. It is not legal advice — for binding obligations, confirm against the applicable regime and a qualified reviewer.

This skill is about handling personal data responsibly in software. It complements the cryptography skill (which covers how to protect data you must keep) and the caching skill (which covers the privacy risks of shared caches). Here the question is what data should exist at all, and how to minimize and control it.

## Core Rules

### Apply Data Minimization First: Collect Only What A Specific Purpose Requires

Before any other decision, ask whether each field is necessary for a defined purpose. Data minimization is the foundational principle underlying most privacy regimes: collect the minimum personal data needed to accomplish the stated purpose, and no more. Data collected "just in case" or "for future analytics" has no defensible purpose and is the most common source of avoidable risk.

For every field you plan to collect or store:

- **Name the specific purpose.** "We need the email to send order confirmations" is a purpose. "We might want it for marketing later" is not — it is speculation that creates permanent liability for a hypothetical benefit.
- **Confirm the purpose actually requires that field.** Does the purpose require the full date of birth, or just an age-check boolean? The full address, or just a country for tax? A precise location, or a city? Collect the least identifying value that serves the purpose.
- **Drop fields that have no current purpose.** If you cannot state the purpose now, do not collect it. You can collect more later when a real purpose exists; you cannot easily un-collect data already stored and replicated.

The strongest privacy posture is the data you never held. Every field not collected is a field that cannot leak, cannot be misused, and cannot complicate a deletion request.

### Identify Both Direct And Indirect Identifiers

Personal data is not only obvious identifiers like names and email addresses. A field is personal data if it relates to an identifiable individual, and identifiability extends far beyond direct identifiers. Misjudging this is how "anonymized" datasets get re-identified.

Classify each field:

- **Direct identifiers.** Name, email, phone, government id, account id, full address, biometric data. These identify a person on their own and require the strongest protection.
- **Indirect / quasi-identifiers.** Date of birth, postal code, gender, employer, job title, partial IP, device fingerprint, precise location, timestamps of activity. Any one may not identify a person, but combinations frequently do: the well-known finding that roughly 87% of the U.S. population is unique by (5-digit ZIP + gender + date of birth) means these "demographic" fields are identifying in combination.
- **Inferred and derived data.** Profiles, scores, segments, and model outputs about a person are personal data too, even if you computed them. A "likely churn risk" score attached to a user is personal data.
- **Linked records.** Data that is not identifying alone but is keyed to an identifier becomes personal data through the link. An "anonymous" session id that is joinable to a user table is not anonymous.

Treat indirect identifiers with the same seriousness as direct ones when they appear together. A dataset that strips names but keeps ZIP + DOB + gender is still identifying for most individuals.

### Make Purpose Limitation Explicit And Enforce It

Personal data collected for one purpose must not be silently repurposed for another. Purpose limitation means the purpose stated at collection time bounds later use, and repurposing requires a new legal basis (often fresh consent). The failure mode is "we collected emails for transactional mail, then used them for marketing, then sold the list" — each step felt incremental, and each was a violation.

- **Record the purpose at collection.** Tie each field to its stated purpose in your data inventory.
- **Gate repurposing.** Before using data for analytics, training, sharing, or a new feature, ask whether the original purpose covers it. If not, obtain a new basis or do not use it.
- **Separate data by purpose where practical.** Transactional data, analytics data, and marketing data often serve different purposes and should be governed separately rather than poured into one warehouse and used freely.

### Set Retention Limits And Make Deletion Real

Data should not live forever by default. Every store of personal data needs a retention rule: how long it is kept, and what happens at the end. Indefinite retention is both a liability and a sign that no one decided.

- **Define a retention period per data category.** Tie it to the purpose: if the purpose is "fulfill the order," retention ends when the order and its legal/financial record-keeping obligation end — not "forever."
- **Automate deletion or anonymization at expiry.** Retention rules that rely on manual action are routinely ignored. Build deletion into the lifecycle (scheduled jobs, TTLs, archival then purge).
- **Account for every copy.** Production database, replicas, backups, logs, analytics warehouse, exports, test/staging copies, caches. A deletion that removes the row from the primary database but leaves it in backups, logs, and a replicated analytics store is not a deletion. Enumerate where each data category lives and ensure the deletion reaches all of them (or document the exception, e.g., backups that age out on a schedule).
- **Honor deletion and access requests.** Users' rights to access, correct, and delete their data are obligations in most regimes. Design the system so these are achievable, not retrofitted under a deadline.

### Control Access And Maintain Auditability

Personal data must be accessible only to those who need it for the purpose, and access must be auditable. Broad access defeats every other control: a database every engineer can query in production is a breach waiting for one compromised credential or one curious employee.

- **Least privilege.** Scope access to the minimum data each role needs. Production personal data should not be routinely accessible to developers; use masked or synthetic data for development.
- **Separate production from non-production.** Never copy real personal data into test, staging, or development environments without anonymization. Production data in a test database is a recurring breach source.
- **Audit access.** Log who accessed what personal data, especially for bulk queries and privileged access. An audit log you cannot query is not a control.
- **Protect the most sensitive categories.** Health, biometric, financial, government-id, children's, and precise location data carry heightened obligations under many regimes. Treat them as a separate, more locked-down tier.

### Choose Masking, Anonymization, Or Pseudonymization Deliberately

These terms are not interchangeable, and confusing them produces false confidence. Each removes identifiability in a different way and provides a different guarantee.

- **Masking / redaction.** Hide part of a value for display or logging (`j••@example.com`). The underlying value still exists in full somewhere; masking protects a specific view, not the store.
- **Pseudonymization.** Replace identifiers with tokens or keys, keeping a mapping that can re-identify (often held separately). This reduces risk but the data is still personal data — re-identification is possible, so obligations largely remain. Useful for limiting exposure within a system, not for "anonymizing" a dataset you publish.
- **Anonymization.** Irreversibly remove or transform identifying information so no one can re-identify individuals, including by combination with other data. True anonymization is hard: it must resist linkage attacks using quasi-identifiers and external datasets. If it succeeds, the result is no longer personal data; if it does not, you have pseudonymous data you are wrongly treating as free of obligation.

Do not label data "anonymous" unless the anonymization genuinely resists re-identification. Stripping a name while keeping unique quasi-identifiers is pseudonymization at best, and often just poorly protected personal data.

### Keep PII Out Of Logs, Analytics, And Error Messages

Logs are a persistent, widely accessible, rarely-deleted store — exactly the wrong place for personal data, yet one of the most common places it leaks. A debug log that prints the full request body, an error message that includes the user's email, or an analytics event that carries the raw identifier all create personal-data stores you did not intend to maintain.

- **Log structures, not payloads.** Log the operation, the outcome, and non-identifying context. Do not log full request/response bodies, headers (which carry tokens and IPs), or form fields by default.
- **Redact known sensitive fields before logging.** Maintain an allowlist of loggable fields or a denylist of fields to scrub, and enforce it in the logging path, not by hoping developers remember.
- **Be careful with identifiers in logs.** Correlation ids and opaque tokens are usually fine; emails, phone numbers, government ids, and raw IP addresses are personal data in the log. Hash or truncate where a stable reference is needed.
- **Scrub error messages.** An exception that includes the input that caused it can write personal data into an error-tracking system. Sanitize before capture.

Treat every log sink, analytics pipeline, and error tracker as a place where personal data, if it arrives, will persist and be hard to remove.

### Control Third-Party And Subprocessor Sharing

Sending personal data to another system extends your responsibility to that system. You remain accountable for what the third party does with it, and each recipient is another potential breach source and another entry in a deletion request.

Before sharing personal data with a third party (analytics, CRM, support tool, ad network, AI/LLM service, cloud subprocessor):

- **Ask whether the sharing is necessary.** The same minimization applies: send the least data needed, not everything you have.
- **Confirm the recipient's obligations.** A data processing agreement, defined purpose, retention, and security commitments. A third party with no contractual privacy obligations is a risk you cannot control.
- **Watch for hidden processors.** SDKs, client-side libraries, and "free" analytics often exfiltrate personal data to vendors you did not consciously choose. Audit what leaves the client and the server.
- **Be especially careful with AI/LLM services.** Sending personal data to a third-party model may store, log, or train on it. Confirm the provider's data handling before sending anything sensitive.
- **Track recipients for deletion.** If a user asks to be deleted, every third party that holds their data must be notified. Maintain the list of recipients as part of the data inventory.

### Build Privacy By Design, Not Privacy By Retrofit

Privacy controls bolted on after a system is built are expensive, incomplete, and frequently too late. Privacy by design means considering data minimization, purpose, retention, access, and deletion as requirements during design, not features to add before launch. Concretely:

- **At feature design, run through the data lifecycle.** What is collected, why, where it is stored, who can access it, how long it lives, how it is deleted, and who it is shared with. Gaps found at design cost hours; gaps found after launch cost incidents.
- **Default to collecting and retaining less.** The safe default is no personal data; add fields only when a purpose demands them.
- **Make deletion and access achievable from the start.** If the architecture cannot answer "delete this user's data everywhere," that is an architectural problem, not a future task.

## Common Traps

### Collecting "Just In Case"

Adding fields, columns, or log lines because they might be useful later, with no current purpose. Each is permanent liability for a hypothetical benefit, and "later" rarely arrives while the data sits and accrues risk. Collect only what a present purpose requires.

### Logging Full Request Bodies Or Errors With Input

Printing the request payload, headers, or the raw input that triggered an exception into logs or error trackers. This silently creates a personal-data store in the logging system that is widely accessible and hard to purge. Log structures and outcomes; redact sensitive fields by default.

### Calling Stripped Data "Anonymous" When Quasi-Identifiers Remain

Removing names but keeping ZIP, date of birth, gender, device fingerprint, or precise timestamps, and labeling the dataset anonymized. Combinations of quasi-identifiers re-identify most individuals, so this is pseudonymous data still under privacy obligations — not free-to-use anonymous data.

### Copying Production Data Into Test Or Staging

Dumping the production database into a development environment for realism, exposing real personal data to every developer and every local machine. Use anonymized or synthetic data for non-production; treat production personal data as access-restricted.

### Indefinite Retention With No Deletion Path

Storing personal data with no retention rule and no deletion mechanism, so it accumulates forever across the primary store, replicas, backups, logs, and analytics. Every category needs a retention period and an automated path to deletion or anonymization — and the path must reach every copy.

### Repurposing Data Beyond Its Collected Purpose

Using emails collected for transactional mail for marketing, or behavioral data collected for the product for ad targeting or model training, without a new basis. Purpose limitation binds later use; repurposing without consent or another legal basis is a violation even if the data was legitimately collected.

### Broad Production Access With No Audit and sending Personal Data To A Third Party Without Checking Its Handling

Letting all engineers query production personal data freely, with no logging. One compromised account or one curious employee is a breach, and with no audit log you cannot detect or investigate it. Least privilege plus auditability is the baseline.

Forwarding identifiers or user content to an analytics vendor, CRM, or AI/LLM service without confirming retention, security, or training practices. You remain accountable for what the recipient does; an unvetted processor is uncontrolled risk. Minimize what you send and confirm the recipient's obligations.

### Treating Pseudonymization As Anonymization and forgetting The Indirect Copies When Deleting

Replacing a user id with a token and believing the data is no longer personal. Because the mapping can re-identify, the data is still personal data under most regimes. Pseudonymization reduces risk; it does not remove obligations.

Deleting a user's row from the primary table and declaring the deletion done, while the data remains in backups, logs, analytics, exports, and caches. A deletion that does not reach every store is incomplete; enumerate the copies as part of the design.

## Self-Check

- [ ] Every personal-data field maps to a specific, current purpose stated at collection time; no field exists "just in case" or for a speculative future use, and the least-identifying value that serves each purpose was chosen.
- [ ] Both direct and indirect identifiers were classified, including quasi-identifiers (ZIP, DOB, gender, device fingerprint, precise location, timestamps) and inferred/derived data — and combinations were assessed for re-identification risk, not just individual fields.
- [ ] Each data category has a retention period tied to its purpose, with automated deletion or anonymization at expiry, and the deletion path was designed to reach every copy (primary, replicas, backups, logs, analytics, exports, caches) — not only the primary table.
- [ ] Access follows least privilege, production personal data is not copied into non-production environments, access to sensitive categories (health, financial, government id, biometric, children's, precise location) is separately locked down, and access is auditable.
- [ ] Masking, pseudonymization, and anonymization are distinguished correctly: nothing is labeled "anonymous" unless re-identification genuinely resists linkage attacks, and pseudonymized data is treated as still under privacy obligations.
- [ ] Logs, analytics events, and error messages do not carry personal data by default; sensitive fields are redacted via an enforced allowlist/denylist, and full request bodies, headers, and raw inputs are not logged.
- [ ] Third-party and subprocessor sharing is minimized, contractually governed, audited for hidden processors (SDKs, client-side libraries, AI/LLM services), and tracked so deletion requests can reach every recipient.
- [ ] Purpose limitation is enforced: data is not repurposed (analytics, marketing, model training) beyond its collected purpose without a new legal basis.
- [ ] Privacy was considered at design time — the full data lifecycle (collect, store, access, retain, delete, share) was worked through before launch, and the system can answer "delete this user's data everywhere" without a retrofit.
- [ ] For binding obligations, the design was checked against the applicable regime (GDPR, CCPA/CPRA, HIPAA, etc.) and a qualified reviewer, not relied on this guidance as legal advice.
