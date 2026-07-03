---
name: nonfunctional_specifications.md
description: Use when the agent is specifying non-functional requirements such as performance reliability security and usability, setting measurable quality targets, or ensuring quality attributes are defined concretely rather than as aspirations so they can be designed for and verified.
---

# Non-Functional Specifications

Non-functional requirements (NFRs) specify the quality attributes of the system: how fast, how reliable, how secure, how usable, how scalable it must be. Unlike functional requirements, which describe what the system does, NFRs describe how well the system does it. They are often the difference between a system that works and a system that succeeds, because users experience quality as much as functionality. Done well, NFRs are specific, measurable targets that can be designed for and verified. Done poorly, they are aspirational statements like "the system should be fast and secure" that provide no guidance for design and no basis for verification. Agents often treat NFRs as secondary, because they are less concrete than functional requirements, which leaves the quality attributes undefined and discovered only when they fail.

The harm this skill prevents is the system that meets its functional requirements but fails on quality. A system that does the right things but too slowly, too unreliably, or too insecurely does not serve its users, and fixing quality problems after launch is far more expensive than designing for them. NFRs specified early, as concrete targets, allow the architecture and design to account for them, which is the only cost-effective way to achieve quality attributes. Aspirational NFRs, specified late or not at all, produce systems where quality is an afterthought and a source of expensive rework.

Use this skill before answering questions such as "how do we specify performance requirements", "what are our non-functional requirements", "how reliable does the system need to be", or "how do we write measurable quality targets". The goal is to prevent the agent from producing NFRs that are aspirations rather than specifications.

## Core Rules

### Make Every NFR Specific And Measurable

Every non-functional requirement should be specific enough to understand and measurable enough to verify. "The system should be fast" is neither; "the system should respond to 95 percent of user requests within 500 milliseconds under normal load" is both. Specificity means the requirement states exactly what quality attribute is constrained and under what conditions. Measurability means the requirement can be checked against evidence, whether through testing, monitoring, or analysis. Without these properties, an NFR is an aspiration that cannot be designed for or verified.

Making NFRs measurable requires defining the metric, the target, the conditions, and the measurement method. For performance: response time, the percentile and threshold, the load conditions, and how response time is measured. For reliability: availability, the target percentage, the measurement window, and what counts as downtime. For each quality attribute, specify these elements, because omitting any leaves the requirement ambiguous and unverifiable. The effort to make an NFR measurable is the effort to make it real.

### Specify NFRs In The Context Of Real Workload And Conditions

NFRs are meaningful only in context. A performance target of 500 milliseconds is meaningless without specifying the workload: how many users, what operations, what data sizes, what network conditions. A reliability target of 99.9 percent availability is meaningless without specifying the measurement window and what counts as an incident. Specify NFRs in the context of the real workloads and conditions the system will face, so that the targets are grounded in reality rather than abstract.

Context also includes the variation in conditions. A system may need to meet performance targets under normal load and degraded-but-functional behavior under peak load. A system may need to maintain reliability during normal operation and graceful degradation during failures. Specify how the quality attributes should behave across the range of conditions, not just the typical case, because the edge conditions are where quality attributes often fail. A target that holds only under ideal conditions provides false confidence about the system's real quality.

### Treat NFRs As Design Drivers, Not Afterthoughts

NFRs are not checklist items to verify at the end; they are design drivers that shape the architecture. A performance target of 50 milliseconds shapes the architecture differently than one of 500 milliseconds. A reliability target of five nines shapes it differently than two nines. Specify NFRs early, before architecture decisions are made, so that the design can account for them. NFRs discovered after the architecture is set are expensive or impossible to achieve, because the architecture may not support them.

This means NFRs should be part of the earliest conversations about a system, alongside the functional requirements. The product manager, the architects, and the engineers should discuss what quality attributes the system needs and what targets are achievable, because these decisions constrain the architecture and the design. Treating NFRs as afterthoughts, to be handled once the functionality is built, virtually guarantees that they will not be met, because the design was not made to meet them.

### Identify The Quality Attributes That Matter For This System

Not all quality attributes matter equally for every system. A real-time gaming system prioritizes latency above almost everything. A banking system prioritizes security and reliability above almost everything. A content site prioritizes availability and scalability. Identify which quality attributes matter most for the specific system, based on its users, its use cases, and its business context, and invest the specification effort there. Trying to specify all attributes to the same depth wastes effort on attributes that do not matter and under-specifies the ones that do.

The attributes that matter are those whose failure would harm the users or the business. For each, ask: what happens if this attribute is not met? If the answer is severe harm, the attribute matters and should be specified precisely. If the answer is minor inconvenience, the attribute can be specified at a lower level of rigor. This risk-based approach focuses specification effort where it pays off, producing NFRs that address the real risks rather than generic quality aspirations.

### Account For Tradeoffs Between Quality Attributes

Quality attributes trade off against each other. Higher performance may come at the cost of consistency. Higher reliability may come at the cost of complexity and cost. Higher security may come at the cost of usability. These tradeoffs are inherent, and a good NFR specification acknowledges them rather than pretending that all attributes can be maximized simultaneously. Identify the tradeoffs that the system faces, and make explicit decisions about which attributes to prioritize and which to accept lower targets on.

Making tradeoffs explicit allows stakeholders to understand and contest the priorities. A decision to prioritize performance over consistency should be documented and justified, so that stakeholders who depend on consistency can weigh in before the decision is locked into the architecture. Implicit tradeoffs, made by engineers optimizing for one attribute without acknowledging the cost to others, produce systems whose priorities no one explicitly chose. Explicit tradeoffs make the quality priorities a deliberate decision.

### Specify NFRs For Verification, Not Just For Design

NFRs should be specified not only to guide design but to enable verification. This means defining how each NFR will be checked: what test, what monitoring, what analysis will confirm that the target is met. A performance NFR is verified through load testing and production monitoring. A reliability NFR is verified through availability tracking and incident review. A security NFR is verified through penetration testing and code analysis. Specifying the verification method ensures that the NFR can be confirmed met, rather than assumed met.

Verification also includes the ongoing monitoring that confirms the NFR continues to be met in production. Quality attributes degrade over time as systems change and loads grow, and without monitoring, the degradation is noticed only when users complain. Specify the monitoring that will track each NFR in production, so that quality is maintained, not just achieved at launch. This connects the NFR specification to operational reality, ensuring that quality attributes are sustained, not just demonstrated once.

## Common Traps

### Aspirational NFRs Without Metrics

"Fast," "secure," "reliable" without targets. The trap is quality attributes that cannot be designed for or verified.

### NFRs Without Context

Targets specified without workload or conditions. The trap is requirements that are meaningless or misleading.

### NFRs As Afterthoughts

Specifying quality after the architecture is set. The trap is targets that are expensive or impossible to achieve.

### All Attributes Specified Equally

Investing equally in attributes that matter unequally. The trap is wasted effort and under-specified critical attributes.

### Ignoring Tradeoffs Between Attributes

Pretending all attributes can be maximized. The trap is implicit priorities that no one deliberately chose.

### NFRs Without Verification Methods

Requirements without specified checks. The trap is quality that is assumed rather than confirmed.

## Self-Check

- [ ] Every NFR is specific and measurable, with metric, target, conditions, and method defined.
- [ ] NFRs are specified in the context of real workloads and conditions, including variation and edge cases.
- [ ] NFRs are treated as design drivers, specified early enough to shape the architecture.
- [ ] The quality attributes that matter most for this system are identified and specified with appropriate rigor.
- [ ] Tradeoffs between quality attributes are acknowledged, with explicit decisions about priorities.
- [ ] Each NFR has a specified verification method, including ongoing production monitoring.
- [ ] The NFR specification enables both design for quality and verification that quality is achieved and sustained.
