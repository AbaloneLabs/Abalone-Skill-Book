---
name: release_notes_and_changelog_writing.md
description: Use when the agent is writing release notes or changelogs for a software product, deciding how to group and describe changes, managing the tone between marketing and engineering, handling breaking changes and migration guidance, writing for different audiences (end users, developers, admins), or diagnosing release notes that are too vague, too technical, or fail to communicate impact.
---

# Release Notes And Changelog Writing

Release notes and changelogs are the interface between a product's development and its users. They tell users what changed, why it matters, and what they must do about it. Despite their importance, they are often treated as an afterthought, generated from commit messages or ticket titles by someone who knows the changes deeply and forgets that the reader does not. The result is notes that list changes the user cannot interpret ("refactored auth module"), bury critical information (a breaking change mentioned in passing), or adopt a tone that serves neither the user who wants to know what they get nor the engineer who needs technical detail. Writing effective release notes is a translation problem: translating the internal knowledge of what changed into the external communication of what the user experiences, at the right level of detail, with the right tone, organized so the reader can find what matters to them.

Agents tend to miss that release notes serve multiple audiences with different needs, that the same change must be described differently for end users and developers, that breaking changes require prominent treatment and migration guidance, and that the tone must balance informativeness with accessibility. The harm is release notes that users cannot use, that hide critical changes, or that read as either marketing fluff or impenetrable engineering log.

Use this skill when writing release notes or changelogs, deciding how to group changes, handling breaking changes, writing for specific audiences, or diagnosing notes that fail to communicate. The goal is to communicate what changed, why it matters, and what the reader must do, in a form the reader can use.

## Core Rules

### Write For The Audience That Will Read The Notes

Release notes are read by different audiences, and the audience determines what to include, how to describe it, and what to emphasize. A consumer app's release notes are read by end users who want to know what new features they get and whether anything will break. A library or SDK's changelog is read by developers who need technical detail to assess compatibility and integration. An enterprise product's notes are read by administrators who must plan upgrades and communicate impact to their organizations. Writing for the wrong audience, or for all audiences simultaneously, produces notes that serve none.

Identify the primary audience and write for them. For end users, focus on visible changes (new features, fixes they will notice, changes to workflow), described in user-facing language without internal jargon. For developers, include technical detail (API changes, dependency updates, behavior changes) with enough specificity to assess impact. For administrators, include deployment implications, configuration changes, and migration requirements. If the notes serve multiple audiences, consider separate sections or documents: a user-facing summary and a technical changelog, or a highlights section and a detailed list. Do not force all audiences into a single undifferentiated list, because the end user overwhelmed by API details and the developer starved of technical specificity are both poorly served.

### Group Changes By Type And Impact

A flat list of changes, organized only by chronology or ticket number, forces the reader to scan everything to find what matters to them. Grouping changes by type and impact creates structure that lets the reader find relevant information quickly and understand the release's overall character at a glance.

Common groupings include: New Features (what the user gains), Improvements (what got better), Bug Fixes (what was broken and is now fixed), and Breaking Changes (what requires action). Within each group, order by impact: the most significant changes first, so a reader who scans the top of each section catches the highlights. For developer-facing changelogs, the "Keep a Changelog" conventions (Added, Changed, Deprecated, Removed, Fixed, Security) provide a recognized structure. Choose a grouping that matches the audience: end users benefit from feature-focused groups; developers benefit from the standardized categories. Be consistent across releases, so readers learn the structure and can navigate predictably. A release that reorganizes its notes each time forces the reader to relearn the structure, which is friction.

### Describe Changes By User Impact, Not Internal Implementation

The most common release notes error is describing changes by their internal implementation rather than by their user impact. "Refactored the authentication module" describes what the engineers did; "signing in is now faster and more reliable" describes what the user experiences. The user does not care about the refactoring; they care about the result. Describe changes by their impact on the user, translating internal work into external consequences.

For each change, ask: what does the user experience differently? If the answer is "nothing visible" (a pure refactor with no behavior change), the change may not belong in user-facing notes at all, or belongs in a developer-facing section as a maintenance note. If the answer is a specific user-facing difference, describe that difference. "Improved performance" is vague; "the dashboard loads twice as fast" is specific. "Fixed a bug" is vague; "fixed an issue where the app crashed when opening large files" is specific. The user evaluates the change by its impact on their experience, so describe the impact, not the implementation. For developer-facing changes, more implementation detail is appropriate, but even there, lead with the impact (what breaks, what changes) before the implementation context.

### Handle Breaking Changes With Prominence And Migration Guidance

Breaking changes, changes that require the user to act or that alter existing behavior, are the highest-stakes content in release notes. A user who misses a breaking change encounters broken functionality, lost data, or failed integrations. Breaking changes must be treated with prominence that matches their stakes, and must include migration guidance that tells the user what to do.

Place breaking changes at the top of the notes, or in a dedicated section that cannot be missed, regardless of where they fall in the change list. Label them clearly ("Breaking Change," "Action Required") and describe what breaks, why it changed, and what the user must do. Migration guidance should be specific and actionable: "Update your configuration file to use the new key names, listed below," not "you may need to update your configuration." If the migration is complex, link to a detailed migration guide. Do not soften breaking changes to avoid alarming users; clear, prominent communication of the required action is more respectful than burying the change in a list. A user who is alarmed but prepared is better served than a user who is calmed but surprised.

### Match Tone To The Audience And The Change

The tone of release notes varies by audience and by the nature of the changes. End-user notes for a consumer app may adopt a warm, feature-highlighting tone that celebrates new capabilities. Developer-facing changelogs adopt a neutral, factual tone that prioritizes precision. Security fixes require a tone that communicates seriousness without alarm. Matching tone to the audience and change ensures the notes are read as appropriate and credible.

For end users, a positive tone that highlights benefits is appropriate, but avoid marketing language that overstates the change ("revolutionary new paradigm" for a minor feature undermines credibility). For developers, a precise, understated tone is appropriate; avoid hype, because developers are skeptical of it. For breaking changes and security fixes, a serious, direct tone communicates the stakes without sensationalism. Across all tones, maintain accuracy: do not claim a change does more than it does, do not minimize a breaking change, and do not exaggerate a minor improvement. The tone is a vehicle for the information, not a substitute for it.

### Include Context For Non-Obvious Changes

Some changes need context for the reader to understand why they matter or why they were made. A bug fix is self-explanatory (something was broken, now it is fixed), but a deprecation, a behavior change, or a removed feature may need context to make sense. Without context, the reader sees the change but does not understand its rationale, and may resist or misinterpret it.

Provide context for changes that are not self-explanatory. For a deprecation: what is being deprecated, why, what is the replacement, and when will it be removed. For a behavior change: what changed, why the old behavior was problematic, and what the user should expect. For a removed feature: what was removed, why, and what the alternative is. Context does not mean lengthy explanation; it means enough information for the reader to understand the change's rationale and respond appropriately. A one-sentence rationale ("we removed this feature because it was used by less than 1% of users and added maintenance burden") transforms a confusing change into an understandable one.

### Maintain Consistency Across Releases For Predictability

Release notes are a series, not one-off documents. Users read them across releases and develop expectations about their structure, terminology, and level of detail. Inconsistency across releases, changing the groupings, the terminology, or the detail level, creates friction as users relearn the format each time.

Maintain consistency in structure (the same groupings and ordering across releases), terminology (calling the same concept by the same name each time), and detail level (providing the same depth of information each time). This does not mean rigid uniformity; adapt to each release's content, but preserve the predictable framework. If the structure needs to evolve, change it deliberately and note the change, rather than letting it drift. Consistency makes the notes a reliable reference that users can navigate efficiently, rather than a puzzle to decode each release.

## Common Traps

### Describing Changes By Implementation Rather Than Impact

Users care about what they experience, not what engineers did. Translate internal work into user-facing consequences.

### Writing For All Audiences In A Single Undifferentiated List

End users and developers have different needs. Serve the primary audience or provide separate sections for each.

### Burying Breaking Changes In The Change List

Breaking changes are the highest-stakes content. Give them prominence and include specific migration guidance.

### Marketing Tone That Overstates Minor Changes

Hype undermines credibility. Match the tone to the actual significance of the change.

### Vague Descriptions That Do Not Communicate Impact

"Improved performance" and "fixed a bug" tell the reader nothing. Be specific about what the user experiences differently.

### Omitting Context For Non-Obvious Changes

Deprecations, behavior changes, and removals need rationale. Without context, the reader cannot understand or respond appropriately.

### Inconsistent Structure Across Releases

Users learn the notes' structure across releases. Maintain consistency for predictability, evolving the structure deliberately rather than letting it drift.

## Self-Check

- [ ] Is the primary audience identified, and are the notes written for that audience's needs (user-facing language for end users, technical detail for developers, deployment implications for administrators)?
- [ ] Are changes grouped by type and impact (New Features, Improvements, Bug Fixes, Breaking Changes, or standardized categories), with the most significant changes first within each group?
- [ ] Is each change described by its user impact rather than its internal implementation, translating engineering work into user-facing consequences?
- [ ] Are breaking changes given prominent placement and clear labeling, with specific, actionable migration guidance?
- [ ] Does the tone match the audience and the nature of the changes (positive for user features, precise for developers, serious for breaking changes and security), without hype or understatement?
- [ ] Do non-obvious changes (deprecations, behavior changes, removals) include enough context for the reader to understand the rationale and respond?
- [ ] Is the structure, terminology, and detail level consistent with prior releases, for predictability across the series?
- [ ] Can a reader of the target audience quickly find the changes relevant to them and understand what they must do in response?
