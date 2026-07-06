---
name: user_story_writing.md
description: Use when the agent is writing user stories, deciding what level of detail a story should contain, framing work from the user's perspective, or ensuring stories communicate intent and context to the team rather than serving as task descriptions.
---

# User Story Writing

A user story is a communication tool, not a specification. Its purpose is to express who needs what and why, in a way that gives the team enough context to build the right thing and make local decisions. Done well, a story carries the user's perspective, the value sought, and the conversation that should happen around the work. Done poorly, it is either a task description stripped of user perspective or a vague aspiration that conveys no actionable information. Agents often write stories as feature descriptions or technical tasks, losing the user-centered framing that makes stories useful, or they write them so vaguely that the team cannot build from them.

The harm this skill prevents is the team building from instructions rather than understanding. When stories are task lists, the team executes without context, making poor local decisions because they do not know what the user needs or why. When stories are too vague, the team fills the gaps with assumptions that are often wrong, producing work that does not serve the user. A good story carries enough context that the team can think, not just execute.

Use this skill before answering questions such as "how do we write user stories", "is this story well-written", "what should a story contain", or "how detailed should stories be". The goal is to prevent the agent from producing stories that are either task descriptions or vague aspirations.

## Core Rules

### Frame The Story From The User's Perspective

A user story expresses what a specific user or role wants to accomplish and why, not what the system should do. The framing matters because it keeps the team oriented toward the user's goal, which enables better decisions about how to serve it. "As a manager, I want to approve expense reports from my phone, so that I can keep approvals moving when I am traveling" centers the user, the goal, and the value. "Implement mobile expense approval" centers the system and strips away the why, leaving the team to guess at context.

The role in the story should be a real user type, not a generic "user" or a technical role. The goal should be something the user actually wants to accomplish, not a system behavior. The value, the so-that clause, should explain why this matters to the user, which is what makes the story actionable context rather than a feature label.

### Include Enough Context To Enable Good Local Decisions

A story should carry the context the team needs to make decisions during implementation: who the user is, what they are trying to accomplish, why it matters, what the current experience is, and any constraints or background that shape the solution. The level of context depends on the team and the work: a team deeply familiar with the area needs less; a team new to it needs more. The test is whether the team can make reasonable decisions about how to serve the user without needing to return for clarification on basics.

Context is not the same as prescription. A story that prescribes the exact solution removes the team's ability to find a better one. The right balance provides the what and why richly and leaves the how open, so that the team can apply its expertise to the solution while staying anchored to the user's goal.

### Keep Stories Small Enough To Be Understandable And Deliverable

A story should be small enough that a team can understand it wholly, estimate it, and deliver it within a reasonable timeframe. Stories that are too large, often called epics, contain multiple user goals, multiple possible solutions, and too much complexity to grasp at once. They are hard to estimate, hard to split, and hard to deliver incrementally. Break large stories into smaller ones, each representing a coherent piece of user value that can be delivered and validated independently.

The right size balances coherence against deliverability. A story should be coherent, representing a complete piece of user value, not an arbitrary slice. But it should also be deliverable, small enough to complete and validate without excessive risk. Stories that are coherent but huge should be split along the seams of user value; stories that are small but incoherent, representing fragments no user would value alone, should be reconsidered or combined.

### Write Stories That Invite Conversation, Not Replace It

A user story is an invitation to conversation, not a complete specification. The card, the written story, is a placeholder for the conversation that happens between the product manager, the team, and other stakeholders about what is really needed. The story should be good enough to start that conversation, and the conversation should refine and complete the understanding. Treating the written story as the complete specification, with no conversation, produces work built on whatever the story happened to capture, including its gaps.

Build conversation into the process: a story is discussed, refined, and confirmed through dialogue before and during implementation. The written artifact evolves as the conversation deepens understanding. A story that is written and then handed off without conversation has lost most of its value, because the shared understanding that conversation creates is what the story is really for.

### Make Stories Testable Through Acceptance Criteria

A story is only complete when it is testable, meaning there are clear conditions that define whether the story is done. These acceptance criteria specify the behaviors and outcomes that must hold for the story to be considered delivered. Without testable criteria, done is a matter of opinion, which leads to disputes, rework, and work that is technically complete but does not serve the user. Write acceptance criteria as part of the story, before implementation, so that the team and the product manager share the same definition of done.

Acceptance criteria should describe outcomes and behaviors from the user's perspective, not implementation details. They should be specific enough to verify but not so prescriptive that they dictate the solution. The criteria are what make the story verifiable; without them, the story is an aspiration that cannot be confirmed.

### Avoid Common Anti-Patterns That Drain Stories Of Value

Several patterns drain stories of their value. Stories written as technical tasks, "implement the API endpoint," strip away user perspective. Stories with a generic "as a user" role lose the specificity that enables decisions. Stories without a so-that clause omit the value that orients the team. Stories that prescribe the solution remove the team's ability to find a better one. Stories that are too large to deliver incrementally create risk and delay. Recognize these patterns and rewrite the stories to restore user perspective, value, appropriate size, and solution openness.

The goal is not formulaic compliance with a template but the underlying qualities the template is meant to ensure: user perspective, clear value, conversational richness, appropriate size, and testability. A story that achieves these through a non-standard format is better than one that follows the format without the qualities.

## Common Traps

### Stories As Technical Tasks

Writing implementation steps instead of user goals. The trap is a team that executes without understanding what the user needs or why.

### Generic User Roles

Using "as a user" instead of a specific role. The trap is lost specificity that prevents the team from making user-appropriate decisions.

### Missing The Value Clause

Omitting the so-that that explains why the work matters. The trap is a team that knows what to build but not why, leading to poor tradeoffs.

### Over-Prescriptive Stories

Dictating the solution in the story. The trap is a team that cannot find a better approach because the story has pre-decided the how.

### Stories Too Large To Deliver

Epics masquerading as stories. The trap is work that is hard to estimate, split, and deliver incrementally.

### Stories Without Acceptance Criteria

Aspirations with no definition of done. The trap is disputes about completeness and work that is technically done but does not serve the user.

## Self-Check

- [ ] Each story is framed from a specific user role's perspective, with the goal and value stated.
- [ ] Stories contain enough context for the team to make good local decisions without returning for basics.
- [ ] Stories are small enough to be understood wholly, estimated, and delivered incrementally.
- [ ] Stories invite and are refined through conversation, rather than treated as complete specifications.
- [ ] Each story has acceptance criteria that define done from the user's perspective.
- [ ] Stories avoid anti-patterns: technical tasks, generic roles, missing value, over-prescription, excessive size.
- [ ] The story preserves solution openness, providing what and why while leaving how to the team.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
