---
name: screening_and_due_diligence.md
description: Use when the agent is advising on sanctions and restricted-party screening, customer and counterparty due diligence, beneficial-ownership identification, third-party and supply-chain risk assessment, transaction and wire-transfer screening, managing false positives and name-matching, risk-rating methodologies, and recordkeeping for sanctions and KYC programs.
---

# Screening and Due Diligence

Screening and due diligence are the operational core of sanctions, export-control, and AML compliance. The legal obligations (not to deal with designated parties, to know your customer, to identify beneficial owners, to detect suspicious activity) are discharged through screening systems and due-diligence processes that must actually work, not merely exist. Regulators assess whether the screening was effective, whether due diligence was risk-based and documented, and whether records evidence the decisions made.

The judgment problem is that screening produces false positives and false negatives, and both are dangerous. False negatives let prohibited transactions through; false positives, if handled mechanically by clearing everything, train staff to ignore alerts. Agents often treat screening as a vendor-tool problem and neglect the calibration, the human review quality, the ownership analysis that screening alone cannot perform, and the documentation that turns a screening event into a defensible decision.

This skill is advisory only. Screening and due-diligence expectations are jurisdiction- and sector-specific. Confirm program design with qualified sanctions/AML counsel.

## Core Rules

### Design screening to cover the right parties at the right moments

Screen all relevant parties (customers, beneficial owners, directors, counterparties, consignees, end-users, intermediaries, suppliers, payment parties) against the relevant lists (sanctions, denied-party, entity, military-end-user, politically-exposed-person, adverse-media) at onboarding and on an ongoing or transaction-triggered basis.

- Define which parties are in scope for which products and channels; a payment screen differs from a trade-finance screen.
- Screen at the moments that matter: onboarding, transaction execution, list updates, and periodic refresh.

### Use risk-based due-diligence tiers

Apply due-diligence intensity proportionate to risk. Standard due diligence for low-risk relationships, enhanced due diligence for higher-risk (PEPs, high-risk jurisdictions, complex ownership, cash-intensive), and simplified or exempt treatment only where genuinely low-risk and permitted.

- Risk-rating factors include jurisdiction, customer type, product, channel, ownership complexity, and adverse information.
- Document the risk rating and the basis; an undocumented rating is not defensible.

### Identify beneficial ownership, not just the contractual counterparty

Sanctions and AML rules require identification of the natural persons who ultimately own or control the customer (commonly the 25-percent threshold for AML beneficial ownership, and the 50-percent threshold for sanctions ownership). Screening the contractual entity alone misses the individuals behind it.

- Obtain and verify beneficial-ownership information, and re-confirm periodically and on red flags.
- Where ownership is opaque or in a high-risk jurisdiction, escalate to enhanced due diligence or decline.

### Calibrate name-matching to manage false positives and negatives

List screening relies on fuzzy matching that produces false positives (similar names) and risk of false negatives (transliteration, aliases, name order). Calibration of match thresholds and the quality of human review determine effectiveness.

- Tune match thresholds to the risk and the data quality; overly broad matching floods reviewers, overly narrow matching misses hits.
- Train reviewers to dispose of alerts based on evidence (date of birth, nationality, address, ID), not habit. A "name match" cleared without distinguishing data is a weak disposition.
- Maintain an audit trail of each alert and its disposition rationale.

### Apply transaction and wire-transfer screening

Beyond party screening, screen transactions (payment messages, trade documents) for sanctioned jurisdictions, entities, vessels, and patterns. Vessel screening matters for trade and shipping; jurisdiction screening catches payments involving embargoed territories even when parties appear clean.

- Screen payment messages for sanctioned jurisdictions, ports, and vessels, and for structured or unusual patterns suggesting evasion.
- Detect "nested" activity where a counterparty processes for an undisclosed sanctioned party.

### Conduct third-party and supply-chain due diligence

Suppliers, distributors, freight forwarders, and service providers can introduce sanctions, export-control, or forced-labor risk. Due diligence should extend to material third parties, with risk-based depth.

- Map the supply chain for high-risk jurisdictions, designated-sector exposure, and diversion risk.
- Contractual sanctions and export-control representations, audit rights, and termination rights support but do not substitute for diligence.

### Document decisions and retain records

Every onboarding, screening alert, risk rating, ownership determination, and escalation should be documented and retained for the required period. Records are the primary evidence of an effective program in an audit or investigation.

- Records should show what was checked, when, by whom, the result, and the rationale for the decision.
- Inability to evidence a decision is treated as the decision not having been made.

## Common Traps

### Screening the contractual counterparty only

Screening the entity on the contract misses beneficial owners, directors, guarantors, consignees, and payment parties who may be designated. Define the full set of parties to screen per relationship and transaction.

### Mechanical clearing of false positives

When reviewers clear alerts by rote ("name match, cleared"), genuine hits are missed. Disposition must be based on distinguishing evidence and documented. High false-positive volume is a calibration and resourcing problem, not a reason to clear without analysis.

### One-time onboarding screening

A party clear at onboarding may be designated later. Without ongoing or transaction-triggered re-screening, subsequent designations are missed. Re-screen on list updates and at transaction execution.

### Ignoring beneficial ownership thresholds

The AML beneficial-ownership threshold (commonly 25 percent) and the sanctions ownership threshold (commonly 50 percent aggregated) are different and must each be applied. Missing the aggregation across multiple owners misses blocked entities.

### Trusting ownership declarations without verification

Self-reported ownership from high-risk jurisdictions or opaque structures may be incomplete or false. Verify against registers, documents, and independent sources; escalate where verification fails.

### Transaction screening limited to party names

Sanctions evasion hides in jurisdictions, vessels, ports, and nesting. Screening party names only misses payments involving embargoed territories or designated vessels. Screen transaction data fields, not just names.

### Records that cannot reconstruct the decision

If records do not show what was screened, when, and why the alert was cleared, the program cannot be defended. Design records to reconstruct each decision.

### Third-party risk treated as contractual

Representations and warranties in contracts do not substitute for diligence and monitoring. A sanctioned supplier behind a clean contract still creates exposure. Diligence, audit, and monitoring are required.

## Self-Check

- Have I defined the full set of parties to screen (customers, beneficial owners, directors, counterparties, consignees, end-users, intermediaries, suppliers, payment parties) for each product and channel?
- Is due diligence risk-based, with documented tiers, risk-rating factors, and the basis for each rating recorded?
- Are beneficial owners identified and verified at the correct thresholds (AML and sanctions), with re-confirmation periodically and on red flags?
- Are screening match thresholds calibrated to risk and data quality, with reviewer dispositions based on distinguishing evidence and documented rationale?
- Is transaction and wire-transfer screening covering jurisdictions, vessels, ports, and nesting, not just party names?
- Has third-party and supply-chain due diligence been conducted for material parties, with risk-based depth and ongoing monitoring?
- Can records reconstruct, for each onboarding, alert, rating, and escalation, what was checked, when, by whom, the result, and the rationale, retained for the required period?
- Have I flagged that screening and due-diligence expectations are jurisdiction-specific and must be confirmed with qualified sanctions/AML counsel?
