---
name: acceptance_criteria_design.md
description: Use when the agent is designing acceptance criteria for user stories, deciding what conditions define done, choosing what behaviors and outcomes to specify, or ensuring criteria are clear enough to verify completion without ambiguity or over-specification.
---

# Acceptance Criteria Design

Acceptance criteria define the conditions under which a user story is considered complete and acceptable. They are the contract between the product manager and the team about what done means, and they are the basis for verification that the work satisfies the user's need. Done well, acceptance criteria are specific enough to verify objectively, comprehensive enough to cover the important behaviors, and focused enough to avoid dictating implementation. Done poorly, they are either too vague to verify, leaving done to opinion, or so prescriptive that they become a specification that removes the team's discretion. Agents often write criteria as a list of features to build rather than as conditions to verify, which makes them useless for confirming completion.

The harm this skill prevents is the ambiguity of done. When criteria are vague or absent, the team and the product manager disagree about whether work is complete, leading to disputes, rework, and work that is technically delivered but does not serve the user. When criteria over-specify, they constrain the team and produce work that meets the letter of the criteria while missing the spirit of the user's need. Good criteria resolve the ambiguity without removing discretion.

Use this skill before answering questions such as "how do we write acceptance criteria", "what should the criteria for this story be", "are these criteria good", or "how detailed should criteria be". The goal is to prevent the agent from producing criteria that are unverifiable, incomplete, or over-prescriptive.

## Core Rules

### Define Done From The User's Perspective, Not The Implementation

Acceptance criteria should describe the behaviors and outcomes that constitute done from the user's perspective, not the implementation steps the team will take. A criterion like "the user can submit the form and receive a confirmation within five seconds" describes a verifiable user-facing behavior. A criterion like "implement an asynchronous queue with retry logic" describes an implementation that the user never sees. Frame criteria around what the user experiences and can verify, leaving the implementation to the team.

This framing keeps criteria stable even if the implementation changes. If the team finds a better way to achieve the user-facing behavior, the criteria still hold, because they describe the outcome, not the mechanism. Criteria tied to implementation become invalid when the implementation changes, requiring rework of the criteria as well as the code.

### Make Each Criterion Independently Verifiable

Each acceptance criterion should be something that can be verified on its own, producing a clear yes or no answer about whether it is met. Vague criteria like "the form is easy to use" cannot be verified objectively, because ease of use is subjective and context-dependent. Verifiable criteria like "the form can be completed in under two minutes by a new user, as measured in usability testing" can be checked against evidence. Write each criterion so that someone could determine whether it is met without needing the author's interpretation.

Independent verifiability also means criteria do not depend on each other in confusing ways. Each criterion should stand as a distinct condition. If criteria overlap or depend on each other, consolidate or clarify them so that each is a clear, separate test of done.

### Cover The Important Behaviors And Outcomes Without Exhaustiveness

Acceptance criteria should cover the important behaviors and outcomes that define done, including the main flow, key alternative flows, and critical constraints. They should not attempt to be exhaustive, listing every possible interaction, because exhaustive criteria become a specification that is as large as the work itself and that constrains the team excessively. Aim for criteria that capture the essential definition of done, including what must work, what must not happen, and what constraints must hold, without enumerating every detail.

The right level of coverage balances completeness against precision. Too few criteria miss important behaviors, leaving gaps in the definition of done. Too many criteria become a specification that removes discretion and that the team must track against exhaustively. Include criteria for the main success path, for important alternatives, for constraints such as performance or security, and for what should not happen, such as error states or regressions.

### Separate Acceptance Criteria From Implementation Constraints

Acceptance criteria describe what done looks like; implementation constraints describe how the team must work or what they must consider. These are different and should be kept separate. A criterion like "the feature must not degrade page load time by more than 200 milliseconds" is an acceptance criterion, because it is a verifiable condition of done. A note like "consider using the existing caching layer" is an implementation suggestion, which may be useful context but is not a criterion. Mixing them confuses the definition of done with guidance about how to achieve it.

When implementation constraints are genuine requirements, such as compliance mandates or architectural standards, they can be expressed as criteria: "the implementation must comply with the data residency policy." But suggestions, preferences, and technical hints are not criteria and should be separated, so that the criteria remain a clean definition of done.

### Write Criteria Before Implementation, Not After

Acceptance criteria should be written before implementation begins, as part of refining the story for planning. Writing criteria first ensures that the team and the product manager share the same definition of done before work starts, preventing disputes later. It also forces clarity about what the story really requires, because writing criteria exposes gaps and ambiguities in the story itself. A story whose criteria are hard to write is often a story that is not yet clear enough to build.

Writing criteria after implementation, or retrofitting them to what was built, defeats their purpose. They become a description of what happened rather than a definition of what was required, providing no contract and no basis for verification against intent. Establish criteria early and refine them as understanding deepens, but always before the work they are meant to govern.

### Use Criteria To Drive Test Design

Well-written acceptance criteria translate directly into tests, because each criterion is a verifiable condition. This connection is valuable: criteria that cannot be tested are not really criteria, and tests derived from criteria ensure that verification covers the definition of done. When writing criteria, consider how each would be tested, and refine criteria that resist testing into forms that can be verified. The link between criteria and tests is what makes done objective rather than opinion.

This does not mean the product manager writes the tests, but that the criteria are written to be testable and that the team's tests map back to them. A criterion that the team cannot figure out how to test is a signal that the criterion needs clarification, just as a test that does not map to a criterion is a signal that the test may be testing the wrong thing.

## Common Traps

### Criteria As Feature Lists

Writing things to build rather than conditions to verify. The trap is criteria that do not define done and cannot be used for verification.

### Vague Criteria That Cannot Be Verified

Criteria relying on subjective judgment. The trap is done becoming a matter of opinion and dispute.

### Over-Specification Removing Team Discretion

Criteria so detailed they dictate implementation. The trap is a team that cannot find better solutions and criteria that break when implementation changes.

### Missing Important Behaviors

Criteria that cover the happy path but miss alternatives and constraints. The trap is work that is technically done but fails in important cases.

### Criteria Written After Implementation

Retrofitting done to what was built. The trap is criteria that describe rather than define, providing no contract.

### Untestable Criteria

Criteria that cannot be turned into tests. The trap is verification that does not actually cover the definition of done.

## Self-Check

- [ ] Criteria are framed from the user's perspective, describing behaviors and outcomes rather than implementation.
- [ ] Each criterion is independently verifiable, producing a clear yes or no without subjective interpretation.
- [ ] Criteria cover important behaviors and outcomes without attempting exhaustive specification.
- [ ] Acceptance criteria are kept separate from implementation suggestions and technical hints.
- [ ] Criteria are written before implementation and refined as understanding deepens.
- [ ] Each criterion can be translated into a test, and criteria that resist testing are clarified.
- [ ] Criteria include what must work, important alternatives, constraints, and what should not happen.
