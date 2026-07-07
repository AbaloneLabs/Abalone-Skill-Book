---
name: terminology_change_and_version_management.md
description: Use when the agent is managing terminology changes over time, propagating updated term decisions to existing translations, versioning the termbase across product releases, handling retroactive terminology corrections in already-published content, or governing the termbase lifecycle as concepts terms and products evolve.
---

# Terminology Change And Version Management

A termbase is not a static artifact; it is a living document that changes as products evolve, concepts are renamed, standards are updated, and past decisions are revised. Every terminology change creates a cascade of consequences: already-translated content now uses a deprecated term, translation memory contains segments with the old term, and future translations must use the new term. Without version management, terminology changes produce inconsistency between old and new content, confuse users who encounter both terms, and create maintenance debt as the gap between the termbase and the actual content widens. The skill is in managing the termbase lifecycle deliberately: deciding when to change a term, versioning the change, propagating it to existing content, and handling the retroactive correction of already-published translations.

Agents tend to miss that terminology changes have downstream effects on existing content and memory, that versioning is needed to track what term was preferred when, that retroactive correction has a cost-benefit tradeoff, and that ungoverned changes accumulate into a termbase that no longer matches reality. The harm is content where the same concept is called different things depending on when it was translated, translation memory that serves outdated terms, and a termbase whose history is lost so no one knows why a term changed or whether the change was deliberate.

Use this skill when managing terminology changes over time, versioning a termbase, propagating term updates to existing translations, handling retroactive corrections, or governing the termbase lifecycle. The goal is to ensure that terminology changes are deliberate, versioned, propagated, and traceable, so that content remains consistent as terms evolve.

## Core Rules

### Treat Every Terminology Change As A Lifecycle Event

A terminology change is not a simple edit to the termbase; it is an event with a before state, a decision, an effective date, and downstream consequences. Treat each change as a lifecycle event that is documented, versioned, and propagated.

For each change, record: the old term (what was preferred before), the new term (what is preferred now), the reason for the change (product rename, standard update, correction of a past error, user feedback), the effective date (when the new term takes effect), the scope (which products, content types, or locales are affected), and the propagation plan (how the change will be applied to existing content and memory). Without this documentation, the change is an invisible edit: no one knows when it happened, why, or what content is affected. A termbase change log that records each lifecycle event is the foundation of version management.

### Version The Termbase To Track Temporal Consistency

Terminology consistency is temporal: content translated at different times may legitimately use different terms if the termbase changed between those times. Versioning the termbase allows you to determine which term was preferred when a given piece of content was translated, which is essential for auditing and for deciding whether an inconsistency is an error or a historical artifact.

Version the termbase by release or date. Each version records the preferred terms, allowed variants, and deprecated terms as of that version's effective date. When auditing content, compare the terms used against the termbase version that was current when the content was translated, not just against the latest version. This distinction matters: content using a term that was preferred at translation time but has since been deprecated is a historical artifact, not an error, though it may still warrant retroactive correction. Content using a term that was never preferred is an error. Versioning enables this distinction.

### Propagate Changes To Existing Content Deliberately

When a terminology change takes effect, existing translated content still uses the old term. The decision of whether and how to propagate the change to existing content is a cost-benefit judgment that depends on the content's visibility, lifespan, and the risk of user confusion from inconsistent terminology.

Assess propagation by content category. High-visibility, long-lifespan content (product UI, core documentation, legal content) usually warrants retroactive correction because users will encounter both old and new terms and be confused. Low-visibility or short-lifespan content (transient marketing campaigns, archived support articles) may not warrant correction because the cost exceeds the benefit. For content that will be corrected, plan the correction as a batch operation: use terminology search-and-replace tools to find all occurrences of the old term and replace with the new term, with human review of the changes to ensure the replacement is correct in context (a term may appear in different forms or contexts where a blanket replace would be wrong). Document which content was corrected and which was left as-is, with the rationale.

### Handle Translation Memory Contamination

Translation memory is a major casualty of terminology changes. Memory segments containing the old term will be served as matches for future translations, introducing the deprecated term into new content. Without memory management, terminology changes are undermined by the memory re-serving outdated terms.

When a terminology change occurs, identify and update or invalidate the memory segments that contain the old term. Options include: updating the segments to replace the old term with the new term (preserving the match leverage while correcting the terminology), invalidating the segments so they are no longer served as matches (forcing fresh translation with the new term), or flagging the segments with a warning so post-editors know to check and correct the terminology. The choice depends on the volume of affected segments and the reliability of automated replacement. For high-volume memory, automated replacement with sampling review is practical; for low-volume or high-risk memory, invalidation may be safer. Do not leave contaminated memory in place, or future translations will reintroduce the deprecated term.

### Manage Retroactive Correction Cost And Benefit

Retroactive correction of already-published content has a real cost: translator or post-editor time, review effort, re-publication, and potential disruption to users who have learned the old term. The benefit is consistency and reduced confusion. The decision to correct retroactively must weigh these explicitly.

For each terminology change, estimate the correction cost (number of occurrences, content volume, review complexity) and the benefit (user confusion avoided, search and indexing improvement, brand consistency). Prioritize correction for changes where the old and new terms are so different that users will not connect them (high confusion risk), where the content is high-visibility, or where the old term is factually wrong or offensive. Defer or skip correction where the terms are similar enough that users will connect them, where the content is low-visibility, or where the content is near end-of-life. Document the cost-benefit decision so it can be revisited if circumstances change.

### Govern The Termbase Through A Change Control Process

Terminology changes should not be made ad hoc by individual translators or teams. Ungoverned changes produce a termbase that reflects personal preferences rather than deliberate decisions, and they create inconsistency when different teams change terms independently. A change control process ensures that changes are proposed, reviewed, approved, and communicated consistently.

Establish a terminology change process: proposals can come from translators, reviewers, product teams, or user feedback; proposals are reviewed by a terminology owner or committee that evaluates the change against criteria (conceptual accuracy, domain acceptance, consistency, user impact); approved changes are documented in the change log, versioned, and communicated to all translation teams and content owners. The process should be lightweight enough not to stall translation but rigorous enough to prevent arbitrary changes. For organizations with ongoing translation, a monthly or quarterly terminology review cycle balances responsiveness with governance.

### Communicate Changes To All Affected Teams

A terminology change that is not communicated affects only the team that made it; all other teams continue using the old term, producing inconsistency. Communication is as important as the change itself.

When a change is approved, communicate it to all affected teams: translation teams (who must use the new term going forward), post-editors (who must correct the old term in MT output), content owners (who may need to update source content if the source term also changed), and product teams (who may need to update UI, marketing, or other content). Include the old term, the new term, the reason, the effective date, and any propagation instructions. Use a central terminology communication channel (a termbase notification, a team announcement, a release note) so the information is accessible and not buried in email. Track acknowledgment so you know which teams have received and understood the change.

## Common Traps

### Editing The Termbase Without Documenting The Change

An undocumented change loses the before state, the reason, and the effective date. No one can tell whether an inconsistency is an error or a historical artifact. Treat every change as a lifecycle event with a change log entry.

### Ignoring Downstream Effects On Existing Content

A terminology change that is not propagated leaves existing content using the old term, creating inconsistency between old and new content. Assess and plan propagation for each change.

### Leaving Translation Memory Contaminated

Memory segments with the old term re-serve the deprecated term into future translations, undermining the change. Update, invalidate, or flag affected memory segments.

### Correcting Everything Retroactively Without Cost-Benefit

Blanket retroactive correction of all content regardless of visibility or lifespan wastes effort on low-value corrections. Prioritize by confusion risk, visibility, and content lifespan.

### Failing To Version The Termbase

Without versioning, temporal consistency cannot be assessed. Content using a term preferred at translation time cannot be distinguished from content using a term that was never correct. Version by release or date.

### Making Changes Ad Hoc Without Governance

Ungoverned changes reflect personal preferences and create inconsistency when teams change terms independently. Use a change control process with proposal, review, and approval.

### Not Communicating Changes To All Teams

A change known only to the team that made it leaves all other teams using the old term. Communicate changes centrally to all affected teams with acknowledgment tracking.

### Assuming A Blanket Search-And-Replace Is Always Safe

Replacing the old term with the new term across all content may be wrong in contexts where the term appears in a different form or meaning. Review replacements in context, especially for terms with multiple senses.

## Self-Check

- [ ] Is each terminology change documented as a lifecycle event with the old term, new term, reason, effective date, scope, and propagation plan recorded in a change log?
- [ ] Is the termbase versioned by release or date, so temporal consistency can be assessed and historical artifacts distinguished from errors?
- [ ] Has the propagation of each change to existing content been planned by content category, with high-visibility long-lifespan content prioritized for retroactive correction?
- [ ] Have translation memory segments containing the old term been identified and updated, invalidated, or flagged to prevent re-serving the deprecated term?
- [ ] Has the cost-benefit of retroactive correction been assessed for each change, with correction prioritized by confusion risk, visibility, and content lifespan?
- [ ] Is a terminology change control process in place with proposal, review, approval, and communication steps, balancing responsiveness with governance?
- [ ] Are approved changes communicated to all affected teams (translators, post-editors, content owners, product teams) through a central channel with acknowledgment tracking?
- [ ] Are retroactive corrections reviewed in context rather than applied as blanket search-and-replace, especially for terms with multiple senses?
- [ ] Is the termbase lifecycle documented with change log, versions, propagation records, and communication records so terminology evolution is traceable and auditable?
