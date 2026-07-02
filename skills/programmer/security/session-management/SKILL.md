---
name: session_management.md
description: Use when the agent is designing, implementing, or reviewing login sessions, session stores, session cookies, JWT or opaque token sessions, session expiry and sliding windows, concurrent-login or single-session policies, logout, session invalidation, idle vs absolute timeouts, remember-me, session fixation, session hijacking defenses, CSRF tokens bound to sessions, or any state that ties an authenticated identity to a sequence of requests. Also covers server-side session storage choice (in-memory, Redis, database), session rotation after login or privilege change, forced logout, revocation under compromise, and the operational consequences of session lifetime on security and UX.
---

# Session Management

A session is the durable proof that an identity was authenticated at some point in the past, trusted on every subsequent request without re-authentication. That trust is the entire value of a session, and also its entire risk: anyone who obtains the session token obtains the user's authority until the session ends. Most account-takeover incidents are not broken authentication; they are broken session management — a token that never expires, a session that survives logout, a cookie that leaks over HTTP, an id that is reused across privilege changes, or a logout that leaves the token valid on five other devices.

Agents tend to under-invest here because the framework's session middleware "just works" for the happy path. Login returns a cookie, the cookie is attached to subsequent requests, logout deletes the cookie. The defects live in the cases the framework does not decide for you: how long the token should live, whether a new login should invalidate the old one, whether logout on one device kills the others, what happens to active sessions when a password is changed, how a stolen token is revoked before it expires, and how a privilege change (user becomes admin, tenant switch, post-login step-up) is reflected in a token issued before the change. These are policy decisions the framework cannot make, and getting them wrong turns a stolen cookie into a permanent account takeover.

The judgment problem is treating the session as a security object with an explicit lifecycle — created, bound, rotated, expired, revoked — rather than as a cookie you set and forget. Every property of that lifecycle (lifetime, scope, storage, rotation, revocation) is a deliberate decision with a security and usability tradeoff.

## Core Rules

### Choose The Session Token Format By Its Revocation Properties

The token format determines what you can do when things go wrong. The two families have opposite revocation profiles:

- **Opaque, server-side-stateful tokens** (a random unguessable id pointing to a server-side record). The server holds the session record; the token is meaningless without it. Revocation is trivial — delete or mark the record. Logout is immediate. Forced logout works. The cost is a session store lookup on every request and a store to operate.
- **Self-contained tokens (JWT signed, sometimes encrypted).** The token carries its own claims and is verified by signature, with no server lookup. This is fast and stateless, but a signed token cannot be revoked before its stated expiry without additional state — by design, verification does not require the issuer to be reachable. "Logout" on a pure stateless JWT only removes the client copy; the token remains valid until it expires.

The recurring mistake is choosing stateless JWTs for their performance and then needing revocation (logout, compromise, role change), which the format does not natively support. If you need real revocation, you need server-side state — either a stateful token, or a JWT plus a server-side blocklist / version check, which gives back the lookup cost the JWT was meant to avoid. Decide the revocation requirement first, then the format.

### Set Lifetimes That Match The Sensitivity And The Threat

Session lifetime is a tradeoff between security (shorter is better — a stolen token expires sooner) and usability (longer is better — users are not re-logged-in constantly). The right value depends on what the session protects:

- **Idle timeout (inactivity).** How long since the last request before the session ends. Protects against a session left open on an unattended device. Most applications need one; sensitive applications need a short one.
- **Absolute timeout (maximum session age regardless of activity).** Caps the lifetime of a stolen token even if the attacker keeps it active. Often set much longer than the idle timeout, but it must exist — a session that lives forever as long as it is touched is a session that lives forever.
- **Sliding expiration (renew on activity).** Extends the idle window on each request. Convenient, but combined with no absolute timeout it produces effectively-immortal sessions.

The trap is the "remember me" checkbox that silently disables all timeouts. Remember-me should extend the absolute lifetime, not remove it, and it should not apply to high-sensitivity contexts (payment, admin, security settings) where re-authentication is the point. Define both timeouts explicitly and document which actions reset which clock.

### Rotate The Session Identifier After Any Privilege Change

Session fixation is the attack where an attacker sets or predicts the victim's session id before login, and the application accepts that same id after login — so the attacker, who knows the id, now has an authenticated session. The defense is mechanical: regenerate the session id on every privilege transition.

Privilege transitions that require rotation include:

- **After successful login.** Always issue a fresh id; never reuse the pre-login (anonymous) session id for the authenticated session.
- **After step-up / re-authentication.** A user who re-enters a password for a sensitive action should get a new token scoped to the elevated context, not the old token with a flag flipped.
- **After a role or tenant change.** If a user becomes an admin or switches organization, the token must reflect the new authority; reusing the old token risks serving stale lower-privilege or wrong-tenant data.
- **After password change or credential reset.** Existing sessions elsewhere should be reconsidered — often invalidated — because a password change is frequently a response to suspected compromise.

A token whose authority can change without the token itself changing is a token that can be confused. Rotate on transition, and encode the authority (or a version of it) in the token or its server-side record.

### Decide And Enforce A Concurrent-Session Policy

"How many sessions can one user have at once, and what happens when a new login occurs?" is a policy question the framework will not answer for you. The options:

- **Unlimited concurrent sessions.** Each new login adds a session; old ones remain. Most convenient, weakest against compromise — a stolen session is never invalidated by the user logging in elsewhere.
- **Single active session (new login kills the old).** Strong against a forgotten or stolen session on another device, but hostile to users with multiple devices and breaks workflows that rely on staying logged in.
- **Bounded number with explicit device management.** The user can see and revoke their active sessions (a security settings page listing devices). This is the strongest usable design — it gives the user control and visibility.

Whatever the policy, it requires server-side state: a registry of active sessions per user, consulted on login and on each request. A pure stateless token cannot enforce a concurrent-session limit, because no central authority knows which tokens exist. If concurrent-session control matters, you need state.

### Make Logout And Revocation Actually Work Everywhere

Logout is where session designs most often fail silently. "Deleting the cookie on the client" is not logout; the token may still be valid on the server, on other devices, in a CDN cache, or in an attacker's possession. Real logout and revocation require:

- **Server-side invalidation.** Mark the session record as ended (stateful) or add the token to a blocklist until its expiry (stateless JWT). The token must fail verification after logout, not merely be absent from the browser.
- **Global logout / "log out everywhere".** A user who suspects compromise needs to end all their sessions at once. This requires iterating the user's session registry and invalidating each — impossible without server-side state.
- **Logout under compromise.** When a token is known stolen, you need a way to revoke it (and ideally all tokens of that user, version, or signing key) faster than its expiry. Key rotation (issuing under a new signing key) is the brute-force revocation for stateless tokens, but it logs out every user, not just the victim.
- **Cookie clearing on logout.** Clear the cookie with the same scope (Domain, Path) it was set with, or the browser keeps a stale copy. A logout that leaves the cookie set produces confusing half-logged-in states.

Test logout by capturing the token before logout and replaying it after. If it still works, logout did not happen on the server.

### Bind CSRF Protection To The Session

A session cookie is automatically attached to cross-site requests, which is the basis of CSRF. The defenses (covered in depth in the client-side attack skill) must be wired to the session: the synchronizer token is generated per session and verified per state-changing request; SameSite is set on the session cookie; and high-stakes actions may require step-up re-authentication that issues a fresh, narrowly-scoped token. The session-management decision is to treat every state-changing endpoint that relies on the session cookie as needing an explicit CSRF defense, and to regenerate CSRF tokens on session rotation so a fixed session does not carry a fixed, predictable token.

### Store Sessions Safely And Operate The Store

Where the session lives at rest is a security decision. A session store compromise is a mass account-takeover, because every record is a live credential.

- **Do not store secrets or sensitive data in the session record beyond what is needed.** The record needs the user id, expiry, and minimal context — not the user's password hash, API keys, or PII pulled in "for convenience."
- **Protect the store like a credential store.** Redis, the database session table, or the in-memory session process must have access control, encryption at rest, and network isolation consistent with the value of what it holds.
- **Encrypt or sign session cookies if they carry state.** If the cookie itself contains session data (not just an id), it must be signed (integrity) and, if the data is sensitive, encrypted (confidentiality). A unsigned cookie with state is tamperable; a signed-but-not-encrypted cookie leaks its contents to the user and to anyone who reads the cookie.
- **Plan for store loss.** If the session store is lost (Redis flush, process restart with in-memory sessions), every user is logged out at once. Decide whether that is acceptable, and whether sessions should survive a store restart (persistent store) or not (acceptable mass re-login).

## Common Traps

### Choosing Stateless JWTs For Performance, Then Needing Revocation

Picking JWTs to avoid a session-store lookup, then discovering logout, forced logout, role change, and compromise response all require server-side state — and bolting on a blocklist that reintroduces the lookup the JWT was meant to eliminate. Decide the revocation requirement first; if you need it, use a stateful token or accept the blocklist cost.

### Session That Never Expires

A session with no absolute timeout, only a sliding window that resets on activity — so a stolen token that is used regularly lives forever. Always set an absolute maximum age, even for "remember me," and treat immortal sessions as a defect.

### Logout That Only Clears The Client Cookie

Implementing logout as "delete the cookie in the response," leaving the token valid on the server. Capturing the token before logout and replaying it succeeds. Logout must invalidate the server-side record or blocklist the token.

### Reusing The Session Id Across Login Or Privilege Change

Accepting the anonymous session id as the authenticated session id after login (session fixation), or keeping the same token after a user becomes an admin or switches tenant. Rotate the id on every privilege transition; encode authority so a stale token cannot grant the new privilege.

### Remember-Me That Disables All Timeouts

Treating "remember me" as "this session never expires." Remember-me should extend the absolute lifetime, not remove it, and should not bypass step-up re-authentication for sensitive actions.

### Storing Sensitive Data In The Session For Convenience

Putting the password hash, API keys, or PII into the session record or cookie so handlers can read it without a lookup. The session is a credential-equivalent artifact; minimize what it holds, and never store a secret that would let a session-store compromise become a broader breach.

### No Way To Revoke A Stolen Token Before Expiry

Issuing long-lived tokens with no revocation path, so a reported stolen token remains valid until it expires — which may be days. Have a revocation mechanism (record invalidation, blocklist, or key rotation) and know its blast radius (one session, one user, or everyone).

### Concurrent Sessions That Cannot Be Listed Or Revoked

Allowing unlimited sessions per user with no registry, so a user who suspects compromise has no way to see or end their other sessions. Maintain a per-user session registry and expose a "devices / active sessions" UI for user-initiated revocation.

## Self-Check

- [ ] The token format was chosen for its revocation properties: if logout, forced logout, or compromise revocation is required, the design uses server-side state (stateful token, or JWT plus blocklist/version) rather than pure stateless tokens that cannot be revoked.
- [ ] Both an idle timeout and an absolute timeout are defined; sliding expiration does not produce effectively-immortal sessions; "remember me" extends rather than removes the absolute lifetime and does not bypass step-up auth.
- [ ] The session id is regenerated on every privilege transition (login, step-up, role or tenant change, password change), and authority is encoded so a stale token cannot grant the new privilege.
- [ ] A concurrent-session policy is decided and enforced with server-side state; if single-session or bounded, new logins behave as designed; if unlimited, users can list and revoke their sessions.
- [ ] Logout invalidates the server-side record or blocklists the token (verified by replaying a captured token after logout), supports "log out everywhere," and clears the cookie with the correct scope.
- [ ] A revocation path exists for a stolen token before its natural expiry, and its blast radius (one session / one user / all users via key rotation) is known.
- [ ] Every state-changing endpoint relying on the session cookie has a CSRF defense wired to the session, and CSRF tokens are regenerated on session rotation.
- [ ] The session store is treated as a credential store (access control, encryption at rest, network isolation), stores only the minimal necessary data, and stateful cookies are signed and encrypted where they carry sensitive state.
- [ ] The consequence of session-store loss (mass logout) is understood and either accepted or mitigated with a persistent store.
