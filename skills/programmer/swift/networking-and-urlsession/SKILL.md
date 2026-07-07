---
name: swift_networking_and_urlsession.md
description: Use when the agent is writing Swift networking with URLSession, async/await networking with URLSession, data tasks and download/upload tasks, URLRequest configuration, HTTP headers and status codes, JSON encoding/decoding with Codable over the network, authentication and tokens, retries and timeouts, certificate pinning and ATS, WebSocket, background sessions, or is diagnosing "the request hangs", "the JSON decode fails", "the token is not sent", ATS errors, retry storms, race conditions in concurrent requests, or "the completion handler is never called". Covers URLSession task lifecycle, async/await bridging, Codable at network boundaries, error modeling, security (ATS/pinning), and the pitfalls of retries, concurrency, and cancellation.
---

# Networking And URLSession In Swift

Networking looks simple in Swift — `URLSession.shared.dataTask(with: url) { ... }` — and that simplicity hides a large surface of failure modes that dominate production network code. A request can hang silently (no timeout configured, a task never resumed), fail at the transport layer (connection reset, DNS), fail at the HTTP layer (a 500 with a body that looks like success), fail at the decode layer (the server's JSON does not match `Codable`), fail at the security layer (App Transport Transport Security blocks the domain, a pinned certificate changed), or fail at the concurrency layer (a retry storm, a race between two requests, a task that is never cancelled when the view disappears). The judgment problem is to treat the network as an untrusted, slow, failing boundary: configure timeouts and the task lifecycle, decode defensively with typed errors, model HTTP status explicitly, handle authentication and retries with backoff, respect cancellation, and configure security (ATS, pinning) deliberately.

Agents fire off `dataTask` calls, assume the completion runs on the main thread (it does not), decode with a single `catch` that flattens every failure to a generic error, retry on any failure with no backoff (a retry storm on a down server), and never cancel in-flight requests when the user navigates away. The remedy is to use the async/await `URLSession` API for new code, to decode with `Codable` behind a typed error model, to distinguish transport/HTTP/decode failures, to retry with exponential backoff and only on idempotent, transient failures, and to tie request lifetime to the view's lifetime with cancellation.

## Core Rules

### Configure Timeouts And Always Resume The Task

A `URLSessionDataTask` does nothing until you call `resume()`; forgetting it is the classic "the request never fires" bug. Configure timeouts (`timeoutIntervalForRequest`, `timeoutIntervalForResource`) on the session or request so a stalled server does not hang the call forever (the default is 60s, often too long for a user-facing request). Use a dedicated `URLSession` (not always `.shared`) when you need custom configuration (timeouts, caching, headers, cellular policy); `.shared` is fine for simple cases but cannot be reconfigured.

- Call `task.resume()` for callback-based tasks; the async `URLSession.data(for:)` API resumes automatically.
- Set `timeoutIntervalForRequest` to a value appropriate to the call (short for user-facing, longer for large uploads).
- Use a configured `URLSession` per API client so timeout/cache/header policy is consistent.

### Use async/await URLSession For New Code, Bridge Callbacks Where Needed

Modern Swift networking uses `try await URLSession.data(for: request)`, which is cancellable via `Task` cancellation, returns on a cooperative executor, and integrates with structured concurrency. Prefer it for new code. For callback-based APIs (legacy URLSession delegates, third-party SDKs), bridge with `withCheckedThrowingContinuation`/`withCheckedContinuation`, and ensure the continuation is resumed exactly once (resume twice is undefined; never resuming leaks the task). Cancellation propagates: cancelling the `Task` cancels the await; for delegate/continuation bridges, check for cancellation and cancel the underlying task.

- `let (data, response) = try await URLSession.data(for: request)` — the modern default.
- Bridge callbacks with continuations resumed exactly once; cancel the underlying task on `Task` cancellation.
- Do not call legacy delegate APIs when the async API suffices.

### Model HTTP, Transport, And Decode Failures As Distinct Typed Errors

A network call fails at one of three layers, and flattening them to a single `Error` loses the information needed to retry, display, or recover. Transport failures (no connection, timeout, reset) come from `URLError` codes (`.notConnectedToInternet`, `.timedOut`, `.cancelled`). HTTP failures (4xx/5xx) require checking the `HTTPURLResponse` status code — a 404 or 500 is not an error from URLSession's perspective; the request "succeeded" with a non-2xx response. Decode failures come from `JSONDecoder`. Model these as a typed error enum (`NetworkError.transport(URLError)`, `.http(status:code:body:)`, `.decoding(DecodingError)`) so callers can branch: retry on transient transport/5xx, surface 4xx to the user, fix the decode mismatch.

- Always check `HTTPURLResponse.statusCode`; do not assume a non-throwing `data(for:)` means a 2xx.
- Map `URLError` codes to retry decisions (retry `.timedOut`/`.networkConnectionLost`, do not retry `.cancelled`/`.badURL`).
- Preserve the response body on HTTP errors so the caller can read the server's error message.

### Decode Defensively And Pin The Codable Contract

`Codable` decoding fails when the server's JSON does not match the Swift type — a missing key, a wrong type, a date format mismatch, an unexpected null. Make the type tolerant where the server is inconsistent (optional fields for keys that may be absent) and strict where the contract matters (required fields non-optional). Configure the decoder for the server's conventions: a consistent `dateDecodingStrategy` (ISO 8601, milliseconds, custom), a `keyDecodingStrategy` if the server uses snake_case. Log `DecodingError` with the key path (`context.codingPath`) so mismatches are diagnosable, not just "failed to decode." Treat the server response as an untrusted boundary: the type is a hope, and the decode validates it.

- Set `decoder.dateDecodingStrategy = .iso8601` (or the server's format) consistently.
- Use optional fields for genuinely optional server data; required fields non-optional so a missing key fails loudly.
- Capture `DecodingError.context.codingPath` in logging to pinpoint the mismatch.

### Retry With Backoff Only On Transient, Idempotent Failures

Retrying every failure produces a retry storm that takes down an already-struggling server and wastes battery. Retry only transient failures (timeout, connection lost, 5xx for idempotent methods), with exponential backoff and jitter, and a bounded attempt count. Do not retry non-idempotent operations (POST that creates a resource) unless you can make them idempotent (an idempotency key), and do not retry 4xx (client errors — the same request will fail the same way). Cancel retries when the task is cancelled.

- Retry `.timedOut`, `.networkConnectionLost`, and 5xx for GET/PUT/DELETE; do not retry 4xx or `.cancelled`.
- Exponential backoff with jitter (`base * 2^attempt + random`), capped attempts.
- For non-idempotent POST, use an idempotency key so a retry does not double-create.

### Tie Request Lifetime To The View With Cancellation

A request started when a view appears should be cancelled when the view disappears, or it continues (wasting bandwidth, decoding into a deallocated view, racing with a newer request). With async/await, scope the request in a `Task` stored on the view/view model and cancel it on disappear; structured concurrency (`Task`/`taskGroup`) propagates cancellation to the URLSession await. For callback tasks, keep the `URLSessionDataTask` and call `cancel()`. Guard against late completion of a cancelled request updating stale UI (check the current state, or use a request id to ignore out-of-order responses).

- Store the `Task` and cancel on disappear; or cancel the `URLSessionDataTask`.
- Ignore responses from cancelled/out-of-order requests (a monotonic request id distinguishes the latest).
- Do not let a slow request update a view that has moved on.

### Configure Security Deliberately: ATS And Certificate Pinning

App Transport Security (ATS) requires HTTPS with modern TLS by default; an HTTP or weak-TLS endpoint is blocked unless you add an exception (and Apple may reject apps with broad exceptions). Configure ATS exceptions narrowly (specific domains, not a global allow). Certificate pinning (validating the server certificate against a known pin, via `URLSessionDelegate.urlSession(_:didReceive:completionHandler:)`) protects against compromised CAs but breaks when the server rotates its certificate — pin the SPKI (subject public key info) rather than the leaf, and have an update path. Treat security configuration as deliberate, not as the default that "just works."

- Keep ATS strict; add narrow exceptions only for endpoints you must use that do not yet support modern TLS.
- Pin the SPKI of a leaf or intermediate, with a backup pin; have a rotation plan.
- Test pinning against a changed certificate to confirm the failure mode is graceful, not a hard outage.

## Common Traps

### Task Never Resumed

`dataTask(with:) { ... }` without `task.resume()` never fires. Call `resume()`, or use the async API.

### No Timeout, Request Hangs Forever

Default 60s is often too long; a stalled server hangs the UI. Set `timeoutIntervalForRequest`.

### 4xx/5xx Treated As Success

`data(for:)` does not throw on a 404; check `HTTPURLResponse.statusCode`.

### Decode Error Flattened To Generic

A single `catch` loses whether it was transport, HTTP, or decode. Model typed errors and log `codingPath`.

### Retry Storm

Retrying every error with no backoff hammers a down server. Retry only transient/idempotent failures with exponential backoff.

### Non-Idempotent POST Retried, Double-Creating

Retrying a create POST duplicates the resource. Use an idempotency key or do not retry.

### Request Outliving The View

A request continues after the view disappears, updating stale UI or racing. Cancel on disappear; ignore out-of-order responses.

### ATS Or Pinning Breaking Silently

An HTTP endpoint blocked by ATS, or a rotated pinned certificate, causes hard failures. Configure narrowly and test rotation.

## Self-Check

- [ ] Tasks are resumed (callback API) or use the async `URLSession.data(for:)` API; timeouts are configured per call/session, and no request can hang indefinitely.
- [ ] New code uses async/await URLSession with structured concurrency; callback APIs are bridged with continuations resumed exactly once and cancelled cooperatively.
- [ ] Failures are modeled as typed errors distinguishing transport (`URLError`), HTTP (status code + body), and decode (`DecodingError` with `codingPath`), and HTTP status is always checked.
- [ ] `Codable` decoding is configured for the server's conventions (date strategy, key strategy), tolerant where the server is inconsistent and strict where the contract matters.
- [ ] Retries apply only to transient, idempotent failures with exponential backoff and jitter, a bounded attempt count, and cancellation respect; non-idempotent operations use idempotency keys or are not retried.
- [ ] Request lifetime is tied to the view (Task cancelled on disappear, or task cancelled), and out-of-order/late responses are ignored via a request id or state check.
- [ ] Security is configured deliberately: ATS kept strict with narrow exceptions, certificate pinning on SPKI with a backup pin and rotation plan, tested against certificate change.
- [ ] The networking design has been considered under slow/failing servers, retries, cancellation, decode mismatch, and security policy, and remains correct, bounded, and user-safe.
