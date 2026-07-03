---
name: invest_and_story_quality.md
description: Use when the agent is evaluating user story quality using INVEST criteria, assessing whether stories are independent negotiable valuable estimable small and testable, or reviewing and improving a backlog of stories to meet quality standards before planning.
---

# INVEST And Story Quality

INVEST is a checklist for evaluating whether a user story is well-formed: Independent, Negotiable, Valuable, Estimable, Small, and Testable. Each criterion catches a different failure mode that makes stories hard to plan, deliver, or verify. Using INVEST systematically surfaces stories that will cause problems before those problems consume capacity in planning or delivery. Done well, INVEST review produces a backlog of stories that can be confidently planned and delivered. Done poorly, it becomes a rote checklist applied without understanding what each criterion is really testing for, missing the failures it is meant to catch. Agents often apply INVEST superficially, checking boxes without examining whether the story actually exhibits each quality.

The harm this skill prevents is the backlog full of stories that will fail in planning or delivery. Stories that are not independent create sequencing tangles; stories that are not valuable waste capacity; stories that are not estimable or small cannot be planned; stories that are not testable cannot be confirmed done. These failures surface as planning friction, delivery surprises, and completeness disputes, all of which consume capacity better spent delivering value.

Use this skill before answering questions such as "are these stories ready for planning", "how do we improve our stories", "what does a good story look like", or "how do we apply INVEST". The goal is to prevent the agent from treating INVEST as a checkbox exercise rather than a diagnostic tool.

## Core Rules

### Test Independence By Whether Stories Can Be Delivered In Any Order

A story is independent when it can be delivered without requiring another specific story to be done first, and when delivering it does not force a particular order on unrelated stories. Independence matters because dependent stories create sequencing constraints that complicate planning and reduce flexibility. Test independence by asking whether the story could be delivered first, last, or in any position relative to the others; if delivery requires a specific predecessor, the dependency should be made explicit or the stories restructured.

Complete independence is often impossible, because real work has real dependencies. The goal is to minimize unnecessary dependencies and to make necessary ones explicit, so that planning can account for them. Stories with hidden dependencies, where the dependency is discovered only during delivery, cause the most disruption, because they stall work mid-flight.

### Test Negotiability By Whether The Story Prescribes Or Describes

A negotiable story describes what is needed and leaves room for the team to negotiate the how. A non-negotiable story prescribes the solution so tightly that there is nothing to negotiate; it is a specification disguised as a story. Negotiability matters because it preserves the team's ability to find better solutions and because it invites the conversation that builds shared understanding. Test negotiability by asking whether the team could propose a different approach to achieving the story's goal; if the story allows only one approach, it is over-prescribed.

Negotiability does not mean vagueness. A story can be specific about the user goal and the value while remaining open about the solution. The line is between specifying the outcome, which is appropriate, and specifying the implementation, which removes the team's contribution. Stories that read like technical specifications have lost their negotiability.

### Test Value By Whether The Story Delivers User Or Business Value Alone

A valuable story delivers value to a user or to the business if delivered on its own. A story that is necessary but not valuable, such as a technical task or a partial slice, fails this test. Value matters because stories that do not deliver value cannot be prioritized by value and do not produce anything worth shipping independently. Test value by asking whether shipping this story alone would make the product better for someone; if not, the story may be a task that should be folded into a valuable story, or a horizontal slice that should be reframed vertically.

Technical work and infrastructure are often necessary but not independently valuable. The response is not to discard them but to reframe them: either combine them with the user-facing work they enable into a single valuable story, or acknowledge them explicitly as technical enablers rather than user stories, with their own justification. Calling non-valuable work a user story does not make it valuable.

### Test Estimable And Small Together As Planning Readiness

A story is estimable when the team can assess its size with reasonable confidence, and small when that size is within a range that can be delivered incrementally. These two criteria are related: a story that is too large is also hard to estimate, because its size and complexity exceed the team's ability to assess. Test estimability by asking whether the team can confidently say how much effort the story requires; test smallness by asking whether the story can be delivered within a reasonable timeframe. Stories that fail either are usually too large or too unclear and should be split or clarified.

Estimability also depends on understanding. A story the team does not understand cannot be estimated, regardless of size. Before estimating, ensure the team has discussed the story and grasps what it involves. A story that remains unestimable after discussion carries unresolved uncertainty that should be addressed through investigation or splitting before it enters planning.

### Test Testable By Whether Done Can Be Verified Objectively

A story is testable when there are clear, objective conditions that define whether it is done. Testability matters because without it, done is subjective, leading to disputes and to work that is technically complete but does not serve the user. Test testability by asking whether someone could verify the story is complete without needing the original author's opinion; if verification requires subjective judgment, the story lacks testable acceptance criteria.

Testability is closely tied to acceptance criteria. A story with well-written acceptance criteria is testable; a story without them is not. Writing acceptance criteria is the mechanism that makes a story testable, and the criteria should be specific enough to verify objectively, covering the behaviors and outcomes that constitute done.

### Apply INVEST As Diagnosis, Not As Gatekeeping

INVEST is most valuable as a diagnostic tool that identifies specific problems in a story, each pointing to a specific remedy. A story that fails independence may need restructuring to remove dependencies; one that fails value may need combining with enabling work; one that fails small may need splitting; one that fails testable needs acceptance criteria. Use the criteria to identify what is wrong and what to do about it, rather than as a pass-fail gate that rejects stories without guidance. The goal is to improve stories, not to filter them.

This also means applying INVEST proportionally. Not every story needs to be perfect on every criterion; the criteria that matter most depend on the context and the risk. A story entering near-term planning should meet a higher bar than a story in a distant backlog. Apply the rigor where it pays off, in stories about to be planned and delivered.

## Common Traps

### Checklist Application Without Diagnosis

Checking INVEST boxes without understanding what each failure means. The trap is stories that pass the checklist but still fail in delivery.

### Hidden Dependencies Discovered Late

Stories that appear independent but stall during delivery. The trap is mid-flight disruption when the dependency surfaces.

### Over-Prescribed Stories That Remove Negotiability

Stories so specific they are specifications. The trap is a team that cannot contribute to the solution.

### Technical Tasks Labeled As Valuable Stories

Calling infrastructure work a user story. The trap is a backlog that cannot be prioritized by value.

### Large Stories Treated As Estimable

Estimating epics with false confidence. The trap is plans built on estimates that bear no relation to reality.

### Subjective Done Without Testable Criteria

Stories completed by opinion rather than verification. The trap is disputes and work that is technically done but does not serve the user.

## Self-Check

- [ ] Independence is tested by whether stories can be delivered in any order, with dependencies made explicit.
- [ ] Negotiability is tested by whether the story allows solution discussion or prescribes the implementation.
- [ ] Value is tested by whether the story delivers user or business value on its own.
- [ ] Estimable and small are tested together as planning readiness, with unestimable stories investigated or split.
- [ ] Testable is verified through objective acceptance criteria that define done without subjective judgment.
- [ ] INVEST is applied as diagnosis pointing to specific remedies, not as pass-fail gatekeeping.
- [ ] The rigor of application matches the context, with higher standards for near-term planning.
