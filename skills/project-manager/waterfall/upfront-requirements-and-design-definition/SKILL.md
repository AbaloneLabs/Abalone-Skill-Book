---
name: upfront_requirements_and_design_definition.md
description: Use when the agent is defining comprehensive requirements and design upfront for a predictive project, managing requirements completeness and traceability before build, or deciding how much upfront definition is enough before committing to implementation.
---

# Upfront Requirements and Design Definition

In a predictive lifecycle, the requirements and design defined upfront are the foundation everything else is built on, and the cost of getting them wrong is amplified by every downstream phase. Because the model assumes definition before execution, an incomplete, ambiguous, or internally inconsistent specification does not stay a documentation problem; it becomes rework in design, rework in build, defects in test, and gaps at delivery. The discipline of upfront definition is therefore not paperwork for its own sake but risk management: the more completely and correctly the work is specified before commitment, the more predictable the build becomes. The difficulty is that complete definition is genuinely hard, and the line between enough definition and gold-plated over-definition is not obvious.

The judgment problem is to decide how much definition is enough for a given project, to drive requirements and design to the level of completeness that downstream phases actually need, and to manage the inevitable tension between the desire for certainty and the reality that some things can only be discovered by building. Agents tend to either under-define, treating upfront work as a formality and paying for it in rework, or over-define, producing voluminous specifications whose detail exceeds what anyone can use and that age into inaccuracy. The skilled move is to define to the level that reduces downstream risk, no more and no less.

## Core Rules

### Define to the Level Downstream Phases Need, Not to Maximum Possible

Enough definition is the level at which the next phase can proceed with acceptable risk of rework. Build needs requirements specific enough to implement without constant clarification and a design concrete enough to code against. Beyond that, additional detail adds documentation cost without proportional risk reduction, and often introduces inaccuracy as speculative detail ages. Ask, for each artifact: what decision or activity does this support, and is it specific enough to support it? Define to that level and stop. Over-definition is not thoroughness; it is wasted effort that creates a false sense of certainty.

### Distinguish Stable From Volatile Requirements and Treat Them Differently

Within an upfront definition, some requirements are genuinely stable (regulatory mandates, interface contracts, core business rules) and some are volatile (user preferences, UI behavior, edge cases). Invest definition effort proportionally: drive stable requirements to high completeness and precision, and acknowledge volatile ones as assumptions with explicit uncertainty rather than pretending they are fixed. Marking volatility explicitly lets the design accommodate likely change in those areas rather than freezing a guess. Treating all requirements as equally fixed produces a specification that is precise where it should be and rigid where it should flex.

### Make Requirements Testable and Internally Consistent

A requirement that cannot be tested cannot be verified, and a set of requirements that contradict each other cannot all be satisfied. Each requirement should be written so that test cases can be derived from it, and the full set should be checked for consistency, completeness against goals, and absence of conflict. Inconsistencies caught in requirements are cheap to fix; the same inconsistencies caught in build or test are expensive. Build verification of the specification itself, walkthroughs, reviews, consistency checks, into the definition phase rather than assuming the document is correct because it was written.

### Carry Design to the Depth That Resolves Build-Time Risk

Upfront design should resolve the architectural and integration decisions whose late reversal would be expensive: system structure, component boundaries, data model, external interfaces, key technology choices, and the approach to major risks. It should not attempt to pre-specify every implementation detail, which is both impossible and counterproductive, because much detail only becomes clear during build. Define the design decisions whose cost of change is high, and leave the low-cost-of-change decisions to build. The test of design completeness is whether the remaining unknowns are cheap to resolve during implementation.

### Establish Traceability From Goals Through Requirements to Design

Traceability links each requirement to the business goal it serves and to the design element that satisfies it, and later to the test that verifies it. Its purpose is not bureaucratic coverage but the ability to assess impact: when a requirement changes, traceability shows what design and test must change; when a goal is dropped, it shows which requirements become orphaned. Establish traceability at a level that supports impact analysis without drowning in links. Without it, change becomes guesswork and completeness cannot be assessed.

### Define and Control the Baseline, With a Realistic Change Process

Once requirements and design are defined to the needed level, they become a baseline: the reference against which build proceeds and against which change is measured. Define what constitutes the baseline, who approves it, and how changes to it are handled. Because predictive projects do encounter change, the change process must be real but not so heavy that it is bypassed. A change process that is too light lets scope leak silently; one that is too heavy drives people to avoid it. Calibrate the process to the project's risk and size.

### Validate Definition Against Stakeholders Before Committing

Upfront definition is only as good as its accuracy, and accuracy requires validation with the stakeholders who know the real needs. Reviews, walkthroughs, and prototypes that expose the specification to users and subject-matter experts catch misinterpretations before they become built-in defects. Do not treat stakeholder sign-off as a formality; use it as a genuine validation that the definition captures real needs. A specification signed off without real examination will be wrong in ways that surface expensively later.

### Acknowledge the Limit of Upfront Definition and Plan for Discovery

Even in a predictive project, some things cannot be known until built: performance characteristics, integration surprises, usability issues. Pretending otherwise produces a brittle plan. Acknowledge the residual uncertainty, reserve contingency for the discovery that will occur, and define a small number of risk-reduction activities (prototypes, spikes, proofs of concept) within the predictive structure to retire the highest unknowns before full build. Honest predictive planning integrates discovery rather than denying it.

## Common Traps

### Under-Defining and Calling It Agility

The project is predictive but under-invests in upfront definition, then pays in rework when build discovers ambiguity. The trap is treating definition as optional overhead. Define to the level downstream phases need.

### Over-Defining to Manufacture Certainty

Voluminous specifications are produced whose detail exceeds what anyone can use, creating false certainty and aging into inaccuracy. The trap is mistaking volume for completeness. Define to the level that reduces risk, then stop.

### Treating All Requirements as Equally Fixed

Stable and volatile requirements are specified identically, so the design is rigid where it should flex. The trap is ignoring volatility. Distinguish and mark requirement stability explicitly.

### Non-Testable or Contradictory Requirements

Requirements are written that cannot be tested or that conflict with each other, surfacing as expensive defects in build or test. The trap is skipping specification verification. Review for testability and consistency.

### Design That Pre-Specifies Implementation Detail

Design attempts to resolve every implementation decision upfront, which is impossible and produces rigid, inaccurate detail. The trap is over-reaching the appropriate design depth. Resolve high-cost-of-change decisions; leave the rest to build.

### Sign-Off as Formality Without Validation

Stakeholders sign off without genuinely examining the specification, so misinterpretations are built in. The trap is ceremony without verification. Use reviews and prototypes for real validation.

### No Traceability, So Change Is Guesswork

Without traceability, the impact of a requirement change cannot be assessed, and orphaned requirements go unnoticed. The trap is treating traceability as bureaucracy. Establish it at a level that supports impact analysis.

### Denying Residual Uncertainty

The plan assumes everything can be known upfront, so when discovery occurs during build, it is treated as failure rather than as the predictable residual uncertainty it is. The trap is false completeness. Acknowledge and plan for discovery.

## Self-Check

- [ ] Is definition driven to the level downstream phases need for acceptable rework risk, neither under- nor over-defined?
- [ ] Are stable and volatile requirements distinguished and treated differently, with volatile ones marked as assumptions?
- [ ] Is each requirement testable, and has the specification been checked for internal consistency and completeness against goals?
- [ ] Does design resolve high-cost-of-change decisions (architecture, interfaces, data model, key risks) while leaving low-cost decisions to build?
- [ ] Is traceability established from goals through requirements to design at a level that supports impact analysis?
- [ ] Is the baseline explicitly defined with a realistic, calibrated change control process that is neither too light nor too heavy?
- [ ] Has the definition been genuinely validated with stakeholders through reviews, walkthroughs, or prototypes, not merely signed off?
- [ ] Is residual uncertainty acknowledged, with contingency and risk-reduction activities (prototypes, spikes) planned within the predictive structure?
- [ ] Has the specification itself been verified (consistency, completeness, testability) rather than assumed correct because it was written?
- [ ] Is the level of definition proportionate to the project's risk and size, rather than applied uniformly by template?
