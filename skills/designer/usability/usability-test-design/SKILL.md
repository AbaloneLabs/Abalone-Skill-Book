---
name: usability_test_design.md
description: Use when the agent is designing or running a usability test, writing task scenarios and test scripts, deciding between moderated and unmoderated testing, recruiting test participants, planning a usability study for a product or prototype, or interpreting usability test results without contaminating the data through leading tasks or facilitator bias.
---

# Usability Test Design

A usability test is only as trustworthy as the tasks it asks and the way it is facilitated. A test with leading tasks, a participant who is trying to please the facilitator, or a script that telegraphs the expected path will produce a clean-looking result that confirms what the team already believes, while hiding the real problems. The judgment problem is designing a test whose findings actually reflect the user's experience rather than the facilitator's expectations, and whose results the team can trust enough to act on. Agents tend to fail by writing tasks that name the buttons the user should click, by helping participants who struggle, and by treating a few sessions as proof that no problems exist.

Use this skill when designing or running any usability test: writing task scenarios, choosing moderated versus unmoderated formats, recruiting participants, building a test script, or interpreting sessions. The goal is a test whose data is not contaminated and whose conclusions match what the evidence can actually support.

## Core Rules

### Define What The Test Must Inform Before Writing Tasks

A usability test without a defined question becomes a tour of the interface. Before writing any task, state what the test is meant to decide: which task flows are being evaluated, which risks the design carries, and which decisions will change based on findings.

Define the test scope first:

- the specific flows or screens under evaluation;
- the user populations who must be represented;
- the risks and assumptions the test is meant to check;
- the decisions that will be made from the results.

Scoping prevents two failures: testing too broadly so that no flow gets enough attention, and testing without a purpose so the results are unfocused. A test tied to a decision gets acted on; a test run out of curiosity gets filed.

### Write Tasks That Reflect Real Goals, Not Button Hunts

The single most common source of contaminated usability data is the leading task. A task that tells the participant exactly what to do measures nothing about the interface's discoverability.

Write tasks that:

- describe the user's goal and situation, not the interface steps;
- avoid naming UI elements like buttons, menus, or labels the participant is meant to find;
- use the participant's language, not the product's internal terminology;
- give enough context that the goal is realistic, without hinting the path.

"Find a blue sweater under fifty dollars and add it to your cart" is a goal-based task; "click the filter button and select blue and under fifty" is a leading task that hands the participant the answer. If the task reveals whether the interface supports the goal, it is well written; if it only reveals whether the participant can follow instructions, it is not.

### Recruit Participants Who Represent The Real Users

A usability test run on the wrong participants produces findings that do not generalize. Match the sample to the population that will actually use the feature.

Recruit to match the real user base:

- the experience level of the target users, not all experts or all beginners;
- the relevant segments, roles, or personas the design serves;
- the devices and environments where the product is really used;
- accessibility needs that the design must support.

Convenience samples of colleagues, friends, or the design team are not representative; they know too much and behave unlike real users. State the recruitment criteria and note where the sample falls short of the target population, so findings are interpreted within their actual scope.

### Choose Moderated Or Unmoderated To Match The Question

Moderated and unmoderated tests answer different questions and carry different risks.

Match format to need:

- moderated tests allow probing, clarification, and observation of nuance, and suit discovery and complex flows, but risk facilitator influence;
- unmoderated tests scale to more participants and remove facilitator bias, but cannot probe and depend on tasks being unambiguous;
- remote tests reach real environments and devices but reduce control over conditions.

Choose the format based on what the test must learn, not on what is easiest to run. A complex flow that requires follow-up questions should not be forced into an unmoderated format, and a simple task that needs many participants should not be slowed by moderation.

### Facilitate Without Leading

In moderated tests, the facilitator is the largest source of bias. The instinct to help a struggling participant, to explain the design, or to finish their sentence destroys the data.

Facilitate neutrally:

- let participants struggle and recover on their own, because the struggle is the data;
- use neutral probes like "tell me what you are thinking" rather than "did you notice the button";
- do not defend or explain the design;
- keep your reactions neutral, since a smile or frown cues the participant.

The facilitator's job is to keep the participant thinking aloud and to capture behavior, not to guide them to success. A participant who completes the task because the facilitator helped has not validated the design.

### Decide Sample Size By What You Need To Detect

Usability testing is not about statistical proof; it is about discovering problems. Small samples reliably reveal severe problems that affect many users, because such problems show up quickly.

Match sample size to goal:

- five to seven participants per user group typically surfaces the major problems in a focused flow;
- more participants are needed to compare designs or to estimate problem frequency;
- fewer participants are acceptable for early, rough validation where any major problem is enough to act on.

Do not claim a design is problem-free because a few participants succeeded. Small samples reveal problems exist; they do not prove problems are absent.

### Separate Task Success From The User's Experience

Completing a task is not the same as the task being easy. A participant can succeed slowly, with confusion, with workarounds, or by accident.

Record more than pass or fail:

- whether the participant succeeded, and how;
- the time and number of errors;
- the participant's confidence and satisfaction;
- the workarounds and detours used.

A task that users complete but hate is a design problem, even if the success metric looks fine. Capture the experience, not just the outcome.

## Common Traps

### Leading Tasks That Name The Path

Tasks that tell participants which buttons to click measure nothing. Describe the goal, not the steps.

### Helping Struggling Participants

The facilitator's help turns a real failure into a false success. Let participants struggle; the struggle is the finding.

### Testing With The Wrong Participants

Colleagues and insiders know too much. Find participants who match the real user population.

### Claiming No Problems From A Small Sample

Five users can reveal problems exist; they cannot prove problems are absent. Do not overclaim.

### Equating Success With Good Experience

A completed task is not necessarily an easy or pleasant one. Capture effort, confusion, and satisfaction, not just pass or fail.

### Defending The Design During The Test

Explaining or justifying the design contaminates the data and cues the participant. Stay neutral.

### Testing Too Broadly With No Focus

A test that touches every screen shallowly finds nothing actionable. Focus on the flows that carry the real risks.

## Self-Check

- [ ] The test is tied to a defined decision or risk, with the flows and populations under evaluation stated.
- [ ] Tasks describe realistic user goals and avoid naming interface elements or hinting the path.
- [ ] Participants match the target user population in experience, segment, device, and accessibility needs.
- [ ] The moderated or unmoderated format matches what the test needs to learn.
- [ ] In moderated sessions, the facilitator let participants struggle, used neutral probes, and did not defend or explain the design.
- [ ] Sample size matches the goal: enough to reveal major problems, more for comparison or frequency claims.
- [ ] Findings capture the experience, effort, errors, and satisfaction, not just task success.
- [ ] No claim of "no problems" is made beyond what the sample can support.
- [ ] The test script was reviewed for leading language before sessions ran.
- [ ] The results are interpreted within their actual scope, with recruitment gaps noted.
