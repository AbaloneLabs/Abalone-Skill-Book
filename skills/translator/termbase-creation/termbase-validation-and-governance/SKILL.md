---
name: termbase_validation_and_governance.md
description: Use when the agent is validating a terminology database for structural and content quality, auditing termbase entries for completeness consistency and correctness, governing the term lifecycle of candidate proposed preferred admitted deprecated and forbidden terms, defining approval workflows and authority, or maintaining terminology governance over time as language products and domains evolve.
---

# Termbase Validation And Governance

A termbase is a living authority, and authorities that are not validated and governed decay into confident-looking sources of wrong guidance. Validation is the work of checking that the termbase is structurally sound and that its entries are complete, consistent, and correct. Governance is the work of controlling the term lifecycle, defining who decides what, and keeping the resource aligned with how the organization actually uses language as products and domains shift. These are distinct from building the termbase: a termbase can be well designed and well populated and still fail, because nobody validated the entries, nobody owns the lifecycle, and deprecated terms linger in active use while preferred terms are ignored. The harm this skill prevents is the slow corruption of a trusted resource, where translators follow guidance that was once right but is now stale, contradictory, or silently wrong.

Agents miss this work because a populated termbase looks finished. Validation and governance are invisible until they are absent, at which point the symptoms, inconsistency, mistrust, and rework, are blamed on translators rather than on the unmanaged resource they were told to follow. Treat validation and governance as ongoing obligations, not one-time setup.

## Core Rules

### Validate Structure Before Validating Content

Before auditing entries, confirm the termbase obeys its own schema. Structural validation catches defects that corrupt every downstream use: fields at the wrong scope, categorical fields holding free-text values, entries missing concept identifiers, language sections with no term, and cross-references pointing to deleted entries. Run structural checks that enforce the schema rules: every concept entry has a definition; every language section has a term; every status value belongs to the controlled picklist; every subject field belongs to the taxonomy; every relation points to an existing entry.

Structural defects must be fixed before content review, because content review against a broken structure wastes effort and produces inconsistent results. A termbase that violates its own schema cannot be queried reliably, so validation reports built on it are themselves unreliable.

### Audit Content For Completeness, Consistency, And Correctness

Content validation operates on three axes. Completeness asks whether each entry has the fields it needs to be usable: definition, context, scope, provenance, and status. An entry missing definition or context is not usable regardless of how correct its equivalent is. Consistency asks whether entries agree with each other: the same concept is not entered twice under different identifiers; related concepts do not prescribe contradictory equivalents; status values are applied uniformly. Correctness asks whether the equivalent, definition, and context are actually right, checked against authoritative sources and current domain usage.

Run all three audits, in that order. Fixing correctness on an incomplete or inconsistent entry is premature, because the entry may be a duplicate that should be merged or an incomplete record that should be retired. Completeness and consistency first, correctness last.

### Detect And Resolve Duplicates And Conflicts

Duplicates and conflicts are the most damaging content defects because they present contradictory guidance as equally authoritative. Detect duplicates by concept, not by surface form: two entries with different source terms but the same definition and overlapping scope are likely the same concept split across records. Detect conflicts by comparing equivalents for the same concept across entries and across domains, and by checking that deprecated terms do not appear as preferred in another entry.

Resolve by merging true duplicates into one concept entry, preserving the richest definition and context and recording the merge. Resolve conflicts by applying the authority structure: determine which entry has the stronger provenance and recast the other as admitted, deprecated, or forbidden. Never leave contradictory entries both marked preferred.

### Govern The Term Lifecycle With Explicit Status Transitions

Terms move through a lifecycle, and governance means controlling those transitions rather than letting status drift. Define the allowed statuses, typically candidate, proposed, preferred, admitted, deprecated, and forbidden, and define the rules for moving between them. A candidate becomes proposed when a terminologist reviews it; proposed becomes preferred when the authority approves it; preferred becomes deprecated when usage or domain change supersedes it; deprecated may become forbidden if it must never be used.

Enforce transitions through workflow, not through goodwill. An entry's status should change only through a recorded action with an actor, a reason, and a date. Without enforced transitions, statuses accumulate inaccurately, and translators cannot trust that preferred actually means currently preferred.

### Define Authority And Approval Workflow

Governance requires named authority. Define who can propose terms, who reviews them, who approves them, and who can deprecate or forbid. In regulated domains, tie approval to subject-matter experts or regulatory authorities, not to the terminologist alone. Define target turnaround for proposals, because slow approval leaves translators working without guidance and tempts them to invent terms.

Record the approver and the basis for every approved entry, so challenges can be resolved against the authority structure rather than by opinion. A termbase whose approvals are anonymous and undated cannot be governed, because no decision can be traced or revisited.

### Reconcile The Termbase Against Real Usage

A termbase that diverges from how the organization actually writes is a liability. Periodically reconcile the termbase against live content: search current translations and source material for terms that appear often but are absent from the termbase, and for terms marked preferred that are rarely or never used in practice. The first reveals gaps; the second reveals entries that are nominally authoritative but operationally dead.

Reconciliation also catches the reverse: deprecated terms still appearing in active content, which signals that updates were not propagated. Feed reconciliation findings back into governance as proposals, retirements, or propagation tasks.

### Propagate Changes And Track Alignment

When governance changes a term's status, the change must propagate, or the termbase and the live content fall out of alignment. When a preferred term is deprecated, plan a review or find-and-replace pass across existing content, and track which content has been aligned to which termbase version. Without propagation tracking, some pages use the new term and others the old one, and users see inconsistency that the termbase was meant to prevent.

Maintain a version or alignment record so stale content is identifiable and so governance can report coverage rather than assuming alignment.

### Assign Ownership So Governance Happens

Governance that is everyone's responsibility is no one's. Assign explicit ownership: a terminologist or terminology lead who owns validation cycles, status transitions, and reconciliation. Give the owner the authority to enforce workflow and the time budget to maintain the resource. A termbase with a named owner and a scheduled review cadence stays accurate; one maintained only when someone happens to notice a problem decays.

## Common Traps

### Reviewing Content Before Structure

Auditing entries against a schema-violating termbase produces unreliable findings. Validate structure first.

### Leaving Contradictory Entries Both Preferred

Two entries prescribing different equivalents for the same concept, both marked preferred, present contradiction as authority. Merge or recast by provenance.

### Letting Status Drift Without Workflow

When statuses change informally, preferred no longer means currently preferred, and translators lose trust. Enforce transitions with recorded actions.

### Anonymous Or Undated Approvals

Decisions that cannot be traced cannot be challenged or revisited, so governance collapses into opinion. Record approver, basis, and date.

### Never Reconciling Against Real Usage

A termbase that ignores how the organization writes accumulates dead preferred terms and misses active unrecorded terms. Reconcile periodically.

### Changing Status Without Propagating

Deprecating a term without updating existing content leaves old and new terms coexisting, the exact inconsistency the termbase prevents. Propagate and track alignment.

### Governance By General Responsibility

When no one owns validation and lifecycle control, the termbase decays unnoticed. Assign a named owner with authority and budget.

### Treating Validation As One-Time Setup

A termbase validated once and never again becomes authoritative-looking and wrong. Treat validation and governance as recurring obligations.

## Self-Check

Before treating a termbase as validated and governed, verify:

- Structural validation confirmed the termbase obeys its schema: correct field scopes, controlled categorical values, required fields present, and valid cross-references.
- Content audit covered completeness, consistency, and correctness, in that order, and incomplete or inconsistent entries were resolved before correctness review.
- Duplicates were detected by concept and merged, and conflicts were resolved by authority rather than left as contradictory preferred entries.
- The term lifecycle has defined statuses and enforced transitions, with every status change recorded by actor, reason, and date.
- Authority and approval workflow are defined with named roles, subject-matter or regulatory involvement where required, and target turnaround for proposals.
- Every approved entry carries a recorded approver and basis, so decisions are traceable and challengeable.
- The termbase has been reconciled against live content to find gaps, dead preferred terms, and deprecated terms still in use.
- Status changes are propagated to existing content, with an alignment record showing which content matches which termbase version.
- A named owner with authority and budget is responsible for validation cycles and lifecycle governance.
- No entry is both stale and preferred, and no preferred term contradicts another preferred term for the same concept.
