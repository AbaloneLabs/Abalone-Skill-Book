---
name: structuring_and_smurfing_detection.md
description: Use when the agent is detecting structuring and smurfing below currency transaction reporting thresholds, aggregating reportable transactions, managing CTR triggers and exemptions, or investigating evasion of BSA reporting requirements.
---

# Structuring And Smurfing Detection

Structuring is the practice of breaking transactions into smaller amounts to evade currency transaction reporting (CTR) and other recordkeeping requirements. Smurfing is a subset in which multiple individuals (smurfs) are used to deposit or move funds below the threshold. Under the Bank Secrecy Act, 31 U.S.C. 5324 makes it a crime to structure transactions for the purpose of evading reporting requirements, and FinCEN expects institutions to detect and report structuring. FATF and EU AMLD impose equivalent obligations. Structuring is both a standalone offense and one of the clearest money laundering red flags, because the act of evading reporting is itself suspicious.

Use this skill before designing structuring detection scenarios, aggregating transactions for CTR purposes, managing exemption lists, or investigating suspected evasion. The goal is to make the agent think about aggregation logic, intent, the boundary between legitimate convenience and evasion, and the reporting obligations that structuring triggers. Missing structuring is a classic examination finding, and over-reporting without intent analysis wastes resources.

## Core Rules

### Know The Reporting Threshold And Aggregation Rules

The US CTR threshold is currency transactions exceeding 10,000 USD per business day. Multiple currency transactions aggregated to the threshold on a single business day by or on behalf of the same person must be treated as a single transaction for CTR purposes. Equivalent thresholds exist in other regimes.

Aggregation rules:

- same person, same business day, multiple transactions;
- transactions conducted by or on behalf of the same person;
- across multiple branches or accounts where the institution has knowledge;
- currency (cash) instruments, not wires or checks;
- the institution must aggregate when it has knowledge of common ownership or control.

The institution must have systems that aggregate across accounts, branches, and time windows where knowledge exists. Failure to aggregate is a reporting failure.

### Detect Structuring Patterns

Structuring detection looks for patterns designed to stay below the threshold. The intent to evade is the key element, inferred from the pattern.

Structuring patterns:

- multiple cash deposits just below 10,000 USD on the same or consecutive days;
- deposits of 9,000 to 9,999 USD repeated across days;
- cash deposits split across branches or ATMs on the same day;
- withdrawals just below the threshold;
- exchanges of small bills for large bills or vice versa to facilitate structuring;
- third parties conducting deposits on behalf of one beneficiary;
- activity that tracks the threshold after the customer is informed of reporting requirements.

The pattern of amounts just below the threshold, repeated and coordinated, is the strongest indicator.

### Recognize Smurfing And Third-Party Aggregation

Smurfing uses multiple people to break up funds. It is common in narcotics proceeds, where cash is deposited by several individuals into one or more accounts.

Smurfing indicators:

- multiple unrelated individuals depositing cash into the same account;
- deposits made in different regions or branches on the same day;
- similar deposit amounts and timing across different depositors;
- depositors who appear coordinated or arrive together;
- funds rapidly withdrawn or wired after aggregation;
- accounts used as funnels to collect and move structured deposits.

When smurfing is suspected, identify the beneficiary account and trace the flow of aggregated funds.

### Distinguish Structuring From Legitimate Activity

Not all sub-threshold transactions are structuring. The institution must assess intent. Legitimate reasons for breaking up transactions exist, but they must be plausible and documented.

Legitimate considerations:

- customer's business naturally generates cash in amounts below the threshold;
- deposits reflect daily receipts from a cash business;
- customer was unaware of the threshold;
- timing reflects genuine business cycles.

However, once a customer is aware of reporting and then adjusts behavior to stay below the threshold, intent to evade is strongly suggested. Document the analysis of intent.

### File CTRs And SARs Correctly

Structuring triggers both reporting and suspicious activity obligations. If currency transactions are aggregated to exceed the threshold, a CTR is required. If the structuring is suspicious, a SAR is required, and structuring is per se suspicious when intent to evade is present.

Reporting actions:

- file a CTR for aggregated currency transactions exceeding the threshold;
- file a SAR for structuring activity, including the pattern and the basis for suspicion;
- reference the structuring typology in the SAR narrative;
- retain supporting documentation for five years.

Do not assume a CTR substitutes for a SAR. They serve different purposes.

### Manage Exemptions Properly

To reduce unnecessary CTRs on legitimate high-volume cash businesses, the BSA allows exemption of certain customers from CTR reporting. Exemptions must be managed rigorously.

Exemption rules:

- eligible customers include listed companies, government entities, and qualifying businesses (phase I and phase II exemptions);
- the customer must have a transaction account;
- the institution must conduct a review and file an initial exemption and annual renewals where required;
- exempt customers must still be monitored for structuring and suspicious activity;
- exemptions do not waive SAR obligations.

An exempt customer that begins structuring must still be reported. Exemptions reduce CTR burden, not monitoring obligation.

### Monitor Across Channels And Accounts

Structuring increasingly occurs across ATMs, mobile deposits, branches, and third-party networks. Detection must span channels and aggregate where the institution has knowledge. Isolated per-branch monitoring misses cross-channel structuring.

## Common Traps

### Missing Aggregation Across Branches

If branches do not share information about same-day deposits by the same customer, the institution fails to aggregate and misses both CTRs and structuring.

### Treating Sub-Threshold As Automatically Benign

Amounts just below the threshold, repeated and coordinated, are the hallmark of structuring. Do not dismiss them as normal.

### Filing A CTR But Not A SAR

Structuring with intent to evade is suspicious and requires a SAR. A CTR alone does not discharge the suspicion obligation.

### Over-Relying On Customer Explanation

Customers rarely admit to structuring. A claim that deposits are daily receipts must be tested against the business profile and the pattern.

### Letting Exemptions Create Blind Spots

Exempt customers can still structure. Monitoring must continue regardless of exemption status.

### Ignoring Smurfing Because Depositors Differ

Smurfing is defined by multiple depositors serving one beneficiary. Focus on the beneficiary account, not only the individual depositors.

### Failing To Detect Post-Notice Behavior

If a customer changes behavior after being told about reporting, that change is strong evidence of intent and should be flagged.

## Self-Check

- Does the institution aggregate currency transactions by or on behalf of the same person across accounts, branches, and channels where it has knowledge?
- Are structuring detection scenarios tuned to amounts just below the threshold, repeated deposits, and coordinated timing?
- Is smurfing detected by identifying multiple depositors serving a common beneficiary account?
- Is intent assessed, distinguishing legitimate sub-threshold activity from evasion, with the analysis documented?
- Are CTRs filed for aggregated transactions exceeding the threshold and SARs filed for structuring with intent?
- Are exemptions managed with eligibility checks, renewals, and continued monitoring for structuring?
- Does detection span ATMs, mobile deposits, branches, and third-party networks?
- Are post-notice behavior changes flagged as evidence of intent?
- Is supporting documentation retained for the required period?
- Are structuring SAR narratives specific about the pattern, amounts, and basis for suspicion?
