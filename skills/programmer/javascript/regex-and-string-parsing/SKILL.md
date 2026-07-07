---
name: javascript_regex_and_string_parsing.md
description: Use when the agent is writing or reviewing JavaScript code that uses regular expressions, parses structured text (URLs, query strings, CSV, log lines, mini-languages), splits or tokenizes strings, applies Unicode-aware matching, handles greedy vs lazy quantifiers and catastrophic backtracking, escapes user input into regex, replaces with functions, or diagnoses "the regex matched too much", "the regex hangs on certain input", "split produced empty strings", or Unicode characters being split mid-code-point. Covers regex semantics, the string parsing decision (regex vs hand-written parser vs library), Unicode and emoji handling in strings, and the pitfalls of regex for structured data.
---

# Regex And String Parsing In JavaScript

String parsing looks deceptively simple in JavaScript — a `.split()`, a `.match()`, a `.replace()` — and most agents reach for a regular expression the moment they see text that needs shaping. That reflex is the source of most parsing bugs. Regular expressions describe regular languages, but almost no real-world text (URLs, HTML, CSV with quoted fields, JSON, locale-aware names, log formats with optional fields) is a regular language. When a regex is forced onto a context-sensitive grammar, the result is a pattern that works on the happy path and silently corrupts data on quoted fields, escaped delimiters, nested structures, or multi-byte characters. The judgment problem is not "can I write a regex for this" but "should this be a regex at all, and if so, what are its failure modes on adversarial and Unicode input."

A second, equally common failure is treating JavaScript strings as byte arrays. JavaScript strings are sequences of UTF-16 code units, not Unicode code points, not grapheme clusters. `"😀".length` is `2`. `"café".slice(0, 4)` can split a combining accent from its base letter. A regex `.` does not match astral-plane characters by default. Case conversion, sorting, and splitting all behave in code-unit terms, which produces the recurring "the emoji got cut in half" and "the accented character broke" bugs. Agents who learned regex on ASCII examples routinely ship patterns that are correct for ASCII and wrong for the rest of the world.

## Core Rules

### Decide Regex vs Parser vs Library Before Writing The Pattern

Before writing a regex, classify the input. If the format is a real grammar with nesting, quoting, or escapes (HTML, JSON, CSV with quotes, URLs with encoded characters, email addresses per RFC), a regex is the wrong tool: use the platform parser (`URL`, `JSON.parse`, `DOMParser`), a dedicated library (`papaparse` for CSV, a real email validator), or a hand-written recursive-descent parser. Reserve regex for genuinely regular fragments: log line prefixes, phone-number-ish tokens, simple delimiter splitting where fields cannot contain the delimiter.

- HTML and XML: never parse with regex. Use `DOMParser` (browser) or a DOM library (Node). Regex on HTML is the canonical "you can't parse HTML with regex" failure; it breaks on nested tags, attributes with `>`, comments, CDATA, and entity encoding.
- CSV: a quoted field can contain the delimiter and newlines. `str.split(',')` is wrong the moment any field is quoted. Use a CSV library.
- URLs: use `new URL(str)`. Regex-based URL parsing mishandles encoded characters, ports, IPv6, and relative refs.
- JSON: use `JSON.parse`. Regex extraction from JSON breaks on escaped quotes and nested objects.

When you do use regex, document what input class it assumes and what it rejects, so the next reader knows the boundary.

### Always Pass The Unicode Flag For Non-ASCII Text

The `u` flag changes regex semantics to operate on code points instead of code units and enables Unicode property escapes (`\p{L}`, `\p{Script=Han}`, `\p{Emoji}`). Without it, astral characters (emoji, many CJK characters, ancient scripts) are seen as two surrogate halves, so `.` matches only one surrogate, character classes behave wrongly, and quantifiers can split a surrogate pair.

- `/^.$/u.test("😀")` is `true`; without `u` it is `false`.
- Use `/\p{L}/u` to match any letter across scripts instead of `[a-zA-Z]`, which silently excludes accented and non-Latin letters.
- The `u` flag also makes the pattern stricter (no lone surrogates, no legacy octal escapes), which surfaces bugs early.

Prefer the `u` (and often `v`) flag on any pattern that touches user text. Reserve non-`u` patterns for byte-oriented legacy protocols.

### Account For UTF-16 Code Units In Indexing And Slicing

String methods (`slice`, `substring`, `substr`, `indexOf`, `length`, `split('')`) operate on UTF-16 code units. For astral characters this means an index can land between a surrogate pair, producing a lone surrogate (`"\uFFFD"`-ish garbage on display).

- `"😀".length === 2`; `"😀".slice(0, 1)` yields a lone high surrogate.
- To iterate by code point, use `for (const ch of str)` (which yields code points) or `Array.from(str)` / `[...str]`.
- To iterate by user-perceived character (grapheme cluster — a base letter plus combining marks, or an emoji plus a skin-tone modifier plus a ZWJ-joined sequence), use `Intl.Segmenter` with `{ granularity: 'grapheme' }`. This is the only correct way to count "characters" as users see them, to truncate without splitting an emoji family, or to limit a text field length.

Decide explicitly which unit you mean: byte (rare in JS), code unit (default), code point (`for..of`), or grapheme (`Intl.Segmenter`). Mixing them produces off-by-one display bugs.

### Escape User Input Before Embedding It In A Pattern

If you build a regex from a variable (`new RegExp('\\b' + userInput + '\\b')`), any regex metacharacter in the input (`.*+?^${}()|[]\\`) is interpreted, which both corrupts matching and is a ReDoS vector if the input can craft a catastrophic pattern. Escape the input with a function that backslash-escapes every metacharacter, or prefer `String.prototype.replaceAll` / `.includes` when you only need literal substring matching.

- For literal substring search, do not use regex at all: `str.includes(needle)` is clearer and immune to metacharacters.
- For literal split, `str.split(needle)` does not interpret the delimiter as regex (unlike the single-arg regex forms).

### Bound The Pattern And Avoid Catastrophic Backtracking

Regex evaluation can be exponential in input length for patterns with overlapping quantifiers (`(a+)+`, `(a|a)*`, nested optional groups). Adversarial or merely unlucky input then hangs the event loop (browser) or the request (Node), a denial-of-service class called ReDoS.

- Avoid nested quantifiers and quantifiers over alternations that can match the same text in multiple ways.
- Prefer possessive/atomic behavior where available; in JS, anchor patterns and keep alternations disjoint.
- Set a timeout only if your runtime supports it (Node has no native regex timeout; consider a library or running untrusted patterns in a Worker so a hang does not kill the main thread).
- Treat any regex that parses untrusted input as a security surface and fuzz it with long, repetitive input.

### Use replace With A Function For Context-Sensitive Substitution

`str.replace(regex, fn)` calls the function with the full match and each capture group, letting the replacement depend on what was matched. This is far safer than building a giant alternation or chaining multiple replaces, and it avoids the `$`-interpolation pitfalls of string replacements (`$&`, `$1` are interpreted in string replacements and can inject surprises).

## Common Traps

### Parsing HTML, JSON, Or URLs With Regex

A regex that "extracts the href" or "strips tags" works on a sample and breaks on `<a title='a>b'>`, `<script>` containing `</script` in a string, or self-closing tags. Use the real parser. The cost of a parser is near zero; the cost of a regex that silently drops or duplicates content is a data-corruption bug discovered in production.

### Splitting On A Regex Without Expecting Empty Matches

`"a,b,,c".split(",")` yields `["a","b","","c"]` — the empty field is preserved, which is usually correct but often unexpected. With a regex that can match the empty string (`"".split(/x*/)`), the result is implementation-defined-ish and surprising. Filter empties explicitly when that is the intent, and never assume `split` drops empty trailing fields (it does not, unless you pass a limit).

### Greedy Quantifier Matching Too Much

`/<.*>/` on `<a><b>` matches the whole string because `.*` is greedy and runs to the last `>`. Use lazy quantifiers (`/<.*?>/`) or negated character classes (`/<[^>]*>/`) which are clearer and faster. The negated-class form is usually the right choice for delimited matches.

### Forgetting The Global Flag Semantics

`/g` patterns are stateful: `regex.test(str)` and `regex.exec(str)` advance `lastIndex` and return different results on successive calls with the same RegExp object. Sharing a `/g` regex across calls or across uses (test then match) produces "it worked once then failed" bugs. Use a fresh literal per operation, or reset `lastIndex`, or use `String.matchAll` which does not mutate a shared object.

### Case Conversion And Comparison Across Locales

`"I".toLowerCase()` is not `"i"` in Turkish (`"ı"`); `"ß".toUpperCase()` is `"SS"` in some locales. `===` comparison after case-folding fails on accented and composed forms (`"café"` vs `"cafe\u0301"`). For user-facing comparison use `Intl.Collator` with the right sensitivity (`accent`, `case`, `base`); for case-insensitive matching use the `i` flag cautiously and prefer locale-aware folding for non-ASCII.

### ReDoS From Untrusted Input

A contact form whose email field is validated with `(a+)+`-shaped pattern can be made to hang the server with a long string of `a`s. Audit any regex on untrusted input for nested/overlapping quantifiers, and prefer simple anchored patterns or library validators.

## Self-Check

- [ ] The input was classified before a regex was chosen: HTML/JSON/URL/CSV use real parsers, and regex is reserved for genuinely regular fragments with documented assumptions.
- [ ] Every regex touching non-ASCII text uses the `u` flag, and Unicode property escapes (`\p{L}`) are used instead of `[a-zA-Z]` where letters of any script are intended.
- [ ] String indexing and slicing use the correct unit: code unit (default), code point (`for..of` / `Array.from`), or grapheme cluster (`Intl.Segmenter`), and no surrogate pair can be split by a slice.
- [ ] Any regex built from variable input escapes metacharacters, or the code uses literal `includes`/`split`/`replaceAll` instead of a dynamic regex.
- [ ] Patterns on untrusted input have been audited for catastrophic backtracking (no nested or overlapping quantifiers), and long repetitive input was tested.
- [ ] Global-flag regexes are not shared across operations in a way that leaks `lastIndex`; `matchAll` or fresh literals are used.
- [ ] Greedy matches on delimited input use lazy quantifiers or negated character classes, and empty matches from `split` are handled deliberately.
- [ ] Case-insensitive and equality comparisons on user text use `Intl.Collator` with explicit sensitivity rather than raw `toLowerCase`/`===`, so locale and accent differences do not cause false mismatches.
