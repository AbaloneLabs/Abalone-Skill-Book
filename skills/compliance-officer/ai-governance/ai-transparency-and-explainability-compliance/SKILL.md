---
name: ai_transparency_and_explainability_compliance.md
description: Use when the agent is designing AI transparency disclosures, implementing explainability requirements, managing deepfake and synthetic media obligations, ensuring meaningful information about automated decision logic, or complying with AI transparency mandates under the EU AI Act, GDPR, sector-specific rules, and emerging synthetic media disclosure laws.
---

# AI Transparency And Explainability Compliance

AI transparency and explainability compliance governs how organizations inform people when AI is used and explain how AI systems reach their decisions. The defining feature is that transparency serves multiple audiences—end users, deployers, regulators, affected individuals—each with different needs, and that "explainability" ranges from disclosing AI use to providing meaningful information about decision logic to enabling independent technical audit. The central difficulty is that the most powerful models (deep neural networks) are the hardest to explain, that trade secret protection conflicts with transparency demands, and that synthetic media disclosure obligations are expanding rapidly.

Use this skill before advising on AI transparency design, explainability implementation, synthetic media labeling, or regulatory disclosure obligations. The goal is to make the agent identify the applicable transparency obligations, the audience needs, the explainability method, and the trade secret boundaries before concluding that transparency is adequate.

## Core Rules

### Map Transparency Obligations By Audience And Regime

Transparency obligations differ by audience and law.

Address:

- end-user transparency (disclosure that users are interacting with AI: chatbots, virtual assistants);
- deployer transparency (instructions for use, documentation for organizations deploying high-risk AI);
- affected-individual transparency (meaningful information about logic for automated decisions);
- regulatory transparency (technical documentation, conformity assessment, audit access);
- synthetic media transparency (labeling deepfakes, AI-generated content);
- the applicable regimes (EU AI Act, GDPR Articles 13-15 and 22, CCPA/CPRA, sector-specific);
- the interaction between transparency and IP/trade secret protection.

Different audiences need different transparency. End users need to know they are talking to a chatbot. Deployers need instructions for use. Affected individuals need meaningful information about automated decision logic. Regulators need technical documentation. The EU AI Act imposes layered transparency obligations across these audiences. Transparency must be balanced against trade secret protection, but the balance cannot be used as a blanket shield.

### Provide Meaningful Information About Decision Logic

Affected individuals have a right to understand automated decisions.

Deliver:

- the main factors considered in the decision;
- the relative weight or importance of those factors;
- how those factors led to the specific decision;
- the source of the data used;
- the existence of the right to contest and request human review;
- information that is understandable to non-technical individuals;
- the distinction between global explainability (how the model works generally) and local explainability (why this specific decision was made).

Meaningful information about the logic is not the full algorithm or source code. It is enough for the individual to understand the main factors, their relative importance, and how they produced the decision. Global explainability describes the model's general behavior; local explainability explains a specific decision. Both may be needed. The information must be accessible to the affected individual, not just data scientists.

### Implement Explainability Methods Appropriate To The Context

Explainability methods vary in suitability.

Consider:

- inherently interpretable models (linear models, decision trees) where the model itself is transparent;
- post-hoc explanation methods (SHAP, LIME, attention visualization) for complex models;
- counterfactual explanations ("if your income were X, you would have been approved");
- the fidelity and stability of explanations;
- the risk of misleading explanations (an explanation that does not accurately reflect the model);
- the tradeoff between model complexity and explainability;
- the audience for the explanation (data scientist, compliance, affected individual).

Inherently interpretable models provide the strongest explainability but may sacrifice accuracy. Post-hoc methods approximate complex models but may be unstable or misleading. Counterfactual explanations are intuitive for affected individuals but may not capture the full decision logic. The method must fit the audience and the stakes. High-stakes decisions may require inherently interpretable models.

### Manage Synthetic Media And Deepfake Disclosure

Synthetic media disclosure obligations are expanding.

Address:

- the obligation to disclose AI-generated or AI-manipulated content (EU AI Act, state laws);
- labeling requirements for deepfakes and synthetic media;
- the distinction between AI-assisted content (acceptable without disclosure) and AI-generated content (requires disclosure);
- the obligations for political advertising and election content;
- the obligations for synthetic media depicting real persons;
- the obligations for synthetic media in commercial communications;
- watermarking and provenance standards (C2PA, content credentials);
- the challenge of detecting and labeling synthetic media at scale.

The EU AI Act requires disclosure that content is AI-generated for deepfakes and synthetic media. Political advertising faces heightened synthetic media restrictions. Synthetic media depicting real persons without consent may violate rights of publicity and deepfake laws. Watermarking and provenance standards (C2PA content credentials) provide technical mechanisms. Detection and labeling at scale remain challenging.

### Balance Transparency With Trade Secret Protection

Transparency and trade secret protection can conflict.

Manage:

- the scope of legitimate trade secret protection (the algorithm, training methods, model weights);
- the information that must be disclosed despite trade secret claims (meaningful logic information, documentation for regulators);
- the use of confidential disclosure (disclosure to regulators under confidentiality, not public disclosure);
- the risk of overbroad trade secret claims that shield non-compliance;
- the documentation of the trade secret analysis and disclosure decisions;
- the role of independent third-party audit as an alternative to public disclosure.

Trade secrets protect competitive advantage but cannot shield regulatory non-compliance. Regulators can receive documentation under confidentiality. Meaningful logic information for affected individuals does not require revealing the full algorithm. Overbroad trade secret claims that prevent required disclosure are non-compliant. Independent third-party audit can provide verification without public disclosure of trade secrets.

### Ensure Transparency Across The AI Supply Chain

Transparency obligations flow through the supply chain.

Manage:

- the provider's obligation to provide documentation to deployers;
- the deployer's obligation to provide information to affected individuals;
- the importer's and distributor's verification obligations;
- the transparency from foundation model providers to downstream developers;
- the documentation that must accompany GPAI models;
- the allocation of transparency obligations when multiple parties are involved;
- the contractual flow of transparency obligations.

The EU AI Act distinguishes providers (who create documentation) and deployers (who provide information to affected individuals). Foundation model providers must provide documentation to downstream developers. Importers and distributors must verify documentation. When multiple parties are involved, the allocation must be clear. Contracts must flow transparency obligations through the chain.

### Address Transparency For General-Purpose And Generative AI

GPAI and generative AI have specific transparency obligations.

Address:

- the obligation to publish a sufficiently detailed summary of training data content (EU AI Act for GPAI);
- the obligation to document and provide information on system capabilities, limitations, and risks;
- the obligation to provide downstream providers with information and technical access;
- the obligation to comply with copyright in training data and provide an opt-out mechanism;
- the systemic risk obligations for GPAI models above the compute threshold;
- the transparency about model architecture and training methodology;
- the interaction with open-source model distribution.

GPAI providers must publish a summary of training data content, document capabilities and limitations, and provide downstream providers with information and technical access. Copyright compliance includes respecting text and data mining opt-outs. Systemic risk GPAI models have additional obligations (adversarial testing, incident reporting, cybersecurity). Open-source distribution creates additional transparency considerations.

### Document Transparency Decisions And Rationale

Transparency decisions must be documented.

Maintain:

- the identification of applicable transparency obligations;
- the method chosen for each obligation and the rationale;
- the trade secret analysis and disclosure decisions;
- the explainability method and its validation;
- the synthetic media labeling decisions;
- the review and approval of transparency materials;
- the periodic review of transparency practices;
- the documentation available for regulatory inspection.

Documentation demonstrates compliance and supports defense. The rationale for explainability method choice should be recorded. Trade secret analysis and disclosure decisions should be documented. Transparency materials should be reviewed and approved. Documentation should be available for regulatory inspection.

## Common Traps

### Vague Transparency Notice ("Powered By AI") Without Meaningful Information

A notice that mentions AI without explaining the logic or consequences is insufficient.

### Complex Model With No Explainability Method

Deploying a black-box model in a context requiring explainability without any explanation method.

### Trade Secret Claim Used To Block Required Disclosure

Overbroad trade secret claims that prevent meaningful logic information or regulatory documentation.

### Synthetic Media Not Labeled

Failing to label AI-generated content violates emerging disclosure laws.

### Transparency For One Audience Mistaken For All Audiences

Providing deployer documentation but not end-user or affected-individual transparency.

### Post-Hoc Explanation That Misrepresents The Model

An explanation method that does not accurately reflect the model's actual behavior is misleading.

### GPAI Training Data Summary Too Vague

A training data summary that does not allow copyright holders to identify their content is insufficient.

## Self-Check

- Are transparency obligations mapped by audience (end users, deployers, affected individuals, regulators, synthetic media) and regime (EU AI Act, GDPR, CCPA/CPRA, sector-specific) with IP/trade secret interaction?
- Is meaningful information about decision logic provided including main factors, relative weight, how factors led to the decision, data sources, contest rights, understandable format, and global vs. local explainability?
- Are explainability methods appropriate to the context including inherently interpretable models, post-hoc methods (SHAP, LIME), counterfactuals, fidelity/stability, misleading explanation risk, complexity-explainability tradeoff, and audience fit?
- Is synthetic media and deepfake disclosure managed with AI-generated content labeling, deepfake requirements, AI-assisted vs. AI-generated distinction, political advertising rules, real-person depiction, commercial communications, watermarking/provenance, and scale challenges?
- Is transparency balanced with trade secret protection through scope analysis, required disclosure despite trade secrets, confidential regulatory disclosure, overbroad claim avoidance, documentation, and third-party audit alternatives?
- Is transparency managed across the AI supply chain with provider documentation, deployer information, importer/distributor verification, foundation model provider obligations, GPAI documentation, multi-party allocation, and contractual flow?
- Are GPAI and generative AI transparency obligations addressed including training data summary, capability/limitation documentation, downstream provider information, copyright opt-out, systemic risk obligations, architecture transparency, and open-source considerations?
- Are transparency decisions and rationale documented with obligation identification, method rationale, trade secret analysis, explainability validation, synthetic media decisions, material review/approval, periodic review, and regulatory availability?
- Is the distinction between AI-assisted and AI-generated content clearly applied for disclosure purposes?
- Are transparency materials reviewed by both technical and legal/compliance functions?