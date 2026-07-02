---
name: transaction_monitoring_alert_investigation_and_disposition.md
description: Use when the agent is investigating transaction monitoring alerts, dispositioning alerts as true positive false positive or escalated, defining investigation standards and evidence requirements, or managing false positive reduction without suppressing genuine suspicious activity.
---

# Transaction Monitoring Alert Investigation And Disposition

An alert is not resolved by a glance. The value of a transaction monitoring system is realized only when alerts are investigated with genuine analysis, dispositioned with consistent standards, and documented with evidence that an examiner can reconstruct. Under FATF Recommendation 20, the BSA, the EU AMLD, FinCEN expectations, and equivalent national regimes, obliged entities must not only generate alerts but investigate them effectively and reach defensible conclusions. The central judgment problem is that alert investigation is where monitoring programs most often fail in practice. Analysts close alerts without genuine analysis, dispositions lack rationale, false positives are reduced by suppression rather than tuning, and genuine suspicious activity is buried under volume. An alert that is closed without investigation is indistinguishable from no monitoring at all.

Use this skill before defining alert investigation standards, disposition procedures, evidence requirements, or false positive reduction strategies. The goal is to make the agent think about investigation depth, disposition consistency, evidence quality, and the decisions that are easy to make too casually. A monitoring program that produces many alerts but few defensible conclusions is not effective.

This skill addresses jurisdiction-specific obligations. Investigation standards, disposition documentation, and SAR decision requirements differ across FATF member states and national regimes. Always confirm the applicable national law and regulator guidance, and consult qualified AML or legal counsel for specific investigation and disposition decisions.

## Core Rules

### Investigate Alerts With Genuine Analysis

An alert requires analysis of the underlying activity, the customer context, and whether the behavior is explainable. Confirming that the data matches the rule is not investigation.

Investigation should examine:

- the full set of transactions that triggered the alert, not just the triggering transaction;
- the customer's expected activity as defined in CDD;
- historical behavior and prior alerts for the customer;
- related accounts, counterparties, and beneficial owners;
- external data such as adverse media, sanctions, and PEP status;
- supporting documentation such as invoices, contracts, or shipping records where relevant;
- whether the activity is consistent with the customer's stated business and occupation;
- the economic purpose of the transactions, where one should exist.

The analyst should reach a reasoned conclusion based on this analysis. An alert closed with a note that the transactions were reviewed, without explaining what was found and why it is or is not suspicious, is not a defensible disposition.

### Apply Consistent Disposition Standards

Dispositions must be consistent and defensible across analysts and over time. Inconsistent disposition of similar alerts is an examination finding and a sign of weak investigation standards.

Standard disposition outcomes:

- true positive: the activity is suspicious or unusual and warrants further action, EDD, or SAR filing;
- false positive: the activity is fully explained by expected behavior and documented customer context;
- escalated: the activity is ambiguous and requires deeper review, EDD, or referral to a senior investigator.

For each disposition, the record should capture the rationale, the evidence reviewed, the analyst, the reviewer, and the date. A disposition of no action with no rationale is not a disposition. The institution should define what constitutes a sufficient rationale for each outcome type and enforce it through quality assurance.

### Read The Alert Against Customer Context

An alert that looks unusual in isolation may be entirely consistent with a documented customer profile. Conversely, normal-looking activity may be suspicious for a high-risk customer. The alert must always be read against CDD.

Context factors to apply:

- the customer's expected activity, occupation, and business type;
- the customer's risk rating and any EDD conditions;
- whether the activity matches the stated purpose of the account;
- whether counterparties are consistent with the customer's business;
- whether the geography is consistent with the customer's profile;
- prior alerts and their dispositions for the same customer.

An alert investigated without CDD context risks both false positives, closing genuinely suspicious activity as normal, and false negatives, escalating benign activity that the profile explains.

### Manage False Positives Through Tuning Not Suppression

High false positive rates erode analyst attention and hide real cases. Reducing false positives is legitimate, but it must be achieved through documented tuning, not silent suppression.

Legitimate false positive reduction:

- refine thresholds using segment-level data;
- incorporate expected activity and customer context into the rule logic;
- add suppression logic for known recurring benign patterns;
- add filters for documented benign explanations;
- improve data quality and counterparty enrichment;
- apply feedback loops from confirmed true positives.

Illegitimate reduction includes silently widening thresholds, blanket-closing alerts by type without review, or reducing investigation depth to increase throughput. Every tuning change should be documented, tested, and approved.

### Document The Investigation Defensibly

An examiner reconstructs the investigation from the record. Each closed alert should show what was reviewed, what was found, what conclusion was reached, and who reached it.

Documentation should include:

- the transactions reviewed and the time period covered;
- the customer context applied, including expected activity and risk rating;
- the external data checked, such as sanctions, PEP, and adverse media;
- the rationale for the disposition in specific terms;
- the analyst and reviewer identities and the disposition date;
- any follow-up action, such as EDD, monitoring change, or SAR referral.

A disposition record that contains only a closure code and no narrative is indefensible. The narrative should explain the reasoning, not merely state the outcome.

### Apply Quality Assurance To Investigations

Quality assurance (QA) reviews investigation quality and catches deficiencies before an examiner does. A monitoring program without QA cannot demonstrate that dispositions are sound.

QA scope:

- sample closed alerts, particularly false positives, to verify the rationale;
- review escalated and true positive alerts for investigation completeness;
- check consistency of dispositions across analysts and teams;
- verify that SAR referrals were made when warranted;
- track QA findings and feed them back into training and tuning;
- assess whether investigation depth matches the risk of the alert.

QA findings should be tracked, trended, and used to improve both investigation quality and scenario tuning. A high QA exception rate is a sign that investigation standards need reinforcement.

### Escalate To SAR When The Standard Is Met

When investigation concludes that activity is suspicious, the alert must be escalated for SAR consideration. The decision to file or not file must follow the institution's SAR procedures and the applicable regulatory standard.

SAR escalation considerations:

- does the activity meet the regulatory threshold for suspicion under the applicable regime;
- is the suspicion based on specific facts and analysis, not just the alert;
- are continuing activity reviews needed if the relationship continues;
- is the SAR filed within the regulatory deadline;
- is supporting documentation retained for the required period;
- is tipping off prohibited and enforced.

Failing to escalate a true positive to SAR consideration is a serious deficiency. The escalation path from alert to investigation to SAR decision must be clear and enforced.

## Common Traps

### Closure Without Genuine Analysis

Closing an alert because the transactions were reviewed, without explaining what was found, is not investigation. Examiners will challenge empty closures.

### Dispositions Without Rationale

A closure code without narrative cannot be defended. The rationale must be specific to the activity and the customer.

### Ignoring Customer Context

Investigating an alert in isolation, without CDD, produces both false positives and false negatives. The alert must be read against the customer profile.

### False Positive Reduction By Suppression

Silently widening thresholds or blanket-closing alerts reduces volume but also reduces detection. Reduction must be documented and tuned.

### No Quality Assurance

A program without QA cannot demonstrate disposition quality. QA is essential to catch deficiencies before examination.

### Failure To Escalate To SAR

A true positive that is not escalated for SAR consideration is a missed reporting obligation. The escalation path must be enforced.

### Inconsistent Dispositions Across Analysts

Similar alerts dispositioned differently by different analysts signal weak standards. Consistency must be enforced through standards and QA.

## Self-Check

- Are alerts investigated with genuine analysis of transactions, customer context, related parties, and external data?
- Are dispositions consistent across analysts, with true positive, false positive, and escalation outcomes documented with specific rationale?
- Is each alert read against the customer's CDD profile, expected activity, and risk rating?
- Are false positives reduced through documented tuning rather than silent suppression or blanket closure?
- Does each disposition record include the transactions reviewed, context applied, external data checked, rationale, analyst, reviewer, and date?
- Is quality assurance applied to closed alerts, with findings tracked and fed back into training and tuning?
- Are true positives escalated to SAR consideration following the institution's procedures and the applicable regulatory standard?
- Are SAR decisions documented with specific facts, filed within deadlines, and supported by retained documentation?
- Is tipping off prohibited and enforced in the escalation process?
- Is the investigation and disposition design confirmed against the applicable national law and regulator guidance rather than a generic standard?