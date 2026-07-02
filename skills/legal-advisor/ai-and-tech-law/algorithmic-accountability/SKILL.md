---
name: algorithmic_accountability.md
description: Use when the agent is advising on algorithmic accountability and AI fairness, bias and discrimination in automated decisions, transparency and explainability obligations, algorithmic impact assessments and audits, risk-based AI regulation such as the EU AI Act, deepfake and synthetic media rules, and enforcement risk from biased or opaque automated systems.
---

# Algorithmic Accountability

Algorithmic accountability is the emerging legal core of AI, and it is easy to mishandle because the harm (bias, discrimination, opacity) is technical but the obligations (fairness, transparency, explanation, audit) are legal. Agents often treat AI bias as a technical bug to be fixed rather than a legal risk of discrimination, or treat transparency as a user-facing notice rather than a system of documentation, impact assessment, and audit. The risk-based regulatory frameworks (notably the EU AI Act) impose obligations that scale with the risk classification of the system, and high-risk systems carry documentation, human oversight, and conformity obligations that a generic privacy or product approach will miss.

Use this skill before advising on AI deployment, automated decision-making, algorithmic hiring or credit, risk classification, impact assessment, or audit. The goal is to force the agent to classify the AI system under the applicable risk framework, identify the discrimination and transparency obligations, and build the documentation and oversight that the law requires. AI regulation is rapidly evolving and jurisdiction-specific; conclusions must be confirmed against the applicable statutes, regulations, and guidance by qualified technology and regulatory counsel.

## Core Rules

### Classify The AI System Under The Applicable Risk Framework

Risk classification drives the obligations.

Determine:

- whether the system is prohibited (unacceptable risk, such as social scoring or certain biometric categorization);
- whether the system is high-risk (employment, credit, education, essential services, law enforcement, migration);
- whether the system is limited risk (transparency obligations, such as chatbots and deepfakes);
- whether the system is minimal risk (most applications);
- the interaction with sectoral rules (employment discrimination, fair lending, data protection).

The same system can be high-risk in one jurisdiction and lightly regulated in another. Classify under each applicable framework.

### Identify Discrimination And Fairness Obligations

Automated decisions can discriminate.

Address:

- the application of existing anti-discrimination law to automated decisions (employment, housing, credit, education);
- the obligation to test for disparate impact and bias before and during deployment;
- the data inputs that encode historical bias;
- the proxy variables that recreate protected characteristics;
- the fairness metrics and their limits (no single metric captures fairness);
- the documentation of testing and the remediation of bias.

An AI hiring or credit tool that produces disparate impact is discrimination, even if the model does not use protected characteristics directly. Test for bias and document it.

### Build Transparency And Explainability

Transparency is layered.

Address:

- the obligation to inform users that they are interacting with an AI system (chatbots, deepfakes, synthetic media);
- the obligation to inform individuals about automated decision-making and their right to explanation and human review (under data protection law);
- the technical documentation obligations (for high-risk systems, the system documentation and the model cards);
- the explainability of individual decisions (the ability to explain why a specific decision was made);
- the limits of explainability for complex models and the tradeoff with accuracy.

Transparency is not just a notice. It includes documentation, individual explanation, and the right to human review. Build the layered transparency.

### Conduct Algorithmic Impact Assessments And Audits

Assessments and audits are the compliance backbone.

Address:

- the impact assessment before deployment (the intended use, the data, the risks, the mitigation);
- the bias and fairness testing methodology and results;
- the ongoing monitoring and the detection of drift;
- the independent audit (where required or best practice);
- the documentation and the duty to update.

An impact assessment is not a one-time exercise. The system changes, the data drifts, and the risks evolve. Build ongoing monitoring and periodic audit.

### Structure Human Oversight And Intervention

Human oversight is required for high-risk systems.

Address:

- the designation of oversight personnel and their competence;
- the ability of the overseer to understand, monitor, and override the system;
- the design of the human-in-the-loop, human-on-the-loop, and human-over-the-loop models;
- the risk of automation bias (humans deferring to the machine);
- the logging and traceability of decisions and overrides.

Token human oversight that defers to the machine is not effective oversight. Design oversight that is competent, empowered, and documented.

### Address Deepfakes, Synthetic Media, And Generative AI

Generative AI adds transparency and IP issues.

Address:

- the obligation to disclose synthetic media and deepfakes;
- the use of copyrighted works in training and the IP exposure;
- the risk of defamation, fraud, and impersonation from synthetic content;
- the provenance and watermarking standards;
- the liability for harmful outputs.

Generative AI creates both transparency obligations (disclosure of synthetic content) and IP exposure (training on copyrighted works). Address both.

### Manage Vendor And Supply Chain Responsibility

AI systems are often built on third-party models and data.

Address:

- the allocation of responsibility between provider, deployer, and vendor;
- the flow-down of obligations in contracts;
- the due diligence on third-party models and data;
- the audit rights and the access to documentation;
- the responsibility for open-source models and components.

The deployer of a third-party high-risk system still has obligations. Allocate responsibility in contracts and conduct due diligence.

### Handle Enforcement And Liability

Enforcement is active and growing.

Prepare for:

- regulator investigations and the demand for documentation;
- discrimination claims and class actions;
- product liability for AI-caused harm;
- the reputational consequences of biased or harmful AI;
- the insurance and risk transfer options.

AI liability is developing across discrimination, product liability, and consumer protection. Build documentation, monitoring, and insurance.

## Common Traps

### Treating AI Bias As A Technical Bug

Bias in automated decisions is a legal risk of discrimination. Test and document it.

### Assuming Risk Classification Is Uniform

The same system can be high-risk in one jurisdiction and minimal in another. Classify under each.

### Treating Transparency As A User Notice

Transparency includes documentation, individual explanation, and human review. Build the layers.

### One-Time Impact Assessment

The system changes and the data drifts. Build ongoing monitoring and audit.

### Token Human Oversight

Oversight that defers to the machine is not effective. Design competent and empowered oversight.

### Overlooking Generative AI IP Exposure

Training on copyrighted works creates IP exposure. Address it.

### Ignoring Vendor And Supply Chain Responsibility

Deployers of third-party systems have obligations. Allocate and document.

### Assuming Explainability Is Solved

Complex models are hard to explain. Acknowledge the tradeoff and build the best available explanation.

## Self-Check

- Is the AI system classified (prohibited, high-risk, limited, minimal) under each applicable risk framework and sectoral rule?
- Are discrimination and fairness obligations (anti-discrimination law, disparate impact testing, proxy variables, fairness metrics, documentation) addressed?
- Is transparency layered (AI interaction notice, automated decision explanation, technical documentation, individual explanation, limits)?
- Are impact assessments and audits (pre-deployment, bias testing, ongoing monitoring, drift, independent audit, documentation) conducted?
- Is human oversight (competence, override ability, automation bias, logging) structured effectively?
- Are deepfakes, synthetic media, and generative AI (disclosure, IP training exposure, defamation, provenance, liability) addressed?
- Is vendor and supply chain responsibility (provider vs. deployer, flow-down, due diligence, open-source) allocated?
- Is enforcement and liability (investigations, discrimination claims, product liability, reputation, insurance) prepared for?
- Does the output recommend confirmation against the applicable statutes, regulations, and guidance by qualified technology and regulatory counsel?
