---
name: kyc_onboarding_and_identity_verification.md
description: Use when the agent is designing customer onboarding workflows, collecting and verifying identity documents, deciding what identification evidence is reliable, calibrating documentary versus non-documentary verification, or handling non-face-to-face and digital onboarding identity risk for KYC compliance.
---

# KYC Onboarding And Identity Verification

Customer onboarding is the moment an institution first establishes who its customer is, and the quality of that identification determines the defensibility of the entire relationship. Under the FATF Recommendations, the Bank Secrecy Act Customer Identification Program (CIP) rule, the EU AML Directives, and equivalent national regimes, financial institutions and obliged entities must collect identifying information and verify it using reliable, independent sources before or during the establishment of a business relationship. KYC onboarding is not a data-entry exercise. It is an evidence-gathering process where the institution must decide what documents are acceptable, how to verify them independently, how to handle non-face-to-face risk, and what to do when identification is incomplete, forged, or inconsistent.

Use this skill before designing onboarding procedures, defining acceptable identity evidence, calibrating documentary and non-documentary verification, or advising on digital and remote onboarding. The goal is to make the agent think about evidence reliability, verification independence, fraud and impersonation risk, and the decisions that are easy to make too casually. Weak onboarding creates accounts that are easy to open and impossible to defend in an examination, and it is the primary entry point for money laundering, terrorist financing, fraud, and sanctions evasion.

This skill addresses jurisdiction-specific obligations. Identification requirements, acceptable documents, and verification standards differ across FATF member states, the United States, the European Union, the United Kingdom, and Asia-Pacific regimes. Always confirm the applicable national law and regulator guidance, and consult qualified AML or legal counsel for specific onboarding design decisions.

## Core Rules

### Collect The Minimum Required Identifying Information

KYC onboarding begins with collecting the identifying information required by the applicable regime. The US CIP rule requires, for individuals, name, date of birth, address, and identification number. The EU AMLD and FATF require similar identifying details plus nationality and, for entities, registration details.

For individuals, collect at minimum:

- full legal name and any aliases or former names;
- date of birth;
- residential address (not a post office box alone);
- government identification number appropriate to the jurisdiction;
- nationality and dual nationality where relevant;
- contact details for verification and ongoing communication.

For legal entities, collect at minimum:

- full legal name and any trade or doing-business-as names;
- formation jurisdiction and registration or company number;
- registered address and principal place of business;
- formation documents, articles of incorporation, and certificates of good standing;
- names of directors, officers, and authorized signers;
- nature of business and expected account activity;
- ownership structure down to the beneficial ownership threshold.

Missing fields are not optional. If a required field cannot be collected, the institution must decide whether to proceed with alternative verification or decline the relationship.

### Verify Identity Using Reliable Independent Sources

Collection is not verification. The institution must confirm that the identifying information is accurate using documents, data, or information from reliable and independent sources. Self-attested data without independent verification is not compliant CDD.

Verification approaches include:

- documentary verification using government-issued photo identification, passports, or national identity cards;
- non-documentary verification using credit bureau data, government database lookups, or electronic identity services;
- biometric matching for digital onboarding, comparing a live selfie to the identification document photograph;
- database cross-checks against sanctions, PEP, and adverse media sources;
- registry lookups for entity formation and good standing.

For higher-risk relationships, combine documentary and non-documentary methods. A single source is rarely sufficient for high-risk customers or non-face-to-face onboarding.

### Calibrate Documentary Versus Non-Documentary Methods

The institution must decide when documentary verification is required and when non-documentary methods are acceptable. The US CIP rule allows both, but the institution's program must describe the circumstances in which each is used.

Factors that should increase reliance on documentary verification:

- high-risk customer segments;
- large initial deposits or high-value products;
- complex or opaque ownership structures;
- customers from high-risk jurisdictions;
- relationships where electronic verification data is unavailable or unreliable.

Factors that may support non-documentary verification:

- low-risk retail customers in jurisdictions with robust electronic identity infrastructure;
- customers with strong credit footprint and database presence;
- account types with low transaction limits and reduced laundering potential.

Document the rationale for the verification method chosen for each risk tier. The program should not default to the cheapest method regardless of risk.

### Manage Non-Face-To-Face And Digital Onboarding Risk

Non-face-to-face onboarding carries elevated impersonation and identity fraud risk. FATF, the EU AMLD, and national regulators require enhanced measures for remote onboarding that are equivalent to face-to-face verification.

Enhanced measures for digital onboarding include:

- liveness detection to confirm the person is physically present during the selfie capture;
- biometric matching between the live image and the identity document;
- cryptographic validation of the identity document, including chip reading for e-passports where available;
- device and network intelligence to detect synthetic identities and device spoofing;
- one-time passcode or similar confirmation sent to a verified channel;
- stepped-up manual review for high-risk profiles or verification anomalies.

A photograph of an identity document alone, without liveness or biometric matching, is not sufficient for high-risk remote onboarding. Synthetic identity fraud, where fabricated identities combine real and false data, is a growing risk that basic document checks will not catch.

### Handle Incomplete, Forged, Or Inconsistent Identification

Onboarding rarely proceeds perfectly. The institution must have a defined process for identification problems.

Scenarios and responses:

- missing required field: attempt alternative verification or escalate to a manual reviewer before proceeding;
- document appears forged or altered: refuse the document, require alternative identification, and consider whether to file a suspicious activity report;
- identity information is inconsistent across sources: investigate the discrepancy and resolve it before onboarding completes;
- customer refuses to provide required information: do not establish the relationship, and consider whether the refusal itself is reportable;
- identity cannot be independently verified: escalate for senior approval or decline.

Do not onboard a customer whose identity cannot be verified simply because the business wants the account. The compliance obligation overrides commercial pressure.

### Record The Verification Decision Defensibly

An examiner reconstructs the onboarding decision from the file. The record should show what information was collected, what documents were reviewed, how verification was performed, what sources were used, who approved exceptions, and when the decision was made.

Records should include:

- the identifying information collected;
- the type and number of any identification document reviewed;
- the verification method and the source used;
- the resolution of any discrepancy;
- the risk rating assigned at onboarding;
- the approver for any exception or high-risk relationship.

Retention periods are jurisdiction-specific but commonly extend at least five years after the relationship ends. Missing or incomplete verification records are indistinguishable from no verification.

## Common Traps

### Treating Document Collection As Verification

Asking for a passport and storing a copy is collection, not verification. The institution must independently confirm the identity is genuine and belongs to the applicant.

### Accepting Any Government Document As Sufficient

Not all government documents meet the reliability standard. A document without a photograph, without a machine-readable zone, or from an unreliable issuing source may not be acceptable for CIP purposes.

### Single-Source Verification For High-Risk Customers

Relying on one electronic database for a high-risk or non-face-to-face customer creates impersonation exposure. Combine sources for elevated risk.

### Skipping Liveness And Biometric Matching In Digital Onboarding

A static photo of an identity document and a stored selfie do not prove the applicant is the document holder. Liveness and biometric matching are essential for remote onboarding equivalence.

### Onboarding Despite Unresolved Inconsistencies

Pressure to open accounts can push staff to proceed when identity data conflicts. Unresolved inconsistencies must be escalated, not papered over.

### No Process For Suspected Forged Documents

Discovering a likely forged document and simply asking for another one without escalation or SAR consideration misses a red flag and may leave the institution exposed.

### Retention Gaps In Verification Records

Storing the collected data without recording the verification method and source leaves the file indefensible. The examiner needs to see how identity was confirmed, not just what was collected.

## Self-Check

- Does the onboarding procedure collect the minimum identifying information required by the applicable regime for both individuals and legal entities?
- Is identity verified using reliable, independent sources rather than relying on self-attested or unverified data?
- Is the choice between documentary and non-documentary methods calibrated to risk, with documentary verification required for higher-risk customers?
- Are enhanced measures for non-face-to-face onboarding, including liveness detection and biometric matching, sufficient to be equivalent to face-to-face verification?
- Is there a defined process for incomplete, forged, altered, or inconsistent identification that escalates rather than proceeds by default?
- Are high-risk or exception onboarding decisions approved at the appropriate level with documented rationale?
- Are synthetic identity and impersonation risks addressed through multi-source verification and device intelligence?
- Are verification records complete enough for an examiner to reconstruct the decision, including method, source, documents, and approver?
- Are retention periods aligned to the applicable jurisdictional minimum and applied consistently?
- Is the onboarding design confirmed against the specific national law and regulator guidance rather than a generic standard?