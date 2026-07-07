---
name: localization_architecture_and_internationalization_readiness.md
description: Use when the agent is assessing whether a product is ready for localization, evaluating internationalization architecture, identifying hard-coded strings and locale-unfriendly design, planning resource extraction and string externalization, or determining what engineering work is required before translation can begin so that localized products function correctly across locales.
---

# Localization Architecture And Internationalization Readiness

Localization cannot fix what internationalization failed to build. Internationalization (i18n) is the engineering work that makes a product adaptable to multiple locales without code changes: externalizing user-facing strings, supporting different character sets and text directions, handling date time and number formats, accommodating text expansion and contraction, and separating locale-specific logic from core code. When internationalization is done poorly or not at all, localization becomes a cycle of engineering rework: translated strings break layouts, non-Latin characters corrupt displays, concatenated strings produce grammatical nonsense in languages with different word order, and every new locale requires code changes. Assessing internationalization readiness before localization begins is the difference between a smooth rollout and an expensive cycle of defects.

Agents tend to miss that localization readiness is an engineering property that must be assessed before translation starts, that hard-coded strings and locale assumptions are the most common blockers, that text expansion and bidirectional support are architectural decisions not afterthoughts, and that skipping the readiness assessment leads to defects that are expensive to fix because they require code changes after translation is underway. The harm is a localization project that appears to start on time but stalls when translated content breaks the product, requiring rework that delays launch and erodes quality.

Use this skill when assessing product readiness for localization, evaluating internationalization architecture, identifying hard-coded strings and locale assumptions, planning string externalization, or determining the engineering work required before translation. The goal is to ensure the product is architecturally ready to be localized before translation begins, preventing defects that require costly engineering rework.

## Core Rules

### Assess Internationalization Readiness Before Starting Localization

Before any content is handed off for translation, assess whether the product is architecturally ready to receive localized content. This assessment is an engineering review, not a content review, and it identifies the work that must be done before localization can proceed without breaking the product.

The readiness assessment covers: string externalization (are all user-facing strings extracted from code into resource files?), character encoding (does the product support Unicode and non-Latin scripts?), text layout (does the UI accommodate text expansion and contraction?), locale formatting (are dates, times, numbers, and currencies formatted according to locale rather than hard-coded?), text direction (does the product support right-to-left languages if needed?), and locale logic (are locale-specific rules, such as sorting or pluralization, handled through locale data rather than code?). Document the findings as a readiness report with pass/fail for each dimension and a remediation plan for failures. Do not begin localization until critical readiness issues are resolved, or the translated content will break the product.

### Identify And Externalize Hard-Coded Strings

Hard-coded strings, user-facing text embedded directly in source code rather than in resource files, are the most common internationalization failure. A hard-coded string cannot be translated without modifying the code, which means every locale requires code changes, and every string update requires engineering effort. Before localization, audit the codebase for hard-coded strings and externalize them into resource files that translators can work with.

The audit should identify strings in UI labels, button text, error messages, tooltips, notifications, email templates, and any other user-facing text. Watch for strings that are partially hard-coded (a label assembled from code and text, such as "Welcome, " + username), which break when translated because the word order differs across languages. Watch for strings embedded in images, which cannot be translated at all without recreating the image. Externalize all user-facing strings into resource files with clear keys, and establish a process that prevents new hard-coded strings from being introduced (such as linting rules that flag string literals in UI code). A product with remaining hard-coded strings is not ready for localization.

### Design For Text Expansion And Contraction

Translated text rarely matches the source text in length. Translations from English into German, French, or Spanish typically expand by 20 to 40 percent. Translations into Asian languages may contract in character count but expand in pixel width due to wider characters. A UI designed to fit English text exactly will break when the translated text overflows: labels truncate, buttons resize inconsistently, layouts shift, and dialog text may be cut off.

Design for expansion from the start. Allow UI elements to grow with their content rather than fixing their dimensions. Use flexible layouts (flow layouts, responsive design) rather than absolute positioning. Test with pseudo-localization, which artificially expands and accents the source text to reveal where the layout breaks before any real translation is done. Provide text expansion guidelines to designers: plan for 30 to 40 percent expansion for European languages, and test layouts at expansion limits. A layout that breaks on expansion is an internationalization defect that must be fixed before localization, because fixing it after translation requires redesigning the UI for every affected screen.

### Handle Locale-Specific Formatting Through Locale Data

Dates, times, numbers, and currencies are formatted differently across locales. A date written as 03/04/2025 means March 4 in the United States and April 3 in most of the rest of the world. A number written as 1,234.56 in English may be written as 1.234,56 in German. These differences must be handled through locale data and formatting libraries, not through hard-coded formats.

Audit the product for hard-coded date, time, number, and currency formats. Replace them with locale-aware formatting that uses the user's locale to determine the format. Ensure that the formatting library supports the target locales and that locale data is complete and current. Watch for formats embedded in strings (such as "on " + date + " at " + time), which break when translated because the sentence structure differs. Use locale-aware templates with placeholders that the formatting library fills according to locale rules. Also handle pluralization rules, which differ across languages (some have more than singular and plural forms), through locale-aware pluralization rather than simple if-singular-else-plural logic.

### Support Character Encoding And Non-Latin Scripts

A product that supports only Latin characters cannot be localized into languages that use Cyrillic, Arabic, Chinese, Japanese, Korean, Thai, or other scripts. Character encoding support is a foundational internationalization requirement: the product must use Unicode (typically UTF-8) throughout, from storage to display to transport, to handle all scripts correctly.

Audit the product for encoding issues: databases, file formats, network protocols, and display layers must all use Unicode consistently. Watch for legacy encoding assumptions, byte-length validation that fails on multi-byte characters, string manipulation that breaks on combining characters or grapheme clusters, and font support for the target scripts. Test with representative text in each target script to confirm it displays, edits, and transmits correctly. Encoding problems are particularly insidious because they may not surface until a specific locale is attempted, and they can corrupt data silently.

### Plan For Bidirectional And Complex Script Support

Languages such as Arabic and Hebrew are written right-to-left (RTL), which requires the UI to mirror its layout: navigation moves to the opposite side, text alignment reverses, and directional icons may need to flip. Complex scripts (such as those requiring character shaping or contextual forms, like Arabic and some Indic scripts) require rendering support beyond simple character display.

If the target locale set includes RTL or complex-script languages, assess whether the product supports bidirectional layout and complex text rendering. For RTL, the layout must mirror logically, not just visually, which requires the UI framework to support RTL mode. Test with actual RTL content to confirm the layout mirrors correctly and that mixed-direction text (such as an English word within Arabic text) renders properly. For complex scripts, confirm the rendering engine handles character shaping, ligatures, and contextual forms. Bidirectional and complex script support is an architectural property that is difficult to retrofit; if the product does not support it and the locale set requires it, significant engineering work is needed before localization can proceed.

### Separate Locale Logic From Core Code

Locale-specific logic, such as sorting order, address formats, name formats, legal disclaimers, and regulatory requirements, should be separated from core code and driven by locale data or configuration. Hard-coding locale logic in code means every new locale requires code changes and testing.

Audit for hard-coded locale assumptions: sorting that assumes Latin alphabetical order, address forms that assume a specific country format, name fields that assume first-name-last-name structure, and business logic that assumes specific regulatory rules. Externalize these into locale-specific configuration or data, so adding a locale is a data change, not a code change. This separation also makes it easier to maintain locale-specific content as regulations and conventions evolve.

## Common Traps

### Starting Localization Before Assessing Readiness

Beginning translation before the product is architecturally ready leads to defects that require code changes mid-project, delaying launch and degrading quality. Assess readiness first.

### Leaving Hard-Coded Strings In The Codebase

Hard-coded strings cannot be translated without code changes. Audit and externalize all user-facing strings before localization, and prevent new ones with linting rules.

### Designing UI For Exact Source Text Length

UIs designed for exact English text length break when translations expand. Design for expansion, use flexible layouts, and test with pseudo-localization.

### Hard-Coding Date Time Number And Currency Formats

Hard-coded formats produce incorrect or ambiguous displays in other locales. Use locale-aware formatting libraries and locale data.

### Assuming Latin Character Support Is Sufficient

A product that supports only Latin characters cannot be localized into most of the world's languages. Use Unicode throughout and test with target scripts.

### Ignoring Bidirectional And Complex Script Requirements

RTL and complex scripts require architectural support that is difficult to retrofit. If the locale set includes these languages, assess and build support before localization.

### Embedding Locale Logic In Core Code

Hard-coded locale logic requires code changes for every new locale. Separate locale-specific rules into configuration or data.

## Self-Check

- [ ] Has an internationalization readiness assessment been conducted before localization begins, covering string externalization, character encoding, text layout, locale formatting, text direction, and locale logic?
- [ ] Have all hard-coded user-facing strings been identified and externalized into resource files, with a process (such as linting) to prevent new hard-coded strings?
- [ ] Does the UI design accommodate text expansion and contraction, with flexible layouts and pseudo-localization testing to verify layouts at expansion limits?
- [ ] Are date, time, number, and currency formats handled through locale-aware formatting libraries and locale data, rather than hard-coded?
- [ ] Does the product use Unicode (UTF-8) throughout, with consistent encoding across storage, display, and transport, tested with target scripts?
- [ ] If the locale set includes RTL or complex-script languages, does the product support bidirectional layout and complex text rendering, tested with actual content?
- [ ] Is locale-specific logic (sorting, address formats, regulatory rules) separated from core code into locale configuration or data?
- [ ] Has a readiness report been produced with pass/fail for each dimension and a remediation plan for failures, with critical issues resolved before localization begins?
- [ ] Is the internationalization architecture documented with string externalization scheme, encoding standard, layout strategy, formatting approach, and locale logic separation so engineering readiness can be verified?
