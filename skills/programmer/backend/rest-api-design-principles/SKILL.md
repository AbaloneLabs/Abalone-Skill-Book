---
name: rest_api_design_principles.md
description: Use when the agent is designing or reviewing a REST/HTTP API — modeling resources and URLs, choosing HTTP methods and their semantics, deciding CRUD vs action/rpc-style endpoints, naming collections and items, designing request and response bodies, choosing status codes for success and failure, deciding between REST and RPC/gRPC, applying HATEOAS or not, or diagnosing chatty APIs, leaky abstractions over database tables, wrong verbs (GET that mutates, POST that should be PUT), non-idempotent creates, or APIs clients find hard to use. Covers resource modeling, HTTP method semantics, idempotency, status codes, and the boundary between a clean API contract and a database wrapper.
---

# REST API Design Principles

A REST API is a contract between a server and many clients you do not control, and the design of that contract determines whether clients can use the API correctly without reading your source, whether it evolves without breaking those clients, and whether it maps to the domain or merely to your database tables. The common failure is to expose CRUD over tables: every database table becomes a collection, every operation a POST/GET/PUT/DELETE, and the API becomes a thin remote-database wrapper that leaks the storage model, requires clients to make a dozen calls to accomplish one task, and breaks the moment the schema changes. A well-designed API models resources and actions at the domain level, uses HTTP methods and status codes for their actual semantics, and treats the contract as a stable public surface that evolves deliberately.

The judgment problem is not "what are the HTTP verbs" but "what are the resources a client reasons about, what is the granularity of each operation, does each verb do what its semantics promise, and is the contract stable enough to survive schema and feature changes." Agents tend to model resources as tables (one endpoint per table), to use POST for everything (because it "always works"), to return 200 with a body that signals errors (breaking HTTP semantics), and to design for the server's convenience rather than the client's needs. Each produces an API that works for a demo and is painful, chatty, or brittle for real clients.

## Core Rules

### Model Resources At The Domain Level, Not As Database Tables

The first and most consequential decision is what the resources are. A resource is something a client reasons about and operates on — an order, a payment, a document, a configuration — not necessarily a database table. Modeling resources as tables produces the "remote database" anti-pattern: an order is split across `order_items`, `order_status`, `shipping_address` collections, and creating an order requires the client to POST to five endpoints in the right sequence, each of which can fail. Modeling at the domain level produces an `orders` collection where POST creates a complete order (with items, address, etc.) in one operation, and the server manages the internal tables.

Design resources around the client's intent and the domain's nouns, then decide how each maps to storage internally (often several resources share storage, or one resource spans several tables). Ask, for each proposed collection: would a client think of this as a thing, or is it an implementation detail? `users`, `orders`, `documents` are things; `user_order_join`, `order_status_history`, `audit_log` are usually implementation details that should be embedded, linked, or hidden. Expose what clients need to do their work; hide what they should not have to know. The resource model is the API's public ontology, and it should match the domain, not the schema.

### Use HTTP Methods For Their Semantics; Never Mutate On GET

HTTP methods have defined semantics, and using them correctly is what makes an API predictable and toolable (caches, proxies, idempotency-aware clients all rely on method semantics). `GET` is safe (no side effects) and cacheable; it must never mutate state. `POST` creates a new resource (non-idempotent, generally) or triggers an action. `PUT` replaces a resource at a known URL (idempotent — the same PUT twice leaves the same state). `DELETE` removes a resource (idempotent). `PATCH` partially updates a resource. These are not arbitrary labels; caches will store GET responses, idempotency-aware retries will re-issue PUT/DELETE safely, and a GET that mutates breaks both.

The dominant violation is GET-that-mutates: a `GET /api/flag-user?id=5` that flags the user, or a `GET /api/cancel-order/123` that cancels. This breaks caching (a CDN or browser may serve a cached response and never hit the server), breaks retry safety (a client that retries a failed GET re-mutates), and violates the principle of least surprise. State-changing operations use POST, PUT, PATCH, or DELETE. The second violation is POST-for-everything: using POST for reads (`POST /api/getOrder` with a body) defeats caching, idempotency, and HTTP tooling. Map operations to the verb whose semantics match; reserve POST for creates and for actions that have no natural verb.

### Make Creates Idempotent And Decide Create-vs-Update (POST vs PUT) Deliberately

Resource creation has two idiomatic shapes, and the choice affects idempotency and client behavior. **POST to a collection** (`POST /orders` with a body) creates a new resource and the server assigns the id; it is non-idempotent, so a client retry can create duplicates unless protected by an idempotency key. **PUT to a specific URL** (`PUT /orders/123` with a body) creates or replaces the resource at that URL, with the client supplying the id; it is idempotent, so retries are safe. POST-to-collection is the common default for server-assigned ids; PUT-to-specific-URL fits when the client has a natural id (a UUID it generated, a slug, a composite key) and wants create-or-replace semantics.

Because POST-to-collection is non-idempotent and clients retry, production creates need an idempotency key (a client-supplied unique key the server deduplicates within a window) so a retried create does not double-create. This is not optional for mutating endpoints that clients retry — a missing idempotency key on a payment or order create is a double-charge/duplicate-order bug waiting for the first network blip. For updates, PUT replaces the whole resource (the client must send the full representation; omitting a field means "delete it"), while PATCH partially updates (only the sent fields change). Choose PUT vs PATCH by whether the client is sending the full resource or a delta, and be consistent; a PUT that merges some fields and ignores others is neither PUT nor PATCH and confuses clients.

### Choose Status Codes That Tell The Client What To Do

Status codes are the API's primary feedback channel, and the right code tells a client whether to retry, fix its request, or give up. Success: `200 OK` (with a body), `201 Created` (for creates, with a `Location` header pointing at the new resource), `202 Accepted` (for async work started but not finished), `204 No Content` (success with no body, e.g., a DELETE). Client errors: `400 Bad Request` (malformed), `401 Unauthorized` (not authenticated), `403 Forbidden` (authenticated but not allowed), `404 Not Found`, `409 Conflict` (state conflict, e.g., duplicate or version mismatch), `422 Unprocessable Entity` (well-formed but semantically invalid). Server errors: `500` (server bug), `502/503/504` (downstream/overload/timeout, often retryable).

The discipline is to pick the code whose semantics match and to be consistent across the API. Do not return 200 with an error body (breaks every client and tool that checks status). Do not return 500 for client problems (validation, not-found) — clients and on-call treat 500 as a server incident and may retry it. Do distinguish retryable (5xx from downstream, 429) from terminal (4xx) so clients can recover. The full error-shape and retryability discipline is covered in the error-and-status-codes skill; the API-design rule is that status codes are part of the contract, chosen deliberately, documented, and stable.

### Keep APIs Coarse Enough To Avoid Chatter, And Compose With Sub-Resources And Embedding

A chatty API — one where a client must make many round-trips to accomplish one task — is slow, fragile, and a sign of table-level modeling. Creating an order should be one call, not five. Fetching an order with its items and shipping address should be one call, not three followed by client-side joins. Coarseness means designing operations at the granularity of the client's task: the unit of work a client performs becomes the unit of the API call. This often means accepting and returning compound representations (an order with its items embedded) rather than forcing the client to assemble them.

Use sub-resources and embedding to express relationships without chattiness: `GET /orders/123` can embed the items by default or via a parameter (`?expand=items,shipping`), and `GET /orders/123/items` addresses the items as a sub-collection. Use links (a `links`/`_links` section or a `Location`/`self` URL) so clients navigate by following links rather than constructing URLs (this is the practical, limited form of HATEOAS — full HATEOAS is rarely worth the complexity, but stable link relations are). Decide embedding vs linking by size and use: embed what most clients always need (small, common), link what is large or rarely needed. The goal is that common client tasks take few calls, not that every field is a separate endpoint.

### Design The API As A Versioned, Evolving Contract

An API is a public contract that clients depend on, and it must evolve without breaking those clients. Breaking changes — removing a field, changing a field's type or meaning, changing a status code's semantics, requiring a previously optional field, changing error shapes — must be gated behind a version. Non-breaking changes — adding an optional field, adding a new endpoint, loosening a validation rule — can ship without a version bump. Decide the versioning strategy (URL path `/v1/`, header, content negotiation) and apply it consistently; URL path versioning is the most common and the most tool-friendly, despite being "impure" by strict REST.

The deeper discipline is to design fields and operations for forward compatibility from the start: clients must ignore unknown fields (so the server can add them), use stable field names that will not need renaming, and prefer optional fields over required ones (a required field is a permanent constraint). The full versioning discipline (deprecation, sunset, parallel versions) is covered in the api-versioning skill; the design principle here is that the API is not a snapshot but a long-lived contract, and every design decision (field names, requiredness, status codes, error shapes) is a commitment to future clients. Design as if the API will be used for a decade by clients you cannot update, because it often will be.

### Decide REST vs RPC Deliberately, And Use RPC Where REST Fits Poorly

Not every operation is a CRUD on a noun. Some operations are verbs with no natural resource: "send a message," "approve a request," "calculate a quote," "train a model," "export a report." Forcing these into REST shapes (`POST /messages` for send is fine, but `POST /approvals/123/actions` for approve, or `POST /quote-calculations` for a pure function, gets awkward) produces contortions. It is acceptable, and often cleaner, to expose RPC-style endpoints (`POST /orders/123/cancel`, `POST /reports/export`) for actions that do not map to resource CRUD, while keeping REST shapes for the resource CRUD that does fit.

The rule is to choose the style that makes the operation clear to the client, and to be consistent within the API. A hybrid API (REST for nouns, RPC for verbs) is common and reasonable; a pure-REST API that twists verbs into noun collections to satisfy doctrine is worse than an honest RPC endpoint. For internal, performance-sensitive, or schema-heavy APIs, gRPC (with protocol buffers) may be a better fit than REST/JSON entirely — it gives typed contracts, streaming, and efficient encoding, at the cost of browser/tooling simplicity. The choice is a design decision driven by the clients, the performance needs, and the operation shapes, not by REST dogma.

## Common Traps

### One Endpoint Per Database Table (Remote-Database API)

`/users`, `/user_orders`, `/order_items`, `/addresses` each a collection, forcing clients to assemble an order from many calls. Model resources at the domain level; coarsen operations to the client's task.

### GET That Mutates State

`GET /cancel-order/123` or `GET /flag-user?id=5`. GET must be safe and cacheable; use POST/PUT/PATCH/DELETE for mutations.

### POST For Everything (Including Reads)

`POST /getOrder` with a body, defeating caching, idempotency, and HTTP tooling. Use GET for reads; reserve POST for creates and actions.

### Non-Idempotent Create With No Idempotency Key

A POST create that clients retry, double-creating orders or charges. Accept an idempotency key and deduplicate; or use PUT-to-specific-URL for client-supplied ids.

### PUT That Merges Instead Of Replaces

A PUT that updates only the sent fields (which is PATCH semantics), confusing clients about what omitting a field means. PUT replaces the whole resource; PATCH updates partially — pick one and be consistent.

### Wrong Or Inconsistent Status Codes

200-with-error-body, 500 for validation, 401 where 403 is meant. Choose codes by semantics, document them, and be consistent; encode retryability so clients can recover.

### Chatty API Requiring Client-Side Joins

Many small endpoints where a few coarse ones with embedding/links would serve the common task in one call. Coarsen to the client's task; embed common data, link rare data.

### Breaking Changes Shipped Without A Version Bump

Removing or renaming a field, changing a status code, tightening validation — breaking clients that depend on the contract. Gate breaking changes behind a version; design for forward compatibility (optional fields, stable names, clients ignore unknowns).

### Forcing Verbs Into Noun Collections To Satisfy REST Dogma

`POST /cancellations` for "cancel order 123" when `POST /orders/123/cancel` is clearer. Use RPC-style action endpoints where REST CRUD fits poorly; a hybrid API is fine.

## Self-Check

- [ ] Resources are modeled at the domain level (orders, payments, documents), not as one collection per database table; the resource ontology matches what clients reason about, not the storage schema.
- [ ] HTTP methods match their semantics: GET is safe and never mutates; POST creates or triggers actions; PUT replaces (idempotent); DELETE removes (idempotent); PATCH partially updates; no GET-mutates and no POST-for-reads.
- [ ] Creates are idempotent (POST-to-collection with an idempotency key, or PUT-to-specific-URL with a client-supplied id), and PUT vs PATCH is chosen consistently (full replace vs partial update).
- [ ] Status codes are chosen by semantics and documented (201 + Location for creates, 202 for async, 204 for no-content, 4xx for client errors with 401 vs 403 vs 404 vs 409 vs 422 distinguished, 5xx for server errors with retryable distinguished), and no endpoint returns 200-with-error-body.
- [ ] The API is coarse enough that common client tasks take few calls; compound representations, sub-resources, and embedding/links reduce chatter rather than forcing client-side joins.
- [ ] The API is designed as an evolving contract: fields are optional and stably named, clients ignore unknown fields, breaking changes are gated behind a version, and the versioning strategy is applied consistently.
- [ ] REST vs RPC (and REST/JSON vs gRPC) is chosen deliberately per operation, with RPC-style action endpoints used where CRUD-on-nouns fits poorly, and the choice documented rather than driven by dogma.
- [ ] The API was reviewed from a client's perspective (write the client code for the common tasks): if a client needs many calls, undocumented sequencing, or knowledge of internal ids/tables to accomplish a task, the design is wrong.
