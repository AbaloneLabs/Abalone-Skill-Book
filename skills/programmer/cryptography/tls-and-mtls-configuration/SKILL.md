---
name: tls_and_mtls_configuration.md
description: Use when the agent is configuring TLS termination, choosing TLS versions or cipher suites, managing server or client certificates, enabling mutual TLS (mTLS) for service-to-service authentication, setting up certificate pinning in a mobile or client app, validating certificates and certificate chains, handling OCSP or CRL revocation checks, planning certificate rotation and renewal, hardening against downgrade or protocol-interception attacks, designing an internal PKI, or reviewing an HTTPS or mTLS setup for correctness and security. Also covers HSTS interaction, SNI, trust store management, the failure modes of self-signed certificates, disabled verification, and the difference between encrypted and authenticated channels.
---

# TLS And mTLS Configuration

TLS is the mechanism that turns an untrusted network into an authenticated, confidential, integrity-protected channel — but only when it is configured correctly. A TLS connection that "establishes successfully" is not the same as a secure connection. The most damaging TLS defects do not break the handshake; they quietly weaken it: a verification callback that returns true on every certificate, a cipher suite list that still permits a legacy mode, a client that connects without checking the hostname, a private key with no rotation plan, or an mTLS deployment where only the server is authenticated. The channel is encrypted, the dashboards are green, and an active attacker is reading or forging traffic anyway.

Agents tend to under-invest here because modern TLS defaults are strong and most libraries negotiate a good connection automatically. The defects live in the configuration the defaults do not cover: which versions and ciphers are permitted, whether the peer's certificate is actually validated against the right trust anchor and name, how certificates are rotated before they expire, how revocation is handled, and whether both directions of a connection are authenticated. The judgment problem is treating TLS as an explicit, reviewable security configuration — versions, ciphers, certificates, trust anchors, verification behavior, and lifecycle — rather than as a checkbox that HTTPS provides. Encryption without authentication is a wiretap waiting to happen; authentication without verification is theater.

This skill is about configuring TLS and mTLS correctly. It complements the encryption skill (which covers the underlying primitives) and the signing skill (which covers certificate and chain validation as a verification problem). Here the question is how to establish channels that are confidential, authenticated, and resilient to downgrade, interception, and expired or revoked credentials.

## Core Rules

### Never Disable Certificate Verification In Production

The single most common and most dangerous TLS defect is disabling verification. A connection configured to accept any certificate is encrypted against passive eavesdropping but offers no protection against an active man-in-the-middle, who presents their own certificate and relays traffic. This defect is invisible: the connection succeeds, the data flows, and an attacker reads everything.

- **Never set verification to "accept all" or implement a callback that returns true unconditionally** in production code. This pattern appears in tutorials and debugging snippets; it must not reach production.
- **Validate the full chain against a trusted root store**, and confirm the leaf certificate is valid for the hostname you connected to (hostname/SAN verification). A valid chain for the wrong name is not authentication.
- **Treat "I disabled it to debug and forgot to re-enable" as a likely defect.** Make verification the default and require an explicit, logged, reviewed override to disable it — and only in development.
- **Verify the certificate on the client side too.** A client that connects to a server without verifying the server certificate has no authentication of that server.

If any production connection skips verification, the channel provides confidentiality against passive observers only, and an active attacker controls it. Audit every TLS client and server for verification behavior, not just for "does it use TLS."

### Pin TLS Versions And Reject Legacy Protocols

TLS version negotiation allows the client and server to agree on the highest mutually supported version, but it also enables downgrade attacks if an attacker can force a fallback. The defense is to pin the minimum acceptable version and reject everything below it.

- **Disable SSLv3, TLS 1.0, and TLS 1.1.** These have known weaknesses (POODLE, BEAST-style issues, weak MACs) and are deprecated. They should not be offered.
- **Prefer TLS 1.3.** It removes vulnerable legacy features, mandates forward secrecy, encrypts more of the handshake, and is faster. TLS 1.2 is acceptable as a fallback only with a hardened cipher list.
- **Set a hard minimum, not a preference.** A configuration that "prefers TLS 1.3" but accepts TLS 1.0 still allows a downgrade. The minimum version must be enforced and non-negotiable.
- **Beware downgrade signals.** Some legacy middleboxes force downgrades; a configuration that silently accommodates them weakens everyone. Prefer to fail closed on a tampered handshake rather than fall back to a broken version.

The strong configuration offers only modern versions and fails rather than downgrading. The weak configuration leaves old versions enabled "for compatibility" and inherits all their weaknesses.

### Curate The Cipher Suite List; Do Not Trust Defaults Blindly

Cipher suites determine the key exchange, authentication, bulk encryption, and MAC used by a TLS 1.2 connection (TLS 1.3 fixes most of this). The suite list and ordering directly affect security. Defaults are often better than they were, but they are not always correct for your threat model.

- **For TLS 1.2, prefer AEAD suites** (AES-GCM, ChaCha20-Poly1305) and forward-secret key exchange (ECDHE). Remove non-forward-secret and non-AEAD suites.
- **Remove weak algorithms entirely** (RC4, 3DES, CBC modes without robust mitigations, static RSA key exchange, MD5, SHA-1 for handshake integrity). Leaving them "at the bottom of the list" still allows a forced downgrade in misconfigured peers.
- **Order matters.** Put the strongest, most preferred suite first; some clients honor order.
- **For TLS 1.3, the cipher list is smaller and safer by design**, but confirm the configuration does not re-enable legacy behavior through compatibility flags.

A cipher list copied from an outdated tutorial, or left at the library default from years ago, is a latent weakness. Review the actual negotiated suites, not just the configured list.

### Design Certificate Lifecycle Around Expiry, Not Incidents

Certificates expire. An expired certificate is a service outage, and outages caused by certificate expiry are among the most common and most preventable production failures. The lifecycle must be automated, monitored, and tested before expiry — not discovered when the handshake fails.

- **Automate issuance and renewal.** Use ACME (Let's Encrypt and compatible CAs) or an internal PKI with automated enrollment. Manual renewal is renewal that will eventually be missed.
- **Renew well before expiry.** Renew at a fraction of the lifetime (e.g., at 2/3 elapsed), so a renewal failure leaves time to fix it before the cert expires.
- **Monitor expiry and alert on it.** Every certificate in use must have an alert that fires days before expiry, not on the day. Treat "no certificate monitoring" as a defect.
- **Plan key rotation.** When a private key is rotated, the new certificate must be deployed everywhere before the old one is withdrawn. Versioned, overlapping deployment avoids a cutover outage.
- **Test the renewal path.** A renewal automation that has never run end-to-end is unverified. Test it in staging, including the failure modes (CA unavailable, DNS challenge fails).

The recurring incident is a long-lived certificate, renewed manually, with no monitoring, that expires on a weekend. Automation, early renewal, and expiry alerts prevent the entire class.

### Use mTLS When The Client Must Be Authenticated, And Authenticate Both Directions

Mutual TLS authenticates the client to the server in addition to the server to the client, using a client certificate. It is the strongest service-to-service authentication when the parties can distribute and trust certificates. But mTLS is only mTLS if the server actually requires and validates the client certificate.

- **Require the client certificate; do not merely request it.** A server configured to request but not require a client cert still accepts connections that present none.
- **Validate the client certificate against the right trust anchor.** A client cert signed by a public CA is not the same as one signed by your internal PKI; pin the CA(s) you expect for each listener.
- **Bind identity to authorization.** The client certificate's identity (SPIFFE ID, CN, SAN, or a custom extension) must drive authorization decisions, not just be checked for presence.
- **Plan client certificate rotation.** Client certs expire too; rotation must be automated and overlapping, or every mTLS-dependent service breaks at once.
- **Distinguish mTLS from one-way TLS.** One-way TLS authenticates the server only; it does not prove who the client is. Do not assume "we use TLS" means "we know the caller."

mTLS is powerful because authentication happens at the transport layer, before application code runs. But its strength depends entirely on the server enforcing client cert presence and validating it against the correct trust anchor.

### Handle Revocation Deliberately, Knowing Its Limits

A certificate may be valid by signature and expiry but revoked by its issuer (key compromise, CA compromise, cessation of operation). Revocation checking is how a verifier learns a cert is no longer trustworthy — but it is unreliable in practice and must be understood, not assumed.

- **OCSP (Online Certificate Status Protocol)** checks the status of a specific certificate in real time, but it leaks privacy (the CA learns what you are visiting), adds latency, and fails open if the responder is unreachable (soft-fail).
- **CRL (Certificate Revocation List)** is a downloaded list of revoked certs, but it is large, stale, and rarely checked rigorously by clients.
- **OCSP stapling** has the server fetch and attach the OCSP response, fixing privacy and latency, but it requires server configuration and a willing CA.
- **CRLite / CRLSets** (browser-scale) compress revocation data; application-level TLS stacks usually do not have these.
- **Decide a policy and accept its tradeoff.** Hard-fail on revocation unavailability improves security but risks breaking on a flaky OCSP responder; soft-fail preserves availability but accepts revoked certs when the responder is down. For high-value internal PKIs, hard-fail against your own responder is often correct.

Do not assume "the library checks revocation" without confirming the behavior, especially the fail-open default. For internal PKIs, prefer short-lived certificates (so revocation matters less) over relying on revocation infrastructure.

### Use Certificate Pinning Carefully, And Only Where It Applies

Pinning binds a connection to a specific certificate, public key, or CA, refusing all others. It defends against a compromised or coerced CA issuing a fraudulent certificate for your domain. It is high-risk: if the pinned credential rotates and the pin is not updated, every client is locked out.

- **Pin the public key (SPKI), not the certificate.** The public key survives certificate renewal as long as the same key pair is reused; the certificate does not.
- **Always pin a backup key.** Pin at least the current key and a pre-generated backup, so a key loss does not require an emergency rotation that breaks every pinned client.
- **Pinning is for clients you control (mobile apps, thick clients), not for browsers.** Browser-based pinning via static HPKP is deprecated and dangerous (it can permanently lock users out); do not deploy it.
- **Pinning breaks the CA trust model.** A pinned client no longer trusts the public CA infrastructure for that host; this is the point, but it means a pinning mistake is a self-inflicted outage with no CA fallback.

Use pinning for high-value clients where the threat of a fraudulent CA-issued certificate is real and you control the client update path. Avoid it where the operational risk of lockout exceeds the threat.

## Common Traps

### Disabling Verification "Just For Testing" And Shipping It

A verification callback that returns true, or a client configured with `verify = false`, written to debug a certificate issue and never re-enabled. The connection works, and an active attacker controls it. Make verification the default and audit for disabled verification in production.

### Accepting A Valid Certificate For The Wrong Hostname

Validating the chain but skipping hostname/SAN verification, so any valid certificate (including one for an unrelated domain) is accepted. A valid chain proves the cert is genuine; only hostname verification proves it is genuine for the host you intended.

### Leaving Legacy TLS Versions Enabled "For Compatibility"

Offering TLS 1.0 or 1.1 because an old client might need them, allowing a downgrade attack against modern clients. Disable legacy versions; require modern clients or provide a hardened upgrade path.

### Copying An Outdated Cipher List From A Tutorial

A cipher configuration from years ago that still permits RC4, 3DES, or static RSA key exchange. Defaults have improved; a stale explicit list can be weaker than the current default. Review the actual negotiated suites.

### Manual Certificate Renewal With No Monitoring

A long-lived certificate renewed by a human who must remember, with no alert when expiry approaches. It expires on a weekend and takes down the service. Automate renewal, renew early, and alert on expiry.

### mTLS Server That Requests But Does Not Require The Client Cert

Configuring the server to request a client certificate but accept connections that present none, so the "mutual" authentication is optional and an unauthenticated client connects anyway. Require the cert and validate it.

### Soft-Fail Revocation Checking Treated As Real Protection

An OCSP check that fails open when the responder is unreachable, so a revoked certificate is accepted whenever the responder is down. Understand the fail mode; for high-value PKIs, prefer short-lived certs or hard-fail against a reliable responder.

### HPKP Or Static Pinning In A Browser Context and reusing A Private Key Across Renewals Indefinitely

Deploying HTTP Public Key Pinning (deprecated) or static pins that can permanently lock users out of their own browser if the key is lost. Pin only in clients you control and can update, and always with a backup pin.

Renewing the certificate but keeping the same private key forever, so a compromise of that key compromises every period it covered. Rotate keys periodically, with overlapping deployment.

## Self-Check

- [ ] No production TLS client or server disables certificate verification; every connection validates the full chain against a trusted root store and confirms the hostname/SAN matches the intended host.
- [ ] Legacy protocols (SSLv3, TLS 1.0, TLS 1.1) are disabled with a hard minimum version; TLS 1.3 is preferred and TLS 1.2 is accepted only with a hardened cipher list.
- [ ] The cipher suite list contains only AEAD and forward-secret suites for TLS 1.2, with weak algorithms (RC4, 3DES, static RSA, MD5/SHA-1) removed entirely — not merely ranked low.
- [ ] Certificate issuance and renewal are automated (ACME or internal PKI), renewal happens well before expiry, expiry is monitored with alerts that fire days in advance, and the renewal path has been tested end-to-end including failure modes.
- [ ] Where mTLS is used, the server requires (not merely requests) the client certificate, validates it against the correct trust anchor, binds its identity to authorization, and has an automated client-cert rotation plan.
- [ ] Revocation handling is a deliberate decision with a known fail mode (hard-fail vs soft-fail); for internal PKIs, short-lived certificates reduce reliance on revocation rather than depending on unreliable OCSP/CRL.
- [ ] Certificate pinning, if used, pins the public key with a backup, applies only to controllable clients (not browsers via HPKP), and has an update path that prevents lockout on key rotation.
- [ ] Private keys are rotated periodically with overlapping deployment, not reused indefinitely across renewals.
- [ ] The actual negotiated TLS versions and cipher suites were verified against a live connection, not assumed from the configuration text.
