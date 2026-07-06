---
name: post_editing_level_selection.md
description: Use when the agent is deciding between raw machine translation use, light post-editing, full post-editing, and human retranslation, scoping a post-editing job, setting the expected quality level for a content type, writing post-editing instructions, or negotiating the post-editing effort and quality tradeoff with requesters and reviewers.
---

# Post-Editing Level Selection

Post-editing level selection is the decision made before any editing begins that defines what "good enough" means for a given piece of machine-translated content. It is not a quality judgment about the MT output; it is a contract about how much human effort will be spent and what defects will be fixed. The two recognized levels, light post-editing and full post-editing, are genuinely different jobs with different rules, different costs, and different acceptable outcomes. Choosing the wrong level is expensive in both directions: choosing full post-editing when light would have sufficed burns budget and time on content that only needed to be understandable; choosing light post-editing when the content must read as human-quality publishes text full of stylistic and terminological defects. Agents miss this decision because they treat post-editing as a single activity and then drift between levels segment by segment, producing documents that are over-edited in some places and under-edited in others.

The harm this skill prevents is threefold: wasted effort from over-editing low-stakes content, published defects from under-editing high-stakes content, and the inconsistency that results when no level was chosen and each editor improvised. The agent's freedom here is real but bounded by the content's purpose, the requester's quality expectation, and the downstream consequences of error.

## Core Rules

### Decide The Level Before Editing, Based On Purpose And Consequence

The level must be chosen before editing starts and must follow from what the text is for, not from how good the MT happens to look. The right question is not "is this MT good?" but "what is the cost of a residual error in this content, and who bears it?" Walk through the content's lifecycle: who reads it, what decision do they make from it, what happens if a sentence is slightly wrong, and will it be revised later. Light post-editing fits content where the reader needs to extract information and tolerates awkward phrasing: internal knowledge bases, support tickets, technical logs, gisting of incoming correspondence, and draft material that will be rewritten anyway. Full post-editing fits content that represents the organization's voice or carries liability: customer-facing documentation, marketing, legal disclosures, safety instructions, and anything published under a brand. When the consequence of error is injury, legal exposure, or financial loss, full post-editing may still be insufficient and human retranslation is the correct route. Never let the apparent fluency of the MT seduce you into downgrading the level; fluent wrong text is the most common cause of under-editing.

### Define Each Level By Explicit Acceptance Criteria

Vague level names cause drift. Define, in writing, what each level accepts and rejects so that every editor applies the same standard. Light post-editing accepts stylistic awkwardness, non-idiomatic phrasing, register inconsistency, and minor fluency issues, provided the meaning is recoverable; it requires fixing meaning errors only: mistranslations, omissions, additions, hallucinations, negation flips, and wrong numbers, entities, dates, and units. Full post-editing rejects everything light rejects plus stylistic and fluency defects: it requires idiomatic phrasing, consistent register, correct terminology, and a reading experience indistinguishable from competent human translation. The boundary between the levels is the treatment of style: light tolerates it, full fixes it. Write the criteria into the job instructions and reference them at handoff so reviewers evaluate against the same standard. Without explicit criteria, two editors will produce two different quality levels from the same MT.

### Match The Level To Content Type, Not To Segment Quality

A common error is to choose the level segment by segment based on how each sentence looks. This produces inconsistent documents and defeats the efficiency logic of post-editing. The level is a property of the job, set by content type and risk, and applied uniformly. Within a job, an editor may spend more time on a hard segment and less on an easy one, but the acceptance bar stays the same. If a segment is so bad that meeting the chosen level requires near-retranslation, that is a signal to flag the segment or the engine, not to silently upgrade the whole job to full post-editing. Reserve per-segment escalation for genuinely unfixable cases, and document them so the pattern becomes engine feedback.

### Factor In The Audience And The Shelf Life Of The Content

The level should account for who reads the text and how long it lives. Internal, transient content read once and discarded can tolerate light post-editing even when imperfect, because the cost of lingering awkwardness is low and the content will not accumulate reputation damage. Published, durable content that will sit on a website or in a product for years must meet full post-editing standards, because defects in it compound over time and shape the audience's perception of the brand. Audience expertise also matters: a specialist audience may forgive stylistic roughness if the terminology is exact, while a general audience needs fluent, idiomatic text to stay engaged. Ask who suffers if this text is wrong or awkward, and weight the level toward their tolerance.

### Negotiate The Effort-Quality-Cost Tradeoff Explicitly With The Requester

Level selection is a tradeoff among effort, quality, and cost, and the requester is the party that bears the consequences. Make the tradeoff explicit rather than implicit. Present the options as: raw MT at near-zero cost with high residual risk; light post-editing at moderate cost fixing meaning only; full post-editing at higher cost approaching human translation quality; and human retranslation at full cost with full control. State what each level does and does not guarantee, and let the requester choose with eyes open. Record the chosen level in the brief so it cannot be relitigated at review. When the requester asks for full quality at light cost, name the contradiction and force a decision; silent compromise produces work that satisfies no one.

### Account For Reviewer And Downstream Expectations

The level you choose must be communicated to everyone downstream: reviewers, QA, layout, and the client. Reviewers who do not know the level will either demand full-quality fixes on a light job, wasting effort and demoralizing editors, or accept light-quality output on a full job, under-delivering to the client. Put the level, the acceptance criteria, and any segment-level exceptions in the handoff package. If the content will be reviewed by a third party with its own standard, reconcile the standards before editing begins, not after. A level that reviewers do not share is a level that will not hold.

### Revisit The Level When Conditions Change

A level chosen once is not permanent. Conditions that justify a change include a shift in content purpose (a draft becomes published), a change in audience (internal becomes customer-facing), a discovered risk (a field turns out to be safety-relevant), or a sustained change in MT quality (the engine improves enough to lower the level, or degrades enough to raise it). Build a checkpoint into the workflow to revisit the level for recurring content types, and treat the level as a parameter that can be tuned, not a verdict. Document level changes and their reasons so the decision history survives staff turnover.

## Common Traps

### Choosing The Level From MT Fluency Rather Than Content Purpose

Fluent MT feels finished and tempts a downgrade to light or raw, even when the content is high-stakes. Fluency is orthogonal to accuracy; a beautiful sentence that is wrong is worse than an awkward sentence that is right. Anchor the level to purpose and consequence, never to how the text reads at a glance.

### Drifting Between Levels Within One Document

When no level is set, editors over-edit the segments that annoy them and under-edit the ones that look fine, producing a document that is inconsistent in both quality and voice. This is the most visible defect of unskilled post-editing and the easiest for a reader to notice. Set one level per job and enforce it.

### Using Light Post-Editing For Customer-Facing Or High-Stakes Content

Light post-editing tolerates stylistic and terminological defects that are unacceptable in published or liability-bearing text. Applying it to save cost on content that represents the brand or carries safety or legal weight publishes defects and creates exposure. The savings are illusory because the downstream cost of error dwarfs the editing cost.

### Using Full Post-Editing On Throwaway Internal Content

Full post-editing on content that only needs to be understood wastes budget and time and delays delivery for no reader benefit. Over-editing low-stakes content is a failure of scope discipline, not a sign of quality. Match effort to consequence.

### Letting The Requester Have Full Quality At Light Cost

When a requester demands publication quality at light-post-editing cost, the agent who silently complies produces work that meets neither expectation. The honest response is to surface the contradiction, present the tradeoff, and force a documented decision. Silent compromise is how under-delivery happens.

### Failing To Communicate The Level To Reviewers

A level that the editor holds but the reviewer does not know leads to rejected light jobs for style issues and accepted full jobs that under-deliver. The level is a shared contract; if it is not in the handoff, it does not exist for the reviewer.

### Treating The Level As Permanent

Content purpose, audience, risk, and MT quality all change over time. A level fixed once and never revisited drifts out of alignment with reality and either over-spends or under-delivers. Revisit recurring content types on a schedule.

### Escalating Individual Bad Segments Into A Full Job

When a few segments are bad, the response is to flag those segments, not to silently upgrade the whole job to full post-editing. Upgrading hides an engine problem and inflates cost; flagging feeds the improvement loop.

## Self-Check

- Was the post-editing level, light or full, chosen before editing began, and was it based on content purpose and the consequence of error rather than on how fluent the MT looked?
- Are the acceptance criteria for the chosen level written down, with a clear boundary on what style and fluency defects are tolerated versus fixed?
- Is the level applied uniformly across the whole job, with per-segment escalation reserved for genuinely unfixable cases and documented as engine feedback?
- Does the level match the content's audience, shelf life, and risk profile, with durable or customer-facing content held to full post-editing and transient internal content held to light?
- Was the effort-quality-cost tradeoff presented explicitly to the requester, with the chosen level and its guarantees recorded in the brief?
- Is the level communicated to reviewers, QA, and downstream parties so evaluation happens against the same standard?
- Are recurring content types revisited on a schedule so the level stays aligned with changing purpose, audience, risk, and MT quality?
- Are catastrophically bad segments flagged rather than silently absorbed by upgrading the whole job?
- No high-stakes content was routed to light post-editing or raw MT to save cost, and no throwaway content was over-edited to full post-editing.
- The chosen level is defensible against the question: what happens to the reader, the brand, or the liable party if a residual defect slips through?
