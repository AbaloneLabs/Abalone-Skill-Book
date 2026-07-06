---
name: encoding_and_corruption_handling.md
description: Use when the agent is diagnosing or preventing character encoding problems in translation files, handling mojibake and garbled text, detecting wrong encoding declarations, managing UTF-8 byte order marks and legacy encodings, recovering text from corrupted or damaged source files, or ensuring non-Latin and special characters survive the translation round-trip without degradation.
---

# Encoding And Corruption Handling

Character encoding is the silent foundation of every text deliverable, and it fails silently. A file that displays correctly in one environment may show mojibake, the garbage characters produced when bytes are interpreted under the wrong encoding, in another. A translation that introduces accented characters, CJK ideographs, right-to-left scripts, or typographic symbols into a file that was not declared or saved with the right encoding will corrupt those characters on save, on merge, or on transfer. Corruption is worse: a damaged file may have missing bytes, truncated segments, or unreadable regions that destroy content the translator cannot recover. Agents miss encoding and corruption problems because the text looks fine in their working environment, because the failure is deferred to the moment a different tool, operating system, or downstream system opens the file, and because the root cause, a mismatch between the bytes and the declared or assumed encoding, is invisible in the text itself. The harm this skill prevents is deliverables that are linguistically correct but technically broken: characters that render as garbage, special characters that vanish, entire scripts that become unreadable, and files that fail to parse because corruption introduced during processing was never detected.

Use this skill when diagnosing or preventing encoding problems, handling mojibake or garbled output, verifying encoding declarations, managing byte order marks and legacy encodings, recovering text from corrupted files, or ensuring that non-Latin scripts and special characters survive the translation process intact. The goal is deliverables whose every character is correctly encoded, correctly declared, and recoverable across environments.

## Core Rules

### Establish The Source Encoding Before Any Processing

Before translating a single character, establish the source file's actual encoding. Do not assume UTF-8 because it is common; verify it. A file may declare one encoding in its header or byte order mark while its bytes are actually another, a mismatch that produces mojibake when a tool trusts the declaration. Inspect the file's byte-level content, check for a byte order mark, read any encoding declaration in the file's header or metadata, and confirm that the declared encoding matches the actual byte patterns. Legacy files may use ISO-8859-1, Windows-1252, Shift-JIS, GB2312, or other locale-specific encodings, and treating them as UTF-8 corrupts the non-ASCII characters. If the source encoding is ambiguous or undeclared, resolve it before processing, because every downstream step inherits the assumption. An encoding error at intake propagates through the entire project and is expensive to correct after translation.

### Standardize On UTF-8 For The Working And Deliverable Formats

For translation work, UTF-8 without a byte order mark is the safest default working and deliverable encoding, because it represents all scripts, is universally supported, and avoids the cross-environment problems that legacy encodings cause. Convert source files from legacy encodings to UTF-8 at intake, using a verified conversion that preserves all characters, and work in UTF-8 throughout. However, respect destination requirements: some systems expect a byte order mark, some legacy systems require a specific encoding such as Shift-JIS or ISO-8859-1, and some formats have encoding rules in their specification. Match the deliverable encoding to what the destination environment expects, and verify the expectation rather than guessing. An encoding that is correct for the translator's environment but wrong for the destination fails on delivery.

### Manage The Byte Order Mark Deliberately

The byte order mark is a source of frequent, subtle failure. A UTF-8 file with a byte order mark may break systems that do not expect one, particularly Unix-based tools, parsers, and build systems that treat the leading bytes as content. A UTF-8 file without a byte order mark may be misread by systems that expect one, particularly Windows applications. The decision to include or omit the byte order mark must be deliberate and matched to the destination environment, not left to the tool's default. Know whether the destination expects a byte order mark, configure the tool accordingly, and verify the output file's leading bytes. A stray or missing byte order mark produces errors that look like encoding corruption but are actually a format mismatch, and they are hard to diagnose because the text content is correct.

### Preserve Special Characters And Typographic Marks

Translation introduces characters that the source may not have contained: accented letters, CJK ideographs, right-to-left script characters, typographic quotation marks and dashes, currency symbols, and mathematical symbols. These characters must survive every step of the process: entry in the CAT tool, storage in the interchange format, merge into the deliverable, and transfer to the client. Each step is a point where a character can be corrupted if the encoding is wrong, dropped if the format cannot represent it, or substituted if a tool normalizes characters. Verify that special characters round-trip correctly by testing a representative sample through the full chain before committing to full processing. Pay particular attention to characters that have visually similar but distinct Unicode code points, such as curly versus straight quotation marks, the non-breaking space versus a regular space, and full-width versus half-width CJK punctuation, because silent substitution of these changes the text's meaning and appearance.

### Diagnose Mojibake By Its Pattern Not Its Appearance

When mojibake appears, diagnose it by its byte pattern rather than by guessing. Mojibake has characteristic signatures: UTF-8 bytes read as ISO-8859-1 produce sequences like "Ã©" for an accented e; Shift-JIS read as ISO-8859-1 produces scattered half-width characters; UTF-16 read as UTF-8 produces chains of null-byte artifacts. Identify the actual byte values of the corrupted characters, determine what encoding would render those bytes correctly, and re-decode accordingly. Do not attempt to fix mojibake by manually retyping characters, because that guesses at content and cannot recover characters that were already lost. If the original bytes are intact, re-decoding recovers the text; if the bytes were corrupted by a lossy save, the text may be unrecoverable and the source must be re-obtained. Distinguish recoverable mojibake, bytes intact but misread, from irreversible corruption, bytes damaged or lost.

### Detect And Contain File Corruption

Corruption, distinct from encoding error, is physical or structural damage to a file: missing bytes from an interrupted transfer, truncation from a crashed save, unreadable regions from storage failure, or structural breakage from a malformed merge. Detect corruption by verifying file integrity at intake, using checksums or file-size comparison against the source, by confirming that files parse in their native format, and by scanning for truncation or unreadable regions during processing. When corruption is detected, contain it immediately: stop processing the corrupted file, preserve the corrupted version for diagnosis, obtain a clean copy from the client, and do not attempt to translate through corrupted regions, because the translator cannot know what the missing or damaged content was. Translating corrupted content and guessing at the gaps produces a deliverable that is wrong in ways no one can detect. Flag corruption to the client and require a clean source.

## Common Traps

### Assuming UTF-8 Without Verifying The Actual Encoding

Treating every file as UTF-8 because it is the common case corrupts legacy-encoded files at the first non-ASCII character. This is a trap because ASCII content looks correct under any encoding, so the error is invisible until accented or non-Latin text appears. Always verify the actual byte-level encoding.

### Mismatching The Deliverable Encoding To The Destination

Delivering UTF-8 to a system that expects Shift-JIS, or omitting a byte order mark a Windows application requires, causes the file to fail or display garbage at the client. This is a trap because the file is correct in the translator's environment. Confirm and match the destination's encoding expectation.

### Leaving The Byte Order Mark Decision To The Tool Default

A stray or missing byte order mark breaks parsers and build systems in ways that look like encoding corruption. This is a trap because the text content is correct, so the error is hard to trace. Decide the byte order mark deliberately based on the destination.

### Silently Substituting Similar-Looking Characters

Replacing a curly quote with a straight quote, or a non-breaking space with a regular space, during processing changes the text's appearance and sometimes its meaning. This is a trap because the characters look almost identical. Test special and typographic characters through the full round-trip.

### Manually Retyping Mojibake Instead Of Re-Decoding

Retyping garbled characters guesses at content and cannot recover what was lost. This is a trap because it feels like fixing the problem while embedding guesses. Diagnose by byte pattern and re-decode, or re-obtain the source if bytes are damaged.

### Translating Through Corrupted Regions And Guessing The Gaps

Processing a file with missing or damaged content and inferring the gaps produces a deliverable wrong in undetectable ways. This is a trap because the pressure to continue discourages stopping. Detect corruption, contain it, and require a clean source.

## Self-Check

Before delivering files that contain non-ASCII, non-Latin, or special characters, verify:

- The source file's actual encoding was established by byte-level inspection before processing, not assumed, and any mismatch between declared and actual encoding was resolved at intake.
- The working format is UTF-8 and the deliverable encoding matches what the destination environment expects, confirmed rather than guessed.
- The byte order mark decision, present or absent, is deliberate and matched to the destination, not left to a tool default.
- Special characters, accented letters, CJK ideographs, RTL characters, typographic marks, and symbols were tested through the full round-trip and survive intact without silent substitution.
- Any mojibake encountered was diagnosed by byte pattern and re-decoded, not manually retyped, and a distinction was made between recoverable misreading and irreversible byte damage.
- File integrity was verified at intake via checksum or size comparison, files were confirmed to parse in their native format, and no corrupted regions were translated through.
- Corruption, where detected, was contained, the corrupted version preserved, and a clean source obtained rather than guessed at.
- The final deliverable's every character is correctly encoded, correctly declared, and renders correctly in the destination environment, not just in the translator's working tool.
