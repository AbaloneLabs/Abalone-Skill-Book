---
name: secrets_in_configuration.md
description: Use when the agent is handling secrets such as API keys, database passwords, tokens, and certificates in application configuration, injecting secrets via environment variables or a vault, managing secret rotation, masking secrets in logs and error output, integrating secrets into CI/CD pipelines, or preventing accidental secret exposure in source control, containers, and crash dumps.
---

# Secrets in Configuration

Secrets are the keys to the kingdom, and they are leaked with a regularity that should be embarrassing but is instead routine, because the number of ways a secret can escape is larger than most engineers track. The classic leak is a secret committed to source control, but the modern leaks are more varied: a secret in an environment variable that ends up in a crash dump sent to a third-party error tracker, a secret passed on the command line visible in the process list, a secret baked into a container image and pulled by anyone with registry access, a secret logged by a framework that helpfully prints all environment variables on startup, a secret in a CI variable that leaks into build artifacts or logs. Agents who think of secret handling as "use an environment variable" have addressed one leak vector and ignored the rest.

The judgment problem is that a secret must be usable by the application but invisible everywhere else, and "everywhere else" is a large and hostile surface. The agent must ensure secrets are injected at runtime from a secure source (not baked into artifacts), that they never transit channels that retain or broadcast them (logs, command lines, error reports, version control), that they can be rotated without downtime and revoked if leaked, and that the blast radius of any single secret is bounded. A secret-handling design that passes a casual review ("the secret is in an env var, not the code") can still leak through logs, process inspection, or image layers. The discipline is to enumerate the leak vectors and close each one, and to assume that any secret will eventually leak and design so that a leak is survivable.

## Core Rules

### Never store secrets in source control, container images, or build artifacts

The absolute baseline. Secrets in version control are leaked forever (history is retained even after deletion, and clones/forks persist them). Secrets baked into container images leak to anyone who can pull the image, and persist across image layers even if removed in a later layer. Secrets in build artifacts (compiled binaries, bundled front-end assets) leak wherever the artifact goes. Inject secrets at runtime from a secure source (a vault, a secret manager, a sealed secret) so that artifacts and source contain no secrets and can be stored, shared, and shipped freely. If a secret has been committed, treat it as leaked and rotate it; do not rely on removing it from history.

### Inject secrets at runtime from a managed secret store

Use a dedicated secret manager or vault (HashiCorp Vault, AWS Secrets Manager, GCP Secret Manager, Kubernetes Secrets with appropriate encryption, SOPS-encrypted files in git) as the source of truth, and have the application or its runtime fetch secrets at startup or on demand. This separates the secret from the code and the artifact, enables rotation without redeployment, and provides an audit trail of access. Avoid the pattern of baking secrets into config files that ship with the application; config files should contain references to secrets, not the secrets themselves. The runtime resolves the reference to the actual value.

### Do not pass secrets through channels that leak: command lines, plain env vars in shared contexts, logs

Each transit channel for a secret is a leak vector. Command-line arguments are visible in the process list to any user on the host. Environment variables are dumped by many frameworks on crash or on verbose startup, and are included in some crash reporters. Logs capture anything printed, and a debug statement or a framework's env-dump will capture secrets. Prefer secret files (mounted, with tight permissions) or direct vault fetches over command-line args. Scrub or redact known secret patterns from logs and error reports, and configure error trackers and observability tools to drop or mask sensitive fields. Assume any secret that touches a logging or reporting channel is leaked.

### Mask secrets in logs, error messages, and crash dumps proactively

Logging frameworks and error reporters are aggressive secret-capture devices: they print environment variables, request headers, and object state on the assumption that more detail helps debugging. This assumption is fatal for secrets. Configure logging to redact known secret keys (authorization headers, password fields, API keys) before output. Be aware that crash dumps and core files may contain the process memory including secrets; configure crash reporting to avoid sending full dumps to third parties, or ensure the reporter redacts secrets. Review what your observability and error-tracking tools actually send; a tool that "helpfully" captures all headers or all env vars is a leak waiting to happen.

### Rotate secrets, and design for rotation without downtime

A secret that cannot be rotated is a permanent liability. Design the system so secrets can be rotated without downtime: support overlapping validity (old and new secret both accepted during a transition window), fetch secrets at runtime so a vault update propagates without redeployment, and avoid hardcoding a single secret value anywhere that would require a code change to rotate. Define a rotation cadence (more frequent for high-value secrets) and an emergency rotation procedure for when a secret is suspected leaked. The ability to rotate quickly is the recovery plan for a leak; a system that requires a multi-day redeploy to rotate a key has no recovery plan.

### Bound the blast radius: scope and isolate secrets

A single secret with broad access is a single point of catastrophic leak. Scope each secret to the least privilege it needs (a database credential for one service, not a shared admin password; an API key scoped to one action, not a master key). Use distinct secrets per environment (production secrets separate from staging, separate from development) so a non-production leak does not expose production. Use distinct secrets per service so compromising one does not compromise all. The goal is that leaking any single secret exposes only the access that one secret grants, not the entire system.

### Audit secret access and alert on anomalies

A secret store should log who accessed what and when, and you should alert on anomalies: unexpected access patterns, access from unfamiliar sources, or bulk reads. This is how you detect a compromise before it becomes a breach. If a secret is fetched by an unexpected service or at an unexpected time, investigate. Treat the secret store's audit log as a security signal, not just an operational record.

## Common Traps

### Committing secrets to version control

Secrets in git history are leaked permanently (clones and forks persist them). Inject at runtime instead, and rotate any secret that was ever committed.

### Baking secrets into container images or build artifacts

Images and artifacts are shared and pulled widely; secrets in them leak to anyone with access. Inject secrets at runtime from a vault.

### Passing secrets as command-line arguments

Process arguments are visible in the process list to any user on the host. Use secret files or direct vault fetches instead.

### Frameworks or error reporters that dump environment variables or headers

A helpful env-dump on crash or a verbose startup log captures secrets into logs and third-party reports. Configure redaction and review what observability tools actually send.

### No rotation plan, so a leaked secret cannot be replaced

A secret that requires a multi-day redeploy to change has no recovery path. Design for rotation with overlapping validity and runtime fetch.

### A single shared secret with broad access

One master key or shared admin password means one leak compromises everything. Scope secrets to least privilege and isolate per environment and service.

### Treating "it is in an env var, not the code" as sufficient

Environment variables leak through logs, crash dumps, and framework env-dumps. Address all leak vectors, not just source control.

## Self-Check

- Are secrets injected at runtime from a managed secret store (vault, secrets manager, SOPS), with source control, container images, and build artifacts free of secrets?
- Are secrets passed via secret files or direct vault fetches rather than command-line arguments, and are command-line arguments avoided for secret values?
- Do logging and error-reporting tools redact known secret keys (authorization headers, password fields, API keys) before output, and have you reviewed what they actually send?
- Can secrets be rotated without downtime (overlapping validity, runtime fetch, no hardcoded values requiring code changes), with a defined rotation cadence and emergency procedure?
- Is each secret scoped to least privilege, with distinct secrets per environment and per service so a single leak is bounded?
- Does the secret store log access, and do you alert on anomalous access patterns (unexpected sources, bulk reads)?
- If a secret were committed to git today, is there a defined rotate-and-scrub procedure, and is it treated as leaked rather than deleted-and-ignored?
- Have you enumerated the leak vectors (source control, images, command line, env-var dumps, logs, crash reports, CI variables into artifacts) and confirmed each is closed?
