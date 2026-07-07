---
name: api_version_migration_and_deprecation.md
description: Use when the agent is migrating an API to a new version, choosing a versioning strategy (URL path, header, media type, query parameter), planning a deprecation and sunset timeline, classifying a change as breaking or non-breaking, keeping multiple API versions running in parallel, communicating breaking changes to consumers, adding deprecation or sunset HTTP headers, or deciding when an old API version can finally be retired.
---

# API Version Migration And Deprecation

An API version migration is the replacement of a contract that external consumers depend on, and unlike an internal refactor, you do not control all the callers. Once an API is public — to other teams, to partners, to the open internet — its consumers build against its current shape and will break in ways you cannot predict and cannot directly observe when that shape changes. The defining risk of an API change is asymmetry: you understand why the change is safe (you can see all your code), but your consumers cannot see your reasoning, and a change that is obviously non-breaking to you may be quietly catastrophic to a consumer whose integration relied on an undocumented behavior, an incidental field ordering, or a status code you considered an implementation detail. A breaking change that breaks an unknown consumer is a breaking change you cannot detect in your own tests, which is exactly why it reaches production and causes incidents you only learn about from a support ticket.

Agents tend to treat an API version migration as a code change: rename the field, add the version to the URL, ship it. This ignores the properties that make API changes dangerous — the impossibility of knowing every consumer, the long tail of integrations built against old behavior, the cost of running multiple versions, and the obligation to communicate and time the transition so consumers can adapt. The judgment problem is to classify changes honestly (a change that removes or reinterprets a field is breaking, however convenient it would be to call it additive), to choose a versioning and deprecation strategy that lets old and new coexist long enough for consumers to migrate, to communicate changes in a way consumers will actually see, and to drive toward retirement of the old version on a timeline that is fair to consumers rather than open-ended or abruptly enforced.

## Core Rules

### Classify Changes Honestly — Breaking Means Breaking, Even If It Looks Small

The most consequential decision in an API change is whether it is breaking, and the most common failure is classifying a breaking change as non-breaking because it looks minor. A change is breaking if any existing consumer could behave differently or fail as a result of it — and that bar is higher than "my own tests pass." Breaking changes include the obvious (removing a field, changing a type, altering a URL) and the subtle (narrowing the range of an accepted value, changing the order of a list that consumers happened to rely on, making a previously-optional field required, tightening validation that rejects inputs previously accepted, changing error status codes, or changing the precision or format of a returned value).

Classify with the consumer's perspective, not the provider's:

- **Removing or renaming anything** a consumer can reference — fields, endpoints, parameters, status codes, error shapes — is breaking.
- **Tightening constraints** — requiring a previously-optional field, rejecting previously-valid input, narrowing an accepted range — is breaking, because a consumer sending valid-then input now fails.
- **Changing semantics** of an existing value — returning a timestamp in a different timezone, changing what a boolean means, altering pagination behavior — is breaking even if the field name and type are unchanged.
- **Truly non-breaking changes are additive only**: new optional fields, new endpoints, new enum values a consumer can ignore, looser validation. Even additions have edge cases (a consumer with a strict deserializer that rejects unknown fields will break on a new field).

When in doubt, classify as breaking. The cost of treating a non-breaking change as breaking is minor process overhead; the cost of treating a breaking change as non-breaking is broken consumers you cannot see.

### Choose A Versioning Strategy That Supports Coexistence

A versioning strategy is not just a convention for naming versions; it is the mechanism that lets old and new API versions run in parallel long enough for consumers to migrate without forced, synchronized upgrades. The strategy must let a consumer pin to a version and keep working while the new version is built, deployed, and adopted, and it must let the provider route requests to the correct implementation.

Common strategies and their tradeoffs:

- **URL path versioning** (`/v1/`, `/v2/`) — explicit, easy to route, easy for consumers to understand and test, and visible in logs and metrics. The cost is that a version change is a URL change, which some consider part of the resource identity. It is the most operationally robust and most common choice.
- **Header or media-type versioning** — the URL stays stable and a header (custom or `Accept` media type) selects the version. This preserves REST resource semantics but is harder to test (the version is invisible in the URL), harder to route, and easier for consumers to get wrong silently.
- **Query parameter versioning** — simple but fragile; parameters are easily dropped by caching layers, proxies, or consumer HTTP clients, and the version becomes invisible in default logging.

Choose deliberately based on how consumers integrate, how visible the version must be in operations, and how long coexistence must last. Whatever the strategy, ensure it genuinely supports running multiple versions at once — a strategy that allows only one live version forces big-bang consumer upgrades, which is the failure mode the versioning is meant to prevent.

### Run Old And New In Parallel And Verify The New Before Retiring The Old

During the migration, both versions must coexist, with the old remaining the reliable path for existing consumers while the new is built, tested, and adopted. The old version must not be neglected during this period — bug fixes and security patches must be applied to both — and the new version must be verified against real traffic before it is trusted as the primary path.

Parallel-running practices:

- **Keep the old version correct and secure** for its entire supported life. A deprecated version that stops receiving security fixes while consumers still depend on it is a liability, not a graceful retirement.
- **Shadow or canary the new version** against real request shapes to find divergences (a field the new version omits, a value it formats differently) before consumers depend on it.
- **Track adoption** — know which consumers use which version, so retirement decisions are based on actual usage, not assumption. You cannot safely retire a version whose consumers you cannot enumerate.
- **Reconcile behavior** between versions where they overlap, so that a consumer migrating from old to new gets equivalent results, not surprises.

The old version can be retired only when adoption of the new is sufficient and the remaining consumers have been identified and migrated or knowingly accepted as unsupported. Retiring a version with unknown consumers is cutting off integrations you cannot see.

### Communicate Deprecation On A Timeline Consumers Can Actually Meet

A deprecation is a contract with consumers about how long they have to migrate, and it is only as good as the consumers' awareness of it. A deprecation announced only in a changelog that no one reads is, to most consumers, not a deprecation at all — it is a sudden breakage when the old version is eventually removed. Communication must reach consumers where they actually look, and the timeline must be realistic for consumers who cannot migrate instantly.

Effective deprecation communication:

- **Signal in the API itself** — return `Deprecation` and `Sunset` HTTP headers (and a `Link` to the successor) on deprecated responses, so consumers discover the deprecation through the integration they already run, not through a channel they may not monitor.
- **Announce through multiple channels** — documentation, changelog, direct outreach to known consumers, developer portals, and console warnings — because no single channel reaches everyone.
- **Set a sunset date far enough out** for consumers to plan and execute a migration, and honor it. A sunset date that keeps moving trains consumers to ignore deprecations; one that arrives without warning breaks them.
- **Provide a migration path**, not just a deadline — document what changed, why, and exactly how to adapt, with examples. A deprecation without a migration guide shifts the work of understanding the change onto every consumer individually.

The deprecation timeline is a balance: too short and it breaks consumers who cannot react in time; too long and the old version lingers as permanent maintenance debt with no pressure to migrate. Set it deliberately and hold to it.

## Common Traps

### Classifying A Breaking Change As Non-Breaking

A change that removes, renames, tightens, or reinterprets existing behavior breaks consumers you cannot see, and your own tests will not catch it because your code already conforms to the new shape. Classify from the consumer's perspective; when in doubt, treat it as breaking.

### Assuming Additions Are Always Safe

Adding a field can break consumers with strict/unknown-field-rejecting deserializers, and adding an enum value can break consumers with exhaustive switches. Even additive changes have edge cases; verify against real consumer patterns, not the assumption that additions are free.

### Deprecation Announced Only Where Providers Look, Not Where Consumers Look

A changelog or docs-only deprecation reaches providers and misses most consumers. Signal deprecation in the API response itself (headers) and through direct outreach, because consumers discover problems through the integration they run, not channels they may not monitor.

### Retiring A Version Without Knowing Who Still Uses It

Removing a version whose consumers you cannot enumerate cuts off integrations invisibly. Track adoption per consumer, and retire only when remaining usage is identified, migrated, or knowingly accepted as unsupported.

### A Sunset Date That Keeps Moving Or Arrives Without Warning

A shifting sunset trains consumers to ignore deprecations; an unannounced sunset breaks them. Set a realistic date, communicate it through channels consumers see, and honor it.

## Self-Check

- [ ] Every change was classified as breaking or non-breaking from the consumer's perspective — removals, renames, type changes, tightened validation, narrowed ranges, semantic changes, and status code changes are breaking; only purely additive, backward-compatible changes are non-breaking.
- [ ] Additive changes were checked against real consumer patterns (strict deserializers, exhaustive enum switches), not assumed universally safe.
- [ ] The versioning strategy (URL, header, media type) genuinely supports running old and new versions in parallel long enough for consumers to migrate, rather than forcing a synchronized big-bang upgrade.
- [ ] Both versions run in parallel during transition, the old version continues to receive bug fixes and security patches, and the new version was verified (shadow/canary, adoption tracking) before being trusted as the primary path.
- [ ] Deprecation is signaled in the API itself (`Deprecation`, `Sunset`, `Link` headers) and communicated through multiple channels consumers actually monitor, not only a changelog or docs page.
- [ ] A concrete sunset date exists, is realistic for consumers to meet, is documented with a migration guide (not just a deadline), and is honored rather than silently shifted.
- [ ] Adoption is tracked per consumer, and the old version is retired only when remaining usage is identified, migrated, or knowingly accepted as unsupported — not while unknown consumers still depend on it.
- [ ] No breaking change reaches consumers as a silent breakage discovered only through a support ticket rather than through advance deprecation communication.
