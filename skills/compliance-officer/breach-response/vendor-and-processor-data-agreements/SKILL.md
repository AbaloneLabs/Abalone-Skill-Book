---
name: vendor_and_processor_data_agreements.md
description: Use when the agent is negotiating or reviewing a data processing agreement, applying GDPR Article 28 terms, configuring standard contractual clauses, managing sub-processor flow-downs, audit rights, breach cooperation duties, or deletion-on-exit obligations.
---

# Vendor And Processor Data Agreements

Most personal data leaves the controller's environment through processors. The contract that governs that relationship is not a procurement formality; it is the instrument that makes the controller's privacy obligations flow to the vendor, allocates breach responsibility, and determines what happens to the data when the relationship ends. A weak or missing data processing agreement (DPA) means the controller retains liability for the processor's failures without the contractual levers to manage them.

Use this skill before onboarding a vendor that will process personal data, renewing a major contract, or responding to an incident involving a processor. The goal is to make the agent build DPAs as enforceable operational instruments, not as boilerplate appended to a master agreement.

## Core Rules

### Identify When A DPA Is Required And Who Is Controller Or Processor

The DPA obligation attaches whenever a vendor processes personal data on behalf of a controller. The first step is correctly characterizing the role.

Determine roles:

- controller: determines the purposes and means of processing;
- processor: processes on behalf of, and on the documented instructions of, the controller;
- joint controllers: jointly determine purposes and means, requiring an Article 26 arrangement;
- independent controller: the vendor processes for its own purposes, in which case a DPA may not apply but a transfer or transparency analysis still might.

Mischaracterizing an independent controller as a processor, or vice versa, distorts the entire relationship and the contractual terms. Document the role determination.

### Include The Article 28 Mandatory Terms

GDPR Article 28(3) sets out the terms a processor contract must contain. These are not optional and cannot be replaced by general assurances.

The DPA must bind the processor to:

- process only on the controller's documented instructions, including transfers;
- ensure persons authorized to process are under confidentiality obligations;
- take all measures required by Article 32 for security;
- respect the conditions for engaging sub-processors, including prior notice and objection rights;
- assist the controller in responding to individual rights requests;
- assist with breach notification and data protection impact assessments;
- delete or return data at the end of the services, at the controller's choice;
- make available information necessary to demonstrate compliance and allow audits.

A DPA missing these terms is non-compliant regardless of the vendor's actual practices.

### Control Sub-Processor Engagement And Flow-Downs

Processors often rely on sub-processors, and the chain can extend several layers. The controller's control over the chain depends on the contract.

Control sub-processors by:

- requiring prior notice of new sub-processors, with a right to object;
- ensuring the processor imposes equivalent Article 28 terms on each sub-processor, with liability flowing back to the processor;
- maintaining a current list or mechanism to track approved sub-processors;
- requiring notification of material changes to the sub-processor chain;
- reserving the right to require the processor to cease using a sub-processor that fails to meet requirements.

A sub-processor the controller cannot identify or influence is a gap in the control chain.

### Address Cross-Border Transfers Explicitly

If the processor or its sub-processors will access data from outside the controller's jurisdiction, the DPA must address transfers.

Address transfers by:

- identifying the countries from which data will be accessed or stored;
- incorporating the appropriate transfer mechanism, such as standard contractual clauses, binding corporate rules, or an adequacy decision;
- requiring a transfer impact assessment and supplementary measures where needed;
- prohibiting transfers to jurisdictions not covered by a valid mechanism;
- requiring notification of changes to transfer patterns.

Silent cross-border access by a vendor's remote support team is a transfer that needs a mechanism.

### Build Breach Cooperation And Notification Duties

Processors must assist controllers in meeting breach obligations. The DPA should specify how, not merely that they will.

Specify:

- a timeframe for the processor to notify the controller of a breach, often shorter than the controller's external deadline, to leave time for assessment;
- the information the processor must provide, including the nature, data categories, individuals, and likely consequences;
- cooperation in investigation, containment, and remediation;
- the processor's duty not to notify individuals or regulators on the controller's behalf unless instructed.

A processor that notifies the controller late, or notifies individuals directly without instruction, both create compliance failures.

### Secure Audit And Assessment Rights

The right to audit is often weakened in negotiation. A meaningful audit right is what makes the DPA enforceable.

Secure:

- the right to audit the processor's compliance, directly or through a third party;
- alternative evidence such as certifications, reports (for example SOC 2 or ISO 27001), and questionnaires where on-site audit is impractical;
- the right to follow up on findings and require remediation within defined timeframes;
- the flow-through of audit rights to sub-processors where feasible.

A certification is useful evidence but does not replace the contractual right to assess specific risks.

### Define Deletion Or Return On Exit

The end of the relationship is when data is most at risk of lingering. The DPA must require deletion or return and provide evidence.

Require:

- deletion or return of all personal data, at the controller's choice, within a defined period after termination;
- deletion of copies including backups, subject to a documented backup position;
- a written certification of deletion from the processor;
- transition assistance to export data in a usable format where portability is needed;
- continued confidentiality and security obligations after termination.

A vendor that retains data indefinitely after termination, or cannot certify deletion, is an ongoing liability.

### Align Liability, Insurance, And Indemnity

The DPA should align commercial levers with privacy risk. Unlimited or uncapped liability for data breaches is often justified given the controller's exposure to regulators and individuals.

Consider:

- a separate or higher liability cap for data breach and confidentiality claims;
- cyber liability insurance commensurate with the data volume and sensitivity;
- indemnity for losses arising from the processor's breach of the DPA;
- cost allocation for breach response and regulator penalties where permitted.

## Common Traps

### Skipping The DPA For Small Or Friendly Vendors

The Article 28 duty does not scale down for small vendors. A vendor processing personal data without a DPA is a compliance gap regardless of size.

### Mischaracterizing Roles

Calling an independent controller a processor, or a processor a controller, leads to the wrong contract and the wrong obligations.

### Generic Security Language

"Industry-standard security" without specifics, certifications, or measurable commitments is not a security obligation.

### Ignoring Sub-Processor Chains

Approving a processor without visibility into, or control over, its sub-processors leaves a gap the controller cannot manage.

### Silent Cross-Border Access

Remote support or offshore teams accessing data without a transfer mechanism is an unlawful transfer hiding inside a compliant-looking contract.

### Weak Audit Rights Reduced To Certifications

Accepting a certification in lieu of any audit right removes the ability to assess specific or emerging risks.

### No Deletion Certification On Exit

Terminating a vendor without requiring and receiving a deletion certification leaves data in an uncontrolled state.

### Late Breach Notification By The Processor

A processor notification timeframe that matches the controller's external deadline leaves no time for the controller's own assessment and notification.

## Self-Check

- Is a DPA in place for every vendor processing personal data on the controller's behalf, with controller or processor roles correctly determined and documented?
- Does the DPA contain all Article 28(3) mandatory terms, including documented instructions, confidentiality, security, sub-processor control, rights assistance, breach assistance, deletion or return, and audit rights?
- Are sub-processors controlled through prior notice, objection rights, flow-down terms, and a current list?
- Are cross-border transfers addressed with a valid mechanism, transfer impact assessment, and supplementary measures where needed?
- Does the DPA require prompt processor breach notification with sufficient detail and cooperation duties?
- Are audit or assessment rights meaningful, with remediation follow-up and flow-through to sub-processors?
- Does the DPA require deletion or return on exit, with backup handling and a written deletion certification?
- Are liability, insurance, and indemnity aligned with the data volume, sensitivity, and breach exposure?
- Are joint controller or independent controller scenarios handled with the correct arrangements rather than a DPA?
- Is the DPA reviewed on renewal and when the vendor's processing, locations, or sub-processors change?
