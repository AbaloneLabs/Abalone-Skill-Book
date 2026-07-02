---
name: ai_and_privacy_governance.md
description: Use when the agent is assessing the lawfulness of training data for AI or ML models, governing generative AI input and output data, applying automated decision and fairness duties, establishing model governance, or reconciling privacy obligations with the EU AI Act.
---

# AI And Privacy Governance

AI intensifies every existing privacy concern and introduces new ones. Training data raises lawful-basis and minimization questions at vast scale; generative models can ingest and regurgitate personal data; automated decisions trigger Article 22 and fairness duties; and the EU AI Act layers risk-based obligations on top of privacy law. The recurring failure is treating AI as a technology project that privacy reviews at the end, when the data has already been collected, the model already trained, and the risks already baked in. AI privacy governance must be built into the model lifecycle, not appended to it.

Use this skill before collecting training data, training or fine-tuning a model, deploying an AI feature that processes personal data, or establishing model governance. The goal is to make the agent identify the privacy implications of each stage of the AI lifecycle and build controls that match the risk, rather than relying on generic AI ethics statements.

## Core Rules

### Establish A Lawful Basis For Training Data

Training a model on personal data is processing that needs a lawful basis. The scale and secondary nature of training data make this especially difficult, and the basis chosen constrains what the model can later be used for.

Establish a basis by:

- identifying whether training data contains personal data, including text, images, audio, and behavioral data that can identify individuals;
- selecting a basis that fits the purpose, recognizing that consent at training-data scale is fragile and withdrawable;
- assessing legitimate interest with a rigorous balancing test, especially where data was collected for other purposes;
- documenting the basis decision and the necessity and minimization analysis.

Repurposing data collected for service delivery into training data without a compatibility assessment and basis analysis is a high-risk and common error.

### Apply Data Minimization To Training And Fine-Tuning

Minimization matters in AI because more data feels safer, but more personal data expands exposure and basis problems. Minimize at every stage.

Minimize by:

- excluding direct identifiers from training datasets where not needed;
- pseudonymizing or aggregating data before training where possible;
- limiting training data to fields necessary for the model's purpose;
- avoiding special category data unless essential and justified;
- documenting the minimization choices in a dataset or data card.

Training a general-purpose model on full customer support transcripts, chat inputs, or behavioral logs without minimization is a serious exposure.

### Govern Generative AI Input And Output Data

Generative models create two new privacy surfaces: the inputs users provide and the outputs the model produces. Both can contain personal data, and both can leak training data.

Govern inputs by:

- informing users that inputs may be processed and for what purposes;
- avoiding using confidential or personal user inputs to train shared models without a valid basis;
- providing enterprise or privacy-preserving modes that do not retain or train on inputs;
- minimizing retention of prompts and conversations.

Govern outputs by:

- assessing the risk that the model regurgitates training-data personal data;
- applying filters, guardrails, and output checks for personal data leakage;
- handling requests to correct or delete data that appears in outputs;
- documenting the measures used to prevent memorization and leakage.

A generative model that emits a real person's data from its training set creates a privacy incident, not just a quality bug.

### Apply Automated Decision And Profiling Duties

AI that drives or informs decisions about individuals triggers automated decision-making duties. The threshold is significant effect, and the duties include human intervention, transparency, and contestability.

Apply duties by:

- determining whether the model's output produces legal or similarly significant effects;
- ensuring a valid basis for solely automated significant decisions (contract necessity, legal authorization, or explicit consent);
- building genuine human intervention with authority to override;
- providing meaningful transparency about the logic and factors;
- maintaining an audit trail of inputs, outputs, and overrides.

A model that scores, ranks, or filters individuals without recognizing it as automated decision-making is uncontrolled risk.

### Address Fairness, Bias, And Disparate Impact

Privacy law and adjacent regimes increasingly require fairness and nondiscrimination in automated processing. Bias is both an ethical and a legal risk.

Address fairness by:

- testing models for disparate impact across protected characteristics before deployment;
- monitoring for biased outcomes in production;
- avoiding proxies for special category data where possible;
- documenting bias testing, findings, and mitigations;
- providing recourse for individuals affected by biased outcomes.

Bias discovered after deployment, with no testing record, is far more damaging than bias found and addressed pre-launch.

### Build Model Governance Across The Lifecycle

AI privacy governance requires lifecycle controls, not a one-time review. Models, data, and use evolve, and governance must follow.

Build governance by:

- maintaining a model inventory with purpose, data, basis, owner, and risk classification;
- requiring review before training, before deployment, and on material change;
- defining rollback or disablement triggers for degraded or harmful behavior;
- tracking data lineage from source through training to deployment;
- reviewing model versions as new processing when data or architecture changes materially.

A model inventory that exists only for production models, ignoring experimental and fine-tuned variants, leaves gaps.

### Reconcile Privacy With The EU AI Act

The EU AI Act imposes risk-based obligations that overlap with privacy law but are distinct. High-risk AI systems carry duties around data quality, documentation, human oversight, and post-market monitoring.

Reconcile by:

- classifying AI systems under the AI Act risk tiers (prohibited, high-risk, limited-risk, minimal-risk);
- applying the AI Act's high-risk obligations where triggered, including technical documentation and conformity;
- coordinating privacy impact assessments (DPIAs) with AI Act fundamental rights impact assessments;
- ensuring transparency obligations for limited-risk systems such as chatbots and deepfakes;
- recognizing that privacy compliance does not substitute for AI Act compliance, and vice versa.

### Manage Vendors And Foundation Models

Most organizations use foundation models or AI services from vendors, which introduces processor relationships and training-data uncertainty.

Manage vendors by:

- confirming whether vendor inputs are used to train shared models and on what basis;
- securing data processing agreements and, where relevant, AI-specific terms;
- assessing the vendor's training-data lawfulness and output-leakage controls;
- restricting use of personal data with third-party AI services where the basis is unclear.

Sending personal data to a generative AI service that trains on inputs, without a basis, is both a transfer and a repurposing problem.

### Document And Be Accountable

AI privacy governance must be demonstrable. Regulators and individuals will ask what data was used, on what basis, and what controls are in place.

Document by:

- maintaining dataset cards, model cards, and basis decisions;
- recording bias testing, drift monitoring, and incident response;
- keeping version history of models and training data;
- documenting human oversight, transparency measures, and recourse.

## Common Traps

### Training On Data Collected For Other Purposes

Repurposing service data into training data without compatibility and basis analysis is a high-profile enforcement theme.

### More Data Treated As Always Better

Collecting maximum personal data for training expands exposure and basis problems without proportional benefit.

### Generative Outputs Leaking Training Data

A model that regurgitates personal data from its training set creates privacy incidents that are hard to contain.

### Missing Automated Decision Recognition

Treating scoring or ranking as ordinary analytics, when it produces significant effects, leaves Article 22 duties unmet.

### No Bias Testing Until A Complaint

Discovering bias only after harm, with no testing record, multiplies legal and reputational damage.

### Lifecycle Gaps For Experimental Models

Governance that covers only production models ignores the experimental and fine-tuned variants that often cause incidents.

### Assuming Privacy Compliance Equals AI Act Compliance

Privacy compliance does not satisfy AI Act high-risk obligations, and the two regimes must be coordinated.

### Sending Personal Data To AI Vendors Without A Basis

Using third-party generative AI on personal data without confirming the basis and training-data handling is a compounding risk.

## Self-Check

- Is a lawful basis established and documented for training data, with a compatibility assessment where data was collected for other purposes?
- Is data minimization applied to training and fine-tuning, excluding identifiers and special category data unless essential and justified?
- Are generative AI inputs and outputs governed, with retention limits, training-use controls, and leakage prevention?
- Where AI drives significant decisions, are automated decision duties applied, including a valid basis, genuine human intervention, and meaningful transparency?
- Are models tested for bias and disparate impact before deployment and monitored in production, with recourse for affected individuals?
- Is there model governance across the lifecycle, including an inventory, pre-deployment review, rollback triggers, and version review on material change?
- Are AI systems classified under the EU AI Act risk tiers, with high-risk and transparency obligations applied where triggered?
- Are AI vendors and foundation models governed by DPAs, AI-specific terms, and confirmation of training-data handling?
- Is the AI privacy governance documented through dataset cards, model cards, basis decisions, bias testing, and incident records?
- Are experimental and fine-tuned models included in governance rather than only production systems?
