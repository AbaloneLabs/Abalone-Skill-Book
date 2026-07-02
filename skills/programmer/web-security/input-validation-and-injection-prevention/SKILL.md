---
name: input_validation_and_injection_prevention.md
description: Use when the agent is handling user input, query parameters, request bodies, headers, file uploads, or any data that crosses a trust boundary; building or reviewing SQL queries, NoSQL queries, OS or shell commands, LDAP or XPath expressions, template rendering, log statements, or config that interpolates external data; choosing between parameterized queries and string concatenation; deciding where to validate and what "validated" means; selecting whitelist versus blacklist validation; planning output encoding; or reviewing code for injection vulnerabilities such as SQL injection, command injection, expression language injection, or second-order injection. Also covers defense in depth, trust boundaries, context-aware encoding, and the failure modes of sanitization and escaping.
---

# Input Validation And Injection Prevention

Injection is what happens when data is treated as code. Every interpreter the program feeds — a SQL engine, a shell, a template renderer, an LDAP server, an expression evaluator, a log pipeline — is a context with its own grammar and its own metacharacters. When untrusted data is concatenated into that context without being confined to a data role, the interpreter cannot tell the data from the instructions, and an attacker who controls the data controls the instruction. The recurring root cause is not a missing filter; it is a missing separation between "this is a value" and "this is a command."

Agents tend to under-invest here because the happy path works: the input they test with does not contain metacharacters, so the query or command looks correct, and they conclude the code is safe. The vulnerability is invisible until an attacker supplies input designed to escape the data role. The harm is severe and direct: full database read or write, arbitrary command execution, authentication bypass, data exfiltration, and lateral movement. These are not theoretical risks; injection remains one of the most exploited vulnerability classes.

The judgment problem is deciding, for each piece of external data and each interpreter it reaches, how to keep the data confined to a value role, where in the flow to validate it, what "validated" actually guarantees, and which defenses are real versus theatrical. The agent has an obligation to be conservative: injection defenses must be correct for the worst-case input, because the attacker is precisely the person who supplies it.

## Core Rules

### Keep Data And Commands Separate At The Interpreter Boundary

The primary defense against injection is not filtering characters; it is using an interface that separates values from the command structure. Every modern data-access layer, shell library, and template engine offers such an interface. Use it unconditionally.

- **SQL.** Use parameterized queries or prepared statements with placeholders for every value. The database driver binds the value as data, so it can never be parsed as SQL. This is correct by construction; string-building queries is correct only by accident.
- **OS commands.** Prefer APIs that take an argument array (exec with a list, never a single shell string) over shell invocation. If a shell is unavoidable, do not interpolate external data into the command string; pass it through environment or a controlled argument list.
- **NoSQL.** Use the driver's query-building API with typed operators rather than constructing query objects from raw input. Be especially careful with operators injected as keys (`$where`, `$gt` in MongoDB), which are a common NoSQL injection vector.
- **LDAP, XPath, expression languages.** Use the library's escaping or parameter binding for that specific grammar. Each interpreter has its own escaping rules; a generic escape does not transfer across contexts.

When an interface forces you to concatenate external data into a command string, treat that as a design smell. The correct fix is almost always to find or build an interface that accepts the value separately.

### Validate At The Trust Boundary, And Define What "Valid" Means

Validation answers "does this input have the shape we expect," and it belongs at the point where data crosses a trust boundary — where untrusted data enters a trusted domain. Validate early, once, against an explicit schema, and then let trusted code assume the shape holds.

Define validation concretely, because "validated" is meaningless without a stated contract:

- **Type and format.** Is it an integer, a UUID, an ISO date, a URL, a JSON object with these fields? Reject anything that does not match.
- **Length and size bounds.** Enforce maximum lengths on strings, counts on arrays, and sizes on uploads. Unbounded input is a denial-of-service surface as well as an injection enabler.
- **Allowed values (whitelist).** For enumerated fields (status, role, sort column, direction), accept only values from a known set. This is especially important for values that become structural, like a `sort` parameter that becomes a column name — a column name cannot be parameterized in SQL, so it must come from a whitelist.
- **Range and business constraints.** Quantities, dates, and identifiers should satisfy range and consistency rules.

Strong validation produces data the rest of the system can reason about. Weak validation — "it's a string, I checked" — leaves every downstream interpreter to defend itself, and one of them will fail to.

### Prefer Whitelist Validation Over Blacklist Filtering

A whitelist accepts only known-good values; a blacklist tries to reject known-bad ones. Whitelists are strictly safer because they fail closed on the unknown, while blacklists fail open on whatever the author did not think of.

- Whitelist a `role` field against `{admin, editor, viewer}` rather than stripping the word "admin" from arbitrary input.
- Whitelist a `sort` column against the actual column set rather than trying to strip SQL keywords.
- Whitelist allowed characters in a username (letters, digits, limited punctuation) rather than stripping a list of dangerous characters.

Blacklists are brittle because the set of dangerous inputs is effectively infinite and evolves with the interpreter. Every new metacharacter, encoding, or grammar feature is a gap the blacklist does not cover. Use a blacklist only as defense in depth on top of a whitelist, never as the primary control. "I removed the quotes and semicolons" is not injection prevention; it is an attacker puzzle.

### Encode For The Specific Output Context

Validation makes input safe to process; encoding makes data safe to emit into an interpreter. Encoding is context-dependent: the escaping rules for a SQL string literal differ from those for HTML text, which differ from those for a URL query parameter, which differ from those for a shell argument. There is no universal "sanitize" function; applying the wrong encoding leaves the data dangerous in the target context.

- Encode at the point of emission, not the point of input. Input-time encoding bakes in assumptions about every downstream context, which change; output-time encoding targets the actual interpreter being fed.
- Use the encoding function built for that context: the database driver's parameter binding for SQL, the template engine's auto-escaping for HTML, a URL encoder for query parameters, the shell library's argument quoting for commands.
- Re-encode when data crosses into a new context. A value safe for HTML is not safe for a SQL query; a value safe for a SQL result is not safe for a shell argument.

The discipline is: know which interpreter the data is about to enter, and apply that interpreter's encoding. Generic "escape HTML" applied to data heading for a shell does nothing useful.

### Model Every Trust Boundary Explicitly

Injection defenses are organized around trust boundaries: the lines data crosses between untrusted and trusted domains. The HTTP request body is untrusted; the parsed, validated, typed object inside your handler is trusted; the value you pass to a parameterized query is trusted as data. Each crossing is a place where validation or encoding must occur.

Map the boundaries for each flow:

- **Network to application.** Every byte from HTTP, webhooks, message queues, file uploads, and third-party APIs is untrusted until validated.
- **Application to interpreter.** Data fed to SQL, shell, template, LDAP, or log must be confined to a value role at this boundary, via parameterization or context encoding.
- **Storage to application.** Data you stored is not automatically trustworthy. Second-order injection occurs when attacker-supplied data is stored cleanly, then later concatenated unsafely into a query or command by code that assumes stored data is safe. Treat stored data with the same caution as fresh input when it heads for an interpreter.
- **Internal service to internal service.** Internal callers are more trusted than the public, but a compromised or buggy internal service can send malicious data. Validate at boundaries that face untrusted upstream sources even inside the network.

State where each boundary is and what control applies there. A flow with no named trust boundary has an implicit, undefended one.

### Treat Sanitization As A Last Resort, Not A Default

"Sanitize the input" is often proposed as a single step that makes data safe everywhere. In practice, sanitization is the weakest control because it tries to neutralize dangerous content while preserving meaning, which is context-dependent and error-prone. Prefer, in order:

1. **Parameterization or binding** — the value never enters the command grammar.
2. **Whitelist validation** — the value is confined to a known-good set.
3. **Context-specific encoding** — the value is escaped for the target interpreter.
4. **Sanitization** — dangerous substrings are removed or transformed.

Sanitization is appropriate only when the data must retain rich content that cannot be whitelisted, such as user-supplied HTML. Even then, use a established, context-aware sanitizer (a HTML allowlist parser) rather than regex-based stripping, and treat the output as still untrusted for any context other than the one it was sanitized for. "I sanitized it" without naming the target context is not a guarantee.

### Apply Defense In Depth, But Do Not Substitute It For The Primary Control

Injection is high-stakes, so layering defenses is prudent — but each layer must be real, and none may substitute for keeping data out of the command grammar. Reasonable layers include: parameterized queries (primary), least-privilege database accounts (limits blast radius), input validation (reduces attack surface), output encoding (covers secondary contexts), and a web application firewall (catches known patterns). Each layer assumes the others may fail.

The trap is treating a secondary layer as sufficient. A firewall that blocks the word `UNION` is defeated by encoding; least privilege limits damage but does not prevent the read of every row the account can see; validation that rejects `<script>` does nothing for SQL. Name the primary control for each interpreter and confirm it is in place before relying on depth.

## Common Traps

### String-Building A Query And Calling It "Parameterized"

Concatenating external values into a SQL string and then executing it through a function whose name contains "prepare," without actual placeholders and bound values. The function name does not make it safe; only true parameter binding confines the value to a data role. Verify that placeholders are used and values are bound, not interpolated.

### Blacklisting A Few Dangerous Characters And Declaring Victory

Stripping quotes, semicolons, or the word "script" from input and treating the result as safe. Attackers bypass this with encoding, alternate characters, grammar features the author missed, and contexts the blacklist was not written for. Blacklists fail open on the unknown; only whitelists fail closed.

### Validating Once At Input, Then Concatenating Unsafely Later

Validating input at the controller, then building a query or command by string concatenation deeper in the code on the assumption that "it was already validated." Validation proves shape; it does not make concatenation safe, because shape does not prevent metacharacters from escaping the data role. Parameterize at the interpreter regardless of earlier validation.

### Second-Order Injection Through Stored Data

Accepting attacker-supplied data, storing it verbatim, and later reading it back into a query or command without parameterization because "it came from our own database." Stored data retains whatever the attacker originally sent. Treat data leaving storage for an interpreter with the same defenses as data arriving from the network.

### Using HTML Escaping For A Non-HTML Context

Applying an HTML entity encoder to data heading for SQL, a shell, or a URL, and believing it is now safe. Each interpreter has its own metacharacters; HTML encoding does not neutralize a quote in SQL or a space in a shell argument. Encode for the specific interpreter the data is about to enter.

### Trusting A "Sanitize" Function As Universal

Calling a single sanitize routine on input and assuming the output is safe for every downstream context. Sanitization is context-specific and lossy; the output is safe only for the context the sanitizer targeted. Re-evaluate and re-encode at each interpreter boundary.

### Whitelisting The Wrong Thing

Whitelisting the value of a structural parameter (like a column name or table name) when only the value can be parameterized. A `sort=...` parameter that becomes a column name cannot be bound as a SQL value; it must be whitelisted against the real column set. Confusing "I validated this string" with "this string is safe as a SQL identifier" leaves an injection point.

### Assuming Internal Callers Are Safe

Skipping validation or parameterization for endpoints called only by internal services, on the assumption that internal code is trusted. A compromised, buggy, or evolved internal service can send malicious data, and an endpoint that is later exposed directly becomes a vulnerability. Validate and parameterize at trust boundaries regardless of who calls them today.

## Self-Check

- [ ] Every value that reaches SQL, NoSQL, shell, LDAP, XPath, a template, or an expression interpreter is passed through parameterization, binding, or that interpreter's native escaping — not concatenated into a command string.
- [ ] Validation happens at the trust boundary against an explicit schema (type, format, length, allowed values, range), and the contract "validated" refers to is documented, not assumed.
- [ ] Enumerated and structural inputs (roles, statuses, sort columns, directions, table names) are validated against a whitelist of known-good values, not filtered by a blacklist of dangerous ones.
- [ ] Encoding is applied at the point of emission for the specific target context (SQL, HTML, URL, shell), and data is re-encoded when it crosses into a new context; no single "sanitize" call is trusted across contexts.
- [ ] Trust boundaries are identified for each flow (network to app, app to interpreter, storage to app, internal service to internal service), and a control is named at each boundary, including defense against second-order injection from stored data.
- [ ] Sanitization is used only where rich content must be preserved, uses an established context-aware sanitizer rather than regex stripping, and its output is still treated as untrusted outside the sanitized context.
- [ ] Defense in depth layers (least privilege, validation, WAF) are present but none substitutes for the primary control of keeping data out of the command grammar; the primary control for each interpreter is named and confirmed.
- [ ] The code was reviewed against adversarial input designed to escape the data role (encoded metacharacters, alternate encodings, second-order payloads), not only against benign test input.
