---
name: secret_management.md
description: Use when the agent is handling application secrets such as API keys, database passwords, signing keys, OAuth client secrets, service-account credentials, TLS private keys, or third-party tokens; deciding where secrets live (environment variables, config files, secrets manager, KMS, HSM); rotating or revoking secrets; scoping secret access by least privilege; detecting or preventing secret leakage in logs, source control, container images, crash dumps, or CI/CD output; masking secrets in observability; injecting secrets into applications at runtime; or responding to a suspected secret exposure. Also covers the difference between secrets and keys, secret rotation blast radius, short-lived vs long-lived credentials, and why hardcoding or committing secrets is a defect even in non-production.
---

# Secret Management

A secret is any value whose secrecy is its entire security value — an API key, a database password, a signing key, an OAuth client secret. The moment an attacker obtains it, they hold whatever authority it grants, and most secrets grant authority that is broad, long-lived, and hard to revoke. The defining failure of secret management is not a weak algorithm or a clever attack; it is the secret sitting somewhere it should not — committed to source control, baked into a container image, printed in a log line, echoed in a crash dump, passed through an environment variable that leaks into a process listing — and staying there, discoverable, until it is used against you.

Agents tend to under-invest here because the application works the moment the secret is present. Hardcoding a test key makes the build pass; putting the production database password in an environment variable makes the service start; logging the full request "to debug" reveals the Authorization header. None of these break the happy path, and all of them create a durable exposure that outlives the feature. The judgment problem is treating every secret as a liability with a lifecycle — generated, distributed, used under least privilege, rotated, and retired — rather than as a string you paste wherever the code needs it.

The stakes are severe and asymmetric. A leaked source file is a code-disclosure incident; a leaked source file containing a hardcoded API key is a production compromise. Secrets are the highest-leverage artifact an attacker can find, and they are the artifact most often left lying in plain sight.

## Core Rules

### Never Hardcode, Commit, Or Embed Secrets In Artifacts

The baseline rule, violated more often than any other: secrets must not live in source code, configuration files committed to version control, container images, build artifacts, infrastructure templates, or any artifact that is stored, shared, or reproduced. Once a secret is in version control, it is effectively permanent — it persists in history even after deletion, and history is cloned to developer machines, CI runners, mirrors, and forks.

- **Treat any committed secret as exposed, not as "still secret because we deleted it."** Git history retains it; rotate the secret, do not merely remove the line.
- **Do not bake secrets into container or machine images.** An image is widely distributed and long-lived; a secret in an image is a secret given to everyone who can pull it.
- **Do not put secrets in infrastructure-as-code templates committed to a repository**, even templated ones, unless the template references a secrets manager and never contains the value.
- **Differentiate non-secret config from secrets.** Feature flags, URLs, and tuning parameters may live in config; credentials and keys must not.

The strong pattern is that the artifact never contains the secret; the secret is injected at runtime from a secrets manager or KMS, and the artifact contains only a reference. If you can reconstruct the secret by reading the repository or pulling the image, the design is wrong.

### Choose The Storage Backend By The Secret's Sensitivity And Lifecycle

Where a secret lives at rest determines who can reach it, how it is rotated, and what a compromise of the store means. From strongest to weakest:

- **Hardware Security Module (HSM) or cloud KMS.** The key never leaves the hardware; operations (sign, decrypt) happen inside it. Strongest protection, and the right home for master keys and root signing keys. Use envelope encryption so application data keys are wrapped by a KMS-resident master key.
- **Secrets manager (Vault, AWS Secrets Manager, GCP Secret Manager, cloud KMS-backed stores).** Centralized, access-controlled, auditable, with rotation and versioning. The right default for application secrets in production.
- **Encrypted config with keys from KMS.** A step down: config files encrypted at rest, decrypted at startup with a KMS key. Better than plaintext, weaker than a secrets manager (no central audit, no native rotation).
- **Environment variables or plaintext config.** The weakest acceptable option, and only for low-stakes or local development. Environment variables leak into process listings, crash dumps, container inspect output, child-process inheritance, and debugging tools. Never use them for high-value production secrets.

The decision should match the secret's value. A throwaway dev API key in an environment variable is tolerable; the production database password in an environment variable is a defect. Prefer a secrets manager as the production default, and envelope encryption for data keys so the master key never reaches the application.

### Apply Least Privilege To Secret Access

A secret grants authority; access to the secret grants that authority. Scope who and what can read each secret as narrowly as the function requires:

- **Per-application, per-environment scoping.** The staging service should not read production secrets; the billing service should not read the email-service credentials. Each secret is readable only by the identity (service account, role) that needs it.
- **Read vs use distinction.** Where possible, prefer "use" over "read": the application asks KMS to sign or decrypt with the key, rather than fetching the key material. The key never enters the application's memory.
- **Audit access.** Every secret read should be logged with who, what, when. Unexplained reads are the signal of compromise or misuse.
- **Human vs machine access.** Humans rarely need direct access to production secrets; they need break-glass access under approval and audit. Routine operations should run through the application's own scoped identity.

A secrets manager that grants every service read access to every secret provides centralization without isolation — the blast radius of any compromise is total. Least privilege is what makes a secrets manager worth operating.

### Plan Rotation Around Blast Radius, Not A Schedule

Rotation limits the damage of a single secret's exposure: a rotated secret is worthless to an attacker who captured the old one. But rotation is only useful if it is designed to be survivable, and most rotation designs fail because they require coordinated, downtime-inducing changes. Design rotation around its blast radius:

- **Prefer secrets that can rotate without coordination.** Short-lived credentials issued by a central authority (OAuth tokens, cloud instance metadata credentials, SPIFFE/SPIRE, STS tokens) rotate automatically and frequently because the consumer just fetches a new one. This is strictly better than long-lived shared secrets.
- **Version keys rather than replacing them in place.** Support multiple active versions (old and new) during a rotation window so consumers can move to the new key without a synchronized cutover; retire the old version only after all consumers have migrated.
- **Isolate the blast radius of rotation failure.** If rotation fails, the old secret should remain valid (graceful degradation), not leave the system unable to authenticate.
- **Make rotation the easy path.** If rotating a secret requires a coordinated deploy, a maintenance window, and a runbook that no one remembers, it will not happen on schedule. Automate rotation where the secrets manager supports it.

The strongest designs minimize long-lived shared secrets in favor of short-lived, automatically-rotated, centrally-issued credentials. Every long-lived secret that must be manually rotated is a rotation that will eventually be overdue.

### Prevent Secret Leakage In Logs, Errors, And Observability

Most secret exposures are not breaches of the secrets manager; they are secrets printed by the application itself into logs, error messages, traces, or metrics. The defenses are systematic, not per-line:

- **Redact at the logging boundary.** Configure loggers and observability libraries to redact known secret fields (Authorization headers, password fields, API keys) by name or pattern, at the shipping boundary, so a developer who logs a struct does not leak its secrets.
- **Never log full request or response bodies, headers, or environment dumps** in systems that handle credentials. A "debug" log of the incoming request is the most common source of leaked Authorization headers and cookies.
- **Beware error messages that include context.** An exception that captures the arguments or the connection string can write a secret into the error log and the crash report. Sanitize before serialization.
- **Beware string interpolation of config.** Logging "connecting to postgresql://user:PASSWORD@host" leaks the password that was loaded from the secret store. Log the host, not the full DSN.
- **Scrub crash dumps, core dumps, and telemetry.** These can contain the process's memory, including secrets. Treat them as sensitive, or disable them where secrets are present.

Treat observability as a secret-leakage surface, not just a debugging tool. The logging skill covers PII redaction; here the concern is credentials, tokens, and keys, which must never appear in any shipped output.

### Treat CI/CD As A Secret-Handling Environment

CI/CD pipelines handle secrets at scale — deploy credentials, registry tokens, signing keys, cloud credentials — and they are a frequent exposure point. The pipeline runs third-party actions, executes arbitrary code, produces logs, and caches state, all of which can leak secrets:

- **Scope pipeline secrets to the job that needs them, and mask them in logs.** Most CI systems can mask known secret values in output; use it, and understand its limits (a secret transformed or split is not masked).
- **Do not pass secrets to untrusted steps.** A third-party action with access to the job's environment can exfiltrate secrets to an external endpoint. Pin actions to a version, review them, and give secret access only to trusted steps.
- **Do not write secrets into build artifacts, caches, or intermediate files** that persist beyond the job. A secret written to a cached layer is a secret given to the next job that reads the cache.
- **Use short-lived, scoped deploy credentials.** A long-lived deploy key with broad cloud access, stored in CI, is a high-value target. Prefer OIDC-federated, per-job, short-lived credentials over static keys.
- **Scan for committed secrets in CI.** Run secret-scanning on pull requests and pushes so a committed secret is caught before merge, not after it has been cloned.

CI/CD is where secrets are most concentrated and where third-party code runs closest to them. Treat the pipeline as a production secrets environment, not as a build tool.

## Common Traps

### Hardcoding "Just For The Demo" Or The Local Build

Pasting a real API key into source to make the demo or the local build work, then forgetting to remove it, or removing it from the working copy but not from history. Treat any hardcoded secret as committed-and-exposed; rotate it and move it to a secrets manager.

### Environment Variable Treated As "Secure Enough" For Production

Using environment variables for production database passwords or signing keys because "they're not in the code." Environment variables leak into process listings (`ps e`), container inspect output, crash dumps, child-process inheritance, and debugging tools. Use a secrets manager or KMS for production secrets.

### Logging The Request Body Or Headers "For Debugging"

Adding a debug log of the incoming request — including the Authorization header or cookies — that ships to the aggregation backend, where it is stored, indexed, and readable by anyone with log access. Redact secrets at the logging boundary; never log full headers or bodies in credential-handling systems.

### Rotation That Requires Coordinated Downtime

Designing rotation as "change the secret everywhere at once," so it requires a maintenance window and a multi-team runbook — and therefore never happens on schedule. Version keys with overlap, or move to short-lived automatically-rotated credentials.

### Secrets Manager With Broad Read Access

Centralizing secrets in a manager but granting every service read access to every secret, so one compromised service can exfiltrate all of them. Scope access per-application, per-environment; prefer "use" over "read" where the backend allows.

### Committed Secret "Deleted" But Not Rotated

Removing a secret from the current code and believing it is safe, when it remains in git history, in clones, and possibly already exfiltrated. A committed secret must be rotated, not just removed.

### Long-Lived Deploy Key In CI

Storing a long-lived, broad-scope cloud key in CI secrets for deploys, where any job (including third-party actions) can read it. Use OIDC-federated, per-job, short-lived credentials; scope static keys narrowly and rotate them.

### Error Or Crash Dump That Serializes Secrets

An exception handler that serializes the connection string, the request, or the function arguments into the error message — and the error message into the log or crash report. Sanitize objects before serialization; never let raw config or request state reach an error report.

## Self-Check

- [ ] No secret is hardcoded, committed to version control, baked into a container/machine image, or embedded in an infrastructure template; artifacts contain only references, and secrets are injected at runtime.
- [ ] The storage backend matches the secret's sensitivity (KMS/HSM for master keys, secrets manager for production application secrets), and environment variables are used only for low-stakes or local development — never for high-value production secrets.
- [ ] Secret access is scoped per-application and per-environment under least privilege, access is audited, and "use" (sign/decrypt in KMS) is preferred over "read" (fetching key material) where the backend allows.
- [ ] Rotation is designed to be survivable (versioned keys with overlap, or short-lived automatically-rotated credentials), and every long-lived secret has a rotation path that does not require coordinated downtime.
- [ ] Logs, error messages, traces, crash dumps, and telemetry are configured to redact secrets at the shipping boundary; full request/response bodies and headers are not logged in credential-handling systems; connection strings and config are not interpolated into log lines.
- [ ] CI/CD secrets are scoped per-job, masked in logs, never passed to untrusted steps or written to caches/artifacts, and deploy credentials are short-lived and OIDC-federated rather than long-lived static keys; secret scanning runs on commits.
- [ ] Any previously committed or exposed secret has been rotated (not merely removed), and the team treats git history as a permanent exposure surface.
