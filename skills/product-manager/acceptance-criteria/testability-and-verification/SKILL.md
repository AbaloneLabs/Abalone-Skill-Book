---
name: testability_and_verification.md
description: Use when the agent is ensuring acceptance criteria and requirements are testable, translating criteria into verifiable conditions, deciding what evidence confirms completion, or addressing requirements that resist objective verification and need reframing to become checkable.
---

# Testability And Verification

Testability is the property that a requirement or acceptance criterion can be verified through objective evidence. A testable requirement is one where, given the built system, someone can determine whether the requirement is met by examining the system rather than by asking the author. Testability is what makes done meaningful: without it, completion is a matter of opinion, and disputes arise about whether work satisfies the requirement. Done well, testable requirements translate cleanly into tests, evidence, and verification, providing confidence that the work meets the need. Done poorly, requirements are aspirational statements that no one can confirm, leaving the team and stakeholders to argue about whether they have been satisfied. Agents often write requirements that sound good but that cannot be verified, because they do not consider how the requirement would be checked.

The harm this skill prevents is the untestable requirement that cannot be confirmed. When a requirement says "the system should be fast" or "the experience should be intuitive," there is no way to verify it, because the terms are subjective and undefined. The team builds something, declares it done, and the product manager disagrees, but neither can point to evidence, because the requirement provided no basis for evidence. Testable requirements resolve this by specifying what would count as meeting them.

Use this skill before answering questions such as "how do we make this requirement testable", "is this criterion verifiable", "how will we know when this is done", or "what evidence confirms completion". The goal is to prevent the agent from producing requirements that sound clear but that cannot be objectively verified.

## Core Rules

### Define What Evidence Would Confirm The Requirement Is Met

For each requirement or criterion, define what evidence would confirm that it is met. This is the core of testability: a requirement is testable if you can specify the evidence that would verify it. Evidence might be a measurement, a demonstration, an inspection, or a test result, but it must be something objective that an observer can examine. "The page loads in under two seconds" is confirmed by a load time measurement. "The user can complete the task" is confirmed by a demonstration. Defining the evidence forces clarity about what the requirement really means and exposes requirements that have no possible evidence.

If you cannot define what evidence would confirm a requirement, the requirement is not testable as written. This is a signal to reframe it: make subjective terms specific, add measurable thresholds, or break a vague requirement into specific, checkable conditions. The effort to define evidence is the effort to make the requirement real, because a requirement with no possible evidence is an aspiration, not a requirement.

### Replace Subjective Terms With Specific, Measurable Conditions

Subjective terms are the enemy of testability. Words like "fast," "intuitive," "robust," "user-friendly," and "high quality" sound clear but mean different things to different people, making them impossible to verify. Replace them with specific, measurable conditions that define what the term means in this context. "Fast" becomes "responds within 500 milliseconds for 95 percent of requests." "Intuitive" becomes "a new user can complete the core task within five minutes without assistance." The specific condition is testable; the subjective term is not.

This does not mean every requirement needs a number, but that every requirement needs a definition specific enough to check. Some requirements are verified by demonstration or inspection rather than measurement, but even then, the condition must be clear: "the error message identifies the specific field that failed" is verifiable by inspection, even without a number. The goal is specificity, however it is achieved.

### Distinguish Between Different Types Of Verification

Requirements can be verified through different types of evidence, and choosing the right type matters. Demonstration verifies by showing the system does the thing, appropriate for functional requirements. Measurement verifies by quantifying a property, appropriate for performance and quality requirements. Inspection verifies by examining the system or its artifacts, appropriate for structural and compliance requirements. Analysis verifies by reasoning from principles or models, appropriate for requirements that cannot be directly observed. Match the verification type to the requirement, because using the wrong type produces evidence that does not actually confirm the requirement.

For example, verifying a security requirement by demonstration, showing that the system blocks an attack, is weaker than verifying by analysis, showing that the design prevents the class of attack. Understanding which type of evidence each requirement needs ensures that verification is meaningful rather than superficial.

### Address Requirements That Resist Direct Verification

Some requirements resist direct verification because they concern qualities that are hard to observe or measure directly. User satisfaction, long-term reliability, and emergent system properties are examples. For these, testability comes from defining proxy measures or leading indicators that correlate with the requirement and that can be checked. User satisfaction might be proxied by survey scores or task completion rates. Reliability might be proxied by mean time between failures in testing. The proxy is not the requirement itself, but it is evidence about it.

Be honest about the gap between the proxy and the requirement. A proxy that correlates weakly provides weak evidence, and a requirement resting on weak proxies is less testable than one resting on strong ones. Document the proxies used and their limitations, so that stakeholders understand what the evidence does and does not show. Overstating the testability of a hard-to-verify requirement creates false confidence.

### Make Verification Feasible Within The Project's Constraints

A requirement is only testable in practice if the verification is feasible within the project's constraints of time, cost, and capability. A requirement that can only be verified by a million-user load test is technically testable but practically untestable for most teams. Consider feasibility when writing requirements: can the team actually perform the verification, with the tools, time, and expertise available? If not, either provide the resources for verification or reframe the requirement to something verifiable within constraints.

This is not an argument for lowering standards, but for honesty. A requirement that the team cannot verify is a requirement that will not be verified, leaving done undefined. Better to write a requirement that is verifiable in practice, even if less ambitious, than one that is theoretically testable but never actually checked. Feasibility is part of testability.

### Connect Testability To The Definition Of Done

Testability is closely connected to the definition of done. A story is done when its acceptance criteria are met, and the criteria are met when they are verified. If the criteria are not testable, done cannot be determined, and the story cannot be confidently completed. Ensure that the definition of done, at both the story and the increment level, rests on testable criteria, so that completion is a matter of evidence rather than assertion. This connection is what makes testability practically important, not just theoretically desirable.

When a team struggles to determine done, the root cause is often untestable criteria. Rather than arguing about completion, revisit the criteria and make them testable, so that future disputes can be resolved by evidence. This turns done from a source of conflict into a shared, verifiable standard.

## Common Traps

### Subjective Terms Passed Off As Requirements

"Fast," "intuitive," "robust" without definition. The trap is requirements that sound clear but cannot be verified by anyone.

### No Evidence Defined For Verification

Requirements without a specified check. The trap is completion determined by opinion rather than evidence.

### Wrong Verification Type For The Requirement

Using demonstration where analysis is needed. The trap is evidence that does not actually confirm the requirement.

### Overstating Testability Of Hard Requirements

Claiming proxies prove the requirement. The trap is false confidence in verification that only weakly relates to the requirement.

### Infeasible Verification Ignored

Requirements theoretically testable but practically impossible. The trap is requirements that are never actually verified.

### Done Determined By Assertion

Completing stories without testable criteria. The trap is disputes about completion that evidence cannot resolve.

## Self-Check

- [ ] Each requirement has defined evidence that would confirm it is met.
- [ ] Subjective terms are replaced with specific, measurable or inspectable conditions.
- [ ] The verification type matches the requirement, providing meaningful evidence.
- [ ] Requirements that resist direct verification use defined proxies with documented limitations.
- [ ] Verification is feasible within the project's time, cost, and capability constraints.
- [ ] The definition of done rests on testable criteria, making completion a matter of evidence.
- [ ] Requirements that cannot be made testable are reframed or acknowledged as aspirational rather than treated as checkable.
- Are assumptions, uncertainties, and confidence levels stated explicitly rather than buried in a confident-sounding conclusion?
