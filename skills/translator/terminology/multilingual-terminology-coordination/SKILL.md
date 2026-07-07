---
name: multilingual_terminology_coordination.md
description: Use when the agent is coordinating terminology across multiple languages and translator teams, maintaining a shared multilingual termbase, resolving terminology conflicts between locales, enforcing term consistency across vendors and projects, or governing terminology decisions in an ongoing multilingual translation program.
---

# Multilingual Terminology Coordination

Terminology consistency within a single language is hard enough. Coordinating terminology across many languages, translator teams, vendors, and projects is an order of magnitude harder, and it is where multilingual programs most often break down. Each locale team makes decisions in isolation, the shared termbase drifts, the same source concept acquires different conceptual treatments in different languages, conflicts between locales go unresolved, and the termbase becomes a graveyard of contradictory entries that no one trusts. The result is a product that uses inconsistent terminology across markets, reviewers who argue over terms that should have been settled once, and translators who ignore the termbase because it is unreliable. Multilingual terminology coordination is the governance work that prevents this: it treats terminology as a shared asset governed by consistent concept-based logic, enforced across all locales, and maintained as a living resource. Without coordination, each language reinvents terminology, and the program pays for that reinvention in inconsistency and rework forever.

Use this skill when coordinating terminology across languages, maintaining a shared multilingual termbase, resolving cross-locale conflicts, enforcing consistency across teams, or governing terminology in an ongoing program. The goal is to make terminology a coherent, trusted, shared asset rather than a source of conflict and drift.

## Core Rules

### Build Terminology Around Concepts, Not Words

A multilingual termbase must be organized around concepts, not around source-language words. Concept-based structure is what allows many languages to share one entry without confusion.

Each termbase entry should represent one concept, defined clearly with its domain, scope, and boundaries. The source term is one designation of that concept, not the concept itself. Each target language then records its own equivalent designation for that same concept. When entries are word-based rather than concept-based, the same concept scatters across multiple entries, synonyms proliferate, and languages attach to different surface forms, producing inconsistency. When entries are concept-based, all languages anchor to the same meaning, and equivalence is verifiable.

Define the concept once, then attach designations in each language. This is the foundation of multilingual coordination.

### Establish Centralized Governance With Distributed Input

A multilingual termbase needs both centralized governance and distributed input, and the balance between them determines whether the termbase stays coherent.

Centralize the decisions that affect all locales: the concept definitions, the entry structure, the status and approval workflow, the naming conventions, and the conflict-resolution authority. Distribute the input to the locale teams who know their language and domain: each locale proposes and validates equivalents for its language. A termbase governed only centrally lacks locale expertise; one fed only by distributed input without coordination fragments into inconsistent entries.

Define who can propose, who validates, who approves, and who has final authority, and make the workflow explicit so entries move through it consistently.

### Enforce A Consistent Entry Structure

Every termbase entry, across all languages, must follow a consistent structure, or the termbase becomes unusable. Define and enforce the structure.

A robust entry includes the concept identifier, the concept definition, the domain, the source term, the target term per language and locale, the part of speech, the term status such as preferred, admitted, or deprecated, the source citation or evidence, context examples, and notes such as usage restrictions or regional variants. When entries omit fields or fill them inconsistently, users cannot trust or compare them. Enforce required fields at entry time, and review legacy entries for compliance.

A termbase whose entries are structurally inconsistent is treated as unreliable and bypassed by translators.

### Resolve Cross-Locale Conflicts Deliberately

Conflicts between locales are inevitable and must be resolved through a deliberate process, not ignored. A conflict left unresolved propagates inconsistency.

Conflicts arise when locales propose different conceptual treatments, when one locale's equivalent denotes a different concept, when regional variants within a language diverge, or when a source term is ambiguous and locales resolve it differently. Establish a resolution process: identify the conflict, examine the concept definitions, consult the affected locale teams, decide based on conceptual accuracy and domain authority, and update all affected entries. Record the resolution and its rationale so the same conflict does not recur.

Ignoring conflicts lets them compound; resolving them deliberately keeps the termbase coherent.

### Maintain Term Status And Lifecycle

Terms have a lifecycle. A termbase that records only current terms, without status, becomes cluttered with deprecated and competing variants. Manage term status explicitly.

Use statuses such as preferred, admitted, deprecated, and provisional. Mark a term preferred when it is the sanctioned equivalent; admitted when it is acceptable but not preferred; deprecated when it should no longer be used; and provisional when it is proposed but not yet validated. When a term's status changes, update the entry rather than deleting the old term, because deprecated terms must remain visible so translators know to avoid them and reviewers know they were once used.

Without lifecycle management, the termbase accumulates competing terms with no signal about which to use.

### Enforce Terminology Across Vendors And Projects

A termbase creates value only if it is actually used. Enforce terminology across all vendors, projects, and translation rounds.

Provide the termbase to every translator and vendor at the start of every job, and require terminology compliance as an acceptance criterion. Check deliverables against the termbase during review, and flag deviations. Where a translator deviates for a good reason, capture the decision and either update the termbase or record the exception. A termbase that is maintained but not enforced is ignored, and the effort spent maintaining it yields no consistency benefit.

Enforcement closes the loop between terminology governance and terminology outcomes.

### Synchronize Terminology With Source And Product Evolution

Products and source content evolve, and terminology must evolve with them. A termbase that is not synchronized becomes stale and misleading.

Establish a process to detect new source terms, changed concepts, and deprecated terms as the source evolves. New concepts require new entries with equivalents across locales. Changed concepts require updated definitions and re-validation of equivalents. Deprecated source terms require status updates. Assign responsibility for ongoing terminology maintenance, because without it the termbase decays and translators stop trusting it.

Terminology coordination is a continuous governance activity, not a one-time setup.

### Communicate Terminology Decisions To All Stakeholders

Terminology decisions affect translators, reviewers, vendors, content owners, and sometimes engineers and product managers. Communicate decisions so they are adopted.

When a new term is approved, a conflict is resolved, or a status changes, communicate it to all affected stakeholders with the rationale. Maintain a changelog or release notes for the termbase so stakeholders can see what changed and why. Decisions made silently are not adopted, and stakeholders continue using outdated terms.

## Common Traps

### Organizing The Termbase Around Words Instead Of Concepts

Word-based entries scatter concepts, create synonyms, and make cross-language equivalence unverifiable.

### Governing Only Centrally Or Only Locally

Central-only governance lacks locale expertise; distributed-only input fragments the termbase. Both are needed.

### Inconsistent Entry Structure

Entries missing fields or filled inconsistently make the termbase unusable and untrustworthy.

### Ignoring Cross-Locale Conflicts

Unresolved conflicts propagate inconsistency and recur each time the concept appears.

### Omitting Term Status And Lifecycle

Without status, preferred, admitted, deprecated, the termbase fills with competing variants and gives no signal which to use.

### Maintaining The Termbase Without Enforcing It

A termbase not enforced at acceptance is ignored, and maintenance effort yields no consistency.

### Letting Terminology Decay With Source Evolution

Without ongoing synchronization, the termbase becomes stale and translators stop trusting it.

### Making Decisions Silently

Terminology decisions not communicated are not adopted, and stakeholders keep using outdated terms.

## Self-Check

Before approving multilingual terminology coordination for a program, verify:

- The termbase is organized around concepts, with each entry representing one defined concept and designations attached per language.
- Governance balances centralized authority over structure and conflicts with distributed locale input for equivalents.
- A consistent entry structure is defined and enforced, with required fields filled across all entries and languages.
- Cross-locale conflicts are identified and resolved through a deliberate process with recorded rationale.
- Term status and lifecycle are managed, with preferred, admitted, deprecated, and provisional terms visible and updated.
- Terminology is enforced across vendors and projects, with compliance checked at acceptance and deviations captured.
- A process synchronizes terminology with source and product evolution, with responsibility assigned for ongoing maintenance.
- Terminology decisions are communicated to all stakeholders with rationale and recorded in a changelog.
- No locale is making terminology decisions in isolation, and no entry lacks a concept definition.
- The termbase is trusted, current, enforced, and coherent across all languages in the program.
