---
name: compliance_walkthroughs_and_design_assessment.md
description: Use when the agent is performing compliance walkthroughs, mapping processes and controls, interviewing control owners, evaluating whether controls are designed effectively, or confirming that a control can theoretically achieve its objective before testing operation.
---

# Compliance Walkthroughs And Design Assessment

A walkthrough is the structured exercise of tracing a transaction or process from initiation to completion to confirm how it actually works, who performs each step, what evidence is generated, and where the controls sit. It is the foundation for any conclusion about a control. Testing the operation of a control that is poorly designed, misunderstood, or disconnected from the real process produces a meaningless clean result. Design assessment is the judgment of whether a control, as designed and as understood by its owner, can plausibly achieve its objective.

Use this skill before testing control operation, before concluding a control is effective, when onboarding a new process, or after a process change. The goal is to make the agent treat the walkthrough as an evidence-gathering and gap-finding exercise, not as a courtesy meeting that ends with a process flow diagram.

## Core Rules

### Trace The Process End To End With The People Who Perform It

A walkthrough that relies only on documented procedures will miss what actually happens. Procedures decay, systems change, and workarounds develop. Walk the process with the people who do the work.

For each process walkthrough:

- start from initiation and follow the transaction to final recording, reporting, or disposal;
- identify each step, who performs it, and what system is used;
- capture the inputs, the decision points, the approvals, and the outputs;
- note where the process crosses systems, teams, geographies, or third parties;
- record the evidence generated at each step and where it is stored;
- ask about exceptions, workarounds, and what happens when the system is down.

Interview more than one person where a process spans roles. A single informant may describe the ideal rather than the actual. Cross-check what different performers say, especially at handoff points where failures often hide.

### Map The Controls Into The Process, Not Beside It

Controls are effective only when they sit at the right point in the flow. A control described in a policy but not embedded where the work happens will not operate.

For each control identified:

- locate it precisely in the process flow;
- confirm whether it is preventive or detective and whether that placement is appropriate;
- confirm it acts on the correct population and at the correct time;
- identify the trigger that causes the control to run;
- identify the evidence it generates and whether that evidence is retained;
- identify what happens when the control flags an exception.

A reconciliation performed after the money has left the account is detective but not preventive; that distinction matters for the risk it addresses. A control that runs on a subset of transactions leaves the rest uncovered.

### Confirm The Control Owner Understands The Objective

A control operated by someone who does not understand why it exists will degrade into ritual. During the walkthrough, ask the owner to explain the risk the control addresses and what they look for.

Assess:

- whether the owner can state the control objective in their own words;
- whether they know what constitutes an exception;
- whether they know what to do when an exception arises;
- whether they know who to escalate to;
- whether they believe the control is taken seriously or treated as bureaucracy.

If the owner cannot explain the objective, that is itself a design weakness, regardless of whether the steps are being performed. Training and communication are part of control design.

### Evaluate Whether The Design Can Achieve The Objective

Design effectiveness is the judgment that, if the control operates exactly as designed, it would achieve its objective. This is a necessary precondition to testing operating effectiveness.

Evaluate the design against:

- the specific risk it is meant to mitigate;
- whether the control type matches the risk (preventive controls for high-likelihood risks, detective controls where prevention is impractical);
- whether the control covers the entire population or only part;
- whether the threshold or parameter is set correctly;
- whether the evidence generated is sufficient to prove operation;
- whether the control can be bypassed or overridden, and how;
- whether the control depends on a prior control that may be weak;
- whether the control is timely enough to prevent or detect the harm;
- whether the control accounts for exceptions and edge cases.

A control that approves gifts above a threshold but cannot see gifts routed through a different system is not designed effectively, even if it works perfectly within its narrow view.

### Identify Design Gaps Before Testing Operation

The purpose of design assessment is to find gaps early. Testing the operation of a control with a known design gap wastes effort and produces misleading conclusions.

Common design gaps:

- the control covers only one channel while transactions flow through several;
- the threshold or rule is outdated relative to current risk or regulation;
- the control relies on manual evidence that can be reconstructed or backdated;
- the control detects but does not require resolution of exceptions;
- the control has no owner or the owner has changed without handover;
- the control depends on data that is incomplete or unreliable;
- the control was designed for a process that has since been automated or outsourced;
- the control has no escalation path for unresolved exceptions.

Document each design gap, its risk implication, and whether it must be remediated before operating effectiveness can be meaningfully tested.

### Document The Walkthrough With Evidence, Not Only Narrative

A walkthrough conclusion must be supportable. A narrative that says "the process works as follows" without evidence is not defensible to a regulator or auditor.

Documentation should include:

- a process flow or narrative covering each step and decision point;
- the controls mapped to specific points in the flow;
- samples of actual evidence examined, such as a real approval, reconciliation, or log entry;
- the systems, reports, and data sources used;
- the people interviewed and their roles;
- exceptions or workarounds described by staff;
- identified design gaps and their risk implications;
- the conclusion on design effectiveness and its basis.

Retain copies or references to the actual evidence inspected, so the walkthrough can be corroborated later.

### Re-Walk The Process After Material Change

Processes are not static. A walkthrough performed a year ago may no longer describe reality after a system migration, reorganization, new product, outsourcing, or regulatory change.

Triggers for re-walking include:

- system implementation or significant configuration change;
- process reengineering or reorganization;
- new product, market, or customer segment;
- outsourcing or vendor change;
- regulatory change affecting the process;
- a control failure or incident;
- turnover in key control owners;
- merger, acquisition, or divestiture.

Treat the walkthrough as a living understanding. A stale walkthrough is a common cause of testing the wrong control or the wrong process.

### Use The Walkthrough To Scope Operating Effectiveness Testing

The walkthrough should directly inform what is tested, how, and on what population. A good walkthrough narrows the test to the controls and attributes that matter.

Translate the walkthrough into:

- the specific controls deemed designed effectively and ready for operating testing;
- the controls with design gaps that require remediation before testing;
- the population definition derived from the actual process flow;
- the attributes and evidence identified during the walk;
- the risk areas where deeper or expanded testing is warranted;
- the handoff points and edge cases that deserve targeted testing.

## Common Traps

### Trusting The Documented Procedure Over The Actual Process

Written procedures often lag reality. Walk the live process with performers, not only the procedure manual.

### Interviewing Only One Person Across A Multi-Role Process

A single informant may describe the ideal or their slice. Cross-check at handoffs where failures hide.

### Mapping Controls Beside The Process Rather Than Within It

A control that exists in policy but not at the point of work will not operate. Confirm placement in the flow.

### Concluding Design Effective Without Examining Bypass And Override

If a control can be circumvented, its design is incomplete. Ask explicitly about overrides, manual entries, and off-system paths.

### Skipping Design Assessment And Jumping To Operation Testing

Testing operation of a poorly designed control yields a false clean result. Confirm design effectiveness first.

### Treating The Walkthrough As A One-Time Formality

Processes change. Re-walk after system, organizational, product, or regulatory change.

### Accepting Reconstructed Evidence As Normal

If walkthrough evidence is routinely reconstructed or backdated, the control design enables evasion. Flag this as a design gap.

## Self-Check

- Does the walkthrough trace the process end to end with the people who actually perform each step, cross-checked at handoffs?
- Are controls mapped precisely into the process flow at the correct point, population, and timing?
- Can the control owner explain the objective, exceptions, escalation, and why the control matters?
- Has design effectiveness been evaluated against the specific risk, population coverage, threshold, bypass, timeliness, and edge cases?
- Are design gaps identified, risk-rated, and flagged for remediation before operating effectiveness testing?
- Does the walkthrough documentation include a process flow, mapped controls, actual evidence samples, systems, interviewees, workarounds, and conclusions?
- Is the process re-walked after system, organizational, product, vendor, regulatory, or personnel change?
- Does the walkthrough directly scope the operating effectiveness test, including population, attributes, and targeted risk areas?
- Are bypass, override, manual entry, and off-system paths explicitly examined rather than assumed away?
- Is the walkthrough conclusion supported by retained evidence rather than narrative alone?
