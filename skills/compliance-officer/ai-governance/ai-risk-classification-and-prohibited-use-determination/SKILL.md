---
name: ai-risk-classification-and-prohibited-use-determination.md
description: Use when the agent is deploying or procuring an AI system, classifying it under a risk-based regulatory regime, determining whether a use case is prohibited or high-risk, or deciding what conformity and documentation obligations attach to an AI application.
---

# AI Risk Classification and Prohibited-Use Determination

AI governance regimes increasingly follow a risk-based architecture: the obligations a system triggers depend on the risk class into which its intended use falls, and some uses are prohibited outright. Agents often assume that a general-purpose model carries no sector-specific obligations, or that a tool sold as productivity software is not an AI system at all. The judgment problem is correctly classifying the intended use under the applicable regime, recognizing when a use is prohibited regardless of how it is marketed, and determining the conformity, transparency, and documentation obligations that flow from the classification.

This skill applies to any organization developing, deploying, procuring, or integrating AI or machine-learning systems, including general-purpose and generative models. AI regulation is evolving rapidly and differs by jurisdiction. The classification rules, prohibited-practice lists, and conformity requirements change frequently. Verify the current text of the applicable regime and consult specialist counsel before finalizing any classification, especially for high-stakes or borderline use cases.

## Core Rules

### Classify the Intended Use, Not the Technology

Risk-based AI regimes classify the intended use of a system, not the underlying algorithm. The same model can be low-risk in one application and high-risk or prohibited in another depending on what decision it informs and who is affected. Classification must begin from a precise statement of the intended use: what the system does, what decision or output it produces, who uses the output, who is subject to it, and what the consequences are for the affected person. A vague description such as a data-analytics tool is insufficient. Force specificity about the use case before attempting classification.

### Screen Every Use Case Against the Prohibited-Practice List First

Before assessing risk class, screen the intended use against the regime's prohibited-practice list. Prohibited uses are unlawful regardless of safeguards, documentation, or consent, and they typically include practices such as subliminal manipulation, exploitation of vulnerabilities, certain forms of social scoring, certain real-time biometric identification in public spaces, and certain emotion inference in workplaces and schools. The exact list and its exceptions differ by jurisdiction and are subject to change. A use case that falls within a prohibited category cannot be made compliant through transparency or risk management; it must not be deployed. Document the screening and its reasoning.

### Apply the High-Risk Definition Conservatively and Document the Reasoning

High-risk classifications typically attach to uses that affect fundamental rights, safety, or access to essential opportunities, such as employment selection, credit scoring, education admissions, essential public-service access, law enforcement, migration and border control, and the administration of justice. The definition often includes both an enumerated list of use cases and a general test based on the significance of the impact on affected persons. Where a use case is borderline, classify conservatively as high-risk and assume the obligations apply, because under-classification leads to non-compliance while over-classification primarily adds cost. Document the classification rationale with the specific provisions relied upon, so the decision is auditable and defensible.

### Identify Conformity Obligations That Flow From the Classification

The risk class determines the obligations. High-risk systems typically trigger a package of obligations including a risk-management system, data-governance and quality requirements, technical documentation, record-keeping and logging, transparency to deployers and affected persons, human oversight, accuracy and robustness requirements, and post-market monitoring. General-purpose model providers may have separate transparency and copyright obligations. Limited-risk systems may trigger only transparency obligations such as disclosing that content is AI-generated or that a person is interacting with an AI. Map the full obligation set that flows from the classification before deployment, and assign owners for each obligation.

### Re-Classify When the Intended Use Changes

Classification is tied to intended use, so any material change in use, scope, affected population, or integration with a decision process requires re-classification. A system classified as low-risk for internal analytics becomes high-risk if it begins informing hiring decisions; a system classified as supporting human decision-making may become prohibited if the human oversight becomes nominal. Build change-control that triggers re-classification review whenever the use case, the user base, or the decision integration changes.

### Allocate Responsibilities Across the AI Value Chain

AI regimes increasingly distribute obligations across the value chain: the provider that develops the system, the deployer that puts it into use, the importer and distributor, and the downstream entity that integrates a general-purpose model into a high-risk application. A company may be a deployer of one system and a provider of another. Identify the company's role for each system, because the obligations differ by role. Where a company substantially modifies a system or puts it into a high-risk use, it may assume provider obligations even if it did not build the original system.

## Common Traps

### Classifying the Technology Instead of the Use Case

Two deployments of the same model can fall into different risk classes or one into a prohibited category. Classification must start from the intended use and its impact on affected persons, not from the model architecture.

### Skipping the Prohibited-Use Screen

Prohibited uses cannot be cured by documentation or safeguards. Failing to screen against the prohibited-practice list before investing in compliance work wastes effort and exposes the organization to enforcement. Screen first.

### Under-Classifying to Avoid Obligations

Defining a use case narrowly to escape high-risk classification is tempting but creates a non-compliance that is exposed when the actual use is examined. Regulators and affected persons look at what the system actually does, not how the vendor describes it.

### Assuming Procurement Shifts the Obligations to the Vendor

Deployers of high-risk AI carry their own obligations, including human oversight, monitoring, and incident reporting, that cannot be contracted away to the provider. Procuring a system does not transfer the deployer's responsibilities.

### Treating Classification as a One-Time Decision

Classification is tied to intended use. When the use, scope, or decision integration changes, the classification must be revisited. Static classification files that are never updated are a common source of drift into non-compliance.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I base the classification on a precise statement of the intended use, including the decision produced, the user, the affected population, and the consequences, rather than on the technology?
- Did I screen the intended use against the regime's prohibited-practice list first, and document the reasoning that the use is not prohibited?
- Where the use case is borderline, did I classify conservatively as high-risk and document the specific provisions relied upon?
- Did I map the full set of conformity, transparency, documentation, oversight, and monitoring obligations that flow from the classification, and assign owners?
- Did I build change-control that triggers re-classification when the use case, user base, or decision integration changes?
- Did I identify the company's role, provider, deployer, importer, or distributor, for each system and map the role-specific obligations?
- Did I confirm the current text of the applicable regime, recognizing that prohibited-practice lists and classification rules change, and consult specialist counsel for borderline or high-stakes classifications?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
