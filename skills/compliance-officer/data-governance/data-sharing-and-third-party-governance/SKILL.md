---
name: data-sharing-and-third-party-governance.md
description: Use when the agent is sharing personal or confidential data with third parties, negotiating data processing agreements, onboarding vendors or processors, conducting third-party risk assessments, defining controller-processor responsibilities, or monitoring ongoing vendor compliance with data protection terms.
---

# Data Sharing and Third-Party Governance

Modern data operations depend on third parties: cloud providers, SaaS applications, analytics vendors, service processors, and business partners. Each data-sharing relationship extends the organisation's data beyond its direct control and creates compliance, security, and reputational risk that the organisation remains accountable for. The controller remains responsible for the processor's compliance; a breach at a vendor is the controller's breach for accountability purposes. This skill addresses the judgment involved in deciding what to share, with whom, under what terms, and how to govern the ongoing relationship so that the extension of data does not extend unmanaged risk.

## Core Rules

### Determine the role: controller, processor, or joint controller

Before sharing data, determine each party's role, because the role determines obligations:

- **Controller**: determines the purposes and means of processing; bears primary accountability;
- **Processor**: processes data only on the controller's documented instructions; has limited, specific obligations (security, confidentiality, sub-processor control, assistance, deletion at end);
- **Joint controllers**: jointly determine purposes and means; allocate responsibilities in an arrangement.

Mischaracterising a relationship (calling a controller a processor to avoid accountability, or vice versa) creates gaps in obligation and exposure. Document the role determination and align the contract to it.

### Put a written data processing agreement in place before sharing

A data processing agreement (DPA) is required (under the GDPR and comparable regimes) between a controller and a processor, and it must address:

- The subject matter, duration, nature, and purpose of processing;
- The type of personal data and categories of data subjects;
- The processor's obligation to process only on documented instructions;
- Confidentiality obligations on processor personnel;
- Required security measures;
- Restrictions on sub-processing (prior authorisation or contract) and flow-down of equivalent terms;
- Assistance with data subject rights and breach response;
- Deletion or return of data at end of service;
- Audit and inspection rights;
- Demonstrating compliance (certifications, reports).

Do not share data before the DPA is executed. A relationship that proceeds on a generic master agreement without a compliant DPA is non-compliant.

### Conduct third-party risk assessment before onboarding

Before sharing data with a new vendor or processor, assess:

- The vendor's security posture (certifications like ISO 27001, SOC 2, penetration test summaries);
- The vendor's data protection practices and track record;
- The vendor's sub-processors and their locations;
- The data transfer implications (cross-border, if applicable);
- The vendor's financial and operational stability;
- The vendor's incident history and breach response capability.

Calibrate the depth of assessment to the sensitivity of the data and the criticality of the vendor. A vendor handling restricted data warrants deep diligence; one handling public data warrants less.

### Limit data sharing to what is necessary for the purpose

Apply minimization to sharing: share only the data fields necessary for the vendor's defined purpose, not the entire dataset. Techniques include:

- Field-level filtering so the vendor receives only required fields;
- Pseudonymisation or tokenisation so the vendor processes identifiers it cannot resolve;
- Aggregation where individual-level data is not needed;
- Time-limited access so data is not retained beyond the engagement.

Sharing more data than the vendor needs expands the breach surface and the compliance scope without benefit.

### Define and control sub-processing

Processors often engage sub-processors (for example, a SaaS vendor hosting on a cloud provider). The controller must:

- Be informed of sub-processors and have a right to object;
- Ensure the processor flows down equivalent data protection terms to the sub-processor;
- Understand the sub-processing chain and its transfer implications;
- Monitor changes to the sub-processor list.

Uncontrolled sub-processing (the processor engaging sub-processors the controller never approved, in jurisdictions the controller never assessed) breaks the chain of accountability.

### Monitor ongoing vendor compliance, not just onboarding

Onboarding diligence is necessary but insufficient. Vendors change: security postures degrade, sub-processors are added, incidents occur, certifications lapse. Establish ongoing monitoring:

- Periodic re-assessment (annual or risk-based);
- Review of audit reports and certifications;
- Monitoring of sub-processor changes and breach notifications;
- Contractual right to audit or inspect;
- Defined escalation for incidents and non-compliance.

A vendor that passed onboarding diligence three years ago and was never re-assessed may no longer be compliant.

### Plan for termination and data return or deletion

At the end of the relationship, the vendor must return or delete the data. Define this in the contract and operationalise it:

- Trigger (contract end, termination for cause, data subject request);
- Method (return in a defined format, or certified deletion);
- Confirmation (certification of deletion, including backups and sub-processor copies);
- Transition support (data portability for continuity).

A vendor that retains data after termination, in backups or otherwise, continues to hold unmanaged risk. Obtain deletion certification.

### Allocate liability and ensure adequate insurance

The contract should allocate liability for data incidents fairly, including indemnification for losses caused by the vendor's non-compliance, and the vendor should carry adequate cyber liability insurance. Do not accept unlimited liability caps that leave the organisation bearing the vendor's risk, nor liability caps so low that a major incident is uninsured.

## Common Traps

### Sharing data before the DPA is executed

The relationship proceeds on commercial urgency and the DPA is "coming later." Data is shared without the required terms in place, creating a compliance gap from day one.

### Mischaracterising the controller-processor relationship

A vendor that determines purposes or means of processing is a controller, not a processor. Calling it a processor to simplify the arrangement leaves its controller obligations unaddressed.

### Sharing the full dataset when a subset would suffice

Convenience sharing — giving the vendor the whole database rather than the needed fields — expands risk without benefit. Apply minimization to sharing.

### Onboarding diligence without ongoing monitoring

A vendor compliant at onboarding may degrade over time. Without re-assessment, the organisation continues sharing data with a non-compliant vendor unknowingly.

### Ignoring the sub-processing chain

The processor's sub-processors, and their locations, are part of the risk. Uncontrolled sub-processing extends data to parties and jurisdictions the controller never assessed.

### No enforceable data return or deletion at termination

The contract is silent or vague on end-of-relationship data handling. The vendor retains data indefinitely after termination, continuing the risk with no governance.

## Self-Check

- Is each party's role (controller, processor, joint controller) determined and documented, with the contract aligned to it?
- Is a compliant data processing agreement executed before any data is shared, covering instructions, security, sub-processing, assistance, deletion, and audit?
- Was a third-party risk assessment conducted proportionate to data sensitivity and vendor criticality?
- Is data sharing minimised to the fields necessary for the purpose, with pseudonymisation or aggregation where feasible?
- Are sub-processors controlled through notification, objection rights, flow-down terms, and transfer assessment?
- Is ongoing vendor compliance monitored through periodic re-assessment, report review, sub-processor change tracking, and incident escalation?
- Is data return or deletion at termination contractually defined and operationally enforced, with deletion certification including backups?
- Is liability allocated fairly with adequate vendor cyber insurance?
