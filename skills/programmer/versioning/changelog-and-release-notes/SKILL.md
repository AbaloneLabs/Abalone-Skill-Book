---
name: changelog_and_release_notes.md
description: Use when the agent is writing or automating a changelog or release notes, classifying changes by type and user impact, writing migration guidance for breaking changes, automating release notes from commits or PRs, targeting different audiences (users, operators, developers), or keeping release notes fresh, accurate, and useful as the project releases frequently.
---

# Changelog and Release Notes

A changelog is the document that tells users what changed between versions and what they must do about it, and it is consistently written for the author rather than the reader. The author knows what they did and why, so they write "refactored auth module" or "fixed bug #1234," which is meaningful to them and useless to a user deciding whether to upgrade. The result is a changelog that exists but that no one reads, because reading it does not answer the only questions that matter: does this release affect me, does it break anything, and what do I need to do. Agents often treat the changelog as an afterthought generated from commit messages, and discover that it fails its only job, which is to inform decisions.

The judgment problem is that a changelog serves multiple audiences with different needs, and it must be written from their perspective, not the developer's. An end user wants to know what new things they can do and what bugs are fixed. An operator wants to know about breaking changes, required migrations, performance impacts, and security fixes. A developer integrating the library wants to know about API changes and compatibility. A single flat list of commit-derived messages serves none of them well. The skill is classifying changes by type and impact, leading with what the audience must act on (breaking changes, security fixes, required migrations), writing each entry from the reader's perspective (what changed for them, not what you did), and automating the tedious parts without letting automation produce noise. A good changelog is read and trusted; a bad one is ignored, which means users upgrade blind.

## Core Rules

### Write for the reader who decides whether to upgrade, not for the author

The reader's questions are: does this affect me, does it break anything, what must I do, and what do I gain. Write each entry to answer these. "Fixed a race condition in the connection pool that caused intermittent errors under high load" tells the reader whether they are affected and what they gain; "fix pool bug" tells them nothing. Phrase changes in terms of user-visible behavior and impact, not implementation. When a change has no user-visible impact (a pure internal refactor), consider whether it belongs in the changelog at all; cluttering the log with internals trains readers to skip it.

### Classify changes by type and lead with what requires action

Group changes into consistent categories so readers can scan to what matters to them. A common scheme: Breaking Changes, Security, Deprecated, Added, Changed, Fixed, Removed, and Internal/Performance. Order categories by action-urgency: breaking changes and security fixes first (these require action), then deprecations (these require planning), then additions and fixes (these are good news). A reader in a hurry should be able to look at the top of the release notes and know immediately whether action is required. Burying a breaking change in a flat list of features is a common and damaging failure.

### Call out breaking changes, migrations, and security fixes prominently and separately

These are the changes that cause incidents when missed, and they must be impossible to overlook. A breaking change buried in "Changed" will be missed by a reader scanning quickly. Put breaking changes in their own section at the top, with a clear statement of what breaks and the exact migration step required. Do the same for security fixes (which may warrant a separate advisory in addition to the changelog) and required migrations (configuration changes, data migrations, dependency bumps). The cost of a missed breaking change is paid by the user in an incident; the cost of prominent callouts is a few lines of formatting.

### Provide concrete migration guidance, not just a description of the change

For any change requiring user action, state the action concretely. Not "the config format changed" but "rename `timeout` to `request_timeout` in your config file; the old name is accepted but deprecated and will be removed in v3." Not "the API was updated" but "replace `client.fetch()` with `client.get()`; see the migration guide." Migration guidance that requires the reader to reverse-engineer the action from the description is incomplete. Link to detailed migration docs where the change is complex.

### Automate collection, but curate the output

Generating the changelog entirely by hand is tedious and error-prone at high release cadence; generating it entirely from commit messages produces noise. The effective pattern is to automate collection (require PRs or commits to carry a changelog fragment or a labeled category, then assemble fragments at release time) while a human curates the assembled log: merges related entries, rewrites commit-jargon into user-facing language, promotes or demotes entries between categories, and removes internals that do not belong. Automation handles the volume; curation handles the quality. Pure automation produces a commit log, not a changelog.

### Target the right level of detail for the audience and release

A patch release fixing one bug needs a one-line note. A major release with breaking changes, new features, and migrations needs a structured document with sections and links to detail. Match the format to the release's significance, and do not pad a small release with ceremony or compress a large one into a flat list. If you serve multiple audiences (end users vs. operators vs. integrators), consider whether a single changelog serves all or whether a separate, more technical operator-focused note is warranted for releases with significant operational impact.

### Keep the changelog current with each change, not written at release time

Writing the changelog at release time, from memory of weeks of work, produces omissions and inaccuracies. Require each change (PR or commit) that affects users to contribute its changelog entry at the time the change is made, when the author knows exactly what changed and why. Assemble and curate at release time, but the raw material is captured fresh. This also forces the author to articulate the user impact at the moment they understand it best, which improves quality and catches changes that have no user-visible impact (and thus do not belong).

## Common Traps

### Commit-message-derived entries written for the author

"Fix pool bug" or "refactor auth" means nothing to a user deciding whether to upgrade. Write from the reader's perspective, in terms of user-visible impact.

### Burying breaking changes in a flat list

A breaking change missed by a reader scanning quickly causes an incident. Put breaking changes, security fixes, and required migrations in their own prominent section at the top.

### Pure automation producing a commit log

Generating from commits without curation produces noise that trains readers to skip the changelog. Automate collection, but curate the output into user-facing language.

### Writing the changelog at release time from memory

This produces omissions and inaccuracies. Capture each entry when the change is made, when the author knows the impact best.

### No concrete migration guidance

"Config format changed" without the exact action leaves the reader to reverse-engineer the step. Provide concrete, copy-able migration instructions.

### Cluttering the log with internal refactors

Internal-only changes that no user sees train readers to skip the log, causing them to miss the entries that matter. Omit pure internals or relegate them to a clearly marked section.

### One format for every release regardless of size

A patch fix padded with ceremony, or a major release compressed into a flat list, both fail the reader. Match format to release significance.

## Self-Check

- Is each entry written from the perspective of a reader deciding whether to upgrade (what changed for them, what they gain), rather than from the author's perspective (what you did)?
- Are changes classified into consistent categories (Breaking, Security, Deprecated, Added, Changed, Fixed, Removed), ordered by action-urgency so readers can scan to what matters?
- Are breaking changes, security fixes, and required migrations called out in their own prominent section at the top, impossible to overlook?
- Does every change requiring action include concrete, copy-able migration guidance (exact rename, exact step, link to detail), not just a description of the change?
- Is collection automated (changelog fragments or labeled PRs assembled at release) while a human curates the output into user-facing language, rather than pure commit-derived noise?
- Is the format matched to the release's significance (brief for patches, structured for majors), with separate operator-focused notes if the audience warrants?
- Are changelog entries captured at the time each change is made (when impact is best understood), then assembled and curated at release, rather than reconstructed from memory?
- Are pure internal changes with no user-visible impact omitted or clearly marked, so the log does not train readers to skip it?
