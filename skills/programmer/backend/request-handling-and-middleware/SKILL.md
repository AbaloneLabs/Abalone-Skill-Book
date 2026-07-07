---
name: request_handling_and_middleware.md
description: Use when the agent is designing or reviewing an HTTP/gRPC request handling layer, writing middleware (auth, logging, tracing, rate limiting, CORS, compression), composing middleware chains, mapping handlers to routes, deciding request-scoped vs shared state, handling timeouts and context propagation, streaming request/response bodies, or diagnosing middleware ordering bugs, header mutation hazards, leaked request state, missing timeouts, or context-not-propagated failures. Covers middleware composition order, per-request isolation, idempotency, body handling, and the hazards of shared mutable state and blocking in handlers.
---

# Request Handling And Middleware

A request handling layer is the boundary where untrusted, arbitrary input from the network meets your application logic, and where your application's behavior is composed from a stack of cross-cutting middleware. Both halves are easy to get superficially right and subtly wrong. A handler that works for the happy path hangs forever on a slow client because nobody set a timeout; a middleware that logs the request body breaks streaming uploads because it buffered the whole body into memory; an auth middleware placed after the logging middleware logs requests as if they succeeded when they were rejected; a shared variable mutated by concurrent handlers corrupts state under load. The request layer is where most production incidents in a web service originate, because it is where concurrency, untrusted input, timeouts, and cross-cutting concerns all intersect.

The judgment problem is not "how do I register a route" but "what is the lifecycle of a single request, what middleware runs in what order, what state is shared versus isolated, and what happens under slow clients, large bodies, partial reads, cancellation, and concurrent load." Agents tend to write handlers as if each request were the only one (ignoring shared-state races), to compose middleware without reasoning about order (auth before logging or after?), to forget that request bodies are streams (buffering them eagerly), and to omit timeouts because the local test responds instantly. Each of these compiles, passes a unit test, and fails in production.

## Core Rules

### Treat Every Request As Concurrent, Untrusted, And Potentially Malicious

A request handler runs concurrently with every other request the server is processing, and its input is arbitrary bytes from an untrusted source. Two implications dominate. First, shared state: any variable reachable from the handler that is not request-scoped is shared across all concurrent handlers, and any mutation of it is a race unless synchronized. The classic bug is a handler that caches a parsed result in a package-level map without a lock, or stashes request state in a global registry that one request mutates while another reads. Second, input validation: every field, header, parameter, and body must be validated and bounded before use — never trust content-length, never trust content-type, never assume a JSON body parses, never assume a field is present or in range. Treat the request as hostile until proven otherwise, because on the public internet it will be.

Design handlers to be pure functions of their input plus explicitly injected dependencies, with all mutable state either request-local or accessed through a synchronized/owned abstraction. If you find a handler reading or writing a global, that is almost certainly a concurrency bug waiting for the right load.

### Order Middleware So Each Layer's Assumptions Hold

Middleware is a chain, and order is semantics. The rule is to reason about what each layer assumes and what it produces, and order so that assumptions are met. A common defensible order, from outermost to innermost: error recovery (catch panics, return 500) → request-id/trace propagation → logging/access-log → rate limiting → authentication → authorization → CORS → compression → the handler. The reasoning: recovery must be outermost so it catches everything; tracing must be early so every span has the trace context; rate limiting before auth if you want to limit unauthenticated traffic, after auth if you want per-user limits; auth before authz (you need identity to authorize); CORS before the handler but typically after auth for preflight handling; compression innermost so it wraps the final response.

The traps are concrete. Logging before auth logs the request as "200 OK" when auth will later reject it (the log lies). Auth after rate limiting means an attacker can exhaust a global rate limit with unauthenticated junk. Compression before auth compresses the 401 response, which is wasteful but not wrong — but compression of an error response can break clients that do not expect it. Write down the order and the reason for each layer; a middleware stack whose order is "whatever order they were added in" is an accident.

### Propagate Cancellation And Deadlines Through The Request

Every request must have a deadline, and that deadline must propagate to every downstream operation. A handler with no timeout hangs forever on a slow downstream; a handler that does not respect client disconnect keeps doing work for a client that has gone away. Use the platform's context/cancellation mechanism (Go `context.Context`, Java `CompletableFuture.orTimeout`, Node `AbortSignal`, Python `asyncio.CancelledError`) and thread it through every blocking call: database queries, downstream HTTP calls, cache reads, message publishes.

Set the deadline at the edge (the server's read/write timeout, or a middleware that wraps the handler in a timeout context), and make it shorter than the client's timeout so you return a controlled error rather than the client giving up first. Distinguish two failure modes: client disconnect (the client went away — stop work, the result is useless) and server timeout (the operation is slow — you may still want to complete it for side effects, or cancel it). Both must be handled; neither is optional in production.

### Handle Request Bodies As Streams, Not As Buffers

A request body is a stream of bytes, and the decision of when (and whether) to materialize it into memory is a resource and security decision. Eagerly reading the whole body (`io.ReadAll`, `req.text()`, `byte[] body = req.getInputStream().readAllBytes()`) is simple and correct for small, bounded bodies, and catastrophic for large or unbounded ones: a malicious or buggy client sending a 10GB body exhausts memory before your handler runs. The defenses are: enforce a maximum body size at the edge (a `MaxBytesReader` in Go, a body-size limit in the server config), stream large bodies to disk or to a downstream rather than buffering, and prefer streaming APIs (`io.Copy`, `pipe`, `Flow`) for large uploads/downloads.

The same applies to response bodies: do not buffer a large generated response in memory before sending; stream it with chunked encoding so memory stays bounded. Middleware that logs request or response bodies must be especially careful — a logging middleware that buffers the body to log it has just turned a streaming upload into a 10GB in-memory buffer. Log body metadata (size, content-type, hashes) rather than the full body, or sample/truncate.

### Make Middleware Composable, Side-Effect-Aware, And Transparent

Good middleware is composable: each layer wraps the next, adds its concern, and calls through. Each layer should be transparent about its side effects — it should document whether it reads the body, mutates headers, short-circuits the chain (returns without calling the next handler), or wraps the response writer. A middleware that short-circuits (rate limiter returning 429, auth returning 401) must do so deliberately and must not leave the chain in a half-applied state (e.g., a trace span opened but never closed).

Two composability rules. First, header mutation must happen before the response is committed — once bytes are flushed to the wire, header changes are lost, and middleware that sets headers after the handler has started streaming silently fails. Second, response-writer wrapping (to capture status code, body size, or to inject compression) must wrap before the handler runs, not after — a common bug is a logging middleware that wraps the writer only after the handler returns, missing everything. Test middleware in composition, not in isolation, because the interactions are where the bugs live.

### Distinguish Idempotent From Non-Idempotent Handling, And Design For Retries

Clients retry. Networks duplicate. Load balancers reissue. A request is not a single event; it is at-least-once delivery of an intent, and your handler must be correct under duplication. GET, PUT, and DELETE should be idempotent by design (a second identical request has the same effect as the first). POST and other non-idempotent methods must either be made idempotent with an idempotency key (the client sends a unique key; the server deduplicates within a window) or accept that duplicates cause duplicate side effects (a double charge, a duplicate record).

Design every mutating handler with the question: what happens if this runs twice? If the answer is "the side effect happens twice and that is wrong," you need an idempotency key, a unique constraint, or a deduplication layer. Do not rely on the client not retrying; clients retry, and so do intermediaries. This is the same discipline as in the concurrency and resilience skills, applied at the request boundary.

### Isolate Failures And Return Structured, Bounded Errors

A handler must never crash the server. Panics in a handler (a nil dereference, an out-of-bounds index, an unchecked cast) must be recovered at the boundary and turned into a 500 with a logged stack trace, not propagated to kill the process. Every handler should be wrapped in recovery middleware. Equally, a handler must not leak internal structure in its errors: a raw SQL error, a stack trace, an internal id, or a library message returned to the client leaks implementation detail and can leak secrets. Map domain errors to structured, bounded external errors at the boundary (see the error-and-status-codes skill), and log the full detail server-side.

The corollary is that errors must be actionable: a 500 with a generic "internal error" and no log entry is un-debuggable; a 500 with a correlated request id that matches a log entry with the stack trace is debuggable. Generate a request id at the edge, attach it to logs and traces and the response, so a user-reported error can be traced end to end.

### Size And Bound Everything: Timeouts, Bodies, Connections, Concurrency

A request handling layer is a resource manager, and every resource it touches must be bounded: timeout per request, maximum body size, maximum header size, maximum concurrent connections, maximum concurrent in-flight requests per worker, maximum concurrent downstream calls. Unbounded resources are how a server dies under load — a slowloris attack holds connections open, a large body exhausts memory, a flood of requests exhausts file descriptors or goroutines/threads. Set every bound deliberately, even if the default seems fine, and make the bounds observable (expose counters for rejected requests, timed-out requests, queued requests).

The discipline is to identify, for each resource, the bound and the back-pressure behavior when the bound is hit: reject with 429 (too many requests), 413 (payload too large), 408/504 (timeout), or queue with a bounded queue and a timeout. A server with no explicit bounds has implicit bounds (the OS limits), and hitting those is a crash rather than a controlled degradation.

## Common Traps

### Shared Mutable State In Handlers

A package-level map, cache, or counter mutated by concurrent handlers without synchronization, racing under load. Keep handler state request-scoped; synchronize any shared state explicitly.

### Wrong Middleware Order

Logging before auth (logs lie about success), rate limiting after auth (attacker exhausts global limit), compression before auth (wasteful). Write down the order and the reason per layer.

### No Per-Request Timeout

A handler with no deadline hangs on a slow downstream or a slow client, exhausting workers. Set a timeout at the edge shorter than the client's timeout, and propagate it.

### Buffering The Entire Request Body

`io.ReadAll(req.Body)` on an unbounded body exhausts memory under a large upload. Enforce a max body size at the edge; stream large bodies.

### Logging Middleware That Buffers The Body

A logging middleware that reads the whole request body to log it turns a streaming upload into an in-memory buffer. Log metadata (size, type, hash), not the full body; sample or truncate.

### Header Mutation After The Response Is Committed

Setting headers after the handler has flushed bytes to the wire silently fails. Wrap the response writer and set headers before the handler writes.

### Non-Idempotent Mutating Handler With No Deduplication

A POST that charges or inserts, with no idempotency key, double-charging or double-inserting on client retry. Use an idempotency key or unique constraint.

### Unbounded Concurrency Or Connections

No limit on in-flight requests, file descriptors, or worker threads, so a flood of requests crashes the server at OS limits. Set explicit bounds and back-pressure.

### Leaking Internal Errors To The Client

Returning raw SQL errors, stack traces, or internal ids in the response body. Map to structured external errors at the boundary; log the detail server-side with a request id.

## Self-Check

- [ ] Handlers are pure functions of request input plus injected dependencies; all mutable state is request-scoped or accessed through synchronized/owned abstractions, with no handler reading or writing a global without a documented concurrency story.
- [ ] All request input (headers, params, body, content-length, content-type) is validated and bounded before use; the request is treated as untrusted and potentially malicious.
- [ ] Middleware order is documented with a reason per layer (recovery outermost, tracing early, rate limit vs auth decided deliberately, auth before authz, compression innermost), and no layer's assumptions are violated by an earlier layer.
- [ ] Every request has a deadline set at the edge (shorter than the client timeout), propagated via the platform's context mechanism to every blocking downstream call, with client disconnect and server timeout both handled.
- [ ] Request and response bodies are streamed, not eagerly buffered; a maximum body size is enforced at the edge; logging/tracing middleware logs body metadata rather than the full body.
- [ ] Middleware is composable and transparent: response-writer wrapping and header mutation happen before the handler commits the response; short-circuiting layers (rate limit, auth) do not leave spans/locks half-applied.
- [ ] Mutating handlers are idempotent by design or protected by an idempotency key / unique constraint / deduplication layer, so client or intermediary retries do not cause duplicate side effects.
- [ ] Every handler is wrapped in recovery middleware that turns panics into 500s with logged stack traces; errors are mapped to structured, bounded external errors at the boundary with no internal detail leaked, and a request id correlates client-visible errors to server logs.
- [ ] Every resource (timeout, body size, header size, connections, in-flight requests, downstream concurrency) has an explicit bound and a defined back-pressure behavior (429/413/408/504 or bounded queue), and the bounds are observable via counters/metrics.
