---
name: terminology_consistency_and_variant_control.md
description: Use when the agent is enforcing terminology consistency within a single target language, managing allowed and forbidden term variants, controlling synonyms and competing renderings, resolving cases where multiple acceptable terms exist for one concept, or auditing translated content for terminology drift and inconsistency against the termbase.
---

# Terminology Consistency And Variant Control

A termbase that defines the correct term for each concept is necessary but not sufficient. The recurring problem in real translation work is that the correct term is defined, yet the translated content still contains variants: synonyms, alternative renderings, deprecated forms, and competing terms that creep in through different translators, different projects, or machine translation. Terminology consistency and variant control is the discipline of ensuring that once a term is chosen for a concept, that term, and only that term, is used throughout the content, and that competing variants are identified, classified, and either permitted, deprecated, or forbidden. Without variant control, a termbase becomes a reference document that content ignores, and the user experience fragments: the same feature is called three different names across the product, the documentation, and the support articles, and the user cannot tell whether they refer to the same thing.

Agents tend to miss that consistency is enforced through variant management, not just term definition, that some variants are legitimately allowed (regional, register-based) while others are errors, that machine translation is a major source of uncontrolled variants, and that consistency must be audited against the termbase rather than assumed. The harm is content that looks professional in each segment but is inconsistent across the whole, eroding user trust and making content harder to search, index, and maintain.

Use this skill when enforcing terminology consistency within a target language, managing term variants, controlling synonyms, resolving competing renderings, or auditing content for terminology drift. The goal is to ensure that the chosen term is used consistently and that variants are managed deliberately rather than allowed to accumulate.

## Core Rules

### Enforce Consistency Through Variant Management, Not Just Term Definition

Defining the preferred term in the termbase is the first step; enforcing its use is the ongoing work. A termbase entry should not only state the preferred term but also list the variants that may appear, classifying each as allowed, deprecated, or forbidden. This classification transforms the termbase from a passive reference into an enforcement tool.

For each concept, document: the preferred term (the one that should be used), allowed variants (acceptable alternatives, such as regional variants or register differences), deprecated variants (formerly used terms that should no longer appear, included so they can be detected and corrected), and forbidden variants (wrong terms, mistranslations, or terms that denote a different concept, included so they can be flagged as errors). When auditing content, check against all classifications: preferred terms pass, allowed variants pass with possible notes, deprecated variants trigger correction, and forbidden variants trigger error flags. A termbase without variant classification cannot enforce consistency because it cannot distinguish an acceptable alternative from an error.

### Classify Variants By Reason To Determine Handling

Not all variants are errors. Some are legitimate and should be permitted; others are errors and should be corrected. Classify each variant by the reason it exists to determine the correct handling.

Regional variants: a term that is correct in one locale variant of the language but not another (for example, a computing term that differs between European and Latin American Spanish). These are allowed within their locale but should be controlled so the correct regional variant is used for each target locale. Register variants: a term that is appropriate in formal content but not in informal, or vice versa. These are allowed within their register context. Context-dependent variants: a term that is correct in one product area or document type but not another. These are allowed within their context. Deprecated variants: terms that were once preferred but have been superseded. These should be corrected to the current preferred term. Error variants: mistranslations, wrong concepts, or terms that were never correct. These must be corrected. Classifying by reason ensures that legitimate variants are not wrongly "corrected" and that real errors are not dismissed as acceptable variation.

### Control Synonyms And Competing Renderings Deliberately

When multiple acceptable terms exist for the same concept, the choice among them must be deliberate, not left to individual translator preference. Uncontrolled synonymy, where each translator picks their preferred term, produces content where the same concept appears under different names, confusing users and fragmenting search and indexing.

For each concept with competing acceptable terms, make a decision: designate one as the preferred term and the others as allowed variants (if regional or contextual differences justify them) or deprecated variants (if the goal is to standardize on one). Document the decision and the rationale. When a new translator or machine translation engine introduces a competing rendering, check it against the termbase: if it is a forbidden or deprecated variant, correct it; if it is a new variant not yet classified, evaluate it and add it to the termbase with a classification. Do not allow unclassified variants to accumulate in the content; each new variant is a termbase decision waiting to be made.

### Audit Content For Terminology Drift Against The Termbase

Consistency is not maintained by hope; it is maintained by audit. Terminology drift, the gradual accumulation of variants that differ from the termbase, occurs through translator turnover, machine translation, content updates, and simple human error. Regular audits compare the terms used in the content against the termbase and flag deviations.

Conduct terminology audits using automated terminology extraction and comparison tools that scan translated content, identify terms, and match them against termbase entries. Automated audits catch forbidden and deprecated variants at scale. Supplement with sampling-based human review for cases that automated tools miss, such as variants that are morphologically different from the termbase entry or that appear in multi-word phrases. Report audit findings as a terminology consistency score (percentage of terms matching the termbase) and a list of deviations with their classifications. Feed repeated deviations back into the termbase review: if a forbidden variant appears frequently, it may indicate that the preferred term is unintuitive and that the termbase decision should be revisited, or that translators need training on the decision.

### Manage Machine Translation As A Variant Source

Machine translation engines are a major source of uncontrolled terminology variants. An MT engine translates the same source term differently in different contexts, produces variants that do not match the termbase, and may use terms that are regionally or stylistically inappropriate. Deploying MT without terminology control produces content with high variant density.

Integrate the termbase with the MT engine so that termbase terms are enforced in the output. Most modern MT and post-editing environments support terminology enforcement, where the engine is constrained to use termbase terms for defined concepts. Where enforcement is not fully effective, use post-editing terminology checks that flag deviations from the termbase for the post-editor to correct. Do not treat MT output as terminology-compliant by default; audit it against the termbase just as human translation is audited.

### Handle Variant Conflicts Between Content Types And Products

Terminology consistency must extend across content types and products within the same brand or product family. A feature called one name in the user interface, another in the documentation, and a third in the support articles creates user confusion even if each individual content type is internally consistent. Cross-content-type variant control ensures the same concept uses the same term everywhere the user encounters it.

Establish a master termbase that governs all content types for a product or brand. When different content teams (UI, documentation, marketing, support) have historically used different terms, reconcile them to a single preferred term, or document allowed variants by content type if the differences are justified (for example, marketing may use a friendlier term than technical documentation, but this should be a deliberate decision, not an accident). Audit across content types, not just within one. Cross-content consistency is harder to achieve because it requires coordination across teams that may not communicate regularly, but it is essential for a coherent user experience.

### Maintain A Forbidden Term List For Active Enforcement

A forbidden term list is a specialized termbase component that lists terms that must never appear, along with the reason (mistranslation, wrong concept, offensive or inappropriate in the target culture, deprecated brand name, legal risk). The forbidden term list is used for active enforcement: automated checks flag any occurrence, and translators and post-editors are alerted to avoid or correct them.

Maintain the forbidden term list alongside the preferred and allowed terms. When a forbidden term is detected in content, the correction is mandatory, not optional. Update the forbidden term list when new mistranslations or inappropriate variants are discovered, so the same error is not repeated. A forbidden term list is especially valuable for content with cultural sensitivity, brand consistency requirements, or legal constraints, where using the wrong term has consequences beyond inconsistency.

## Common Traps

### Defining Terms Without Classifying Variants

A termbase that lists only preferred terms cannot distinguish an acceptable variant from an error. Classify each variant as allowed, deprecated, or forbidden to enable enforcement.

### Treating All Variants As Errors

Some variants are legitimate (regional, register-based, contextual). Correcting legitimate variants to the preferred term when the variant is appropriate for the locale or context produces unnatural or wrong text. Classify by reason before correcting.

### Allowing Synonyms To Accumulate Without Decision

When multiple acceptable terms exist and no decision is made, each translator picks their preference, fragmenting the content. Make a deliberate decision and document it.

### Assuming Consistency Without Auditing

Terminology drift accumulates silently. Without regular audits against the termbase, inconsistencies grow undetected. Audit content, report consistency scores, and feed findings back into termbase review.

### Deploying MT Without Terminology Enforcement

MT engines produce uncontrolled variants. Without termbase integration and post-editing terminology checks, MT output has high variant density and low consistency.

### Ignoring Cross-Content-Type Consistency

Consistency within one content type is insufficient if the same concept is named differently across UI, documentation, marketing, and support. Establish a master termbase and audit across all content types.

### Letting Unclassified Variants Accumulate

Each new variant introduced by a translator or MT engine is a termbase decision. Allowing unclassified variants to accumulate in content without evaluating and classifying them erodes the termbase's authority.

## Self-Check

- [ ] Does each termbase entry classify variants as preferred, allowed, deprecated, and forbidden, enabling enforcement rather than passive reference?
- [ ] Has each variant been classified by reason (regional, register, contextual, deprecated, error) to determine whether it should be permitted or corrected?
- [ ] Have competing acceptable terms for the same concept been resolved through a deliberate decision, with the rationale documented?
- [ ] Is content regularly audited against the termbase using automated extraction and comparison, with a terminology consistency score and deviation list reported?
- [ ] Is machine translation integrated with the termbase for terminology enforcement, with post-editing checks flagging deviations?
- [ ] Does a master termbase govern terminology across all content types (UI, documentation, marketing, support) for the product or brand?
- [ ] Is a forbidden term list maintained and actively enforced, with mandatory correction of any occurrence?
- [ ] Are repeated deviations fed back into termbase review to determine whether the preferred term should be revisited or translators need training?
- [ ] Is the variant control process documented with classifications, audit results, and enforcement actions so consistency can be verified and maintained?
