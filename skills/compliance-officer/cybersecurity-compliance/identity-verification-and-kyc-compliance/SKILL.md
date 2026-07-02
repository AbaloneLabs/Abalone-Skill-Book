---
name: identity_verification_and_kyc_compliance.md
description: Use when the agent is implementing customer identity verification, managing KYC and CDD programs, evaluating beneficial ownership identification, conducting enhanced due diligence, or ensuring compliance with AML/KYC requirements under the Bank Secrecy Act, FATF recommendations, and financial services regulations for customer onboarding and ongoing monitoring.
---

# Identity Verification And KYC Compliance

Identity verification and KYC compliance governs how organizations confirm who their customers are and assess the risk they present. The defining feature is that KYC is both a regulatory obligation (AML/CFT) and a fraud prevention measure, that the level of due diligence must match the customer's risk, and that verification is not a one-time event but an ongoing obligation. The central difficulty is that identity verification creates friction that conflicts with customer experience goals, that beneficial ownership of legal entities is layered and opaque, and that the line between legitimate business and money laundering is often visible only in patterns over time.

Use this skill before advising on customer onboarding, CDD/EDD programs, beneficial ownership identification, or ongoing monitoring. The goal is to make the agent identify the customer risk profile, the required level of due diligence, the beneficial ownership structure, and the ongoing monitoring obligations before concluding that KYC is adequate.

## Core Rules

### Implement Risk-Based Customer Due Diligence (CDD)

CDD must be proportionate to customer risk.

Implement:

- a customer risk assessment that classifies customers by risk tier;
- standard CDD for lower-risk customers (identity verification, purpose of relationship, ongoing monitoring);
- enhanced due diligence (EDD) for higher-risk customers (additional information, senior approval, enhanced monitoring);
- simplified due diligence (SDD) only where permitted for clearly low-risk customers;
- the risk factors that drive tiering (customer type, geography, product, channel, behavior);
- periodic review and re-tiering of customer risk;
- documentation of the risk assessment and CDD decisions.

Risk-based CDD focuses resources where risk is highest. The risk assessment considers customer type (individual vs. entity, PEP status), geography (high-risk jurisdictions), product (cash-intensive, cross-border), channel (face-to-face vs. non-face-to-face), and behavior (transaction patterns). EDD is required for high-risk customers including PEPs, high-risk jurisdictions, and complex ownership structures. SDD is only permitted where clearly justified by low risk.

### Verify Customer Identity To The Required Standard

Identity verification must meet regulatory standards.

Verify:

- the customer's full legal name;
- the date of birth for individuals;
- the residential or business address;
- the identification number (SSN, tax ID, government ID number);
- the verification through reliable, independent sources (government ID, database verification, documentary verification);
- the distinction between collecting information and verifying it;
- the handling of non-documentary verification for online onboarding;
- the standards for accepting foreign identification documents.

Identity verification requires both collecting identifying information and verifying it through reliable, independent sources. Documentary verification (government ID) and non-documentary verification (database, knowledge-based) have different reliability profiles. Online onboarding presents challenges for document authentication. Foreign identification documents require assessment of reliability. Biometric verification (facial match to ID) is increasingly used but raises privacy concerns.

### Identify And Verify Beneficial Ownership Of Legal Entities

Legal entity customers require beneficial ownership identification.

Identify:

- the beneficial ownership threshold (typically 25% ownership under the CDD Rule, subject to jurisdiction);
- the control person (the individual with significant managerial control);
- the ownership structure through to ultimate beneficial owners;
- the verification of beneficial owner identity;
- the collection of ownership information at onboarding and updating;
- the challenges of layered, cross-border ownership structures;
- the use of corporate registries and ownership databases;
- the certification of ownership information by the customer.

Legal entity customers can be used to obscure the identity of the beneficial owner. The CDD Rule requires identification of individuals owning 25% or more and the control person. Verification of beneficial owner identity is required. Layered and cross-border structures complicate identification. Corporate registries (including the FinCEN beneficial ownership registry where applicable) provide information. The customer must certify the accuracy of ownership information.

### Conduct Enhanced Due Diligence For High-Risk Customers

EDD adds depth for higher-risk relationships.

Implement:

- additional information collection (source of funds, source of wealth);
- senior management approval for establishing or continuing the relationship;
- enhanced ongoing monitoring of transactions;
- the identification of PEPs (politically exposed persons) and their associates and family;
- the assessment of high-risk jurisdiction exposure;
- the review of adverse media and negative news;
- the documentation of EDD findings and approval;
- the escalation of findings that may require exit or SAR filing.

EDD is required for PEPs, customers in high-risk jurisdictions, and complex or unusual relationships. Source of funds and source of wealth must be understood. Senior management approval is required. Enhanced monitoring detects unusual activity. PEP identification must cover foreign, domestic, and international organization PEPs, plus family and close associates. Adverse media screening provides risk signals. EDD findings that suggest illicit activity may trigger SAR filing or relationship exit.

### Screen Against Sanctions, PEP, And Watchlists

Screening is a continuous obligation.

Screen:

- OFAC sanctions lists (SDN, sectoral, country-based);
- UN, EU, and other international sanctions lists;
- PEP lists (foreign, domestic, international organization);
- adverse media and negative news;
- watchlists and law enforcement lists;
- at onboarding and on an ongoing basis (continuous or periodic re-screening);
- the handling of potential matches (true vs. false positives);
- the escalation and resolution of confirmed matches.

Sanctions screening is mandatory and continuous. New sanctions designations require re-screening of the existing customer base. PEP screening identifies higher-risk customers requiring EDD. Adverse media screening provides risk signals. Potential matches must be investigated to distinguish true from false positives. Confirmed sanctions matches require immediate action (blocking, freezing, reporting). Screening must cover customers, beneficial owners, and related parties.

### Implement Ongoing Monitoring And Customer Information Updating

KYC is not a one-time event.

Implement:

- transaction monitoring against expected activity;
- periodic review and refresh of customer information;
- the updating of risk ratings based on observed activity;
- the triggering of event-driven reviews (large transactions, unusual activity, adverse media alerts);
- the refresh of beneficial ownership information;
- the refresh of identification documents that expire;
- the remediation of customer files with missing or outdated information;
- the exit of customers who cannot be remediated or whose risk is unacceptable.

Ongoing monitoring compares actual activity to expected activity based on the customer profile. Periodic reviews refresh information and risk ratings, with frequency based on risk tier. Event-driven reviews respond to triggers. Beneficial ownership and identification documents must be refreshed. Customer files with gaps must be remediated. Customers who cannot be remediated or whose risk has increased to unacceptable levels must be exited.

### Manage KYC For Correspondent And Cross-Border Relationships

Correspondent banking has heightened KYC obligations.

Address:

- the enhanced due diligence for correspondent banking relationships;
- the assessment of the respondent institution's AML controls;
- the prohibition on correspondent payable-through accounts without proper controls;
- the assessment of foreign bank ownership and regulation;
- the documentation of the correspondent relationship;
- the ongoing monitoring of correspondent activity;
- the risk of nested correspondent relationships (respondent's respondents).

Correspondent banking is high-risk because the correspondent provides access to the financial system for the respondent's customers. EDD must assess the respondent's AML controls, ownership, and regulation. Payable-through accounts require additional controls. Nested relationships (where the respondent provides access to its own respondents) create opacity. The correspondent must understand the full chain of access.

### Maintain Records And Support SAR/STR Filing

KYC records support suspicious activity reporting.

Maintain:

- CDD records and supporting documentation;
- beneficial ownership identification and verification;
- transaction monitoring records and alerts;
- EDD documentation and approvals;
- screening results and resolution;
- the retention period (typically 5 years after the relationship ends);
- the availability of records for regulatory examination;
- the integration of KYC data with SAR/STR filing.

KYC records must be retained for the regulatory period (typically 5 years). Records must support SAR/STR filings—the customer profile, transaction history, and investigation findings form the basis for the report. Records must be available for regulatory examination. KYC data should integrate with the SAR/STR process to enable complete and timely reporting.

## Common Traps

### Identity Information Collected But Not Verified

Collecting a customer's stated identity without independent verification is non-compliant.

### Beneficial Ownership Taken At Face Value Without Verification

Accepting customer-certified ownership without independent verification for high-risk entities.

### CDD Conducted At Onboarding But Never Refreshed

One-time CDD that is never updated misses changes in risk and ownership.

### EDD Not Conducted For PEPs Or High-Risk Jurisdictions

Failing to identify PEPs or conduct EDD for high-risk customers is a core AML failure.

### Sanctions Screening At Onboarding Only

Failing to re-screen the existing customer base when new sanctions are designated.

### Customer Risk Rating Never Updated Based On Activity

A static risk rating that does not reflect observed behavior misses emerging risk.

### Correspondent Relationships Without Understanding Nested Accounts

Correspondent relationships where the respondent's own respondents are unknown create systemic risk.

## Self-Check

- Is risk-based CDD implemented with customer risk assessment, standard CDD, EDD for high-risk, SDD only where permitted, risk factor tiering, periodic re-tiering, and documented decisions?
- Is customer identity verified to the required standard including name, date of birth, address, identification number, reliable independent sources, collection vs. verification distinction, online onboarding handling, and foreign ID acceptance?
- Is beneficial ownership of legal entities identified and verified at the 25% threshold, control person, ownership structure, identity verification, onboarding and updating, layered/cross-border challenges, registry use, and customer certification?
- Is EDD conducted for high-risk customers with source of funds/wealth, senior approval, enhanced monitoring, PEP identification (foreign/domestic/international, family/associates), high-risk jurisdiction assessment, adverse media, documentation, and SAR/exit escalation?
- Is screening conducted against OFAC, UN, EU, international sanctions, PEP lists, adverse media, watchlists, at onboarding and ongoing, with match investigation (true vs. false positive), and confirmed match escalation/resolution?
- Is ongoing monitoring and customer information updating implemented with transaction monitoring, periodic review/refresh, risk rating updates, event-driven reviews, beneficial ownership refresh, document expiry refresh, file remediation, and customer exit?
- Is KYC for correspondent and cross-border relationships managed with EDD, respondent AML control assessment, payable-through prohibitions, foreign bank assessment, documentation, ongoing monitoring, and nested relationship risk?
- Are records maintained to support SAR/STR filing with CDD records, beneficial ownership, transaction monitoring, EDD documentation, screening results, 5-year retention, examination availability, and SAR/STR integration?
- Is the customer risk assessment methodology documented and reviewed periodically?
- Are KYC processes tested through independent audit or quality assurance?