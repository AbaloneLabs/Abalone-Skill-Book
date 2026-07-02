---
name: rails_conventions_and_magic.md
description: Use when the agent is building or refactoring a Ruby on Rails application, using callbacks, concerns, validations, associations, scopes, eager loading, counter caches, background jobs, or diagnosing N+1 queries, unexpected callback side effects, autoloading errors, and implicit behavior that is hard to trace in Rails apps.
---

# Rails Conventions and Magic

Rails sells productivity through "convention over configuration," and that convention is the source of both its speed and its hardest bugs. The framework infers a tremendous amount from names, file locations, and associations, and when the inference matches your intent everything feels effortless. When it does not, you get callbacks that fire in surprising orders, queries that multiply into N+1 storms, autoloading errors that appear only in specific environments, and logic spread across concerns and helpers in ways that make the call path nearly impossible to follow. The judgment problem is not "how do I use a callback" but when the convention is helping you and when it is hiding behavior that should be explicit.

The recurring failure mode is a developer who leans on Rails magic because it is fast, accumulates implicit behavior across callbacks, associations, and concerns, and then cannot diagnose why an object in an unexpected state behaves differently than expected. A `before_save` that triggers a mailer that triggers another save that triggers another callback; a `has_many :through` that silently loads records; a scope that looks like a query but mutates state. None of these are bugs in isolation; they are the compound interest of implicit behavior.

## Core Rules

### Treat callbacks as a last resort, not a default

`before_save`, `after_create`, and friends are powerful and seductive because they centralize behavior. They are also the most common source of surprising side effects, because the save path now does things the caller did not ask for and cannot easily opt out of. Rules:

- Use callbacks for invariants the model must always enforce (e.g., normalizing a field, setting a timestamp).
- Do not use callbacks for orchestration that only some callers need (sending mail, charging a card, enqueueing jobs). Move that to a service object the caller invokes explicitly.
- A callback that enqueues side effects (mailer, job) makes every save, including in tests and migrations, trigger those side effects. Prefer explicit service calls for anything with external effects.

If a callback's behavior is conditional on context, that is a strong signal it belongs in an explicit service.

### Make query cost explicit; hunt N+1 always

Rails' lazy association loading makes N+1 queries invisible in development and devastating in production. A view that iterates `post.comments.each` triggers one query per post. Rules:

- Use `includes`, `preload`, or `eager_load` for associations you will access in a loop. Prefer `preload` (separate queries) unless you need to filter on the association, in which case use `eager_load` (JOIN).
- Counter caches (`counter_cache: true`) avoid count queries; use them for counts that are read often.
- In tests, enable N+1 detection (e.g., `bullet`) so regressions surface in CI, not production.
- Beware of callbacks and serializers that touch associations; they re-introduce N+1 inside the model layer.

### Understand autoloading and naming before fighting it

Rails autoloads constants based on file path and class name (Zeitwerk in modern Rails). A constant `Foo::Bar` must live in `foo/bar.rb`. Violating this produces `NameError` in some environments (production eager-loads) but not others (development lazy-loads). Rules:

- Match file path to constant name exactly.
- Do not define constants of the same name in different namespaces expecting one to shadow the other.
- Do not reference autoloadable constants in `class` body initialization or in places that run at boot before autoload is ready (initializers, `Rakefile`); use `to_prepare` or lazy references.
- Test in production-like (eager-loaded) mode in CI, because autoload bugs surface only there.

### Keep concerns focused and compose sparingly

Concerns let you share behavior across models, but each concern adds implicit methods and associations to every class that includes it. A model with five concerns has a public surface and callback chain that no single file reveals. Rules:

- A concern should add one coherent capability, not a grab bag.
- Document what methods, associations, scopes, and callbacks a concern adds; the including class's behavior is the union.
- Avoid concerns that depend on methods defined by other concerns; that coupling is invisible and fragile.
- Prefer composition (service objects, value objects) over concerns when the shared behavior does not naturally belong on the model.

### Validate at the right layer

Rails validations (`validates`, `validate do`) enforce model-level invariants and are appropriate for data integrity (presence, uniqueness, format). They are the wrong tool for multi-step workflow rules (e.g., "a draft can only be published if reviewed") because they run on every save and mix persistence with workflow. Use form objects or service objects for workflow validation, and keep model validations to invariants that must always hold.

### Make background job boundaries explicit

ActiveJob makes enqueuing trivial, but the boundary between synchronous and asynchronous code is a design decision with real consequences. Rules:

- A job must be idempotent: re-running it (after a crash, retry, or duplicate enqueue) must not double-charge or double-send. Design for at-least-once delivery.
- Pass primitive arguments (IDs, not serialized ActiveRecord objects) because GlobalID serialization can resurrect stale or deleted records.
- Set `retry` and `dead` policies deliberately; the defaults retry for days, which may be wrong for time-sensitive work.
- Beware of jobs enqueued from callbacks; a save in a job retry can re-enqueue, creating loops.

### Treat the framework upgrade as a compatibility event

Rails upgrades change defaults, deprecate APIs, and alter security behavior. Read the upgrade notes for every minor, run `rails app:update`, and test against the new defaults before adopting them. A "seamless" upgrade that silently changes a default (e.g., CSRF handling, default order of callbacks) is a common source of production regressions.

## Common Traps

### Callbacks that trigger external side effects

A `after_create :send_welcome_email` makes every `create` send mail, including in tests, console, and seeds. External side effects belong in explicit service calls invoked by the controller or job, not in the persistence path.

### `default_scope` that silently filters results

`default_scope` applies to every query, including associations and joins, and is almost impossible to opt out of cleanly. It produces "why is this record missing" bugs. Prefer named scopes that callers apply explicitly.

### `has_many` and `belongs_to` that load on access

Accessing `user.posts` always queries; there is no lazy cache across reloads. Code that accesses the same association multiple times in a request triggers multiple queries. Eager-load at the query site, not at access time.

### Concerns that mutate the including class's callback chain

A concern with `included do; before_save :x; end` adds a callback to every includer, and the callback order depends on include order. This is invisible from the model file. Document or avoid.

### Relying on `update_columns` or `update_attribute` to skip callbacks

These skip validations and/or callbacks, which is sometimes intended but often a source of invalid data. If you must skip, name why in a comment, and prefer explicit methods whose names communicate intent.

### Time-zone and `Time.now` vs `Time.current` confusion

Rails sets a time zone that `Time.now` ignores; use `Time.current`/`Time.zone.now` for app-local time. Mixing them produces off-by-offset bugs in scheduling and comparisons.

### Treating the console as ground truth

The Rails console runs in development with lazy autoloading and no N+1 detection. Behavior that "works in console" may fail in production with eager loading and real load. Validate against production-like configuration.

## Self-Check

- Are callbacks limited to invariants the model must always enforce, with external side effects (mail, jobs, payments) moved to explicit service objects?
- Have you run N+1 detection in the test suite, and are associations that are accessed in loops eager-loaded via `includes`/`preload`?
- Do file paths match constant names exactly, and does the test suite run in eager-loaded (production-like) mode to catch autoload bugs?
- For each concern, is the added surface (methods, associations, scopes, callbacks) documented, and do concerns avoid cross-concern method dependencies?
- Are validations split between model invariants (always hold) and workflow rules (form/service objects)?
- Are background jobs idempotent, taking primitive IDs as arguments, with deliberate retry and dead-letter policies?
- Are there any `default_scope` definitions, and if so, have they been replaced with explicit named scopes?
- Is time handling consistent (`Time.current`/`Time.zone.now`), and have you validated behavior in production-like configuration rather than only the console?
