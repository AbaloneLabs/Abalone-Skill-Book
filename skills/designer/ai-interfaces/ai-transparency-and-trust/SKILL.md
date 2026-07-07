---
name: ai_transparency_and_trust.md
description: Use when the agent is designing AI transparency features, trust signals, AI disclosures, explainability surfaces, model cards, AI labels, consent flows for AI, or reviewing how users understand, trust, verify, and control AI behavior in a product.
---

# AI Transparency And Trust

Trust in AI is not earned by claiming the AI is trustworthy. It is earned by letting users see what the AI is doing, understand why, verify the output, and intervene when something is wrong. AI features that operate opaquely, that present output without disclosure, or that hide their limitations produce a particular kind of harm: users either over-trust and act on confident errors, or they sense the opacity, distrust everything, and lose the value the feature could provide. Transparency is the bridge that lets users calibrate trust to reality, and it is a design responsibility, not a legal footnote.

Use this skill before designing or reviewing AI disclosures, trust signals, explainability features, AI labels and badges, consent and opt-in flows for AI, data-use notices, model or capability documentation, feedback and reporting paths, or any surface where users form beliefs about whether and how to trust an AI feature. The goal is to prevent the agent from treating transparency as a disclaimer to be buried and ignoring the practical, ethical, and relational work of letting users understand and control AI behavior.

## Core Rules

### Disclose AI Involvement Where It Affects User Judgment

Users form different expectations about content, decisions, and actions depending on whether a human or a model produced them. Hiding AI involvement manipulates those expectations.

Disclose AI involvement when:

- content is generated, summarized, or substantially shaped by a model;
- a decision affecting the user, such as ranking, filtering, or eligibility, is model-driven;
- a recommendation or suggestion is inferred rather than editorially chosen;
- the user is interacting with an AI agent rather than a human;
- output could be mistaken for human-authored, verified, or factual.

Disclosure should be visible at the point of use, not buried in a policy. A user who acts on AI output without knowing it was AI-generated has been deprived of information they would use to judge it.

### Make Disclosure Useful, Not Performative

A generic "AI-generated" label satisfies a checkbox but often tells the user nothing useful. Effective disclosure explains what the user needs to know to act appropriately.

Useful disclosure answers:

- what part is AI-generated or AI-assisted?
- what is the AI's role: drafting, deciding, summarizing, recommending?
- what should the user do with this information: verify, review, treat as a suggestion?
- what are the known limitations relevant to this output?

A label that says "AI" without context is better than nothing but rarely enough. Pair the label with the guidance the user actually needs.

### Provide Explainability Proportional To Stakes

Not every AI output needs a deep explanation, but the higher the stakes, the more the user needs to understand the reasoning. A low-stakes suggestion needs only a label; a decision about eligibility, risk, or cost needs a real explanation.

Scale explainability to stakes:

- low stakes: simple disclosure that AI is involved;
- medium stakes: brief reasoning, such as the main factors considered;
- high stakes: detailed explanation, the inputs used, alternatives considered, and how to challenge the outcome;
- contested or harmful outcomes: a clear path to human review and appeal.

Explainability is not about exposing the model's internals. It is about giving the user enough to understand, verify, and contest the result.

### Let Users Verify, Not Just Trust

Trust is stronger when users can check the AI's work rather than taking it on faith. Design verification into the feature rather than asking users to trust blindly.

Support verification through:

- source links or citations for generated claims;
- traceability from output back to the input or evidence;
- the ability to compare AI output against a manual or baseline approach;
- confidence indicators or alternatives where the model is uncertain;
- easy ways to flag errors and have them addressed.

A feature that asks users to act on AI output but gives them no way to check it is asking for misplaced trust.

### Design Consent And Opt-Out For AI Data Use

AI features often process user input or behavior to improve models or personalize output. Users must understand and consent to this, especially when the data is sensitive.

- explain what user data the AI processes and for what purpose;
- distinguish data used to serve the user from data used to train or improve models;
- offer meaningful opt-out, not just a buried toggle;
- handle sensitive categories, such as health, financial, or personal content, with extra care;
- honor deletion and retention expectations.

Consent obtained through dark patterns or buried settings is not consent. Transparency here is both ethical and increasingly legal.

### Be Honest About Limitations And Failure Modes

Users trust AI features more, not less, when they understand what can go wrong. Hiding limitations to protect the feature's image backfires the first time the user encounters a failure they were not warned about.

Communicate limitations through:

- plain-language notes on known failure modes, such as hallucination or bias;
- guidance on when not to rely on the feature;
- honesty about staleness, coverage, or accuracy bounds;
- clear behavior when the model refuses or cannot complete a task.

Honesty about limits calibrates trust; hiding limits sets users up for harmful overtrust.

### Provide Paths To Human Review And Recourse

AI features make mistakes, and some mistakes affect users significantly. There must be a path to human review, correction, and appeal, especially for consequential decisions.

Design recourse for:

- contested decisions, such as eligibility, moderation, or risk scoring;
- harmful or wrong output that affected the user;
- requests for explanation beyond what the interface provides;
- correction of persistent errors the system keeps making;
- escalation when the AI path fails the user.

A feature that makes decisions about users but offers no human recourse is not accountable, and users will sense that.

### Avoid Trust Theater

Trust signals that look reassuring but carry no substance, such as generic badges, unverifiable claims, or security theater, can undermine trust when users notice. Trust is built by verifiable behavior, not by decoration.

Avoid:

- badges or seals that imply guarantees the feature cannot back;
- claims of accuracy, safety, or fairness without evidence or methodology;
- hiding limitations behind reassuring language;
- using transparency language to launder untrustworthy behavior.

Real transparency sometimes means telling the user the feature is limited. That honesty builds more durable trust than any badge.

## Common Traps

### Buried Or Generic AI Disclosure

A policy-page disclaimer or a generic "AI" label that tells the user nothing useful fails the purpose of disclosure. Disclose meaningfully at the point of use.

### Hiding Limitations To Protect Image

Concealing known failure modes to make the feature look more capable produces harmful overtrust and bigger fallout when failures occur.

### Trust Signals Without Substance

Badges, seals, and reassuring claims that the feature cannot back undermine trust when users notice the gap.

### No Path To Verify Output

Asking users to act on AI output without any way to check the work demands blind trust and guarantees harm when the AI is wrong.

### Consent Through Dark Patterns

Obtaining AI data-use consent through pre-checked boxes, buried toggles, or pressure is not real consent and erodes trust.

### No Recourse For Consequential Decisions

AI-driven decisions about users with no human review or appeal path leave users powerless and damage the relationship.

### Explainability That Is Either Too Shallow Or Too Technical

A one-word label tells the user nothing; a dump of model internals tells them nothing usable. Calibrate explanation to what the user needs to act.

### Using Transparency Language To Launder Untrustworthy Behavior

Reassuring transparency copy paired with opaque or manipulative behavior is worse than no transparency, because it exploits the user's trust in transparency itself.

## Self-Check

- [ ] AI involvement is disclosed at the point of use wherever it affects how users should judge content, decisions, or actions.
- [ ] Disclosure is useful, explaining the AI's role, what to do with the output, and relevant limitations, not just a generic label.
- [ ] Explainability scales with stakes, from simple labels for low-stakes output to detailed reasoning and appeal paths for high-stakes decisions.
- [ ] Users can verify AI output through sources, citations, traceability, confidence indicators, or comparison, rather than trusting blindly.
- [ ] Consent and opt-out for AI data use are meaningful, visible, and sensitive to personal, health, financial, and confidential content.
- [ ] Known limitations and failure modes are communicated honestly in plain language, including when not to rely on the feature.
- [ ] Consequential decisions offer a clear path to human review, correction, and appeal.
- [ ] Trust signals are backed by verifiable behavior and evidence, not generic badges or unsupported claims.
- [ ] Transparency language is not used to disguise opaque, manipulative, or untrustworthy behavior.
- [ ] A user could, after reading the disclosure and explanation, accurately calibrate how much to trust this specific AI output.
