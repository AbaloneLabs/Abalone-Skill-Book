---
name: api_response_boundary.md
description: Use when the agent is designing, changing, or reviewing an API response and must decide which fields, relationships, identifiers, errors, permissions, and derived values are safe to expose to clients.
---

# API Response Boundary

An API response is a public contract, not a convenient dump of server state. The programmer must decide what the caller needs, what the caller is allowed to know, what the client can safely rely on, and what must remain an internal implementation detail.

This skill applies before adding a new endpoint, changing response shape, exposing a model through serialization, building admin or partner APIs, adding list/detail views, or wiring a client directly to persistence objects. It is especially important when an agent is tempted to return an ORM entity, document record, database row, debug object, or broad DTO because that is faster than defining a response contract.

## Core Rules

### Define The Caller And Their Entitlement

Start by naming the caller, not the data source. A response for the account owner, a team member, an organization admin, a public visitor, a support operator, and an internal background system may all read from the same table but must not receive the same fields.

For each response, answer:

- Who can call this endpoint?
- Which subject is the response about?
- Does the caller own the subject, administer it, observe it, or only receive a public projection?
- What fields are required for the immediate client workflow?
- Which fields are withheld even if they exist in storage?
- Are there tenant, organization, region, age, consent, or policy boundaries?

Do not let route names replace authorization reasoning. A path like `/users/me` does not prove that every nested relation belongs to the current user. A path like `/admin/users` does not prove that every admin should see every private attribute.

### Separate Storage Models From Response Models

Never treat the persistence model as the API model by default. Storage objects often contain fields that are useful for implementation but unsafe or unstable as a public contract:

- password hashes, password reset state, and auth provider ids
- session tokens, refresh tokens, API keys, secrets, and signing keys
- internal role flags, permission cache state, and policy evaluation artifacts
- fraud, moderation, risk, scoring, or abuse signals
- billing provider ids, payment metadata, and invoice internals
- soft delete flags, archival state, migration flags, and internal timestamps
- private notes, support comments, audit trails, and operator-only metadata
- raw foreign keys that reveal relationships the caller should not infer

Build explicit response structs, serializers, or projection functions. Make the allowlist visible in code. If the endpoint needs only `id`, `display_name`, and `avatar_url`, the response type should say that directly.

### Treat Identifiers As Information

Identifiers are not automatically harmless. Sequential ids can reveal scale, ordering, or undisclosed existence. Foreign keys can reveal relationship graphs. Provider ids can expose vendor choices or link accounts across systems. Internal correlation ids can help attackers join logs, requests, and leaked data.

Prefer stable public ids when clients need durable references. Avoid returning database primary keys unless the system has deliberately chosen them as public identifiers. If internal ids are already public, still check whether new endpoints expose ids from tables that were not previously visible.

### Check Nested And Bulk Responses Harder Than Detail Responses

List endpoints and nested includes are common leak points. A detail endpoint may be carefully designed, while a list endpoint returns a broad model to save code. A nested `owner`, `member`, `payment`, `organization`, or `latest_event` object may use a generic serializer that exposes more than the parent endpoint intended.

For every nested object, decide whether it uses:

- the same public projection as its own endpoint;
- a reduced embedded projection;
- a role-specific projection;
- or no projection at all, using only a count, status, or boolean.

Bulk exports, search results, autocomplete endpoints, and dashboards deserve the same care. They often cross more subjects, more tenants, and more historical records than ordinary detail views.

### Make Missing, Forbidden, And Redacted States Deliberate

The response contract should distinguish between data that does not exist, data that exists but the caller cannot see, and data that is intentionally redacted. The choice affects privacy and product behavior.

Sometimes a forbidden resource should return `404` to avoid confirming existence. Sometimes a field should be omitted. Sometimes it should be `null`. Sometimes it should be present with a redaction reason. Pick deliberately and keep the behavior consistent for the endpoint family.

Do not leak through error messages, validation details, pagination counts, timing, sorting behavior, or different status codes. If unauthorized callers can tell that an email, invite token, organization slug, document id, or private project exists, the response boundary is still leaking.

### Keep Derived Fields Accountable

Derived values can leak sensitive source data even when raw fields are hidden. Risk scores, badges, eligibility flags, "has_payment_method", "is_suspended", "last_seen", "mutual_contacts", "team_count", or "available_actions" can reveal private state.

For each derived field, ask:

- What raw data does this imply?
- Could the caller infer sensitive facts by comparing responses over time?
- Does this field encode policy or internal scoring?
- Would this field be safe in a screenshot, analytics event, support ticket, or browser cache?

Derived fields should have clear ownership. They are part of the response contract and need the same review as raw fields.

### Design For Future Compatibility

Once clients rely on a response, removing or changing fields becomes expensive. Do not expose internal fields "for now" with the hope of cleaning them later. Temporary response fields become permanent through mobile app versions, third-party clients, bookmarked exports, analytics pipelines, and undocumented scripts.

Prefer adding narrow fields for known workflows over exposing broad objects. Use versioning, feature flags, or additive migrations when response changes may affect deployed clients. Document stability expectations for public APIs and internal APIs separately.

### Audit Logs And Caches Are Part Of The Boundary

Response data does not only go to the visible UI. It may be cached by browsers, CDNs, reverse proxies, mobile apps, observability tools, analytics SDKs, test snapshots, AI transcripts, and customer support systems.

When a response contains sensitive data, confirm:

- cache headers are appropriate;
- logs do not capture full bodies;
- monitoring redaction is configured;
- test fixtures do not become public examples;
- client-side storage does not persist secrets or unnecessary personal data.

## Common Traps

### Returning The ORM Entity

The fastest implementation is often the most dangerous one. Serializing an entity directly makes every future column a potential API field. A migration that adds `internal_note`, `risk_score`, or `deleted_at_reason` can silently become a data exposure. Even if current fields are safe, direct serialization couples the public contract to storage design.

### Assuming Frontend Hiding Is Security

If the server sends the field, the user can inspect it. Hiding a column in a UI, removing it from a component, or relying on a client role check does not protect the data. The response itself is the boundary.

### Reusing Admin Serializers For User Views

Admin responses often include operational context that ordinary users should not see. Support tools may also contain private notes, flags, or escalation state. Never reuse those serializers for customer-facing endpoints without an explicit field-by-field review.

### Leaking Through Counts And Booleans

Counts, booleans, and status labels feel safe, but they can reveal existence and behavior. `has_blocked_you`, `team_member_count`, `unread_private_message_count`, `has_active_investigation`, or `payment_failed` can expose facts that the caller is not entitled to know.

### Forgetting Multi-Tenant Boundaries

Multi-tenant systems must check both the primary object and every included object. A user may be allowed to see a project but not every member, billing record, comment, attachment, audit event, or linked integration under that project.

### Treating Internal APIs As Harmless

Internal endpoints often become semi-public through browser tools, partner integrations, CLI helpers, worker services, or future reuse. Apply response boundary discipline even when the first caller is internal.

## Self-Check

- [ ] The response uses an explicit allowlisted response model, projection, or serializer rather than directly exposing a persistence object.
- [ ] The caller, subject, ownership, tenant, and authorization assumptions are clear for this endpoint.
- [ ] Sensitive storage fields, internal flags, secrets, provider ids, risk signals, private notes, and audit details are not exposed.
- [ ] Nested objects, list responses, search results, exports, and counts were reviewed for indirect data exposure.
- [ ] Identifiers in the response are intentionally public and do not reveal unnecessary relationship or scale information.
- [ ] Missing, forbidden, redacted, and nonexistent states have deliberate and consistent response behavior.
- [ ] Derived fields were checked for what they imply about hidden source data.
- [ ] Error messages, pagination metadata, sorting behavior, timing, logs, and caches do not leak data the response body hides.
- [ ] The response contract avoids temporary internal fields that clients may later depend on.
- [ ] Browser, CDN, app, analytics, logging, and test-snapshot persistence were considered for sensitive response data.
