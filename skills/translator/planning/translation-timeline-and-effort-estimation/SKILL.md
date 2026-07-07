---
name: translation_timeline_and_effort_estimation.md
description: Use when the agent is estimating translation effort and word counts, computing linguist capacity and throughput, setting milestones and deadlines for translation projects, allocating buffer for review and rework, or managing dependencies between source finalization translation review and delivery in a multilingual schedule.
---

# Translation Timeline And Effort Estimation

Translation timelines fail most often not because the translation was slow but because the estimate was wrong. A realistic timeline depends on accurate effort estimation, which in turn depends on word count, content complexity, leverage from translation memory, linguist throughput, review capacity, and the rework cycles that quality assurance inevitably requires. An agent who estimates a translation timeline by dividing word count by a generic words-per-day figure will systematically underestimate, because that figure ignores leverage (which speeds work), complexity (which slows it), review (which adds time), and rework (which extends it). The result is a timeline that slips from the start, creating pressure that degrades quality and erodes trust with stakeholders.

Agents tend to miss the factors that inflate real effort: content complexity beyond raw word count, the leverage discount from translation memory, the difference between production throughput and review throughput, the rework loop between translator and reviewer, and the dependencies that gate the start of each stage. The harm is a committed deadline that is unachievable, a team that cuts corners to meet it, or a project that ships late because the estimate never accounted for the full cycle.

Use this skill when estimating translation effort, setting milestones and deadlines, computing linguist capacity, allocating buffer, or building a multilingual schedule. The goal is to produce estimates that are realistic, defensible, and account for the full production-to-delivery cycle, not just the raw translation step.

## Core Rules

### Estimate Effort From Word Count Adjusted For Complexity And Leverage

Effort estimation starts with word count but must be adjusted for two factors that dominate the real work: content complexity and translation memory leverage. Raw word count divided by a generic throughput rate produces a baseline that is wrong for most real content.

Complexity multiplies effort. A marketing text with creative language, idioms, and transcreation requirements takes longer per word than a straightforward technical procedure. Legal text with precise terminology and no tolerance for ambiguity takes longer than a casual blog post. Highly repetitive content with consistent phrasing takes less effort per new word. Classify content by complexity (simple, standard, complex, highly creative or highly technical) and apply a complexity factor to the baseline throughput. A complex text may take twice the per-word effort of a simple one.

Leverage reduces effort. Translation memory matches mean some segments need no translation (exact matches) or light editing (fuzzy matches). Estimate the leverage rate from historical data or a sample analysis: if 40 percent of the content is exact matches, those words require minimal effort. Apply leverage to reduce the effective new word count before computing effort. A project with strong leverage may take half the effort of a from-scratch translation of the same word count. Never estimate effort on gross word count without accounting for both complexity and leverage.

### Use Realistic Throughput Rates By Task Type

Throughput, the words a linguist can process per day, varies by task type. New translation throughput is typically lower than post-editing throughput, which is lower than review throughput. A common error is using a single throughput figure for all tasks. Use task-specific rates: translation, post-editing (full and light), review, and proofreading each have different throughput ranges.

Also account for individual variation. Senior specialized linguists may be faster on complex content but slower on volume tasks. Junior linguists may be faster on repetitive content. When multiple linguists work on a project, estimate at the team level using a blended rate, but recognize that splitting work across linguists introduces coordination overhead and consistency risk. Do not assume peak throughput for every hour of every day; real throughput includes breaks, context switching, query resolution, and administrative time. A sustainable daily rate is lower than a peak burst rate.

### Build The Full Cycle Into The Timeline, Not Just Production

A translation timeline that includes only the production (translation) stage will always be too short. The full cycle includes source finalization (the source must be complete and approved before translation begins), pre-production (file preparation, terminology setup, memory leveraging), production, review, quality assurance, rework, and final delivery. Each stage takes time and has dependencies on the prior stage.

Map the full cycle and assign durations to each stage based on effort estimates and capacity. Recognize dependencies: review cannot start until production is substantially complete, rework cannot start until review feedback is delivered, and delivery cannot happen until rework is verified. A common failure is running stages in parallel that actually depend on each other, such as starting review before production is far enough along, which produces rework on content that is still changing. Sequence stages to respect dependencies, and do not compress the cycle by eliminating stages that protect quality.

### Allocate Buffer For Rework And Unpredictable Issues

Rework is not an exception; it is a normal part of the translation cycle. Review identifies issues, the translator corrects them, and the corrections must be verified. Additional rework arises from source content changes mid-project, terminology disputes, client feedback, and technical issues with files or tools. A timeline with no buffer for rework will slip on almost every project.

Allocate buffer proportional to risk. High-risk or complex content warrants more buffer. Projects with evolving source content need more buffer. Teams new to the content or client need more buffer. A common practice is to add 15 to 30 percent buffer to the production estimate, with more for higher uncertainty. Place buffer strategically: some at the end of production before review, some between review and delivery. Do not present a timeline with zero buffer as realistic; it sets up failure and forces quality compromises when the inevitable issues arise.

### Account For Review Capacity Separately From Production Capacity

Review is a distinct capacity constraint. A team that can produce 10,000 words per day in translation may be able to review only 15,000 to 20,000 words per day, but review capacity depends on the number of available reviewers, who may be a different and smaller pool than translators. A project that has ample translation capacity but insufficient review capacity will bottleneck at review.

Estimate review capacity separately. Identify how many qualified reviewers are available, their review throughput, and whether they can work in parallel. If review capacity is the bottleneck, either expand the reviewer pool, reduce the scope requiring human review (using automated QA for mechanical checks), or extend the timeline. Do not assume that production throughput implies review throughput; they are separate constraints that must each be planned.

### Manage Source Finalization As A Hard Dependency

Translation cannot produce a final deliverable from a source that is still changing. Source finalization is a hard dependency: until the source is complete, approved, and frozen, the translation timeline is provisional. A common failure is starting translation on a draft source to save time, then reworking when the source changes, which often costs more than waiting.

Manage source finalization explicitly. Confirm that the source is final before committing to a translation deadline. If the source is not final, either delay the commitment or build in explicit rework cycles for source updates, with adjusted timelines. For continuous localization where source evolves constantly, establish a process for handling incremental updates rather than treating each change as a crisis. Document the source freeze date and make clear to stakeholders that timeline commitments are contingent on source stability.

### Sequence Multilingual Timelines By The Slowest Critical Path

In a multilingual project, the overall timeline is governed by the slowest language on the critical path, not by the average. If 18 languages can be delivered in two weeks but two scarce-resource languages take four weeks, the full rollout takes four weeks unless those two languages are phased separately.

Identify the critical path by estimating each language's full cycle and finding the longest. Determine whether the slowest languages gate the overall delivery or can be phased. If they gate delivery, either add resources to accelerate them or accept the longer timeline. Do not present a multilingual timeline based on the average or the fastest language; stakeholders need to know the real delivery date for the full set, which is the maximum, not the mean.

## Common Traps

### Estimating From Gross Word Count Without Complexity Or Leverage

Gross word count divided by a generic rate ignores that complex content takes longer and leveraged content takes less. Always adjust for complexity and apply leverage before computing effort.

### Using A Single Throughput Rate For All Tasks

Translation, post-editing, review, and proofreading have different throughput rates. Using one figure for all overestimates or underestimates depending on the task mix.

### Omitting Review And Rework From The Timeline

A timeline with only production is always too short. The full cycle includes review, QA, rework, and delivery, each with dependencies. Map the full cycle.

### Allocating Zero Buffer For Unpredictable Issues

Rework, source changes, and technical issues are normal, not exceptional. A zero-buffer timeline slips on nearly every project and forces quality compromises.

### Assuming Review Capacity Equals Production Capacity

Review is a separate constraint with a different pool and throughput. A project with ample translation capacity can bottleneck at review if reviewer capacity is not planned.

### Starting Translation On An Unfinalized Source

Translating a draft source to save time usually costs more in rework when the source changes. Confirm source finalization before committing to deadlines.

### Presenting Multilingual Timelines Based On The Average Language

The full multilingual delivery is governed by the slowest language on the critical path. Presenting the average or fastest language's timeline misleads stakeholders about the real delivery date.

### Ignoring Individual Variation In Linguist Throughput

Peak burst rates are not sustainable daily rates, and individual linguists vary. Estimate at a sustainable team level, not at an optimistic individual peak.

## Self-Check

- [ ] Has the effort estimate been computed from word count adjusted for content complexity (simple, standard, complex, highly creative or technical) rather than from raw word count alone?
- [ ] Has translation memory leverage been applied to reduce the effective new word count, with leverage rates estimated from historical data or sample analysis?
- [ ] Have task-specific throughput rates been used for translation, post-editing, review, and proofreading, rather than a single generic rate?
- [ ] Does the timeline include the full cycle: source finalization, pre-production, production, review, QA, rework, and delivery, with dependencies respected?
- [ ] Has buffer of 15 to 30 percent (or more for high uncertainty) been allocated for rework, source changes, and unpredictable issues?
- [ ] Has review capacity been estimated separately from production capacity, with the reviewer pool and throughput planned as a distinct constraint?
- [ ] Has source finalization been confirmed or explicitly flagged as a dependency, with timeline commitments contingent on source stability?
- [ ] For multilingual projects, has the critical path been identified as the slowest language, with the overall timeline based on the maximum rather than the average?
- [ ] Has the estimate been documented with word count, complexity classification, leverage rate, throughput assumptions, stage durations, buffer, and dependencies so it can be reviewed and defended?
