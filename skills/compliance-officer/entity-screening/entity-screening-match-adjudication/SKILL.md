---
name: entity-screening-match-adjudication.md
description: Use when the agent is adjudicating sanctions or watchlist screening hits, deciding whether a potential match is a true or false positive, calibrating match-score thresholds, reviewing fuzzy-match logic, handling near-matches and partial identifiers, documenting disposition decisions, or managing alert backlogs in a compliance screening operation.
---

# Entity Screening Match Adjudication

Screening systems produce alerts; humans make the disposition. The hardest and highest-risk moment in any entity-screening operation is the adjudication step — the moment a compliance analyst decides whether an alert is a true match requiring action (blocking, reporting, asset freeze) or a false positive that can be cleared. A wrong "false positive" call lets a sanctioned party transact; a wrong "true match" call freezes an innocent counterparty and creates legal, reputational, and operational harm. The skill exists because adjudication is not a binary yes/no reflex but a judgment under uncertainty that depends on identifier quality, list semantics, ownership context, and the legal consequences of each disposition.

## Core Rules

### Treat adjudication as a reversible decision with asymmetric costs

The cost of a false negative (missing a real sanctioned party) and a false positive (blocking an innocent party) are both severe but asymmetric. A false negative exposes the organisation to enforcement, fines, and criminal liability. A false positive causes business disruption, customer harm, and potential litigation. Do not optimise for one at the expense of the other; calibrate thresholds and review depth to the risk tier of the counterparty and the severity of the list.

### Evaluate the totality of identifiers, not a single field

A name match alone is almost never sufficient to confirm a true positive. Confirm or refute a match using the combination of available identifiers:

- Full legal name and any aliases, transliterations, or former names;
- Date of birth or date of registration, and any partial dates;
- Nationality, country of incorporation, or jurisdiction of operation;
- National identification numbers, tax IDs, or registration numbers;
- Address, city, and country;
- Place of birth or principal place of business;
- Any known affiliations, ownership, or control relationships.

Weight identifiers by their discriminating power. A matching national ID number is far more probative than a matching common surname. Document which identifiers matched, which did not, and which were unavailable.

### Understand what the matched list entry actually means

Before confirming a match, read the list entry in full. Determine:

- Which program or authority designated the party (OFAC, UN, EU, UK OFSI, national regulator);
- What the designation scope is — full blocking, sectoral, investment ban, travel ban, or reporting-only;
- Whether the entry includes aliases, "also known as," or "linked to" notes;
- Whether the entry has a general licence or exemption that permits the activity;
- Whether the entry is primary (the party itself) or derived (a party owned or controlled by a designated person).

A match against a reporting-only list requires different action than a match against a blocking list. Never assume the consequence; read the program scope.

### Apply the ownership and control lens to entity matches

For corporate counterparties, a name match may be to a subsidiary, affiliate, or related party. Determine whether the matched party is owned 50 percent or more, or otherwise controlled, by a designated person. If so, the entity itself may be blocked under the 50 Percent Rule (US OFAC) or equivalent aggregation principles in other jurisdictions. Record the ownership chain evidence and the basis for the control determination.

### Require independent review for confirmed true matches

A single analyst should never be the sole decision-maker on a confirmed true match that leads to blocking, freezing, reporting, or termination. Require a second-line review (a different analyst or a supervisor) before any restrictive action is taken. This four-eyes principle reduces individual error and bias and creates an auditable record of independent confirmation.

### Document the disposition and its rationale contemporaneously

Every alert disposition — true positive, false positive, or escalation — must be documented at the time of the decision with:

- The identifiers reviewed and their match status;
- The list and program referenced;
- The reasoning for the disposition;
- The reviewer identity and, for true matches, the second reviewer;
- Any evidence relied upon (corporate registry extracts, ID documents, ownership disclosures).

Reconstructing rationale weeks later during an audit or enforcement inquiry is unreliable and weakens the defensibility of the program.

### Manage alert backlogs with defined aging thresholds

Unadjudicated alerts are latent risk. Define maximum aging limits by risk tier (for example, high-risk alerts cleared within 24-48 hours) and escalate overdue alerts to a supervisor. A growing backlog is itself a compliance deficiency that examiners and auditors will flag.

## Common Traps

### Confirming a match on name alone because the system flagged it

A fuzzy name match on a common name (for example, "Mohammed Ali" or "John Smith") against a list containing thousands of entries is statistically likely to be a false positive. Treating a system alert as proof is an abdication of the adjudication role. Always corroborate with additional identifiers.

### Clearing an alert as false positive because "it is probably not them"

Wishful clearance is the mirror failure. An analyst under time pressure may rationalise differences (a slightly different date of birth, a different country) as sufficient to clear, when the differences may reflect data quality issues, aliases, or deliberate obfuscation. Apply the same evidentiary rigour to clearances as to confirmations.

### Over-relying on match scores without understanding the algorithm

A 95 percent match score sounds authoritative but may be driven by a common name and a coincidental country. Understand how the scoring weights name similarity, phonetic matching, token order, and identifier overlap. A high score on weak identifiers is less meaningful than a moderate score on strong identifiers.

### Ignoring the list version or staleness

Screening against an outdated list snapshot can miss newly added designations or fail to clear parties who have been removed. Confirm the screening engine is using the current list version and that historical transactions are re-screened against current lists where required.

### Treating "no hits" as proof of cleanliness

A clean screening result is only as good as the input data and the list coverage. A "no hit" on a misspelled name or against an incomplete list is not assurance. Validate the quality of the screened data and the comprehensiveness of the list set before relying on a clean result.

### Failing to escalate when identifiers are missing

When a counterparty provides only a name with no date of birth, nationality, or ID number, adjudication is genuinely ambiguous. Do not default to clearing because there is "nothing to contradict" the match. Escalate for enhanced due diligence or additional data collection.

## Self-Check

- Did I confirm the match or clearance using multiple discriminating identifiers, not name alone?
- Did I read the full list entry and understand the program scope and consequences before acting?
- For corporate matches, did I check ownership and control (50 Percent Rule) implications?
- For a confirmed true match, was there independent second-line review before any blocking or restrictive action?
- Is the disposition documented contemporaneously with the identifiers reviewed, reasoning, and reviewer identity?
- Are alert aging thresholds defined, and are any alerts exceeding them?
- Did I validate that the screening used the current list version and that the input data quality was sufficient to rely on the result?
- Where identifiers were missing or ambiguous, did I escalate rather than default to clearance?
