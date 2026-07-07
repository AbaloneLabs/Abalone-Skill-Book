---
name: python_strings_bytes_and_encoding.md
description: Use when the agent is handling Python text and binary data, converting between str and bytes, choosing encodings (utf-8, ascii, latin-1), reading/writing files in text vs binary mode, normalizing Unicode (NFC/NFD), handling emoji and combining characters, reasoning about the Unicode sandwich pattern, debugging UnicodeDecodeError/UnicodeEncodeError or mojibake, processing CSV/JSON with encodings, or reviewing code that mixes str and bytes incorrectly. Covers the str/bytes boundary, the Unicode sandwich, encoding selection, normalization, source encoding, and the tradeoff between text-mode convenience and binary correctness.
---

# Strings, Bytes, And Encoding

Python 3 drew a hard line between text (`str`, sequences of Unicode code points) and binary data (`bytes`, sequences of raw byte values). This separation prevents a large class of bugs but requires you to handle the boundary explicitly. The judgment problem is knowing where text ends and bytes begin, choosing the right encoding, normalizing Unicode consistently, and never letting encoded bytes leak into text processing or vice versa.

Agents tend to call `.encode()`/`.decode()` with the wrong or default encoding, mix `str` and `bytes` in comparisons and concatenations, assume one character equals one byte, or ignore Unicode normalization so that "logically equal" strings compare unequal. The harm appears as `UnicodeDecodeError` on production data, mojibake (Ã© for é) from double-encoding, strings that look identical but compare unequal, and CSV/JSON parsers that choke on emoji. The real work is applying the Unicode sandwich, pinning encodings explicitly, normalizing for comparison, and treating the str/bytes boundary as a first-class design concern.

## Core Rules

### Apply The Unicode Sandwich: Bytes At Edges, str In The Middle

The foundational pattern is to decode bytes to text at the system boundary, process text everywhere in the core, and encode back to bytes only when leaving the system.

- **Decode on input**: when reading from a file, socket, or subprocess, decode bytes to `str` immediately using a known encoding.
- **Process `str` internally**: all business logic, comparisons, and manipulation happen on text, never on raw bytes.
- **Encode on output**: when writing to a file, socket, or network, encode `str` to bytes using a known encoding.

This keeps the core simple (it only deals with text) and concentrates encoding logic at the boundaries. Mixing bytes into the core forces every function to handle both, doubling complexity and inviting bugs.

### Always Specify The Encoding Explicitly, Never Rely On The Default

`open(path)` and `.encode()`/`.decode()` default to a platform-dependent encoding (`locale.getpreferredencoding()`), which is UTF-8 on most Linux systems but cp1252 on Windows or ASCII in some containers. Code that works on the developer's machine breaks in production.

- Always pass `encoding="utf-8"` (or the correct encoding) to `open()`, `.encode()`, `.decode()`, and `subprocess` calls.
- For text files, `open(path, encoding="utf-8")` is explicit and portable.
- For network protocols, use the encoding the protocol specifies (HTTP headers are latin-1; JSON is UTF-8 by spec).

Relying on the default is a portability bug waiting to happen. Pin it explicitly everywhere.

### Know When To Use Text Mode vs Binary Mode For Files

`open(path)` in text mode (the default) returns `str` and handles encoding/decoding and newline translation. `open(path, "rb")` in binary mode returns `bytes` and does no translation.

- **Text mode** (`"r"`, `"w"`): for human-readable text files. Specify `encoding=`. Be aware it translates newlines (`\r\n` ↔ `\n`) by default, which is usually desired but can corrupt binary data.
- **Binary mode** (`"rb"`, `"wb"`): for binary files (images, archives, opaque blobs) and when you need exact byte control. No encoding, no newline translation.
- A common bug is opening a binary file in text mode (corrupting it with encoding errors and newline translation) or a text file in binary mode (getting `bytes` where `str` is expected).

Match the mode to the data. When in doubt about whether data is text, open it binary and inspect.

### Normalize Unicode Before Comparing Or Deduplicating

The same visible text can have multiple Unicode representations. "é" can be one code point (U+00E9, NFC) or two (U+0065 + U+0301, NFD). They look identical but compare unequal and have different byte lengths.

- Normalize before comparison, deduplication, or indexing: `unicodedata.normalize("NFC", s)`.
- **NFC** (composed) is the convention for storage and exchange; most modern text is NFC.
- **NFD** (decomposed) is used by some filesystems (HFS+) and normalization-sensitive operations.
- Emoji and combining characters add further complexity: a family emoji may be multiple code points joined by ZWJ; a flag is two regional indicators. Length and slicing by code point may not match user expectations of "characters".

Without normalization, "duplicate" detection fails, lookups miss matches, and byte lengths surprise. Normalize at the boundary where text enters, store normalized, and compare normalized.

### Understand That One Character Is Not One Byte (Or Even One Code Point)

In UTF-8, a single code point is 1-4 bytes. A character with a combining mark is multiple code points. A grapheme cluster (what a user perceives as one character) may be many code points.

- `len(s)` counts code points, not bytes and not graphemes. `"é".encode("utf-8")` is 2 bytes; `len("é")` is 1 (NFC) or 2 (NFD).
- Slicing by index can split a combining character from its base, producing broken text.
- For user-perceived length and slicing (e.g., truncating a display string), use a grapheme-aware library (`grapheme`, `regex` with `\X`), not `len()` and slicing.

Assuming 1 char = 1 byte breaks on any non-ASCII text; assuming 1 char = 1 code point breaks on combining characters and emoji.

### Handle Source Code Encoding And Literals

Python source files default to UTF-8 (Python 3+). String literals are `str` (text); `b"..."` literals are `bytes`. Mixing them (`"text" + b"bytes"`) raises `TypeError`. When embedding non-ASCII in source, ensure the file is saved as UTF-8; an explicit `# -*- coding: utf-8 -*-` header is only needed for very old Python or unusual encodings.

### Choose The Encoding For The Use Case

- **UTF-8**: the default for almost everything — web, JSON, config, source. Variable-width, ASCII-compatible, can represent all Unicode.
- **ASCII**: a subset of UTF-8; safe only if you guarantee no non-ASCII data. Using it as a decode encoding on UTF-8 data raises `UnicodeDecodeError` on the first non-ASCII byte.
- **latin-1 (ISO-8859-1)**: maps every byte 0-255 to a code point, so it never fails to decode. Useful for opaque byte handling (e.g., HTTP headers) but produces wrong characters for actual UTF-8 text. Do not use it as a "make the error go away" encoding.
- **UTF-16**: used by some systems (Windows APIs, some JSON in certain environments); requires a BOM or known endianness.

## Common Traps

### Relying On The Default Encoding

`open(path)` or `.decode()` without `encoding=` uses a platform default that differs across systems. Always specify UTF-8 (or the correct encoding) explicitly.

### Mixing `str` And `bytes`

`"a" + b"b"` raises `TypeError`. Decide which you are working with and convert at the boundary, never mix in the core.

### Assuming One Character Equals One Byte

UTF-8 encodes a code point in 1-4 bytes. `len(s)` is code points, not bytes. Byte-indexing text breaks non-ASCII characters.

### Unnormalized Strings Comparing Unequal

"é" in NFC vs NFD look identical but `!=`. Normalize before comparison, deduplication, and indexing.

### latin-1 To Silence A Decode Error

Switching to latin-1 makes `UnicodeDecodeError` disappear but produces mojibake on real UTF-8 data. Fix the encoding instead of masking the error.

### Opening Binary Data In Text Mode

`open(image_path)` in text mode corrupts binary data with decoding and newline translation. Use `"rb"` for binary.

### Slicing Through A Combining Character

`s[:5]` can split a base character from its combining mark, producing broken text. Use grapheme-aware operations for display truncation.

### Newline Translation Corrupting Data

Text mode translates `\r\n` ↔ `\n`, which corrupts binary data and can surprise exact-byte protocols. Use binary mode or `newline=""` when you need exact bytes.

## Self-Check

- [ ] The Unicode sandwich is applied: bytes decoded to `str` at input boundaries, text processed in the core, `str` encoded to bytes at output boundaries.
- [ ] Every `open()`, `.encode()`, `.decode()`, and subprocess call specifies the encoding explicitly (typically UTF-8), never relying on the platform default.
- [ ] Files are opened in text mode with explicit encoding for text data, and binary mode (`rb`/`wb`) for binary data or exact-byte control.
- [ ] Text is normalized (NFC) before comparison, deduplication, or indexing so logically-equal strings compare equal.
- [ ] No code assumes one character equals one byte or one code point; `len()` and slicing are understood to operate on code points, not bytes or graphemes.
- [ ] `str` and `bytes` are not mixed in operations; conversions happen explicitly at boundaries.
- [ ] Display truncation and user-perceived length use grapheme-aware methods where combining characters or emoji are possible.
- [ ] latin-1 is not used to silence decode errors; the correct encoding is determined and used instead.
- [ ] Binary data is never opened in text mode, and newline translation is controlled where exact bytes matter.
