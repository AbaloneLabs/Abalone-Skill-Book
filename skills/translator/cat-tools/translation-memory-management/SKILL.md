---
name: translation_memory_management.md
description: Use when the agent is managing translation memory, deciding on TM leverage and matching thresholds, maintaining and cleaning TM databases, handling TM conflicts and contamination, or configuring TM settings for a project to maximize reuse without propagating errors.
---

# Translation Memory Management

Translation memory (TM) is the asset that turns one-time translation work into compounding value: segments translated once can be reused in future projects, cutting cost and time while improving consistency. But a TM is only as valuable as it is clean, well-organized, and correctly applied. A contaminated TM, one loaded with unreviewed machine translation, outdated terminology, or segments from a different domain, propagates errors across every project that uses it. A TM with poor metadata cannot be filtered, so good matches are buried and bad matches are applied. A TM applied without judgment leads translators to accept matches that do not fit, importing wrong terminology or stale phrasing into new content. Managing TM is an act of stewardship: the decisions made about what enters the TM, how it is organized, and how it is applied determine whether the asset appreciates or decays. Treat the TM as a long-term asset, not as a free byproduct of translation work.

Use this skill when managing translation memory, configuring TM settings for a project, deciding how to handle matches, or maintaining TM quality over time. The goal is a TM that maximizes safe reuse, stays clean and organized, and does not become a source of propagated errors.

## Core Rules

### Treat TM As A Long-Term Asset

A TM is not a byproduct; it is an asset whose value compounds or decays based on how it is managed. Treat it as long-term.

Decisions about what enters the TM, how it is segmented and tagged, and how it is maintained determine whether future projects benefit from reuse or suffer from contamination. Invest in TM hygiene from the start: only load reviewed, approved translations; tag segments with metadata such as client, domain, and content type; and clean the TM periodically. A TM treated as disposable provides poor matches and propagates errors; a TM treated as an asset provides high-quality leverage and consistency across years of work.

The cost of poor TM management is paid in every future project that uses it.

### Organize TM With Meaningful Metadata

A TM without metadata is an undifferentiated mass from which useful matches cannot be extracted. Organize with metadata.

Tag every segment with attributes such as client, project, domain, content type, language variant, and date. This allows filtering: a medical project can pull medical-domain matches and exclude marketing; a client-specific TM can be used for that client's work and a general TM for everything else. Metadata also enables maintenance: you can identify and remove segments from a deprecated project or a client whose contract ended. Without metadata, the TM is opaque, and good matches are buried among irrelevant ones.

Define a metadata schema and enforce it on every import.

### Apply TM With Judgment, Not Blind Acceptance

TM matches are suggestions, not commands. Apply them with judgment.

A 100% match means the source segment is identical to a previously translated segment, but the previous translation may be wrong, may use outdated terminology, or may not fit the new context. A translator who accepts 100% matches without reading them imports whatever errors the TM contains. Review every match against the current context: confirm terminology is current, confirm the fit with surrounding text, and confirm that contextual differences do not change the meaning. Fuzzy matches, partial matches, require even more scrutiny because they are not exact and may introduce inconsistency.

Blind acceptance is how TM contamination spreads. Judgment is how it is contained.

### Manage TM Context And In-Context Matches

A 100% match is not always contextually correct. Manage context to avoid false matches.

The same source segment can require different translations in different contexts: a UI label in a button versus in a heading, a term in a legal context versus a marketing one, a string with different surrounding text that changes emphasis. In-context matching, which considers surrounding segments, helps identify when a 100% match is safe versus when it needs review. Configure tools to flag context-dependent matches and train translators to check them. For UI and software strings, use context features such as screenshots or key paths to verify that a match fits.

A 100% match that is wrong for its context is worse than no match, because it feels authoritative.

### Prevent And Clean Contamination

Contamination, the presence of bad segments in the TM, is the greatest threat to TM value. Prevent and clean it.

Prevent contamination by loading only reviewed, approved translations into the master TM; keeping machine-translated or draft content in a separate workspace until approved; and not merging project TMs back to the master without review. Clean contamination by periodically auditing the TM for quality: spot-check segments, search for known problematic terms, and remove or correct bad segments. If a major terminology change occurs, update the TM rather than leaving stale terms to be matched. Contamination is easier to prevent than to clean, so gate what enters the TM.

A contaminated TM is a liability: every match it provides is suspect, and translators learn to distrust it.

### Use Client-Specific And Domain-Specific TMs

Not all content should share one TM. Use client-specific and domain-specific TMs to improve match quality.

A general TM mixes clients and domains, so matches often come from irrelevant content and require rejection. Client-specific TMs ensure that a client's preferred terminology and style are applied to their work. Domain-specific TMs ensure that medical content matches against medical content, not marketing. Use a primary TM for the relevant client or domain and a secondary, general TM for broader leverage. Configure precedence so the most relevant TM is checked first.

This organization increases match relevance and reduces the risk of importing another client's terminology.

### Handle TM Conflicts And Updates Deliberately

When terminology or style changes, the TM contains outdated segments that conflict with current guidance. Handle updates deliberately.

When a term changes, decide whether to update the TM globally, which is efficient but may alter segments whose context justified the old term, or to flag old segments for review. Document terminology changes and their effective dates so translators know which to follow when matches conflict. When multiple TMs return conflicting matches, define precedence and train translators to prefer the authoritative source. Unhandled conflicts lead to inconsistency, because some segments reflect old terminology and others new.

Plan TM updates as part of terminology management, not as an afterthought.

### Measure And Report TM Leverage

TM value is measurable. Track leverage to justify investment and identify problems.

Track metrics such as match rates, the percentage of segments with 100%, fuzzy, and no matches; cost savings from leverage; and time savings. Low match rates on content that should reuse may indicate segmentation problems or a fragmented TM. High rejection rates on matches may indicate contamination or poor metadata. Use these metrics to guide TM maintenance and to demonstrate value to stakeholders.

## Common Traps

### Loading Unreviewed Content Into The Master TM

Drafts, machine translation, or unreviewed work contaminates the TM and propagates errors.

### Blindly Accepting 100% Matches

A 100% match can be wrong, outdated, or contextually inappropriate; review before accepting.

### No Metadata Or Filtering

An undifferentiated TM buries good matches and cannot be filtered by client, domain, or content type.

### Mixing Clients And Domains In One TM

Cross-contamination imports one client's terminology into another's content.

### Ignoring Context Dependence

The same segment can require different translations in different contexts; in-context checks prevent false matches.

### Not Updating After Terminology Changes

Stale segments conflict with current terminology and cause inconsistency.

### Treating TM As Disposable

Failing to maintain the TM lets it decay, reducing leverage and increasing error propagation over time.

### Not Measuring Leverage

Without metrics, TM problems go undetected and investment cannot be justified.

## Self-Check

Before finalizing TM management for a project or a maintenance cycle, verify:

- The TM is treated as a long-term asset, with only reviewed, approved translations loaded into the master.
- Segments are tagged with meaningful metadata such as client, domain, content type, language variant, and date, enabling filtering.
- TM matches are applied with judgment, with 100% matches reviewed for current terminology and contextual fit before acceptance.
- Context dependence is managed, with in-context matching or context features used to flag matches that may not fit.
- Contamination is prevented by gating what enters the TM and is cleaned through periodic audits and removal of bad segments.
- Client-specific and domain-specific TMs are used where relevant, with precedence configured so the most relevant TM is checked first.
- Terminology and style changes are handled with deliberate TM updates, and conflicts are documented with effective dates.
- TM leverage is measured and reported, with match rates and rejection rates used to guide maintenance.
- No unreviewed or machine-translated content is merged into the master TM without approval.
- The TM maximizes safe reuse without becoming a source of propagated errors.
