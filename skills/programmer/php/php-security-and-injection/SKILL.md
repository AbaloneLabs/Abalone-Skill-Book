---
name: php_security_and_injection.md
description: Use when the agent is building PHP web applications, writing SQL queries, rendering user input to HTML, handling file uploads, managing sessions and cookies, deserializing data, configuring php.ini and framework security settings, or diagnosing SQL injection, XSS, CSRF, session hijacking, and insecure deserialization vulnerabilities in PHP applications.
---

# PHP Security and Injection

PHP's history as the language of the early web means most security guidance is "don't trust user input," and that guidance, while true, is too shallow to prevent the bugs that actually ship. The real judgment problem is that PHP applications assemble output (SQL, HTML, shell commands, file paths, serialized data) from a mix of trusted and untrusted input, and the boundary between them is rarely enforced by the type system. A query that uses bound parameters for the WHERE clause but concatenates an ORDER BY column name is still injectable. A template that escapes body text but inserts user input into a JavaScript context is still XSS. A file upload that checks the extension but not the content type or storage path is still arbitrary file write. Security here is about understanding each output context and the correct encoding for it, not about a single "sanitize input" step.

The recurring failure mode is a developer who applies one defense (parameterized queries, or `htmlspecialchars`, or a CSRF token) and assumes the application is secure, while leaving other contexts unguarded. Parameterized queries do not protect identifiers (table/column names) or dynamic SQL fragments. Output escaping for HTML does not protect JavaScript, URL, or CSS contexts. CSRF tokens on forms do not protect same-site JSON endpoints that rely on CORS misconfiguration. The defense must match the specific injection vector, and "I already escaped the input" is the most common precursor to a vulnerability report.

## Core Rules

### Use parameterized queries for all values, and never concatenate identifiers

SQL injection is prevented by binding values as parameters, full stop. The trap is identifiers and dynamic SQL:

- Column names, table names, ORDER BY values, and LIMIT clauses cannot be bound as parameters in most drivers; they must be validated against an allowlist and interpolated.
- Never pass user input directly as an identifier; map a user-supplied sort key to a known-safe column name via a whitelist.
- Query builders (Eloquent, Doctrine, PDO) protect values when used correctly, but methods that accept raw SQL fragments (`whereRaw`, `DB::raw`, native `query()`) bypass parameterization and must be audited.

### Escape output by context, not by a single global function

XSS is prevented by encoding output for the specific context where it lands:

- **HTML body/text**: `htmlspecialchars($val, ENT_QUOTES|ENT_HTML5, 'UTF-8')` or a templating engine that auto-escapes (Twig, Blade `{{ }}`).
- **HTML attribute**: the same HTML escaping, but only inside quoted attributes; never put user input in an unquoted attribute.
- **JavaScript context**: do not interpolate user input into `<script>`; pass it via a data attribute or a JSON blob with proper JSON encoding (`json_encode` with appropriate flags).
- **URL context**: `urlencode` query parameters; validate the scheme/host of any user-supplied URL before using it in a link or redirect.
- **CSS context**: do not interpolate user input into CSS; it is almost never safe.

A single `htmlspecialchars` call does not protect JavaScript, URL, or CSS contexts. Match the encoding to the context.

### Defend CSRF on every state-changing request

CSRF exploits the browser's automatic sending of cookies. Defenses:

- Include a synchronizer token (per-session, per-form) in every POST/PUT/DELETE and verify it server-side. Frameworks provide this (Laravel `@csrf`, Symfony form tokens).
- For stateless APIs using tokens in headers (not cookies), CSRF is not applicable, but then authentication is explicit per request.
- SameSite cookie attributes (Lax/Strict) provide defense-in-depth but are not a complete substitute for tokens on sensitive actions.

### Validate and store file uploads safely

File uploads are arbitrary file write if mishandled. Rules:

- Validate the actual content (mime from content, not from the user-supplied type), size, and extension against an allowlist.
- Store uploads outside the web root, or in a web-root directory that does not execute PHP (disable script execution in the upload dir).
- Generate a new filename server-side; never use the user-supplied filename for storage path.
- Serve user-uploaded content with a `Content-Disposition: attachment` or from a separate domain to prevent stored XSS and HTML execution.

### Manage sessions to prevent fixation and hijacking

- Regenerate the session ID on privilege change (login, privilege escalation) to prevent session fixation.
- Set cookie flags: `HttpOnly`, `Secure` (HTTPS only), and `SameSite`.
- Implement session timeout and server-side invalidation; rely not only on cookie expiry.
- Bind sessions to identifying properties (IP, user-agent) cautiously, as these can cause false invalidation behind proxies.

### Treat deserialization of untrusted data as a critical risk

PHP's `unserialize` on user-controlled data is a well-known remote code execution vector via magic methods and gadget chains. Rules:

- Never `unserialize` user input. Use `json_decode` for untrusted data.
- If you must deserialize internal formats, restrict allowed classes (`unserialize($data, ['allowed_classes' => [...]])`) and treat even that with suspicion.
- Audit dependencies for known deserialization gadgets.

### Lock down configuration and error reporting

- In production, disable `display_errors` and `display_startup_errors`; log errors to a file, never to the output stream.
- Disable dangerous functions and extensions not needed (`eval`, `exec`, `shell_exec`, `system`) via `disable_functions` when the deployment allows.
- Set `open_basedir` and `session.save_path` to restrict file access.
- Keep PHP and all dependencies patched; subscribe to security advisories.

### Authenticate and authorize explicitly, never implicitly

- Verify authentication on every protected route via middleware, not by remembering to call a check in each controller.
- Authorize the specific resource (the user owns this order), not just the action (the user is logged in), using policies/voters.
- Rate-limit authentication endpoints to mitigate brute force.
- Use constant-time comparison for tokens and password hashes (`hash_equals`).

## Common Traps

### Parameterized values but concatenated identifiers

`ORDER BY $_GET['sort']` is injectable even with parameterized WHERE values. Validate sort keys against an allowlist.

### `htmlspecialchars` assumed to cover all XSS

It covers HTML body context. User input in `<script>`, `href`, `style`, or event handlers is still vulnerable. Encode per context.

### CSRF token on forms but not on same-site JSON endpoints

A same-site JSON endpoint that mutates state based on cookies is CSRF-vulnerable. Protect it with a token header or SameSite cookies, or require a custom header the browser will not send cross-site without preflight.

### File upload validated by extension only

A `.jpg` whose content is PHP, stored in a web-accessible directory that executes PHP, becomes arbitrary code execution. Validate content, store outside web root, disable execution.

### `unserialize` on cached or "internal" data

If the data ever crosses a trust boundary (cache poisoning, file tampering), `unserialize` is an RCE vector. Use JSON.

### Session ID not regenerated on login

Failure to regenerate on privilege change enables session fixation. Regenerate and invalidate the old ID.

### Error messages leaking to output in production

`display_errors = On` in production exposes paths, queries, and secrets. Turn it off and log instead.

### Authorization checks on the action but not the resource

"Is logged in" is not "owns this resource." IDOR (insecure direct object reference) lets an authenticated user access other users' data by changing an ID. Authorize the specific resource.

## Self-Check

- Are all SQL values parameterized, and are identifiers (columns, tables, ORDER BY, LIMIT) validated against allowlists rather than concatenated from user input?
- Is output escaped per context (HTML, attribute, JavaScript, URL, CSS), with raw SQL/template methods audited and user input kept out of `<script>` and event handlers?
- Is every state-changing request protected by a CSRF synchronizer token (or, for token-auth APIs, is authentication explicit per request without relying on cookies)?
- Are file uploads validated by content (not just extension/mime), stored outside the web root or in non-executing directories, with server-generated filenames?
- Are session IDs regenerated on privilege change, with `HttpOnly`/`Secure`/`SameSite` cookie flags and server-side timeout?
- Is `unserialize` avoided for any data that crosses a trust boundary, replaced by `json_decode`, with `allowed_classes` restricted where unavoidable?
- Is production `display_errors` off with errors logged, dangerous functions disabled, and `open_basedir`/`session.save_path` set?
- Does authorization check the specific resource (ownership), not just authentication, on every protected route, with rate-limiting on auth endpoints and constant-time token comparison?
