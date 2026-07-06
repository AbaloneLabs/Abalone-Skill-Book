---
name: api-authentication-and-error-triage.md
description: Use when the agent is supporting API authentication, API keys, OAuth, tokens, scopes, permissions, rate limits, HTTP errors, request IDs, SDK failures, API version issues, developer support tickets, or customer integration troubleshooting where risks include exposing secrets, misreading authorization failures, asking for unsafe credentials, giving unsupported code advice, or escalating without the evidence engineering needs.
---

# API Authentication And Error Triage

API support sits between customer development work and product engineering. A support agent may need to help customers diagnose authentication failures, permission issues, rate limits, version mismatches, or request errors without seeing secrets or becoming the customer's developer. Agents often ask for too much sensitive data, over-explain code, or escalate to engineering with incomplete evidence. This skill helps the agent triage API issues safely and usefully.

Use this skill when handling API keys, OAuth, bearer tokens, scopes, SDK errors, HTTP status codes, request IDs, API versioning, rate limits, or integration tickets. The agent should collect reproducible evidence while protecting credentials and staying inside support boundaries.

## Core Rules

### Classify the failure before collecting data

Determine whether the issue appears to be authentication, authorization, request shape, endpoint selection, API version, rate limit, timeout, payload validation, SDK usage, environment mismatch, product bug, or customer code. The evidence needed differs by category.

Do not ask for full codebases or secrets before narrowing the failure. A focused classification prevents unsafe data collection and faster escalation.

### Never request or expose secrets

Do not ask customers to send API keys, bearer tokens, refresh tokens, client secrets, private keys, passwords, or full authorization headers. If logs include secrets, ask the customer to rotate the credential where policy indicates and provide redacted logs instead.

When giving examples, use placeholders. If internal tools expose token fragments or sensitive identifiers, do not copy them into customer-facing replies or broad internal notes.

### Separate authentication from authorization

Authentication proves the request is from a recognized client. Authorization determines what that client can do. A 401, 403, missing scope error, tenant restriction, disabled integration, expired token, or wrong environment can look similar to a customer.

Ask for safe details: auth method, token type, environment, app or integration name, scopes granted, endpoint, timestamp, request ID, and exact error body after redaction. Do not assume an invalid token when the real issue is missing permissions.

### Use request evidence that engineering can trace

Strong API tickets include timestamp with timezone, endpoint, method, request ID or correlation ID, status code, redacted response body, account or tenant identifier allowed by policy, API version, SDK version, environment, and whether the issue is reproducible.

Avoid escalating screenshots of generic errors without trace IDs. Engineering needs enough to find the request without receiving sensitive payloads.

### Watch rate limits and retry behavior

Rate-limit issues may come from customer retry loops, bulk jobs, many users sharing credentials, webhook consumers retrying incorrectly, or a product-side threshold. Explain safe retry and backoff guidance only from approved documentation.

Do not suggest aggressive retries to "push through" errors. That can worsen the outage or trigger abuse controls.

### Bound code and architecture advice

Support can point to documentation, known examples, SDK guidance, and error interpretation. It should not become responsible for debugging the customer's full application architecture unless that is a supported service. When code snippets are used, make them minimal, generic, and caveated.

If a customer requests custom implementation design, route to developer relations, solutions engineering, professional services, or documentation where appropriate.

### Consider product incident and regression signals

If multiple customers report similar API failures, a known endpoint changes behavior, latency spikes, or request IDs show server-side errors, treat it as potential incident or regression. Follow escalation and incident processes rather than handling each case as customer code.

Record pattern evidence: endpoint, time window, account segments, status codes, versions, and customer impact.

### Communicate uncertainty clearly

API triage often has incomplete evidence. Tell the customer what is confirmed, what is likely, what evidence is needed, and what the next owner will do. Avoid definitive statements such as "your code is wrong" unless evidence supports it.

If the next step requires customer developer action, state it precisely and safely.

## Common Traps

- Asking customers to paste API keys, tokens, or full authorization headers.
- Treating every 401 or 403 as the same authentication problem.
- Escalating without request ID, timestamp, endpoint, status code, version, or redacted error body.
- Giving custom code advice that support cannot maintain or verify.
- Suggesting retry behavior that worsens rate limits or service load.
- Ignoring environment mismatch between sandbox, staging, and production.
- Handling a multi-customer regression as isolated customer implementation errors.
- Copying sensitive identifiers or payloads into tickets unnecessarily.

## Self-Check

- Is the failure category classified before broad evidence collection?
- Has the agent avoided requesting or exposing API keys, tokens, secrets, passwords, or private keys?
- Are authentication and authorization possibilities separated?
- Are endpoint, method, status code, timestamp, request ID, API version, SDK version, and environment captured where relevant?
- Are payloads, logs, and screenshots redacted according to policy?
- Is retry or rate-limit advice based on approved documentation?
- Is custom code or architecture advice bounded and routed when outside support scope?
- Are multi-customer patterns checked for incident or regression escalation?
- Is uncertainty communicated without blaming the customer's code prematurely?
- Would engineering be able to trace the issue without receiving unsafe secrets?
