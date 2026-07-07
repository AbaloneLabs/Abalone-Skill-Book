---
name: content_audit_and_inventory.md
description: Use when the agent is conducting a content audit or inventory, cataloging existing content across a product or site, assessing content against quality and lifecycle criteria, identifying redundant, outdated, or duplicate content, planning a content migration or redesign, mapping current content to a future information architecture, or deciding what content to keep, revise, merge, or retire before restructuring a product.
---

# Content Audit And Inventory

A content audit is the systematic accounting of everything a product currently contains, and it is the unglamorous prerequisite of almost every IA and redesign effort. Teams redesign navigation, restructure taxonomies, and rebuild templates on the assumption that they know what content exists, and they are almost always wrong. Real products accumulate years of orphan pages, duplicated explanations, contradictory versions of the same policy, and dead-end articles that no one owns. Designing a new structure on top of an unexamined content base produces an architecture that fits the team's imagined content rather than the content that actually exists, and the migration then surfaces the gap at the worst possible moment.

Agents tend to fail content audits in predictable ways. They skip the audit because it is tedious and proceed straight to redesign, then discover during migration that half the content does not fit the new structure. They confuse an inventory, a bare list of what exists, with an audit, which evaluates what exists against criteria. They audit only the top-level pages and miss the long tail where most content lives. Or they produce a sprawling spreadsheet with no decision criteria, so the audit becomes an artifact that is filed and ignored rather than a basis for keep, revise, merge, and retire decisions.

Use this skill before restructuring any product, planning a migration, or redesigning templates that content must populate. The goal is a complete, evaluated inventory that turns "what do we have" into "what do we do with each piece," and that grounds the new architecture in the real content base.

## Core Rules

### Distinguish Inventory From Audit

An inventory and an audit are different operations, and conflating them wastes effort. An inventory catalogs what exists; an audit evaluates it. Both are needed, but they answer different questions and must be sequenced.

Separate the two:

- the inventory captures the universe of content: every page, article, record, document, or component, with its location, owner, format, and basic metadata;
- the audit evaluates each item against criteria such as accuracy, currency, relevance, quality, redundancy, and performance;
- run the inventory first to establish scope, then audit, because evaluating before scoping produces a partial and biased picture;
- recognize that a full audit of a large content base may be infeasible, in which case sample strategically and label the sampling explicitly.

### Inventory The Full Content Base, Including The Long Tail

Most content audits fail because they capture only the prominent pages and miss the long tail: archived articles, legacy help docs, regional variants, status pages, and content in less-visible sections. The long tail is usually where redundancy and rot live, and ignoring it guarantees migration surprises.

Inventory comprehensively:

- crawl the full site or product, including authenticated, regional, and archived content where accessible;
- capture content in all relevant formats: pages, articles, records, downloadable documents, emails, and in-product help;
- include metadata that matters for later decisions: owner, last updated, format, locale, template, and analytics identifiers;
- flag content with no clear owner, because unowned content is where outdated and contradictory material accumulates.

A crawl that captures only the top navigation captures the content the team is proud of, not the content that will break the migration.

### Define Evaluation Criteria Before Auditing

An audit without criteria becomes an opinion exercise, and different auditors will judge the same content differently. Define the criteria in advance so evaluation is consistent and the audit produces decisions rather than impressions.

Define criteria deliberately:

- accuracy: is the content correct and current;
- completeness: does it cover what its title promises;
- relevance: does it serve a real user need or business goal;
- quality: is it clear, well-structured, and on-brand;
- redundancy: does it duplicate other content;
- performance: does anyone use it, measured by analytics;
- risk: is it legally sensitive, regulated, or load-bearing in a way that demands special handling.

Apply criteria consistently, and where multiple auditors are involved, calibrate them on a shared sample before dividing the work.

### Assign A Disposition To Every Item

The point of an audit is not to describe the content but to decide what to do with it. Every audited item should receive a disposition that drives the migration or redesign.

Assign dispositions explicitly:

- keep: the content is accurate, useful, and fits the future structure;
- revise: the content is worth keeping but needs updating, restructuring, or rewriting;
- merge: the content duplicates or overlaps another item and should be combined;
- retire: the content is outdated, unused, or no longer relevant and should be removed;
- archive: the content is no longer active but must be retained for legal or historical reasons.

A dispositionless audit is a spreadsheet no one acts on. The disposition is what converts the audit into a work plan.

### Map Current Content To The Future Architecture

An audit performed without reference to the future structure cannot tell whether existing content will fit. Mapping current content to the proposed IA reveals gaps, where the future structure expects content that does not exist, and orphans, where existing content has no home in the future structure.

Map deliberately:

- tag each current item with its proposed future location, or flag it as an orphan with no clear home;
- identify gaps where the future IA requires content that does not yet exist, and plan its creation;
- identify orphans that need a new home, a merge, or retirement;
- use the mapping to pressure-test the future IA, because content that resists placement often signals a structural problem.

### Account For Content Lifecycle And Ownership

Content is not static. It is created, updated, and eventually retired, and an audit that ignores lifecycle produces a snapshot that rots immediately. Ownership is the lever that keeps content alive, because unowned content decays.

Capture lifecycle signals:

- record the owner or responsible team for each item, and flag unowned content for assignment or retirement;
- capture last-updated dates and review cycles, because stale content is a leading source of user distrust;
- identify content that requires periodic review, such as pricing, legal, or policy material, and ensure a review cadence is defined;
- plan for ongoing governance, because an audit is a moment, not a permanent fix.

### Plan The Migration From The Audit

The audit should drive the migration plan, not sit beside it. A migration that ignores the audit reproduces the old content's problems in the new structure.

Connect audit to migration:

- sequence migration by disposition, keeping and revising content before merging and retiring;
- prioritize high-traffic and high-risk content, because errors there are most visible;
- define redirects for retired and merged content, so users and search are not stranded;
- test migrated content against the future IA before launch, because placement that looked right in the spreadsheet may fail in the interface.

## Common Traps

### Skipping The Audit And Redesigning Anyway

Designing a new structure without knowing the current content produces an architecture that fits imagined content and breaks on the real base during migration.

### Confusing Inventory With Audit

A bare list of what exists does not evaluate it. Both are needed, sequenced inventory then audit.

### Capturing Only The Top-Level Pages

The long tail holds most redundancy and rot. A crawl that misses it guarantees migration surprises.

### Auditing Without Criteria

Without predefined criteria, evaluation is inconsistent and the audit produces opinions rather than decisions.

### Leaving Items Without A Disposition

An audit without keep, revise, merge, retire, or archive decisions is a spreadsheet no one acts on.

### Ignoring The Future Architecture

Auditing without mapping to the future IA cannot reveal gaps or orphans and cannot pressure-test the new structure.

### Forgetting Lifecycle And Ownership

Unowned, unreviewed content decays. An audit that ignores lifecycle rots immediately after it is completed.

### Migrating Without Using The Audit

A migration that ignores the audit reproduces the old content's problems in the new structure.

## Self-Check

- [ ] The inventory was completed before the audit, capturing the full content base including the long tail, archived, regional, and unowned content.
- [ ] Evaluation criteria, such as accuracy, relevance, redundancy, performance, and risk, were defined before auditing and applied consistently.
- [ ] Every audited item has an explicit disposition: keep, revise, merge, retire, or archive.
- [ ] Current content was mapped to the future IA, revealing gaps, orphans, and structural problems.
- [ ] Content lifecycle signals, including owner, last-updated, and review cadence, were captured, and unowned content was flagged.
- [ ] The migration plan is driven by the audit, sequenced by disposition, and prioritized by traffic and risk.
- [ ] Redirects are planned for retired and merged content so users and search are not stranded.
- [ ] High-risk content, such as legal, pricing, and policy material, has a defined review cadence.
- [ ] Where a full audit was infeasible, sampling was strategic and explicitly labeled.
- [ ] The audit is connected to ongoing governance, not treated as a one-time snapshot.
