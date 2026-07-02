---
name: authentication_authorization.md
description: Use when the agent is designing, implementing, or reviewing login, session handling, permission checks, role-based access, tenant isolation, ownership rules, or any feature where a caller may access or mutate protected resources.
---

# Authentication And Authorization

Authentication answers who the caller is. Authorization answers what that caller may do. A programmer must keep those decisions separate and explicit. Many security defects happen when code checks that a user is logged in but forgets to check ownership, tenant membership, resource state, role scope, or operation-specific permission.

Use this skill before building endpoints, background jobs, admin tools, internal dashboards, file access, invitation flows, organization features, API keys, webhooks, or any feature where an identity can read, create, update, delete, approve, export, or trigger work.

## Core Rules

### Separate Identity From Permission

A valid identity is only the start. The user, service account, API key, session, or integration may be real but still not allowed to perform the requested action.

For each protected operation, define:

- the actor type: user, service, admin, support operator, integration, anonymous visitor;
- the resource type and specific resource instance;
- the action: read, list, create, update, delete, export, invite, approve, impersonate, trigger;
- the relationship between actor and resource;
- the conditions that narrow the permission.

Do not rely on a generic `is_admin`, `is_authenticated`, or `role == owner` check unless it fully describes the operation. Permissions often depend on resource state, tenant, plan, region, billing status, feature flag, or delegated scope.

### Check Object-Level Access

Route-level middleware can prove that a caller has a session or broad role. It usually cannot prove that the caller may access a specific object. Object-level authorization must happen after loading or identifying the resource and before returning or mutating it.

Common object checks include:

- user owns the object;
- user belongs to the organization that owns the object;
- user has a role within the project, workspace, account, or group;
- service account has a scope for this resource type;
- support operator has an approved reason to access the customer record;
- resource is in a state that allows the action.

Every endpoint that accepts an id, slug, token, path, filename, or foreign key needs an object-level access review.

### Enforce Tenant Isolation Everywhere

Multi-tenant boundaries must appear in queries, mutations, joins, batch jobs, search indexes, caches, exports, and background tasks. It is not enough to check tenant membership once if later code loads related records without tenant filters.

Prefer queries that include tenant constraints directly:

- select project where `id = ? and organization_id in actor_orgs`;
- update document where `id = ? and workspace_id = actor_workspace`;
- list events only through authorized parent objects.

Avoid loading by bare primary key and then checking later unless the pattern is unavoidable and consistently audited. A missing later check becomes a data breach.

### Model Permissions As Capabilities

Roles are shorthand. Capabilities are the actual decisions. A role like `admin`, `editor`, `manager`, or `viewer` may mean different things across features. Convert roles to named capabilities such as `document.read`, `billing.update`, `member.invite`, or `case.export`.

This helps agents avoid accidental permission expansion. Adding a new button or endpoint should not simply ask "is this user an admin?" It should ask "which capability is required for this action, and which roles grant it under which conditions?"

### Validate Input Before Authorization Side Effects

Authorization often depends on parsed input. Validate identifiers, scopes, requested roles, target emails, file paths, and action names before using them in permission logic. However, do not leak resource existence through validation order. If a caller supplies an id they cannot access, the system may need to return the same response as an id that does not exist.

Avoid side effects before the authorization decision is final. Do not send invites, enqueue jobs, create audit events with sensitive data, write partial records, or call external systems before confirming the actor can perform the action.

### Treat Delegation And Impersonation As High Risk

Invitations, share links, API keys, OAuth tokens, support impersonation, delegated admin, and machine-to-machine credentials all create authority outside a direct login session.

For each delegated access mechanism, define:

- who can create it;
- what exact scope it grants;
- when it expires;
- how it can be revoked;
- whether it is single-use or reusable;
- how it is audited;
- whether it can be escalated or chained.

Tokens should be unguessable, stored safely, and compared safely. Do not put long-lived secrets in URLs if they will be logged, copied, or sent as referrers.

### Log Decisions Without Leaking Secrets

Security reviews need auditability. Record the actor, action, resource, decision, reason code, and request context. Do not log passwords, session tokens, API keys, private documents, full request bodies, or sensitive personal data.

Authorization denial logs should be useful for debugging and abuse detection, but they should not create a second sensitive data store.

### Test Negative Cases

Permission systems are not validated by happy-path tests alone. Each protected operation should have tests for:

- unauthenticated caller;
- authenticated but unrelated caller;
- caller in same tenant without required role;
- caller with old or revoked access;
- caller using a resource id from another tenant;
- caller attempting a disallowed state transition;
- service account missing required scope.

If an endpoint lists resources, test that forbidden resources are absent, not merely that allowed resources are present.

## Common Traps

### Checking Authentication Only

The common defect is `current_user` exists, so the operation proceeds. This misses ownership, tenant, role, and state. Login is not permission.

### Trusting Client-Supplied Ownership

Do not trust `user_id`, `organization_id`, `role`, `is_admin`, `owner_id`, or scope values sent by the client. The server must derive authority from the authenticated principal and trusted server-side relationships.

### Authorizing The Parent But Not The Child

A caller may access a workspace but not every file, comment, billing record, private channel, or audit event within it. Nested resources need their own rules, especially when they cross visibility levels.

### Reusing Admin Endpoints Internally

Internal tools and admin endpoints often bypass normal policy. If reused by product code, integrations, or automation, they can grant broader access than intended. Admin access should still be capability based and audited.

### Missing Authorization In Background Jobs

Jobs started by a request may run later under system authority. Store the initiating actor, intended scope, and authorization decision. Re-check when needed if permissions can change before execution.

### Letting Caches Cross Boundaries

Caches keyed only by resource id can serve one tenant or role's response to another. Cache keys for protected data must include tenant, actor class, permission version, visibility scope, or another sufficient boundary.

## Self-Check

- [ ] The code separates authentication from authorization and names the required action for each protected operation.
- [ ] Every id, slug, token, file path, or foreign key input has object-level access control.
- [ ] Tenant, organization, workspace, project, and ownership boundaries are enforced in queries and mutations, not only in UI code.
- [ ] Roles are mapped to explicit capabilities or operation-specific permissions.
- [ ] Client-supplied ownership, role, scope, or tenant values are not trusted as authority.
- [ ] Nested resources, list endpoints, exports, search, background jobs, and caches preserve the same access boundaries.
- [ ] Delegated access, API keys, invite links, impersonation, and service accounts have explicit scope, expiry, revocation, and audit behavior.
- [ ] Denials avoid leaking resource existence unless the API intentionally reveals it.
- [ ] Security logs record useful decision context without storing secrets or unnecessary personal data.
- [ ] Negative authorization tests cover unrelated users, wrong tenants, insufficient roles, revoked access, and missing service scopes.
