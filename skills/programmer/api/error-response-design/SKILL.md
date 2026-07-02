---
name: error_response_design.md
description: Use when the agent is designing API error responses, choosing HTTP status codes, defining error code taxonomies, structuring machine-readable error payloads, handling partial failures across batch endpoints, or coordinating error semantics with idempotency and retries.
---

# Error Response Design

Error responses are a load-bearing part of an API contract, yet they are routinely treated as an afterthought. Developers consuming an API rely on errors far more than success paths during integration, and the shape of an error determines whether a client can retry safely, surface a useful message to an end user, or distinguish a transient fault from a permanent misuse. A weak error design forces every consumer to guess, to add heuristics like string-matching messages, or to treat all failures as fatal.

The judgment problem is not "return an error object." It is deciding what categories of failure exist, how a client should distinguish them, what information may safely be revealed, how errors compose across partial and batch operations, and how error semantics interact with retries and idempotency. An error that omits a retry hint causes retry storms; an error that leaks internal stack traces creates security exposure; an error that overloads `500` for both transient and permanent faults makes recovery logic impossible.

This skill applies whenever you are defining, reviewing, or refactoring how an API communicates failure. The agent has latitude to choose conventions, but should not invent ad-hoc per-endpoint error shapes.

## Core Rules

### Decide the failure taxonomy before the payload shape

Before choosing fields, enumerate the categories of failure your API can produce. A useful baseline split is four orthogonal axes:

- **Client faults** (the request is wrong and will stay wrong if retried unchanged): malformed input, schema violations, unauthorized access, missing prerequisites, conflict with current state.
- **Server faults** (something broke on your side and may succeed on retry): internal exceptions, dependency timeouts, resource exhaustion.
- **Transient external faults** (a downstream dependency failed temporarily): upstream rate limits, temporary unavailability.
- **State conflicts** (the request was well-formed but cannot apply to the current resource state): stale version, duplicate idempotency key with different payload, concurrent modification.

Each category implies a different client response. Mixing them into one shape forces clients to reverse-engineer intent.

### Use HTTP status codes for their intended semantics

Status codes are a coarse but standardized signal. Clients and proxies (caches, load balancers, SDKs) interpret them, so misusing a code silently breaks intermediaries.

- `400` for malformed/wrong input that the client can fix. Prefer problem-specific `4xx` codes when they fit: `401` (unauthenticated), `403` (forbidden), `404` (resource absent), `409` (conflict with current state), `422` (semantically valid but business-rule violation), `429` (rate limited), `412`/`428` (preconditions).
- `5xx` for server-side problems. Do not return `500` for client validation errors; it hides bugs and prevents clients from distinguishing retryable faults.
- Use `429` with a `Retry-After` header for rate limiting, not `503` unless the server itself is shedding load.
- Reserve `5xx` for genuinely unexpected internal failures. If a failure is expected and recoverable (e.g., a downstream timeout), consider whether `503` with a `Retry-After` is more honest than `500`.

Weak choice: returning `500 Internal Server Error` for a known validation rule violation. Strong choice: `422` with a code that names the violated rule.

### Make errors machine-readable, not just human-readable

A client must be able to branch on an error without parsing a message string. Provide:

- A stable, documented **error code** or type identifier (e.g., `insufficient_funds`, `version_conflict`) that is independent of the HTTP status and the message. Codes are part of the contract and must not change without a version bump.
- A stable **category** or retryability hint when useful (e.g., `retryable: true/false`), because status codes alone do not always disambiguate (a `503` may or may not be retryable depending on cause).
- Optional structured details (field-level validation errors, conflicting version numbers, idempotency key state) so clients can act programmatically.

Messages are for humans and may change at any time. Never make a client depend on message wording.

### Write user-friendly messages, but never as the contract

Messages should be clear, actionable, and free of internal jargon and identifiers. "Insufficient funds for account ending in 1234" is useful; "BalanceCheckException: null" is not. However, treat messages as documentation, not API. If a client needs to react, that information belongs in a code or structured field.

### Handle partial failure explicitly for batch and bulk endpoints

Batch endpoints are a frequent source of broken error contracts. A batch of 10 items where 3 fail is neither a `200` nor a `4xx`; it is a partial success. Decide and document:

- Whether the batch is atomic (all-or-nothing) or per-item independent.
- The response shape for partial success: per-item status, per-item error code, and an aggregate status.
- The HTTP status for partial failure. Common conventions: `207 Multi-Status`, `200` with per-item errors, or `422` if the whole batch was rejected. Pick one and document it.

Weak choice: returning `500` for a batch where some items succeeded. Strong choice: `207` with per-item results, so the client knows exactly which items to retry.

### Coordinate errors with idempotency and retries

If the API supports idempotency keys, errors must compose with them:

- A retried request with the same idempotency key must not double-execute. If the first attempt committed, the retry must return the original result, not a conflict error.
- Distinguish "conflict because the key was reused with a different payload" (permanent client error) from "conflict because of concurrent modification" (potentially retryable).
- Mark transient errors as retryable and include backoff guidance. Permanent errors must be clearly marked non-retryable so clients do not hammer a hopeless request.

### Limit information leakage

Error responses are an attack surface. Do not include stack traces, internal file paths, SQL fragments, raw exception class names, or account numbers of other users. Confirm whether a resource exists before reporting authorization failures only when enumeration is a concern (the `401` vs `404` on login tradeoff). Prefer generic messages for authentication failures and specific messages only when the caller is already authenticated.

### Version your error contract

Error codes are part of the API surface. Adding new codes is backward-compatible; removing or renaming codes is a breaking change. Document the policy: codes are additive within a major version. When you must change semantics, introduce a new code rather than repurposing an old one.

## Common Traps

### Overloading a single status code for all failures

Returning `500` for everything, including validation and auth errors, is common in early APIs. It makes client retry logic impossible because clients cannot tell transient from permanent faults, and it hides genuine bugs in noise. The trap is that it feels simpler until the first integration partner asks how to retry safely.

### Relying on message strings for control flow

If documentation tells clients to "check if the message contains 'duplicate'," the contract is already broken. Messages get localized, reworded, or truncated. Any branching logic belongs in a code or structured field.

### Making every error retryable (or none)

Marking all errors retryable causes retry storms against permanent failures; marking none retryable defeats the purpose of retries. Retryability is a per-error property, not a global one. A `409 version_conflict` may be retryable; a `400 invalid_email` never is.

### Leaking internals in error payloads

Including raw exceptions, ORM error text, or internal identifiers in production errors turns every error response into reconnaissance for an attacker. The trap is that these details are invaluable during local development, so developers leave them on. Gate detailed errors behind an explicit debug/staging flag.

### Ignoring idempotency on retried failed requests

If a request fails after partial commit and the client retries with the same key, returning a fresh error (or re-executing) can corrupt state. The trap is assuming a failed request left no side effects.

### Inconsistent partial-failure semantics across endpoints

One batch endpoint returns `207`, another returns `200` with errors buried in the body, a third returns `422` for any partial failure. This forces clients to write per-endpoint logic. Standardize across the API.

### Using 404 to hide authorization failures without thinking it through

Returning `404` instead of `403` to avoid leaking resource existence is defensible, but only when enumeration is a real threat. Applying it everywhere degrades debugging and breaks legitimate clients that need to distinguish "I lack permission" from "it does not exist."

## Self-Check

- Does every documented failure category have a distinct, stable error code independent of the HTTP status and the message?
- Are HTTP status codes used for their intended semantics, with no `500` returned for client-side validation or auth errors?
- Can a client decide whether to retry purely from structured fields (code and/or retryability), without parsing the message?
- Do batch/bulk endpoints document their atomicity model and return per-item results for partial failures?
- For idempotent endpoints, does a retried request after a partial-commit failure return the original committed result rather than re-executing or erroring?
- Are transient errors marked retryable with backoff guidance, and are permanent errors clearly marked non-retryable?
- Do error payloads in production omit stack traces, internal paths, raw exceptions, and other users' data?
- Is the error contract versioned, with codes treated as additive within a major version?
- Are user-facing messages actionable and free of internal jargon, while not being relied upon for control flow?
- Have you confirmed the chosen partial-failure HTTP convention is consistent across all batch endpoints in the API?
