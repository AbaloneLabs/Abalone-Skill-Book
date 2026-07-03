---
name: forensic_techniques.md
description: Use when the agent is applying forensic examination techniques, data-mining for fraud red flags, running Benford's Law or duplicate detection, performing link analysis, tracing concealed assets or transactions, detecting ghost employees or vendors, or examining documents and lifestyle indicators.
---

# Forensic Techniques

Forensic techniques are powerful precisely because they surface patterns that humans miss, but every technique produces false positives and can be misread as proof. Agents often run a Benford's test, flag a cluster of round-dollar transactions, or build a link diagram and then treat the output as a finding rather than a lead. The value of these techniques lies in disciplined interpretation: understanding what each test can and cannot prove, controlling the data quality that determines whether the result is meaningful, and corroborating analytics with documents and interviews before concluding. A technique applied to dirty data, or interpreted without business context, misdirects the investigation and can accuse the innocent.

Use this skill when selecting and applying forensic data analytics, document examination, link analysis, or asset-tracing techniques during an investigation or enhanced audit. The goal is to choose the right technique for the scheme, run it on validated data, interpret red flags with skepticism, and convert leads into corroborated findings.

## Core Rules

### Validate Data Before Trusting Any Analytic Output

Every forensic technique depends on the completeness, accuracy, and field integrity of the underlying data. A duplicate-payment test run on an extract that excludes manual journals or a second ledger will miss the duplicates entirely. Data validation is not a preliminary chore; it is the foundation that determines whether the result means anything.

Before running analytics, confirm:

- the source system and whether it is the system of record;
- extraction date, time zone, and filters applied;
- reconciliation of record counts and totals to the general ledger;
- inclusion of manual entries, adjustments, and voided or reversed items;
- completeness of key fields (vendor, employee, account, date, amount);
- handling of multiple currencies, entities, and locations;
- duplicate identifier issues and merged master records;
- data type and format consistency;
- treatment of nulls, blanks, and default values;
- confirmation from the data owner that the extract is complete.

Document the validation. An analytic on unvalidated data is an analytic that cannot be defended.

### Match The Technique To The Suspected Scheme

No single technique detects all fraud. Choosing a test because it is familiar, rather than because it fits the scheme, wastes effort and creates false confidence. Start from the fraud hypothesis and select techniques that would surface evidence of that specific scheme.

Map techniques to schemes:

- ghost employees: payroll-to-HR reconciliation, duplicate bank accounts, no-deduction employees;
- ghost vendors: vendor master review, address overlap with employees, sequential invoice numbers;
- duplicate payments: same amount, vendor, date, and invoice number across systems;
- kickbacks: round-dollar payments, below-threshold splits, vendor concentration;
- financial statement fraud: unusual journal entries, period-end clustering, estimate bias;
- concealed accounts: bank reconciliations, intercompany plugging, unsupported suspense entries;
- skimming: gaps in sequence, voids after the fact, missing deposits.

State which scheme each test targets before running it.

### Apply Benford's Law With Discipline

Benford's Law describes the expected frequency of leading digits in naturally occurring numbers. Deviations can indicate manipulation, but the test is frequently misapplied. It fails on data that is not naturally scaled, that is assigned, capped, or clustered, and it produces noise as much as signal.

Apply Benford's correctly by:

- using data sets large enough to be meaningful (generally hundreds of records);
- restricting to naturally occurring, unbounded amounts;
- excluding assigned numbers, thresholds, and recurring fixed amounts;
- stratifying by account or transaction type rather than mixing populations;
- testing first, first-two, and second digits as appropriate;
- comparing to expected frequencies with a documented test;
- investigating significant deviations, not minor ones;
- corroborating deviations with documents before concluding manipulation.

Benford's is a pointer, never a conclusion.

### Design Duplicate And Below-Threshold Tests Precisely

Duplicate and just-below-threshold tests catch common concealment, but imprecise parameters create either a flood of false positives or silent misses. The parameters must reflect how the scheme would actually be split or hidden.

Tune parameters for:

- exact versus fuzzy matching on amount, date, vendor, and invoice;
- tolerance bands for near-duplicates and transposition errors;
- threshold values tied to actual approval limits, not arbitrary round numbers;
- splitting just below authorization thresholds;
- sequential or gap analysis on invoice and check numbers;
- same bank account across employees or vendors;
- same address, phone, or contact across ostensibly unrelated parties;
- weekend, holiday, and after-hours timestamps;
- round-dollar amounts disproportionate to the population;
- payments to vendors created shortly before payment.

Review a sample of hits to confirm the parameters are calibrated before scaling.

### Build Link Analysis To Expose Hidden Relationships

Link analysis reveals connections among employees, vendors, bank accounts, addresses, and transactions that are invisible in a list. Its value depends on clean master data and on interpreting relationships rather than assuming guilt from a link. A shared address may be a coincidence, a family member, or a shell entity.

Use link analysis to surface:

- vendor-employee address, phone, or bank overlap;
- common banking details across multiple vendors;
- layered ownership or nominee structures;
- intercompany flows that net to zero with no business purpose;
- clusters of payments around the same dates or amounts;
- introducers or approvers common to suspicious transactions;
- entities that exist only to receive and pass funds.

Validate each relationship against business records before treating it as evidence of collusion.

### Trace Assets And Lifestyle Against Known Income

Asset and lifestyle analysis tests whether a person's visible means exceed their known income, a classic indicator of concealed proceeds. The technique requires lawful access to data and conservative interpretation, because lifestyle is easy to overstate and hard to prove. Document the basis for every comparison.

Compare and document:

- declared income and benefits against asset acquisitions;
- property, vehicle, and significant purchases;
- bank and brokerage activity where lawfully available;
- travel and entertainment spending patterns;
- debt reduction inconsistent with income;
- cash transactions and structured deposits;
- third-party payments on the subject's behalf;
- unexplained transfers from related parties;
- timing of acquisitions relative to suspected scheme periods;
- the gap between lifestyle and means, quantified.

Avoid speculation about assets you cannot evidence; note the limitation.

### Examine Documents For Indicators Of Alteration

Forged or altered documents are common in fraud, and many alterations leave physical or digital traces. Document examination is a specialist skill, but every investigator should recognize the indicators that warrant escalation. Never alter the original; examine a copy and preserve the original for forensic review.

Look for:

- inconsistent fonts, spacing, or formatting within a document;
- misaligned stamps, signatures, or letterheads;
- overwritten, whited-out, or re-typed fields;
- dates inconsistent with document version or metadata;
- invoices with sequential numbers from different periods;
- missing or generic supporting documents;
- photocopies presented where originals should exist;
- metadata inconsistent with the document's stated origin;
- signatures that appear stamped or scanned;
- amounts or payees that look added after the fact.

Escalate suspected alteration to a qualified examiner rather than asserting forgery.

### Sequence And Combine Techniques For Convergence

A single technique rarely proves fraud. Convergence of independent techniques is what converts a lead into a defensible finding. Plan the sequence so each technique narrows the population for the next, and so independent evidence corroborates the same conclusion.

Sequence toward convergence:

- start with data validation and population reconciliation;
- run broad red-flag analytics to focus the field;
- apply scheme-specific tests to the focused population;
- build link analysis around the flagged parties;
- trace assets and lifestyle for the key subjects;
- examine documents for the transactions flagged;
- corroborate analytics with source documents;
- test findings in interviews only after corroboration;
- quantify and document the converging evidence;
- state explicitly what each technique independently supports.

Convergence, not any single test, is the standard for a forensic conclusion.

### Interpret Red Flags As Leads, Not Findings

Red flags are conditions that warrant inquiry, not proof of wrongdoing. Investigators who convert a red flag directly into an accusation expose the entity to defamation and wrongful-action claims and undermine the investigation. Every red flag must be explained, corroborated, or eliminated before it becomes a finding.

For each red flag, determine:

- the legitimate business explanations to rule out;
- the documents that would confirm or refute it;
- the system or process reasons it might occur;
- whether it is isolated or part of a pattern;
- the corroborating evidence from other techniques;
- the materiality and risk if it is fraud;
- the further procedures needed before concluding;
- whether it should be escalated to counsel;
- how it will be documented if it remains unexplained;
- the standard of proof required for the next step.

## Common Traps

### Running Analytics On Unvalidated Data

An extract missing manual entries or a second entity produces clean-looking results that miss the fraud. Validate and reconcile the population before any test.

### Treating Benford's Deviations As Proof Of Fraud

Deviations often reflect the nature of the data, assigned numbers, or thresholds, not manipulation. Investigate and corroborate before concluding.

### Flooding The File With False Positives

Over-broad parameters generate so many hits that the real exceptions are never examined. Calibrate parameters and review a sample before scaling.

### Ignoring False Negatives From Fuzzy Matches

Exact-match duplicate tests miss transpositions, rounded splits, and slightly altered payees. Combine exact and fuzzy logic to avoid silent misses.

### Inferring Guilt From A Single Link

A shared address or bank account can be innocent, familial, or coincidental. Validate the relationship and rule out legitimate explanations.

### Overstating Lifestyle Evidence

Estimating someone's lifestyle from partial data invites challenge and defamation risk. Quantify only what is evidenced and state limitations.

### Asserting Forgery Without A Qualified Examiner

Visual suspicion of alteration is not expert proof. Preserve the original and escalate to a document examiner before alleging forgery.

### Presenting Technique Output As The Conclusion

A list of flagged transactions is a work product, not a finding. Convert leads through corroboration and document the convergence.

## Self-Check

- Was the underlying data validated, reconciled to the ledger, and confirmed complete before any analytic was run?
- Does each technique map to a specific suspected scheme rather than being chosen by familiarity?
- If Benford's Law was used, is the population appropriate, stratified, and interpreted with documented limitations?
- Are duplicate and threshold tests calibrated with exact and fuzzy parameters tied to real approval limits?
- Does link analysis rest on clean master data, with each relationship validated against business records?
- Is asset and lifestyle analysis based on lawfully obtained, evidenced data with stated limitations?
- Are suspected document alterations escalated to a qualified examiner rather than asserted as forgery?
- Do multiple independent techniques converge on the same conclusion before it is treated as a finding?
- Is every red flag either explained, corroborated, or eliminated, with unexplained items documented?
- Are technique outputs clearly separated from conclusions, with corroboration and convergence documented?
