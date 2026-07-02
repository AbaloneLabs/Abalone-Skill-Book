---
name: api_and_reference_documentation.md
description: Use when the agent is writing, updating, or auditing API reference docs, SDK guides, endpoint descriptions, type schemas, code examples for public interfaces, deciding what belongs in code comments versus external docs, documenting version differences, or diagnosing why consumers keep misusing an API.
---

# API And Reference Documentation

API documentation is a contract, not a courtesy. Every consumer who reads it is deciding what they can rely on, what they must not depend on, what happens when things go wrong, and how to call the interface correctly. When the docs are wrong, incomplete, or silently out of sync with the code, consumers build against a fiction — and the fiction breaks in production, on the consumer's schedule, in ways the API author never sees coming. The cost of bad API docs is paid by someone else, far away, which is exactly why it is so chronically under-invested in.

Agents tend to treat reference docs as a transcription of the code: list the parameters, list the return type, move on. That transcription is the least useful part of the documentation, because the type signature already says it. The valuable documentation answers the questions the signature cannot: what does this do, when should I use it versus the alternatives, what guarantees does it make and not make, what errors can it return and what do they mean, what is stable and what may change, and how do I handle the edge cases. The judgment problem is to write documentation that lets a consumer use the interface correctly and safely without reading its source — because most consumers never will.

## Core Rules

### Document The Contract, Not The Implementation

The most important distinction in API docs is between what the interface promises and what the current implementation happens to do. Consumers depend on promises; implementation details are free to change. Documentation that describes internal behavior as if it were a guarantee creates accidental contracts that constrain future changes and break consumers when the implementation evolves.

For each documented element, separate the two explicitly:

- **Promises** — behavior the interface guarantees and that consumers may rely on. These are part of the contract and changing them is a breaking change. Document them deliberately and sparingly; every promise narrows your future freedom.
- **Current behavior** — what the implementation does today, with no guarantee. Label these as such ("currently," "implementation note") or omit them, so consumers do not build on them.

A common failure is documenting a return ordering, a caching behavior, or an internal retry policy as if it were guaranteed. Consumers then depend on it, and a later optimization that changes it becomes a breaking change in disguise. If you are not prepared to guarantee it forever, do not document it as behavior.

### Lead With Working Examples

Most consumers do not read reference docs linearly. They arrive looking for the thing closest to their problem, and the fastest path to a correct call is a runnable example they can adapt. A reference page whose only content is a parameter table forces the reader to reconstruct usage from types; a page that opens with a copy-pasteable example gets them unblocked in seconds.

Strong examples are complete enough to run (real inputs, not `foo` and `bar`), demonstrate the common case first, then the important variations and edge cases, and show the response or outcome so the reader knows what success looks like. Examples are also executable tests of the documentation: if the example does not work, the docs are wrong. Where feasible, generate examples from code that is actually run, so they cannot drift.

### Document Errors, Edge Cases, And Failure Modes

The happy path documents itself; the failure paths are where consumers get hurt. An endpoint that returns 200 on success and silently 500s on a known failure mode, with no documentation of when or why, generates support tickets and outages. Document every error the interface can return, what triggers it, and what the consumer should do about it.

Cover at minimum:

- **Error responses** — status codes, error shapes, and the conditions that produce each. Distinguish client errors from server errors and from partial-success responses.
- **Edge cases** — empty inputs, maximum inputs, concurrent calls, idempotency guarantees, timeouts, and what happens on retry.
- **Resource limits** — rate limits, payload size limits, pagination behavior, and what happens at the boundaries.
- **Side effects and ordering** — what state changes, whether calls are idempotent, and what ordering guarantees exist.

A consumer who knows the failure modes can build resilient code; one who does not will guess, and guesses about distributed systems are usually wrong.

### Make Stability And Versioning Explicit

Consumers need to know what they can depend on across versions. Documentation that does not state stability leaves consumers to infer it, and they usually infer too much. Mark each part of the interface with its stability: stable and versioned, experimental and subject to change, deprecated and scheduled for removal, or internal and not for external use.

When the interface has version differences, document them where the consumer will see them — on the relevant endpoint or type, not buried in a changelog. A consumer reading the v2 docs for an endpoint should immediately see what changed from v1, what was added, and what was removed or renamed. Cross-version behavior differences are a leading cause of integration bugs, and they are invisible unless documented at the point of use.

### Keep Docs And Code In Sync, Or Make The Drift Visible

Documentation rot is the slow death of reference docs. Code changes, docs do not, and within a few releases the docs describe an interface that no longer exists. The consumer who follows the stale docs gets errors and loses trust in all the documentation, including the parts that are still correct.

Treat doc-code sync as a first-class concern:

- **Co-locate docs with code** where the language supports it (doc comments, schema annotations), so a change to the interface is physically adjacent to the docs it must update.
- **Generate reference from source** where feasible, so the parameter list and types cannot drift; supplement with prose for the contract and examples.
- **Test the examples** so a breaking change fails a test rather than silently breaking the docs.
- **Mark docs with freshness signals** (last reviewed, corresponding version) so a reader can judge whether to trust them.

Where sync cannot be automated, make the drift visible: a changelog that calls out doc updates, a CI check that flags interfaces without docs, or an ownership model that makes someone responsible for keeping a section current. Undocumented drift is worse than acknowledged staleness.

### Choose Code Comments Versus External Docs Deliberately

Not all documentation belongs in the same place. Code comments, doc comments, READMEs, and external reference sites serve different audiences and decay at different rates. Putting the wrong content in the wrong place guarantees it will be ignored or rot.

Rough guidance:

- **Code comments** explain why non-obvious code is written the way it is, for the maintainer reading the source. They rot if they describe what the code does (the code already says that).
- **Doc comments** document the public contract of an interface, for the consumer. They belong on public symbols and should travel with the code.
- **External docs** cover guides, tutorials, architecture, and cross-cutting concerns that do not belong to a single symbol. They reach audiences who never open the source.
- **Changelogs and migration guides** capture what changed between versions, for upgraders.

When the same information is needed in multiple places, generate from a single source rather than duplicating, so updates propagate. Duplicated prose is documentation debt that diverges the moment one copy is edited and the other is not.

## Common Traps

### Transcribing The Signature And Calling It Docs

A parameter table that restates the types adds no value; the consumer can read the signature. Documentation earns its place by carrying what the signature cannot — intent, guarantees, errors, examples, and stability. If the docs could be deleted and the consumer would lose nothing, the docs are not doing their job.

### Documenting Implementation Behavior As A Contract

Describing current internal behavior (ordering, caching, retries, internal state) as if it were guaranteed turns implementation details into load-bearing contracts. Consumers depend on them, and the next refactor breaks them. Either commit to the behavior as a promise, or label it clearly as non-guaranteed.

### Examples That Cannot Run

An example with placeholder values like `YOUR_API_KEY` and `foo`/`bar` inputs looks like documentation but teaches nothing, because the consumer cannot run it to see the outcome. Worse, an example that has drifted from the real interface fails when copied. Make examples runnable and, where possible, generated or tested.

### Silent Version Drift

Shipping a new version with changed behavior and no doc update leaves consumers integrating against a fiction. Version differences must be documented at the point of use, not assumed to be obvious. A consumer who upgrades and finds their integration broken with no warning has been failed by the documentation.

### Omitting The Failure Modes

Documenting only success leaves consumers to discover errors in production. Every error path, edge case, and limit that a consumer might hit should be documented with its cause and remedy. The absence of failure documentation is not simplicity; it is a support burden waiting to happen.

### Docs That Live Far From The Code

Documentation in a separate wiki or doc site that no one updates when the code changes rots faster than code-adjacent docs. Prefer co-location and generation; where external docs are necessary, couple their update to the code change through process or CI so they cannot drift silently.

### Over-Documenting Trivial Code

Adding doc comments to every getter and restating the obvious ("returns the name") is noise that trains readers to skip comments, including the important ones. Document what is non-obvious, contractual, or easy to get wrong; let the code speak for itself where it is clear.

## Self-Check

- [ ] The documentation distinguishes promises (guaranteed contract) from current behavior (implementation detail), and nothing is presented as guaranteed that the author is not prepared to maintain.
- [ ] Each documented interface leads with a runnable, realistic example showing the common case and key variations, and the examples are tested or generated so they cannot drift.
- [ ] Error responses, edge cases, resource limits, idempotency, and side effects are documented with their triggers and remedies, not only the happy path.
- [ ] Stability and versioning are marked per interface element, and version differences are documented at the point of use, not buried in a changelog.
- [ ] Reference content is generated from source or co-located with code where feasible, and examples are run in CI so doc drift fails a build.
- [ ] Information that must appear in multiple places is generated from a single source rather than duplicated prose.
- [ ] Code comments explain non-obvious "why" for maintainers, doc comments carry the public contract, and trivial code is not over-documented.
- [ ] A consumer could use the interface correctly and handle its failures without reading the source code.
- [ ] No documented behavior has silently drifted from the implementation; where staleness exists, it is marked with a freshness or version signal.
