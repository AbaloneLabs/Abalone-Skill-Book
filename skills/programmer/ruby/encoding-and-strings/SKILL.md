---
name: ruby_encoding_and_strings.md
description: Use when the agent is handling Ruby strings, encodings, and character data, dealing with UTF-8 vs ASCII-8BIT vs Encoding.default_internal, force_encoding vs encode, String#bytes vs chars, multi-byte character slicing, regex on Unicode, frozen string literals, symbol vs string, StringIO, or is diagnosing "incompatible character encodings", "invalid byte sequence in UTF-8", "string contains null byte", garbled output, encoding errors reading files or HTTP responses, or strings that compare unequal despite looking identical. Covers Ruby's per-string encoding model, transcoding boundaries, byte vs character operations, frozen strings, and the pitfalls of Encoding.default_internal and binary data.
---

# Encoding And Strings In Ruby

Ruby strings are not byte arrays and they are not "just text" — each `String` carries its own `Encoding`, and almost every encoding bug in Ruby comes from ignoring that fact. A string read from a file is tagged with the file's encoding (or `Encoding.default_external`); a string from an HTTP response is tagged `ASCII-8BIT` (binary) until you tell Ruby otherwise; a string literal in source is tagged with the file's `__ENCODING__` (usually UTF-8); a `String#force_encoding` changes the tag without touching the bytes, while `String#encode` transcodes (rewrites the bytes). When two strings of incompatible encodings meet (concatenation, interpolation, comparison), Ruby raises `Encoding::CompatibilityError` ("incompatible character encodings"), which is the most common production encoding failure. The judgment problem is to know the encoding of every string at every boundary (file, network, source, database), to transcode deliberately at those boundaries, to distinguish byte operations from character operations, and to avoid the global `Encoding.default_internal` crutch that hides mismatches rather than fixing them.

Agents routinely call `.force_encoding('UTF-8')` on any string that errors, which silences the error and corrupts the data when the bytes were not actually UTF-8; or they set `Encoding.default_internal = 'UTF-8'` globally and assume all strings become UTF-8, which works until a binary blob or a differently-encoded source appears. The remedy is to treat encoding as a per-string property, to validate and transcode at input boundaries, to use byte operations (`bytes`, `bytesize`, `b`) when bytes are what you mean, and to handle binary data with `ASCII-8BIT` explicitly.

## Core Rules

### Know The Encoding Of Every String At Every Boundary

Every string has an encoding, set by where it came from. A literal has the source file's `__ENCODING__` (UTF-8 by default in modern Ruby). A file read with `File.read` is tagged with `Encoding.default_external` (UTF-8 by default) unless you specify otherwise. An HTTP response body is `ASCII-8BIT` (binary) until you tag it. A database adapter tags strings per the connection encoding. Before operating on a string whose provenance you do not control, know its encoding; if you do not, validate and transcode it.

- At each input boundary (file, socket, HTTP, DB), decide the expected encoding and tag/transcode explicitly.
- Do not assume `Encoding.default_external`/`default_internal` make every string UTF-8; they affect I/O defaults, not strings created other ways.

### Distinguish force_encoding From encode

`force_encoding` changes the encoding tag without changing the bytes — it is a claim about what the bytes already mean. `encode` transcodes, rewriting the bytes from one encoding to another. They are not interchangeable, and using the wrong one is the core encoding bug.

- Use `force_encoding('UTF-8')` only when you *know* the bytes are valid UTF-8 but Ruby tagged them otherwise (e.g., an HTTP body you know is UTF-8 but arrived as `ASCII-8BIT`). Follow it with a validity check.
- Use `encode('UTF-8')` (or `encode('UTF-8', invalid: :replace, undef: :replace)`) to transcode from a known source encoding to UTF-8, rewriting bytes.
- Never `force_encoding` to silence an error without validating the result (`str.force_encoding('UTF-8').valid?`); forcing invalid bytes produces a string that errors on later use.

### Validate And Transcode At Input Boundaries

At each boundary where data of unknown or foreign encoding enters, validate (`str.valid_encoding?`) and transcode to your internal encoding (UTF-8). This centralizes encoding handling and prevents incompatible strings from meeting deep in the application. Replace invalid bytes deliberately (`encode(..., invalid: :replace, undef: :replace)`) rather than letting them propagate, and log the replacement so corrupted input is visible.

- File I/O: open with an explicit encoding (`File.read(path, encoding: 'UTF-8')` or `'UTF-16:UTF-8'` to transcode on read).
- HTTP: tag the body with the response's charset, then validate/transcode.
- Database: set the connection encoding so strings arrive already UTF-8.

### Use Byte Operations When You Mean Bytes

Ruby's character operations (`length`, `chars`, `slice`/`[]`, `each_char`) operate on characters (encoding-aware); byte operations (`bytesize`, `bytes`, `b`, `getbyte`) operate on bytes. For binary protocols, length-prefixed formats, cryptography, and fixed-width records, you almost always mean bytes. Using `str.length` for a binary buffer's size is wrong for any multi-byte content; use `str.bytesize`. A `String#b` returns a binary (`ASCII-8BIT`) copy for safe byte-level work.

- For binary data, work in `ASCII-8BIT` and use `bytesize`/`getbyte`/`setbyte`.
- `str.bytes.each` iterates bytes; `str.chars.each` iterates characters — choose deliberately.
- Slicing by byte index vs character index differs for multi-byte strings; use the one matching the protocol.

### Handle Frozen Strings And Mutation Deliberately

Ruby 3.0+ moves toward frozen string literals, and `# frozen_string_literal: true` at the top of a file makes all literals in it frozen. Mutating a frozen string raises `FrozenError`. Code that does `str << "x"` or `str.gsub!` on a literal breaks under the pragma. Use `+""` (or `String.new`) for a mutable empty string, and `dup`/`+` to unfreeze when mutation is needed. Frozen strings are a performance win (no allocation per literal) and a correctness aid (no accidental mutation of shared literals); adopt the pragma file-by-file and fix the mutation sites.

- `# frozen_string_literal: true` makes `"x"` frozen; `"".dup` or `+""` for mutable.
- Prefer non-mutating methods (`gsub` over `gsub!`) for clarity; reserve `!` methods for deliberate in-place updates.

### Treat Symbols And Strings As Different Types

A Symbol is immutable, interned, and compared by identity; a String is mutable (unless frozen) and compared by value. `:foo == "foo"` is `false`. Hashes with symbol keys vs string keys are different (the Rails `HashWithIndifferentAccess` and JSON parsing are common sources of confusion). Be consistent within an API: keyword-style identifiers as symbols, text data as strings. Convert deliberately (`to_sym`/`to_s`), and avoid creating symbols from untrusted input (symbols are not garbage-collected, a memory-leak vector — a "symbol DoS").

- Use symbols for fixed identifiers (keys, method names, states); strings for text data.
- Do not `untrusted.to_sym` — symbols are not GC'd in older Ruby and accumulate; convert to strings.

## Common Traps

### force_encoding To Silence An Error

`str.force_encoding('UTF-8')` on non-UTF-8 bytes produces a string that errors later or corrupts silently. Validate (`valid_encoding?`) and transcode (`encode`) instead.

### Incompatible Encoding Concatenation

`utf8_str + ascii_8bit_str` raises `Encoding::CompatibilityError` when the binary string has non-ASCII bytes. Tag/transcode the binary string first.

### HTTP Body Treated As UTF-8

A response body is `ASCII-8BIT`; interpolating it into a UTF-8 string errors. Tag it with the charset, validate, and transcode.

### length vs bytesize For Binary Data

`buf.length` counts characters, not bytes; for a binary protocol this is the wrong size. Use `bytesize`.

### Encoding.default_internal Hiding Mismatches

Setting `default_internal = 'UTF-8'` makes I/O transcode automatically, which papers over sources that are not UTF-8 and can silently replace characters. Handle encoding at each boundary explicitly.

### Mutating A Frozen String Literal

Under `# frozen_string_literal: true`, `str = ""; str << "x"` raises `FrozenError`. Use `"".dup` or `+""`.

### Symbol From Untrusted Input

`params[:name].to_sym` creates an un-GC'd symbol per distinct input, a memory leak and DoS vector. Keep untrusted data as strings.

### Regex On A Differently-Encoded String

A regex literal has the source file's encoding; matching it against a string of a different (incompatible) encoding can error or behave unexpectedly. Ensure both share an encoding.

## Self-Check

- [ ] The encoding of every string is known at each boundary (source literal, file, HTTP, DB), and input boundaries validate and transcode to the internal encoding rather than relying on global defaults.
- [ ] `force_encoding` is used only where the bytes are known to be valid (followed by `valid_encoding?`), and `encode` is used to transcode; no `force_encoding` is used merely to silence an error.
- [ ] Invalid bytes are handled deliberately (`encode(..., invalid: :replace, undef: :replace)` with logging) rather than propagated, and no incompatible-encoding concatenation can occur.
- [ ] Binary data is handled in `ASCII-8BIT` with byte operations (`bytesize`, `getbyte`, `b`), and `length`/`chars` are not used where bytes are meant.
- [ ] Frozen string literals are adopted deliberately (`# frozen_string_literal: true`), mutation sites use `dup`/`+`/`String.new`, and non-mutating methods are preferred.
- [ ] Symbols are used for fixed identifiers and strings for text data, no symbols are created from untrusted input, and symbol/string key confusion is resolved consistently.
- [ ] `Encoding.default_internal` is not used as a global crutch to hide mismatches; encoding is handled per-boundary.
- [ ] The string handling has been considered under non-UTF-8 input, binary data, mixed-encoding concatenation, and frozen literals, and remains correct and corruption-free.
