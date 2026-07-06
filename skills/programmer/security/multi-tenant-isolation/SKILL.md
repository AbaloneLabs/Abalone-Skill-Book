---
name: multi_tenant_isolation.md
description: Use when the agent is building, reviewing, or hardening a multi-tenant system where multiple customers, organizations, workspaces, or accounts share the same application and data infrastructure and must not see or affect each other's data. Covers tenant data isolation (logical/shared-schema vs physical/separate-schema vs separate-database), row-level security and tenant-scoped queries, preventing cross-tenant access and data leakage, resource quotas and fair-use limits, noisy-neighbor prevention, tenant-scoped configuration and feature flags, caches and search indexes that can cross tenant boundaries, background jobs and bulk exports, and verifying isolation through testing and audit. Also use when a single missed tenant filter could expose one customer's records to another.
---

# Multi-Tenant Isolation

Multi-tenancy is a security boundary expressed as a data filter. Every record belongs to a tenant, and every operation that reads or writes data must be constrained to the caller's tenant — not as a convention, but as an invariant that holds across every query, every join, every cache, every background job, and every export. The defining failure of multi-tenant systems is not a broken algorithm; it is a missing `where tenant_id = ?` in one of a thousand code paths. One missing filter exposes one customer's records to another, and in a SaaS that is not a bug — it is a breach.

Agents tend to under-invest here because the happy path is trivially correct: the request carries a tenant, the query filters by it, the right data comes back. The defects live in the paths that are easy to forget — a join that loads related records without re-applying the tenant filter, an admin endpoint that loads by primary key, a cache keyed only by resource id, a background job that runs under system authority, a search index that mixes tenants, a bulk export that iterates without scoping. Each is a place where the tenant boundary was assumed rather than enforced, and each assumption is a future cross-tenant exposure. The judgment problem is treating the tenant boundary as a property that must be mechanically enforced at every data touchpoint, not as a filter you remember to add when you think of it.

The stakes are asymmetric. A feature bug affects one user; an isolation bug exposes one customer's confidential data to another customer, often a competitor. Multi-tenant isolation failures are reportable security incidents, and they are almost always caused by ordinary code that forgot a filter, not by a sophisticated attack.

## Core Rules

### Choose The Isolation Model Knowing Its Failure Surface

There are three broad models, and each fails differently. The choice determines where the tenant boundary must be enforced and how a single mistake propagates:

- **Shared schema, logical isolation (a `tenant_id` column on every table).** Highest density, lowest cost, and the largest enforcement surface: every single query must include the tenant filter, and one miss leaks data. Isolation depends entirely on application discipline.
- **Shared database, separate schema per tenant.** Medium isolation; a query that targets the wrong schema reads the wrong tenant. The boundary is the schema/connection, which is easier to enforce than a column but can still be crossed by a connection-pool or routing mistake.
- **Separate database (or separate cluster) per tenant.** Strongest isolation; a tenant's data is physically unreachable from another tenant's connection. The boundary is infrastructure. Highest cost and operational complexity, and still vulnerable to application-level routing bugs that send a request to the wrong database.

The strongest designs do not rely on application discipline alone. Whatever the storage model, enforce the boundary as close to the data as possible (row-level security, tenant-scoped connections, per-tenant database routing) so that a missed filter in application code is caught by a layer that cannot forget. A boundary that exists only in the query writer's memory is a boundary that will eventually be forgotten.

### Enforce The Tenant Filter At The Data Layer, Not Only In Application Code

Application-level filtering (`where tenant_id = ?` in every query) is correct when it is present and catastrophic when it is absent. The robust pattern is to make the database enforce the filter so that a query which forgets it simply cannot return another tenant's data:

- **Row-Level Security (RLS) / tenant-scoped policies.** The database enforces that a session bound to tenant T can only see rows where `tenant_id = T`, regardless of what the query says. A missed application filter then returns no rows (safe failure) instead of all rows (breach).
- **Tenant-scoped connections or contexts.** The connection acquires a tenant context on each request; all queries through it are scoped automatically.
- **Per-tenant data routing.** The request resolves to a tenant-specific database, schema, or collection, so cross-tenant access requires a routing error, not a missing filter.

The principle: isolation should be a property the data layer guarantees, with application filtering as defense in depth — not the only line of defense. If the only thing preventing a cross-tenant read is the developer remembering the `where` clause, the system is one forgotten clause away from a breach.

### Make The Tenant Unforgeable And Derived Server-Side

The tenant identity must come from the authenticated principal and trusted server-side state, never from client-supplied input. A request that sends `tenant_id`, `organization_id`, or `workspace_id` in the body, query string, or a header is an invitation to IDOR-style tenant switching: the attacker changes the id and accesses another tenant.

- **Derive the tenant from the authenticated identity.** Resolve the tenant from the session, token claims, or a trusted mapping — not from a field the client controls.
- **Treat any client-supplied tenant id as a request to access that tenant, requiring explicit authorization.** If a user can legitimately belong to multiple tenants, the switch is an authorization decision (is this user a member of this tenant?), not a parameter echo.
- **Reject ambiguity.** If the tenant cannot be resolved unambiguously from the principal, fail rather than guessing.

The tenant is part of the authority, not part of the data. Treat client-supplied tenant values the way you would treat client-supplied `is_admin`: as untrusted.

### Carry The Tenant Through Every Data Touchpoint

The tenant boundary must survive every hop the data takes. The recurring bug is enforcing the tenant in the primary query and losing it downstream:

- **Joins and eager loading.** A query filtered by tenant that joins to related tables must filter every joined table by tenant too, or the join can pull in another tenant's related records.
- **Caches.** A cache keyed only by resource id serves one tenant's record to the next tenant. Cache keys for multi-tenant data must include the tenant (see the caching skill).
- **Search indexes.** A shared search index (Elasticsearch, OpenSearch) that does not filter by tenant on every query returns cross-tenant hits. The tenant must be a mandatory filter in every search, enforced by the query construction, not optional.
- **Background jobs and bulk operations.** A job that iterates records or tenants must scope each iteration to the correct tenant; a job running under system authority must re-establish the tenant context per item, not assume a global one.
- **Exports, webhooks, and integrations.** Anything that sends data out must be scoped to the tenant; an export that forgets the filter sends one tenant's data to another tenant's integration.
- **Admin and internal tools.** Internal endpoints that load by primary key bypass tenant filters by design; if they are reachable from product code or integrations, they become cross-tenant access paths.

Enumerate every place data is read, written, cached, indexed, exported, or forwarded, and confirm the tenant is enforced at each. A boundary that holds for the primary query but fails for the join, the cache, or the job is no boundary.

### Prevent Noisy Neighbors Through Resource Controls

Isolation is not only about data visibility; it is also about one tenant's behavior not degrading service for others. A tenant that runs an expensive query, a huge export, or a runaway background job can exhaust shared resources (CPU, memory, connection pool, database connections, API rate limits) and starve every other tenant. This is the noisy-neighbor problem.

- **Per-tenant rate limits and quotas.** Cap the requests, jobs, storage, or compute a single tenant can consume, so one tenant cannot monopolize shared capacity.
- **Per-tenant resource pools where feasible.** Separate connection pools, worker pools, or even compute for high-value or high-risk tenants, so their load cannot block others.
- **Query and operation timeouts.** A single tenant's slow query should time out, not hold a database connection indefinitely.
- **Bulk operation isolation.** Large exports, imports, or batch jobs should run in a separate pool or queue so they do not contend with interactive request handling.

Data isolation prevents one tenant from reading another's data; resource isolation prevents one tenant from denying service to the others. Both are part of the tenant boundary.

### Verify Isolation By Testing The Negative Case

Isolation cannot be confirmed by happy-path tests that show a tenant sees their own data. It is confirmed by negative tests that prove a tenant cannot see anyone else's. For each tenant-scoped operation:

- **Cross-tenant id test.** A caller in tenant A supplies a resource id belonging to tenant B; the request must fail (404 or 403), never return the record.
- **Cross-tenant listing test.** A caller in tenant A lists resources; tenant B's resources must be absent, not merely outnumbered by tenant A's.
- **Cross-tenant mutation test.** A caller in tenant A attempts to create, update, or delete a resource in tenant B; it must fail.
- **Join and relation test.** A caller in tenant A fetches a resource and its relations; no relation from tenant B appears.
- **Cache and search test.** After tenant B writes data, tenant A's cache and search results must not include it.

Automate these as part of the test suite for every tenant-scoped endpoint. An endpoint that passes its happy path but has no cross-tenant negative test is an endpoint whose isolation is unverified.

## Common Traps

### Filtering The Primary Query But Not The Joins

A query scoped by tenant that eagerly loads relations without a tenant filter on the joined tables, pulling another tenant's related records into the response. Every join across tenant-owned tables must re-apply the tenant filter.

### Cache Keyed Without The Tenant

A cache keyed by resource id (or id plus parameters) that serves tenant A's record to tenant B. Cache keys for multi-tenant data must include the tenant; this is the same class of bug as cross-user cache leakage, at tenant granularity.

### Trusting Client-Supplied Tenant Id

Accepting `tenant_id` or `organization_id` from the request body or query string and using it directly, so an attacker switches tenants by changing the value. Derive the tenant from the authenticated principal; treat any client-supplied tenant as an authorization request, not a parameter.

### Admin Or Internal Endpoint That Loads By Primary Key

An internal or admin endpoint that fetches by id with no tenant filter, later exposed (directly or via an integration) to a context where the caller's tenant matters. Bypass-by-design endpoints become cross-tenant paths when they leave the internal-only context.

### Background Job Running Under System Authority

A job that processes records across tenants but establishes the tenant context once (or never), so every item is processed under the first tenant's (or the system's) authority. Re-establish the tenant per item, and re-check authorization if it can change before execution.

### Search Index Without Mandatory Tenant Filter

A shared search index where the tenant filter is optional in the query construction, so a query that omits it returns all tenants' documents. Make the tenant a required, non-optional filter enforced by the search abstraction.

### Assuming Separate Schema Means Safe

Choosing separate-schema isolation and then sharing a connection pool that can route to any schema, so a routing bug sends a request to the wrong tenant's schema. The physical separation helps, but the routing layer is still an enforcement point that can fail.

### No Noisy-Neighbor Controls

Relying on data isolation alone, so one tenant's runaway query or export exhausts shared connections and degrades every other tenant. Resource quotas, timeouts, and bulk-operation isolation are part of the tenant boundary, not optional performance features.

## Self-Check

- [ ] The isolation model (shared schema, separate schema, or separate database) was chosen with awareness of its failure surface, and the boundary is enforced as close to the data as possible (RLS, tenant-scoped connections, or per-tenant routing), not solely in application query code.
- [ ] The tenant identity is derived server-side from the authenticated principal; client-supplied tenant ids are treated as authorization requests requiring explicit membership checks, never as trusted parameters.
- [ ] The tenant filter is carried through every data touchpoint — primary queries, joins and eager loads, caches, search indexes, background jobs, bulk exports, webhooks, and admin/internal endpoints — not only the primary query.
- [ ] Cache keys and search queries for multi-tenant data include the tenant as a mandatory component, verified to never serve or return another tenant's data.
- [ ] Per-tenant rate limits, quotas, timeouts, and bulk-operation isolation prevent one tenant's load from degrading service for others (noisy-neighbor defense).
- [ ] Negative isolation tests exist for every tenant-scoped endpoint: cross-tenant id access, cross-tenant listing, cross-tenant mutation, cross-tenant joins/relations, and cross-tenant cache/search — proving absence of other tenants' data, not just presence of the caller's.
- [ ] Internal and admin endpoints that bypass tenant filters are confirmed unreachable from product, integration, or multi-tenant contexts where the caller's tenant matters.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
