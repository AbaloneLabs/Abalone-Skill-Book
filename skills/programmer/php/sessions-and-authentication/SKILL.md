---
name: php_sessions_and_authentication.md
description: Use when the agent is implementing authentication or sessions in PHP (login/logout, password hashing with password_hash/password_verify, session management with $_SESSION/session_start/session_regenerate_id, "remember me" cookies, CSRF tokens, role-based or attribute-based authorization, JWT or OAuth/OAuth2, API token authentication), handling session fixation or hijacking, storing session data server-side vs client-side, managing session expiration and concurrency, or is diagnosing "sessions lost after login", "session fixation", "passwords stored in plaintext or with MD5", "CSRF token missing or not verified", "session data shared across tabs", or authentication/authorization confusion. Covers PHP session mechanics and regeneration, password hashing, CSRF, remember-me, token vs session auth, authorization-vs-authentication, and the security pitfalls of fixation, hijacking, and insecure storage.
---

# Sessions And Authentication In PHP

Authentication in PHP is a security-critical area where common shortcuts create exploitable vulnerabilities. Passwords stored with weak hashes (MD5/SHA1, or unsalted) are trivially crackable; sessions started without regenerating the ID on login are vulnerable to session fixation; "remember me" cookies that store user identifiers in plaintext enable trivial impersonation; CSRF tokens omitted or unverified allow cross-site state changes; and the conflation of authentication (who you are) with authorization (what you may do) leads to broken access control. The judgment problem is to use `password_hash`/`password_verify` (never hand-roll hashing), to manage sessions with `session_regenerate_id(true)` at privilege change, to implement CSRF protection on every state-changing request, to design "remember me" with random tokens stored hashed server-side, to choose session-based vs token-based auth appropriately, and to separate authentication from authorization.

Agents store passwords with MD5, forget `session_regenerate_id` on login, leave CSRF unverified, store "remember me" as a plaintext user ID, or conflate auth with authz. The remedy is to apply the standard secure primitives and to reason about fixation, hijacking, and access control deliberately.

## Core Rules

### Hash Passwords With password_hash / password_verify, Never Hand-Roll

Use `password_hash($pw, PASSWORD_DEFAULT)` (bcrypt/argon2, with salt generated internally) and verify with `password_verify($pw, $hash)`. Never use MD5, SHA1, unsalted hashes, or custom schemes — they are crackable in seconds. Store only the hash in the database. To upgrade legacy hashes, re-hash on successful login (if the hash needs rehashing per `password_needs_rehash`, hash with the current algorithm and update the row). Set appropriate cost/limits (the default is sane; raise only with benchmarking). Do not roll your own hashing, encryption, or token generation.

- `password_hash($pw, PASSWORD_DEFAULT)` + `password_verify`; never MD5/SHA1/custom.
- Re-hash legacy hashes on successful login via `password_needs_rehash`.
- Do not invent hashing, encryption, or token schemes; use the standard primitives.

### Regenerate The Session ID On Privilege Change (Login/Logout/Elevation)

Session fixation attacks work by making the victim use an attacker-known session ID; logging in without changing the session ID means the attacker's ID is now authenticated as the victim. Prevent this by calling `session_regenerate_id(true)` immediately after a successful login (and on any privilege elevation, and on logout destroy the session). The `true` deletes the old session file. Start sessions with secure settings: `session.cookie_httponly=1` (JS cannot read the cookie), `session.cookie_samesite="Lax"` or `"Strict"` (CSRF mitigation), `session.cookie_secure=1` (HTTPS only), and prefer the cookie over URL passing (`session.use_only_cookies=1`).

- `session_regenerate_id(true)` on login, privilege change, and logout (destroy on logout).
- Set `cookie_httponly`, `cookie_samesite` (Lax/Strict), `cookie_secure` (HTTPS), `use_only_cookies`.
- Fixation is prevented by changing the ID when the privilege level changes.

### Protect Every State-Changing Request With CSRF Tokens

Any request that changes server state (POST/PUT/DELETE, or GET with side effects) must carry a CSRF token that is verified server-side, because a logged-in user's browser will attach session cookies to cross-site requests automatically. Generate a random token per session (or per form), store it in the session, embed it in the form (hidden input) or header, and verify it on submission with constant-time comparison. Use `SameSite=Lax|Strict` cookies as defense-in-depth (they block many cross-site requests) but not as the sole defense. GET must never change state (a `<img src="...">` could trigger it).

- Every state-changing request carries and verifies a CSRF token (constant-time compare).
- Generate per-session/per-form tokens; store in session, embed in form/header.
- `SameSite` cookies are defense-in-depth, not a replacement for tokens; GET must not mutate state.

### Implement "Remember Me" With Random Tokens Stored Hashed Server-Side

A secure "remember me" stores a random, high-entropy token in a cookie; the server stores a *hash* of the token (never the plaintext) keyed to the user, and on subsequent requests hashes the cookie and looks it up. On use, rotate the token (issue a new one, invalidate the old) to limit replay. Never store a plaintext user ID or a signed-but-replayable token in the cookie — an attacker who reads the cookie (XSS, leak) gets permanent access. Treat the remember-me token like a password: hashed, single-use-rotating, invalidated on logout.

- Cookie holds a random high-entropy token; server stores a *hash* of it (not plaintext).
- Rotate the token on each use; invalidate on logout.
- Never store a plaintext user ID or a replayable signed token.

### Choose Session-Based vs Token-Based Auth By Context

Session-based auth (cookie + server-side session store) suits browser apps with a same-origin trust model and server-rendered or same-origin APIs; the cookie is sent automatically and CSRF protection applies. Token-based auth (Authorization: Bearer <token>, often JWT or opaque tokens in a store) suits APIs consumed by third parties, mobile apps, or cross-origin SPAs where cookies are awkward; tokens are sent explicitly by the client and are not subject to CSRF (no automatic sending) but must be stored safely client-side (not localStorage if XSS risk) and revoked server-side if opaque (JWTs are stateless and hard to revoke). Match the mechanism to the client and trust model; do not default to JWT for a same-origin browser app (sessions are simpler and revocable).

- Sessions (cookie + server store) for same-origin browser apps; CSRF applies, revocation is easy.
- Tokens (Bearer) for APIs/mobile/cross-origin; not CSRF-subject, but must be stored safely and (for opaque) revocable.
- Do not default to JWT for same-origin browser apps; sessions are simpler and revocable.

### Separate Authentication From Authorization

Authentication establishes *who* the user is; authorization decides *what* they may do. Conflating them ("logged in → can access everything") produces broken access control. After authenticating, enforce authorization explicitly per action/resource: role-based (RBAC) or attribute/claim-based checks, verified on the server for every request (never trust client-side gating). Check the user owns the resource they are modifying (`WHERE owner_id = ?`), not just that they are logged in. Log authorization failures. The most common vulnerability (OWASP #1 broken access control) comes from missing server-side authorization checks.

- Authentication = who; authorization = what. Enforce authorization explicitly per action/resource.
- Verify ownership (`owner_id = current user`) server-side, not just login status.
- Never rely on client-side gating or hidden UI; every request is authorized server-side.

## Common Traps

### MD5/SHA1 Or Unsalted Password Hashes

Trivially crackable. Use `password_hash`/`password_verify`; re-hash legacy on login.

### Missing session_regenerate_id On Login

Session fixation: the attacker's session ID becomes authenticated. Regenerate (delete old) on login/privilege change/logout.

### CSRF Token Missing Or Unverified

State-changing requests without a verified token allow cross-site attacks. Verify on every mutation; GET must not mutate.

### Plaintext User ID In "Remember Me" Cookie

Trivial impersonation on cookie leak. Use random tokens stored hashed, rotated on use.

### Defaulting To JWT For A Same-Origin Browser App

JWTs are stateless and hard to revoke; sessions are simpler for same-origin. Match the mechanism to the client/trust model.

### Conflating Authentication With Authorization

"Logged in" ≠ "authorized". Enforce per-action/resource authorization server-side; verify ownership.

### Storing Tokens In localStorage Under XSS Risk

An XSS exfiltrates the token. Prefer HttpOnly cookies for sessions, or short-lived tokens with refresh.

### SameSite=None Without Secure

A cross-site cookie without `Secure` is dropped by browsers. Pair `SameSite=None` with `Secure` over HTTPS.

## Self-Check

- [ ] Passwords are hashed with `password_hash`/`PASSWORD_DEFAULT` and verified with `password_verify`; no MD5/SHA1/custom hashing exists, and legacy hashes are re-hashed on login.
- [ ] `session_regenerate_id(true)` is called on login, privilege elevation, and logout; session cookies use `httponly`, `samesite`, and `secure`.
- [ ] Every state-changing request (POST/PUT/DELETE) carries a CSRF token verified server-side with constant-time comparison; GET does not mutate state.
- [ ] "Remember me" uses random high-entropy tokens stored *hashed* server-side, rotated on use and invalidated on logout (no plaintext user IDs or replayable signed tokens in cookies).
- [ ] The auth mechanism matches the context (sessions for same-origin browser apps; Bearer tokens for APIs/mobile/cross-origin), and tokens are stored safely (not localStorage under XSS risk).
- [ ] Authorization is enforced explicitly per action/resource server-side (RBAC/ownership checks), independent of authentication; client-side gating is not trusted.
- [ ] Authorization failures are logged; session/token expiration and concurrency (single-device vs multi-session) are considered.
- [ ] The implementation has been considered under fixation, hijacking, CSRF, token leak, and broken access control, and remains secure.
