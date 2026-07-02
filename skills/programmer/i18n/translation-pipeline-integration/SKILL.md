---
name: translation_pipeline_integration.md
description: Use when the agent is integrating a translation management system (TMS) into the development workflow, extracting user-facing strings for translation, providing context and screenshots to translators, managing translation memory, synchronizing translations with release cadence, or maintaining translation quality as the product and codebase evolve.
---

# Translation Pipeline Integration

Translating a product is not a one-time task of swapping English strings for localized ones; it is a continuous pipeline that must stay synchronized with a codebase that changes every day. The failure mode is treating translation as a finish-line activity ("we will localize before launch") rather than as an integrated workflow, which produces a predictable set of problems: translators receive strings with no context and produce wrong translations, new UI strings ship untranslated because the extraction step was forgotten, translations for removed strings linger forever, and releases slip because translation was not on the critical path until it suddenly blocked everything. Agents often focus on the string-format mechanics (ICU MessageFormat, JSON catalogs) and miss that the real difficulty is the human and process loop between developers, translators, and the codebase.

The judgment problem is that translation quality depends on context that developers have and translators lack, and that the pipeline must handle continuous change without falling behind. A translator given the string "Save" with no context cannot know whether it is a verb (the action) or a noun (a saved file), and will guess wrong some percentage of the time. A pipeline that extracts strings once and never re-syncs will ship English to localized users whenever a developer adds a button. The agent must build extraction into the build so strings cannot be missed, provide context (screenshots, comments, string location) so translators translate accurately, use translation memory and glossaries for consistency and cost, and synchronize the translation cycle with the release cycle so localization is never the unexpected blocker. A translation pipeline is a product workflow, not a file format.

## Core Rules

### Make string extraction automatic and part of the build

The most common localization defect is a user-facing string that was never extracted for translation, so it ships in the source language to all users. Prevent this mechanically: wire string extraction into the build or CI so that every user-facing string in the codebase is captured into the translation catalog automatically, and make the build fail when a new string appears that has not been submitted for translation (or at least flag it for review). Manual extraction ("remember to add new strings to the catalog") will be forgotten under deadline pressure. Treat the catalog as a generated artifact that must stay in sync with the code, and detect drift automatically.

### Provide context with every string, because translators cannot see your UI

Translators work from strings, not from the running product, and a string alone is often ambiguous. "Save" could be a verb or a noun. "New" could mean "create" or "recent." "Free" could mean "no cost" or "unrestricted." Provide context with every string: a description of where it appears and what it means, the part of speech or role, max length constraints (UI space is finite and German translations are longer), and ideally a screenshot of the string in context. Modern TMS integrations can attach screenshots and metadata automatically. The cost of providing context is small; the cost of wrong translations reaching users (and being re-translated, re-QA'd, and re-shipped) is large.

### Use translation memory and glossaries for consistency and cost control

Translation memory (TM) remembers previously translated strings and suggests them when the same or similar string reappears, ensuring consistency ("Sign In" is always translated the same way) and reducing cost (repeated strings are not re-translated from scratch). Glossaries define mandatory term translations (product names, domain terms, brand voice) so all translators use the same vocabulary. Invest in both from the start, because consistency problems compound as the string count grows, and re-translating inconsistent strings to harmonize them later is expensive. Treat the TM and glossary as long-lived assets that improve with every translation cycle.

### Synchronize the translation cycle with the release cycle

Translation must be on the critical path by design, not discovered as a blocker at the end. If the release ships every two weeks but translation takes ten days, the translation cycle must start before code freeze, not after. Define the workflow: when strings are frozen for translation, how long translation and review take, when localized strings must be back and merged, and how to handle strings added after the freeze (defer to next release, or fast-track). Build the translation lead time into the release schedule explicitly. A release that is "done" except for translation is not done, and treating translation as someone else's problem guarantees it slips the date.

### Handle string changes, removals, and re-use correctly

Strings are not immutable; they change. When a source string is edited, its existing translations are invalidated (the meaning may have changed) and must be re-translated, though translation memory can suggest the prior translation as a starting point. When a string is removed from the UI, remove it from active translation but preserve it in translation memory for potential re-use. When the same source string appears in two contexts with different meanings (the "Save" verb/noun problem), treat them as separate strings with separate keys and separate context, even though the English is identical, because the translations may differ in other languages. Do not key translations by the English text; key by a stable identifier so that editing English does not break the link to existing translations.

### Never compose translated strings by concatenation

Building localized strings by concatenating fragments ("You have " + count + " unread messages") breaks in languages with different word order, agreement, or grammar, and it makes translation impossible because the translator never sees the full sentence. Use complete sentences with placeholders via a proper message format (ICU MessageFormat is the standard), so the translator translates the whole sentence with the variable in the right place for their language. This also enables correct pluralization and gender handling within the same message. Concatenation is the most common and most damaging localization anti-pattern; eliminate it entirely.

### Test localized builds, not just the source-language build

A localized build can break in ways the source build cannot: translated strings that overflow their UI container (German is long, CJK can be wide), right-to-left layout issues, fonts that lack glyphs for the target script, date/number/currency formatting that ignores locale, and strings that were never translated at all. Build and run localized builds in CI or pre-release QA, at least for a representative set of locales (a long one like German, an RTL one like Arabic, a CJK one like Japanese). Visual QA of localized layouts catches defects that string-level review cannot.

## Common Traps

### Manual string extraction that misses new strings

Relying on developers to remember to add strings to the catalog guarantees untranslated strings ship to users. Make extraction automatic and fail the build on un-submitted strings.

### Sending strings to translators with no context

A bare string is often ambiguous, and translators will guess. Provide descriptions, part of speech, length limits, and screenshots so translations are accurate.

### Treating translation as a finish-line activity

Localization discovered as a blocker at code freeze slips the release. Build translation lead time into the schedule and start the cycle before freeze.

### Composing strings by concatenation

Fragment concatenation breaks word order and grammar in other languages and makes translation impossible. Use complete sentences with placeholders via ICU MessageFormat.

### Keying translations by the source English text

Editing the English breaks the link to existing translations. Key by a stable identifier so source text can change without losing translations.

### Same English string used for two different meanings

"Save" as verb and "Save" as noun may need different translations. Use separate keys with separate context when meaning differs, even if English is identical.

### Never testing localized builds

A localized build can overflow UI, break RTL layout, lack glyphs, or ship untranslated strings that the source build never reveals. Build and visually QA representative localized builds.

## Self-Check

- Is user-facing string extraction automatic and part of the build/CI, with the build failing or flagging when a new string has not been submitted for translation?
- Does every string sent for translation include context (description, part of speech, length constraints, screenshot) so translators can translate accurately rather than guess?
- Are translation memory and glossaries in use to ensure consistency and control cost, and treated as long-lived assets?
- Is the translation cycle synchronized with the release cycle, with lead time built into the schedule and a defined workflow for strings added after freeze?
- Are string changes, removals, and same-English-different-meaning cases handled correctly (re-translation on edit, TM preservation on removal, separate keys for distinct meanings)?
- Are translations keyed by stable identifiers rather than by source English text, so editing English does not break the link to existing translations?
- Is string concatenation eliminated in favor of complete sentences with placeholders via ICU MessageFormat, enabling correct grammar and pluralization?
- Are localized builds built and visually QA'd for representative locales (a long language, an RTL language, a CJK language) to catch overflow, layout, glyph, and formatting defects?
