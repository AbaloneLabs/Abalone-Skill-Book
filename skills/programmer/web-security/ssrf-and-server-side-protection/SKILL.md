---
name: ssrf_and_server_side_protection.md
description: Use when the agent is building a feature where the server fetches a URL supplied or influenced by the user — image previews, link unfurling, webhooks, URL importers, PDF or screenshot generators, headless browsers, OAuth callback fetches, feed readers, file fetching, or any outbound HTTP request whose target is attacker-influenced. Covers Server-Side Request Forgery (SSRF), internal endpoint exposure, cloud metadata service abuse (169.254.169.254), DNS rebinding, redirect-following bypasses, IP and hostname allowlisting, restricting outbound traffic via network egress controls, URL scheme and parsing validation, and the failure of naive blocklists. Also use when reviewing code that takes a URL and makes a request, or hardening a service that runs inside a cloud VPC.
---

# SSRF And Server-Side Protection

Server-Side Request Forgery is what happens when the server treats a user-supplied URL as a trusted destination. The feature looks innocent — fetch a preview of a link, import a file from a URL, call a webhook — but the URL is a vector into the server's own private network. From the server's vantage point, that URL can point at localhost, at internal admin endpoints with no authentication, at cloud metadata services that hand out temporary credentials, at databases and message queues that listen on internal ports, or at services that exist only inside the VPC. The defining SSRF failure is not a broken parser; it is the assumption that a URL the application fetches is just another internet address, when from inside the network it is a key to everything the firewall was meant to protect.

Agents tend to under-invest here because the happy path works: the user pastes a public URL, the server fetches it, the preview renders. The defects live in the inputs the happy path never tests: `http://localhost/admin`, `http://169.254.169.254/latest/meta-data/`, `http://[::1]/`, a domain that resolves to an internal IP, a public domain that redirects to an internal address, a URL whose hostname resolves differently on the second DNS lookup. Each is a way to turn a "fetch this URL" feature into internal network access, credential theft, or remote code execution through an internal admin API. The judgment problem is treating every server-initiated request whose target is attacker-influenced as a high-risk operation that must be constrained by network policy and strict validation, not as a routine HTTP call.

This skill is about preventing SSRF and hardening server-side outbound requests. It complements the input validation skill (which covers injection broadly) and the authentication skill (which covers why internal endpoints often lack auth and thus become SSRF targets). Here the question is how to make a server that fetches user-influenced URLs unable to reach the things it must not reach.

## Core Rules

### Treat Every Server-Initiated Request With Attacker-Influenced Target As SSRF-Exposed

The first step is recognizing the surface. SSRF applies whenever the server makes an outbound request and the user can influence the destination — directly (a URL field) or indirectly (a configured webhook URL, a filename that maps to a path, a hostname derived from user data, a redirect the server follows). Common SSRF-prone features:

- **Link preview / unfurling / rich cards** that fetch a URL to render a preview.
- **Webhooks** where the user configures the destination URL the server calls.
- **"Import from URL"** for images, files, feeds, calendars, or documents.
- **PDF generators, screenshot services, headless browsers** that render a URL server-side.
- **OAuth / SSO callback or discovery fetches** that follow user-supplied URLs.
- **Any integration that lets the user point the server at an arbitrary host.**

If a feature makes the server fetch a URL the user controls or influences, it is SSRF-exposed and needs the defenses below. Do not assume "it's just an image fetch" — the server's network position is what matters, not the content type.

### Block Internal Destinations With Network Egress Controls, Not URL Blocklists

The strongest SSRF defense is network-level: the service that fetches user-influenced URLs should be unable to reach internal addresses at all, regardless of what URL it is handed. This is more reliable than parsing the URL, because parsing-based defenses are bypassed by encoding, redirect, DNS rebinding, and the sheer variety of ways to name an internal host.

- **Egress allowlisting.** Restrict the fetcher's outbound network access to a specific allowlist of external destinations (or a proxy that enforces it). The fetcher simply cannot connect to `10.0.0.0/8`, `169.254.169.254`, localhost, or internal DNS, because the network refuses.
- **Run the fetcher in an isolated network context** — a separate subnet, security group, or sandbox — with no route to internal services and no access to the metadata service.
- **Block the cloud metadata endpoint explicitly** at the network layer. `169.254.169.254` (and `fd00:ec2::254` on IPv6 for AWS, the equivalent on other clouds) hands out instance credentials; a fetcher that can reach it can steal the role's permissions.
- **Prefer this over URL parsing.** A blocklist of "bad IPs" in application code is bypassable; a network that has no route to internal addresses is not.

Network controls are the primary defense. URL validation is defense in depth on top, never the only layer.

### Validate And Resolve The URL Before Connecting, And Re-Check After Resolution

When application-level validation is used (as defense in depth alongside network controls), it must handle the ways an attacker disguises an internal destination. A naive check against the hostname string is trivially bypassed.

- **Parse strictly and reject unexpected schemes.** Allow only `http` and `https`; reject `file://`, `gopher://`, `dict://`, `ftp://`, `data:`, and anything else. `file://` reads local files; `gopher://` and others can speak arbitrary protocols to internal services.
- **Resolve the hostname to an IP yourself, and check the IP against a blocklist of internal ranges** before connecting: loopback (`127.0.0.0/8`, `::1`), link-local (`169.254.0.0/16`), private (`10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `fc00::/7`), and any internal ranges you use.
- **Beware DNS rebinding.** The hostname may resolve to a public IP on your first lookup (passing the check) and to an internal IP on the second lookup (when the HTTP client connects). Defenses: resolve once and connect to the resolved IP directly (pinning the address), use a custom DNS resolver that locks the answer, or use a fetching library that checks the IP at connect time.
- **Do not follow redirects blindly.** A public URL that 302s to `http://localhost/admin` bypasses a check done on the original URL. Either disable redirect-following, or re-validate every hop's destination against the same rules.
- **Reject IP-literal internal addresses in every form**: decimal, hex, octal, IPv6-mapped IPv4, and truncated forms. Attackers encode `127.0.0.1` as `2130706433`, `0x7f000001`, `127.1`, `[::ffff:127.0.0.1]`, and more.

Application validation must assume the input is adversarial and constructed to evade string checks. Resolve, normalize, and check the actual IP — and still rely on network controls as the primary defense.

### Disable Or Sandbox The Metadata Service Access

The cloud instance metadata service (IMDS) is the highest-value SSRF target, because it returns temporary credentials for the instance's IAM role. A fetcher that reaches `169.254.169.254` can steal credentials that often have broad permissions.

- **On AWS, enforce IMDSv2** (token-based metadata access), which defeats SSRF that can only make simple GET requests. IMDSv1 was fetchable by any SSRF; IMDSv2 requires a PUT to obtain a token first, which most SSRF cannot do.
- **Block metadata access at the network layer** for the fetcher specifically, via routing, security groups, or iptables, so even IMDSv2 is unreachable.
- **Apply least privilege to the instance role** so that even if credentials are stolen, the blast radius is small. A fetcher service should not run with a role that can read S3 buckets, invoke admin APIs, or assume other roles.

Metadata theft via SSRF is one of the most common paths from "minor bug" to "full cloud account compromise." Treat the metadata endpoint as the single most important thing the fetcher must not reach.

### Do Not Trust Internal Endpoints To Be Unauthenticated

A major reason SSRF is so damaging is that internal services are frequently deployed without authentication, on the assumption that the network is the boundary. SSRF punctures that assumption: the attacker's request now originates from inside the network.

- **Authenticate internal services too.** Service-to-service auth (mTLS, signed requests, internal tokens) means an SSRF that reaches an internal endpoint still cannot act without credentials.
- **Do not expose admin, debug, or management endpoints on the same network the fetcher can reach.** Put them behind a separate boundary.
- **Bind management interfaces (JMX, debug consoles, metrics, actuator endpoints) to localhost only, or behind auth** — an SSRF to `http://localhost:8080/actuator/env` can leak secrets or enable shutdown.

The network is not an authentication boundary once the server will fetch arbitrary URLs for you. Treat internal endpoints as if they were internet-facing, because via SSRF they are.

### Limit What The Fetcher Can Do And Return

Even when the destination is legitimate, constrain the fetch to reduce blast radius.

- **Set tight timeouts and size limits.** A fetcher that follows a URL to a 10GB file or a slow endpoint is a denial-of-service vector. Cap response size and connection time.
- **Restrict methods and headers.** The fetcher should use GET (or the specific method needed) with minimal headers; do not forward request headers, cookies, or authorization from the original request to the fetched URL (that leaks credentials outward).
- **Validate the response content type** before processing, and do not execute or render fetched content in an unsafe context (a headless browser fetching an attacker URL can be driven to XSS or worse; sandbox it).
- **Do not reflect fetched internal content back to the user** without sanitization — an SSRF that returns internal responses to the attacker is a full read primitive.

## Common Traps

### Trusting A URL Blocklist Of "Bad Strings"

Checking the hostname against a list like `["localhost", "127.0.0.1", "169.254.169.254"]` and believing it blocks internal access. Bypassed by IP encoding (`2130706433`), alternate names, IPv6 (`[::1]`), decimal/hex/octal, subdomains that resolve internally, and redirects. Resolve and check the actual IP; rely on network controls.

### Fetching The URL Without Checking Redirects

Validating the original URL, then letting the HTTP client follow a 302 to `http://localhost/admin`. Either disable redirect-following or re-validate every hop against the same destination rules.

### DNS Rebinding After The Validation Check

Resolving the hostname once (to a public IP that passes the check), then the HTTP client resolving it again (to an internal IP) at connect time. Pin the resolved IP for the connection, or use a resolver/library that validates at connect time.

### Allowing Dangerous Schemes

Accepting `file://`, `gopher://`, `data:`, or `dict://` because the feature "just fetches a URL." `file://` reads local files; `gopher://` and `dict://` speak arbitrary protocols to internal services. Allow only `http` and `https`.

### Reaching The Cloud Metadata Service

A fetcher with default network access reaching `169.254.169.254` and stealing instance credentials via IMDSv1. Enforce IMDSv2, block metadata at the network layer, and apply least privilege to the instance role.

### Internal Admin Endpoint With No Auth, Reachable Via SSRF

An internal service or debug/actuator endpoint deployed without authentication because "it's internal," then reached by an SSRF that now has admin power. Authenticate internal services; bind management interfaces to localhost or behind auth.

### Forwarding Request Credentials Outward and no Size Or Timeout Limit On The Fetch

Passing the original request's cookies, Authorization header, or internal tokens to the fetched URL, leaking credentials to an attacker-controlled server. The fetcher must use its own minimal, scoped credentials, never the caller's.

A fetcher that will download an arbitrarily large file or wait indefinitely, turning the feature into a denial-of-service vector against the server's memory, disk, or connection pool. Cap response size and connection/read time.

### Reflecting Fetched Internal Responses Back To The Attacker and headless Browser Or PDF Renderer With No Sandbox

Returning the raw fetched response to the user, turning a blind SSRF (no response read) into a full read of internal endpoints. Do not reflect fetched content without sanitization, and limit what the fetcher returns.

Rendering an attacker-supplied URL in a headless browser or PDF generator that runs as the service user with full local access. Sandbox the renderer, restrict its network, and treat the URL as malicious.

## Self-Check

- [ ] Every feature where the server fetches a URL the user can influence (previews, webhooks, importers, renderers, callbacks) was identified as SSRF-exposed and given the full set of defenses, not treated as a routine HTTP call.
- [ ] The primary defense is network-level: the fetcher runs in an isolated network context with egress allowlisting and no route to internal addresses or the metadata endpoint — not a URL blocklist in application code.
- [ ] The cloud metadata endpoint is defended (IMDSv2 enforced, blocked at the network layer for the fetcher, and the instance role follows least privilege so stolen credentials have minimal blast radius).
- [ ] Application-level validation (defense in depth) parses strictly, allows only http/https schemes, resolves the hostname and checks the resolved IP against internal ranges (loopback, link-local, private, IPv6-mapped), and resists encoded/alternate IP forms.
- [ ] DNS rebinding is handled (the resolved IP is pinned for the connection, or a resolver validates at connect time), and redirect-following is either disabled or re-validates every hop against the same destination rules.
- [ ] Internal services are authenticated (mTLS, signed requests, internal tokens) and management/debug/actuator endpoints are bound to localhost or behind auth — the network is not relied on as the only auth boundary.
- [ ] The fetcher uses tight timeouts and response size limits, sends only minimal headers with its own scoped credentials (never the caller's cookies or Authorization), validates the response content type, and does not reflect fetched internal content to the user unsanitized.
- [ ] Headless browsers, PDF renderers, and similar are sandboxed with restricted network access and treated the URL as malicious.
- [ ] The defense was tested against adversarial inputs (localhost variants, encoded IPs, metadata URLs, redirect-to-internal, DNS rebinding), not only against benign public URLs.
