---
name: prototype_validation.md
description: Use when the agent is validating a prototype with users or stakeholders, planning what to test in a prototype before building, deciding what feedback to trust and what to discount, running prototype reviews or usability checks on unfinished designs, or avoiding the trap of acting on feedback that reflects the prototype's gaps rather than the design's merits.
---

# Prototype Validation

Validating a prototype is not the same as validating a product, and confusing the two produces decisions built on misread feedback. A prototype is always incomplete, and users react to what is missing, stubbed, or fake as much as to what is designed. Feedback that a feature "does not work" may mean the design is flawed, or it may mean the prototype did not implement it. Feedback that a flow "is confusing" may reflect a real structural problem, or it may reflect a placeholder the user could not interpret. The judgment problem is designing validation that separates real design problems from prototype artifacts, and deciding which feedback to act on and which to discount. Agents tend to fail by taking all feedback at face value, by validating prototypes that are too rough to yield trustworthy results, and by treating positive feedback on a polished prototype as proof the design is right.

Use this skill when validating a prototype with users or stakeholders, planning a prototype review, or interpreting feedback on an unfinished design. The goal is validation that reveals whether the underlying design works, not whether the prototype happened to be convincing.

## Core Rules

### Decide What The Prototype Can And Cannot Validly Test

A prototype can only be validated for what it actually represents. Before running validation, state explicitly what the prototype is fit to test and what it is not.

Define the validation scope:

- which flows and interactions are fully represented and can be tested for real;
- which are partially represented and can be tested only for direction, not detail;
- which are stubbed or absent and cannot be tested at all;
- which dimensions, such as performance or real data, the prototype cannot represent.

Feedback on something the prototype does not actually contain is feedback on a gap, not on the design. State the scope so that findings are interpreted within what the prototype can validly show.

### Match Validation Method To Prototype Maturity

The right validation depends on how finished the prototype is, and over-validating an early prototype produces noise.

Match method to maturity:

- concept sketches are validated through critique and directional feedback, not task-based testing;
- low-fidelity flows are validated for structure and comprehension, not visual or interaction feel;
- high-fidelity prototypes are validated for realistic task behavior and subjective experience;
- code prototypes are validated for performance, real data, and edge cases.

Do not run a full usability test on a wireframe and expect findings about the visual experience, and do not run a concept critique on a near-final prototype and call it validation. Match the rigor to the maturity.

### Separate Design Problems From Prototype Artifacts

The core discipline of prototype validation is telling apart feedback about the design from feedback about the medium. Users cannot always tell what is intentional and what is a placeholder, and their reactions mix the two.

For each piece of feedback, ask:

- does this reflect the design, or the fact that the prototype is incomplete;
- would this problem exist in the real product, or only in this stubbed version;
- is the user reacting to a missing state, hardcoded data, or a non-functional element;
- is the confusion about the concept, or about the prototype's roughness.

Record which findings are design problems and which are prototype artifacts. Acting on artifact feedback leads to fixing the wrong thing; ignoring real design problems because "it is just the prototype" lets genuine issues ship.

### Brief Users On What They Are Looking At

Users approach a prototype expecting a finished product, and without framing they judge it by that standard. Set expectations so their feedback targets the right layer.

Frame the session by:

- explaining that this is an early prototype and some things will not work;
- telling them which parts are real and which are placeholders, where it helps;
- emphasizing that you are testing the design, not them;
- encouraging them to flag anything confusing, whether it is their fault or the prototype's.

Framing reduces the social pressure to be polite about gaps and helps users give honest feedback about the design rather than apologizing for "not getting" a stubbed feature.

### Avoid The Polish Trap In Both Directions

A polished prototype biases users toward positive feedback, because it looks finished and impressive. A rough prototype biases users toward negative feedback, because the roughness reads as low quality. Both biases distort the findings.

Guard against polish bias:

- high polish can make users forgive real problems, so probe specifically for difficulties rather than accepting "it looks great";
- low roughness can make users reject sound ideas, so distinguish "the concept is bad" from "this looks unfinished";
- match polish to the question, so the medium does not predetermine the verdict.

The goal is feedback on the design, not on the craft of the prototype. Recognize which way the polish is biasing the room and compensate.

### Validate The Riskiest Assumptions First

Prototype time is limited, and validating everything shallowly validates nothing well. Prioritize the assumptions that carry the most risk if wrong.

Prioritize validation by risk:

- the core value proposition: does this solve a real problem the user has;
- the riskiest flows: where failure would be most costly;
- the assumptions most likely to be wrong, where the team is least certain;
- the decisions that are hardest to reverse after build.

Validating the safe, familiar parts first feels productive but leaves the dangerous assumptions unchecked. Test what would hurt most to get wrong.

### Triangulate Rather Than Trust A Single Round

A single validation round, especially with few users, is suggestive at best. Triangulate across rounds, methods, and populations before acting on consequential decisions.

Triangulate by:

- running more than one round as the design evolves;
- combining task-based testing with interview and analytics where possible;
- checking that findings hold across different user segments;
- looking for convergence, where multiple sources point to the same conclusion.

A finding that appears once may be noise or a prototype artifact; a finding that recurs across rounds and methods is far more trustworthy.

## Common Traps

### Taking All Feedback At Face Value

Feedback on a prototype mixes design reactions with reactions to gaps. Separate the two before acting.

### Validating A Prototype That Is Too Rough

A prototype below the maturity for the question yields noise, not signal. Match method to maturity.

### Confusing Polish With Proof

A polished prototype that gets positive feedback has not proven the design is right. Probe for real difficulties beneath the surface approval.

### Ignoring Real Problems As "Just The Prototype"

Dismissing every finding as a prototype artifact lets genuine design problems ship. Decide which findings are real.

### Validating The Safe Parts First

Testing familiar territory feels productive but leaves risky assumptions unchecked. Validate what would hurt most to get wrong.

### Acting On A Single Round

One round with few users is suggestive, not conclusive. Triangulate before consequential decisions.

### Not Framing What Is Real

Users who expect a finished product judge the prototype by that standard. Brief them on what they are looking at.

## Self-Check

- [ ] The validation scope states what the prototype can and cannot validly test.
- [ ] The validation method matches the maturity of the prototype, from critique to full usability testing.
- [ ] Each finding is classified as a design problem or a prototype artifact before it is acted on.
- [ ] Users were briefed that the prototype is unfinished and that some elements are placeholders.
- [ ] Polish bias was recognized and compensated for, in whichever direction it leans.
- [ ] The riskiest assumptions and flows were validated first, not the safe familiar parts.
- [ ] Findings were triangulated across rounds, methods, or populations before consequential decisions.
- [ ] No single round of feedback was treated as conclusive proof.
- [ ] Feedback on missing or stubbed features was not mistaken for feedback on the design.
- [ ] The validation reveals whether the underlying design works, not merely whether the prototype convinced.
