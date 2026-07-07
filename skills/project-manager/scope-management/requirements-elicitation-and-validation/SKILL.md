---
name: requirements_elicitation_and_validation.md
description: Use when the agent is gathering and validating project requirements, running elicitation sessions with stakeholders, distinguishing needs from wants and stated from real requirements, managing conflicting requirements across stakeholders, validating requirements against business goals, or diagnosing why delivered scope does not meet actual needs despite documented requirements.
---

# Requirements Elicitation And Validation

Requirements are the bridge between what stakeholders need and what the project delivers, and they fail in characteristic ways. The most common failure is not missing requirements but capturing the wrong ones: stakeholders state what they think they want, which is often not what they need, and the project dutifully delivers it, only to find that the real problem remains unsolved. This happens because stakeholders often describe solutions rather than problems, because different stakeholders have conflicting requirements that are never reconciled, because requirements are captured at a single point and never validated as understanding evolves, and because the elicitation focuses on what stakeholders say rather than on observing what they actually do. The judgment problem is eliciting requirements that reflect real needs rather than stated wants, reconciling conflicts across stakeholders, validating requirements against business goals and against evolving understanding, and recognizing that requirements are discovered through the project, not captured once at the start.

Use this skill when gathering or validating requirements, running elicitation sessions, reconciling stakeholder conflicts, validating against goals, or diagnosing requirements-driven delivery failures. The goal is to prevent the agent from treating stated wants as requirements, from capturing requirements once and freezing them, from leaving conflicts unreconciled, and from delivering precisely what was specified to stakeholders who find it useless.

## Core Rules

### Distinguish Problems From Solutions In Stakeholder Requests

Stakeholders routinely express requirements as solutions: "we need a dashboard," "we need a new system," "we need a report." These solution-statements obscure the underlying problem, and building to them produces a solution that may not address the actual need. The elicitor's job is to dig beneath solution-statements to the problems they are meant to solve.

Distinguish by:

- asking why a stakeholder wants a proposed solution, repeatedly, to surface the underlying problem;
- exploring the current process and its pain points rather than accepting the proposed fix;
- framing requirements around problems and outcomes, not around prescribed solutions;
- being willing to propose alternative solutions that address the problem better than the stakeholder's.

A requirement that specifies a solution without understanding the problem locks in a fix that may not fit the need.

### Capture Real Needs, Not Just Stated Wants

Stated wants are unreliable guides to real needs. Stakeholders ask for what they can imagine, which is constrained by what they know, and they often cannot articulate what they would value until they see it. Capturing stated wants as requirements produces a product limited by stakeholders' current imagination rather than by their actual needs.

Capture real needs by:

- observing how stakeholders actually work, not just asking how they do;
- identifying workarounds and pain points that indicate unmet needs;
- using prototypes and mockups to surface needs stakeholders could not articulate in the abstract;
- distinguishing what stakeholders say they want from what their behavior shows they need.

A stakeholder who asks for a feature may need something different that they cannot yet specify.

### Reconcile Conflicting Requirements Across Stakeholders

Different stakeholders have different, often conflicting, requirements. Operations wants speed; compliance wants control; users want simplicity; sponsors want reporting. Capturing each stakeholder's requirements without reconciling conflicts produces a requirements set that is internally contradictory and impossible to satisfy. Conflicts must be surfaced and resolved.

Reconcile by:

- identifying where stakeholder requirements conflict explicitly rather than papering over differences;
- elevating conflicts to a level where they can be resolved, often by clarifying business goals and priorities;
- using tradeoff analysis to make the costs of each position visible;
- making conflict resolution a decision, with rationale, rather than an implicit accommodation;
- documenting resolved conflicts so they do not resurface as disputes later.

Requirements that conflict silently guarantee that some stakeholders will be disappointed regardless of what is delivered.

### Validate Requirements Against Business Goals

Requirements accumulate detail and drift from the business goals they are meant to serve. A requirement that seemed important at elicitation may not actually advance the business goal when examined. Validating requirements against goals, asking how each requirement contributes to the intended outcome, prunes requirements that do not serve the goals.

Validate against goals by:

- stating the business goals the project is meant to achieve explicitly;
- tracing each requirement to the goal it serves, and challenging those that trace weakly;
- pruning requirements that do not advance a goal, however much stakeholders want them;
- recognizing requirements that serve stakeholder preference rather than business need.

A requirements set that is not validated against goals becomes a wish list that may miss the actual objective.

### Treat Requirements As Discovered, Not Captured Once

The idea that requirements can be fully captured at project start is one of the most damaging myths in project management. Understanding evolves as the project progresses, stakeholders see prototypes and refine their needs, and the environment changes. Treating requirements as captured-once-and-frozen guarantees that the project delivers against an outdated understanding.

Treat as discovered by:

- expecting and planning for requirements evolution through the project;
- using iterative or incremental approaches that allow requirements to refine with feedback;
- validating requirements continuously against developing understanding;
- avoiding the big-design-up-front approach that assumes requirements can be frozen.

Requirements discovered through the project reflect real needs; requirements frozen at start reflect early, incomplete understanding.

### Use Multiple Elicitation Techniques

No single elicitation technique captures all requirements. Interviews surface what stakeholders can articulate; observation surfaces what they do but cannot describe; workshops surface group dynamics and conflicts; document analysis surfaces implicit requirements in existing processes. Relying on one technique misses what the others would reveal.

Use multiple techniques by:

- combining interviews, observation, workshops, and document analysis;
- using interviews for individual stakeholder perspectives;
- using observation to reveal actual practice and workarounds;
- using workshops to surface and resolve cross-stakeholder issues;
- using document and process analysis to capture implicit and structural requirements.

Requirements elicited through a single technique are systematically incomplete in characteristic ways.

### Validate Requirements With Stakeholders Before Committing

Requirements documented but not validated with stakeholders are assumptions, not requirements. Stakeholders may not have meant what the document captures, may have changed their minds, or may not have understood the implications. Validating requirements with stakeholders, through review and walkthrough, confirms that the documented requirements reflect their actual needs.

Validate by:

- reviewing documented requirements with stakeholders before committing to them;
- using walkthroughs that force stakeholders to engage with specifics, not just sign off;
- testing requirements against scenarios to confirm they handle real situations;
- treating stakeholder signoff as informed confirmation, not a formality.

A stakeholder who signs off without understanding has not validated; they have deferred, and disappointment will follow.

### Prioritize Requirements To Enable Tradeoffs

Not all requirements are equal, and when constraints force tradeoffs, as they inevitably do, the project needs to know which requirements matter most. Prioritizing requirements, by value, risk, or dependency, enables informed tradeoffs rather than arbitrary cuts when pressure arrives.

Prioritize by:

- using techniques like MoSCoW, must-should-could-won't, or relative prioritization;
- prioritizing by value to the business goal, not by stakeholder loudness;
- identifying the minimum viable set that would deliver value;
- revisiting priorities as understanding and constraints evolve.

Unprioritized requirements force arbitrary cuts under pressure; prioritized requirements enable deliberate tradeoffs.

## Common Traps

### Accepting Solution-Statements As Requirements

Stakeholders describe solutions, not problems. Dig beneath to the underlying need.

### Stated Wants Treated As Real Needs

Stakeholders cannot always articulate needs. Observe behavior and use prototypes to surface real needs.

### Unreconciled Conflicting Requirements

Different stakeholders want different things. Surface and resolve conflicts explicitly.

### Requirements Not Validated Against Goals

Requirements drift from goals. Trace each to its goal and prune those that do not serve it.

### Frozen Requirements Captured Once

Understanding evolves. Treat requirements as discovered through the project.

### Single-Technique Elicitation

No technique captures everything. Combine interviews, observation, workshops, and analysis.

### Unvalidated Documentation Treated As Requirements

Documents not reviewed with stakeholders are assumptions. Validate through walkthrough before committing.

### Unprioritized Requirements

When constraints bite, unprioritized requirements force arbitrary cuts. Prioritize by value.

## Self-Check

- [ ] Requirements are framed around problems and outcomes, not around stakeholder-prescribed solutions.
- [ ] Real needs are captured through observation and prototypes, not just stated wants.
- [ ] Conflicting requirements across stakeholders are surfaced and reconciled explicitly.
- [ ] Requirements are validated against business goals, with weakly-traced ones pruned.
- [ ] Requirements are treated as discovered through the project, not frozen at start.
- [ ] Multiple elicitation techniques are combined to avoid systematic gaps.
- [ ] Requirements are validated with stakeholders through walkthrough before commitment.
- [ ] Requirements are prioritized to enable informed tradeoffs under constraints.
- [ ] No requirement is committed that traces weakly to a business goal.
- [ ] Stakeholder signoff reflects informed engagement, not deferred formality.