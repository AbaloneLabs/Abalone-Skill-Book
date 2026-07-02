---
name: automated_decision-making-and-profiling-rights.md
description: Use when the agent is evaluating automated decisions or profiling with legal or similarly significant effects, designing human intervention, governing AI or ML decision systems, providing adverse action notices, or applying GDPR Article 22 transparency duties.
---

# Automated Decision-Making And Profiling Rights

Automated decisions and profiling that significantly affect individuals are among the most scrutinized areas of privacy law. GDPR Article 22 gives individuals the right not to be subject to solely automated decisions with legal or similarly significant effects, plus rights to human intervention and to contest. US fair-credit and sector laws add adverse-action and notice duties. As AI and ML proliferate, the line between "analytics" and "regulated automated decision-making" blurs, and the operational failure is usually failing to recognize that a regulated decision is even happening.

Use this skill before deploying a model that affects eligibility, pricing, content, or treatment of individuals, or when responding to a complaint about an automated outcome. The goal is to make the agent identify when Article 22 or analogous duties attach, build genuine human oversight, and provide transparency that is meaningful rather than boilerplate.

## Core Rules

### Recognize When Automated Decision Duties Attach

The first and most common failure is not recognizing that a regulated automated decision exists. Article 22 applies to decisions based solely on automated processing that produce legal or similarly significant effects.

Identify triggering decisions:

- eligibility for credit, insurance, housing, or employment;
- access to education, benefits, or essential services;
- pricing or terms that materially disadvantage an individual;
- content moderation or account suspension with significant impact;
- behavioral or reputational scoring that affects opportunities.

Assess whether the decision is "solely" automated. If a human rubber-stamps a recommendation without meaningful review, the decision is still treated as solely automated. Meaningful human involvement requires the human to have authority, competence, and genuine consideration of the recommendation.

### Identify Permitted Bases For Solely Automated Decisions

Where Article 22 applies, solely automated decisions are permitted only on narrow grounds.

Permitted bases:

- necessary for entering into or performing a contract;
- authorized by law;
- based on the individual's explicit consent.

Even where a basis exists, the controller must implement suitable measures to safeguard the individual's dignity, rights, and interests, including at least the right to human intervention, to express a point of view, and to contest the decision. Special category data may not be the basis for such decisions without an Article 9 condition plus one of the Article 22(2) bases.

### Build Genuine Human Intervention

Human intervention must be real, not theatrical. A process where a human clicks approve on every output is not meaningful intervention.

Design intervention by:

- empowering reviewers to override the automated outcome;
- training reviewers on the factors, data, and limitations of the model;
- giving reviewers access to the inputs and a plain-language explanation of the output;
- requiring reviewers to document the basis for upholding or overturning;
- tracking override rates and feeding disagreements back into model governance.

If reviewers lack authority, training, or time, the intervention is illusory and the decision remains solely automated.

### Provide Meaningful Transparency

Transparency about automated decisions must be meaningful, not a generic statement that automation exists. Individuals should understand the logic involved and the consequences.

Provide:

- notice that a decision is automated or partly automated;
- the main factors considered in the decision, in plain language;
- the significance and envisaged consequences of the processing;
- the right to human intervention, to express a view, and to contest;
- how to exercise these rights.

Avoid "black box" explanations. Even where the full model cannot be disclosed, the key variables and their general effect can be explained, and trade-secret concerns do not eliminate the transparency duty.

### Govern AI And ML Decision Systems

AI and ML systems that drive or inform decisions need governance beyond a one-time privacy review. Models drift, data shifts, and biases emerge over time.

Govern decision systems by:

- documenting the model's purpose, training data, features, and intended use;
- testing for accuracy, bias, and disparate impact before deployment and periodically after;
- monitoring performance and drift in production;
- maintaining a human reviewable audit trail of inputs, outputs, and overrides;
- defining rollback or disablement triggers for degraded performance;
- reviewing changes to training data, features, or model versions as new processing.

Treat material model changes as new processing that may require fresh notice, basis, or impact assessment.

### Provide Adverse Action And Explanation Notices

US sector laws, particularly under fair credit and consumer protection regimes, require adverse-action or notice-of-decision duties when automated systems produce unfavorable outcomes.

Provide notices that:

- state the principal reasons for the adverse outcome;
- identify the information sources used, within permitted limits;
- explain the individual's right to obtain a copy of any report and to dispute accuracy;
- are delivered within the timelines the applicable law requires.

Generic notices that do not state the actual principal reasons do not satisfy these duties.

### Separate Profiling For Decisions From Profiling For Analytics

Not all profiling triggers Article 22. Profiling for general analytics, segmentation, or personalization without significant effects is regulated differently, but the line matters.

Distinguish:

- profiling that informs a significant decision: Article 22 and intervention rights apply;
- profiling for marketing personalization without significant effect: transparency and objection rights apply;
- profiling with significant effects but no decision: still requires a strong justification and safeguards.

When in doubt, treat profiling that materially affects an individual's access, price, or treatment as significant.

### Handle Special Category Data And Vulnerable Groups

Automated decisions involving special category data, or affecting children or vulnerable groups, carry heightened risk and require enhanced safeguards. Avoid using special category proxies, and assess disparate impact on protected groups as part of governance.

## Common Traps

### Failing To Recognize A Regulated Decision

Treating an eligibility, pricing, or suspension decision as ordinary analytics because a human is nominally involved is the most common and most serious failure.

### Rubber-Stamp Human Review

A reviewer who approves every recommendation without authority or analysis does not convert an automated decision into a human one.

### Boilerplate Transparency

Stating "we use automated processing" without explaining the logic, factors, or consequences fails the meaningful-information duty.

### Ignoring Model Drift And Bias

A model that was fair at deployment can become biased as data shifts. Without monitoring, the harm is invisible until a complaint.

### Over-Relying On Trade Secret To Avoid Explanation

Trade-secret concerns reduce but do not eliminate the transparency duty. Key factors can usually be explained without exposing the model.

### Treating Model Updates As Routine Maintenance

Material changes to training data, features, or model architecture can constitute new processing that needs fresh review.

### Missing Adverse Action Timelines

Late or generic adverse-action notices breach fair-credit and consumer-protection duties independently of privacy law.

### Profiling Children For Significant Decisions

Using profiling to make significant decisions about children without enhanced safeguards and a strong basis is high risk and often prohibited.

## Self-Check

- Has it been determined whether the decision produces legal or similarly significant effects, and whether it is solely or partly automated?
- If solely automated with significant effects, is there a valid basis (contract necessity, legal authorization, or explicit consent) plus suitable safeguards?
- Is human intervention genuine, with reviewers holding authority, training, access to inputs, and documented override decisions?
- Does transparency explain the logic, main factors, consequences, and rights to intervene and contest in plain language?
- Are AI and ML decision systems governed with documentation, pre-deployment and ongoing bias testing, drift monitoring, and rollback triggers?
- Are adverse-action or decision notices provided with the actual principal reasons and within statutory timelines?
- Is profiling for significant decisions distinguished from profiling for analytics, with the stricter regime applied where effects are significant?
- Are special category data, children, and vulnerable groups handled with enhanced safeguards and disparate-impact review?
- Are material model changes treated as new processing requiring fresh notice, basis, or impact assessment?
- Is an audit trail maintained of inputs, outputs, human reviews, and overrides for each significant decision?
