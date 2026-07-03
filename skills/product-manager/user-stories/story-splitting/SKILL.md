---
name: story_splitting.md
description: Use when the agent is splitting large user stories or epics into smaller deliverable stories, choosing how to divide work along value seams, ensuring splits produce vertical slices that each deliver value, or breaking down work that is too large to estimate or deliver incrementally.
---

# Story Splitting

Story splitting is the skill of breaking a large story or epic into smaller stories that can each be delivered and validated independently. It is one of the most practically important product skills, because large stories are hard to estimate, hard to deliver, and hard to learn from, while well-split stories enable incremental delivery, faster feedback, and more accurate planning. Done well, splitting produces stories that each deliver a coherent slice of user value and that together accumulate to the epic's full outcome. Done poorly, splitting produces either horizontal layers that deliver no value individually or arbitrary fragments that no user would recognize as valuable. Agents often split along architectural lines, because that is easy, producing horizontal slices that defeat the purpose of incremental delivery.

The harm this skill prevents is the large story that delays and surprises. A story too large to deliver incrementally accumulates risk, provides no feedback until the end, and often misses its estimate dramatically because its complexity was hidden. Splitting is what converts these risky lumps into manageable, learnable increments. But splitting badly, along horizontal lines, merely produces many lumps that still deliver no value until all are done.

Use this skill before answering questions such as "how do we split this epic", "this story is too big", "how do we break this down", or "what are good ways to split stories". The goal is to prevent the agent from splitting along architectural convenience rather than along value seams.

## Core Rules

### Split Along Seams Of User Value, Not Architectural Layers

The fundamental principle of good splitting is to divide along seams of user value, so that each resulting story delivers a coherent piece of value that a user could experience and benefit from. The alternative, splitting along architectural layers, produces horizontal slices: the data model, then the business logic, then the user interface, none of which delivers value alone. Vertical splits, by contrast, deliver a thin end-to-end piece of functionality that provides value, then expand it in subsequent stories. Always prefer vertical splits that preserve user value in each story.

The test of a good split is whether each resulting story delivers value on its own. If a story is necessary but not valuable alone, it is a horizontal layer and the split should be reconsidered. Reframe horizontal layers as components of vertical stories, combining them with the user-facing work they enable, so that each story stands alone as valuable.

### Use Workflow Steps As Natural Split Points

When a story encompasses a multi-step user workflow, the steps often provide natural split points. Each step that delivers value to the user can become its own story, with later steps building on earlier ones. For example, a story about a complete expense management workflow might split into capturing receipts, submitting reports, approving reports, and reimbursing, each of which delivers value and can be delivered and validated sequentially.

Workflow splitting works when each step delivers standalone value. If the steps only make sense together, with none valuable alone, the workflow is a single story and should not be split this way. Look for steps where a user could stop and still have benefited; those are the split points. Steps that are meaningless without the others should remain together.

### Split By Data Type Or Variety When The Story Spans Many Cases

When a story handles many types of data or many varieties of a case, splitting by type can produce valuable increments. A story about importing data might split into importing one data format first, then adding others, with each format delivering value to the users who have that data. A story about supporting many entity types might split into supporting the most important type first, then expanding. Each split delivers value to a subset of users and validates the approach before generalizing.

Data-type splitting works when each type delivers value independently. It fails when all types are required for any value, in which case the split produces fragments that must all be done before anything is useful. Assess whether each type stands alone before splitting this way.

### Split By Complexity, Delivering Simple Cases First

When a story's complexity comes from handling many edge cases or variations, split by delivering the simple, common case first and adding complexity in subsequent stories. The first story handles the happy path that most users follow; later stories add exception handling, edge cases, and rare variations. This delivers value to the majority of users quickly and defers the complexity that serves fewer users, which is often the right priority.

Complexity splitting requires honesty about what the simple case is. The temptation is to define the simple case so narrowly that it serves almost no one, making the first story technically valuable but practically useless. Define the simple case as the one that serves the majority of real users, even if it leaves known gaps for edge cases. The gaps are addressed in subsequent stories, not ignored.

### Split By Interface Or Channel When Value Differs By Surface

When a story spans multiple interfaces or channels, splitting by surface can produce valuable increments if each surface delivers value to its users. A story about making a feature available on web and mobile might split into web first, then mobile, if web users gain value before mobile is done. This works when the surfaces serve distinct user populations or distinct use cases, so that delivering one surface creates real value for its users.

Interface splitting fails when the surfaces must be delivered together to provide coherent value, such as when users expect a consistent experience across surfaces. In those cases, splitting by surface produces inconsistency that harms the experience. Assess whether each surface delivers coherent value alone before splitting this way.

### Ensure Each Split Story Is Itself Well-Formed

Splitting is not complete until each resulting story is itself well-formed: valuable, estimable, small, testable, and independent within the constraints of the epic. A split that produces stories failing these criteria has not solved the problem; it has moved it. Review each split story against quality criteria, and split further or recombine as needed until each story can stand alone in planning and delivery.

This includes ensuring that the split stories together still achieve the epic's outcome. A split that produces valuable individual stories but that collectively misses the epic's goal has fragmented the work without serving its purpose. Verify that the set of split stories, delivered in sequence, accomplishes what the epic was meant to accomplish.

## Common Traps

### Horizontal Splits Along Architectural Layers

Splitting into data model, logic, and interface. The trap is slices that deliver no value until all are complete.

### Splits That Produce Valueless Fragments

Stories too small or partial to deliver value alone. The trap is many stories that must all be done before anything is useful.

### Workflow Splits Where Steps Lack Standalone Value

Splitting a workflow whose steps only make sense together. The trap is fragments that cannot be delivered or validated independently.

### Over-Narrow Simple Cases

Defining the first slice so narrowly it serves no one. The trap is a technically valuable first story that delivers no practical value.

### Interface Splits That Create Inconsistency

Splitting surfaces that users expect to be consistent. The trap is an experience that is coherent on neither surface.

### Split Stories That Are Not Themselves Well-Formed

Splits that produce stories failing quality criteria. The trap is moving the large-story problem rather than solving it.

## Self-Check

- [ ] Splits follow seams of user value, producing vertical slices that each deliver standalone value.
- [ ] Workflow splits occur only at steps that deliver value independently.
- [ ] Data-type and variety splits are used only when each type delivers value to its users.
- [ ] Complexity splitting delivers the simple majority case first, with edge cases deferred, not ignored.
- [ ] Interface splits are used only when each surface delivers coherent value alone.
- [ ] Each split story is itself well-formed: valuable, estimable, small, testable, and appropriately independent.
- [ ] The set of split stories collectively achieves the epic's original outcome.
