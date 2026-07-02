---
name: security_headers_and_transport_security.md
description: Use when the agent is setting or reviewing HTTP security response headers such as Strict-Transport-Security (HSTS), X-Frame-Options, X-Content-Type-Options, Referrer-Policy, Permissions-Policy, Cross-Origin-Opener-Policy, Cross-Origin-Embedder-Policy, or Cache-Control on sensitive responses; configuring HSTS preload and max-age; deciding which headers apply per response type; hardening transport security against SSL stripping; preventing MIME sniffing, clickjacking, or unwanted feature access; or reviewing a response header set for completeness and correctness. Also covers header ordering quirks, the interaction between headers and caching layers, duplicate or conflicting headers, and the failure mode of setting headers only on some responses.
---

# Security Headers And Transport Security

Security headers are a set of browser-enforced controls applied through HTTP response headers. Each one closes a specific class of vulnerability or hardens a specific behavior: HSTS forces HTTPS and defeats SSL stripping; `X-Content-Type-Options` stops MIME sniffing; frame-ancestors and X-Frame-Options stop clickjacking; Referrer-Policy limits what leaks in the Referer header; Permissions-Policy disables browser features the page does not need. They are cheap, strong, and the most common thing to get "almost right" in a way that provides no protection: a header set on most responses but missing on the one error page, an HSTS policy with a max-age so short it is useless, a header duplicated by middleware and a framework into a conflicting pair, or a caching layer that strips the headers before they reach the browser. The harm is a header audit that passes on the happy path and fails on the path an attacker actually uses.

Agents tend to under-invest here because the headers are well-known and feel like a checklist. The defects live in the operational details the checklist hides: which responses carry which headers, whether a reverse proxy or CDN preserves them, whether conflicting duplicates are produced, whether the HSTS policy includes subdomains and preload, whether Cache-Control on a sensitive response actually prevents caching, and whether the headers survive a redirect chain. The judgment problem is treating the header set as a property of every response the server emits, verified end-to-end through every layer, not as a line added to the main controller.

This skill is about applying security and transport headers correctly and completely. It complements the CORS and CSP skill (which covers those two policies in depth) and the TLS skill (which covers the encrypted channel itself). Here the question is how to use response headers to harden transport, framing, content type, referrer leakage, feature access, and caching behavior across every response.

## Core Rules

### Apply Security Headers Universally, Not Only On "Important" Pages

A security header protects only the responses it is on. The recurring defect is setting headers in the main application controller but not on error pages, redirect responses, static asset responses, health checks, or responses served by middleware before the controller runs. An attacker targets the responses without the headers.

- **Set headers at the framework, middleware, or web-server level** so they apply to every response, including errors, redirects, 304s, and static files, not page by page.
- **Verify the headers reach the browser, not just that the code sets them.** A reverse proxy, CDN, load balancer, or API gateway may add, strip, or override headers. Test against the real path the browser uses.
- **Cover every origin and subdomain** that serves content. A header on `app.example.com` does nothing for `static.example.com` if the latter serves user-facing content.
- **Do not rely on a single "security headers" middleware alone if other layers emit responses** (a static file server, a separate admin app, a legacy service) that bypass it.

The test: enumerate every response type and origin the site emits, and confirm each carries the intended headers. A header missing on the login error page or a redirect is a gap an attacker will find.

### Configure HSTS To Actually Force HTTPS, Including Subdomains And Preload

HSTS tells the browser to always use HTTPS for the origin for a given period, defeating SSL-stripping attacks that downgrade the first request to HTTP. A weak HSTS policy is barely better than none.

- **Set a meaningful `max-age`.** At least six months (15552000 seconds); a year is common. A short max-age expires quickly and leaves the origin exposed; `max-age=0` actively disables HSTS (useful for removal, never for protection).
- **Include `includeSubDomains`** if all subdomains are HTTPS-ready. This prevents a stripped HTTP connection on a forgotten subdomain. But verify every subdomain is HTTPS first — `includeSubDomains` will break any subdomain still on HTTP.
- **Submit to the HSTS preload list** for high-value domains, after confirming the entire domain (including subdomains) is permanently HTTPS. Preloading bakes the policy into the browser, protecting even the very first visit. This is hard to reverse — be certain before preloading.
- **HSTS applies only after the browser has seen it once over HTTPS.** The first visit is still vulnerable to stripping until the header is received; preload closes that gap.
- **Do not set HSTS on a site that is not fully HTTPS.** HSTS will lock users out of any HTTP-only resource, including during an incident if you must temporarily serve HTTP.

HSTS is the strongest transport-hardening control, but its strength comes from the completeness of HTTPS deployment it assumes. Preload is a one-way door; use it deliberately.

### Set X-Content-Type-Options nosniff Everywhere To Stop MIME Sniffing

Browsers historically "sniff" the content type of a response when the declared type is absent or generic, which lets an attacker upload a file declared as an image that the browser executes as script. `X-Content-Type-Options: nosniff` tells the browser to respect the declared type and not sniff.

- **Set `nosniff` on every response**, especially on responses serving user-uploaded or user-influenced content. It is cheap and broadly supported.
- **It is defense in depth alongside correct Content-Type setting and CSP**, not a replacement. Still set accurate Content-Types; still use CSP to block unauthorized script.
- **CSP also has a directive for this** (`require-trusted-types-for` and related), but `nosniff` is the universal baseline.

### Use frame-ancestors / X-Frame-Options To Prevent Unauthorized Framing

Clickjacking (UI redress) is defeated by preventing the page from being embedded in a frame by an untrusted ancestor. Two headers do this; use them together for breadth.

- **CSP `frame-ancestors`** is the modern, expressive control (allows a list of permitted ancestors, or `'none'`/`'self'`). Prefer it.
- **`X-Frame-Options`** (`DENY` or `SAMEORIGIN`) is the legacy control; set it too for older clients.
- **Default to not framable** (`frame-ancestors 'none'` or `'self'`, `X-Frame-Options: DENY`/`SAMEORIGIN`), and allow specific partner origins only where framing is genuinely intended (a widget, an embedded app).
- **Apply to authenticated and state-changing pages especially.** A login page or a settings page that can be framed is a clickjacking target.

### Choose Referrer-Policy To Limit What Leaks In The Referer Header

The `Referer` header leaks the full URL of the page that initiated a navigation or subresource request. For an authenticated app, that URL often contains sensitive path segments, query parameters, or tokens. `Referrer-Policy` controls how much is sent.

- **Default to `strict-origin-when-cross-origin`** (the modern browser default). This sends the full URL on same-origin requests, only the origin on cross-origin HTTPS requests, and nothing on HTTPS→HTTP downgrades.
- **Use `no-referrer` or `same-origin`** for pages or links where no cross-origin referrer should leak at all.
- **Avoid `unsafe-url` or `no-referrer-when-downgrade`** which send the full URL (including query string) cross-origin, leaking sensitive path/parameter data to third parties.
- **Beware cross-origin subresource requests** (analytics, images, scripts) that carry the referrer to a third party; the policy applies to those too.

A page that puts a token in the URL and uses `unsafe-url` leaks that token to every third-party script's origin. Match the policy to the sensitivity of what the URL contains.

### Use Permissions-Policy To Disable Unneeded Browser Features

`Permissions-Policy` (formerly Feature-Policy) lets a site disable browser features (camera, microphone, geolocation, payment, USB, etc.) that the page does not need, reducing the attack surface if the page is compromised.

- **Default to denying features the page does not use** (camera, microphone, geolocation, payment, USB, magnetometer, etc.).
- **Allow specific features only on the origins/pages that need them.**
- **This is defense in depth**, not a primary control, but it shrinks what an XSS or a malicious third-party script can access.

### Set Cache-Control Correctly On Sensitive Responses

Caching is a privacy and correctness surface as much as a performance one. A sensitive response (account page, API response with user data, auth token) cached by a shared proxy or the browser is a data leak.

- **Use `Cache-Control: no-store`** for responses containing sensitive, per-user, or authenticated data. `no-store` tells caches not to store the response at all.
- **`no-cache` is not the same as `no-store`.** `no-cache` allows storage but forces revalidation; for truly sensitive data, `no-store` is correct.
- **Set caching headers on the response the cache sees**, which may be different from the response the browser sees if a CDN sits in between. Verify the cache behavior end-to-end.
- **For public, cacheable content, use explicit `public` and a max-age** so caches behave predictably, and ensure no per-user data is in the cached response.

This intersects the caching skill's rule that authenticated responses must not be cached under public keys. The header is the mechanism that enforces it.

### Avoid Duplicate Or Conflicting Headers

When multiple layers (framework middleware, application code, reverse proxy, CDN) each set a header, the result can be duplicates that browsers interpret inconsistently. Two `Content-Security-Policy` headers are intersected (both must allow); two `X-Frame-Options` headers may conflict; two `Set-Cookie` are additive but two `Content-Type` are undefined.

- **Decide which layer owns each header** and set it once. Document the ownership.
- **Test for duplicates** by inspecting the actual response, not the code. A header set in both the app and the proxy is a common, invisible defect.
- **For CSP, prefer one combined policy** rather than two headers, since intersection can produce surprising restrictions.

## Common Traps

### Headers Set On Happy-Path Pages But Missing On Errors Or Redirects

Adding security headers in the main controller, but the 500 error page, the 302 redirect, or the static asset response does not carry them. Set headers at the framework/middleware level so every response includes them, and verify error and redirect paths.

### HSTS With A Trivial max-age Or Without includeSubDomains

`max-age=300` that expires in five minutes, or HSTS on the apex domain without `includeSubDomains` so a subdomain is still strip-able. Use a long max-age, include subdomains only when all are HTTPS, and consider preload.

### Preloading A Domain That Is Not Fully HTTPS

Submitting to the HSTS preload list before confirming every subdomain is permanently HTTPS, then breaking an HTTP-only subdomain or an emergency HTTP fallback. Preload is hard to reverse; verify first.

### no-cache Used Where no-store Was Needed

Setting `Cache-Control: no-cache` on a sensitive response, which still allows the cache to store it (revalidating each time), so the response sits in a shared cache. Use `no-store` for responses that must never be stored.

### Referrer-Policy Left At unsafe-url

`Referrer-Policy: unsafe-url` sending the full URL (with query parameters) to every third-party origin, leaking sensitive path or token data. Use `strict-origin-when-cross-origin` or stricter.

### Duplicate Headers From Multiple Layers

The app and the reverse proxy both setting `Content-Security-Policy` or `X-Frame-Options`, producing duplicates that browsers interpret inconsistently. Decide ownership per header and verify the actual response.

### Permissions-Policy Omitted So All Features Stay Available

Not setting Permissions-Policy, so a compromised page can request camera, microphone, geolocation, or payment access. Deny features the page does not use.

### Headers Stripped By A CDN Or Proxy, Verified Only In Code

Setting headers in code but never testing the response as it reaches the browser, so a CDN that strips unknown headers silently removes the protection. Test end-to-end.

### X-Content-Type-Options Missing On User-Content Endpoints

Forgetting `nosniff` on endpoints that serve user-uploaded files, allowing MIME sniffing to turn an uploaded "image" into executed script. Set `nosniff` universally.

## Self-Check

- [ ] Security headers are applied at the framework, middleware, or web-server level so they appear on every response type (happy path, errors, redirects, static assets, health checks), and this was verified by inspecting actual responses, not just reading the code.
- [ ] HSTS is configured with a long `max-age` (six months to a year), `includeSubDomains` only where every subdomain is HTTPS, and preload submitted only after confirming permanent full-HTTPS; no HSTS is set on sites that are not fully HTTPS.
- [ ] `X-Content-Type-Options: nosniff` is set on every response, especially user-content endpoints, as defense in depth alongside correct Content-Type and CSP.
- [ ] Framing is controlled via CSP `frame-ancestors` (preferred) and `X-Frame-Options`, defaulting to not framable, with specific partner origins allowed only where framing is intended; authenticated and state-changing pages are covered.
- [ ] `Referrer-Policy` is set to `strict-origin-when-cross-origin` or stricter, never `unsafe-url`, so cross-origin requests do not leak sensitive path or query data to third parties.
- [ ] `Permissions-Policy` disables browser features the page does not use (camera, microphone, geolocation, payment, USB, etc.) as defense in depth.
- [ ] Sensitive and authenticated responses use `Cache-Control: no-store` (not merely `no-cache`), public cacheable content uses explicit `public`/`max-age`, and cache behavior was verified end-to-end through any CDN/proxy.
- [ ] No header is duplicated or conflicting across layers; ownership of each header is decided and documented, and the actual response was inspected for duplicates.
- [ ] The header set was verified to reach the browser through every layer (reverse proxy, CDN, load balancer, gateway), not assumed from the application code.
