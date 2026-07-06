---
name: autonomous-systems-and-emerging-tech-risk-assessment.md
description: Use when the agent is deploying autonomous vehicles, drones, robotics, or other autonomous physical systems, allocating liability across the autonomy stack, designing fail-safe and handover behavior, or assessing novel regulatory and insurance risk for technologies where the legal framework is still emerging.
---

# Autonomous Systems and Emerging-Tech Risk Assessment

Autonomous physical systems, self-driving vehicles, delivery drones, industrial and service robots, and autonomous marine and air systems, create a class of risk that existing legal frameworks were not designed for. The judgment problem is allocating liability across the manufacturer, the software provider, the operator, and the owner when an autonomous system causes harm; designing the fail-safe and handover behavior that determines whether a human or the system is responsible at the moment of failure; and operating responsibly in a regulatory environment that is still catching up to the technology. Agents often assume that existing product-liability and operator-license frameworks will simply extend, or that the absence of specific regulation means the absence of obligation.

This skill applies to organizations developing, deploying, operating, or insuring autonomous physical systems. The regulatory framework for autonomous systems is nascent, fragmented, and jurisdiction-specific, and the liability allocation is being actively litigated. This skill provides a framework for responsible decision-making under uncertainty, not legal conclusions. Confirm the applicable operational rules, permitting requirements, and liability framework with specialist counsel and insurers before deployment.

## Core Rules

### Map the Liability Allocation Across the Full Autonomy Stack

When an autonomous system causes harm, liability does not default to a single party; it is allocated across the autonomy stack based on where the failure occurred and who controlled that layer. The vehicle or robot manufacturer may be liable for hardware defects, the autonomy software provider for perception or decision failures, the map and data provider for erroneous inputs, the operator or fleet manager for maintenance and deployment-context decisions, and the human supervisor for failures to intervene when the system requested handover. The contractual allocation of responsibility among these parties, and the insurance covering each, must be mapped before deployment, because ambiguity produces gaps where no party is clearly responsible and leaves the deploying organization exposed. Do not assume that procuring an autonomous system shifts all liability to the manufacturer; the deployer's own obligations around maintenance, operating-domain selection, and supervision remain.

### Design the Handover and Fail-Safe Logic as a Liability and Safety Decision

The design of the human-machine handover, when the system requests human intervention, how much warning it gives, and what it does if the human does not respond, is simultaneously a safety decision and a liability-allocation decision. A handover that gives insufficient warning or that occurs in a situation the human cannot safely recover creates a transfer of responsibility that the human cannot meet, and the resulting failure may be attributed to the system design rather than to the human. Conversely, a system that requires constant human supervision that operators cannot realistically sustain creates a nominal human responsibility with no practical oversight. Design handover and fail-safe logic that is safe in its own right, including the ability to reach a safe state without human intervention when handover fails, and document the design reasoning because it will be central to any incident investigation.

### Define and Enforce the Operational Design Domain

Autonomous systems are safe within a defined operational design domain, the specific conditions of location, weather, traffic, time, and scenario for which they are validated, and unsafe outside it. Operating a system outside its validated domain is a deployment decision that creates risk the system was not designed to manage. Define the domain explicitly, build technical and procedural controls that prevent or warn against operation outside it, and document that the deployment context falls within the domain. A system that performs well in its domain but is deployed outside it, because commercial pressure pushed the boundary, is a foreseeable cause of incidents and a liability exposure for the deployer.

### Operate Responsibly Where Regulation Has Not Caught Up

The absence of specific regulation does not mean the absence of obligation. General product-safety, negligence, road-traffic, aviation, consumer-protection, and tort principles still apply, and regulators and courts will apply them to autonomous systems even where dedicated rules do not yet exist. Adopt voluntary safety standards and frameworks, conduct rigorous testing and validation before deployment, maintain the documentation that demonstrates responsible development, and engage with regulators proactively rather than exploiting regulatory lag. Operating in the gray zone between technology and regulation requires conservative judgment, because the legal framework that will eventually apply is likely to be applied retroactively to incidents that occur in the interim.

### Address Insurance and Bonding for Novel Risk

Conventional insurance products may not cover autonomous-system risk, or may contain exclusions for autonomous operation, software failure, or cyber causes that are precisely the failure modes of these systems. Review existing policies for exclusions before deployment, and obtain coverage specifically structured for the autonomous-system risk profile, including the gap between the manufacturer's product liability and the operator's general liability. Some jurisdictions require specific bonding, insurance, or financial-responsibility demonstrations for autonomous operation. Confirm that the insurance and bonding actually cover the failure modes the system presents, not only the conventional failure modes the policies were written for.

### Build Incident Response and Investigation Capability Specific to Autonomy

Autonomous-system incidents require investigation capabilities that conventional incident response does not provide. The investigation must capture the system's sensor data, decision logs, model versions, operating-domain status, handover requests, and the human's response, in addition to the conventional physical evidence. The data needed to reconstruct an incident may be ephemeral or voluminous, and the preservation window is short. Build the data-capture and preservation capability before deployment, establish the protocols for regulator and insurer notification, and recognize that the investigation will scrutinize the system's design and the deployer's operational decisions as much as the immediate circumstances.

## Common Traps

### Assuming Procurement Shifts Liability to the Manufacturer

Deployers retain obligations around maintenance, operating-domain selection, and supervision. Map the liability allocation across the full autonomy stack before deployment.

### Handover Design That Transfers Unrecoverable Responsibility

A handover with insufficient warning, or in an unrecoverable situation, may be attributed to system design. Design fail-safe logic that can reach a safe state without human intervention.

### Deploying Outside the Validated Operational Design Domain

Operating outside the validated domain is a deployment decision that creates unmanaged risk. Define the domain, enforce it technically, and document that the context falls within it.

### Exploiting Regulatory Lag

The absence of specific rules does not remove obligation under general safety, negligence, and tort principles. Adopt voluntary standards and operate conservatively in the gray zone.

### Uninsured Failure Modes

Conventional policies may exclude the software, cyber, and autonomous-operation failures that define these systems. Obtain coverage structured for the autonomous risk profile.

### No Autonomy-Specific Incident Capability

Conventional incident response cannot reconstruct an autonomous-system failure. Build sensor, decision-log, and model-version capture and preservation before deployment.

## Self-Check

- Did I map the liability allocation across the full autonomy stack, manufacturer, software, data, operator, and supervisor, and confirm the deployer's retained obligations?
- Did I design handover and fail-safe logic that is safe in its own right, including the ability to reach a safe state without human intervention, and document the reasoning?
- Did I define and technically enforce the operational design domain, and document that the deployment context falls within it?
- Am I operating responsibly under general safety and tort principles where dedicated regulation does not yet exist, adopting voluntary standards rather than exploiting regulatory lag?
- Did I review existing insurance for autonomous-operation exclusions and obtain coverage structured for the system's actual failure modes and any required bonding?
- Did I build autonomy-specific incident response capability, including sensor, decision-log, and model-version capture and preservation?
- Did I confirm the applicable operational rules, permitting, and liability framework with specialist counsel and insurers before deployment?
- Are assumptions, uncertainties, and confidence levels stated explicitly rather than buried in a confident-sounding conclusion?
