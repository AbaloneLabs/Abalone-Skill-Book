---
name: client_side_attack_prevention.md
description: Use when the agent is rendering user-generated or third-party content into HTML, building templates that interpolate dynamic data, handling rich-text or markdown input, setting Content-Security-Policy or other security headers, designing cookie attributes, protecting authenticated state-changing actions, implementing CSRF tokens or SameSite cookies, embedding untrusted content in iframes, building single-page-app state that touches the DOM, or reviewing code for cross-site scripting (reflected, stored, or DOM-based) and cross-site request forgery. Also covers output encoding contexts, framework auto-escaping and its limits, Trusted Types, clickjacking defense, and the failure modes of relying on framework defaults.
---

# Client-Side Attack Prevention (XSS And CSRF)

XSS and CSRF are two sides of the same boundary: the browser's trust in the origin and in the user's authenticated session. XSS abuses the browser's willingness to execute script that appears to come from the site, letting an attacker run code in the victim's session and do anything the victim could do — without needing the victim's password. CSRF abuses the browser's willingness to attach the victim's cookies to requests the victim did not intend, letting an attacker perform authenticated actions as the victim — without needing to run any script. Both attacks succeed because the browser is doing exactly what it was designed to do, against a site that did not constrain it.

Agents tend to under-invest here for a familiar reason: the happy path renders correctly, the framework auto-escapes by default, and the CSRF middleware is turned on. The vulnerability appears only when content reaches a context the framework does not auto-escape, when a developer bypasses escaping to render rich text, when a DOM sink receives tainted data, or when a state-changing endpoint lacks a CSRF defense. The harm is direct: session theft, account takeover, data destruction, unauthorized purchases or transfers, and mass compromise of every user who views a poisoned page.

The judgment problem is deciding, for each piece of dynamic content and each authenticated action, which client-side attack class applies, which context the content enters, and which defense is correct for that context. The agent has an obligation to know where the framework's defaults stop protecting and to add explicit defenses there, because attackers look precisely at those seams.

## Core Rules

### Know The XSS Variants And Where Each Comes From

XSS is not one bug; it is a class with three origins, and the defense differs by origin.

- **Reflected XSS.** Input from the current request (a query parameter, a path segment, a header) is echoed into the response without encoding. The attacker delivers the payload by getting the victim to click a crafted link. Defense: encode the value for the output context, and never reflect raw input into HTML.
- **Stored XSS.** Attacker-supplied content is saved (comment, profile field, message, document) and later rendered to other users. Far more dangerous than reflected, because one payload compromises every viewer and requires no per-victim link. Defense: encode on output regardless of where the data came from, and treat all stored content as untrusted when it reaches the DOM.
- **DOM-based XSS.** Data is taken from a client-side source (URL fragment, `location`, `document.referrer`, `postMessage`, storage) and written to a sink (`innerHTML`, `document.write`, eval, attribute setters) without sanitization. The server never sees the payload, so server-side encoding does not help. Defense: avoid the sink, use safe DOM APIs (`textContent`, `setAttribute` with safe values), and sanitize with a context-aware sanitizer when rich content is unavoidable.

Identify which origin applies to each dynamic value. A value can be all three across different pages. The defense must cover the origin that actually feeds the sink.

### Encode For The Exact Output Context

Output encoding is context-dependent, and the wrong encoding is no encoding. The browser parses HTML, attributes, JavaScript, URLs, and CSS with different rules, and a character that is inert in one context is lethal in another.

- **HTML text context.** Encode `<`, `>`, `&`, `"`, `'`. A value placed between tags needs HTML entity encoding.
- **Attribute context.** Quote the attribute and encode the quote character. Unquoted attributes are an injection vector even with partial encoding; always quote.
- **JavaScript context.** Data placed inside a `<script>` block or an event handler must be encoded for JavaScript string literals, and ideally avoided entirely — pass data through a safe channel (`data-` attributes read by trusted code, a JSON blob consumed by a sandboxed parser) rather than interpolating into script.
- **URL context.** Validate the scheme (allow `http`, `https`, relative; reject `javascript:` and `data:`) and URL-encode the components. A `javascript:` URL in an `href` or `src` is XSS regardless of HTML encoding.
- **CSS context.** Avoid interpolating dynamic data into CSS; CSS injection can exfiltrate data and, in some browsers, execute script through expression or import sinks.

The discipline: name the context the value enters, and apply that context's encoding. Framework auto-escaping covers the HTML text context and sometimes attributes; it does not cover JavaScript, URL scheme, or CSS contexts, which need explicit handling.

### Do Not Rely On Framework Auto-Escaping Without Knowing Its Edge

Modern frameworks (React, Vue, Angular, templating engines) auto-escape interpolation by default, which prevents most reflected and stored XSS in the HTML text context. This is a strong baseline, but it has a hard edge that attackers target: the explicit escape hatches developers use to render rich content.

- **`dangerouslySetInnerHTML`, `v-html`, `innerHTML` bindings, or template `|raw` filters** bypass auto-escaping entirely. Any tainted data reaching these sinks is XSS. Use them only with sanitized content, and audit every use.
- **Attribute and URL contexts** may not be covered by the default escaper. A framework that escapes HTML text may still let a `javascript:` URL through into an `href`.
- **Template injection in server frameworks** can occur when user input is used as the template itself rather than as a data value — the template engine then executes the input as code.

Treat every escape hatch as a review point. The framework protects the default path; the developer who opens the hatch owns the safety of what flows through it.

### Sanitize Rich Content With A Context-Aware Allowlist Sanitizer

When the product requires rendering user-supplied HTML (comments with formatting, rich-text editors, email templates), encoding is not enough — encoding would display the markup as text. The correct control is an allowlist sanitizer that parses the HTML, drops unknown tags and attributes, and removes dangerous schemes and event handlers.

- Use an established, maintained sanitizer (the platform's HTML sanitizer library) rather than regex stripping. Regex cannot correctly parse HTML and routinely misses nested or encoded payloads.
- Configure the allowlist to the minimum the feature needs. Fewer allowed tags and attributes means a smaller attack surface.
- Strip event handler attributes (`onload`, `onerror`, `onclick`, anything starting with `on`) and dangerous attributes that take URLs (`href`, `src`, `style`, `formaction`) unless the scheme is whitelisted.
- Sanitize on output, or sanitize on input and store the sanitized form — but in either case, do not trust the raw form to be safe in any context other than the one sanitized for.

The output of a sanitizer is safe for the HTML context it targeted. It is not automatically safe for a JavaScript or URL context, and it is not a license to feed it to `eval` or a script sink.

### Defend Every State-Changing Action Against CSRF

CSRF exploits the browser's automatic cookie attachment, so any request that mutates state using session cookies as authority is vulnerable unless explicitly defended. The judgment is per-action: does this request change server state on behalf of an authenticated user? If yes, it needs a CSRF defense.

The reliable defenses are layered:

- **SameSite cookies.** Set the session cookie to `SameSite=Lax` or `SameSite=Strict`. `Lax` blocks cookies on cross-site POST subrequests (covering most CSRF) while allowing top-level navigations; `Strict` blocks all cross-site sends. SameSite is now a strong default, but older browsers and certain cross-site flows still require a token.
- **Synchronizer (anti-CSRF) tokens.** The server issues an unguessable token tied to the session, the client submits it with each state-changing request, and the server rejects requests without a valid token. Tokens defend against CSRF regardless of cookie behavior.
- **Require a custom header or content type.** Browsers prevent simple cross-site requests from carrying custom headers or non-simple content types; an endpoint that requires, say, `X-Requested-With` or `application/json` cannot be triggered by a plain HTML form. This is defense in depth, not a primary control, because it depends on the browser's CORS enforcement.

Relying on a single defense is fragile. Prefer SameSite cookies as the baseline plus a token for high-stakes actions, and confirm that every state-changing endpoint — including those reached by AJAX, form POST, and any API that accepts cookie auth — is covered.

### Lock Down Cookie And Header Defaults

Several browser-enforced defenses are configured through response headers and cookie attributes. They are cheap, strong, and routinely forgotten.

- **Cookie attributes.** `HttpOnly` prevents JavaScript from reading the cookie (defeating XSS-based session theft). `Secure` restricts the cookie to HTTPS. `SameSite` defends against CSRF. Set all three on session and authentication cookies.
- **Content-Security-Policy (CSP).** A strong CSP (`default-src 'self'`, no `unsafe-inline`, no `unsafe-eval`, a `nonce` or hash for inline script) contains XSS by blocking unauthorized script execution even when an injection occurs. CSP is a powerful defense in depth; a report-only policy can be deployed first to measure breakage.
- **`X-Content-Type-Options: nosniff`** prevents MIME sniffing that can turn uploaded content into executable script.
- **`X-Frame-Options` or CSP `frame-ancestors`** prevents the page from being embedded by another origin, defending against clickjacking (UI redress), where an attacker overlays invisible UI on a framed victim page to trick the user into clicking authenticated actions.

Set these defaults at the framework or middleware level so they apply to every response, not page by page. A header that is set on most pages but forgotten on one is a hole on that one page.

### Treat PostMessage, Iframes, And Storage As Untrusted Sources

Client-side code receives data through channels that bypass the server entirely, and each is an injection source if fed to a sink.

- **`postMessage`.** Validate `event.origin` before acting on a message, and never pass message data to a DOM sink or `eval` without sanitization. A message from an unverified origin is as untrusted as a URL parameter.
- **Iframes and embedded content.** Embedding untrusted content (ads, user widgets, third-party tools) risks both the embedder and the embedee. Sandbox the iframe (`sandbox` attribute with minimal allow-* tokens) and isolate its origin; for user content, render it from a separate origin so it cannot read the parent's cookies or storage.
- **`localStorage` and `sessionStorage`.** Data in storage is readable by any script in the origin, including any injected script. Do not store secrets or session-equivalent tokens in storage if XSS is possible; prefer HttpOnly cookies for session authority. Treat data read from storage as untrusted when it reaches a DOM sink, because an earlier XSS could have poisoned it.

The pattern across all three: identify the source, assume it can carry attacker-controlled data, and apply the same encoding and sanitization you would apply to data from the URL.

## Common Traps

### Trusting Auto-Escaping Past Its Edge

Assuming the framework makes XSS impossible, then using `v-html`, `dangerouslySetInnerHTML`, or a `raw` filter to render user content and introducing stored XSS. Auto-escaping protects the default interpolation path; every escape hatch is a place where the developer re-assumes responsibility for encoding. Audit each one.

### Encoding For HTML But Placing Data In A URL Or Script Context

HTML-encoding a value and then placing it in an `href`, a `src`, or inside a `<script>` block. HTML encoding does not neutralize `javascript:` schemes or JavaScript string metacharacters. Match the encoding to the exact context the value enters, and validate URL schemes explicitly.

### Allowing `javascript:` Or `data:` URLs In User Links

Accepting a user-supplied URL for a profile link or avatar and rendering it into `href` or `src` without scheme validation. A `javascript:` URL executes on click; a `data:` URL can carry script or HTML. Validate the scheme against an allowlist (`http`, `https`, relative paths) before rendering any user-controlled URL.

### Stripping `<script>` And Calling It Sanitized

Removing the literal string `<script>` from user HTML and believing the result is safe. Attackers bypass this with case variants, nested tags, event handlers on allowed tags (`<img onerror=...>`), encoded payloads, and attributes that take script URLs. Use a real allowlist HTML sanitizer, not substring removal.

### Relying On SameSite Alone For High-Stakes Actions

Setting `SameSite=Lax` and assuming CSRF is solved, when the action is high-stakes (password change, fund transfer, account deletion) and an exception to Lax behavior or an older browser could still allow a forged request. Layer a synchronizer token on high-stakes endpoints so that a single-defense gap does not produce account takeover.

### Forgetting CSRF On AJAX And API Endpoints

Defending form POSTs with tokens but leaving JSON API endpoints that accept cookie auth undefended, on the assumption that "AJAX is safe." Cross-site requests can be forged with custom content types or through browser quirks; any cookie-authenticated state-changing endpoint needs a CSRF defense regardless of how the client calls it.

### Storing Session Tokens In LocalStorage

Placing the session or access token in `localStorage` or a JS-readable cookie so the SPA can read it, which means any XSS can exfiltrate it. Prefer HttpOnly, Secure, SameSite cookies for session authority; if a SPA needs a token, use a short-lived in-memory token backed by a refresh cookie, and accept that XSS still has session-equivalent power within the origin.

### Embedding Untrusted Content Without Sandbox Or Origin Isolation

Rendering user-generated HTML or third-party widgets in an iframe without the `sandbox` attribute or from the main origin, so the embedded content can read cookies, storage, or the DOM of the host application. Embed untrusted content from a separate origin and with a minimal sandbox; never co-locate it with the authenticated origin.

## Self-Check

- [ ] For each dynamic value rendered to the client, the XSS origin (reflected, stored, DOM-based) is identified, and the value is encoded or sanitized for the exact output context (HTML text, attribute, JavaScript, URL, CSS), not a generic escape.
- [ ] No tainted data reaches an escaping bypass (`dangerouslySetInnerHTML`, `v-html`, `innerHTML`, `raw` filter, `document.write`, `eval`) without passing through a context-aware allowlist sanitizer, and every such use was audited.
- [ ] User-supplied URLs rendered into `href`, `src`, or similar are validated against an allowlist of schemes (`http`, `https`, relative); `javascript:` and `data:` URLs are rejected.
- [ ] Rich-text content is sanitized with a maintained allowlist HTML sanitizer configured to the minimum tags and attributes, with event handlers and dangerous URL attributes stripped; regex-based stripping is not used.
- [ ] Every state-changing endpoint (form POST, AJAX, API with cookie auth) is defended against CSRF with SameSite cookies plus synchronizer tokens for high-stakes actions, and no authenticated mutation is left undefended.
- [ ] Session and authentication cookies carry `HttpOnly`, `Secure`, and `SameSite`; session authority is not stored in `localStorage` or JS-readable storage where XSS could exfiltrate it.
- [ ] A Content-Security-Policy is in place that blocks unauthorized script (`default-src 'self'`, no `unsafe-inline`/`unsafe-eval` or a nonce/hash scheme), and `nosniff` and frame-ancestors defenses are set on every response.
- [ ] `postMessage` handlers verify `event.origin`, iframe-embedded untrusted content is sandboxed and origin-isolated, and data read from storage is treated as untrusted before reaching a DOM sink.
- [ ] The code was reviewed against stored XSS (the most damaging variant), DOM sinks that bypass server-side encoding, and CSRF on every authenticated mutation — not only against the reflected-XSS happy path.
