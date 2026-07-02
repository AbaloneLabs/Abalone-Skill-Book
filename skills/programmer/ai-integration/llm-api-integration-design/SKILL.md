---
name: llm_api_integration_design.md
description: Use when the agent is integrating a large language model (LLM) into a backend service via an API (OpenAI, Anthropic, open-source hosted models), designing the client wrapper, managing token costs and budgets, handling rate limits and 429s, implementing retries with backoff and fallback models, streaming responses (SSE) to clients, managing context window limits and token counting, selecting between models by task, or wiring an LLM call into a production request path with latency and reliability requirements. Also covers the failure modes of unbounded token spend, synchronous LLM calls blocking under load, naive retries that amplify cost, context overflow truncating the wrong end, streaming that loses error semantics, and the recurring mistake of treating an LLM endpoint like a deterministic database query.
---

# LLM API Integration Design

Calling an LLM from a backend is not like calling a database. A database query is fast, cheap, deterministic, and bounded; an LLM call is slow (seconds), priced per token, stochastic, and unbounded in both latency and output length. The judgment problem is that the integration patterns that work for ordinary APIs actively harm here. A synchronous call with no timeout blocks a request thread for 30 seconds while the model generates. A naive retry on failure re-sends a 4,000-token prompt and doubles the cost. A "just send the whole conversation" approach silently overflows the context window and the client truncates the system prompt — the most important part. An LLM endpoint looks like an HTTP API but behaves like an expensive, variable-latency subprocess, and the integration must be designed around that reality.

Agents tend to miss these problems because a single call in development returns in two seconds, costs fractions of a cent, and produces a plausible answer. The harm appears at scale and at the edges. A feature that calls the model per-request multiplies token spend by traffic and produces a surprise bill. A retry loop on a persistent failure re-sends large prompts and amplifies cost without resolving anything. A streaming response that errors mid-stream leaves the client with a half-finished answer and no retry semantics. A context window managed by "trim from the front" silently drops the system instructions and the model starts ignoring the format. The judgment problem is to treat the LLM call as a first-class integration with explicit cost, latency, reliability, and context-budget engineering — not a function call that happens to return text.

This skill covers cost and token management, rate limiting and retries, streaming, context window management, and model selection. It complements the prompt-engineering skill (which covers the prompt itself) and the ai-guardrails skill (which covers validating the output).

## Core Rules

### Budget Tokens Like Money, Because They Are

Every call costs money proportional to input tokens, output tokens, and the model's per-token price. At production traffic, unmanaged token usage becomes the dominant cost of a feature and can spike without warning. Design for cost as a first-class constraint:

- **Count tokens before sending.** Use the model's tokenizer (tiktoken, the provider's tokenizer) to estimate input tokens client-side, and reject or truncate inputs that would exceed a per-request budget before you pay for them.
- **Cap output tokens (`max_tokens`).** An uncapped completion can run to thousands of tokens on a runaway response. Set `max_tokens` to the most the feature needs, not the model's maximum.
- **Set per-feature and per-tenant budgets.** Track token spend per feature and per user/tenant, with hard limits that stop spend rather than silently accumulating. A single abusive user or a bug that loops calls can otherwise produce a five-figure bill overnight.
- **Prefer cheaper models for cheap tasks.** Routing a classification or extraction task to a small model and reserving the large model for hard reasoning can cut cost by an order of magnitude with no quality loss. Route by task, not by default.
- **Cache identical prompts and results.** Many LLM workloads have repeated identical prompts (system prompts, few-shot examples, common queries); caching at the prompt or response level (provider caches, semantic caches with caution) avoids paying twice.

Make cost visible: log token counts per call, aggregate spend per feature, and alert on spend anomalies. A feature whose cost is invisible will eventually surprise you.

### Treat Latency And Timeouts As Primary, Not Incidental

LLM calls are slow and variable: hundreds of milliseconds to tens of seconds depending on prompt size, output length, and provider load. A synchronous call with no timeout is a request-thread killer under load:

- **Set explicit timeouts, shorter than the worst case.** Decide the maximum latency the user experience tolerates and timeout at (or below) it. A 60-second hang is worse than a 5-second timeout with a fallback.
- **Decide synchronous vs asynchronous vs streaming by UX.** A short generation can be synchronous; a long one should stream so the user sees progress; a very long one (report generation) should be asynchronous with a job/poll or webhook model, not a blocking HTTP call.
- **Reserve capacity, do not assume infinite concurrency.** Provider rate limits and your own connection/thread pools bound concurrency. Queue or reject excess load rather than letting requests pile up and time out en masse.
- **Measure p95/p99 latency, not just average.** LLM latency has a long tail; the average looks fine while the p99 is unusable. Track the tail and set SLOs against it.

### Design Retries, Fallbacks, And Rate-Limit Handling Deliberately

LLM APIs fail in specific ways: transient errors (429 rate limit, 500/503 overload), content-policy rejections (400 with a policy flag), and context-window overflows (400). Each needs different handling, and naive retry makes several of them worse:

- **Retry only transient failures, with bounded exponential backoff.** 429 and 5xx are often transient; retry with jittered exponential backoff and a bounded attempt count. Respect the provider's `Retry-After` header on 429s.
- **Do not retry non-transient failures.** A 400 due to content policy, malformed request, or context overflow will fail identically on retry; retrying wastes tokens (on prompt re-send) and money. Distinguish and fail fast.
- **Beware retry cost amplification.** Each retry re-sends the full input prompt and pays for it again. For large prompts, a few retries can double or triple cost. Bound retries tightly and consider whether the prompt can be cached or the retry skipped.
- **Define fallback behavior.** When the primary model fails or times out, what happens? Fallback to a smaller/cheaper model, a cached result, a rule-based default, or a graceful error. Decide explicitly; the worst fallback is an unhandled exception surfaced to the user.
- **Implement idempotency for stateful effects.** If the LLM output triggers a side effect (writing to a database, sending an email), ensure retries do not duplicate the effect. Use idempotency keys or check-before-act.

### Stream Long Responses, But Preserve Error Semantics

Streaming (server-sent events) is essential for long generations: it lets the user see tokens as they arrive instead of waiting for a full completion. But streaming changes error handling in ways that are easy to get wrong:

- **Errors can arrive mid-stream.** A stream that starts successfully can fail halfway (provider overload, content policy triggered on later tokens, network drop). The client may have already rendered partial output. Define the contract: can the client retry, is partial output usable, is the error surfaced?
- **Surface stream errors explicitly.** Many streaming implementations swallow mid-stream errors or hang. Forward error events to the client and close the stream cleanly on failure.
- **Handle client disconnects.** If the client disconnects mid-stream, stop generating (cancel the upstream call) rather than burning tokens for output no one will read. Many providers support cancellation; wire it to the client connection lifecycle.
- **Decide streaming vs non-streaming per endpoint.** Not every call benefits from streaming (short extractions don't), and streaming adds complexity. Use it where latency-to-first-token matters to UX.

### Manage The Context Window Explicitly — Truncate The Right Thing

Every model has a context window (input + output tokens). Exceeding it is a hard error or a silent truncation, and the default truncation often removes the most important content:

- **Know the window and count against it.** Track total tokens (system + prompt + history + retrieved context + reserved output) against the model's limit before sending.
- **Truncate the least-important content, not the front.** The naive "trim oldest messages" can drop the system prompt or the original user request. Preserve the system prompt and the current user turn; trim history and retrieved context first, and within those, trim the oldest or least-relevant.
- **Reserve output budget.** The context window is shared between input and output; if you fill it with input, there is no room for the response. Reserve `max_tokens` for output and cap input accordingly.
- **Decide summarization vs truncation for long history.** For long conversations, summarizing older turns (and keeping recent ones verbatim) preserves more signal than blunt truncation, at the cost of an extra model call.

### Choose The Model By Task, Latency, And Cost — Not By "Best"

Model selection is a real engineering decision with tradeoffs, not a quest for the single best model:

- **Match model capability to task difficulty.** A small, fast, cheap model handles classification, extraction, formatting, and simple Q&A; a large model is justified for complex reasoning, code generation, or nuanced judgment. Over-provisioning wastes cost and latency; under-provisioning produces wrong answers.
- **Latency and cost scale with size.** A large model is slower and more expensive per token; if a small model solves the task, use it. Benchmark candidates on a representative task set, not on a single impressive demo.
- **Consider determinism needs.** If the task needs structured/parseable output, prefer models and settings that support structured output (JSON mode, function calling) and lower temperature, not the most creative model at high temperature.
- **Plan for model deprecation and versioning.** Providers deprecate models; pin to a specific version, abstract the model choice behind your client, and have a plan to migrate. Hardcoding `gpt-4-latest` into business logic couples you to a moving target.

## Common Traps

### Synchronous LLM Call With No Timeout Blocking Under Load

Calling the model synchronously in a request handler with no timeout, so under provider latency spikes every request thread hangs and the service appears down. Set explicit timeouts and choose sync/async/streaming by UX.

### Naive Retry That Re-Sends Large Prompts And Amplifies Cost

Retrying any failure with the full prompt, so a persistent error re-bills the input tokens on every attempt. Retry only transient failures, bound attempts, and recognize that retries cost money.

### Retrying Non-Transient Failures (Policy, Overflow)

Retrying a 400 content-policy rejection or context overflow, which fails identically and wastes tokens. Distinguish transient from non-transient and fail fast on the latter.

### Uncapped Output Tokens On A Runaway Response

Leaving `max_tokens` unset or at the model maximum, so a degenerate response runs to thousands of tokens. Cap output at what the feature needs.

### Context Overflow Truncating The System Prompt

Letting the context window overflow and the default trimming drop the system instructions, so the model starts ignoring format and rules. Count tokens, reserve output budget, and truncate history/retrieved context first.

### Streaming That Swallows Mid-Stream Errors

A streaming endpoint that starts returning tokens, fails halfway, and leaves the client with partial output and no error signal. Forward stream errors and define the partial-output contract.

### Burning Tokens After Client Disconnect

Continuing to generate after the client disconnects mid-stream, paying for output no one reads. Cancel upstream generation on client disconnect.

### Routing Everything To The Largest Model

Defaulting every call to the most capable (and most expensive, slowest) model, when a small model would handle most tasks at a fraction of the cost and latency. Route by task.

### No Per-Tenant Budget, So One User Creates A Huge Bill

A feature with no per-user or per-tenant spend cap, so an abusive user or a looping bug generates unbounded token spend. Set and enforce budgets with hard limits.

### Hardcoding A Floating Model Version

Pinning business logic to `gpt-4-latest` or equivalent, so a silent provider upgrade changes behavior in production. Pin specific versions, abstract model choice, and plan migrations.

## Self-Check

- [ ] Tokens are budgeted: inputs are counted before sending, output is capped with `max_tokens`, per-feature and per-tenant spend limits are enforced with hard caps, and spend is logged and alerted on.
- [ ] The cheapest capable model is routed per task (small models for extraction/classification, large for hard reasoning), chosen by benchmarking on a representative set, not by default.
- [ ] Latency is engineered: explicit timeouts below the worst case, sync/async/streaming chosen by UX, concurrency bounded, and p95/p99 (not just average) tracked against an SLO.
- [ ] Retries target only transient failures (429/5xx) with bounded jittered backoff and respect `Retry-After`; non-transient failures (policy, overflow, malformed) fail fast without re-sending the prompt.
- [ ] Fallback behavior is defined (smaller model, cached result, rule-based default, or graceful error), and side effects are idempotent against retries.
- [ ] Streaming endpoints surface mid-stream errors to the client, define the partial-output contract, and cancel upstream generation on client disconnect.
- [ ] The context window is managed explicitly: total tokens counted against the limit, output budget reserved, and truncation removes history/retrieved context (never the system prompt or current user turn), with summarization considered for long history.
- [ ] Model versions are pinned and abstracted behind the client, with a migration plan for deprecations, rather than hardcoded floating versions.
- [ ] The highest-risk cases were verified — an untimed synchronous call under load, retry cost amplification, context overflow dropping the system prompt, mid-stream error swallowing, token burn after disconnect, and unbounded per-tenant spend — not only the single happy-path development call.
