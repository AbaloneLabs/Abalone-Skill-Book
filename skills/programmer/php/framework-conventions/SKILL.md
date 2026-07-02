---
name: framework_conventions.md
description: Use when the agent is building or refactoring Laravel or Symfony applications, using service containers and dependency injection, writing middleware, defining routes and controllers, modeling with Eloquent or Doctrine, configuring services and environment, or diagnosing scope pollution, hidden dependencies, and convention-violating behavior in PHP frameworks.
---

# Framework Conventions

Modern PHP frameworks (Laravel, Symfony) are built on conventions and inversion-of-control containers that make features feel magical, and that magic is the source of the hardest bugs. The service container resolves dependencies by type or name, facades resolve to application-wide singletons, middleware wraps every request, and ORM relationships lazy-load on access. Each of these is convenient in isolation and a source of hidden coupling in aggregate. The judgment problem is not "how do I register a service" but understanding what the framework is doing implicitly (auto-resolution, global state, query generation, request lifecycle) so that the explicit code you write composes correctly with it.

The recurring failure mode is a developer who treats the framework as a black box: calls a facade from anywhere, relies on the container to auto-wire a service whose constructor depends on configuration that is not always present, uses Eloquent relationships as if they were local fields and triggers N+1 queries, or assumes middleware runs in a particular order without verifying. The result is code that works in the common path and fails in edge cases (console commands, queue workers, tests, sub-requests) where the framework's implicit setup differs. Framework conventions are not optional context; they are the runtime model your code executes in.

## Core Rules

### Understand the service container and dependency injection as the backbone

Both Laravel and Symfony route object creation through a service container that resolves dependencies. Rules:

- Inject dependencies via constructors (or method injection in controllers) rather than calling `new` or resolving from the container manually. Manual resolution hides dependencies and breaks testability.
- Bind interfaces to implementations explicitly when the concrete choice matters; rely on auto-wiring only for simple concrete services.
- Be aware of singleton vs. transient binding scope; a singleton shared across requests in long-lived workers (e.g., Swoole, RoadRunner) retains state and causes cross-request leakage.

The container is the source of truth for what your service depends on; if you bypass it, your dependencies become invisible.

### Treat facades and global helpers as application-wide state

Laravel facades (`Cache`, `DB`, `Log`) and global helpers resolve to services bound in the container, which are application-wide singletons in most web runtimes. Rules:

- Facades are convenient but hide the dependency from the constructor signature; prefer injecting the underlying service in library/service code, and reserve facades for quick application-level code.
- In long-lived runtimes (Swoole, RoadRunner, FrankenPHP), application singletons persist across requests; clear or reset request-scoped state explicitly.
- Be cautious with static state and singletons that hold request data; they leak between requests in persistent runtimes.

### Model the request lifecycle and middleware order explicitly

Middleware wraps the request before it reaches the controller and the response after. Order matters: authentication must run before authorization; session start before session use; CORS before the response is built. Rules:

- Know your framework's middleware pipeline order and verify it in tests, not just by reading config.
- Do not put business logic in middleware that depends on controller-specific state; middleware runs before the controller.
- Terminate middleware (Laravel) and kernel events (Symfony) run after the response is sent; do not rely on them for work the client needs.

### Choose Eloquent vs Doctrine by the project's modeling needs

- **Eloquent (Laravel)**: active record, conventions-driven, fast to start, convenient for CRUD and simple domains. Tends to couple persistence to domain model and to hide query cost behind relationship access.
- **Doctrine (Symfony)**: data mapper, explicit mappings, richer domain modeling, better for complex domains and large teams. More ceremony and configuration.

Both can produce N+1 queries if relationships are accessed lazily in a loop. Eager-load (`with` in Eloquent, fetch joins in Doctrine) wherever relationships are iterated.

### Keep controllers thin; move domain logic to services

Controllers should parse input, call a service, and format output. Business logic in controllers is hard to test, hard to reuse across entry points (web, API, console, queue), and mixes HTTP concerns with domain rules. Rules:

- Controllers delegate to service classes that hold the domain logic.
- Services are container-resolved and depend on repositories, gateways, and other services, not on the request.
- Console commands and queue jobs call the same services, ensuring consistent behavior across entry points.

### Configure via environment and config files, not constants and globals

Frameworks load configuration from files that read environment variables. Rules:

- Read configuration via the framework's config layer (`config('...')` in Laravel, parameters in Symfony), not from `getenv` directly or from constants.
- Never commit secrets; use environment variables and secrets management.
- Cache configuration in production (`config:cache`, container compilation) for performance, and remember that cached config means changes require a cache clear.

### Validate and authorize at the boundary

Use framework validation (Form Requests in Laravel, Validator in Symfony) at the request boundary, and authorization policies/guards before acting. Rules:

- Validate input shape and constraints before it reaches the service layer.
- Authorize the action (not just authentication) using policies/voters; do not scatter permission checks in controllers.
- Keep validation rules in one place (form request / validator class) so they are reusable and testable.

### Handle the ORM's identity map and unit of work

Doctrine keeps a managed entity's identity in the unit of work; modifying it persists on flush without an explicit save call. This is powerful and surprising. Rules:

- Understand that Doctrine tracks managed entities; changes flush automatically.
- Detach or clear when you need to avoid accidental persistence.
- Eloquent models persist on explicit `save()`; do not assume Doctrine-style auto-persistence across frameworks.

## Common Traps

### Calling facades or `app()` from service constructors

Resolving from the container in a constructor hides the dependency and breaks when the service is used outside the framework's bootstrapped context (e.g., in a standalone script). Inject dependencies via the constructor.

### N+1 via lazy relationship access

Iterating `$order->items` in a loop queries per order. Eager-load (`with('items')` / fetch joins) wherever relationships are iterated, and use N+1 detection in tests.

### Singletons leaking state in persistent runtimes

In Swoole/RoadRunner, application singletons persist across requests. A service holding request-scoped data leaks between users. Reset state explicitly or avoid singletons for request data.

### Business logic in controllers

Controllers with domain logic cannot be reused by console commands, queue jobs, or other controllers. Move logic to services called from every entry point.

### Assuming middleware order without verifying

Authentication, session, and CORS middleware must run in a specific order. Verify the pipeline order in tests; misordered middleware causes auth failures or missing sessions.

### Configuration via `getenv` or constants

Direct `getenv` calls bypass the config cache and break in tests. Use the framework's config layer so configuration is cached, testable, and consistent.

### Mixing Doctrine auto-persistence with Eloquent habits

Assuming explicit `save()` in Doctrine means changes are lost (Doctrine flushes managed entities); assuming auto-persistence in Eloquent means changes are lost (Eloquent requires `save()`). Know your ORM's model.

## Self-Check

- Are dependencies injected via constructors rather than resolved manually via facades or `app()`, making them visible and testable?
- Are facades/global helpers reserved for application-level code, with services depending on injected dependencies, and is request-scoped state reset in persistent runtimes?
- Is the middleware pipeline order known and verified in tests, with business logic kept out of middleware that depends on controller state?
- Are relationships that are iterated eager-loaded, with N+1 detection enabled in the test suite?
- Are controllers thin, delegating to services reused across web, API, console, and queue entry points?
- Is configuration read via the framework config layer (not `getenv`/constants), with secrets in environment and config cached in production?
- Are validation and authorization applied at the request boundary using form requests/validators and policies/voters, rather than scattered in controllers?
- For Doctrine, is the identity map/unit-of-work behavior understood, and are entities detached/cleared when accidental persistence must be avoided?
