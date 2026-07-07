---
name: prototype_testing.md
description: Use when the agent is planning or running a test using a prototype, designing test tasks for prototypes, recruiting participants, deciding what can and cannot be learned from a prototype test, interpreting prototype test findings, or ensuring that prototype limitations do not contaminate the conclusions drawn from testing.
---

# Prototype Testing

Testing with a prototype is not the same as testing with a finished product, and treating them as equivalent produces conclusions that do not survive contact with reality. A prototype is always partial: some states are missing, some data is fake, some interactions are simulated, and some performance is unrepresentative. Each of these gaps can contaminate the findings if the test is not designed to account for them. The judgment problem is designing prototype tests that reveal real insights about the design without over-claiming what the prototype can show, writing tasks that test the design rather than the prototype's quirks, and interpreting results with honest awareness of where the prototype misled participants. Agents tend to fail by testing prototypes as if they were products, by drawing conclusions about performance or data behavior from stubbed prototypes, and by attributing prototype-caused confusion to design flaws.

Use this skill when planning or running any test that uses a prototype: writing tasks, recruiting, moderating, or interpreting findings. The goal is prototype tests that produce trustworthy, appropriately-scoped insights.

## Core Rules

### Define What The Prototype Can And Cannot Answer

Before designing the test, establish what the prototype is capable of revealing and what it cannot. A prototype's gaps define the boundaries of legitimate conclusions, and testing beyond those boundaries produces findings that do not transfer.

Clarify the answerable:

- structure, navigation, and comprehension: testable if the flows are wired;
- interaction feel and motion: testable if transitions are represented;
- visual and emotional response: testable only at sufficient fidelity;
- performance and real-data behavior: generally not testable unless the prototype is in code with real data.

State explicitly what the test can and cannot claim. A finding about "the flow is confusing" may be valid; a finding about "the system feels fast" is not, if the prototype fakes the response.

### Design Tasks That Test The Design, Not The Prototype

Test tasks should reveal how users interact with the intended design, not how they cope with the prototype's artifacts. Tasks that depend on missing states, fake data, or simulated interactions test the prototype's limitations, not the design.

Design clean tasks by:

- routing participants through flows the prototype fully supports;
- avoiding tasks that require states or data the prototype does not have;
- writing tasks in terms of goals, not steps, so participants navigate naturally;
- ensuring the task's success criteria are achievable within the wired flows.

A task that sends participants into an unwired dead end produces confusion about the prototype, not insight about the design. Keep tasks within the prototype's honest scope.

### Account For The Prototype's Fidelity In Interpretation

Participants react to what they see, and a prototype's fidelity shapes their reactions in ways that have nothing to do with the design. Low fidelity can make a sound concept feel unfinished; high fidelity can make an unresolved flow feel final.

Interpret with fidelity in mind:

- distinguish reactions to the concept from reactions to the polish;
- recognize that visual complaints at low fidelity may vanish at high fidelity;
- recognize that approval of high-fidelity polish may mask structural problems;
- weight behavioral findings, what participants did, over attitudinal ones, what they said, when fidelity is uncertain.

A participant who says "it looks cheap" at wireframe fidelity is reacting to the medium. A participant who cannot complete the task at any fidelity is revealing a design problem. Learn to tell them apart.

### Do Not Draw Conclusions Beyond The Prototype's Scope

The most common over-claim is generalizing prototype findings to the finished product without accounting for what the prototype lacked. Performance, real data volume, edge cases, and integration behavior are often absent from prototypes, and conclusions about them are invalid.

Avoid over-claiming by:

- not concluding the product will feel fast from a prototype that fakes responses;
- not concluding the design handles scale from a prototype with ten items;
- not concluding error handling works from a prototype with no error states;
- labeling findings with their scope and confidence level.

Honest scoping protects the team from acting on findings that will not hold. A well-scoped finding is more valuable than a confident over-claim.

### Recruit Participants Who Match The Real Audience

Prototype findings are only as valid as the participants are representative. Testing with convenient but non-representative participants produces findings that fit the wrong audience.

Recruit appropriately by:

- defining the user segments the design serves and recruiting within them;
- ensuring participants have the relevant context, expertise, or circumstances;
- avoiding over-reliance on internal participants, who know too much;
- including enough participants per segment to reach saturation, not statistical certainty.

A prototype tested with the wrong participants produces confidently wrong conclusions. Match the sample to the audience.

### Moderate To Observe Behavior, Not To Guide

In prototype testing, the moderator's temptation to help is especially strong, because the prototype's gaps invite confusion. Guiding participants to success validates nothing and contaminates the findings.

Moderate by observing:

- let participants struggle and recover, because recovery is part of what you are testing;
- avoid hinting at the next step or explaining the design;
- probe with open questions about what they expected and what they are trying to do;
- note where the prototype's limitations, not the design, caused confusion.

A participant who succeeds only because the moderator guided them has taught the team nothing. Let the design succeed or fail on its own.

### Separate Prototype Artifacts From Design Findings

Some confusion in prototype testing is caused by the prototype itself: a transition that did not work, data that was obviously fake, a state that was missing. These artifacts must be separated from genuine design findings.

Separate by:

- noting when confusion stemmed from a prototype limitation rather than the design;
- discounting findings attributable to missing states or fake data;
- flagging areas that need retesting once the prototype is more complete;
- being honest about which findings are robust and which are provisional.

Conflating prototype artifacts with design flaws leads to fixing the wrong things. Keep the two categories distinct in synthesis.

### Plan What Happens After The Test

A prototype test is not an endpoint; it is input to the next decision. Without a plan for acting on findings, the test produces a report that nothing changes.

Plan follow-through by:

- defining before the test what decisions the findings will inform;
- synthesizing findings into specific, actionable changes;
- deciding which findings require retesting at higher fidelity;
- closing the loop by confirming how findings changed the design.

A test whose findings are not acted upon was wasted effort. Design the test to produce decisions, not just observations.

## Common Traps

### Testing The Prototype As If It Were The Product

Prototypes are partial, and their gaps contaminate findings. Account for what is missing.

### Tasks That Hit Unwired States

Tasks that require missing states test the prototype's limitations, not the design. Keep tasks within scope.

### Over-Claiming Beyond The Prototype's Scope

Conclusions about performance, scale, or edge cases are invalid from stubbed prototypes. Scope findings honestly.

### Confusing Reactions To Fidelity With Reactions To Design

Low fidelity invites polish complaints; high fidelity masks structural flaws. Interpret with fidelity in mind.

### Non-Representative Participants

Convenient participants produce confidently wrong conclusions. Recruit to match the real audience.

### Moderator Guidance That Validates Nothing

Hinting and explaining help participants succeed without revealing whether the design works. Observe, do not guide.

### Conflating Prototype Artifacts With Design Flaws

Confusion caused by missing states or fake data is not a design problem. Separate the categories.

### Findings That Lead To No Action

A test that produces a report but no design change was wasted. Plan the decisions the test will inform.

## Self-Check

- [ ] What the prototype can and cannot answer is defined before the test, given its fidelity and gaps.
- [ ] Test tasks route participants through fully wired flows and avoid missing states and fake data.
- [ ] Findings are interpreted with awareness of fidelity, distinguishing concept reactions from polish reactions.
- [ ] No conclusions are drawn about performance, scale, or behavior the prototype did not actually represent.
- [ ] Participants match the real audience segments, with enough per segment for saturation.
- [ ] Moderation observes behavior without guiding participants to success.
- [ ] Confusion caused by prototype artifacts is separated from genuine design findings.
- [ ] Findings are labeled with their scope and confidence level.
- [ ] Areas needing retesting at higher fidelity are flagged.
- [ ] The test is tied to specific decisions, and findings are acted upon, not just reported.
