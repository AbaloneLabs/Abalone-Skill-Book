---
name: cors_and_csp_design.md
description: Use when the agent is configuring Cross-Origin Resource Sharing (CORS) policies, setting Access-Control-Allow-Origin and credentials headers, handling CORS preflight requests, allowing or blocking credentialed cross-origin reads, designing or tightening a Content-Security-Policy, choosing CSP directives (script-src, connect-src, frame-src, default-src), deciding between nonces, hashes, strict-dynamic, or unsafe-inline, deploying CSP in report-only versus enforce mode, interpreting CSP violation reports, or reviewing a web application for cross-origin data leakage or script injection containment. Also covers the difference between CORS and CSRF, credentialed CORS risks, the failure of reflecting arbitrary origins, CSP as defense-in-depth for XSS, and the traps of unsafe-inline with nonces.
---

# CORS And CSP Design

CORS and CSP are two browser-enforced policies that are routinely misunderstood because they sound like "security headers you set once" but are in fact precise contracts with the browser about who may read what, and what may execute where. CORS governs whether a script on origin A may read a response from origin B. CSP governs what content the browser will execute or load on a page. Both are powerful, both are easy to configure in a way that "works" while providing little or no protection, and both fail in specific, predictable ways: a CORS policy that reflects the request origin and allows credentials turns the same-origin policy into a suggestion; a CSP that still permits `unsafe-inline` blocks nothing. The harm is a false sense of security — the headers are present, the scanner is green, and the protection is absent.

Agents tend to under-invest here because the default-permissive configuration makes the feature work immediately. Setting `Access-Control-Allow-Origin: *` makes the cross-origin call succeed; leaving `unsafe-inline` in the CSP keeps the legacy inline scripts running. The defects live in the cases where permissive defaults become vulnerabilities: credentialed responses readable by any origin, a CSP that lists every CDN in the world and thus any XSS on them, a report-only policy that was never switched to enforcement, or a preflight that is bypassed by a content type the server did not expect. The judgment problem is treating each policy as a deliberate, minimal allowlist reviewed for what it permits, not as a header that makes the feature work.

This skill is about designing CORS and CSP as real controls. It complements the client-side attack skill (which covers XSS and CSRF mechanisms and where CSP and SameSite fit as defenses) and the security headers skill (which covers the broader header set). Here the question is how to configure cross-origin read policy and content execution policy so they actually constrain what they claim to.

## Core Rules

### Treat CORS As Read-Permission For Cross-Origin Scripts, Not As Authentication

CORS does not authenticate or authorize; it tells the browser whether a script on another origin may read the response. The confusion that causes most CORS defects is believing CORS protects the server. It does not — the server receives the request regardless; CORS only decides whether the browser hands the response to the script. A request without a browser (a server-to-server call, curl) is unaffected by CORS entirely.

- **CORS is enforced by the browser, on the response, for cross-origin script reads.** The server still receives and processes the request; a state-changing endpoint that relies on CORS to block malicious requests is undefended (that is CSRF's job).
- **Do not confuse CORS with CSRF defense.** CORS controls reads; CSRF defenses (SameSite, tokens) control forged writes. An endpoint can have correct CORS and still be CSRF-vulnerable.
- **A permissive CORS policy does not make the server less secure against direct attacks, but it lets any website's script read responses that the same-origin policy would otherwise protect** — which matters when those responses contain user-specific data and the request carries credentials.

Start from the understanding that CORS is a relaxation of the same-origin policy. Every relaxation must be justified; the default should be no cross-origin read access.

### Default To No Credentials, And Never Combine Wildcard Origin With Credentials

The most dangerous CORS configuration is credentialed access (cookies, HTTP auth, client certificates) combined with a permissive origin policy. This lets any website's script make an authenticated request to your API and read the response — effectively a cross-origin data theft that the browser considers authorized.

- **`Access-Control-Allow-Credentials: true` combined with `Access-Control-Allow-Origin: *` is invalid and rejected by browsers** — but the equivalent mistake is reflecting the request's `Origin` header back as the allowed origin while allowing credentials. This is functionally `*` with credentials and is a severe vulnerability.
- **If you allow credentials, the allowed origin must be a specific, enumerated, trusted origin** — never the reflected request origin, never a wildcard.
- **Prefer not allowing credentials at all** for public APIs that use token-based auth in a header (not cookies). Token-in-header APIs are not subject to the browser's automatic credential attachment, so credentialed CORS is unnecessary.
- **Whitelist origins explicitly.** Maintain a server-side allowlist of permitted origins; return that origin only if the request matches, and omit the header otherwise. Do not echo.

The rule: credentialed CORS requires an explicit, reviewed origin allowlist. Reflecting the Origin header with credentials is the canonical CORS vulnerability.

### Understand Preflight And What It Does And Does Not Guarantee

For "non-simple" cross-origin requests (custom headers, non-simple methods like PUT/DELETE, non-simple content types like `application/json`), the browser sends a preflight `OPTIONS` request, and the actual request proceeds only if the preflight is approved. Preflight is a browser-side gate, and misunderstanding it causes two classes of defect.

- **Preflight does not protect "simple" requests.** A `GET` or `POST` with simple content types (`application/x-www-form-urlencoded`, `multipart/form-data`, `text/plain`) and no custom headers is sent directly, with no preflight. If such a request mutates state and relies on cookies, it is CSRF-vulnerable regardless of CORS.
- **Do not put authorization logic in the preflight handler.** The preflight `OPTIONS` response describes policy; it does not authenticate. The actual request handler must enforce auth independently. A common bug is treating a successful preflight as authorization.
- **Cache preflight results appropriately** (`Access-Control-Max-Age`) to avoid an OPTIONS round-trip on every request, but understand the cache is per-origin in the browser.

Preflight is a mechanism for the browser to ask permission for a richer request; it is not a security boundary the server can rely on for simple requests or for authorization.

### Design CSP As A Minimal Allowlist, Defaulting To 'self'

Content-Security-Policy is the strongest browser-enforced defense against XSS when configured correctly: it tells the browser which sources may load and execute content, and it blocks everything else. A strong CSP defaults to allowing only the same origin and adds exceptions deliberately. The recurring failure is a CSP that permits so much that it blocks nothing meaningful.

- **Start from `default-src 'self'` and widen only what is needed.** Every directive not set inherits from `default-src`, so a strict default cascades.
- **Remove `unsafe-inline` and `unsafe-eval`.** These directives effectively disable CSP's script protection. `unsafe-inline` for `script-src` means any inline script executes — which is exactly what XSS injects. The fix is nonces or hashes (see below), not leaving `unsafe-inline` on.
- **Whitelist script sources narrowly.** Every domain in `script-src` is a domain whose compromise becomes XSS on your site. Listing a dozen CDNs and `*.google.com` makes the policy weak; prefer `'self'` plus specific, reviewed hosts.
- **Cover the directives that matter for your page**: `script-src`, `style-src`, `img-src`, `connect-src` (which limits fetch/XHR/websocket destinations — useful against data exfiltration), `frame-src`/`frame-ancestors`, `object-src` (set to `'none'` unless you need plugins), `base-uri`, and `form-action`.

A CSP that includes `unsafe-inline` or a broad wildcard is a CSP in name only. The strength is in the minimality of the allowlist.

### Use Nonces Or Hashes To Allow Legitimate Inline Script Without unsafe-inline

Modern applications often need some inline script. The correct way to permit it without disabling CSP's protection is nonces or hashes, not `unsafe-inline`.

- **Nonces.** Generate a fresh, unguessable nonce per request, add it to the `script-src` directive (`'nonce-<random>'`), and tag each legitimate inline script with `nonce="<same random>"`. The browser executes only scripts whose nonce matches. Nonces must be unique per request and never reused; a predictable or static nonce defeats the mechanism.
- **Hashes.** For stable inline scripts, include the hash of the script content (`'<sha256-...>'`). The browser executes the inline script only if its content matches the hash.
- **`strict-dynamic`** lets trusted scripts (those with the nonce/hash) load further scripts transitively, which helps with frameworks and third-party loaders without enumerating every source — but it trusts whatever those scripts load.
- **Never combine `'nonce-...'` or `'hash-...'` with `'unsafe-inline'`.** Browsers ignore nonces/hashes when `unsafe-inline` is present (per the spec, with a nonce exception), so the policy silently degrades. Remove `unsafe-inline` entirely when using nonces.

The transition from `unsafe-inline` to nonces is the single highest-leverage CSP hardening step, and it is the one most often skipped because legacy inline scripts break.

### Deploy In Report-Only First, Then Enforce, And Watch The Reports

A CSP that breaks the page (blocking a legitimate script) creates immediate pressure to add `unsafe-inline` back. The disciplined path is to measure first.

- **Start with `Content-Security-Policy-Report-Only`** with the intended policy. The browser reports violations without blocking, so you can see what would break.
- **Collect and review violation reports** via a `report-uri` / `report-to` endpoint. Triage the violations: each is either a real policy gap (add the source) or an actual injection (fix the code, do not relax the policy).
- **Switch to enforcement (`Content-Security-Policy`) only after violations are understood and the policy is stable.** Keep reporting on in enforce mode to catch regressions.
- **Do not leave a policy in report-only forever.** A report-only CSP that is never enforced provides no protection; it is a monitoring tool, not a control.

The trap is deploying an enforcement CSP under deadline, breaking the page, and panic-adding `unsafe-inline`. Report-only first avoids this.

### Use frame-ancestors To Prevent Framing (Clickjacking), Not X-Frame-Options Alone

CSP `frame-ancestors` controls which origins may embed the page in a frame/iframe, defending against clickjacking. It supersedes the older `X-Frame-Options` header and is more expressive (it allows a list of allowed ancestors, not just DENY/SAMEORIGIN).

- **Set `frame-ancestors 'none'` or `'self'`** unless the page is genuinely intended to be framed by a specific partner origin.
- **Prefer CSP `frame-ancestors` over `X--Frame-Options`** for new work, but set both for breadth, since not all clients honor CSP.
- **Beware pages that must be framed by a third party** (a widget, an OAuth popup): the allowed ancestor must be specific and reviewed, not a wildcard.

## Common Traps

### Reflecting The Origin Header With Credentials

Returning `Access-Control-Allow-Origin: <whatever the request sent>` along with `Access-Control-Allow-Credentials: true`, so any website's script can make an authenticated request and read the response. Use an explicit origin allowlist; never reflect.

### `Access-Control-Allow-Origin: *` On A Credentialed, User-Specific Endpoint

Believing the wildcard is safe because "browsers block credentials with `*`" — true, but the same endpoint reached via cookie auth with a reflected origin is the vulnerability. For user-specific data with cookie auth, never use wildcard or reflected origins.

### Treating CORS As CSRF Defense

Relying on CORS to block malicious cross-origin writes, when CORS governs reads and "simple" requests bypass preflight entirely. Defend writes with SameSite cookies and CSRF tokens; CORS is a separate concern.

### Putting Auth Logic In The Preflight Handler

Authorizing the request inside the `OPTIONS` preflight handler and assuming the actual request is therefore trusted. The preflight describes policy; the actual request handler must authenticate independently.

### CSP With `unsafe-inline` And `unsafe-eval` Still Present

A CSP that includes `unsafe-inline` in `script-src`, which disables the policy's core protection against injected inline script. Replace with nonces or hashes; remove `unsafe-inline`.

### Broad Wildcard Or Huge Source List In script-src

`script-src *` or a long list of CDNs and `*.google.com`, any of whose compromise becomes script execution on your site. Minimize the list to specific, reviewed hosts plus `'self'`.

### Report-Only Policy Never Switched To Enforcement and nonce Combined With unsafe-inline

A CSP deployed in `Content-Security-Policy-Report-Only` that stays report-only indefinitely, providing monitoring but no blocking. Triage reports, then enforce.

Including both `'nonce-...'` and `'unsafe-inline'`, which causes browsers to ignore the nonce (the policy degrades to allowing inline). When using nonces, remove `unsafe-inline` entirely.

### Static Or Predictable Nonce and trusting A CDN Domain That Hosts User-Uploaded Content

A nonce that is hardcoded or reused across requests, defeating the per-request uniqueness that makes nonces secure. Generate a fresh CSPRNG nonce per response.

Allowing a storage/CDN domain in `script-src` that also serves user-uploaded files, so an attacker uploads a script and it executes under the allowed source. Separate user content origins from script-allowed origins.

## Self-Check

- [ ] CORS is understood as a browser-enforced read-permission policy, not as authentication or CSRF defense; state-changing endpoints are defended by SameSite and CSRF tokens independently of CORS.
- [ ] No credentialed endpoint (`Access-Control-Allow-Credentials: true`) uses a wildcard or reflected origin; allowed origins come from an explicit, server-side, reviewed allowlist, and public token-header APIs do not allow credentials at all.
- [ ] Preflight is understood to protect only non-simple requests; simple GET/POST form-style requests bypass it, and no authorization logic lives in the `OPTIONS` handler.
- [ ] The CSP defaults to `default-src 'self'`, covers the relevant directives (script, style, img, connect, frame, object, base-uri, form-action), and does not contain `unsafe-inline` or `unsafe-eval` for script sources.
- [ ] Legitimate inline script is permitted via per-request nonces (or content hashes), the nonce is generated fresh by a CSPRNG each response, and `unsafe-inline` is removed wherever nonces are used.
- [ ] The script/style source lists are minimal — specific, reviewed hosts rather than wildcards or broad CDNs — and no allowed source also serves user-uploaded content.
- [ ] The CSP was deployed in `Content-Security-Policy-Report-Only` first, violation reports were collected and triaged (distinguishing policy gaps from real injections), and the policy was then moved to enforcement rather than left report-only indefinitely.
- [ ] `frame-ancestors` is set to `'none'` or `'self'` (with specific reviewed partners where framing is intended) to prevent clickjacking, alongside `X-Frame-Options` for breadth.
- [ ] The policy was reviewed for what it permits, not merely that it is present — a permissive CSP provides no protection regardless of being set.
