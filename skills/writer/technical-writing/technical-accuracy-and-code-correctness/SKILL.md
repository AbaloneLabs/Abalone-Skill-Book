---
name: technical_accuracy_and_code_correctness.md
description: Use when the agent is ensuring technical accuracy in documentation, verifying code examples run, checking API accuracy and version-specific behavior, testing instructions, keeping docs in sync with code changes, documenting known limitations, or setting up review processes for technical correctness.
---

# Technical Accuracy And Code Correctness

In technical writing, accuracy is not a quality concern; it is the foundation. A literary essay that contains a small factual error may still enlighten. A technical document that contains a small error may waste hours of a reader's time, break their build, send them down a wrong path, or destroy their trust in the entire documentation set. The reader of technical docs is usually trying to do something, and they are relying on the document to describe reality. When the document describes something that is not true, the reader's first failure is blamed on themselves, then on the system, and only later on the documentation, by which point the damage is done.

The common failure is to treat accuracy as a matter of careful writing rather than of verification. The writer drafts the code example from memory, describes the API as they believe it works, and ships. The document reads well. It is wrong. Code rots: the API changed last release, the flag was renamed, the default behavior shifted, the example that worked in version two fails in version three. A document that was accurate when written becomes inaccurate through no fault of the writer, unless the writer built verification and maintenance into the process. The skill is in treating accuracy as something that must be actively produced and actively maintained, not assumed.

Use this skill when verifying technical accuracy, when writing or checking code examples, when documenting version-specific behavior, when testing instructions, when keeping docs synchronized with a changing codebase, or when deciding how to document known limitations. The agent has freedom in depth and format, but the obligation to ensure the document describes reality is not optional.

## Core Rules

### Treat Accuracy As Verification, Not As Careful Writing

Accuracy cannot be achieved by writing carefully. It can only be achieved by checking the document against the reality it describes. A code example must be run. An API signature must be checked against the actual interface. A described behavior must be reproduced. A stated default must be confirmed against the current configuration. The writer's belief about how the system works is not evidence; the system's actual behavior is. Build verification into the writing process, so that every factual claim is checked against a source of truth.

Verification practices:

- run every code example, in a clean environment, exactly as the reader would;
- check every API signature, parameter, and return type against the actual code or spec;
- confirm every stated default, limit, and version requirement against the source;
- reproduce every described behavior, including the error cases;
- cite the version or commit the document was verified against.

A document whose claims have been verified is trustworthy. A document whose claims are plausible is a hazard.

### Run Every Code Example

Code examples are the most failure-prone element of technical documentation, because they are specific, executable, and version-sensitive. An example that is almost right fails completely. The only reliable way to ensure an example works is to run it, in the environment and version the reader will use, and confirm it produces the stated result. Writing code from memory or copying from a working session without re-running it invites errors: a missing import, a typo in a method name, a changed default that breaks the flow.

For every code example:

- run it from scratch, in a clean environment, not in the session where it was developed;
- confirm it produces the output the document claims;
- include all necessary imports, setup, and context, so the reader can reproduce it;
- test it in the version the document targets, and note the version;
- check that it still works after any edit to the prose around it, because edits introduce errors.

An example the reader cannot run is worse than no example, because it wastes their time and erodes trust.

### Document Version-Specific Behavior Explicitly

Technical systems change across versions. An API that accepted a string in version one may require an object in version two. A default that was true may become false. A feature may be added, deprecated, or removed. A document that does not specify which version it describes becomes inaccurate the moment a new version ships, and the reader has no way to know whether the document applies to their situation. Version specificity is not optional detail; it is the context that makes the document usable.

Version documentation practices:

- state the version or version range the document applies to, prominently;
- note where behavior differs across versions, especially breaking changes;
- mark deprecated features as deprecated and point to the replacement;
- date the document or tie it to a release, so the reader can judge currency;
- plan to update version-specific content when new versions ship.

A reader should never have to guess whether the document describes their version. If the answer is unclear, the document has failed them.

### Test The Instructions As A Reader Would

Instructions that the writer has not followed literally are untested. The writer's familiarity leads them to fill in steps mentally that the document omits. The remedy is to follow the instructions exactly as written, doing only what the document says, in a clean state, and noting every place the instructions are insufficient. This is the single most effective accuracy check, because it reveals the gaps the writer's knowledge conceals.

When testing instructions:

- start from a clean environment, not the writer's working setup;
- follow only what is written, not what you know should happen;
- note every assumption the document makes that you had to supply;
- note every step that failed or produced a different result;
- revise the document based on what the test revealed, then test again.

Instructions that survive a literal following are trustworthy. Instructions that have only been read are not.

### Keep Docs In Sync With Code Changes

Documentation drifts from reality as the code changes. An accurate document becomes inaccurate through no error of the writer, simply because the system evolved. Managing this requires a process, not just care. Documentation that describes code should be coupled to that code's lifecycle, so that changes to the code trigger review of the docs. Without this coupling, docs rot silently until a reader discovers the rot the hard way.

Sync practices:

- treat documentation changes as part of the code change, in the same PR or ticket;
- link docs to the code they describe, so changes surface the related docs;
- schedule periodic reviews of docs against current code, especially before releases;
- mark docs with the version or date verified, so stale docs are identifiable;
- provide a path for readers to report inaccuracies, and act on those reports.

Accuracy is not a one-time achievement but a maintained state. Plan for maintenance from the start.

### Document Known Limitations Honestly

No system is without limitations, and a document that presents only the happy path misleads the reader. Known limitations, edge cases, failure modes, and unsupported scenarios are part of accurate documentation. Documenting them honestly builds trust and saves the reader from discovering them through failure. A document that admits its system's limits is more credible, not less, because it signals the writer knows the system rather than is selling it.

Document limitations by:

- listing known constraints, such as maximum sizes, unsupported platforms, or excluded features;
- describing failure modes and how to recognize them;
- noting workarounds where they exist, and stating clearly where they do not;
- distinguishing permanent limitations from temporary ones, such as planned future support;
- avoiding the temptation to hide limitations to make the system look more capable.

A reader who hits an undocumented limitation blames the documentation. A reader who hits a documented limitation was warned and can plan around it.

### Establish A Technical Review Process

The writer cannot catch every accuracy error alone, because the same knowledge gaps that produce errors hide them. A review process, where someone with current knowledge of the system checks the document against reality, catches what the writer missed. The reviewer should verify claims, run examples, and confirm the document matches current behavior. For high-stakes documentation, such as APIs used in production, technical review should be mandatory, not optional.

Review process elements:

- assign a reviewer who knows the current state of the system;
- have the reviewer verify factual claims and run code examples;
- check that version and limitation information is current;
- review before publication and after any significant code change;
- treat reader-reported inaccuracies as review input and feed corrections back.

## Common Traps

### Treating Accuracy As Careful Writing

Writing carefully does not produce accuracy; verification does. A well-written document that has not been checked against reality is a hazard. Verify every claim.

### Code Examples That Have Not Been Run

Examples written from memory or copied from a session fail in ways the writer did not anticipate. Run every example in a clean environment.

### Unstated Version Assumptions

A document that does not specify its version becomes inaccurate and unusable as versions change. State the version and note differences.

### Untested Instructions

Instructions the writer has not followed literally contain hidden gaps. Follow them exactly as written, in a clean state, and fix what fails.

### Docs That Drift From Code

Without a sync process, accurate docs rot as the code changes. Couple doc review to code changes and schedule periodic reviews.

### Hiding Limitations

A document that presents only the happy path misleads readers who hit the limits. Document known limitations honestly to build trust and save reader time.

### No Technical Review

The writer's knowledge gaps hide their own errors. Use a reviewer with current knowledge to verify claims and run examples, especially for high-stakes docs.

### Stale Examples After Edits

Editing the prose around a code example can break the example. Re-run examples after any edit to the surrounding content.

## Self-Check

Before treating the document as technically accurate, verify:

- Every factual claim was verified against the system's actual behavior, not written from belief or memory.
- Every code example was run in a clean environment, in the target version, and produces the stated result.
- The version or version range the document applies to is stated prominently, with differences noted.
- Instructions were followed literally, as a reader would, in a clean state, and gaps were fixed.
- A process exists to keep the documentation in sync with code changes, including review on change.
- Known limitations, edge cases, and failure modes are documented honestly rather than hidden.
- A technical reviewer with current knowledge checked the document against reality before publication.
- Examples were re-run after any edit to surrounding prose, because edits introduce errors.
- The document is dated or versioned so readers can judge its currency.
- A reader following the document would succeed without hitting an inaccuracy the writer could have caught.
