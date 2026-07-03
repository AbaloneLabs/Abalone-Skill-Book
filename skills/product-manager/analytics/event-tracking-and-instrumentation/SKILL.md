---
name: event_tracking_and_instrumentation.md
description: Use when the agent is defining analytics event schemas, designing a tracking plan, specifying event properties and identity resolution, reviewing data collection for quality and privacy, or coordinating instrumentation with engineering before launch.
---

# Event Tracking And Instrumentation

Instrumentation is the layer where product behavior becomes analyzable data. A tracking plan written casually produces dashboards that quietly disagree, experiments that cannot be trusted, and cohorts that mix the wrong users together. The product manager rarely writes the SDK calls, but owns the contract: which events exist, what each event means, which properties it carries, who the user is, and how the data stays usable over time.

The harm this skill prevents is invisible until launch. Events are named inconsistently, properties are added ad hoc, identity is merged too late or too early, sensitive data leaks into analytics, and historical analysis breaks when an event is renamed. By the time the dashboard looks wrong, the data is already collected and the cost of fixing it is a re-launch.

Use this skill before writing a tracking plan, specifying events for a new feature, reviewing an instrumentation pull request, setting up identity resolution, defining event properties, versioning events, or coordinating with engineering on data collection. Ask broadly: which events must exist, what must each carry, who is the user, how do we know the data is clean, and how do we keep it usable a year from now.

## Core Rules

### Build A Tracking Plan As A Contract, Not A Wishlist

A tracking plan is the single source of truth for every event the product emits. It is not a list of nice-to-have events; it is a contract shared between product, engineering, and data. Each event entry must define the event name, a one-sentence definition, the trigger condition, the required properties with types and allowed values, the identity fields, and the owner.

Treat the plan as versioned documentation. When an event changes, record the change, the date, and the reason, so analysts can align historical data with the current definition. A tracking plan that lives only in a chat thread or a stale spreadsheet is already broken.

### Choose A Stable Naming Convention And Enforce It

Event names are forever. Once analysts write queries and dashboards against a name, renaming it silently invalidates every downstream artifact. Pick a convention early and enforce it through review or tooling.

Common conventions include object-action (`signup_completed`, `payment_refunded`) or verb-past-tense (`signed_up`, `refunded`). Whichever is chosen, require consistency in tense, snake_case or camelCase, namespace prefixes for surfaces (`checkout_payment_succeeded`), and a clear distinction between exposure, action, and outcome. Reject event names that encode the UI (`blue_button_clicked`) because the UI changes while the behavior should not.

### Separate Exposure, Action, And Outcome Events

Conflating these three is the most common instrumentation mistake and it corrupts conversion math. Exposure events record that a user could act, such as a screen viewed or an element rendered. Action events record that the user did something, such as a button tapped or a form submitted. Outcome events record a meaningful result, such as an account created or a purchase completed.

A conversion rate is only valid when the denominator is an exposure or earlier action and the numerator is a later action or outcome. If the only event is `purchase_completed`, the team cannot compute checkout conversion because there is no `checkout_started` denominator. Define the full chain and instrument each link separately.

### Design Properties To Answer Expected Questions

Every property should be traceable to a question someone will ask. Before adding a property, name the analysis it enables. If no question depends on it, do not collect it. Properties without a purpose become schema noise, increase storage cost, and widen the privacy surface.

For each event, distinguish identifiers (`user_id`, `session_id`, `device_id`, `experiment_variant`), context (`app_version`, `platform`, `locale`, `build`), and attributes specific to the event (`plan_tier`, `item_count`, `error_code`). Require enums with allowed values rather than free text where the set is bounded, so that `platform: ios` and `platform: iOS` do not split a segment.

### Solve Identity Resolution Before You Need It

Users start anonymous and become known. A visitor arrives on a device, browses, and only later signs in or pays. If the anonymous device id and the known account id are never linked, the same human appears as two users, and retention, funnel, and lifetime analyses are all wrong.

Decide the merge model: deterministic merge on sign-in, deterministic merge on email match, or probabilistic merge. Record the anonymous id on every event, capture the sign-in event with both ids, and backfill the merge so historical events are attributed to the known user. Document the merge latency, because some dashboards will read pre-merge data and disagree with post-merge data for a window of time.

### Enforce Data Quality Gates Before Trusting Dashboards

Data quality is not a hope; it is a set of checks. Before a dashboard or experiment is trusted, validate that events arrive in the expected volume, that required properties are populated, that values fall in allowed ranges, that events arrive in the expected order, that duplicates are controlled, and that sampling does not silently hide low-traffic segments.

Define what "good" looks like per event: expected daily volume band, non-null rate for required properties, value distribution sanity, and ordering constraints such as `signup_started` before `signup_completed`. When a check fails, the dashboard should be marked stale or untrusted, not presented as fact. A team that learns to distrust its own data stops using it.

### Apply Privacy By Design And Data Minimization

Analytics data accumulates, and what is collected can be subpoenaed, breached, or repurposed. The product manager must apply data minimization at the schema level, not after the fact. Do not collect raw personal data, credentials, full message content, location at fine granularity, or free text the user did not intend to share unless there is a governed reason.

Map each property to a purpose and a retention. Hash or tokenize identifiers where possible. Respect consent and opt-out at collection time, not downstream. Ensure region-specific rules such as GDPR or CCPA are reflected in what is sent. If a property is "just in case," remove it.

### Version Events Without Breaking History

Products change, and so do events. The challenge is evolving an event without invalidating months of historical analysis. Prefer additive changes: add a new optional property rather than changing the meaning of an existing one. When the semantic meaning must change, create a new event (`checkout_completed_v2`) rather than redefining the old one in place.

Record a changelog with effective dates. Where a breaking change is unavoidable, mark the boundary in dashboards and experiments so analysts do not compare pre-change and post-change data as a continuous series. Never silently redefine an event and hope no one notices.

## Common Traps

### Over-Instrumenting Everything

Adding an event for every click feels thorough but creates event bloat. Hundreds of low-value events inflate cost, slow queries, drown analysts in noise, and widen the privacy surface. Each event should earn its place by answering a question that informs a decision.

### Naming Events After The UI

An event named `blue_button_clicked` becomes meaningless after a redesign. Name events after the user intent or business action (`payment_method_selected`), not the visual element. UI-coupled names force a re-instrumentation every time the interface changes.

### Conflating Exposure With Action

If `checkout_viewed` is also fired when the user only scrolled past without rendering, exposure counts are inflated and conversion rates look artificially low. Define exposure precisely: the element rendered and was actually visible to a real user, not a bot or a pre-render.

### Merging Identity Too Early Or Too Late

Merging on the first weak signal creates false links between unrelated users; merging only at sign-in leaves anonymous behavior orphaned forever. Pick a merge model deliberately, test it on known edge cases such as shared devices and account switching, and document the latency.

### Trusting Volume Without Checking Quality

A dashboard showing the expected event volume does not prove the data is correct. Required properties can be null, values can be wrong, and duplicates can inflate counts while the total still looks plausible. Volume is necessary but not sufficient; validate properties and ordering, not just totals.

### Letting Properties Drift To Free Text

Free-text properties such as `source: "facebook ad"` and `source: "Facebook Ad"` fragment segments and break filters. Bound every property with an enum or a controlled vocabulary, and reject values outside it at collection time rather than cleaning them later.

### Breaking History With A Silent Redefinition

Redefining `purchase_completed` to also include refunds, without a version boundary, makes the time series jump and no one knows why. Silent redefinitions are worse than missing data because they look continuous. Always version or branch the event and record the change.

### Skipping QA Of Instrumentation

Engineering ships the events, but no one verifies them against the plan. Properties are missing, events fire twice on retry, or the identity id is empty on one platform. Treat instrumentation like a feature: write test cases for each event, run them on every platform, and block release on failure.

## Self-Check

- [ ] Every event in the tracking plan has a definition, trigger condition, required properties with types and allowed values, identity fields, and an owner.
- [ ] Event names follow one enforced convention and describe user intent or business actions, not UI elements.
- [ ] Exposure, action, and outcome events are separated so conversion denominators and numerators are unambiguous.
- [ ] Each property traces to an analysis question; properties without a purpose were removed.
- [ ] Identity resolution has a documented merge model, captures anonymous-to-known links, and notes merge latency.
- [ ] Data quality gates check volume, non-null rates, allowed values, ordering, duplicates, and sampling before dashboards are trusted.
- [ ] Personal, sensitive, or regulated data is minimized, hashed or tokenized where possible, and mapped to a purpose and retention.
- [ ] Event changes are additive or versioned with a changelog and effective dates, and breaking changes are marked in downstream artifacts.
- [ ] Instrumentation was QA-tested per platform against the tracking plan before release.
- [ ] The plan avoids event bloat and each event earns its place by supporting a decision.
