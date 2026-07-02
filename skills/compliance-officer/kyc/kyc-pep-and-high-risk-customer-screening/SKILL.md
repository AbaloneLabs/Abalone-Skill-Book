---
name: kyc_pep_and_high_risk_customer_screening.md
description: Use when the agent is screening customers against PEP, sanctions, and adverse media lists, defining exposure categories for politically exposed persons, calibrating enhanced due diligence for high-risk relationships, or deciding how to handle domestic versus foreign PEPs, family members, and close associates under AML obligations.
---

# KYC PEP And High-Risk Customer Screening

Politically exposed persons (PEPs), sanctioned individuals and entities, and adverse-media subjects represent the highest-risk customer segments in any AML program. Under FATF Recommendation 12, the BSA, the EU AMLD, OFAC sanctions programs, UN Security Council resolutions, and equivalent national regimes, obliged entities must screen customers and beneficial owners against these categories, apply enhanced due diligence to PEP relationships, and block or report sanctioned parties. The central judgment problem is that PEP and sanctions screening is not a simple name match. PEP exposure extends to family members and close associates, definitions vary by jurisdiction, sanctions lists change constantly, and false negatives can result in catastrophic enforcement while false positives can drown the program in noise.

Use this skill before designing PEP and sanctions screening procedures, defining PEP exposure categories, calibrating EDD for high-risk relationships, or advising on how to handle domestic versus foreign PEPs and their associates. The goal is to make the agent think about exposure breadth, screening accuracy, EDD depth, and the decisions that are easy to make too casually. Missing a sanctioned party or mishandling a PEP relationship can trigger severe penalties and reputational harm.

This skill addresses jurisdiction-specific obligations. PEP definitions, sanctions lists, family-member and close-associate scope, and EDD requirements differ across FATF member states and national regimes. Always confirm the applicable national law, regulator guidance, and current sanctions listings, and consult qualified AML, sanctions, or legal counsel for specific screening and EDD decisions.

## Core Rules

### Screen At Onboarding And On An Ongoing Basis

PEP, sanctions, and adverse media screening must occur at onboarding and continue throughout the relationship. A customer who is not a PEP or sanctioned at onboarding may become one later, and sanctions designations change frequently.

Screening points include:

- at onboarding, for the customer and all identified beneficial owners;
- on an ongoing basis against updated sanctions lists, typically daily or at the frequency the list is updated;
- on an ongoing basis for new PEP status, adverse media, or sanctions exposure;
- at periodic review for all customers;
- at trigger events such as ownership change or adverse news.

Ongoing screening is not optional. A one-time onboarding check that is never refreshed is a critical gap, because sanctions designations and PEP status change over time.

### Define PEP Exposure Categories Correctly

PEP status triggers enhanced due diligence, but the definition and scope vary by jurisdiction. FATF defines PEPs as individuals entrusted with prominent public functions, and the obligation extends to family members and close associates.

PEP categories typically include:

- foreign PEPs, individuals holding prominent public functions for or on behalf of a foreign government;
- domestic PEPs, individuals holding prominent public functions domestically, with some regimes requiring EDD only for higher-risk domestic PEPs;
- international organization PEPs, senior officials of international organizations;
- family members and close associates of any PEP category, including spouses, children, parents, siblings, and known close business or personal associates.

The institution must define which categories it screens for and what EDD each triggers. Some regimes require EDD for all PEPs, while others apply a risk-based approach to domestic PEPs. Do not assume one definition fits all jurisdictions.

### Apply Enhanced Due Diligence To All PEP Relationships

FATF Recommendation 12 requires enhanced due diligence for foreign PEPs, and a risk-based approach for domestic and international organization PEPs. Many regimes require EDD for all PEP categories regardless of perceived risk.

EDD measures for PEPs include:

- obtaining senior management approval to establish or continue the relationship;
- establishing the source of wealth and source of funds for the relationship and for specific transactions;
- conducting enhanced ongoing monitoring of the relationship;
- gathering additional identification and background information;
- conducting more frequent periodic review;
- reviewing adverse media and public-source information about the PEP.

Source of wealth and source of funds for a PEP must be corroborated, not merely self-certified. A PEP statement that funds come from salary is not sufficient. Corroborate with payslips, audited accounts, asset declarations, or independent verification.

### Handle Sanctions Matches With Blocking And Reporting

Sanctions screening is distinct from PEP screening. A sanctions match is not a risk factor to manage. It is an obligation to block, reject, or report, depending on the sanctions program.

Sanctions response depends on the program:

- OFAC and UN sanctions generally require immediate blocking of transactions and assets and reporting to the relevant authority within strict deadlines;
- EU and UK sanctions require asset freezes and reporting to the national competent authority;
- sectoral sanctions may restrict certain transactions without requiring a full asset freeze;
- secondary sanctions can expose the institution to risk even when no primary violation occurs.

The institution must have a defined process for potential matches, including false-positive review, escalation to a sanctions officer or legal counsel, blocking action, and regulatory reporting. A sanctions match must never be cleared casually by a frontline analyst.

### Manage False Positives Without Losing Signal

PEP, sanctions, and adverse media screening generate high false-positive rates, particularly for common names. Reducing false positives is legitimate, but it must not suppress genuine matches.

False-positive management approaches:

- use fuzzy matching with calibrated similarity thresholds to reduce near-miss noise;
- incorporate additional identifiers such as date of birth, nationality, and identification number to disambiguate;
- apply tiered review where low-confidence matches are dispositioned quickly and high-confidence matches escalated;
- document the rationale for each cleared match;
- periodically audit cleared matches to detect false negatives.

Never reduce matches by loosening thresholds silently. Every tuning change should be documented, tested, and approved.

### Screen Beneficial Owners And Counterparties

PEP and sanctions exposure is not limited to the named customer. The obligation extends to beneficial owners, authorized signers, and, in transaction screening, to counterparties.

Screening scope should include:

- all beneficial owners identified through ownership tracing;
- directors, officers, and authorized signers of entity customers;
- counterparties in wire transfers and trade transactions, where feasible;
- parties to correspondent banking and nested relationships.

A customer that is not a PEP or sanctioned, but whose beneficial owner is, must be treated according to the beneficial owner's status. Do not stop screening at the customer name.

### Document Screening Decisions Defensibly

Each screening event should be documented with the list version, the match review, the disposition, the rationale, and the reviewer. For PEPs, the record should show the EDD performed, the source of wealth and funds corroboration, and senior management approval. For sanctions, the record should show the blocking action or the rationale for clearing a potential match.

## Common Traps

### One-Time Onboarding Screening Only

A customer screened once and never again can become a PEP or be designated under sanctions without any compliance response. Ongoing screening is mandatory.

### Domestic PEPs Treated As Low Risk By Default

Some regimes require EDD for all PEPs, including domestic. Assuming domestic PEPs are always low risk without a risk-based assessment is a gap.

### Family Members And Close Associates Excluded

PEP exposure extends beyond the individual. Excluding family members and close associates from screening misses the channel through which PEP corruption is often laundered.

### Sanctions Match Cleared Casually

A potential sanctions match must be escalated and reviewed by trained staff, not cleared by a frontline analyst to avoid delay.

### Self-Certified Source Of Wealth For PEPs

A PEP statement about the origin of funds is not corroboration. Source of wealth and funds must be independently verified for PEP relationships.

### Threshold Loosening To Reduce Noise

Silently widening matching thresholds to reduce false positives suppresses genuine matches. Tuning must be documented and governed.

### Counterparties And Beneficial Owners Not Screened

Screening only the named customer misses exposure through ownership and transaction networks. Beneficial owners and counterparties must be in scope.

## Self-Check

- Is PEP, sanctions, and adverse media screening performed at onboarding and on an ongoing basis with list updates?
- Are PEP exposure categories defined to include foreign, domestic, international organization PEPs, family members, and close associates as required by the applicable regime?
- Is enhanced due diligence applied to PEP relationships, including senior management approval, source of wealth and funds corroboration, and enhanced monitoring?
- Is there a defined sanctions response process for blocking, reporting, and escalating potential matches within regulatory deadlines?
- Are false positives managed through documented tuning and tiered review without suppressing genuine matches?
- Are beneficial owners, authorized signers, and transaction counterparties included in screening scope?
- Is source of wealth and funds for PEPs corroborated independently rather than self-certified?
- Are screening decisions documented with list version, match review, disposition, rationale, and reviewer?
- Are sanctions list changes monitored and applied promptly, with blocking actions taken within deadlines?
- Is the screening design confirmed against the applicable national law, regulator guidance, and current sanctions listings rather than a generic standard?