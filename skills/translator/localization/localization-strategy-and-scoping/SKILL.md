---
name: localization_strategy_and_scoping.md
description: Use when the agent is planning a software or product localization effort, deciding which locales and content types to localize, sequencing locale rollouts, choosing between translation and localization, or scoping a localization project before content is handed off to linguists and engineers.
---

# Localization Strategy And Scoping

Localization is the work of adapting a product, not merely its words, so that it functions for users in another locale. It is broader than translation: it covers language, locale-specific formats, legal and regulatory requirements, cultural conventions, visual design, functional behavior, and content architecture. A common failure is to treat localization as a final word-swap applied to finished content, which produces products that are technically translated but locally broken. Dates collapse, currencies display wrong, names do not fit forms, layouts overflow, legal notices are missing, and the product feels foreign. Strategy and scoping happen before any string is translated, because the decisions made here determine whether downstream linguistic and engineering work can succeed at all.

Use this skill when planning a localization effort, selecting target locales, deciding what to localize, sequencing rollouts, estimating scope and effort, or defining the boundary between translation and broader localization work. The goal is to prevent a localization project from starting with undefined scope, missing locale requirements, or a content pipeline that cannot be localized efficiently.

## Core Rules

### Define What Localization Means For This Product

Localization can mean different things depending on the product. Define the scope explicitly before scoping.

Clarify whether the work is language translation only, full locale adaptation including formats and conventions, functional localization including UI behavior and feature adaptation, or market-specific localization including legal, regulatory, and cultural adaptation. A marketing website may need transcreation and cultural adaptation. A medical device interface may need strict terminology, regulatory compliance, and format conversion but minimal creative adaptation. A game may need voice, cultural references, and content rewrites. The scope determines what skills, budget, and timeline are required.

Do not assume localization means the same thing across products. State the adaptation depth and the dimensions covered.

### Select Locales Deliberately, Not By Language Alone

Locale selection drives everything downstream. A locale is not a language; it is a language combined with a region, and the region carries format, legal, and cultural expectations.

Distinguish language from locale. Spanish is not one locale; es-ES, es-MX, es-AR differ in vocabulary, formality, date and number formats, and legal context. English is not one locale; en-US, en-GB, en-IN, en-SG differ in spelling, units, currency, and conventions. Chinese Simplified and Traditional differ in script, region, and sometimes terminology. Selecting a language without a locale produces ambiguous requirements and inconsistent output.

Prioritize locales by business value, user base size, regulatory necessity, and support feasibility. Consider whether a locale is required for market entry, whether it is mandated by regulation, and whether the organization can support customer service and content maintenance in that locale. Avoid localizing into locales with no ongoing support plan, because stale localized content damages trust.

### Assess Content Localizability Before Committing

Not all content is ready to localize. Assess localizability before scoping effort, because fixing non-localizable content during localization is expensive and error-prone.

Check whether the source content is internationalized. Are strings externalized from code, or hardcoded? Are dates, numbers, and currencies formatted through locale-aware functions, or baked in? Does the UI layout accommodate text expansion, or will translated text overflow? Are images locale-neutral, or do they contain embedded text that must be recreated for each locale? Is concatenation avoided, or do sentences assemble from fragments that break in other grammars?

Content that is not internationalized should be remediated first. Localizing non-internationalized content produces broken products and multiplies rework every time the source changes.

### Scope Content Types And Volumes

A localization project spans multiple content types, each with different requirements. Scope each type explicitly.

Identify and quantify the content types involved: UI strings, documentation, help content, marketing pages, legal and policy pages, email templates, in-app messages, error messages, metadata, screenshots, video and audio, app store listings, and support knowledge base. Each type has different constraints: UI strings have character limits and placeholder rules, legal pages have mandated wording, marketing needs transcreation, and video needs timing and subtitles.

Estimate volume per content type and per locale. Volume drives cost, timeline, and resource allocation. Underestimating volume is a primary cause of missed deadlines and budget overruns.

### Account For Locale-Specific Requirements

Each locale carries requirements beyond language. Map these during scoping so they are not discovered late.

Requirements include date, time, number, and currency formats; address and name formats, including field order and required fields; calendar systems where relevant; week start and weekend conventions; legal and regulatory content requirements, such as mandatory privacy notices, cookie banners, or consumer disclosures; right-to-left layout for Arabic and Hebrew locales; character set and font support; and measurement unit conversion where the locale uses different units.

Missing a locale-specific requirement late in the project causes rework across content, engineering, and design.

### Sequence Locale Rollouts Strategically

Localizing into many locales at once is risky. Sequence rollouts to manage risk and learn from early locales.

A common approach is a pilot locale, often one with high business value but manageable complexity, followed by expansion. The pilot reveals process gaps, tooling problems, and content issues that are cheaper to fix before scaling. Avoid launching all locales simultaneously unless the product is simple and the pipeline is proven, because a defect discovered after a full multi-locale launch must be fixed across every locale.

Consider linguistic and cultural distance when sequencing. A locale close to the source language reveals fewer structural issues than a distant one. Include at least one structurally different locale early, such as a right-to-left or CJK locale, to surface layout and engineering problems before they propagate.

### Define The Localization Workflow And Tooling

Scoping includes defining how content moves from source to localized deliverable. The workflow determines efficiency and consistency.

Decide on the translation management system or CAT tooling, the termbase and translation memory setup, the machine translation integration if any, the review and QA stages, and the handoff and handback process between content owners, localization managers, linguists, and engineers. Define where content is stored, how versions are tracked, how updates are propagated to localized versions, and how signoff works.

A workflow designed during scoping prevents the chaos of ad hoc file exchanges, lost updates, and inconsistent terminology that plagues unplanned localization.

### Estimate Cost, Effort, And Timeline Honestly

Localization cost is driven by volume, language pair, content complexity, review depth, and engineering work. Estimate honestly, including hidden costs.

Hidden costs include internationalization remediation, image and screenshot recreation, linguistic testing, engineering fixes for layout and concatenation, project management, and ongoing maintenance of localized content as the source evolves. Maintenance cost is often underestimated: localized content must be updated whenever the source changes, and the update pipeline must be sustained, not treated as one-time.

Build contingency into timelines, especially for first-time locales and complex content. Localization consistently takes longer than expected when internationalization is incomplete or requirements are unclear.

### Plan For Ongoing Localization, Not A One-Time Project

Products evolve, and localized content must evolve with them. Scope localization as a continuous capability, not a single launch.

Define how source content updates trigger localization updates, how frequently locales are refreshed, who owns localized content quality over time, and how terminology and translation memory are maintained across releases. Without an ongoing plan, localized content drifts out of sync with the source, terminology erodes, and the product degrades in every locale except the source.

## Common Traps

### Treating Localization As A Final Word-Swap

Applying translation to finished, non-internationalized content produces broken products. Localizability must be designed in before content is finalized.

### Selecting Language Without Locale

Requesting Spanish or Chinese without a region produces ambiguous requirements and inconsistent output. Always specify language plus region.

### Localizing Into Locales With No Support Plan

Localized content that is never updated or supported erodes trust and signals neglect. Do not localize into a locale you cannot sustain.

### Ignoring Text Expansion

Translations are often longer than the source. Layouts designed for the source language overflow, truncate, or break when localized.

### Launching All Locales Simultaneously

A defect discovered after a full multi-locale launch must be fixed everywhere. Sequence rollouts to learn from early locales.

### Underestimating Maintenance Cost

Localized content must track source updates indefinitely. Failing to plan for ongoing maintenance causes drift and quality decay.

### Assuming Locale Requirements Transfer

Date formats, legal notices, name fields, and measurement units differ by locale. Assuming they transfer from the source produces locally broken products.

### Skipping Internationalization Assessment

Localizing content that is not internationalized multiplies rework. Assess and remediate internationalization before scoping localization effort.

## Self-Check

Before approving a localization strategy and scope, verify:

- The meaning and depth of localization for this product is defined, including which dimensions are in scope.
- Target locales are selected by language plus region, prioritized by business value, regulatory need, and support feasibility.
- Content localizability was assessed, with internationalization gaps identified and remediation planned before localization.
- Content types and volumes are identified and quantified per locale, including UI, documentation, marketing, legal, and multimedia.
- Locale-specific requirements, formats, legal content, layout direction, and unit conversion are mapped for each target locale.
- Locale rollouts are sequenced to manage risk, with at least one structurally different locale included early to surface engineering issues.
- The localization workflow, tooling, termbase, translation memory, review stages, and handoff process are defined.
- Cost, effort, and timeline estimates include hidden costs: internationalization, testing, engineering, project management, and ongoing maintenance.
- An ongoing localization plan exists for propagating source updates to localized content and maintaining quality over time.
- No locale is being launched without a support and maintenance plan, and no non-internationalized content is being localized as-is.
