---
name: data-processing-agreements.md
description: Use when the agent is drafting, reviewing, or negotiating data processing agreements (DPAs) and controller-processor terms, including roles and responsibilities, purpose and processing instructions, sub-processor authorization and flow-downs, security measures, personal data breach notification, cross-border transfer mechanisms (SCCs, UK IDTA, adequacy, TIA), audit and inspection rights, deletion and return on exit, and reconciling DPA terms with the GDPR, UK GDPR, CCPA/CPRA, and sectoral regimes.
---

# Data Processing Agreements

Data processing agreements (DPAs) are the contractual backbone of controller-processor accountability. The defining judgment problem is that a DPA is not merely a set of representations; it allocates legal responsibility for compliance failures between parties who often have unequal visibility and control. An agent drafting or reviewing a DPA must reason about who decides purpose and means (controller vs. processor), what the processor is actually allowed to do, how sub-processors and cross-border transfers are controlled, and how breach, audit, and exit obligations will actually operate when something goes wrong.

## Core Rules

### Determine the role before drafting the terms

- **Controller vs. processor** is a functional determination based on who decides the purposes and means of processing. The same entity can be a controller for some processing and a processor for other processing; a party that decides purposes and means for its own goals is a controller, not a processor, regardless of label.
- **Joint controllers** independently determine purposes and means for the same processing; their arrangement requires an article-compliant agreement allocating responsibilities and providing transparent information to data subjects.
- Mislabeling a controller as a processor (or vice versa) distorts every downstream obligation. Start from the actual decision-making, not the contract's recitals.

### Processing instructions define the perimeter

- A processor must process personal data only on documented instructions from the controller, including with regard to transfers, unless required by law (in which case the processor must notify the controller unless prohibited).
- The DPA should specify the scope of permitted processing precisely. Vague instructions ("to provide the services") invite scope creep and can convert the processor into a controller for unauthorized uses.
- The processor must not use the data for its own purposes (e.g., service improvement, analytics, AI training) without a separate lawful basis and appropriate transparency.

### Sub-processors require authorization and flow-down

- The processor may engage another processor (sub-processor) only with the controller's prior specific or general written authorization, and must impose equivalent data-protection terms by contract or other legal act.
- General authorization typically requires the processor to inform the controller of intended changes, giving the controller an opportunity to object. The mechanism for notice and objection must be operable, not theoretical.
- The processor remains fully liable to the controller for performance of the sub-processor's obligations. Liability does not stop at the contract with the sub-processor.

### Security obligations must be risk-appropriate and measurable

- Security measures should be described with enough specificity to be enforceable and auditable, while allowing for evolving technology. Annexes listing specific controls (encryption, access control, logging, testing) are common.
- Security must be appropriate to the risk, considering state of the art, costs, and the nature, scope, context, and purposes of processing. A blanket "industry-standard security" clause is weak.
- The processor must notify the controller of any personal data breach without undue delay and within any contractual or statutory deadline (e.g., 72 hours under the GDPR), with enough information for the controller to meet its own notification obligations.

### Cross-border transfers require a valid mechanism and a transfer impact assessment

- Transfers outside the adequate/adequacy jurisdictions require a transfer mechanism: Standard Contractual Clauses (SCCs), the UK International Data Transfer Agreement (IDTA) or UK Addendum to the EU SCCs, binding corporate rules, or derogations.
- A transfer impact assessment (TIA) evaluates whether the law of the destination country (especially government access) undermines the transfer mechanism, and whether supplementary measures are needed. TIAs must be documented and revisited.
- Transfers should be mapped: what data, which categories of data subjects, which destinations, which sub-processors, and under which mechanism. A DPA that does not address transfers is incomplete for any international service.

### Audit, inspection, and assistance

- The controller has a right to audit, but the scope, notice, frequency, and cost must be reconciled with security, confidentiality, and the practical reality that the processor's environment may be shared.
- Second-party audit reports (e.g., recognized certification or independent audit reports) are a common alternative to on-site audits; they should be current, scoped to the relevant services, and reviewable.
- The processor must assist the controller with data-subject rights, DPIAs, and regulator engagement, to the extent possible and proportionate.

### Deletion and return on exit

- On termination, the processor must return or delete personal data (at the controller's choice) and delete existing copies, unless retention is required by law.
- Exit obligations should be operable: format, timeframe, cost, and transition assistance. Data that cannot practically be extracted creates lock-in and compliance risk.

### Reconcile overlapping regimes

- The GDPR, UK GDPR, CCPA/CPRA, and sectoral regimes (health, financial, children's) impose different but overlapping DPA requirements. A single DPA may need to satisfy several regimes simultaneously.
- U.S. state privacy laws increasingly require specific processor terms (purpose limitation, confidentiality, sub-processor flow-downs, deletion, and the contractor's own compliance obligations) that may not be identical to the GDPR template.

## Common Traps

- **Mislabeling the role.** Calling a party a "processor" when it determines purposes and means for its own goals does not make it one; the functional analysis controls.
- **Vague processing instructions.** "As needed to provide the services" permits scope creep and can convert a processor into a controller for unauthorized uses.
- **Sub-processor authorization that cannot be exercised.** A notice-and-object mechanism that gives no real opportunity to object, or no list of current sub-processors, is not meaningful authorization.
- **Security clauses that cannot be verified.** "Industry-standard" or "reasonable" security without specifics or audit rights provides false assurance.
- **Ignoring transfers.** A DPA silent on cross-border transfers is non-compliant for any international service and misses TIA obligations.
- **Breach notice without content.** A 72-hour notice obligation without the information the controller needs to assess its own obligations is hollow.
- **Exit terms that do not work.** A deletion obligation with no format, timeframe, or transition mechanism creates lock-in and practical non-compliance.
- **One-size-fits-all template across regimes.** A GDPR-only DPA may fail CCPA/CPRA or sectoral requirements; regimes must be reconciled deliberately.

## Self-Check

- Is the controller/processor role determined functionally (who decides purposes and means), and is the DPA consistent with that determination?
- Are processing instructions specific enough to prevent scope creep and unauthorized secondary uses (including AI training and analytics)?
- Is there an operable sub-processor authorization mechanism, with a current list, meaningful notice of changes, and a real opportunity to object?
- Are security measures described specifically and risk-appropriately, with enforceable breach notification that gives the controller the information it needs?
- Are cross-border transfers mapped, supported by a valid mechanism (SCCs, UK IDTA/Addendum, BCRs, adequacy), and backed by a documented TIA?
- Are audit/inspection rights reconciled with shared-environment realities, with current third-party reports as a practical alternative?
- Are exit obligations (return/deletion, format, timeframe, transition) operable and enforceable?
- Does the DPA reconcile all applicable regimes (GDPR, UK GDPR, CCPA/CPRA, sectoral), rather than defaulting to a single template?
- Have I recommended qualified data-protection counsel admitted in the relevant jurisdiction(s) for role determinations, transfer mechanisms, and enforcement-sensitive drafting?
