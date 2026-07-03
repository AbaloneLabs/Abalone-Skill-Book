---
name: contract-compliance.md
description: Use when the agent is auditing adherence to contract terms, testing compliance with covenants and obligations under commercial agreements, evaluating vendor or supplier contract performance, auditing grant or funding agreement conditions, assessing penalty and milestone clauses, or determining whether an entity met its contractual duties to customers, partners, regulators, or funding bodies.
---

# Contract Compliance

Contract compliance auditing determines whether the parties to an agreement have fulfilled their obligations and whether the economic and operational reality matches the written terms. Contracts vary enormously in structure, from standardized procurement terms to bespoke joint-venture agreements, and the obligations within them are not always labeled "compliance." The judgment problem is that contract terms are interdependent: a breach of one clause may trigger penalties, termination rights, or acceleration of others. Agents frequently test individual clauses in isolation, miss conditional or contingent obligations, or accept performance reports from the party whose compliance is being audited as sufficient evidence.

## Core Rules

### Obtain and read the complete operative contract, including amendments

Never test compliance against a summary, a draft, or the base agreement alone. Identify the fully executed version and all amendments, side letters, addenda, exhibits, and change orders that modify it. Establish which version was operative for each period under audit, because obligations may have changed mid-term. Confirm the contract was properly executed and is legally in force; testing compliance with an unenforceable or unsigned agreement produces meaningless findings.

### Build a clause-by-clause obligations register

Before testing, decompose the contract into a structured register of obligations, each tagged by:

- the responsible party (who must perform);
- the nature of the obligation (payment, delivery, reporting, confidentiality, insurance, audit cooperation);
- the timing or trigger (milestone, recurring date, conditional event);
- the measurable acceptance criterion (quantity, quality standard, deadline, format);
- the consequence of breach (penalty, liquidated damages, termination, cure period).

This register becomes the testing universe. Obligations that are vague, conditional, or dependent on external events require explicit interpretation before testing; document the interpretation and its basis.

### Distinguish obligations from representations and permissions

Contracts contain several categories of text that are easy to confuse:

- **obligations** — mandatory duties ("the vendor shall maintain ISO 27001 certification");
- **representations and warranties** — assertions of fact made at signing ("the seller represents that all licenses are valid");
- **permissions and options** — discretionary rights ("the buyer may extend the term by notice");
- **conditions precedent** — events that must occur before obligations arise.

Test obligations for performance, representations for accuracy, and conditions precedent for occurrence. Do not treat a permission as an obligation or vice versa.

### Define the evidence of performance for each obligation

For each obligation, determine what evidence would demonstrate compliance before seeking it. Consider:

- transaction records, delivery receipts, acceptance certificates;
- third-party certifications, inspection reports, or test results;
- payment confirmations and bank records;
- system logs or access records for technical obligations;
- notices and correspondence for communication-based obligations.

Where the contract specifies the required form of evidence (e.g., a signed acceptance certificate), absence of that specific form is itself a deviation even if performance arguably occurred.

### Test material obligations and high-consequence breach clauses first

Prioritize obligations where breach carries significant consequence: termination rights, financial penalties, IP ownership shifts, exclusivity loss, or regulatory exposure. A missed minor reporting deadline is rarely as consequential as a breached exclusivity covenant or an unmet insurance requirement. Sequence testing to surface high-stakes non-compliance early.

### Evaluate conditional and contingent obligations explicitly

Many obligations are conditional ("upon achievement of milestone X, party Y shall..."). Test both whether the trigger event occurred and whether the resulting obligation was met. Agents often test the obligation without confirming the trigger, or assume a trigger never occurred without verifying. Map the dependency chain for each conditional clause.

### Assess cure periods and notice requirements before concluding breach

Many contracts allow a cure period after notice of breach before remedies accrue. Before reporting a breach, determine:

- whether formal notice was required and given;
- whether the cure period has elapsed;
- whether the breach was cured within the period;
- whether the breach is curable at all (some breaches, such as insolvency or IP infringement, may trigger immediate remedies).

A deviation that was properly cured within the contractual cure period may not constitute a reportable breach.

### Independently verify counterparty and self-reported performance data

Where compliance evidence comes from the party whose performance is being tested, corroborate it independently. Vendor self-certification, internal performance dashboards, and management's compliance attestations are starting points, not conclusions. Cross-check against system records, third-party reports, physical inspection, or transaction-level evidence.

### Consider the interaction between multiple contracts

Entities often operate under interrelated agreements: master services agreements with statements of work, framework agreements with purchase orders, or prime contracts with subcontracts. A breach or performance issue in one may flow through to others. Identify linked contracts and test obligations consistently across the related suite.

## Common Traps

- **Testing against a summary or the base agreement only.** Missing amendments, change orders, or side letters that materially alter the operative obligations.
- **Treating performance reports as evidence.** Accepting the audited party's own dashboard or certification without independent corroboration.
- **Ignoring conditional obligations.** Testing a contingent duty without first establishing whether the triggering event occurred.
- **Confusing permissions with obligations.** Reporting non-performance of a clause that granted a discretionary right rather than imposing a duty.
- **Overlooking cure periods.** Reporting a breach where the contract allowed a cure period that was properly used.
- **Clause-by-clause tunnel vision.** Testing each obligation in isolation without recognizing that breach of one clause triggers remedies under another.
- **Stale contract versions.** Testing obligations against a version that was superseded partway through the audit period.
- **Vague-term interpretation drift.** Applying a convenient interpretation to an ambiguous obligation without documenting the basis, leading to inconsistent or indefensible conclusions.
- **Ignoring conditions precedent.** Assuming obligations are in force when a condition precedent never occurred, rendering the obligations unenforceable.
- **Missing informal modifications.** Relying only on formal amendments when the parties' actual conduct or correspondence established a de facto variation.

## Self-Check

- Have I obtained and read the complete operative contract including all amendments, exhibits, side letters, and change orders, and confirmed the version effective for each period tested?
- Does my obligations register distinguish obligations, representations, permissions, and conditions precedent, with each tagged to a responsible party, trigger, and acceptance criterion?
- For each tested obligation, did I define the required evidence of performance in advance and obtain evidence of the specified form where the contract mandates one?
- Have I prioritized and tested high-consequence breach clauses (termination, penalties, exclusivity, IP) rather than only easy-to-verify administrative clauses?
- For conditional obligations, did I verify both the occurrence of the trigger event and performance of the resulting duty?
- Before concluding breach, did I assess notice requirements, cure periods, and whether the breach is curable under the contract?
- Did I independently corroborate self-reported performance data rather than relying on the audited party's own certifications or dashboards?
- Have I considered linked or interrelated contracts and tested obligations consistently across the related agreement suite?
