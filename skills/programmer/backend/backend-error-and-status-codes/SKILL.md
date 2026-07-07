---
name: backend_error_and_status_codes.md
description: Use when the agent is designing or reviewing backend error handling, HTTP/gRPC status code selection, error response shapes, retryability classification, mapping domain errors to transport errors, idempotency and partial-failure responses, bulk/batch operation errors, or diagnosing wrong status codes (404 vs 400, 401 vs 403, 409 vs 422), 500-for-everything anti-patterns, errors that clients cannot retry, leaked internal details, or ambiguous error responses. Covers status code semantics, retryable vs terminal classification, error response contracts, partial success in bulk APIs, and the boundary between domain errors and transport errors.
---

# Backend Error And Status Codes

Backend error handling is the discipline of translating "something went wrong" into a response a client can act on correctly: retry, surface to the user, fix its request, or escalate. It looks like a lookup table ("validation error → 400") and it is a contract design problem that determines whether clients retry intelligently or hammer a broken dependency, whether users see a helpful message or a stack trace, whether an outage cascades or is contained. A blanket 500 for every error prevents clients from distinguishing a transient network blip (retry) from a validation failure (do not retry) from a permanent server bug (escalate). A 200-with-error-body breaks HTTP semantics and every client that checks status. A 404 returned for "you do not have permission" leaks the existence of a resource. Each is a small mistake that produces large, systemic failure under real conditions.

The judgment problem is not "what is the list of status codes" but "what does the client need to know to act correctly, what must not leak, and is the error classification consistent and stable." Agents tend to pick status codes by gut feel (everything validation-shaped is 400, everything else is 500), to leak internal structure in error bodies (SQL messages, stack traces, internal ids), and to forget that errors are a public API that clients depend on and that must therefore be stable and documented. The cost ranges from clients that cannot recover (retrying non-retryable errors, or failing permanently on transient ones) to security leaks to brittle integrations that break on every error-shape change.

## Core Rules

### Classify Every Error By What The Client Should Do, Not By What Went Wrong

The purpose of a status code is to tell the client what to do next, and the classification that matters is action-oriented: should the client retry (the failure is transient — a timeout, a downstream 503, a connection reset), should the client fix its request (the request is malformed or invalid — a 400/422), should the client give up permanently (the request is correct but the server cannot satisfy it — a 404, a 409 conflict, a 422 business-rule violation), or should the client authenticate/authorize differently (401/403)? This classification is more important than the precise code: a client that knows "retryable vs terminal" can recover; a client that only knows "error" cannot.

Encode retryability explicitly and consistently. A 408, 425, 429, 500, 502, 503, 504 are conventionally retryable (with backoff); a 400, 401, 403, 404, 405, 409, 410, 422 are not. When in doubt, document the retryability in the error response (a `retryable: true/false` field, or a `Retry-After` header for 429/503) so clients do not have to guess from the status code. A response that is retryable but coded as 400 will never be retried; a response that is terminal but coded as 500 will be retried forever. Both waste resources and fail to recover.

### Use The Correct Status Code, And Know The Common Confusions

HTTP status codes have specific semantics, and the common confusions recur. **401 vs 403**: 401 means "not authenticated" (who are you? — the client must provide credentials); 403 means "authenticated but not authorized" (I know who you are, but you cannot do this — the client should not retry with the same credentials). Returning 401 for an authorized-but-forbidden request prompts the client to re-authenticate pointlessly; returning 403 for an unauthenticated request leaks that the resource exists. **404 vs 403**: when a resource exists but the user lacks permission, returning 404 (rather than 403) is a deliberate choice to hide existence — acceptable for some threat models, but be consistent and intentional, not accidental. **400 vs 422**: 400 is for malformed requests (bad JSON, missing required field, wrong type); 422 is for well-formed requests that fail semantic validation (a date in the past, a value out of business range). Many APIs use 400 for both, which is acceptable if documented, but 422 is more precise. **409 vs 422**: 409 is for a conflict with the current state of the resource (duplicate id, concurrent modification, version conflict); 422 is for semantic invalidity independent of state.

Know the codes you use, document why, and be consistent across the API. A client that has to learn "in this API, 400 means validation, but in that endpoint 400 means auth" has been given an inconsistent contract that will produce bugs. Treat the status code mapping as a documented, reviewed part of the API, not an afterthought.

### Never Return 200 For An Error, And Never Return 5xx For A Client Problem

Two anti-patterns dominate. The first is returning `200 OK` with an error body (`{ "error": "..." }`), usually justified as "it makes client handling uniform." It does the opposite: it breaks HTTP semantics, it defeats every client that checks status (proxies, load balancers, monitoring, retry logic), and it forces every client to parse the body to know if the request succeeded. Status codes exist precisely so that success and failure are distinguishable without parsing the body; use them. The second is returning `500 Internal Server Error` for client problems (validation failures, not-found, unauthorized), usually because the handler caught everything in one bucket. A 500 means "the server broke" and clients/middleware treat it as retryable and as a server-side incident; using it for a 400-shaped problem triggers retries that cannot succeed and pages on-call for a client bug.

Map domain errors to the correct status at the boundary, in one place (a handler-level mapper or a framework interceptor), based on the error type. Business logic returns domain errors (`UserNotFound`, `InvalidEmail`, `InsufficientStock`, `ConcurrentModification`); the boundary translates each to its status. This keeps the mapping consistent, documented, and changeable without touching business logic.

### Make Error Responses Structured, Stable, And Documented

An error response is a public API. Clients parse it, branch on it, and depend on its shape, so it must be structured, stable, and documented — just like a success response. A good error response has: a stable `code` or `error` identifier (a machine-readable string like `INSUFFICIENT_STOCK`, not a localized message), a human-readable `message` (safe to show users, no internal detail), optional `details`/`field` information (which field failed validation, which constraint), and a `requestId`/`traceId` for correlation with server logs. The shape must be consistent across the API: every error has the same fields, in the same structure, regardless of the underlying cause.

Stability matters because clients depend on it. Do not rename error codes, do not restructure the response, do not change a field's meaning, without a versioning strategy — clients will break. Document every error code the API can return, what it means, and whether it is retryable, as part of the API contract. An undocumented error is a support ticket waiting to happen; an error shape that changes between releases is a broken integration.

### Do Not Leak Internal Details In Error Responses

Error responses cross a trust boundary to an untrusted client, and they must not leak implementation detail. A raw SQL error (`duplicate key value violates unique constraint "users_email_key"`) leaks the schema. A stack trace leaks the language, framework, and library versions. An internal id (`user 4827193 not found`) leaks identifier structure. A library message (`connection refused to 10.0.4.23:5432`) leaks the internal network topology. Each of these is an information disclosure that helps an attacker map the system, and each is a common mistake when error handling just serializes the underlying exception.

Map internal errors to safe external messages at the boundary: a generic "internal error" with a request id for 500s, a domain-specific message for business errors, and the full detail logged server-side (with the request id) for debugging. Never serialize raw exceptions, stack traces, SQL, or internal identifiers to the client. Review error responses for secrets (tokens, passwords, PII) that might appear in messages or detail fields — a `Debug` derive on an error struct can quietly include fields you did not mean to expose.

### Handle Partial Failure In Bulk And Batch Operations Deliberately

A bulk or batch endpoint (POST many items, process a list) has a failure mode that single-item endpoints do not: some items succeed and some fail. Three strategies exist, and the choice is a contract decision. **All-or-nothing** (transactional): the whole batch succeeds or the whole batch fails — simplest for the client, but requires a transaction and rejects valid items when one is invalid. **Partial success with per-item results**: the response is 200 (or 207 Multi-Status) with a per-item status, so the client knows which succeeded and which failed and can retry just the failures — most flexible, but the client must handle a complex response. **Fail-fast on first error**: stop at the first failure and return which items were processed before the stop — predictable but wasteful if most items would have succeeded.

Choose based on the semantics and document it. Never silently process some items and fail others without telling the client which — a bulk update that reports 200 but only applied half the items is a data-integrity bug from the client's perspective. For partial-success responses, use 207 Multi-Status (or a documented 200-with-per-item-results) and make the per-item status machine-readable so clients can retry precisely the failures. Make each item's processing idempotent (see below) so a retry of the failed items does not re-process the succeeded ones.

### Make Error Handling Idempotent And Safe Under Retry

Because clients retry, error handling must be correct under re-execution. If a request times out after the server applied the change but before the client received the response, the client will retry — and the server must not apply the change twice. This requires idempotency: either the operation is naturally idempotent (PUT, DELETE), or the client sends an idempotency key that the server uses to deduplicate within a window. Without idempotency, a retried timeout on a charge or an insert double-charges or double-inserts.

The error path must also be idempotent. A validation error that the client fixes and retries must succeed on the retry; a conflict (409) on a retried create must be handled as "already created" rather than "failed." Store the idempotency key with the result so a retry returns the original result (success or failure) rather than re-processing. This is the same discipline as in the concurrency and request-handling skills: assume every mutating request is re-sent, and design so re-sending is safe.

### Distinguish Client Errors From Server Errors In Observability

4xx errors are client problems; 5xx errors are server problems, and they must be treated differently in monitoring and alerting. A spike in 400s is clients sending bad requests (possibly a documentation gap, a client bug, or an attack) — worth investigating but not an on-call emergency. A spike in 500s is the server breaking — that is an incident. Alert on 5xx rate (and on specific 5xx codes like 503 for dependency outage), not on 4xx rate, and dash-board them separately so client errors do not drown out server errors.

Within 5xx, distinguish retryable (502, 503, 504 — downstream problems) from non-retryable (500 — a bug). A rising 500 rate is a bug introduced by a deploy; a rising 503 rate is a dependency degrading. Each warrants a different response. Instrument error rates by code, by endpoint, and by underlying cause, so you can see not just "errors are up" but "validation errors on the signup endpoint are up" or "dependency timeouts on the payment endpoint are up."

## Common Traps

### 500 For Every Error

A blanket 500 prevents clients from distinguishing retryable from terminal and triggers retries on client bugs. Map domain errors to correct statuses at the boundary.

### 200 With An Error Body

Returning success status with an error payload breaks HTTP semantics and every client that checks status. Use the correct error status code.

### 401 For Authorization Failures

Returning 401 (not authenticated) when the user is authenticated but unauthorized prompts pointless re-authentication; use 403. Returning 403 for unauthenticated requests leaks resource existence.

### Leaking Internal Detail In Errors

Raw SQL, stack traces, internal ids, or library messages in the response body. Map to safe external messages at the boundary; log detail server-side with a request id.

### Inconsistent Or Undocumented Error Shapes

Error responses that vary by endpoint or change between releases, breaking clients. Document the error contract; treat it as a stable public API.

### Retryable Errors Coded As Terminal (And Vice Versa)

A transient timeout coded 400 is never retried; a validation error coded 500 is retried forever. Encode retryability explicitly and consistently.

### Bulk Endpoint With Silent Partial Failure

A batch update that returns 200 but only applied some items, leaving the client unaware. Use 207 or per-item results; document the partial-success contract.

### Non-Idempotent Mutating Endpoint

A create or charge with no idempotency key, double-applying on client retry. Use idempotency keys; return the original result on retry.

### Alerting On 4xx The Same As 5xx

Treating client errors as incidents, paging on-call for client bugs. Alert on 5xx (server problems); investigate 4xx separately.

## Self-Check

- [ ] Every error is classified by client action (retry / fix request / give up / authenticate) and the status code reflects that classification, with retryability documented in the response or header where ambiguous.
- [ ] Status codes are used correctly: 401 (unauthenticated) vs 403 (unauthorized) vs 404 (hide-existence, deliberate) vs 400 (malformed) vs 422 (semantic) vs 409 (conflict), and the mapping is documented and consistent across the API.
- [ ] No endpoint returns 200 with an error body, and no client problem (validation, not-found, unauthorized) is returned as 500; domain errors are mapped to correct statuses at the boundary in one place.
- [ ] Error responses are structured (stable machine-readable code, safe human message, optional field details, request id), documented as part of the API contract, and stable across releases.
- [ ] No internal detail (SQL, stack traces, internal ids, library messages, secrets) leaks in error responses; raw exceptions are mapped to safe external messages with full detail logged server-side.
- [ ] Bulk/batch endpoints have a deliberate partial-failure strategy (all-or-nothing, partial success with per-item results via 207, or fail-fast), documented and machine-readable so clients can retry precisely the failures.
- [ ] Mutating endpoints are idempotent (naturally or via idempotency key) so client retries do not double-apply, and the error path returns the original result on retry rather than re-processing.
- [ ] Observability distinguishes 4xx (client problems, investigated not paged) from 5xx (server problems, alerted), and within 5xx distinguishes retryable downstream failures (502/503/504) from bugs (500), with error rates instrumented by code, endpoint, and cause.
- [ ] The error contract was reviewed as a public API: codes documented, shapes stable, retryability explicit, and no information disclosure.
