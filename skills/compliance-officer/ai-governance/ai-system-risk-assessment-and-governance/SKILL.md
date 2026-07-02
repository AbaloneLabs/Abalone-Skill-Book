---
name: ai_system_risk_assessment_and_governance.md
description: Use when the agent is assessing AI system risks, establishing AI governance frameworks, evaluating model risk management, conducting pre-deployment AI risk assessments, or ensuring compliance with the EU AI Act, NIST AI RMF, and emerging AI regulations for high-risk and general-purpose AI systems.
---

# AI System Risk Assessment And Governance

AI system risk assessment and governance governs how organizations identify, evaluate, and mitigate the risks of artificial intelligence systems before and during deployment. The defining feature is that AI risks are dynamic—the model learns, drifts, and interacts with users in ways traditional software does not—and that regulatory expectations are rapidly evolving with the EU AI Act, NIST AI Risk Management Framework, and sector-specific guidance. The central difficulty is that AI risk is multi-dimensional (bias, security, privacy, transparency, safety, human autonomy), that general-purpose models create risks their developers cannot fully foresee, and that governance must span the full lifecycle from design to decommissioning.

Use this skill before advising on AI risk classification, governance structures, model validation, or regulatory compliance. The goal is to make the agent identify the risk tier, the governance controls, the validation requirements, and the regulatory obligations before concluding that an AI system is ready for deployment.

## Core Rules

### Classify AI Systems By Risk Tier

Risk classification drives the applicable obligations.

Determine:

- prohibited uses (unacceptable risk under EU AI Act: social scoring, manipulative AI, real-time biometric ID in public spaces with exceptions);
- high-risk systems (employment, education, essential services, law enforcement, migration, justice, democratic processes);
- limited-risk systems (transparency obligations: chatbots, deepfakes, emotion recognition);
- minimal-risk systems (spam filters, inventory optimization);
- general-purpose AI (GPAI) and foundation models with systemic risk thresholds;
- the interaction between sector-specific rules and horizontal AI rules;
- the evolving classification as use cases change.

EU AI Act risk tiers determine the compliance burden: prohibited systems cannot be deployed; high-risk systems require conformity assessment, risk management, data governance, transparency, human oversight, and post-market monitoring. GPAI models with systemic risk (training compute threshold) have additional obligations. Classification can shift as a system is repurposed.

### Establish A Governance Framework With Clear Accountability

AI governance requires defined roles and decision rights.

Build:

- an AI governance board or committee with cross-functional membership;
- clear ownership for AI risk (business owner, model owner, risk owner);
- defined approval workflows for new AI use cases;
- risk assessment triggers and review cycles;
- integration with existing enterprise risk management;
- escalation paths for high-risk or novel applications;
- documentation of governance decisions and rationale;
- board and senior management oversight reporting.

Governance must be cross-functional: legal, compliance, risk, data science, security, privacy, ethics, and business units. Accountability must be clear—who owns the risk, who can approve deployment, who monitors post-deployment. Novel or high-risk applications should escalate. The governance framework should integrate with, not duplicate, existing risk management.

### Conduct Pre-Deployment Risk Assessments

Every AI system should undergo risk assessment before deployment.

Assess:

- the intended purpose and use context;
- the data used for training, validation, and testing (provenance, quality, bias);
- the model architecture and known limitations;
- the potential for harm (physical, financial, reputational, rights-based);
- the affected stakeholders and vulnerable populations;
- the technical robustness and security;
- the transparency and explainability requirements;
- the human oversight mechanisms;
- the accuracy and performance metrics relevant to the use case;
- the fallback and override mechanisms.

Pre-deployment assessment must cover the full risk surface. Data quality and bias are foundational—if the training data is biased, the model will be. The use context matters: a model for medical triage has different requirements than one for ad targeting. Assessment should identify both the likelihood and severity of potential harms.

### Implement Model Validation And Testing

Models must be validated before deployment and monitored after.

Validate:

- performance metrics (accuracy, precision, recall) relevant to the use case;
- robustness testing (adversarial inputs, edge cases, distribution shift);
- bias and fairness testing across protected groups;
- security testing (model inversion, membership inference, poisoning);
- explainability and interpretability appropriate to the use case;
- the effect of drift over time;
- comparison to non-AI alternatives and baselines;
- documentation of validation methodology and results.

Validation is not a one-time event. Models drift as the world changes. Bias testing must cover all relevant protected groups. Security testing addresses AI-specific threats (adversarial examples, model extraction). Explainability requirements scale with risk: high-risk systems need more interpretable decision logic.

### Ensure Human Oversight And Control

Human oversight is required for high-risk AI.

Implement:

- human-in-the-loop, human-on-the-loop, or human-over-the-loop configurations;
- the competence and authority of human reviewers;
- override mechanisms that are real, not token;
- the avoidance of automation bias (over-reliance on AI output);
- the ability to intervene or shut down the system;
- the design of interfaces that support meaningful human review;
- the documentation of human review decisions.

Human oversight must be meaningful, not rubber-stamp. Automation bias—where humans default to accepting AI recommendations—undermines oversight. The oversight mechanism must be proportionate to risk: high-risk systems need human-in-the-loop. Override authority must be real and exercised.

### Manage Transparency And Disclosure Obligations

Transparency obligations vary by risk tier and use case.

Address:

- disclosure that users are interacting with an AI (chatbots, deepfakes);
- information to deployers of high-risk systems (instructions for use, capabilities, limitations);
- meaningful information about the logic for automated decisions affecting individuals;
- documentation for authorities (technical documentation, conformity assessment);
- model cards and system documentation;
- the tension between transparency and IP/trade secret protection;
- the right to explanation for solely automated decisions with legal/significant effects.

Transparency serves multiple audiences: end users, deployers, regulators, and affected individuals. Chatbots must disclose AI nature. Deepfakes must be labeled. High-risk system deployers need instructions for use. Affected individuals have a right to meaningful information about automated decision logic. Transparency must be balanced against trade secret protection.

### Implement Post-Deployment Monitoring And Incident Response

AI risks continue after deployment.

Monitor:

- performance drift and degradation over time;
- bias drift as input distributions change;
- emerging harms and unintended uses;
- user feedback and complaint patterns;
- the effect of model updates and retraining;
- security incidents (adversarial attacks, model theft);
- a feedback loop to the governance process;
- incident response and serious incident reporting.

Post-market monitoring is required for high-risk systems under the EU AI Act. Drift detection identifies when model performance degrades. Unintended uses may emerge that were not anticipated. Serious incidents must be reported to authorities. The monitoring feedback loop should inform governance decisions about whether to continue, modify, or retire the system.

### Manage Vendor And Third-Party AI Risk

Third-party AI models and services carry supply chain risk.

Control:

- due diligence on AI vendors and model providers;
- contractual terms for transparency, validation, and incident reporting;
- understanding the training data and known limitations of vendor models;
- the allocation of liability between provider and deployer;
- the effect of model updates by the provider;
- the risk of vendor lock-in and model discontinuation;
- the distinction between provider and deployer obligations.

Many organizations deploy AI built by others. Vendor due diligence must address the model's training data, validation, known limitations, and update practices. The EU AI Act distinguishes obligations of providers (developers) and deployers (users). Liability allocation should be clear. Provider model updates can change behavior and require re-validation.

### Address Data Governance For AI Training

AI training data governance is a core risk area.

Manage:

- the lawful basis for using personal data in training;
- data provenance and licensing (including web-scraped data);
- data quality, representativeness, and bias;
- the exclusion of toxic, harmful, or copyrighted content;
- data subject rights applicable to training data;
- the right to opt out of training (where applicable);
- documentation of training data composition;
- the handling of synthetic and augmented data.

Training data is the foundation of model behavior. Using personal data for training requires a lawful basis. Web-scraped data may include copyrighted content. Data quality and representativeness directly affect bias. Documentation of training data composition is increasingly required. Data subject rights (access, erasure) apply to training data, though practical compliance is complex.

## Common Traps

### Deploying AI Without Risk Classification

Failing to classify the system's risk tier leads to under-compliance for high-risk uses.

### Governance Committee Exists But Never Reviews Deployments

A governance structure that does not actually gate deployments provides no protection.

### Validation Only At Deployment, No Ongoing Monitoring

Models drift; one-time validation does not capture post-deployment risks.

### Human Oversight That Is Token Or Rubber-Stamp

Nominal human review that always defers to the AI is not meaningful oversight.

### Using Personal Data For Training Without A Lawful Basis

Training on personal data without establishing a lawful basis is a privacy violation.

### Vendor Model Deployed Without Understanding Limitations

Deploying a third-party model without understanding its training data and limitations transfers unknown risk.

### Transparency Treated As A Disclaimer Rather Than Meaningful Information

Vague disclaimers do not satisfy the right to meaningful information about automated decisions.

## Self-Check

- Is the AI system classified by risk tier (prohibited, high-risk, limited-risk, minimal-risk, GPAI/systemic risk) under applicable regulations including the EU AI Act?
- Is a governance framework established with a cross-functional board, clear ownership, approval workflows, risk assessment triggers, ERM integration, escalation paths, decision documentation, and senior oversight?
- Are pre-deployment risk assessments conducted covering purpose, training data, model limitations, harm potential, stakeholders, robustness, transparency, oversight, metrics, and fallbacks?
- Is model validation and testing implemented for performance, robustness, bias, security, explainability, drift, baselines, and documented methodology?
- Is human oversight meaningful with appropriate loop configuration, reviewer competence, real override authority, automation bias mitigation, intervention capability, interface design, and decision documentation?
- Are transparency obligations met including AI interaction disclosure, deployer instructions, meaningful logic information, authority documentation, model cards, IP balance, and explanation rights?
- Is post-deployment monitoring implemented for drift, bias drift, emerging harms, feedback, updates, security, governance feedback loop, and serious incident reporting?
- Is vendor and third-party AI risk managed with due diligence, contractual terms, training data understanding, liability allocation, update effects, lock-in risk, and provider/deployer distinction?
- Is data governance for AI training addressed with lawful basis, provenance, quality, bias, content exclusion, data subject rights, opt-out, documentation, and synthetic data handling?
- Are AI governance decisions documented and auditable to demonstrate compliance to regulators?