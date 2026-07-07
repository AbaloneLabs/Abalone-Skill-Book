---
name: go_http_servers_middleware_and_context.md
description: Use when the agent is writing or reviewing Go HTTP servers and clients with net/http, http.Handler and http.HandlerFunc, middleware chains, chi/gin/echo router choice, request context and cancellation, timeouts (ReadTimeout, WriteTimeout, Server.TimeoutHandler), graceful shutdown, http.Client connection pooling and timeouts, reverse proxies, or diagnosing goroutine leaks from hijacked connections, missing timeouts, context propagation gaps, shared http.Client misuse, or slow-loris and header-smuggling vulnerabilities.
---

# Go HTTP Servers, Middleware, And Context

Go's `net/http` package is a production-grade HTTP server and client, but using it correctly requires understanding request lifecycle, context propagation, timeouts, and connection management. The server gives you `http.Handler` and a per-request `context.Context` that is cancelled when the client disconnects; middleware composes handlers in a chain; the client (`http.Client`) pools connections and must be reused, not recreated per request. The judgment problem is that HTTP is a protocol with many correctness and security edges — request timeouts, slow clients, header smuggling, connection leaks — and Go's defaults are safe but not complete: you must set server timeouts, propagate context, configure client connection pools, and handle graceful shutdown deliberately.

Agents tend to leave server timeouts at zero (the default, which lets a slow client hold a connection forever), to create a new `http.Client` per request (defeating connection pooling), to ignore the request context (so work continues after the client disconnects), or to write middleware that does not compose cleanly. The judgment problem is to set server and client timeouts deliberately, to propagate context through the request lifecycle, to reuse the client, and to structure middleware as composable, ordered layers. This skill is about treating HTTP server and client code as a lifecycle, timeout, and resource-management problem, not a handler-function convenience.

## Core Rules

### Set Server Timeouts; The Defaults Let Slow Clients Hold Connections Forever

`http.Server` defaults to zero timeouts, meaning a slow or malicious client can hold a connection indefinitely (the slowloris attack) and exhaust the server's file descriptors. Set at least `ReadHeaderTimeout` (always — it caps the time to read headers, defeating slowloris), and usually `ReadTimeout` and `WriteTimeout` for full request/response limits:

```
srv := &http.Server{
    Addr:              ":8080",
    Handler:           mux,
    ReadHeaderTimeout: 5 * time.Second,
    ReadTimeout:       15 * time.Second,
    WriteTimeout:      30 * time.Second,
    IdleTimeout:       60 * time.Second,
}
```

`ReadHeaderTimeout` is the most important: it caps the time to receive headers, which is where slowloris attacks. For endpoints with known long-running or streaming responses, use `Server.TimeoutHandler` (which wraps a handler with a deadline and returns 503 on timeout) or handle context deadlines in the handler. Do not ship a server with zero timeouts.

### Propagate The Request Context And Respect Cancellation

Each `*http.Request` carries a `context.Context` (`r.Context()`) that is cancelled when the client disconnects or the server shuts down. Propagate this context through your handler to all downstream work (database queries, upstream HTTP calls, goroutines doing request work): pass it to `http.NewRequestWithContext`, `db.QueryContext`, etc. When the context is cancelled, downstream work should abort, freeing resources for other requests.

The bug is ignoring the context: a handler that does `db.Query(...)` (no context) continues the query after the client has gone, wasting resources. A handler that launches a goroutine that outlives the request and uses request-scoped state leaks or races. Respect cancellation: check `ctx.Err()` in long loops, pass the context to all blocking calls, and detach request work into a background context only when the work must complete after the request (rare, and then copy any needed data out of the request).

### Reuse http.Client; Never Create One Per Request

`http.Client` pools connections (keep-alive) and must be reused across requests for performance and resource efficiency. Creating `&http.Client{}` per request defeats pooling (every request opens a new connection) and can exhaust file descriptors under load. Use a single shared client (package-level, or injected), configured with a `Transport` that sets timeouts and pool limits:

```
var client = &http.Client{
    Timeout: 10 * time.Second,
    Transport: &http.Transport{
        MaxIdleConns:        100,
        MaxIdleConnsPerHost: 10,
        IdleConnTimeout:     90 * time.Second,
    },
}
```

`http.DefaultClient` and `http.DefaultTransport` are shared and reasonable defaults, but they have no timeout — set `Timeout` on your own client. For testing, inject the client (or an interface) so tests can use `httptest.NewServer`.

### Structure Middleware As Composable, Ordered Layers

Middleware in Go is a function that takes a handler and returns a handler (`func(h http.Handler) http.Handler`), wrapping it to add behavior (logging, auth, CORS, recovery). Compose them in a chain; the order matters: recovery and logging usually go outermost (so they wrap everything), auth before the handler that needs it, CORS before auth if preflight must pass unauthenticated.

```
handler := chain(
    app,
    recoverMiddleware,
    loggingMiddleware,
    authMiddleware,
)
```

Keep middleware small and single-purpose; a god-middleware that does logging, auth, and CORS is hard to test and reuse. Use `http.HandlerFunc` to adapt functions. For complex routing, a router library (chi, gorilla/mux) provides typed middleware and route groups; evaluate whether the dependency is worth it — `net/http` (Go 1.22+) has method-based routing (`mux.HandleFunc("GET /items/{id}", ...)`) that suffices for many cases.

### Handle Graceful Shutdown To Drain In-Flight Requests

A hard server stop (`ListenAndServe` then kill) drops in-flight requests. Use `srv.Shutdown(ctx)` to drain: stop accepting new connections, let in-flight handlers complete (until the context deadline), then return. Wire this to OS signals:

```
go func() { srv.ListenAndServe() }()
sigCh := make(chan os.Signal, 1)
signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
<-sigCh
ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
defer cancel()
srv.Shutdown(ctx)
```

Shutdown respects request contexts: handlers that check their context will see cancellation as the deadline approaches. Set a shutdown deadline so a stuck handler does not block shutdown forever.

### Validate Input And Set Security Headers

HTTP handlers receive untrusted input. Validate path parameters, query strings, headers, and bodies against expected formats and lengths; reject early. Set security headers on responses (Content-Security-Policy, X-Content-Type-Options, Strict-Transport-Security) — usually via middleware. Be careful with header handling: Go's `Header.Set` normalizes, but custom header parsing must account for smuggling (reject headers with embedded CRLF). Do not trust `X-Forwarded-For` for auth unless you control the proxy chain.

### Use http.Flusher And Streaming Deliberately

For streaming responses (server-sent events, chunked progress), implement `http.Flusher` by calling `flusher, ok := w.(http.Flusher); flusher.Flush()` after each chunk. Be aware that streaming changes the timeout model: `WriteTimeout` counts from the start of writing, so a long stream hits it. For long-lived streams, set per-write deadlines or disable the write timeout for that handler and rely on context cancellation.

## Common Traps

### Zero Server Timeouts (Slowloris)

`http.Server{}` with no timeouts lets a slow client hold a connection forever. Always set `ReadHeaderTimeout` at minimum.

### New http.Client Per Request

Creating a client per request defeats connection pooling and can exhaust file descriptors. Share one client, configured with a transport and timeout.

### Ignoring The Request Context

`db.Query` (no context) continues after the client disconnects. Propagate `r.Context()` to all blocking downstream calls.

### Goroutine Outliving The Request

A handler that launches a goroutine referencing request data, without detaching to a background context and copying data, leaks or races. Detach deliberately or complete within the request.

### Middleware Order Wrong

Auth before CORS breaks preflight; logging inside recovery misses panics. Order recovery and logging outermost, then security, then feature middleware.

### No Graceful Shutdown

A hard stop drops in-flight requests. Use `srv.Shutdown(ctx)` wired to SIGINT/SIGTERM with a drain deadline.

### http.DefaultClient With No Timeout

`http.Get` / `http.DefaultClient` have no timeout; a slow server hangs the caller forever. Use a client with `Timeout` set.

### Trusting X-Forwarded-For For Auth

`X-Forwarded-For` is client-controllable unless you control the proxy chain. Do not use it for auth/identity without a trusted proxy.

## Self-Check

- [ ] The `http.Server` sets `ReadHeaderTimeout` (always), and `ReadTimeout`/`WriteTimeout`/`IdleTimeout` appropriate to the workload; no server ships with zero timeouts.
- [ ] The request context (`r.Context()`) is propagated to all downstream blocking calls (DB, upstream HTTP, long loops), so work aborts when the client disconnects.
- [ ] A single shared `http.Client` (with a configured `Transport` and `Timeout`) is reused across requests; no client is created per request, and `http.DefaultClient` is not used without a timeout.
- [ ] Middleware is composed as small, single-purpose, ordered layers (recovery and logging outermost), and the router choice (stdlib Go 1.22+ vs chi/echo) is justified by actual routing needs.
- [ ] Graceful shutdown (`srv.Shutdown` wired to SIGINT/SIGTERM with a drain deadline) drains in-flight requests without dropping them.
- [ ] Input (path, query, headers, body) is validated for format and length, and security headers are set via middleware; header smuggling (CRLF) is rejected.
- [ ] Streaming responses use `http.Flusher` deliberately, with the write-timeout model adjusted for long-lived streams.
- [ ] No goroutine launched in a handler references request-scoped state beyond the request lifetime without detaching to a background context and copying the needed data.
