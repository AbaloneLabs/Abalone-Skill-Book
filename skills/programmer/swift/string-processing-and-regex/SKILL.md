---
name: swift_string_processing_and_regex.md
description: Use when the agent is processing text in Swift, using String, Substring, Character, and CharacterSet, Swift Regex literals and RegexBuilder, NSRegularExpression bridging, Unicode scalar and grapheme cluster handling, splitting and parsing, localized case-insensitive comparison, or is diagnosing "the emoji is split", "the regex matched wrong", "index out of range" from integer indexing, encoding/decoding with formats, or "count is wrong for multi-byte text". Covers Swift's grapheme-cluster String model, integer-free indexing, Regex and RegexBuilder, Unicode correctness, and the pitfalls of treating Swift strings like C strings.
---

# String Processing And Regex In Swift

Swift's `String` is not a byte array and not an array of code units — it is a collection of extended grapheme clusters, which is the most user-correct model available and also the source of every surprise agents encounter with it. A `String`'s `count` is the number of user-perceived characters (an emoji family with a skin-tone modifier is one character), not the number of UTF-8 bytes, not the number of UTF-16 code units, and not the number of Unicode scalars. Indexing is not integer-based: `str[5]` does not compile, because the index space is not contiguous in memory and computing an index requires walking the grapheme boundaries. Agents who learned strings in C, Java, or JavaScript reach for integer indexing, assume `count` is byte length, and try to parse with NSRegularExpression as if it were a byte scanner — and then ship code that crashes on emoji, miscounts fields, or splits a grapheme cluster in half. The judgment problem is to embrace the grapheme model, to use the index API (or higher-level APIs) instead of integers, to choose RegexBuilder/`Regex` over NSRegularExpression for new code, and to be explicit about which unit (grapheme, scalar, UTF-8 byte, UTF-16 unit) an operation targets.

The harm is concentrated in user-generated text: names, messages, emoji, accented characters. Code that works on ASCII test data crashes or corrupts on real input. The remedy is to use the grapheme-aware APIs, to parse with `Regex`/`RegexBuilder` (compile-checked, type-capturing) rather than ad-hoc string slicing, and to drop to byte/scalar operations only when a format genuinely requires them.

## Core Rules

### Treat String As A Collection Of Grapheme Clusters, Not Bytes

`String` is a collection of `Character` values, where each `Character` is an extended grapheme cluster — a user-perceived character that may be multiple Unicode scalars (a base letter plus combining marks, an emoji plus a skin-tone modifier plus ZWJ-joined components). `"e\u{301}"` (e + combining acute) is one `Character`, `"👨‍👩‍👧"` is one `Character`. `count` reflects this. Operations that assume one byte or one code unit per character (`str.count` for buffer sizing, `str[i]` for random access) are wrong for any non-ASCII text.

- `str.count` is grapheme count, not byte length; use `str.utf8.count` for byte length, `str.unicodeScalars.count` for scalar count.
- Iteration `for ch in str` yields grapheme clusters (`Character`), the user-correct unit.
- For byte-level protocols, work in `utf8` (`str.utf8` is a collection of `UInt8`).

### Use The Index API, Not Integer Subscripts

Swift `String` indices (`String.Index`) are opaque positions, not integers, because computing a position requires walking grapheme boundaries. There is no `str[5]`. To access by position, use `str.index(str.startIndex, offsetBy: 5)`, or — preferably — use higher-level APIs (`first`, `last`, `prefix`, `suffix`, `split`, `contains`) that avoid manual indexing. Integer offset indexing is O(n) (it walks from the start), so it is both awkward and slow in a loop. `Substring` shares storage with the original (zero-copy) and is preferred for temporary slices; convert to `String` when you need a value that outlives the source.

- Avoid `index(_:offsetBy:)` in loops; use `for ch in str` or `enumerated()` where you need position.
- Use `split(separator:)` and `components(separatedBy:)` for parsing instead of manual index math.
- `Substring` is efficient but holds the original string alive; convert to `String` for long-lived storage.

### Prefer Regex And RegexBuilder Over NSRegularExpression And Manual Parsing

Swift's `Regex` (literals `/.../` and the `RegexBuilder` DSL) is compile-checked, type-capturing (captures can be strongly typed), and grapheme/Unicode-aware. It supersedes `NSRegularExpression` for new code: NSRegularExpression operates on UTF-16 code units and bridges awkwardly, losing Swift's string model. Use `Regex` for matching, capturing, and replacing; use `RegexBuilder` for readable, composable patterns. For simple delimiter splitting, `split` is clearer than a regex.

- `let r = /(\d+)-(\w+)/; if let m = input.firstMatch(of: r) { ... m.1 ... }` — typed captures.
- `RegexBuilder` DSL for complex patterns: `OneOrMore(.digit)`, `Capture { ... }`, composable and readable.
- Reserve `NSRegularExpression` for legacy compatibility or features `Regex` lacks; bridge the result back to Swift strings carefully.

### Be Explicit About The Unit: Grapheme, Scalar, UTF-8, UTF-16

Different operations need different units. User-facing counts, truncation, and field lengths use grapheme clusters (`String`/`Character`). Unicode property matching (`\p{L}`, script checks) uses scalars (`unicodeScalars`). Byte protocols and cryptography use UTF-8 (`utf8`). Legacy `NSString`/`NSRegularExpression` APIs use UTF-16 (`utf16`). Mixing units silently produces off-by-one bugs: truncating by grapheme count is correct for display; truncating by UTF-16 count can split a surrogate pair; truncating by byte count can split a multi-byte sequence. State which unit each operation uses.

- Truncation for display: grapheme (`String.prefix(n)`).
- Length for a byte buffer: `utf8.count`.
- `NSString.length` is UTF-16 code unit count, not grapheme count — do not equate the two.

### Localize Case-Insensitive Comparison And Sorting

`"I".lowercased()` is not `"i"` in Turkish; `"ß".uppercased()` is `"SS"`. `==` after case-folding fails across locales and composed forms. For user-facing comparison and sorting, use `String.localizedStandardCompare`, `localizedCaseInsensitiveCompare`, or `String.CompareOptions` with the right locale. For locale-insensitive exact comparison, `==` is fine. Regex case-insensitivity (`/(?i)/` or `.ignoresCase()`) respects Unicode case folding, which is usually what you want, but be aware of locale-specific exceptions.

- Use `localizedStandardCompare` for user-facing sort order.
- Use `localizedCaseInsensitiveContains` for user-facing search.
- Do not hand-roll case-insensitive comparison with `lowercased()` for non-ASCII.

### Parse Structured Formats With Dedicated Parsers, Not Regex

CSV, JSON, XML, URLs, and date formats are not regular languages. Use `JSONDecoder`/`JSONEncoder`, `URL`/`URLComponents`, `DateFormatter`/`ISO8601DateFormatter`, and a CSV parser rather than regex extraction. Regex is appropriate for genuinely regular fragments (log line prefixes, simple tokens). For typed parsing, `Regex` with typed captures plus a decoding step gives both flexibility and type safety.

## Common Traps

### Integer Indexing That Does Not Compile Or Is O(n)

`str[5]` does not compile; `str.index(str.startIndex, offsetBy: 5)` is O(n). Use higher-level APIs or iteration.

### count Mistaken For Byte Length

`str.count` is grapheme count; a UTF-8 buffer sized by `count` is too small for multi-byte text. Use `utf8.count`.

### Splitting A Grapheme Cluster

Slicing by UTF-16 or byte index can split an emoji or a combining-accent sequence. Slice by grapheme (`String.Index`) for display.

### NSRegularExpression On UTF-16 Surprising Results

NSRegularExpression operates on UTF-16 code units; ranges and captures are in UTF-16 terms, which mismatch Swift's grapheme indices. Use Swift `Regex`.

### Regex Without Unicode Awareness

A pattern assuming ASCII (`[a-z]`) excludes accented and non-Latin letters. Use Unicode property escapes (`/\p{L}/`) for "any letter".

### Substring Retaining The Original

A long-lived `Substring` keeps the original large string alive. Convert to `String` when the slice outlives the source.

### Hand-Rolled Case-Insensitive Comparison

`a.lowercased() == b.lowercased()` fails on locale-specific and composed-form differences. Use localized comparison.

### Parsing JSON/CSV/URL With Regex

Regex extraction from JSON/CSV/URLs breaks on escapes, quotes, encoded characters. Use the dedicated parser.

## Self-Check

- [ ] `String` is treated as grapheme clusters: `count` is grapheme count (not byte/UTF-16 length), iteration uses `Character`, and byte protocols use `utf8`.
- [ ] No integer subscript (`str[i]`) is used; indexing uses `String.Index` via higher-level APIs (`prefix`, `suffix`, `split`, `first`, `last`), and `index(_:offsetBy:)` is avoided in loops.
- [ ] `Substring` is used for temporary slices and converted to `String` when it outlives the source, so no large original string is retained.
- [ ] New regex uses Swift `Regex` literals / `RegexBuilder` (compile-checked, typed captures, Unicode-aware), with `NSRegularExpression` reserved for legacy compatibility and bridged carefully.
- [ ] Each operation's unit (grapheme, scalar, UTF-8, UTF-16) is explicit and correct for its purpose; no unit is mixed in a way that splits a grapheme or miscounts.
- [ ] Case-insensitive comparison and sorting use localized APIs (`localizedStandardCompare`, `localizedCaseInsensitiveContains`), not hand-rolled `lowercased()` comparison, for user-facing text.
- [ ] Structured formats (JSON, CSV, URL, dates) are parsed with dedicated parsers/`Codable`, not regex; regex is reserved for genuinely regular fragments.
- [ ] The string handling has been considered under emoji, combining marks, multi-byte text, and locale, and remains correct and crash-free.
