---
name: ai_vendor_and_procurement_compliance.md
description: Use when the agent is procuring AI systems, evaluating AI vendor risk, negotiating AI procurement contracts, managing third-party model risk, or ensuring due diligence and contractual safeguards for AI suppliers, foundation model providers, and AI-enabled services.
---

# AI Vendor And Procurement Compliance

AI vendor and procurement compliance governs how organizations acquire AI systems and services from third parties while managing the unique risks that AI supply chains introduce. The defining feature is that AI vendors range from foundation model providers to niche tool vendors, that the AI itself may change behavior over time through vendor updates, and that the allocation of responsibility between provider and deployer is often unclear in contracts. The central difficulty is that AI procurement cannot rely on standard software vendor due diligence, that model opacity limits the buyer's ability to assess risk, and that vendor lock-in and discontinuation create operational and compliance exposure.

Use this skill before advising on AI vendor selection, procurement contracts, due diligence, or ongoing vendor management. The goal is to make the agent identify the vendor risk profile, the required contractual safeguards, the due diligence scope, and the ongoing monitoring obligations before concluding that a third-party AI procurement is compliant.

## Core Rules

### Conduct AI-Specific Vendor Due Diligence

AI vendor due diligence must go beyond standard software assessment.

Assess:

- the vendor's AI governance and ethics framework;
- the training data provenance, quality, and bias testing;
- the model's known limitations and failure modes;
- the vendor's validation and testing methodology;
- the vendor's incident response and breach history;
- the vendor's regulatory compliance posture (EU AI Act, sector rules);
- the vendor's financial stability and business continuity;
- the vendor's subcontractors and subprocessor chain;
- the vendor's transparency about model updates and changes;
- intellectual property and indemnification for training data.

AI vendors should be assessed on their AI maturity, not just their software practices. Training data provenance is critical: if the vendor cannot explain the data, the buyer inherits unknown bias and IP risk. Model limitations must be disclosed. The vendor's update practices affect the buyer's compliance posture over time. IP indemnification for training data is a major exposure.

### Negotiate Contractual Safeguards For AI Risk

AI procurement contracts need AI-specific terms.

Include:

- warranties on training data lawfulness and IP clearance;
- warranties on model performance and accuracy claims;
- transparency obligations (model cards, documentation, change notices);
- the right to audit or independently validate the model;
- data protection terms (controller/processor, DPA, SCCs);
- security obligations and certifications (SOC 2, ISO 27001);
- incident notification timelines and cooperation duties;
- the allocation of liability for AI-caused harm;
- restrictions on use of buyer data for vendor model training;
- exit and data portability terms;
- service level agreements for availability and performance;
- the vendor's obligations if the model is deprecated or discontinued.

Standard software contracts often lack AI-specific protections. Training data warranties protect against IP and privacy claims. Performance warranties must be tied to measurable metrics. Transparency obligations ensure the buyer can understand and validate the model. The right to audit is critical but may be resisted by vendors. Exit terms prevent lock-in.

### Manage The Provider-Deployer Responsibility Allocation

AI regulations distinguish providers and deployers with different obligations.

Map:

- the provider's obligations (documentation, conformity, risk management, post-market monitoring);
- the deployer's obligations (human oversight, fundamental rights impact assessment, monitoring, logging);
- where the buyer sits in this framework;
- the contractual allocation of obligations the buyer cannot fulfill;
- the obligations that flow through to the buyer from the provider's documentation;
- the interaction with product liability for AI-caused harm.

Under the EU AI Act, the provider (developer) and deployer (user) have distinct obligations. A buyer deploying a high-risk system inherits deployer obligations (oversight, FRIA, monitoring) but depends on the provider for documentation and conformity. The contract must allocate responsibility for obligations neither party can fulfill alone. Product liability for AI-caused harm is an emerging area.

### Address Model Opacity And Explainability

Buyers must understand models they cannot fully inspect.

Manage:

- the level of transparency the vendor provides (model cards, system documentation);
- the buyer's ability to validate or independently test the model;
- the explainability of model decisions for the buyer's use case;
- the documentation needed for the buyer's own compliance (e.g., automated decision-making explanations);
- the risk of vendor claims of trade secret limiting transparency;
- independent third-party evaluation or benchmarking;
- the documentation needed for regulatory inspection.

Model opacity is a core challenge. Vendors may limit transparency citing trade secrets. Buyers need enough transparency to meet their own compliance obligations, including the ability to provide meaningful information about automated decision logic to affected individuals. Independent benchmarking or third-party evaluation can supplement vendor disclosures.

### Manage Vendor Model Updates And Changes

Vendor model updates can change behavior and compliance posture.

Control:

- notice requirements for material model changes;
- the right to reject or delay updates;
- re-validation obligations after updates;
- version pinning and the ability to use a specific model version;
- the effect of updates on performance, bias, and safety;
- the vendor's obligation to maintain backward compatibility;
- the documentation of changes for the buyer's compliance records.

AI models are frequently updated. Updates can change performance characteristics, introduce or mitigate bias, and alter behavior. The buyer needs notice of material changes, the ability to re-validate, and ideally the ability to pin to a specific version. Without these controls, the buyer's compliance posture can change without their knowledge.

### Manage Data Protection In AI Procurement

AI procurement intersects with data protection law.

Address:

- the controller/processor roles for data used with the AI system;
- data processing agreements (DPAs) where the vendor processes personal data;
- restrictions on vendor use of buyer data for model training;
- the lawful basis for any training on buyer data;
- cross-border data transfer mechanisms (SCCs, adequacy);
- data subject rights fulfillment and the vendor's cooperation;
- data security and encryption requirements;
- data localization requirements.

If the buyer provides personal data to the AI vendor, data protection obligations apply. The DPA must address the vendor's role (processor or controller), restrict use of buyer data for the vendor's own training, and ensure cross-border transfers are covered. The vendor must cooperate with data subject rights requests. Security requirements must be AI-appropriate.

### Plan For Exit And Vendor Discontinuation

AI vendor exit requires planning.

Plan:

- data portability and the ability to extract buyer data and outputs;
- model portability or replacement alternatives;
- the transition timeline and vendor cooperation obligations;
- the effect of vendor discontinuation on the buyer's operations;
- escrow or source code access for critical systems;
- the cost and feasibility of switching to an alternative;
- the documentation needed to maintain compliance during transition.

AI vendor lock-in is a significant risk. Exit planning must address data extraction, model replacement, and transition support. If the vendor discontinues the model, the buyer needs alternatives. Escrow or source code access may be needed for critical systems. The cost of switching should be assessed at procurement, not at exit.

### Manage AI Supply Chain And Subprocessor Risk

AI supply chains have multiple layers.

Map:

- the AI vendor's own vendors (cloud providers, data providers, model providers);
- the subprocessor chain for personal data processing;
- concentration risk (many buyers dependent on the same foundation model);
- the risk of upstream changes (data provider terms, cloud service changes);
- the effect of upstream incidents on the buyer;
- due diligence on the full chain, not just the direct vendor.

An AI vendor may itself depend on a foundation model provider, a cloud provider, and data providers. Changes or incidents upstream affect the buyer. Concentration risk means many organizations depend on the same foundation models, creating systemic risk. Due diligence should extend to the full chain where feasible.

## Common Traps

### Standard Software Contract Used For AI Procurement

Standard contracts lack AI-specific warranties, transparency, update, and exit terms.

### No Training Data Warranties Or IP Indemnification

Without training data warranties, the buyer inherits IP and privacy risk from the vendor's data practices.

### No Control Over Vendor Model Updates

Updates can change model behavior and compliance posture without the buyer's knowledge.

### Vendor Trade Secret Claims Block Needed Transparency

Overbroad trade secret claims prevent the buyer from meeting its own transparency obligations.

### No Exit Plan Or Data Portability

Without exit planning, vendor discontinuation or lock-in creates operational and compliance exposure.

### Deployer Obligations Assumed Without Provider Documentation

Deploying without provider documentation means the buyer cannot meet deployer obligations.

### Subprocessor Chain Not Mapped

Unknown subprocessors create hidden data protection and supply chain risk.

## Self-Check

- Is AI-specific vendor due diligence conducted covering governance framework, training data provenance/quality/bias, model limitations, validation methodology, incident history, regulatory posture, financial stability, subcontractors, update transparency, and IP indemnification?
- Are contractual safeguards negotiated including training data warranties, performance warranties, transparency obligations, audit rights, data protection terms, security obligations, incident notification, liability allocation, training use restrictions, exit/portability, SLAs, and deprecation handling?
- Is the provider-deployer responsibility allocation mapped with provider obligations, deployer obligations, buyer positioning, contractual allocation, flow-through obligations, and product liability interaction?
- Is model opacity and explainability managed with vendor transparency, validation ability, explainability for the use case, compliance documentation, trade secret balance, independent evaluation, and regulatory documentation?
- Are vendor model updates controlled with notice requirements, rejection rights, re-validation, version pinning, performance/bias/safety effects, backward compatibility, and change documentation?
- Is data protection addressed with controller/processor roles, DPAs, training use restrictions, lawful basis, cross-border transfers, data subject rights, security, and localization?
- Is exit and vendor discontinuation planned with data portability, model portability, transition timeline, discontinuation effects, escrow/source access, switching cost, and compliance maintenance?
- Is the AI supply chain and subprocessor risk mapped including vendor's vendors, subprocessor chains, concentration risk, upstream changes, upstream incidents, and full-chain due diligence?
- Are AI vendor relationships monitored throughout the contract term, not just at procurement?
- Is the buyer able to demonstrate to regulators that third-party AI risk is managed through due diligence, contracts, and ongoing monitoring?