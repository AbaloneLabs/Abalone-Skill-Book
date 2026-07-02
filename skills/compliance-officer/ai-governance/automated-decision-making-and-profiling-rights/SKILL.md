---
name: automated_decision_making_and_profiling_rights.md
description: Use when the agent is evaluating solely automated decisions with legal or significant effects, managing profiling and targeted advertising compliance, implementing human review requirements, or ensuring compliance with GDPR Article 22, CCPA/CPRA automated decision-making rights, and sector-specific rules on automated decisions in credit, employment, and insurance.
---

# Automated Decision-Making And Profiling Rights

Automated decision-making and profiling rights compliance governs the rules that apply when algorithms make decisions about people without meaningful human involvement. The defining feature is that individuals have specific rights when subject to solely automated decisions with legal or similarly significant effects, that profiling for marketing and advertising is separately regulated, and that the line between "assisted" and "solely automated" decisions is often blurred in practice. The central difficulty is that organizations frequently deploy automated decision systems without recognizing they trigger these rights, that human review must be meaningful rather than token, and that the scope of "similarly significant effect" is context-dependent and evolving.

Use this skill before advising on automated decision systems, profiling practices, human-in-the-loop design, or individual rights compliance. The goal is to make the agent identify whether a system triggers automated decision-making rights, what those rights require, and whether human oversight is meaningful before concluding the system is compliant.

## Core Rules

### Determine Whether A System Triggers Automated Decision Rights

The trigger is a solely automated decision with legal or similarly significant effect.

Assess:

- whether the decision is solely automated (no human involvement) or has meaningful human involvement;
- whether the decision has legal effect (creates, changes, or extinguishes legal rights);
- whether the decision has similarly significant effect (access to employment, credit, education, essential services);
- the special category rules for decisions based on special category data;
- the applicable legal framework (GDPR Article 22, CCPA/CPRA ADMT, sector-specific);
- the distinction between profiling (automated processing to evaluate aspects of a person) and automated decision-making (decisions based solely on automated processing).

GDPR Article 22 grants individuals the right not to be subject to solely automated decisions with legal or similarly significant effects, except where necessary for contract, authorized by law, or based on explicit consent. CCPA/CPRA grants the right to opt out of automated decision-making technology and to access information about its use. The trigger is the combination of solely automated and significant effect. A decision with token human involvement (rubber-stamping) is still solely automated.

### Ensure Meaningful Human Review Where Required

Human review must be genuine, not performative.

Implement:

- human reviewers with the authority and competence to override the decision;
- reviewers who actually review the relevant information, not just final outputs;
- training to prevent automation bias (over-reliance on algorithmic recommendations);
- the ability for the reviewer to access and understand the factors driving the decision;
- documentation of human review decisions and override rates;
- the design of interfaces that support meaningful review;
- the avoidance of rubber-stamp or tick-box review.

Meaningful human review requires that the human has the authority, competence, and information to override the algorithm. If the human always agrees with the algorithm, the review is not meaningful. Automation bias—where humans default to accepting algorithmic recommendations—must be actively countered. Reviewers must understand why the algorithm made its recommendation. Override rates that are near zero suggest rubber-stamping.

### Provide Transparency And Information Rights

Individuals must be informed about automated decision-making.

Provide:

- the existence of automated decision-making, including profiling;
- meaningful information about the logic involved;
- the significance and envisaged consequences for the individual;
- the right to obtain human intervention, express a view, and contest the decision;
- the source of the data used in the decision;
- the categories of data used;
- the retention of decision data;
- information in a form that is accessible and understandable.

Transparency requires more than a vague notice. Individuals must receive meaningful information about the logic—not the full algorithm, but enough to understand the factors and how they are weighed. The significance and consequences must be explained. The rights to human intervention, to express a view, and to contest must be communicated. The information must be accessible to non-technical individuals.

### Implement Profiling And Targeted Advertising Compliance

Profiling for marketing is separately regulated.

Address:

- the right to object to direct marketing profiling (GDPR Article 21);
- the right to opt out of sale/sharing and targeted advertising (CCPA/CPRA);
- the special rules for profiling children and minors;
- the restrictions on profiling based on special category data;
- the transparency requirements for profiling and targeted advertising;
- the distinction between profiling that leads to automated decisions and profiling for marketing;
- the effect of digital services regulations (DSA, DMA) on profiling.

Profiling for direct marketing can be objected to under GDPR Article 21 (absolute right for direct marketing). CCPA/CPRA grants opt-out rights for sale/sharing that includes targeted advertising. Profiling children and minors faces heightened restrictions. Profiling based on special category data requires explicit consent. The EU Digital Services Act and Digital Markets Act add restrictions on profiling-based advertising, including a prohibition on profiling-based advertising to minors.

### Manage The Legal Bases And Exceptions For Automated Decisions

Automated decisions require a valid legal basis.

Establish:

- necessity for entering into or performing a contract;
- authorization by law (the law must establish the decision and safeguards);
- explicit consent (freely given, specific, informed, unambiguous);
- the special rules for decisions based on special category data (explicit consent only, with exceptions);
- the safeguards required: the right to obtain human intervention, express a view, contest the decision;
- the documentation of the legal basis;
- the limitations on using legitimate interests for solely automated decisions.

Under GDPR Article 22, solely automated decisions with legal or significant effects require one of three bases: contract necessity, legal authorization, or explicit consent. Legitimate interests is not a basis for solely automated decisions. Decisions based on special category data require explicit consent (with narrow exceptions). The safeguards (human intervention, expression of view, contest) apply regardless of the basis.

### Address Sector-Specific Automated Decision Rules

Different sectors have specific automated decision rules.

Comply with:

- credit and lending: ECOA adverse action notices, fair lending review, model risk management;
- employment: EEOC guidance on algorithmic hiring, NYC Local Law 144 bias audits, background check rules;
- insurance: state unfair claims practices laws, algorithmic underwriting review, proxy discrimination prohibitions;
- healthcare: clinical decision support rules, HIPAA, algorithm transparency;
- government benefits: due process requirements, algorithmic accountability;
- education: FERPA, admissions algorithm review;
- housing: Fair Housing Act disparate impact for tenant screening and lending algorithms.

Sector-specific rules layer on top of general privacy rights. Credit decisions require adverse action notices explaining the reasons. Employment algorithmic hiring requires bias audits (NYC) and EEOC compliance. Insurance underwriting faces state review for unfair discrimination. Government benefit decisions trigger due process. Each sector has its own framework for automated decision oversight.

### Handle Contestability And Redress

Individuals must be able to contest automated decisions.

Implement:

- a clear process for individuals to request human review of automated decisions;
- the ability for individuals to provide additional information or context;
- a defined timeline for review and response;
- the ability to override the automated decision based on human review;
- notification of the outcome and the reasons;
- escalation paths if the individual disagrees with the review;
- documentation of contestation and review;
- the integration of contestation data into model improvement.

Contestability is a core safeguard. The process must be accessible, not buried in fine print. The timeline must be reasonable. Human review must have the authority to override. The outcome and reasons must be communicated. Contestation data—where individuals provide context the algorithm missed—should feed back into model improvement.

### Monitor And Audit Automated Decision Systems

Automated decision systems require ongoing oversight.

Monitor:

- the accuracy and fairness of decisions over time;
- the rate of human overrides and contestations;
- the distribution of decisions across protected groups;
- the effect of model updates on decision patterns;
- complaint and grievance patterns related to automated decisions;
- the alignment between stated and actual decision processes;
- external audit or third-party validation for high-stakes systems.

Ongoing monitoring catches drift, bias, and unintended effects. Override rates near zero suggest rubber-stamping. Decision distributions that correlate with protected characteristics indicate potential bias. Complaint patterns reveal problems that metrics miss. High-stakes systems should undergo external audit. The gap between stated processes (meaningful human review) and actual processes (rubber-stamping) must be identified and corrected.

## Common Traps

### Token Human Review That Always Defers To The Algorithm

Nominal human review with near-zero override rates is not meaningful oversight.

### Automated Decision System Not Recognized As Triggering Rights

Organizations deploy automated systems without recognizing they trigger GDPR Article 22 or CCPA ADMT rights.

### Vague Transparency That Does Not Explain The Logic

Notices that mention "automated processing" without meaningful information about the logic are insufficient.

### Profiling Of Children Without Heightened Protections

Profiling children for targeted advertising without special protections violates emerging regulations.

### Legitimate Interests Used As Basis For Solely Automated Decisions

Legitimate interests is not a valid basis for solely automated decisions under GDPR Article 22.

### No Contestability Mechanism Or Buried Process

A contestation process that individuals cannot find or use does not provide meaningful redress.

### Sector-Specific Rules Overlooked

Applying only general privacy rights while missing sector-specific automated decision rules (credit, employment, insurance).

## Self-Check

- Is it determined whether the system triggers automated decision rights with assessment of solely automated vs. human-involved, legal effect, similarly significant effect, special category rules, applicable framework, and profiling vs. decision-making distinction?
- Is human review meaningful with reviewer authority and competence, actual information review, automation bias prevention, factor access, override documentation, interface design, and rubber-stamp avoidance?
- Are transparency and information rights provided covering existence of ADM, meaningful logic information, significance/consequences, rights to intervention/view/contest, data sources, data categories, retention, and accessible format?
- Is profiling and targeted advertising compliance addressed with objection rights, opt-out rights, children's rules, special category restrictions, transparency, profiling-for-decisions vs. profiling-for-marketing distinction, and DSA/DMA effects?
- Are legal bases and exceptions established for contract necessity, legal authorization, explicit consent, special category data rules, required safeguards, basis documentation, and legitimate interests limitations?
- Are sector-specific automated decision rules addressed for credit (ECOA, adverse action), employment (EEOC, NYC Local Law 144), insurance (state unfair practices), healthcare, government benefits, education (FERPA), and housing (Fair Housing Act)?
- Is contestability and redress implemented with clear human review process, additional information ability, defined timeline, override authority, outcome/reasons notification, escalation paths, documentation, and model improvement feedback?
- Are automated decision systems monitored and audited for accuracy/fairness, override rates, group distributions, model update effects, complaint patterns, stated vs. actual processes, and external validation?
- Is the distinction between profiling and automated decision-making clearly understood and applied?
- Are automated decision rights documented and demonstrable to regulators on request?