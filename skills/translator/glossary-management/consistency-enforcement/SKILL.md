---
name: consistency_enforcement.md
description: Use when the agent is enforcing terminology and style consistency across a translation or localization project, applying glossary and termbase rules, resolving conflicts between consistency and naturalness, or managing consistency across multiple files translators and versions.
---

# Consistency Enforcement

Consistency is what makes a translation feel professional and a brand feel unified. When the same concept is rendered three different ways in the same product, users notice, even if they cannot articulate why the text feels off. When terminology drifts between the documentation, the interface, and the marketing, trust erodes. When style shifts register across a deliverable, the reader's confidence drops. Yet consistency is not simply a matter of applying a glossary mechanically. Terms conflict with naturalness, different contexts call for different renderings, and enforcing rigid uniformity can produce text that is consistent but wrong. Consistency enforcement is the disciplined work of applying agreed terminology and style, detecting and resolving deviations, and knowing when genuine variation is correct and when it is an error to fix.

Use this skill when enforcing terminology and style consistency, when applying glossaries and termbases during translation and review, when resolving consistency conflicts, or when managing consistency across multiple files, translators, and versions. The goal is to deliver text that is uniformly correct and professional without enforcing uniformity where variation is appropriate.

## Core Rules

### Establish The Consistency Authorities

Consistency requires authorities that everyone follows. Establish them before work begins and make them accessible.

The primary authorities are the termbase or glossary for terminology, the style guide for register and conventions, the translation memory for previously approved renderings, and reference content for voice and tone. Each authority governs a different aspect. The termbase governs what terms to use. The style guide governs how to write. The translation memory governs what was approved before. Reference content governs how the brand sounds.

When authorities conflict, define the precedence. Typically termbase overrides translation memory, because an old translation may use deprecated terms. Style guide overrides translator preference. Document the precedence so reviewers apply it consistently.

### Apply Terminology Systematically

Terminology consistency means the same concept is rendered the same way throughout. Apply the termbase systematically, not selectively.

Configure the translation environment to surface termbase entries during work. When a source term appears, use the approved equivalent unless there is a documented reason not to. Track exceptions, such as cases where the approved term does not fit the syntax or where context requires an admitted alternative. Exceptions should be deliberate and recorded, not accidental.

Do not substitute synonyms for variety. In terminology work, variety is a defect, not a virtue. The same concept should use the same term, even at the cost of repetition, because repetition aids clarity and search.

### Detect And Resolve Deviations

Consistency errors are easy to miss during drafting because each segment looks correct in isolation. Detect them through systematic review.

Run terminology consistency checks that compare the source terms against the target to find cases where an approved equivalent was not used. Run concordance searches on key terms to find variant renderings. Review for style consistency by checking register, address form, punctuation, and formatting conventions across the deliverable. Cross-check interface terms against documentation and marketing to ensure they align.

When a deviation is found, decide whether it is an error to fix or a legitimate variation. Fix errors. Document legitimate variation so reviewers do not re-flag it.

### Balance Consistency With Naturalness

Rigid consistency can produce unnatural text. Know when to enforce and when to allow controlled variation.

A term that works as a noun may need a different form as a verb. A UI label that must match the interface exactly may sound abrupt in flowing prose, requiring a decision about whether to match the UI or to write naturally. Repetition that aids clarity in instructions can feel heavy in marketing. Balance these by defining rules, such as match the UI term on first mention and in procedural steps, but vary naturally in descriptive prose where ambiguity is not a risk.

Document the balance rules so consistency enforcement does not become mechanical uniformity that harms readability.

### Manage Consistency Across Multiple Translators

When several translators work on related content, consistency depends on coordination, not individual diligence.

Share the termbase, style guide, and reference content with all translators before work begins. Assign a lead translator or reviewer to own consistency across the team. Use a shared translation memory so approved renderings propagate. Hold a kickoff to align on key terms and decisions, and maintain a living decisions log so choices made by one translator are visible to all.

Without coordination, each translator's correct individual choices combine into an inconsistent deliverable.

### Manage Consistency Across Versions And Updates

Content evolves, and updates must stay consistent with prior versions while incorporating approved changes.

When updating a translation, compare against the previous version to preserve established terminology unless the termbase has changed. When the termbase changes, plan a find-and-replace or review pass to update deprecated terms across existing content. Track which content has been aligned to which termbase version, so stale content is identifiable.

Ignoring version consistency produces content where one page uses the new term and the next page uses the old one, confusing users.

### Enforce Consistency In Non-Obvious Elements

Consistency extends beyond terminology to many elements users notice. Enforce them all.

These include capitalization conventions, date and number formats, units and unit formatting, abbreviation and acronym handling, product and feature name spelling, punctuation conventions such as serial comma or quotation marks, heading and title capitalization, and the treatment of non-translatable elements. Inconsistency in any of these signals carelessness, even when the prose is correct.

Include these elements in the style guide and check them during review.

### Use QA Tools But Verify Their Output

Automated quality assurance tools detect many consistency errors, but they produce false positives and miss context-dependent issues. Use them as aids, not authorities.

Run QA checks to flag potential terminology deviations, double meanings, formatting inconsistencies, and numeric mismatches. Review each flag and decide whether it is a real error. Do not auto-accept all corrections, because some flags reflect legitimate variation. Do not ignore all flags, because the real errors are hidden among the false positives.

## Common Traps

### Substituting Synonyms For Variety

In terminology work, variety is a defect. The same concept should use the same term throughout.

### Ignoring The Termbase Selectively

Applying approved terms sometimes and inventing alternatives other times defeats the termbase and confuses users.

### Enforcing Uniformity Where Variation Is Correct

Mechanical consistency that ignores syntax, register, and context produces text that is consistent but unnatural or wrong.

### Failing To Coordinate Multiple Translators

Without shared authorities and a decisions log, individual correct choices combine into inconsistent deliverables.

### Letting Updates Drift From Prior Versions

Updating content without checking prior versions creates pages where terminology shifts mid-product.

### Overlooking Non-Obvious Consistency Elements

Capitalization, formats, punctuation, and name spelling signal carelessness when inconsistent, even if prose is correct.

### Blindly Accepting QA Tool Output

Auto-accepting corrections introduces errors from false positives; ignoring flags misses real errors.

## Self-Check

Before approving a translation for consistency, verify:

- Consistency authorities, termbase, style guide, translation memory, and reference content, are established and accessible, with defined precedence.
- Terminology is applied systematically from the termbase, with exceptions deliberate and recorded.
- Deviations were detected through terminology checks, concordance searches, and style review, and resolved as errors or documented as legitimate variation.
- Consistency is balanced with naturalness, with rules defining where to enforce and where to allow controlled variation.
- Multiple translators were coordinated through shared authorities, a lead reviewer, shared memory, and a decisions log.
- Version and update consistency was checked against prior content and termbase changes.
- Non-obvious elements such as capitalization, formats, punctuation, abbreviations, and name spelling are consistent.
- QA tool output was reviewed flag by flag, with real errors fixed and false positives dismissed deliberately.
- The same concept uses the same term throughout, except where documented variation is correct.
- The deliverable reads as the work of one unified voice, not a patchwork of individual choices.
