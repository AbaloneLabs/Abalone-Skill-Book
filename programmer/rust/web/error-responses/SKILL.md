---
name: web-error-responses
description: Rust web server error-to-HTTP-response mapping rules. Use when converting application errors into HTTP responses, designing status code mapping, or ensuring consistent API error formats in axum/actix-web.
---

# Web Error Responses

Mapping internal errors to HTTP responses consistently is what makes an API usable. Keep these in mind.

## Every Error Becomes a Status Code

| Situation | Status |
|-----------|--------|
| Resource not found | 404 |
| Validation failed | 400 / 422 |
| Unauthorized | 401 |
| Forbidden (authenticated but no permission) | 403 |
| Conflict / duplicate | 409 |
| Internal failure | 500 |

Common mistake: returning 500 for a 404 or 400. The client can't distinguish "you asked wrong" from "server is broken".

## Consistent Error Body

Return a structured JSON error, not a bare string:
```rust
#[derive(Serialize)]
struct ApiError {
    error: String,
    code: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}
```
Inconsistent error shapes force clients to guess.

## Common Traps

### Leaking internal details in 500s
Don't expose stack traces, SQL errors, or internal paths to clients. Log the full error server-side; return a generic message to the client.

### Using `unwrap()` in handlers
A panic in a handler may crash the worker or return an opaque 500. Convert all fallible operations to proper error types.

### Forgetting to log before mapping to response
Once mapped to a response, the original error is often lost. Log the full error with context before converting it to an HTTP response.

### Mixing `?` without IntoResponse
In axum, handler error types must implement `IntoResponse`. A bare `?` with a non-converting type won't compile - implement the trait or map the error.

## Self-Check

- [ ] Does each error case map to the correct HTTP status (not blanket 500)?
- [ ] Is the error response body a consistent JSON structure?
- [ ] Are internal details (SQL, paths, traces) hidden from clients?
- [ ] Is the full error logged server-side before being converted to a response?
- [ ] Do handler error types implement the framework's response trait?
