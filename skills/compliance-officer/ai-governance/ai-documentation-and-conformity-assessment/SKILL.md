---
name: ai_documentation_and_conformity_assessment.md
description: Use when the agent is preparing AI technical documentation, managing EU AI Act conformity assessments, maintaining model cards and system documentation, establishing CE marking for high-risk AI, or ensuring post-market monitoring documentation and regulatory reporting for AI systems.
---

# AI Documentation And Conformity Assessment

AI documentation and conformity assessment governs the evidence and regulatory processes that demonstrate an AI system meets applicable requirements before it reaches the market. The defining feature is that high-risk AI systems under the EU AI Act require extensive technical documentation, a conformity assessment, CE marking, and registration in an EU database before placement on the market. The central difficulty is that documentation requirements are detailed and retrospective-proof, that conformity assessment pathways differ by product coupling, and that documentation must be maintained throughout the system's lifecycle.

Use this skill before advising on technical documentation, conformity pathways, CE marking, registration, or post-market reporting. The goal is to make the agent identify the documentation scope, the conformity pathway, the registration obligations, and the ongoing maintenance requirements before concluding that an AI system can be placed on the market.

## Core Rules

### Maintain Technical Documentation For High-Risk AI

Technical documentation is the core compliance artifact.

Document:

- a general description of the AI system and its intended purpose;
- the development process and development teams;
- the system architecture and design choices;
- the data used for training, validation, and testing (description, provenance, labeling);
- data preparation, cleaning, and preprocessing;
- the human oversight measures;
- an evaluation of the system's accuracy, robustness, and cybersecurity;
- changes and versions throughout the lifecycle;
- the system's intended use and reasonably foreseeable misuse;
- the instructions for use for deployers;
- the automatic logs generation capability;
- a description of the system's interoperability with other systems;
- a post-market monitoring plan;
- the standards applied (harmonized standards where applicable).

Technical documentation must be drawn up before the system is placed on the market and kept up to date. It must demonstrate conformity with the high-risk requirements. It must be proportionate to the system's complexity. Documentation that is created after the fact to justify decisions is both non-compliant and difficult to produce accurately.

### Select The Correct Conformity Assessment Pathway

The conformity pathway depends on the system type and product coupling.

Determine:

- whether the AI system is a safety component of a regulated product (machinery, medical devices, vehicles, toys, etc.);
- if so, conformity assessment under the relevant sectoral legislation (third-party assessment often required);
- if not, internal control (Module A) assessment by the provider;
- whether the system involves biometrics, which may require third-party assessment;
- the role of harmonized standards (presumption of conformity);
- the role of common specifications as an alternative;
- substantial modification triggers for re-assessment.

AI systems that are safety components of regulated products follow the conformity assessment of the relevant product legislation, which may require third-party (notified body) involvement. Standalone high-risk AI systems generally follow internal control. The use of harmonized standards provides a presumption of conformity. Substantial modifications trigger re-assessment.

### Implement Quality Management And Risk Management Systems

High-risk AI requires both a QMS and risk management system.

Establish:

- a quality management system covering design, development, testing, deployment, and post-market;
- a risk management system that is iterative throughout the lifecycle;
- regulatory compliance strategies and procedures;
- procedures for data management, technical documentation, and logging;
- procedures for handling changes and substantial modifications;
- procedures for post-market monitoring and incident reporting;
- resource management (human, technical, financial);
- accountability and governance structure.

The QMS must cover the full lifecycle. The risk management system must be continuous and iterative, identifying known and reasonably foreseeable risks, estimating and evaluating risks, and adopting risk control measures. Residual risks must be communicated to deployers. The QMS and risk management system are subject to assessment.

### Manage CE Marking And EU Database Registration

CE marking and registration are gateways to the EU market.

Implement:

- CE marking affixed after conformity assessment;
- the name and registered trade name of the provider;
- the conditions for affixing the CE mark;
- the AI system's name, type, and identification number;
- registration in the EU database before placing on the market;
- the information required for registration;
- the obligations of authorized representatives for non-EU providers;
- the distinction between provider and distributor/importer obligations.

CE marking indicates conformity with applicable EU requirements. The EU database registration is mandatory for high-risk systems. Non-EU providers must appoint an authorized representative in the EU. Importers and distributors have their own verification obligations. Placing a non-conforming system on the market is prohibited.

### Maintain Instructions For Use For Deployers

Deployers of high-risk AI need adequate instructions.

Provide:

- the identity and contact details of the provider;
- the characteristics, capabilities, and limitations of performance;
- the intended purpose and reasonably foreseeable misuse;
- the human oversight measures;
- the input data and output descriptions;
- the operating conditions and constraints;
- the maintenance and care requirements;
- the reporting of serious incidents to the provider and authorities.

Instructions for use must enable deployers to use the system correctly and interpret its output. They must be clear, comprehensive, and proportionate. Deployers have their own obligations (fundamental rights impact assessment, human oversight, monitoring) that depend on adequate instructions from the provider.

### Implement Logging And Traceability

High-risk AI must automatically log events.

Implement:

- automatic recording of events (timestamps) throughout the system's operation;
- logging level appropriate to the system's purpose and risk;
- the ability to ensure traceability of the system's functioning;
- logging of user interactions and system decisions;
- retention of logs for an appropriate period;
- the use of logs for post-market monitoring and incident investigation;
- the protection of logs from tampering.

Logging must be automatic, not manual. The logging level must be proportionate: systems used for biometric identification or critical infrastructure need more detailed logging. Logs support post-market monitoring, incident investigation, and regulatory inspection. Logs must be protected from tampering and unauthorized access.

### Conduct Post-Market Monitoring And Incident Reporting

Obligations continue after the system reaches the market.

Implement:

- a post-market monitoring plan proportionate to the system's nature and risk;
- proactive and systematic monitoring of system performance;
- collection and analysis of data on performance and incidents;
- evaluation of continuing compliance with requirements;
- reporting of serious incidents to market surveillance authorities and relevant providers;
- reporting timelines (serious incidents: without undue delay, within defined periods);
- corrective action for identified non-conformities;
- cooperation with authorities on investigations.

Post-market monitoring is a continuing obligation. Serious incidents (breaches of obligations to protect fundamental rights, health, safety) must be reported without undue delay. The provider must take corrective action for non-conformities. Cooperation with authorities is mandatory.

### Manage Documentation Across The Lifecycle And Supply Chain

Documentation must be maintained and flowed through the supply chain.

Manage:

- documentation updates for every version and change;
- substantial modification documentation and re-assessment;
- documentation flow from providers to deployers to importers to distributors;
- the obligations of each actor in the supply chain;
- the preservation of documentation for the system's lifetime plus a defined period;
- documentation available for authorities upon request (in an EU official language);
- the interaction with IP and trade secret protection.

Documentation is a living requirement. Every change must be documented. Substantial modifications may trigger re-assessment. The supply chain has defined roles: providers, authorized representatives, importers, distributors, and deployers, each with documentation obligations. Documentation must be available to authorities upon request, typically within a defined period and in an EU official language.

## Common Traps

### Technical Documentation Created After The Fact

Retroactively created documentation is often inaccurate and fails to reflect actual development.

### Wrong Conformity Pathway For Product-Coupled AI

Using internal control when third-party assessment is required for safety components of regulated products.

### CE Marking Affixed Without Completing Conformity Assessment

Affixing CE marking before completing the assessment is a violation.

### Instructions For Use That Are Vague Or Incomplete

Instructions that do not enable deployers to interpret output or exercise oversight are non-compliant.

### No Logging Or Manual Logging Only

Manual logging or absence of automatic logging violates traceability requirements.

### Post-Market Monitoring Treated As Optional

Failing to monitor after deployment misses emerging risks and violates continuing obligations.

### Serious Incidents Not Reported Within Deadlines

Delayed reporting of serious incidents is a separate violation.

### Documentation Not Updated After Changes

Outdated documentation that does not reflect the current system is non-compliant.

## Self-Check

- Is technical documentation maintained covering system description, development process, architecture, training data, data preparation, human oversight, accuracy/robustness/cybersecurity evaluation, versions, intended use/misuse, instructions, logging, interoperability, post-market plan, and standards?
- Is the correct conformity assessment pathway selected considering product coupling, sectoral legislation, internal control vs. third-party, biometrics, harmonized standards, common specifications, and substantial modification triggers?
- Are quality management and risk management systems established covering the full lifecycle, iterative risk management, compliance strategies, data management, documentation, change handling, post-market, resources, and governance?
- Is CE marking and EU database registration completed with correct marking, provider identification, conditions, system identification, pre-market registration, required information, authorized representatives, and actor obligations?
- Are instructions for use adequate for deployers covering provider identity, characteristics, limitations, purpose, misuse, oversight, input/output, conditions, maintenance, and incident reporting?
- Is logging implemented automatically with appropriate level, traceability, user interaction recording, retention, post-market use, and tamper protection?
- Is post-market monitoring and incident reporting implemented with a monitoring plan, proactive monitoring, data collection/analysis, continuing compliance evaluation, serious incident reporting, timelines, corrective action, and authority cooperation?
- Is documentation managed across the lifecycle and supply chain with version updates, substantial modification handling, supply chain flow, actor obligations, preservation period, authority availability, and IP protection?
- Is the distinction between provider, authorized representative, importer, distributor, and deployer obligations understood and documented?
- Are harmonized standards applied where available to benefit from the presumption of conformity?