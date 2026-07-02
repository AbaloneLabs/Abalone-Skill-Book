---
name: pluralization_and_gender.md
description: Use when the agent is formatting messages with variable counts or quantities, implementing plural forms, handling CLDR plural rules (one, few, many, other), dealing with grammatical gender in localized strings, using ICU select or choice expressions, building message templates with placeholders, or testing pluralization and gender across diverse languages.
---

# Pluralization and Gender

Pluralization and gender are where naive localization breaks, because English is unusually simple in both and engineers generalize from it. English has two plural forms (one and other), so the common pattern is `"item" + (count == 1 ? "" : "s")`, and this works in English and fails almost everywhere else. Arabic has six plural forms. Polish and Russian have three with complex rules. Many languages mark grammatical gender on nouns, adjectives, and verbs, so a single sentence may need different translations depending on the gender of the subject. An agent who handles plurals with an if/else and gender with a hardcoded assumption ships code that is subtly wrong in most of the world's languages, and the wrongness is invisible until a localization bug report arrives from a user to whom the sentence is ungrammatical or nonsensical.

The judgment problem is that pluralization and gender are grammar, not formatting, and they interact with the rest of the sentence in language-specific ways. You cannot pick a plural form in isolation and paste it into a sentence, because the form may depend on the number, the noun's gender, the surrounding words, or all three. The correct approach is to use a message format that lets the translator write the full sentence for each plural form and each gender variant, with the logic expressed in the message (ICU MessageFormat) rather than in the application code. The agent must avoid the temptation to handle these in code (which cannot know the target language's grammar) and must test against languages that expose the failure, not just against English and one other language where the bug happens to hide.

## Core Rules

### Use CLDR plural rules, never a hardcoded one/other branch

Plural forms are defined per-language by the CLDR (Common Locale Data Repository), and they are far more varied than English's one/other. Arabic has six forms (zero, one, two, few, many, other); Russian, Polish, and Czech have three or more with rules based on the final digits; some languages have no plural distinction at all (Japanese, Chinese use the same form for all counts). Never write plural logic in application code (`if (n == 1) singular else plural`); this encodes English grammar and is wrong for most languages. Use a pluralization library or message format that applies the correct CLDR rule for the target locale, so the right form is selected per language. The library, not your code, knows the rules.

### Let the translator write the full sentence per plural form, not fragments

The English pattern of "N item(s)" works only because English pluralization is a suffix. In other languages, the entire sentence structure changes with plurality: word order, agreement on adjectives and verbs, and even different vocabulary. You cannot compose the localized sentence by picking a plural noun and pasting it into a template. Instead, use ICU MessageFormat (or equivalent) to let the translator write a complete sentence for each plural form, with the count as a placeholder: `{count, plural, one {You have one new message} other {You have # new messages}}`. The translator controls the full sentence in each form, so the grammar is correct. Fragment composition (noun + count + verb) cannot produce grammatical sentences across languages.

### Handle grammatical gender explicitly with select expressions

Many languages (Romance, Slavic, Semitic, and others) mark grammatical gender on nouns, adjectives, and sometimes verbs. A sentence like "You are subscribed" may need different translations depending on whether "you" is being addressed as male or female in languages with grammatical gender agreement. Handle this with ICU `select` expressions that let the translator provide a variant per gender: `{gender, select, male {...} female {...} other {...}}`. Do not assume a default gender or hardcode one variant; provide all relevant variants and let the message resolve the right one. Note that gender here is a grammatical property of the language and sentence, not necessarily the user's identity; consult the localization team about which distinctions each target language requires.

### Avoid choice/ternary patterns in code; put the logic in the message

A common anti-pattern is handling pluralization or gender in application code with conditionals (`if count == 1 return singular else return plural`) and passing the result to a template. This fails because the code cannot know the target language's grammar, and because the conditions are evaluated before localization. Put the selection logic inside the message via ICU plural/select expressions, so the message carries all variants and the localization library selects the right one for the locale at render time. The application code passes the raw values (count, gender); the message decides the form.

### Do not assume cardinal plural rules apply to ordinal or other contexts

CLDR defines different plural rules for different uses: cardinal (1 item, 2 items), ordinal (1st, 2nd, 3rd), and ranges (1-2 items). English ordinal rules differ from cardinal ("1st" not "1th"), and other languages vary further. Do not reuse cardinal plural logic for ordinals or ranges; use the appropriate CLDR rule set for the context. A "3rd place" message formatted with cardinal rules will be wrong in languages where ordinal forms differ.

### Test pluralization against languages that expose the failures

Testing only English and one other language hides pluralization bugs, because the bug may not manifest in the chosen pair. Test against a language with many plural forms (Arabic, with six) and one with complex rules (Russian or Polish, with three and digit-based rules), in addition to a language with no plural distinction (Japanese). Verify that counts like 0, 1, 2, 3, 11, 21, 101 (the boundary cases where rules differ) produce the correct form in each. If your pluralization works for Arabic and Russian boundary cases, it likely works for most languages; if you only test English, you have tested almost nothing.

### Handle the zero case explicitly where the language or UX requires it

Some languages (Arabic) have a distinct "zero" plural form; others use "other" for zero. Some products want a special message for zero ("No new messages") distinct from the plural form. Decide the desired UX for zero and ensure the message and plural rules handle it correctly per language. Do not assume zero behaves like "other" or like a singular; check the target language's rules and design the zero case deliberately.

## Common Traps

### Hardcoded one/other plural branch

This encodes English grammar and is wrong for Arabic, Russian, Polish, and many others. Use CLDR plural rules via a library or ICU MessageFormat.

### Composing sentences from plural fragments

Picking a plural noun and pasting into a template breaks grammar in languages where the whole sentence changes. Let the translator write the full sentence per plural form.

### Handling plural/gender logic in code instead of the message

Code cannot know the target language's grammar. Put selection logic in the message via ICU plural/select expressions; pass raw values from code.

### Assuming a default gender or ignoring grammatical gender

Many languages require gender agreement. Provide all variants via select expressions and consult localization on which distinctions each language needs.

### Reusing cardinal rules for ordinals or ranges

Ordinal and range plural rules differ from cardinal. Use the appropriate CLDR rule set for the context (1st vs 1, ranges vs single counts).

### Testing only English and one other language

This hides bugs. Test against a many-form language (Arabic), a complex-rule language (Russian/Polish), and a no-plural language (Japanese), at boundary counts (0, 1, 2, 11, 21, 101).

### Assuming zero behaves like "other" or singular

Zero has its own form in some languages and its own UX needs. Handle it explicitly per language and per desired message.

## Self-Check

- Are plural forms selected via CLDR rules (library or ICU MessageFormat) rather than a hardcoded one/other branch in application code?
- Does each plural variant contain a complete sentence written by the translator (with the count as a placeholder), rather than a composed noun fragment?
- Is grammatical gender handled with ICU select expressions providing all variants, with no hardcoded default gender, and informed by the localization team's guidance per language?
- Is the plural/gender selection logic inside the message (ICU plural/select) with the application passing only raw values, rather than logic evaluated in code before localization?
- Are ordinal and range contexts using their own CLDR rule sets rather than reusing cardinal plural logic?
- Have you tested pluralization against a many-form language (Arabic), a complex-rule language (Russian/Polish), and a no-plural language (Japanese), at boundary counts (0, 1, 2, 3, 11, 21, 101)?
- Is the zero case handled explicitly per language rule and per desired UX, rather than assumed to behave like "other" or singular?
- Would the pluralization still be correct if the product added Arabic or Russian tomorrow, or does it silently assume English-like grammar?
