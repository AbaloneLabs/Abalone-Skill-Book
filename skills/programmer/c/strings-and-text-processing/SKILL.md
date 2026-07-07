---
name: c_strings_and_text_processing.md
description: Use when the agent is writing or reviewing C code that handles null-terminated strings, fixed-size character buffers, wide characters and wchar_t, multi-byte and UTF-8 text, snprintf/strncpy/strlcpy, string parsing and tokenization, locale-dependent functions, or any text processing where buffer bounds, encoding correctness, truncation, and locale independence are correctness and security concerns.
---

# C Strings And Text Processing

C has no string type. A "string" is a `char` array terminated by a null byte, and the standard library provides functions (`strcpy`, `strcat`, `strlen`, `sprintf`, `strtok`) that operate on these arrays with no bounds checking and no awareness of encoding. The result is that string handling in C is the single largest historical source of security vulnerabilities (buffer overflows) and the single largest source of encoding bugs (treating bytes as characters, mishandling UTF-8, locale-dependent comparisons). The judgment problem is that every string operation is a bounds-checking decision and an encoding decision, and the language provides no help with either.

Agents tend to reach for `strcpy`/`sprintf`/`strtok` because they are familiar, then produce code that overflows a fixed buffer, truncates a multi-byte sequence, breaks on non-ASCII input, or behaves differently under different locales. The judgment problem is to always use bounded operations, to understand that a `char` is a byte and a "character" may be several bytes in UTF-8, and to decide deliberately whether a function should be locale-dependent or locale-independent. This skill is about treating C string handling as a security- and correctness-sensitive discipline, not a casual convenience.

## Core Rules

### Always Use Bounded Operations, Never strcpy Or sprintf

`strcpy(dst, src)` copies until it finds a null in `src`, with no bound on `dst`. If `src` is longer than `dst`, you overflow. `sprintf` has the same problem. These are banned in any security-conscious codebase and should never be used for untrusted input. Use the bounded variants:

- `snprintf(dst, dstsz, fmt, ...)` — writes at most `dstsz-1` bytes plus the null terminator. Check the return value against `dstsz` to detect truncation.
- `strncpy(dst, src, n)` — copies at most `n` bytes, but does *not* null-terminate if `src` is >= `n` bytes. This is a common trap; you must null-terminate manually. `strncpy` also pads the remainder with nulls, which is wasteful.
- `strlcpy(dst, src, dstsz)` and `strlcat` (BSD, not standard C, but widely available) — always null-terminate within the bound and return the length that would have been written, so truncation is detectable. Prefer these where available.
- For your own copy loops, always pass the destination size and stop before the bound, then null-terminate explicitly.

The rule is absolute: if the source length is not known to fit, the destination size must be a parameter to the operation. No exceptions for "I know it's short enough."

### Treat char As A Byte, And Decide Encoding Deliberately

A C `char` is one byte. In UTF-8 (the de facto standard encoding), a single user-perceived character may be 1 to 4 bytes. `strlen` returns the byte count, not the character count. Indexing `s[i]` gives a byte, not a character. String functions that operate byte-by-byte (`strchr`, `strstr`) work correctly on UTF-8 *only* for ASCII search characters, because UTF-8 is self-synchronizing for the ASCII subset; searching for a multi-byte substring with `strstr` works because UTF-8 bytes do not appear as continuation bytes of other sequences, but case conversion, comparison, and character counting do not.

Decide and document the encoding of every string in your API. The modern default is UTF-8. For operations that need character-level semantics (counting characters, case conversion, comparison), use a Unicode-aware library (ICU, or a minimal UTF-8 helper) rather than byte-level libc functions. Never assume one byte equals one character.

### Understand wchar_t Is Not Portable Across Platforms

`wchar_t` is a wide character type intended for fixed-width characters, but its width is implementation-defined: 2 bytes on Windows (UTF-16), 4 bytes on Linux/macOS (UTF-32). Code that assumes `wchar_t` is 4 bytes breaks on Windows; code that assumes 2 bytes breaks elsewhere. The wide-character functions (`wcslen`, `wcscpy`, `swprintf`) have the same bounds issues as their narrow counterparts.

For portable text handling, prefer UTF-8 in `char` with a Unicode library, over `wchar_t`. If you must use `wchar_t`, never assume its width; use `sizeof(wchar_t)` and `WCHAR_MAX`. For Windows interop where UTF-16 is required, use explicit `uint16_t` (or `char16_t` in C11) rather than relying on `wchar_t` semantics.

### Make Locale Dependence A Deliberate Choice

Many libc string functions are locale-dependent: `isalpha`, `toupper`, `strcoll`, `strftime`, `strtod` (decimal point). Their behavior changes with `setlocale`. This is a feature when you want user-localized behavior (formatting a date for display) and a bug when you want deterministic parsing (parsing a number from a wire format that always uses `.` as decimal point).

- For parsing machine-generated text, use locale-independent variants: `strtod` is locale-dependent, but many platforms offer `strtod_l` with an explicit locale, or you can parse with the C locale. `isalpha((unsigned char)c)` in the C locale gives ASCII-only behavior.
- Always cast `char` to `unsigned char` before passing to `<ctype.h>` functions, because `char` may be signed and a negative value is undefined behavior in these functions.
- Decide whether your function should respect the user's locale or be locale-independent, and document it. Mixing the two within a parser produces inconsistent results.

### Avoid strtok; It Is Stateful And Not Reentrant

`strtok` parses a string into tokens, but it stores a pointer to the parsing position in static state, so it cannot be used recursively or from two calls interleaved, and it is not thread-safe (though some libc offer a thread-safe `strtok_r`). Worse, `strtok` modifies the input string, replacing delimiters with null bytes. Use `strtok_r` (POSIX) or `strtok_s` (C11 Annex K, optional), or better, write an explicit parsing loop with `strchr`/`strstr` and pointer arithmetic that does not mutate the input and is fully reentrant.

### Validate And Bound All Text From Untrusted Sources

Text from files, network, environment, or command-line arguments is untrusted. Validate it: check length against a sane maximum, check encoding validity (UTF-8 is easy to validate byte-by-byte), reject or sanitize characters that are dangerous in the destination context (control characters, null bytes embedded in a string). Never copy untrusted text into a fixed buffer without a bound; never pass untrusted text to a format function as the format string (`sprintf(buf, user_input)` is a format-string vulnerability — always `sprintf(buf, "%s", user_input)` or `snprintf`).

## Common Traps

### strcpy Or sprintf Into A Fixed Buffer

`char buf[64]; strcpy(buf, user_input);` overflows if `user_input` is longer than 63 bytes. Always use the bounded variant and check for truncation.

### strncpy Not Null-Terminating

`strncpy(dst, src, sizeof(dst))` leaves `dst` unterminated if `src` is at least `sizeof(dst)` bytes. Always write `dst[sizeof(dst)-1] = '\0';` after, or use `strlcpy`, or pass `sizeof(dst)-1` and null-terminate.

### Assuming One char Is One Character

`s[strlen(s)-1]` to "remove the last character" removes the last byte, which may split a multi-byte UTF-8 sequence, producing invalid encoding. Use a UTF-8-aware operation to remove a code point.

### Passing A Negative char To ctype Functions

`isalpha(c)` where `c` is a signed `char` with a negative value is undefined behavior. Always cast: `isalpha((unsigned char)c)`.

### Using strtok In A Reentrant Or Multi-Threaded Context

`strtok` uses static state, so nested or concurrent calls corrupt each other. Use `strtok_r` or an explicit non-mutating parser.

### Format String Vulnerability

`sprintf(buf, user_controlled)` lets the caller read/write memory via `%n`/`%s`. Always use a literal format: `snprintf(buf, sizeof(buf), "%s", user_controlled)`.

### Assuming wchar_t Width

Code that assumes `wchar_t` is 4 bytes (UTF-32) breaks on Windows where it is 2 bytes (UTF-16). Use `sizeof(wchar_t)` or avoid `wchar_t` for portable code.

### Locale-Dependent Parsing Of Machine Text

`strtod("3.14")` in a locale using `,` as decimal point fails to parse. Use the C locale or a locale-independent parser for machine-generated numbers.

## Self-Check

- [ ] No `strcpy`, `strcat`, or `sprintf` is used on untrusted or length-unknown input; all copies use bounded variants (`snprintf`, `strlcpy`, `strncpy` with explicit null-termination, or a size-checked loop).
- [ ] The encoding of every string is documented (UTF-8 by default), and byte-level operations are not mistaken for character-level operations; character counting, case, and comparison use a Unicode-aware library where semantics matter.
- [ ] `wchar_t` is not assumed to be a fixed width across platforms; portable text handling prefers UTF-8 in `char`, and Windows UTF-16 interop uses explicit width types.
- [ ] Locale dependence is a deliberate choice: machine-format parsing uses the C locale or locale-independent functions, and `<ctype.h>` functions always receive `(unsigned char)` casts.
- [ ] `strtok` is not used; parsing uses `strtok_r`/`strtok_s` or an explicit non-mutating, reentrant parser.
- [ ] Untrusted text is validated for length and encoding before use, never passed as a format string, and never copied into a fixed buffer without a bound.
- [ ] Truncation is detected and handled (snprintf return checked against buffer size, strlcpy return checked), not silently accepted.
