---
name: code_sample_verification.md
description: Use when the agent is editing documentation or instructional content that contains code samples, commands, and snippets, verifying that code runs as shown, checking that examples match the described APIs and versions, confirming imports and dependencies are complete, ensuring snippets are reproducible by readers, or preventing broken misleading or outdated code from reaching users.
---

# Code Sample Verification

Code samples in documentation are promises. A reader who copies an example expects it to work, and an example that fails, because of a typo, a missing import, an outdated API, or an unshown prerequisite, breaks that promise and erodes trust in the entire document. Unlike prose, code is binary in its correctness: it either runs or it does not, and an error that a reader could not catch is an error the editor should have caught. Verifying code samples is therefore a distinct editorial discipline, closer to testing than to copy-editing, and it requires deliberate effort that prose review alone cannot substitute for.

Use this skill when editing any documentation, tutorial, guide, or instructional content that contains code samples, commands, configuration snippets, or terminal output. It covers verifying that code runs as shown, checking that examples match described APIs and versions, confirming imports and dependencies are complete, ensuring snippets are reproducible, and preventing broken, misleading, or outdated code from reaching users. The editor's task is to treat every code sample as a claim that must be tested, not merely read.

## Core Rules

### Run Every Code Sample Before Publication

The foundational discipline of code verification is that examples must be executed, not merely read. Reading code catches some errors, but it cannot catch the errors that matter most, a misspelled identifier that compiles differently, a dependency that is not installed, an API that changed between versions, or a snippet that works only in the author's specific environment. Running the code is the only reliable verification.

For each code sample, execute it in an environment as close as possible to the reader's, using the versions and setup the document specifies. Confirm that it runs without error and produces the output shown. Where the sample is part of a sequence, confirm that each step builds on the previous and that the cumulative result is what the document claims. For samples that cannot be run directly, because they require specific hardware, data, or access, verify as much as possible and document what was not tested. A sample that has not been run is a sample that has not been verified, regardless of how correct it looks. The cost of running code is far lower than the cost of publishing broken examples that thousands of users will encounter.

### Match Examples To The Described API And Version

Code samples must match the APIs, libraries, and versions the document describes, and this match must be verified rather than assumed. APIs change between versions, libraries rename or remove functions, and default behaviors shift, so an example that was correct when written may be wrong by publication. Version drift is one of the most common causes of broken documentation.

For each sample, confirm the APIs and libraries used match what the document describes, including version numbers where specified. Check that function signatures, parameter names, return types, and behaviors match the version the reader will use, not an earlier or later one. Where the document targets a specific version, verify against that version; where it is version-agnostic, confirm the sample works across the supported range or note version-specific behavior. Watch for examples copied from older documentation or other sources that may reflect a previous API. For fast-moving libraries, verify recency, since an example even months old may be outdated. Matching examples to the described API protects readers from errors that are invisible until they try to run the code.

### Confirm Imports Dependencies And Prerequisites Are Complete

A common reason code samples fail for readers is that they omit prerequisites the author had in their environment but did not show. Missing imports, undeclared dependencies, assumed environment variables, or unstated setup steps cause examples to fail for readers who follow the document faithfully. Completeness of prerequisites is part of code correctness.

For each sample, confirm that everything needed to run it is shown or referenced. Check that imports and includes are present and correct, since omitting them to save space produces errors. Check that dependencies, libraries, packages, and tools, are stated and that installation or setup is covered where needed. Check that prerequisites, environment variables, configuration files, prior steps, are documented rather than assumed. Where a sample depends on context established earlier in the document, confirm the cross-reference is clear. The test is whether a reader starting from the document's stated setup could run the sample without needing information the document does not provide. Completeness is what makes an example reproducible rather than merely illustrative.

### Ensure Snippets Are Reproducible In Isolation

Code samples often appear as fragments, excerpts meant to illustrate a point rather than complete programs. Fragments are legitimate, but they must be clearly framed as fragments, and any complete example presented as runnable must actually be runnable. Confusion about whether a snippet is complete or partial causes readers to attempt to run code that cannot run.

Distinguish clearly between complete runnable examples and illustrative fragments. For complete examples, ensure they include everything needed to run, including the boilerplate, main function, or entry point that a real program requires. For fragments, frame them as excerpts, using ellipses, comments, or surrounding text to indicate what is omitted, so readers do not mistake them for complete programs. Where a fragment uses names or functions defined elsewhere, note where they come from. The goal is that a reader can tell, from the sample itself, whether it is meant to run as shown or to illustrate a piece of a larger whole. Reproducibility is not required for every snippet, but clarity about what is reproducible is.

### Verify Output And Results Match The Claims

Many code samples are followed by the output they produce, and that output must match what the code actually generates. Mismatched output, where the shown result differs from the real result, misleads readers who compare their own output to the documented example and find a discrepancy they cannot explain. Output verification is part of code verification.

For each sample with shown output, confirm the output matches what the code produces when run. Watch for output that reflects an earlier version of the code, edited for brevity without noting the edit, or cleaned up in ways that obscure the real result. For output that varies, due to randomization, environment, or timing, note the variability rather than presenting a single result as definitive. For error messages and stack traces, confirm they match what the described failure would actually produce, since fabricated or inaccurate error text misleads readers troubleshooting real problems. Output is part of the example, and it must be as accurate as the code itself.

### Check Commands Paths And Environment Specifics

Command-line examples, file paths, and environment-specific details are frequent sources of broken documentation. Commands that work on one operating system fail on another, paths that exist in one environment do not in another, and shell-specific syntax breaks for users of different shells. These specifics must be verified and, where relevant, generalized or noted.

For command examples, confirm they work as shown and note any environment dependence. Check that file paths use appropriate conventions or are clearly framed as examples, since a path that works on the author's system may not on the reader's. For shell commands, note shell-specific syntax and provide alternatives where the audience spans shells or operating systems. Watch for commands that assume specific tools are installed, specific directories exist, or specific configurations are in place, and document those assumptions. Environment specifics are not errors, but unstated assumptions about them cause examples to fail for readers whose environments differ from the author's.

### Coordinate Code Updates With Software Changes

Documentation code samples have a maintenance burden that prose does not: the software they document changes, and examples that were correct can become wrong without any change to the document. Code verification is not only a pre-publication task but an ongoing discipline, and documentation that is not maintained against software changes decays in accuracy over time.

Establish a process to verify code samples against software changes, particularly for API or library updates that affect documented examples. When the software releases a new version, check whether documented examples still work and update them where they do not. Note the version or date the examples were verified against, so readers and future editors know the currency of the documentation. For documentation that ships with the software, verify examples as part of the release process rather than treating them as static text. Code samples are a living part of the documentation, and their accuracy depends on maintenance that prose does not require.

## Common Traps

### Reading Code Instead Of Running It

Visual review cannot catch runtime errors. Execute every sample in an environment close to the reader's.

### Version Drift Between Code And API

Examples correct when written become wrong as APIs change. Verify against the targeted version.

### Omitted Imports And Prerequisites

Missing dependencies cause examples to fail for readers who follow the document faithfully. Show what is needed.

### Fragments Mistaken For Complete Programs

Unclear framing leads readers to attempt running excerpts. Distinguish complete examples from illustrative fragments.

### Output Mismatching The Code

Shown results that differ from real output mislead readers comparing their results. Verify output accuracy.

### Environment-Specific Commands Unmarked

Commands and paths that assume a specific environment fail for others. Note or generalize environment specifics.

### Static Documentation Against Changing Software

Unmaintained examples decay as the software evolves. Verify and update samples with each release.

### Copied Examples From Other Sources

Samples borrowed from elsewhere may reflect different APIs or contexts. Verify against the current document.

## Self-Check

Before treating code sample verification as complete, verify:

- Every code sample has been executed in an environment close to the reader's, with unrunnable samples documented as such.
- Examples match the APIs, libraries, and versions the document describes, with version-specific behavior noted.
- All imports, dependencies, and prerequisites needed to run each sample are shown or clearly referenced.
- Complete runnable examples are distinguished from illustrative fragments, with framing that prevents confusion.
- Shown output matches what the code actually produces, with variability noted where it exists.
- Commands, paths, and environment-specific details are verified and marked or generalized for the audience.
- A maintenance process exists to verify and update samples against software changes, with verification dates noted.
- No sample depends on information, setup, or context the document does not provide.
- A reader following the document faithfully could run each complete example and obtain the shown result.
