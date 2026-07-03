---
name: architectural_constraints_definition.md
description: Use when the agent is defining non-functional requirements and architectural constraints, specifying performance, scalability, reliability, security, and compliance needs, or translating product and business requirements into the constraints that architecture and engineering must satisfy.
---

# Architectural Constraints Definition

Non-functional requirements, the constraints on how a system must behave rather than what it must do, are where product and architecture meet. They are also where products most often fail invisibly. A product can meet every functional requirement and still fail users because it is too slow, unavailable when needed, insecure, unable to scale, or non-compliant with obligations the team never surfaced. These failures are invisible in functional demos and devastating in production, and they trace back to constraints that were never defined, were defined vaguely, or were defined by engineering guessing at what the product needed.

Defining architectural constraints is the product manager's responsibility, in partnership with engineering, because these constraints flow from product and business needs that engineering cannot infer alone. How fast must the product respond, for which users, under what load? How available must it be, and what is the cost of downtime? What security and compliance obligations apply, and what do they require of the architecture? These questions have product answers, and the architecture must serve them. Constraints left undefined default to whatever is easiest to build, which is rarely what the product needs.

Use this skill before defining non-functional requirements, before specifying performance or reliability targets, before a launch that depends on the system meeting unstated expectations, or when a product is failing in ways that functional requirements did not cover. Ask: have the performance, scalability, reliability, security, and compliance constraints been defined, or are they assumed? Are they specific enough to design and test against? Do they reflect real product and business needs, or inherited numbers no one has justified?

## Core Rules

### Define Constraints From Product And Business Needs

Architectural constraints are not arbitrary technical targets; they are translations of product and business needs into system requirements. Performance targets come from what users will tolerate and what the experience requires. Availability targets come from the cost of downtime to the business and to users. Scalability targets come from expected growth and usage patterns. Security and compliance constraints come from the data handled, the obligations that apply, and the risk tolerance. Each constraint should trace back to a product or business driver that justifies it.

For each constraint, identify the driver. Why does this matter, and to whom? What product experience or business outcome depends on it? A constraint without a driver is either a guess or an inherited number, and both are suspect. If the driver is unclear, clarify it before committing the constraint, because architecture built to an unjustified constraint either over-invests, wasting effort on tightness no one needs, or under-invests, failing a need that was never articulated. Grounding constraints in drivers ensures the architecture serves real needs at appropriate cost.

### Make Constraints Specific And Testable

A constraint like "the product should be fast" is not a constraint; it is an aspiration that supports no design or verification. A constraint like "the main task flow must respond within two seconds for the ninety-fifth percentile of users under normal load" is a constraint: it specifies the metric, the threshold, the percentile, and the condition, and it can be tested. The specificity determines whether the constraint actually shapes the architecture and whether conformance can be verified.

Express each constraint in measurable terms. For performance, specify response times, percentiles, and conditions. For availability, specify uptime targets, recovery objectives, and what constitutes an outage. For scalability, specify the load the system must handle and how it should degrade under excess. For security, specify the threats to resist and the standards to meet. For compliance, specify the obligations and the evidence required. Vague constraints produce architecture that cannot be verified and failures that cannot be diagnosed; specific constraints produce architecture that can be tested and improved against evidence.

### Distinguish Must-Have From Nice-To-Have Constraints

Not all constraints carry the same weight, and treating them as equal distorts both architecture and prioritization. Some constraints are non-negotiable: a healthcare product must meet regulatory obligations, a payments product must not lose data, a real-time product must meet latency targets or fail its purpose. These must-haves warrant architectural investment proportional to their stakes. Other constraints are desirable but flexible: faster is better, but the product functions if it is merely acceptable; higher availability is better, but brief downtime is tolerable. These nice-to-haves should be pursued within reason but should not drive disproportionate investment.

Classify each constraint as must-have or nice-to-have, and communicate the classification. Must-have constraints are release-blocking and warrant whatever architecture they require. Nice-to-have constraints are optimized for within budget but do not block release if they fall short. This classification lets engineering invest proportionally and lets product make informed tradeoffs when constraints conflict or when investment is constrained. Treating everything as must-have over-invests and stalls; treating everything as nice-to-have under-invests and fails the non-negotiables.

### Account For Peak And Degraded Conditions

Constraints defined for normal conditions are insufficient, because systems fail users most under peak load and degraded conditions. A product that performs well at average load but collapses during a peak, or that becomes unavailable when a dependency fails, fails users at exactly the moments that matter most. Constraints must specify behavior under peak, under partial failure, and under degraded conditions, not only the happy path.

Define how the system must behave under stress. What peak load must it handle while meeting performance targets? How should it degrade when load exceeds capacity, prioritizing critical functions? What is the availability target during dependency failures or infrastructure issues? What is the recovery objective after an outage? These constraints shape architecture toward resilience, not just normal-case performance. A product whose constraints cover only normal conditions will perform well in testing and fail in production, because production is where peaks and degradations happen.

### Surface Security And Compliance Early

Security and compliance constraints have a special property: they are far cheaper to design in from the start than to retrofit, and retrofitting is sometimes impossible without rebuilding. A product architected without considering data protection obligations may require fundamental changes to comply. A product built without threat modeling may have vulnerabilities baked into its structure. These constraints must enter the architectural conversation at the beginning, not after the architecture is set.

Identify the security and compliance obligations early, based on the data handled, the users served, the jurisdictions involved, and the industry context. Translate them into architectural constraints: how data must be stored, transmitted, and isolated; what access controls are required; what logging and audit capabilities are needed; what standards must be met. Engage security and compliance expertise to define these constraints accurately. Late discovery of security or compliance needs produces expensive rework or, worse, a product that ships non-compliant and incurs the consequences.

### Revisit Constraints As The Product Evolves

Constraints defined at one stage of a product's life may not hold as the product grows. Performance targets that sufficed for early users may be inadequate at scale. Availability expectations may rise as the product becomes critical to users. Compliance obligations may expand as the product enters new markets or handles new data types. Constraints treated as permanent will drift from the product's actual needs, and the architecture built to them will diverge accordingly.

Review constraints periodically against the current and anticipated product. Has usage grown such that scalability targets need revision? Has the product become more critical such that availability targets should tighten? Have new markets or data types introduced new compliance obligations? Update constraints as needs change, and treat architectural gaps revealed by the updates as prioritized work. Constraints are not set-and-forget; they are living requirements that must track the product they serve.

### Verify Constraints Before Launch And Ongoing

Constraints that are never verified are wishes. A constraint defined but not tested provides no assurance that the architecture meets it, and the failure is discovered in production, by users, at the worst time. Verification must be part of the launch process and, for ongoing constraints like performance and availability, part of continuous monitoring.

Build constraint verification into the release process. For performance, load test against the targets before launch. For availability, confirm the recovery and failover mechanisms work. For security, conduct the appropriate testing and review. For compliance, gather the evidence required. After launch, monitor the constraints that must hold ongoing, such as performance under real load and availability in production, so degradation is detected before users are affected. Constraints without verification are assumptions, and architectural assumptions fail in production.

## Common Traps

### Undefined Constraints Defaulting To Easy

Leaving non-functional requirements unspecified so architecture optimizes for build convenience. The trap is a product that meets functional requirements and fails users on performance, reliability, or security.

### Aspirational Instead Of Measurable Constraints

Constraints too vague to design or test against. The trap is architecture that cannot be verified and failures that cannot be diagnosed.

### Normal-Condition-Only Constraints

Defining targets for average load and ignoring peaks and degraded conditions. The trap is a product that performs well in testing and collapses in production.

### Late Security And Compliance

Discovering obligations after architecture is set. The trap is expensive rework or shipping non-compliant.

### Treating All Constraints As Equal

Investing uniformly across must-have and nice-to-have constraints. The trap is over-investment in flexibility no one needs and under-investment in non-negotiables.

### Set-And-Forget Constraints

Holding initial constraints as the product grows and its needs change. The trap is architecture that drifts from the product it serves.

## Self-Check

- [ ] Each constraint traces to a product or business driver that justifies it, not an inherited or guessed number.
- [ ] Constraints are expressed in measurable terms with metrics, thresholds, percentiles, and conditions.
- [ ] Constraints are classified as must-have or nice-to-have, and investment is calibrated accordingly.
- [ ] Behavior under peak load, partial failure, and degraded conditions is specified, not only normal-case performance.
- [ ] Security and compliance constraints were identified early and translated into architectural requirements before the architecture was set.
- [ ] Constraints are reviewed periodically and updated as the product grows, enters new markets, or handles new data.
- [ ] Constraint verification, including load, availability, security, and compliance testing, is part of the launch process.
- [ ] Ongoing constraints like performance and availability are monitored in production to detect degradation.
- [ ] No constraint is treated as permanent if the product needs it serves have changed.
- [ ] The cost of not meeting each must-have constraint is understood and accepted only deliberately, not by oversight.
