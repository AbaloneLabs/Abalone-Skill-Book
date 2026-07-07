---
name: php_psr_standards_and_http.md
description: Use when the agent is writing PHP HTTP-handling code (receiving requests, sending responses, middleware, PSR-7 ServerRequest/Response, PSR-15 middleware handlers, PSR-17 factories), implementing or consuming a PSR (PSR-3 logger, PSR-11 container, PSR-4 autoload, PSR-12 coding style), designing middleware pipelines, handling headers and status codes and content negotiation, working with $_SERVER/$_GET/$_POST vs PSR-7, sending cookies and redirects, streaming large response bodies, or is diagnosing "headers already sent", "cannot modify header information", "PSR-7 responses are immutable", "middleware order is wrong", or PSR conformance issues. Covers PSR-7/15/17 HTTP messages and middleware immutability, the headers-already-sent trap, $_SERVER vs PSR-7, status/header/cookie correctness, streaming, and PSR coding/container/logger standards.
---

# PSR Standards And HTTP In PHP

PHP's HTTP story has two faces: the raw superglobals (`$_GET`, `$_POST`, `$_SERVER`) and the imperative `header()`/`echo` output model, and the modern PSR-7/15/17 standard (immutable request/response value objects, middleware handlers, factory interfaces). Agents frequently mix the two incorrectly, misunderstand PSR-7 immutability (the message objects are immutable; `withHeader()` returns a *new* object, and discarding the return value silently does nothing), trip over the "headers already sent" error (any output before `header()` — a blank line, a BOM, an echo — makes header emission impossible), and order middleware wrongly (middleware wraps the handler like an onion; the order of wrapping determines whether a middleware runs before or after the inner handler). The judgment problem is to use PSR-7/15/17 correctly with immutability, to understand the raw PHP output model and its "headers already sent" constraint, to design middleware pipelines in the right order, and to apply the relevant PSRs (3 logger, 11 container, 4 autoload, 12 style) consistently.

Agents call `withHeader()` without capturing the return value (nothing changes), emit output before `header()` and hit "headers already sent", reverse middleware order, or reinvent PSR-7 by hand. The remedy is to treat messages as immutable, keep all output after header emission, and order middleware as an onion.

## Core Rules

### Treat PSR-7 Messages As Immutable; Capture with* Return Values

PSR-7 request and response objects are immutable: every `with*` method (`withHeader`, `withAttribute`, `withStatus`, `withBody`) returns a *new* instance; the original is unchanged. The single most common bug is calling `$request->withAttribute('user', $user)` and not assigning the result — the attribute is set on a discarded copy, and downstream code sees no attribute. Always capture: `$request = $request->withAttribute('user', $user);`, and pass the modified object to the handler. The same applies to responses: `$response = $response->withHeader('X', 'y');`. This immutability makes messages safe to pass around (no accidental shared mutation), but demands disciplined return-value capture.

- `with*` returns a new instance; the original is unchanged. Always assign the result and pass it on.
- Never call `with*` for its side effect — there is none; discarding the return value silently does nothing.
- Immutability enables safe sharing; trade-off is verbose `$r = $r->with*` chains.

### Understand The "Headers Already Sent" Constraint In Raw PHP

PHP sends headers before body output; once *any* byte of body output is emitted (a blank line before `<?php`, a UTF-8 BOM, an `echo`, whitespace, a warning emitted to output), headers cannot be set and `header()`/`setcookie()`/`session_start()` fail with "Cannot modify header information — headers already sent". Prevent this by: ensuring no output before header-setting code (no closing `?>` that leaks whitespace, no BOM, error reporting to log not screen during development), using output buffering (`ob_start`) where unavoidable, and structuring code to compute the full response (status, headers, body) before emitting anything. The PSR-7 model sidesteps this by building a response object and emitting it once at the end.

- Any output before `header()` makes header emission fail. Eliminate stray output (BOM, blank lines, closing `?>`, on-screen warnings).
- Compute the full response (status/headers/body) before emitting; emit once at the end.
- Use output buffering or, better, the PSR-7 build-then-emit model to avoid the trap entirely.

### Order Middleware As An Onion (Before/After Semantics)

PSR-15 middleware wraps the handler like layers of an onion: each middleware can act *before* calling `$handler->handle($request)` (the inbound side) and *after* receiving the response (the outbound side). The order in which middleware is added to the pipeline determines execution order: the first-added middleware is the outermost (runs first on the way in, last on the way out). So authentication middleware added before routing runs for every request (good); added after, only for matched routes. Logging/error-handling should be outermost (catches everything); routing in the middle; body-parsing/auth scoped appropriately. Reason about the onion: "what must run before the handler, what after, and in what order" — and add middleware in that order.

- Middleware is an onion: first-added is outermost (inbound first, outbound last).
- Put cross-cutting concerns (error handling, logging) outermost; routing in the middle; scoped concerns (auth for specific routes) innermost.
- Each middleware: do inbound work, call the handler, do outbound work on the response, return it.

### Use PSR-17 Factories, Do Not Construct Messages By Hand

PSR-7 messages should be created via PSR-17 factory interfaces (`ServerRequestFactoryInterface`, `ResponseFactoryInterface`, etc.), not `new Response()`, to stay decoupled from a specific PSR-7 implementation (Nyholm/PSR-17, Guzzle, etc.). Inject the factories (or a `ResponseFactoryInterface`), letting the framework/container supply the implementation. This keeps code portable across PSR-7 implementations and testable (mock the factory). Hardcoding `new \GuzzleHttp\Psr7\Response()` couples you to that library.

- Create messages via PSR-17 factory interfaces (`ResponseFactoryInterface`, etc.), injected by the container.
- Avoid `new SomeImplementationResponse()`; it couples code to one library.
- Factories make code portable across PSR-7 implementations and testable.

### Read Input From $_SERVER/$_GET/$_POST Or PSR-7 Deliberately

In raw PHP, request data lives in superglobals (`$_GET`, `$_POST`, `$_SERVER`, `$_COOKIE`); in PSR-7, it is accessed via the `ServerRequestInterface` methods (`getQueryParams`, `getParsedBody`, `getServerParams`, `getCookieParams`, `getUploadedFiles`). Mixing the two in one codebase is confusing and untestable. In a PSR-7/15 app, read everything from the request object (so tests can construct a fake request); reserve superglobals for non-PSR-7 legacy code. Note `getParsedBody` returns the parsed body (POST data for form posts, or null/decoded JSON if middleware parsed it) — JSON bodies are *not* auto-populated in `$_POST`; they must be read from `php://input` (raw) or via a body parser.

- In PSR-7 apps, read all input from the request object (`getQueryParams`, `getParsedBody`, `getUploadedFiles`), not superglobals.
- JSON request bodies are not in `$_POST`; read `php://input` or use a body-parsing middleware.
- Keep one model per codebase (PSR-7 request object OR superglobals); do not mix.

### Apply The Relevant PSRs Consistently

Beyond HTTP messages, the PHP standards ecosystem (PSRs) covers cross-cutting concerns. Use them rather than reinventing: PSR-3 (logger interface — type-hint `Psr\Log\LoggerInterface`, inject a concrete logger), PSR-11 (container interface — type-hint `Psr\Container\ContainerInterface`, avoid the service-locator anti-pattern by injecting dependencies not the container), PSR-4 (autoload structure — namespace maps to directory), PSR-12 (coding style — the modern successor to PSR-2; use a formatter like PHP CS Fixer). Consistent PSR adoption makes code interoperable across libraries and predictable to other PHP developers.

- PSR-3: inject `LoggerInterface`; PSR-11: inject dependencies, not the container (avoid service-locator).
- PSR-4: namespace→directory autoload; PSR-12: coding style via a formatter.
- Adopt PSRs consistently for interoperability and predictability.

## Common Traps

### Calling with* Without Capturing The Return Value

`withHeader`/`withAttribute` returns a new instance; discarding it silently does nothing. Always assign and pass on.

### "Headers Already Sent" From Stray Output

A BOM, blank line, closing `?>`, or on-screen warning before `header()`. Eliminate stray output; build-then-emit.

### Reversed Middleware Order

Auth added after routing only protects matched routes; logging not outermost misses errors. Order as an onion: outermost cross-cutting, routing middle, scoped inner.

### new Response() Coupling To An Implementation

Hardcoding a PSR-7 implementation couples code. Use PSR-17 factories, injected.

### Reading JSON Body From $_POST

JSON bodies are not auto-populated in `$_POST`. Read `php://input` or use a body parser; in PSR-7, `getParsedBody` after parsing.

### Forgetting Status Codes And Content-Type

A response with a body but default 200 and no Content-Type misbehaves. Set status and Content-Type deliberately.

### Injecting The Container As A Service Locator

Pulling dependencies from the container inside a class hides coupling and breaks testing. Inject dependencies directly.

## Self-Check

- [ ] All `with*` calls on PSR-7 messages capture and pass on the return value; no `with*` is called for a (nonexistent) side effect.
- [ ] No output (BOM, blank lines, closing `?>`, on-screen warnings) precedes `header()`/`setcookie()`/`session_start()`; responses are built then emitted once.
- [ ] Middleware is ordered as an onion (cross-cutting concerns outermost, routing middle, scoped concerns innermost), with inbound/outbound semantics understood.
- [ ] PSR-7 messages are created via injected PSR-17 factories, not `new ImplementationResponse()`.
- [ ] Request input is read consistently from one model (PSR-7 request object in PSR-7 apps, not mixed with superglobals); JSON bodies are read from `php://input` or a body parser, not `$_POST`.
- [ ] Status code, Content-Type, and relevant headers (cookies, redirects) are set deliberately on responses, not left at defaults.
- [ ] Cross-cutting concerns use the relevant PSR (3 logger, 11 container-as-injector-not-locator, 4 autoload, 12 style), applied consistently.
- [ ] Large response bodies are streamed (PSR-7 stream) rather than materialized, and the immutability/headers/order pitfalls have been considered.
