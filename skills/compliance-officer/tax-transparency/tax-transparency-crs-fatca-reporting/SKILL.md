---
name: tax_transparency_crs_fatca_reporting.md
description: Use when the agent is managing CRS or FATCA reporting obligations, classifying account holders for automatic exchange of information, handling documentation and self-certification, assessing reportable accounts, or advising on cross-border tax transparency and information exchange compliance.
---

# Tax Transparency CRS FATCA Reporting

Cross-border tax transparency has transformed financial institutions into tax information collectors. Under the OECD Common Reporting Standard (CRS), the US Foreign Account Tax Compliance Act (FATCA), the EU Directive on Administrative Cooperation (DAC2 and successors), and equivalent intergovernmental agreements, financial institutions must identify the tax residence of their account holders, collect documentation and self-certifications, classify accounts as reportable or not, and transmit specified information to tax authorities for automatic exchange. The central judgment problem is that CRS and FATCA reporting is not a simple data export. It requires correct classification of account holders and controlling persons, validation of self-certifications against other information, treatment of passive entities and controlling persons, and handling of recalcitrant and undocumented accounts. Errors in classification or reporting can trigger significant penalties and reputational harm, and the obligations apply to a broad range of financial institutions and accounts.

Use this skill before designing CRS or FATCA reporting procedures, classifying account holders, handling self-certifications, or assessing reportable accounts. The goal is to make the agent think about classification accuracy, self-certification validation, controlling person tracing, and the decisions that are easy to make too casually. A reporting program that treats CRS and FATCA as a data pull rather than a classification and validation process will produce errors.

This skill addresses jurisdiction-specific obligations. CRS adoption status, FATCA intergovernmental agreement model, reporting schemas, deadlines, and penalties differ across jurisdictions. Always confirm the applicable national law, intergovernmental agreements, and tax authority guidance, and consult qualified tax or legal counsel for specific reporting decisions.

## Core Rules

### Determine Whether The Institution Is A Reporting Entity

The first step is determining whether the institution is in scope as a reporting financial institution under CRS, FATCA, or both.

Reporting entity scope:

- CRS applies to financial institutions located in participating jurisdictions, including depository institutions, custodial institutions, investment entities, and specified insurance companies;
- FATCA applies to foreign financial institutions, with different treatment under different intergovernmental agreement models;
- the institution must register where required, such as FATCA registration with the IRS for a global intermediary identification number;
- the institution must determine its reporting jurisdiction and the jurisdictions to which it reports;
- some entities qualify as exempt or deemed compliant beneficial owners under specific criteria.

An institution that incorrectly concludes it is out of scope, or that misses a registration requirement, faces non-compliance from the start. The classification must be documented with the rationale and reviewed periodically.

### Classify Financial Accounts Correctly

CRS and FATCA define financial accounts and apply different due diligence and reporting rules by account type and value.

Account classification:

- depository accounts, custodial accounts, and equity or debt interests in investment entities;
- cash value insurance contracts and annuity contracts;
- individual accounts versus entity accounts, with different due diligence paths;
- pre-existing accounts versus new accounts, with different documentation and review requirements;
- lower value versus higher value accounts, with different diligence standards for pre-existing individual accounts.

The institution must classify each account correctly, because the classification determines what diligence applies, what documentation is required, and whether the account is reportable. Misclassification, such as treating an entity account as individual or a new account as pre-existing, leads to incorrect diligence and reporting.

### Identify Reportable Account Holders And Controlling Persons

The core of CRS and FATCA is identifying which account holders are reportable, based on tax residence, and for entity accounts, tracing to controlling persons.

Reportable person identification:

- for individual accounts, identify whether the account holder is a tax resident of a reportable jurisdiction;
- for entity accounts, determine whether the entity is a reportable person itself or a passive non-financial entity with controlling persons who are reportable;
- for passive entities, identify controlling persons using the AML or KYC standard, typically the beneficial ownership threshold;
- apply the account holder's tax residence, not nationality, as the primary test under CRS;
- for FATCA, identify US persons, including US citizens, residents, and certain entities.

A common error is stopping at the entity account holder without tracing to controlling persons of passive entities. A passive entity in a non-reportable jurisdiction whose controlling persons are reportable must be reported with respect to those controlling persons.

### Collect And Validate Self-Certifications

Self-certifications are the primary documentation tool for establishing tax residence and reportable status. They must be collected and validated.

Self-certification requirements:

- collect a self-certification for new entity accounts and for individual accounts where the account holder's residence is unclear;
- the self-certification must state the account holder's tax residence and taxpayer identification number;
- for entity accounts, the self-certification must address the entity's classification and, for passive entities, the controlling persons' residences;
- validate the self-certification against other information in the institution's possession, including account opening documents and AML records;
- treat inconsistencies as a trigger for follow-up and re-documentation;
- retain self-certifications for the required period.

A self-certification that is not validated against other information is unreliable. The institution must confirm that the self-certification is consistent with what it otherwise knows, and resolve inconsistencies before relying on the classification.

### Handle Recalcitrant And Undocumented Accounts

Some account holders will not provide the required documentation. The institution must have a process for recalcitrant and undocumented accounts.

Recalcitrant and undocumented account handling:

- apply cure periods and follow-up as required by the applicable regime;
- for FATCA, apply withholding or reporting as undocumented accounts where required;
- for CRS, apply the reporting that would apply based on available information, including treating undocumented accounts based on indicators;
- consider account closure or restriction where permitted and where the account holder remains recalcitrant;
- document the follow-up actions and the basis for the treatment applied.

Recalcitrant accounts cannot be ignored. The institution must apply the regime's prescribed treatment and document the process, because leaving accounts undocumented indefinitely is a compliance gap.

### Report Accurately And On Time

The institution must transmit the required information to the tax authority in the prescribed format and within the deadline.

Reporting requirements:

- report the specified information for each reportable account, including identity, account balance, and payments;
- use the prescribed XML schema and transmission method;
- meet the annual reporting deadline for the jurisdiction;
- apply data quality controls to catch errors before submission;
- handle nil returns where no accounts are reportable;
- retain reporting records for the required period.

Reporting errors, whether in data, schema, or deadline, can trigger penalties and require resubmission. The institution should validate the data before submission and have a process for corrections and resubmissions.

### Manage Changes In Account Holder Circumstances

Tax residence and reportable status can change. The institution must monitor for changes in circumstances that affect classification.

Change monitoring:

- monitor for changes in account holder information, such as address or contact details, that indicate a new tax residence;
- re-classify accounts when a change in circumstances is identified;
- collect new self-certifications where a change in circumstances invalidates the prior documentation;
- apply the change from the date it takes effect under the applicable regime;
- document the change and the re-classification.

A self-certification that is never revisited becomes stale. The institution must have triggers that detect changes in circumstances and force re-documentation.

## Common Traps

### Incorrect Reporting Entity Classification

Concluding the institution is out of scope without checking the specific criteria leads to non-compliance from the start.

### Stopping At The Entity Account Holder

Passive entities must be traced to controlling persons. Stopping at the entity misses reportable controlling persons.

### Unvalidated Self-Certifications

A self-certification not checked against other information is unreliable. Inconsistencies must be resolved.

### Ignoring Recalcitrant Accounts

Recalcitrant and undocumented accounts require prescribed treatment, not indefinite tolerance.

### Reporting Errors In Data Or Schema

Data, schema, or deadline errors trigger penalties and resubmission. Validate before submission.

### No Monitoring For Changes In Circumstances

Tax residence changes over time. Self-certifications that are never revisited become inaccurate.

### Confusing Tax Residence With Nationality

CRS tests tax residence, not nationality. Applying nationality as the test produces incorrect classification.

## Self-Check

- Has the institution determined whether it is a reporting financial institution under CRS and FATCA, with documented rationale and required registration?
- Are financial accounts classified correctly by type, value, pre-existing versus new, and individual versus entity?
- Are reportable account holders identified based on tax residence, with passive entities traced to controlling persons?
- Are self-certifications collected for new entity accounts and unclear individual accounts, and validated against other information?
- Are inconsistencies between self-certifications and other information resolved before reliance?
- Are recalcitrant and undocumented accounts handled with prescribed treatment, follow-up, and documentation?
- Is the specified information reported in the prescribed schema and within the annual deadline, with data quality controls?
- Are changes in account holder circumstances monitored and used to trigger re-classification and re-documentation?
- Are reporting records and self-certifications retained for the required period?
- Is the CRS and FATCA reporting design confirmed against the applicable national law, intergovernmental agreements, and tax authority guidance rather than a generic standard?