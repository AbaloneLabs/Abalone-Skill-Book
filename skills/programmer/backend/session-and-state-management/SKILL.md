---
name: session_and_state_management.md
description: Use when the agent is designing or reviewing session management, authentication state, cookies, JWT/OAuth tokens, server-side sessions, distributed session stores (Redis/memcached), CSRF protection, stateless vs stateful architecture, shopping carts, multi-step workflows, or diagnosing session fixation, CSRF, token theft, stale-cache inconsistency, session data growing unbounded, or sticky-session failures under failover. Covers where state lives, how it is authenticated and rotated, consistency across replicas, the security boundaries of cookies and tokens, and the tradeoffs of stateless vs stateful sessions.
---

# Session And State Management

Session and state management is the decision of where the truth about "who is this user and what are they doing" lives, how it survives across requests, and how it stays correct and secure when there are many users, many servers, and an adversarial network in between. It looks simple — "put a cookie on the response" — and it is the source of some of the most damaging bugs in web security and distributed systems: session fixation that lets an attacker steal an account, CSRF that lets a malicious site perform actions as the victim, a JWT that cannot be revoked so a stolen token is valid until it expires, a shopping cart stored in a server-side session that disappears when the load balancer routes the next request to a different replica, a session store that grows unbounded until the cache evicts active users.

The judgment problem is not "how do I set a cookie" but "where is this state authoritative, how is it authenticated on each request, how does it behave under failover and scale, and what is the threat model." Agents tend to default to whatever the framework makes easy (often a server-side session with sticky routing, or a stateless JWT), without reasoning about the consistency, revocation, and security implications of the choice. Each default works for a demo and fails in a specific way in production: sticky sessions break on failover; JWTs cannot be revoked; server-side sessions add a round-trip and a single point of failure; client-side state tampering opens security holes.

## Core Rules

### Decide Where State Lives: Client, Server, Or Shared Store

The first decision is where the session state is authoritative, and each choice has a distinct cost. **Client-side state** (cookies, JWTs, local storage) puts the state on the client and sends it with each request; it is stateless on the server (scales horizontally, no shared store), but it cannot be revoked without extra machinery, it is bounded by size limits, and it must be integrity-protected against tampering. **Server-side state with a shared store** (session id in a cookie, state in Redis/a database) keeps the server authoritative; it can be revoked and updated instantly, holds large state, but adds a round-trip per request and makes the store a dependency. **Server-side state with sticky routing** (session id in a cookie, state in local memory, load balancer pins the user to one replica) avoids the shared store but breaks on failover and complicates deploys.

Choose based on the requirements. If you need revocation (security-sensitive sessions), large state, or instant updates, use a shared store. If you need horizontal scale with no shared dependency and can tolerate un-revokable tokens with short lifetimes, use signed client-side state. Avoid sticky routing as the primary strategy — it is fragile under the exact conditions (failover, rolling deploy) where you most need correctness. The decision is architectural and should be made once, deliberately, not defaulted per feature.

### Authenticate Session State On Every Request; Never Trust The Client

A session identifier or token arriving from the client is a claim, not a proof. The server must verify it on every request: check the signature (for signed tokens/cookies), check the expiry, check against a revocation list or store (for revocable sessions), and check that the session belongs to the requesting context (right audience, right issuer, not replayed). A session id that the server does not look up is just a string the client chose; an unsigned cookie that the server reads as authoritative is a privilege-escalation hole.

The integrity protection must be cryptographic and server-held. Sign cookies with an HMAC keyed by a server secret (or encrypt them if they are confidential); never store privileged state (user id, role, balance) in a plain or base64 cookie. For JWTs, verify the signature with the right key, check `iss`/`aud`/`exp`/`nbf`, and have a revocation story (see below). Treat every piece of client-supplied state as forged until proven otherwise.

### Make Tokens Revocable, Or Keep Their Lifetime Short

A stateless token (JWT, signed cookie) cannot be revoked by design — the server has no list of valid tokens, it just verifies signatures. This is the central security tradeoff of stateless sessions: a stolen token is valid until its expiry, with no way to invalidate it server-side. If a token lives for a week and is stolen, the attacker has a week of access. The defenses are: keep access-token lifetimes short (minutes, not days), use a refresh-token flow where the refresh token is revocable and stored server-side, and/or maintain a revocation list (a blocklist of revoked token ids, checked on each request — which reintroduces some server state).

If you need instant revocation (compromised account, password change, logout that actually logs out everywhere), you need server-side state: either a session store the server checks, or a revocation list the server checks. Do not claim "we use JWTs so we can revoke anytime" — you cannot, without a server-side list, and if you have the list you might as well use sessions. Match the revocation requirement to the mechanism honestly.

### Protect Against Session Fixation, CSRF, And XSS-State-Theft

Three attacks dominate session security, and each has a specific defense. **Session fixation** is when an attacker sets the victim's session id (via a URL or a cookie they force), then takes over after the victim authenticates. Defense: regenerate the session id on authentication (on login, on privilege change), and never accept session ids from URLs. **CSRF** is when a malicious site makes the victim's browser send an authenticated request (the browser attaches the cookie). Defense: use anti-CSRF tokens (a random token in the form, validated server-side) for state-changing requests, set cookies with `SameSite=Lax` or `SameSite=Strict`, and require custom headers or `Origin`/`Referer` checks for API endpoints. **XSS-state-theft** is when a script injection reads the session cookie or token from storage. Defense: mark cookies `HttpOnly` (so script cannot read them) and `Secure` (so they are only sent over HTTPS); store tokens in memory or HttpOnly cookies rather than local storage where any XSS can read them.

These defenses compose and none is sufficient alone. `SameSite` helps with CSRF but is not universal (older browsers, cross-site flows); CSRF tokens help but must cover every state-changing endpoint; `HttpOnly` prevents script theft of cookies but does not help if the token is in local storage. Layer them, and test that each state-changing endpoint is protected.

### Make Session State Consistent Across Replicas, Or Accept The Inconsistency

If session state lives in server memory and requests can land on any replica, you have a consistency problem: a login on replica A is invisible to replica B until the state replicates, and a failover loses in-memory state entirely. The options are a shared store (strong consistency, adds a dependency and a round-trip), sticky routing (each user pinned to one replica, breaks on failover), or eventual consistency (replicate with delay, accept stale reads). Most production systems use a shared store (Redis, a database) for exactly this reason: it makes session state consistent across replicas and survivable across failovers.

If you accept eventual consistency (e.g., caching session data locally with a TTL and replicating changes), be explicit about the staleness window and its consequences: a logout may take seconds to propagate, a role change may take seconds, and during that window the old state is authoritative. Do not pretend eventual consistency is strong consistency; a security-sensitive change (revoking access) that takes seconds to propagate is a seconds-long window of unauthorized access.

### Bound Session State Size And Lifetime

Session state that grows without bound is a memory leak and a denial-of-service vector. A shopping cart that accumulates items forever, a session that stores every page visited, a "recently viewed" list that never trims — each turns the session store into an ever-growing per-user cost, and at scale the aggregate exhausts memory or store capacity. Bound every piece of session state: cap lists at N items, set TTLs so abandoned sessions expire, and periodically compact or evict.

The lifetime decision is also security: a session that never expires stays valid if the device is stolen or the token leaks; a session that expires too aggressively destroys usability. The standard pattern is an idle timeout (expires after N minutes of inactivity) plus an absolute max lifetime (expires after N hours regardless of activity), with a refresh flow for long-lived sessions. Choose the timeouts based on the sensitivity of the data: a banking session times out in minutes; a content site may persist for weeks. Document the choice and the threat model behind it.

### Separate Authentication State From Application Workflow State

There are two kinds of state often conflated under "session," and they have different requirements. **Authentication state** (who the user is, their roles, whether they are logged in) is security-critical: it must be authenticated, revocable, and short-lived. **Workflow/application state** (shopping cart, multi-step form progress, wizard step, draft) is convenience state: it should persist across sessions, can be larger, and its loss is annoying rather than dangerous. Mixing them — putting the shopping cart in the auth session, or putting the user id in the workflow state — creates both security and usability problems.

Separate them. Auth state goes in a signed, short-lived, revocable token or a server-side session store checked on every request. Workflow state goes in its own store (a database, a separate cache key) keyed by user, with its own lifetime and size bounds. A logout clears auth state; it does not need to clear the shopping cart (the user may want it when they log back in). Keeping them separate makes each easier to reason about and avoids the bug where clearing one accidentally clears the other.

### Handle Logout, Expiry, And Token Rotation Deliberately

Logout is not just "delete the cookie on the client" — for server-side state it must invalidate the session in the store, and for stateless tokens it must either rely on short expiry or maintain a revocation list. A logout that only clears the client cookie leaves the token valid if it was captured elsewhere. Implement logout as server-side invalidation plus client-side clearing, and for high-security sessions, invalidate all of a user's sessions on password change or suspected compromise.

Token rotation (issuing a new token on each request or on a schedule) limits the window of a stolen token but introduces its own complexity: the old token must be accepted briefly (the rotation window) or concurrent requests break, and the rotation must be atomic or a race leaves the user logged out. Rotate deliberately, with a documented window and a fallback. Do not rotate on every request unless you have measured the cost (signature/verification overhead, store writes) and have a plan for concurrency.

## Common Traps

### Unsigned Or Tamperable Client-Side State

A cookie or token storing `role=admin` without a signature, so a client can edit it and escalate. Sign or encrypt all client-side privileged state; verify on every request.

### Un-Revokable Long-Lived JWTs

A JWT valid for a week with no revocation list, so a stolen token gives a week of access. Keep access tokens short-lived; use revocable refresh tokens or a blocklist if you need instant revocation.

### Sticky Sessions As The Primary Strategy

Session state in local memory with load-balancer stickiness, breaking on failover or rolling deploy. Use a shared store if state must survive replica changes.

### Missing Session Id Regeneration On Login

Accepting the same session id before and after authentication, enabling session fixation. Regenerate the id on login and on privilege change.

### CSRF Protection Only On Some Endpoints

Anti-CSRF on the login form but not on the password-change or transfer endpoints. Cover every state-changing endpoint; layer `SameSite`, tokens, and origin checks.

### Tokens In Local Storage With No XSS Defense

JWTs in `localStorage` readable by any injected script. Use HttpOnly cookies or in-memory storage; mark cookies `Secure` and `HttpOnly`.

### Session State Growing Without Bound

A shopping cart or history list in the session that never trims, leaking memory per user. Cap list sizes, set TTLs, evict periodically.

### Mixing Auth State And Workflow State

The shopping cart in the auth session, so logout clears the cart, or the user id in the workflow state, so it is not authenticated. Separate them with different stores and lifetimes.

### Logout That Only Clears The Client Cookie

Server-side session left valid after "logout," so a captured token still works. Invalidate server-side; clear client-side; rotate keys on compromise.

## Self-Check

- [ ] The location of session state (client-side signed, server-side shared store, or sticky local) was chosen deliberately based on revocation needs, state size, and scale, and sticky routing is not the primary strategy.
- [ ] Every session id or token from the client is authenticated on every request (signature verified, expiry checked, revocation list or store consulted), and no client-supplied state is trusted without cryptographic integrity protection.
- [ ] Revocation requirements are matched to the mechanism: stateless tokens are short-lived or backed by a revocation list; server-side sessions support instant invalidation; no claim of "revocable JWTs" without a server-side list.
- [ ] Session fixation is prevented (id regenerated on login and privilege change, no session ids from URLs), CSRF is prevented (anti-CSRF tokens on every state-changing endpoint, `SameSite` cookies, origin checks), and XSS-state-theft is prevented (cookies `HttpOnly` + `Secure`, tokens not in local storage).
- [ ] Session state is consistent across replicas via a shared store, or the staleness window of eventual consistency is documented with its security consequences (delayed logout, delayed role change).
- [ ] All session state has size bounds (capped lists, trimmed history) and lifetime bounds (idle + absolute timeouts chosen for the data sensitivity), with a refresh flow for long-lived sessions.
- [ ] Authentication state and workflow/application state are separated into different stores with different lifetimes, so logout clears auth without clearing workflow and user identity is always authenticated.
- [ ] Logout invalidates server-side state (not just the client cookie), token rotation (if used) has a documented window and concurrency plan, and compromise triggers key rotation / full session invalidation.
- [ ] The threat model (token theft, session fixation, CSRF, XSS, failover) was written down and each defense was tested, not just implemented by default.
