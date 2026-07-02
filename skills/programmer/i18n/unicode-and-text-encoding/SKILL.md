---
name: unicode_and_text_encoding.md
description: Use when the agent is handling text, processing strings, comparing or truncating user input, normalizing text, converting encodings, counting string length, splitting by characters, handling filenames or user IDs with non-ASCII characters, or debugging mojibake and garbled output.
---

# Unicode And Text Encoding

Text is the data type that looks simple and is not. A string appears to be a sequence of characters, and most code is written as if it were — iterate, slice, count, compare. But a string is actually a sequence of bytes that decodes to code points that group into what a user perceives as characters, and those three layers do not align one-to-one. Code that slices "characters," counts "length," uppercases, or compares for equality using the naive model works on ASCII and fails, silently or loudly, the moment it meets accented letters, emoji, combining marks, scripts like Arabic or Devanagari, or text from a different normalization form. The failures appear as truncated names, broken searches, duplicate-but-not-duplicate records, and the corrupted gibberish known as mojibake.

Agents tend to treat text as bytes-or-bust and assume UTF-8 makes the problem disappear. UTF-8 solves the encoding-layer problem (how bytes map to code points) but leaves the harder problems intact: a single user-perceived character may be multiple code points, the same character may be represented by multiple byte sequences, "length" is ambiguous, and case and ordering depend on locale. The judgment problem is to know which layer a text operation actually operates on, to choose the right unit (bytes, code points, or grapheme clusters) for the task, and to handle encoding boundaries explicitly so that data does not get silently corrupted crossing them.

## Core Rules

### Know Which Unit You Are Operating On

The core confusion in text handling is conflating three different units. A **byte** is the storage unit. A **code point** (a Unicode scalar value) is the addressing unit — what a character *code* like U+0041 refers to. A **grapheme cluster** is the user-perceived character — what a person thinks of as a single "character" and what a cursor moves over. These rarely align: the emoji "👨‍👩‍👧‍👦" is one grapheme cluster, seven code points, and twenty-five bytes in UTF-8. Most bugs come from using the wrong unit for the operation.

Match the unit to the task:

- **Bytes** for storage, transport, hashing, and truncation-safe sizing of buffers. Never index into bytes to get a "character."
- **Code points** for some algorithmic operations (Unicode property checks, casing rules), but never for display, editing, or user-facing counts.
- **Grapheme clusters** for anything a user perceives: cursor movement, selection, truncation for display, counting "characters" shown to a user, and splitting text at a boundary.

The single most common and damaging mistake is truncating or splitting by code point (or worse, by byte), which can slice a combining mark off its base character or cut an emoji in half, producing corrupted text. Truncate and split by grapheme cluster whenever the result is shown to a user or stored as a meaningful unit.

### Be Explicit At Every Encoding Boundary

Encoding corruption (mojibake) happens at boundaries: where bytes become text and text becomes bytes. At every such boundary — reading a file, receiving a network response, storing to a database, printing to a terminal — the encoding must be specified explicitly, because the default is whatever the system guesses, and the guess is often wrong and inconsistent across environments.

Rules for boundaries:

- **Specify the encoding on every read and write.** Do not rely on the platform default; pass UTF-8 explicitly (or the known encoding of the source).
- **Handle decoding errors deliberately.** When bytes do not form valid UTF-8, decide whether to reject, replace, or escape — but never silently corrupt. A decoder that substitutes replacement characters or drops bytes can hide upstream data problems that surface later.
- **Be suspicious of the byte order mark (BOM).** A BOM at the start of UTF-8 data is optional and often unwanted; some consumers expect it, others choke on it. Decide your convention and strip or add the BOM consistently, and never let it leak into the middle of concatenated text.
- **Normalize at the boundary.** When text enters your system, consider normalizing it to a chosen form (see below) so that downstream comparisons are consistent.

The boundary discipline matters because once text is decoded into your program's string type, the encoding is invisible; corruption introduced at the boundary is discovered far from its cause.

### Normalize Before Comparing Or Storing Identifiers

The same user-perceived text can be represented by multiple byte sequences. "é" can be a single precomposed code point (U+00E9) or a base "e" plus a combining accent (U+0065 U+0301). Both render identically and mean the same thing, but they are not byte-equal and not even code-point-equal. Naive comparison, deduplication, and uniqueness checks treat them as different, producing duplicate records, failed logins, and broken searches.

Unicode normalization forms resolve this:

- **NFC** (composed) — the default for storage and interchange; produces the shortest equivalent representation and is what most modern systems expect.
- **NFD** (decomposed) — useful for certain processing (e.g., before applying certain sorting or accent-stripping operations).
- **NFKC / NFKD** (compatibility) — stronger, folds some distinct characters together (e.g., fullwidth and ASCII); use only when you specifically want compatibility folding, because it loses information.

Normalize to NFC on input for identifiers, usernames, deduplication keys, and any text that will be compared for equality. Document the normalization as part of the contract, because mixing normalized and non-normalized data in the same store reintroduces the duplicates you were trying to avoid. Note that normalization is not enough for correct linguistic comparison — that requires locale-aware collation — but it is the foundation.

### Do Not Trust "Length" Without Knowing What It Counts

String length is not a single value; it depends on the unit being counted, and the unit is rarely what the caller assumes. `length` might count bytes, UTF-16 code units (the historical default in many languages), code points, or — if you are lucky and explicit — grapheme clusters. Each gives a different number for the same text, and only grapheme cluster count corresponds to what a user perceives as "characters."

This matters wherever length is used:

- **Validation** ("username must be 3-20 characters") — a byte or code-unit limit rejects valid names in other scripts; a grapheme limit matches user intent.
- **Truncation** ("first 50 characters for preview") — truncating by code units can corrupt multi-byte text.
- **Storage sizing** — byte length is what matters for buffer and column sizing, not perceived length.

Always ask which unit a length refers to, and choose the unit that matches the purpose. For user-facing limits and display, use grapheme clusters; for storage and buffer sizing, use bytes.

### Handle Case, Sorting, And Comparison As Locale-Dependent

Casing and sorting are not universal properties of text; they depend on the language and locale. Turkish has dotted and dotless i with specific casing rules. German sorts ä differently from Swedish, which sorts it differently again. Case-insensitive comparison done with a generic ASCII `toLowerCase` is wrong for many scripts and many locales. These operations require locale-aware libraries, not naive byte manipulation.

Use the platform's locale-aware collation and casing APIs rather than hand-rolling them, and pass the locale explicitly when the user's locale is known. For identifiers and internal keys where locale does not apply, use Unicode default collation or case folding rather than ASCII lowercasing. The cost of getting this right is small; the cost of getting it wrong is silent data mismatches that are hard to reproduce and debug.

## Common Traps

### Truncating Or Splitting By Byte Or Code Point

Cutting text at an arbitrary byte or code point boundary can sever a multi-byte sequence or split a combining mark from its base character, producing corrupted text. Always truncate and split by grapheme cluster when the result is shown to users or stored as a unit.

### Assuming UTF-8 Solves Everything

UTF-8 solves the byte-to-code-point encoding, but not the code-point-to-grapheme grouping, normalization, casing, or collation problems. Text handling still requires grapheme-aware operations, normalization, and locale-aware comparison. Treating UTF-8 as the whole answer leaves the harder bugs intact.

### Relying On The Platform Default Encoding

Reading and writing without specifying the encoding inherits whatever the platform defaults to, which varies across operating systems, locales, and configurations. The same code produces different results in different environments. Specify the encoding explicitly at every boundary.

### Comparing Strings Without Normalization

Two strings that look identical can have different byte sequences (composed versus decomposed), so naive equality and uniqueness checks produce false negatives — duplicate records, failed matches. Normalize to a chosen form before comparing or storing identifiers.

### Silent Substitution On Decode Errors

A decoder that silently replaces invalid bytes with the replacement character or drops them hides upstream corruption, which surfaces later as mysterious data problems. Handle decode errors explicitly so the cause is visible at the boundary where it occurs.

### Using ASCII Casing For Non-ASCII Text

`toLowerCase` implemented for ASCII mangles or ignores non-ASCII characters and is wrong for locales with special casing rules (Turkish, Lithuanian, etc.). Use locale-aware casing and case folding for any text that may contain non-ASCII characters.

### Counting UTF-16 Code Units As "Characters"

Many languages expose string length as UTF-16 code units, which counts surrogate pairs (emoji, rare scripts) as two units and does not match user-perceived characters. A "20-character limit" enforced on code units rejects valid text and miscounts emoji. Know which unit your length function counts and use grapheme clusters for user-facing purposes.

## Self-Check

- [ ] Every text operation uses the correct unit — bytes for storage and hashing, code points for Unicode property checks, grapheme clusters for display, truncation, selection, and user-facing counts.
- [ ] Truncation and splitting are done by grapheme cluster, never by byte or code point, so multi-byte sequences and combining marks are not severed.
- [ ] Encoding is specified explicitly at every read/write boundary; the platform default is never relied upon, and decode errors are handled deliberately rather than silently substituted.
- [ ] Identifiers, deduplication keys, and equality comparisons normalize text to a documented form (typically NFC) before comparison, and normalized and non-normalized data are not mixed in the same store.
- [ ] String length is interpreted with knowledge of which unit it counts, and user-facing limits and display truncation use grapheme cluster counts, not bytes or code units.
- [ ] Casing, sorting, and case-insensitive comparison use locale-aware APIs with the locale passed explicitly, not ASCII lowercasing.
- [ ] The BOM convention is consistent — either always present or always stripped — and BOM never leaks into the middle of concatenated text.
- [ ] Text handling was tested with non-ASCII inputs (accented characters, emoji with skin tones and ZWJ sequences, combining marks, RTL scripts) and not only with ASCII.
