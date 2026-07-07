---
name: release_notes_and_changelogs.md
description: Use when the agent is writing or editing release notes changelogs or version announcements, deciding what changes to document and at what detail, framing breaking changes and deprecations clearly, ordering and grouping entries for scannability, balancing transparency against readability, managing tone for security and incident communications, or ensuring users can understand what changed and what they must do about it.
---

# Release Notes And Changelogs

Release notes and changelogs are the interface between a product's evolution and its users. They tell users what changed, why it matters, and what they must do about it, and their quality directly affects adoption, support burden, and trust. Poor release notes bury critical changes among trivia, misframe breaking changes, or omit what users need to act, producing confusion, failed upgrades, and avoidable support load. Good release notes are a distinct editorial form, governed by the reader's need to scan, assess, and act, and they require judgment that ordinary prose editing does not.

Use this skill when writing or editing release notes, changelogs, version announcements, upgrade guides, or security and incident communications tied to a release. It covers deciding what to document and at what detail, framing breaking changes and deprecations, ordering and grouping entries for scannability, balancing transparency against readability, and managing tone for sensitive communications. The editor's task is to ensure users can quickly understand what changed, how it affects them, and what action they must take.

## Core Rules

### Select Changes Worth Documenting

A changelog is not a commit log. Not every change merits a user-facing entry, and documenting everything produces noise that buries the changes users care about. The first editorial judgment is selection: which changes affect users enough to document, and which are internal, trivial, or irrelevant to the audience. Over-documentation is as harmful as under-documentation, because it trains users to ignore the changelog.

For each change, apply a user-relevance test. Document changes that affect user-visible behavior, interfaces, performance, security, compatibility, or workflows. Exclude purely internal changes, refactors, test improvements, or dependency bumps that do not affect users, unless they matter for compliance or transparency. For changes with user impact, calibrate the detail to the impact: a minor fix may warrant a single line, while a significant feature may warrant explanation. Where the volume of changes is high, consider tiering, a summary of highlights followed by a complete list, so readers can scan the important changes without wading through everything. Selection is what makes a changelog scannable rather than exhausting.

### Frame Breaking Changes And Deprecations For Action

Breaking changes and deprecations are the highest-stakes content in release notes, because they require users to act to avoid failure. A breaking change buried in a generic list, or a deprecation stated without migration guidance, causes failed upgrades and support crises. These entries must be framed so users immediately recognize the impact and know what to do.

For breaking changes, lead with the impact and the required action, not the technical detail. State clearly what breaks, who is affected, and what the user must do, with a migration path, code example, or link to a guide. Use explicit framing, "Breaking change," "Action required," rather than burying the urgency in prose. For deprecations, state what is deprecated, when it will be removed, and what the replacement is, giving users time to migrate before the removal. Distinguish hard breaks, which fail immediately, from soft deprecations, which warn but still work, since users must prioritize differently. Group breaking changes prominently, often at the top of the notes, so users encounter them first. The goal is that no user suffers a failure they could have avoided by reading the notes.

### Order And Group Entries For Scannability

Release notes are scanned, not read. Users look for what affects them, what's new, what's fixed, and what they must do, and the structure must support that scan pattern. A flat list of changes in arbitrary order forces users to read everything to find what matters, which they will not do. Ordering and grouping are what make notes usable.

Group changes by type and impact: breaking changes and required actions first, then new features, then fixes, then other changes. Use consistent, recognizable headings for these groups so users can navigate to what concerns them. Within groups, order by impact or relevance to the audience, with the most significant changes first. For each entry, lead with the user-relevant summary, not the internal detail, so a reader scanning headings and first lines gets the gist. Keep entries concise, linking to deeper detail where warranted. The structure should let a user answer "does this release affect me, and how?" in seconds, not minutes.

### Calibrate Detail To User Need

Each entry's detail should match what the user needs to understand and act on the change. Too little detail leaves users confused about what changed or what to do; too much buries the actionable point in implementation detail. The right detail depends on the change's impact and the audience's needs.

For each entry, provide enough for the user to understand the change's effect and take any required action. For bug fixes, a clear description of what was fixed may suffice. For features, enough explanation for users to recognize and try the capability, with links to fuller documentation. For breaking changes, the impact, the action, and the migration path. Avoid internal detail that does not help the user, such as implementation specifics or internal ticket references, unless the audience is technical enough to want them. Where an entry is complex, link to a dedicated guide rather than cramming detail into the changelog. Detail calibrated to user need respects the reader's time while ensuring they have what they need to act.

### Balance Transparency Against Readability

Transparency is valuable in release notes, especially for security, reliability, and trust, but unfiltered transparency can harm readability. Documenting every internal change, every rejected approach, or every security detail produces notes that are thorough but unusable. The editor must balance the value of transparency against the cost to scannability and clarity.

Be transparent about changes that affect users, including security vulnerabilities, reliability issues, and compatibility changes, since users need this information to assess risk and act. For security changes, follow responsible disclosure norms, providing enough detail for users to assess their exposure and apply fixes, without providing exploit detail that aids attackers. For internal changes, be transparent where they matter for compliance, audit, or trust, but do not document every internal event. Where full detail would overwhelm, summarize and link to fuller records. The balance is not secrecy versus openness but usefulness versus noise, and the test is whether the transparency serves the reader's needs.

### Manage Tone For Security And Incident Communications

Some release notes communicate failures: security vulnerabilities, outages, data issues, or regressions. The tone of these communications matters as much as the content, because it shapes user trust and the perception of the organization's competence and honesty. Tone that minimizes, deflects, or obfuscates damages trust; tone that is clear, honest, and user-focused preserves it.

For security and incident communications, be direct about what happened, what was affected, and what users should do. Avoid euphemisms and minimization, "a security update" that obscures a critical vulnerability, or "improved reliability" that hides an outage. Acknowledge impact honestly and focus on the user's needs, assessment and action. Avoid blaming language, whether deflecting to users or to individuals internally, since it reads as unprofessional and erodes confidence. For severe incidents, communicate the organization's response and any commitment to prevention, where appropriate. The tone should convey competence and honesty, recognizing that how a failure is communicated affects trust as much as the failure itself.

### Maintain Consistency Across Releases

Release notes are a series, and users develop expectations about their structure, terminology, and detail across releases. Inconsistency between releases, different groupings, varying terminology, or unpredictable detail levels, undermines scannability and forces users to relearn the format each time. Consistency is part of usability.

Establish and follow conventions for the changelog: consistent group headings, entry formats, terminology, and detail levels across releases. Use the same terms for the same concepts, the same framing for breaking changes, and the same structure for migration guidance. Where conventions evolve, note the change rather than letting the format drift silently. For versioned products, align the notes with the versioning scheme, so users can predict what kind of changes a release contains from its version number. Consistency turns each release's notes from a new document into a familiar interface, and it reduces the cognitive cost of staying current with the product.

## Common Traps

### Documenting Every Commit

A changelog is not a commit log. Select changes that affect users and exclude internal trivia.

### Breaking Changes Buried In The List

Users miss required actions and suffer avoidable failures. Lead with impact and required action.

### Flat List Without Grouping Or Ordering

Unstructured notes force users to read everything. Group by type and order by impact.

### Detail Mismatched To User Need

Too much internal detail or too little action guidance both fail the reader. Calibrate to what users need.

### Euphemism Obscuring Security Issues

Minimizing language damages trust. Be direct about impact and required action.

### Inconsistent Format Across Releases

Drifting structure undermines scannability. Maintain conventions for headings, terms, and detail.

### Deprecation Without Migration Path

Users cannot act on deprecations without guidance. State the replacement and timeline.

### Internal Ticket References As User Content

Ticket numbers and internal detail do not help users. Exclude unless the audience is internal.

## Self-Check

Before treating release notes and changelogs as complete, verify:

- Only changes that affect users are documented, with internal and trivial changes excluded.
- Breaking changes and deprecations are framed prominently, with impact, required action, and migration guidance leading each entry.
- Entries are grouped by type and ordered by impact, supporting a user's scan pattern.
- Each entry's detail is calibrated to what the user needs to understand and act, without internal clutter.
- Transparency is balanced against readability, with security and incident changes communicated honestly without overwhelming detail.
- Tone for security and incident communications is direct, honest, and user-focused, without minimization or blame.
- Conventions for structure, terminology, and detail are consistent across releases.
- Deprecations include the replacement and the timeline for removal.
- A user scanning the notes could quickly determine whether the release affects them, how, and what they must do.
