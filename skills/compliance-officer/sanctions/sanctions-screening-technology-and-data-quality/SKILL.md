---
name: sanctions_screening_technology_and_data_quality.md
description: Use when the agent is configuring or tuning a sanctions screening engine, setting fuzzy match thresholds, managing list update cadence and data field quality, evaluating audit trails, or assessing whether screening technology is actually catching sanctioned parties.
---

# Sanctions Screening Technology And Data Quality

Screening technology is only as good as the data it ingests, the lists it loads, the thresholds it applies, and the audit trail it leaves. A screening engine that runs every day but is fed incomplete customer records, stale lists, or a threshold tuned to suppress alerts will produce a clean-looking result that means nothing. Regulators examining sanctions failures routinely find that the technology existed but was misconfigured, under-tuned, or never validated against real targets. The compliance officer must be able to interrogate the engine, not merely trust it.

Use this skill before advising on screening tool selection, threshold tuning, list management, data quality remediation, audit trail design, or the testing needed to prove the engine works. The objective is to ensure the technology would actually catch a sanctioned party, and that the evidence of that capability exists.

## Core Rules

### Match The Engine Capability To The Sanctions Risk Profile

Different screening engines have different strengths. Select and configure the engine against the institution's actual exposure.

Consider:

- name-matching algorithms: exact, phonetic, transliteration, and fuzzy variants;
- support for non-Latin scripts and transliteration from Cyrillic, Arabic, and CJK characters;
- entity matching versus individual matching versus vessel and aircraft matching;
- ability to screen free-text payment and trade documentation fields;
- list coverage and the ease of adding new lists;
- throughput and latency for real-time payment screening;
- case management and workflow for alert disposition;
- audit trail and reporting granularity.

An engine adequate for low-volume customer screening may be inadequate for high-throughput cross-border payments. Match the tool to the volume and the data shape.

### Manage List Currency With Defined Update Cadence

A screening engine running on yesterday's list can miss today's designation. List currency is a control, not a background function.

- Confirm the source and update frequency for OFAC, EU, UN, and UK lists.
- For OFAC, account for intraday updates; designations can occur any business day.
- Record the list version and load timestamp against each screening event so the list state at decision time is reconstructable.
- Monitor for failed or delayed list loads and alert on them as incidents.
- Test that a newly added test entry appears in screening within the expected window.

A list that silently fails to update is worse than no list, because it creates false confidence.

### Tune Fuzzy Match Thresholds Deliberately

Thresholds control the trade-off between false positives and false negatives. Both extremes are dangerous.

- A threshold set too high suppresses real matches and lets sanctioned parties through.
- A threshold set too low floods analysts with alerts, causing fatigue and rote clearing.
- Tune by name type: common names need stricter identifier matching; rare names need broader recall.
- Tune by jurisdiction and script: transliterated names require different scoring than Latin-script names.
- Validate tuning against a test set of known-listed parties and known-good parties.
- Re-tune when lists change materially or when alert volumes shift unexpectedly.

Document the tuning rationale, the test set used, and the resulting threshold values. A threshold with no documented basis cannot be defended.

### Ensure Data Field Quality Drives Match Quality

The engine cannot match identifiers that were never captured. Data quality is the upstream determinant of screening effectiveness.

Capture and maintain:

- full legal name and all known aliases;
- date of birth and place of birth for individuals;
- national identifiers, passport, or tax ID where available;
- nationality and residency;
- address and country;
- for entities: registration number, registered address, and beneficial owners;
- for vessels: IMO number, call sign, flag, and registered owner;
- for payments: originator, beneficiary, and free-text reference fields.

Missing identifiers force the engine to rely on name alone, which is the weakest signal. Treat data capture standards as a screening control.

### Design An Audit Trail That Reconstructs Each Decision

The audit trail must allow a regulator to reconstruct, for any transaction, what was screened, against which list version, what matched, how it was disposed, by whom, and why.

- Record the input data, the list version, the match score, and the matched entry.
- Record the analyst, the disposition decision, and the rationale.
- Preserve the trail immutably for the required retention period.
- Ensure the trail cannot be altered by operations staff.

If the audit trail cannot reconstruct the decision, the screening cannot be proven to have occurred.

### Validate The Engine With Test Names Periodically

A passive assumption that the engine works is not a control. Inject known-listed test names and confirm they alert. Inject known-good names and confirm they do not alert at the configured threshold. Track the pass rate and investigate failures. This is the single most effective way to detect silent degradation.

## Common Traps

### Tuning Thresholds To Suppress Alert Volume

Reducing alert fatigue by raising thresholds feels productive but silently increases false negatives. Threshold changes must be validated against a test set, not chosen for analyst comfort.

### Trusting The List Is Current Without Monitoring

List loads can fail silently. Without monitoring and alerting on load failures, the engine runs on stale data with no one aware.

### Screening Structured Name Fields Only

Sanctions evasion uses payment references and trade documentation. Screening only the name field misses geography, vessel, and party references embedded in free text.

### Capturing Insufficient Identifiers At Onboarding

If date of birth, nationality, or entity registration number are not captured, the engine must rely on name alone, producing either excessive false positives or missed true matches.

### Assuming No Alerts Means The Engine Worked

A clean result may reflect a broken population, a stale list, or a misconfigured threshold. Validate with test names rather than inferring effectiveness from silence.

### Failing To Record List Version Against Each Screen

Without the list version and timestamp, it is impossible to prove which designations were in scope at the time of screening. This gap is fatal in an enforcement review.

### Altering Or Allowing Edits To The Audit Trail

An audit trail that operations staff can modify after the fact has no evidentiary value. Immutability must be enforced technically.

### Using One Threshold For All Names And Scripts

A single global threshold underperforms for common names, rare names, and transliterated scripts. Segment tuning by name type and script.

## Self-Check

- Is the screening engine's capability matched to the institution's volume, data shape, and jurisdictional exposure rather than selected generically?
- Are OFAC, EU, UN, and UK list update cadences documented, with monitoring and alerting on failed or delayed loads?
- Is the list version and load timestamp recorded against each screening event so the list state at decision time is reconstructable?
- Are fuzzy match thresholds tuned by name type, jurisdiction, and script, with the rationale and test set documented?
- Has tuning been validated against a test set of known-listed and known-good parties, with re-tuning triggered by list or volume changes?
- Are onboarding data standards defined to capture full names, aliases, dates of birth, identifiers, nationality, entity registration, and vessel data?
- Are free-text payment and trade documentation fields screened, not only structured name fields?
- Does the audit trail capture input data, list version, match score, matched entry, analyst, disposition, and rationale immutably?
- Is the engine periodically tested by injecting known-listed names to confirm it alerts, with failures investigated?
- Is the pass rate of validation testing tracked over time to detect silent degradation?
