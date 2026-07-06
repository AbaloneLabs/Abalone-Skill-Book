---
name: api_versioning_and_evolution.md
description: Use when the agent is adding, removing, renaming, or changing the type of API fields or endpoints; deciding whether a change is backward or forward compatible; choosing a versioning strategy (URL path, header, media type, query parameter); planning a deprecation or migration window; designing sunset and breaking-change policies; supporting multiple client versions such as mobile apps, third-party integrations, SDKs, or partner APIs; or reviewing whether a proposed change will break deployed clients. Also covers semantic versioning versus API versioning, expand/contract (parallel change) workflows, compatibility definitions, and the operational risks of forcing clients to upgrade.
---

# API Versioning And Evolution

Every API change is a compatibility decision, even when no version number moves. The hard part of API evolution is not picking a versioning scheme; it is deciding, for each change, whether deployed clients will keep working, for how long, and what happens to the ones that do not. An API is a contract with clients the server author does not control — mobile apps already shipped, third-party integrations already written, SDKs already published, scripts already bookmarked. Once a field or behavior is public, removing or redefining it is a coordinated operation, not a refactor.

Agents tend to under-invest here because adding a feature "just works" in the local client and the current test suite. The harm is delayed and external: a mobile app that cannot upgrade crashes on a new response shape, a partner integration breaks at 3am, a removed field silently turns analytics pipelines to nulls, a renamed enum makes a checkout flow fail for a subset of users who cannot update. By the time the breakage is visible, the change is already deployed. The judgment problem is classifying each change by its compatibility impact, choosing a migration path that matches the client population, and resisting the shortcut of "we'll just ship v2."

This skill is about evolution and compatibility. It complements — and does not replace — the response-boundary skill, which covers which fields are safe to expose in the first place. Here the assumption is that fields are already public and the question is how to change them safely.

## Core Rules

### Classify Every Change By Its Compatibility Impact

Before writing any code, classify the change. Compatibility is not a feeling; it is a precise property with two directions:

- **Backward compatible (old clients keep working against the new server).** The server still understands old requests and still produces responses old clients can consume. This is the default you should aim for.
- **Forward compatible (new clients keep working against an old server).** The new client tolerates an older server that does not yet know the new fields or endpoints. This matters when clients and servers deploy independently.

Map common changes to these categories:

- **Adding an optional request field or parameter** — backward compatible (old clients send the old shape). Usually also forward compatible if the server treats unknown fields gracefully.
- **Adding a response field** — backward compatible for tolerant clients, but only safe if old clients ignore unknown fields. It is a breaking change for clients that use strict schema validation or fail on unknown keys.
- **Removing or renaming a field** — breaking in both directions. Old clients send/read the old name; the new server no longer understands it.
- **Changing a field type or semantics** — breaking, even if the name stays the same. Changing `price` from a number to a string-money object, or changing `status: "active"` to mean something subtly different, breaks clients that compiled against the old meaning.
- **Tightening validation (rejecting inputs that were previously accepted)** — breaking. A field that used to accept any string and now rejects emoji is a breaking change for clients that send emoji.
- **Loosening validation or adding enum values** — usually backward compatible for tolerant clients, but a new enum value can break clients that exhaustively switch over the old set.
- **Changing error semantics or status codes** — breaking. Clients branch on status codes and error shapes; silently changing them changes behavior.
- **Changing ordering, defaults, pagination semantics, or side effects** — often breaking even when the schema is untouched.

Write the classification down. A change that "feels small" but tightens validation or shifts semantics is breaking and must be treated as one.

### Separate Semantic Versioning From API Versioning

Package semantic versioning (MAJOR.MINOR.PATCH) describes the evolution of one distributable artifact. API versioning describes the contract between a server and a population of independent clients. They are not the same, and applying semver rules mechanically to an API produces bad decisions.

- A library bumping its major version forces recompilation of callers the author controls. An API breaking change forces migration of callers the author does not control.
- Semver's "breaking change bumps major" assumes consumers can upgrade. API consumers (mobile apps, partners) often cannot upgrade on your schedule.
- An API can add a breaking change without any package version moving, if it tightens validation or changes defaults.

Use API versioning and compatibility reasoning for API decisions. Use semver for your SDK and server packages. Do not let a "we're still 0.x" or "it's only a minor bump" framing weaken API compatibility discipline.

### Choose A Versioning Strategy That Matches Your Client Population

There is no universally correct versioning scheme; each optimizes for a different client shape. Choose deliberately and apply it consistently.

- **URL path versioning (`/v1/...`, `/v2/...`).** Explicit, easy to route, easy to operate (run both versions side by side), easy to deprecate by observing traffic. Cost: URL is part of the contract, so versioning the URL is itself a visible, sometimes awkward, decision. Strong default for public HTTP APIs.
- **Header or media-type versioning (`Accept: application/vnd.example.v2+json`).** Keeps URLs stable, good for content-negotiation-driven APIs. Cost: invisible in casual testing, harder to route and observe, easier to forget. Good when content negotiation is already a first-class concept.
- **Query parameter versioning (`?api-version=2`).** Simple, but mixes version with request data and tends to interact badly with caching and logging. Acceptable for internal or platform APIs where the convention is strong.
- **No explicit versioning, additive-only evolution.** Viable only when the client population is fully under your control and can be upgraded atomically. Dangerous for public or mobile APIs.

The strategy matters less than consistency and observability. Whatever you choose, make it possible to see which versions are in use, route different versions independently, and deprecate a version by watching its traffic fall.

### Prefer Expand-Contract (Parallel Change) Over Big-Bang Rewrites

When a change is breaking, the safe path is usually expand-contract, also called parallel change:

1. **Expand.** Make the server accept both the old and new shape. Add the new field/endpoint/behavior alongside the old. Old clients keep working; new clients can adopt the new shape.
2. **Migrate.** Move internal callers and documented clients to the new shape. Observe traffic to the old shape falling.
3. **Contract.** Only after the old shape has no real traffic, remove it.

This costs more code for a while, but it removes the coordination problem: no client is forced to upgrade at the same instant as the server. Big-bang rewrites require every client to change in lockstep, which is impossible for mobile apps and third parties and fragile even internally.

Reserve big-bang (ship v2, kill v1 immediately) for cases where v1 is fundamentally unsafe or unmaintainable and the client population is small and controllable.

### Define What "Breaking" Means For Your Specific Clients

Compatibility is defined by what real clients do, not by an abstract spec. The same schema change can be safe or breaking depending on client behavior.

- **Strict-schema clients** (generated from OpenAPI/Protobuf, with `additionalProperties: false` or unknown-field rejection) break when you add fields. Adding a response field is breaking for them.
- **Tolerant readers** (parse what they know, ignore the rest) survive additive changes but break on removals and type changes.
- **Exhaustive switchers** (code that handles every enum value and fails on new ones) break when you add an enum value.
- **Clients that depend on undocumented behavior** (ordering, timing, default values, error text) break when you change that behavior, even if the documented contract is untouched.

Before classifying a change, ask which client types exist in your population. A change that is "non-breaking" by the spec may be breaking in practice for generated clients.

### Run A Deprecation Process, Not A Removal

Removing a public field, endpoint, or behavior is a process, not a single change. A healthy deprecation includes:

- **Announce.** Document the deprecation, the replacement, and the timeline where clients can find it (changelog, response headers like `Deprecation`/`Sunset`, SDK warnings, direct partner outreach).
- **Observe.** Measure real traffic to the deprecated surface. Do not schedule removal by calendar alone; schedule it by observed adoption of the replacement.
- **Warn, then enforce.** Emit deprecation warnings (headers, logs, metrics) before returning errors. Give clients a signal in their own telemetry.
- **Remove only when traffic justifies it.** For mobile and third-party clients, "low traffic" may still be real users who cannot upgrade. Decide explicitly how long to carry them.

Skipping the process and removing on a date breaks the clients that could not move. Skipping the process and never removing leaves the API carrying dead surfaces forever. Both are failures of evolution discipline.

### Account For Client Diversity And Upgrade Asymmetry

Different client populations upgrade at very different rates, and this changes the right evolution strategy.

- **Server-to-server clients** can often be upgraded in a coordinated window, but third-party partners upgrade on their own schedule and may be slow.
- **Mobile apps** upgrade over weeks to months, and a fraction of users never upgrade. A breaking change can require carrying old behavior for a long time, or accepting that some users must be forced to update.
- **Web clients** can be upgraded quickly but may be cached, embedded in third-party pages, or used by browser extensions that pin old behavior.
- **SDKs** ship versioned, and consumers pin versions. A breaking change in the API must be matched by a breaking change in the SDK, which consumers adopt slowly.
- **Exported data, webhooks, and analytics schemas** are consumed by pipelines and warehouses that may depend on field shapes for years.

For each change, name the slowest-upgrading client population and confirm the migration window is realistic for them. If the slowest client is a mobile app that cannot force updates, the safe answer is often additive change only.

### Make Versioning Observable

You cannot safely evolve an API you cannot observe. Before deprecating or removing anything, ensure you can answer:

- What versions and fields are currently in use, by traffic volume?
- Which clients (by SDK version, app version, partner identity, user-agent) still call the deprecated surface?
- When a client fails on a new shape, can you attribute it to a version mismatch?

Route version into logs, metrics, and traces. Without this, deprecation becomes guesswork and removal becomes a gamble.

### Know When A Breaking Change Is Justified

Breaking changes are not forbidden; they are expensive and should be made deliberately. Justified reasons include:

- The old contract is a security or correctness hazard that cannot be fixed additively.
- The old shape makes a critical feature impossible and additive workarounds are worse than the break.
- Maintenance of the old surface is unsustainable and the client population is small enough to migrate.

When a breaking change is justified, make the cost visible: a version bump, a migration guide, a sunset timeline, and explicit owner accountability. Do not smuggle a breaking change through as a "minor" update.

## Common Traps

### Treating "Adding A Field" As Always Safe

Adding a response field is backward compatible only for tolerant clients. Generated clients with strict schemas, or clients that fail on unknown keys, break on additive changes. If your population includes strict-schema clients, an addition is a coordinated change, not a free one.

### Renaming Via Removal And Re-Addition Without A Window

Removing `firstName` and adding `givenName` in the same deploy, expecting clients to switch instantly. Every client reading the old name breaks immediately. The safe path is to serve both during a migration window and remove the old name only after traffic moves.

### Changing Semantics While Keeping The Name

Keeping the field `price` but changing it from a float to a string, or from "price including tax" to "price excluding tax." Clients compiled against the old meaning now misbehave silently. A semantic change is a breaking change even when the field name is unchanged; treat it as a removal plus an addition, or version the field.

### Tightening Validation And Calling It A Bugfix

Rejecting inputs that used to be accepted (stricter regex, shorter max length, new required field) breaks clients that were sending the previously-valid input. Even if the old behavior was a bug, tightening validation is a breaking change for deployed clients and needs a migration path, not a silent fix.

### Assuming A New Enum Value Is Non-Breaking

Adding `status: "archived"` to a previously `{active, inactive}` enum breaks clients that exhaustively switch over the values and throw on the default case. Enum additions are safe only for clients that treat unknown values as a distinct, handled case.

### Versioning Only The URL And Ignoring Behavior

Bumping `/v1` to `/v2` but keeping the same fragile validation, ordering, and error semantics. The version number communicates nothing if the behavior inside is unstable. Version the behavior contract, not just the path.

### Killing An Old Version By Calendar Without Measuring Traffic

Setting a hard sunset date for v1 and removing it on that date regardless of remaining traffic. Long-tail mobile users and pinned SDK consumers keep calling it and break. Removal should be gated on observed traffic, not only on a date.

### Forgetting Webhooks, Exports, And Event Schemas and big-Bang v2 Because v1 "Got Messy"

Evolving the request/response API carefully but changing webhook payloads, exported reports, or event schemas without a compatibility pass. These are APIs too, consumed by pipelines and integrations that depend on their shapes for years.

Rewriting the whole API as v2 and telling clients to migrate, when the mess could have been fixed through additive evolution. v2 doubles the maintenance surface (you now carry two APIs) and forces every client to coordinate. Reserve full rewrites for cases where incremental evolution is genuinely impossible.

## Self-Check

- [ ] Each change was explicitly classified as backward compatible, forward compatible, or breaking, and the classification accounts for strict-schema, tolerant, and exhaustive-switching clients in the population.
- [ ] Breaking changes (removals, renames, type changes, semantic changes, validation tightening, new enum values) are treated as breaking, not smuggled through as minor updates.
- [ ] The versioning strategy (URL, header, media type, query) was chosen deliberately to match the client population and is applied consistently, not mixed ad hoc.
- [ ] Breaking changes follow an expand-contract path (serve old and new together, migrate, then remove) rather than a big-bang rewrite, unless the client population is fully controllable.
- [ ] Deprecations include announcement, observable traffic measurement, warning signals, and removal gated on real adoption — not removal by calendar alone.
- [ ] The slowest-upgrading client population (mobile apps, third-party partners, pinned SDKs, analytics pipelines) was named and the migration window confirmed realistic for them.
- [ ] Semantic changes were not hidden behind unchanged field names; the field was versioned, removed-and-re-added, or accompanied by a documented behavior change.
- [ ] Webhooks, exports, event schemas, and analytics shapes were included in the compatibility analysis, not only the request/response API.
- [ ] Version usage is observable in logs, metrics, and traces so deprecation and removal decisions are based on real traffic.
- [ ] Any justified breaking change carries a visible cost: version bump, migration guide, sunset timeline, and owner accountability.
