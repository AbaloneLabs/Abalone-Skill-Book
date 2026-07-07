---
name: authentication_flow_and_oauth_design.md
description: Use when the agent is implementing a login or sign-in flow, building OAuth 2.0 or OpenID Connect integration (authorization code flow, PKCE, client credentials, device flow, refresh tokens), designing a single sign-on (SSO) or social login feature, handling OAuth callbacks and token exchange, choosing between implicit and authorization-code-with-PKCE flows, implementing a redirect-based federation, designing token storage on the client, or reviewing a login flow for session fixation, open redirect, token leakage, or account-takeover risks. Covers OAuth/OIDC flow selection, PKCE and state/nonce, redirect URI validation, token handling and refresh, and the security pitfalls specific to delegated authorization.
---

# Authentication Flow And OAuth Design

Delegated authentication — where your application trusts an identity provider to assert who a user is, or where a user grants your application access to their data on another service — is one of the most defect-prone areas of web security. The protocols (OAuth 2.0, OpenID Connect, SAML, custom redirect-based federation) involve redirects across origins, tokens passed in URLs or headers, state maintained between request and callback, and trust relationships between parties who never directly share a secret. Each of these is a place where a subtle implementation error becomes an account-takeover: an open redirect that lets an attacker capture the authorization code, a missing state check that enables a login CSRF, a token stored where JavaScript can exfiltrate it, a callback validation loose enough to accept an attacker-controlled URI. The protocols are specified carefully precisely because the intuitive implementation is usually wrong.

Agents tend to implement authentication flows by following a tutorial or copying a library's quickstart, which produces a flow that works for the happy path and is broken for the security path. The judgment problem is recognizing that an authentication flow is a chain of trust assertions, each link of which must be verified: the redirect that initiates the flow must be integrity-protected, the callback that completes it must be validated against what was initiated, the tokens that result must be stored and used without leaking, and the refresh path must not create a persistent takeover vector. This skill covers the design and review of redirect-based authentication and delegated authorization flows, focusing on the decisions and validations that distinguish a secure flow from one that works until it is attacked.

## Core Rules

### Choose The Correct Flow For The Client Type

OAuth and OIDC define multiple flows because different clients have different threat models, and using the wrong flow exposes tokens to theft. The flow must match where the client runs and what it can keep secret.

- **Authorization Code flow with PKCE is the default for all clients, including server-side.** PKCE (Proof Key for Code Exchange) binds the authorization request to the client that initiated it, so an intercepted authorization code cannot be redeemed by an attacker. It adds no cost and closes the interception risk; use it everywhere, not only for SPAs and mobile apps.
- **Avoid the Implicit flow.** The Implicit flow returns tokens directly in the URL fragment and provides no client authentication and no PKCE binding. It exists for pre-PKCE browsers and is deprecated for these reasons. Use Authorization Code with PKCE instead.
- **Client Credentials flow is for machine-to-machine, not users.** It authenticates a confidential client (service) with its own credentials and returns a token for that service's own access — it does not involve a user. Never use it to impersonate a user.
- **Device Authorization flow is for input-constrained devices** (TVs, CLI tools) that cannot open a browser securely. It has the user authorize on a separate device via a code. Do not adapt it for web or mobile clients.
- **Refresh tokens must be protected by client confidentiality where applicable.** A confidential client (server-side) can hold a refresh token safely; a public client (SPA, mobile) storing a refresh token in accessible storage creates a persistent takeover vector if that storage is read.

### Protect The Authorization Request With State And Nonce

The redirect from your application to the identity provider, and the callback back, cross origins and carry attacker-influencable parameters. Two bindings protect the round-trip, and omitting either is a known vulnerability class.

- **Use the `state` parameter to bind the callback to the initiating request and prevent login CSRF.** Generate a cryptographically random `state` before redirecting, store it in the user's session, and verify on callback that the returned `state` matches. Without it, an attacker can initiate a login as themselves and trick the victim's browser into completing it, logging the victim into the attacker's account.
- **Use the `nonce` parameter in OIDC to bind the ID token to the authentication.** Generate a random `nonce`, include it in the request, and verify the `nonce` claim in the returned ID token matches. This prevents token replay and injection.
- **Generate `state` and `nonce` with a CSPRNG and compare them correctly.** They must be unguessable (sufficient entropy) and compared with constant-time comparison, like any secret binding.

### Validate The Redirect URI And Callback Strictly

The redirect URI is where the authorization code or token is sent after the user consents. An attacker who can control or influence the redirect URI can capture the code or token. Validation must be exact, not lenient.

- **Register exact, allow-listed redirect URIs.** Match the full URI (scheme, host, path, and decide explicitly on query) against a registered allow-list. Do not use substring or prefix matching, which an attacker can abuse (`https://app.example.com/callback` is not the same as `https://app.example.com/callback/../../evil.com` or `https://app.example.com/callback.evil.com`).
- **Do not allow open redirects in the post-login path.** After authentication, redirecting to a user-supplied `next` or `return_to` parameter without validation lets an attacker phish users to a look-alike site immediately after login. Validate return URLs against an allow-list of trusted paths, or ignore them.
- **Prevent redirect URI manipulation at the provider.** If you are the provider, enforce that the client's registered redirect URIs are exact and that wildcard or path-traversal patterns are rejected.

### Handle Tokens Without Leaking Them

Tokens (access, refresh, ID) are bearer credentials — whoever holds them can act as the user until they expire or are revoked. Where they are stored and how they are transmitted determines whether a script, a log, or a referrer header can steal them.

- **Do not put tokens in the URL.** URLs are logged by proxies, browsers, and servers, and leak via the Referer header to any third-party resource on the page. The Implicit flow's use of the URL fragment is one reason it is deprecated. Pass tokens in headers (`Authorization: Bearer`) or secure cookies.
- **Store tokens where the client's threat model allows.** A server-side app stores tokens server-side. A browser SPA faces a choice: in-memory (lost on refresh, but safe from persistent XSS theft), or in a secure, HttpOnly, SameSite cookie (protected from JS, but vulnerable to CSRF if not paired with CSRF protection). Storing tokens in `localStorage`/`sessionStorage` makes them readable by any XSS on the page — acceptable only with strong CSP and XSS hardening, and never for high-value tokens.
- **Use short-lived access tokens and protected refresh tokens.** An access token should expire quickly (minutes to an hour); a refresh token, protected by client confidentiality or secure storage, obtains new access tokens. This bounds the damage of a stolen access token.
- **Revoke tokens on logout and on suspicion.** Call the provider's revocation endpoint on logout so the token is invalid even if captured. Design for revocation; a system that cannot revoke a refresh token cannot respond to a compromise.

### Verify The Tokens You Receive

When your application receives a token (as a resource server, or on callback), it must verify the token's integrity and claims before trusting it. Trusting an unverified token is trusting an assertion from anyone.

- **Verify signatures with the correct keys.** For JWTs, verify the signature with the provider's published keys (JWKS), validate the `iss`, `aud` (your client/resource), `exp`, `nbf`, and `alg` (reject `none` and unexpected algorithms). Fetch keys from a trusted, pinned source and cache with refresh.
- **Validate the audience and issuer.** A token issued for client A must not be accepted by client B. The `aud` claim must match your resource; the `iss` must match the expected provider. Cross-client token confusion is a real attack.
- **Reject unsigned or algorithm-confused tokens.** The `alg: none` attack and algorithm-confusion (RS256 token verified as HS256 with the public key as the HMAC secret) exploit libraries that accept the token's claimed algorithm. Pin the expected algorithm and reject mismatches.

### Treat Federation And Account Linking As High Risk

When users can sign in via multiple providers, or when an existing account can be linked to a new identity, the mapping between external identity and internal account is a frequent takeover vector.

- **Do not auto-link a federated identity to an existing account by email alone.** If an attacker controls an email at a provider you trust less (or a provider that does not verify email), they can link to a victim's account. Require explicit confirmation, or link only at login of an already-authenticated user.
- **Record which provider asserted the identity and at what trust level.** An email verified by a strong provider is not equivalent to an unverified email at a weak one. Treat the identity's provenance as part of the account's security state.
- **Handle account deletion and de-provisioning across providers.** When a user leaves an organization or revokes access, ensure the federated identity no longer grants access, and that linked tokens are revoked.

## Common Traps

### Using The Implicit Flow Or Omitting PKCE

Returning tokens in the URL fragment (Implicit) or omitting PKCE in the Authorization Code flow, allowing interception of the authorization code or token. Use Authorization Code with PKCE for all clients.

### Missing Or Unverified State Parameter

Omitting the `state` binding, or not verifying it on callback, enabling login CSRF where the victim is logged into the attacker's account. Generate, store, and verify `state` for every flow.

### Open Redirect In The Post-Login Path

Redirecting to an unvalidated `return_to` or `next` parameter after login, sending users to an attacker's phishing site immediately after authenticating. Validate return URLs against an allow-list.

### Lenient Redirect URI Matching

Matching redirect URIs by prefix or substring, allowing an attacker to register or craft a URI that captures the authorization code. Match exact, registered URIs.

### Tokens In URLs Or Accessible Storage

Passing tokens in the URL (logged, leaked via Referer) or storing them in `localStorage` where XSS can read them. Use headers or secure cookies; store refresh tokens only where the client threat model permits.

### Trusting Unverified Tokens

Accepting a JWT without verifying signature, audience, issuer, expiry, or algorithm — including the `alg: none` and algorithm-confusion attacks. Verify all claims and pin the expected algorithm.

### Auto-Linking Federated Identities By Email

Linking a federated login to an existing account based solely on a matching email, allowing takeover via a provider that does not verify email or that the attacker controls. Require explicit linking by an authenticated user.

### Refresh Tokens Without Revocation Path

Issuing long-lived refresh tokens with no revocation mechanism, so a stolen refresh token grants access indefinitely. Use short-lived access tokens, protected refresh tokens, and design for revocation.

## Self-Check

- [ ] The flow matches the client type — Authorization Code with PKCE for all clients (including server-side), Client Credentials only for machine-to-machine, Device flow only for input-constrained devices — and the deprecated Implicit flow is not used.
- [ ] The `state` parameter binds the callback to the initiating request (preventing login CSRF) and, for OIDC, the `nonce` binds the ID token to the authentication; both are CSPRNG-generated and constant-time compared.
- [ ] Redirect URIs are validated by exact match against a registered allow-list (full scheme/host/path, no prefix or substring matching), and post-login return URLs are validated against a trusted allow-list to prevent open redirect.
- [ ] Tokens are transmitted in headers or secure cookies, never in URLs; tokens are stored according to the client threat model (server-side for confidential clients, in-memory or HttpOnly SameSite cookies for browsers, never in `localStorage` for high-value tokens), and access tokens are short-lived with protected refresh tokens.
- [ ] Received tokens are verified: signature against the provider's published keys, with `iss`, `aud`, `exp`, `nbf`, and `alg` claims validated, the expected algorithm pinned (rejecting `none` and algorithm confusion), and keys fetched from a trusted source with refresh.
- [ ] Federated identity linking does not auto-link by email alone; linking requires an authenticated user's explicit action, the asserting provider and email-verification status are recorded, and de-provisioning revokes access and linked tokens.
- [ ] Logout and suspicion trigger token revocation via the provider's revocation endpoint, and the system can revoke refresh tokens to respond to a compromise.
- [ ] The flow has been reviewed for the specific attack classes: login CSRF (state), code/token interception (PKCE), open redirect (return URL validation), token leakage (storage and transport), token confusion (audience/issuer/algorithm), and account takeover via federation (linking rules).
