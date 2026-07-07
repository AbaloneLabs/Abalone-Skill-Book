---
name: ai_cost_and_latency_optimization.md
description: Use when the agent is optimizing the cost, latency, or throughput of an LLM feature — routing requests by task complexity to cheaper or distilled models, designing prompt caching or semantic caching to avoid repeat spend, using batch APIs for non-interactive workloads, trading off context length against token price, choosing a smaller model where it is sufficient, streaming to reduce perceived latency, or setting cost and latency budgets that prevent runaway spend and degraded availability. Also covers the failure modes of defaulting every call to the largest model, caching that serves wrong answers, batch jobs that amplify bad inputs, context windows stuffed with cheap tokens that inflate price, and the recurring mistake of treating token cost as a finance concern rather than a correctness and availability risk at scale.
---

# AI Cost And Latency Optimization

In development, an LLM call costs fractions of a cent and returns in two seconds, so cost and latency feel like non-issues. In production, those fractions multiply by traffic, the two-second call sits on the critical path of every request, and the largest model — the one everyone defaults to because it is "best" — is also the slowest and most expensive per token. The judgment problem is that cost and latency are not secondary concerns to optimize later; they are constraints that determine whether a feature is viable, profitable, and reliable, and the decisions that control them (which model, how much context, what to cache, whether to stream) are made early and are hard to undo once they are embedded in the request path.

Agents tend to miss these problems because the single development call hides the economics. A feature that calls the largest model per request looks fine until traffic turns a five-figure monthly bill into an availability incident — rate limits hit, the feature degrades or fails, and cost has become a reliability problem. A prompt that includes the full document history inflates input tokens tenfold with no quality gain. A cache that returns a stale or semantically-matched-but-wrong answer trades a small saving for a correctness regression. The trap is to reach for the most capable model by default and to treat cost as something the finance team notices later. The correct framing is that token spend and tail latency are engineering constraints with budgets, routing, caching, and fallbacks — designed before launch, not apologized for after.

This skill covers model routing, prompt and semantic caching, batch APIs, context-length economics, smaller-model selection, and streaming for perceived latency. It complements the llm-api-integration skill (call mechanics, retries, timeouts) and the prompt-engineering skill (the prompt itself); here the focus is the economic and performance optimization strategy.

## Core Rules

### Route By Task Complexity, Not By Default Model

The single largest cost lever is using a smaller or cheaper model where the task does not require the largest one. Classification, extraction, formatting, summarization of short text, routing, and simple Q&A are routinely solved by small or distilled models at a tenth the cost and latency of the flagship model:

- **Classify the task before choosing the model.** Maintain a routing layer that sends easy tasks to a small model and reserves the large model for genuine reasoning, code generation, or nuanced judgment. The routing decision can be rule-based (by feature/endpoint), a cheap classifier, or a cascade (try small, escalate only on low confidence).
- **Benchmark the cheaper model on the task before routing to it.** A small model that is "good enough" on three examples may regress on the long tail. Validate on the evaluation set (see ai-evaluation-and-benchmarking) and route only where quality holds.
- **Beware cascades that always escalate.** A "try small, fall back to large" pattern saves nothing if the small model fails often and every request escalates anyway, paying for both calls. Measure the escalation rate; if it is high, the small model is the wrong choice for that task.
- **Pin model versions per route.** A routing decision is only stable against a pinned model; a silent provider upgrade can change which tasks the small model handles well.

### Cache To Avoid Paying For The Same Work Twice

Many LLM workloads repeat: identical system prompts, common queries, few-shot prefixes, and the same popular questions. Caching turns repeated spend into a one-time cost, but the cache must be correct, not merely cheap:

- **Use provider prompt caching for static prefixes.** Long system prompts, tool schemas, and few-shot blocks that are identical across requests are often cacheable by the provider at a discount. Structure prompts so the static, cacheable prefix comes first and the variable part last; small changes to the prefix invalidate the cache.
- **Cache exact-match responses for deterministic or low-temperature tasks.** If the same input should yield the same output, store the output and return it. This is safe for deterministic tasks and risky for high-temperature creative ones.
- **Treat semantic caching as approximate, not exact.** A semantic cache (embed the query, return a cached answer for a similar-enough query) can serve a wrong answer when "similar" is not "equivalent." Set a conservative similarity threshold, scope caches by task, and never semantically cache high-stakes or fact-dependent answers. Measure the wrong-answer rate the cache introduces.
- **Bound cache freshness.** A cached answer that was correct yesterday may be stale today (data changed, model improved, policy shifted). Set TTLs and invalidation keyed to the inputs that affect correctness, and version the cache by (model, prompt) so a change does not serve old outputs.

### Use Batch APIs For Non-Interactive Workloads

Many providers offer batch APIs that process large volumes asynchronously at a substantial discount (often half the price) in exchange for latency measured in minutes or hours. For workloads that are not user-facing, this is free money — but it changes failure handling:

- **Route background work to batch.** Bulk classification, embedding generation, document summarization, dataset labeling, and offline enrichment have no real-time requirement; send them as batches and capture the discount.
- **Batch amplifies bad inputs.** A malformed prompt or a runaway output repeated across a million-row batch is a million-row bill, not a one-off. Validate and cap per-item before submitting, and dry-run on a sample.
- **Handle partial failures explicitly.** Batch jobs fail per-item, not all-or-nothing. Define retry and reporting for the failed subset rather than re-running the whole batch (and re-paying for it).

### Treat Context Length As A Cost Lever, Not Just A Limit

Input tokens are priced, and context is the part most under your control. The instinct to "include everything for safety" inflates cost on every call and slows time-to-first-token:

- **Prune context to what the task needs.** Retrieved documents, conversation history, and tool results accumulate; include only the relevant ones. More context is not just more cost, it can also degrade output quality (the "lost in the middle" effect).
- **Summarize instead of carrying full history.** For multi-turn features, summarize older turns once and carry the summary plus recent verbatim turns, rather than re-sending the entire transcript every turn.
- **Prefer cheaper input over cheaper models when the input dominates.** If a request is 90% retrieved context, trimming context saves more than switching models; if it is 90% reasoning, model choice matters more. Optimize the dominant cost component.

### Stream To Reduce Perceived Latency, Not Actual Work

Streaming (server-sent events) does not make generation faster, but it makes it *feel* faster by showing the first token immediately. The value is perceived latency, and the design must respect that:

- **Stream where time-to-first-token matters to the user.** Chat and long-form generation benefit; short structured extractions do not, and streaming adds complexity for no perceived gain.
- **Measure time-to-first-token separately from total time.** A feature can feel fast (low TTFT) while being slow (high total); track both and set targets against the one the UX depends on.
- **Stop generation on disconnect.** Streaming without cancel-on-disconnect burns tokens for output no one reads (see llm-api-integration skill).

### Set Cost And Latency As Budgets With Teeth

Cost and latency must be enforced, not merely observed. A budget that alerts but does not act is a postmortem, not a control:

- **Enforce per-feature and per-tenant spend caps that stop calls, not just log them.** When a tenant hits the cap, the feature degrades gracefully (cheaper model, cached result, rate-limited) rather than producing an unbounded bill.
- **Set latency SLOs against the tail (p95/p99), not the average.** LLM latency is long-tailed; the average hides the unusable worst case.
- **Treat cost spikes as availability incidents.** A runaway loop or an inflating prompt can hit provider rate limits and take the feature down; cost is a correctness and availability risk, not just a finance line item.

## Common Traps

### Defaulting Every Call To The Largest Model

Choosing the flagship model for every request because it is "best," paying ten times the cost and latency for tasks a small model handles identically. Route by task complexity and benchmark the cheaper model first.

### Semantic Cache Serving A Wrong Answer

Returning a cached answer for a query that was merely similar but not equivalent, trading a small saving for a correctness regression that is invisible until a user is misled. Use conservative thresholds, scope by task, and measure the wrong-answer rate.

### Prompt That Invalidates The Cache Every Request

Putting variable content (user ID, timestamp, per-request data) at the start of the prompt so the provider can never cache the static prefix, paying full price every time. Structure prompts static-first, variable-last.

### Batch Job That Amplifies A Bad Input

Submitting a million-row batch where one malformed prompt or uncapped output is repeated a million times, producing a runaway bill and partial corruption. Validate and cap per-item and dry-run on a sample first.

### Stuffing Context "For Safety" And Inflating Every Call

Including all retrieved documents and full history on every request to avoid missing something, which inflates input tokens, slows TTFT, and can degrade quality. Prune to relevance and summarize history.

### Cascade That Always Escalates

A try-small-then-large pattern where the small model fails so often that most requests pay for both calls, saving nothing while adding latency. Measure the escalation rate and drop the small model where it is wrong for the task.

### Cost Budget That Alerts But Never Acts

A spend limit that sends an email but keeps serving calls, so a looping bug or abusive tenant runs up a five-figure bill and then trips rate limits. Enforce caps that degrade or stop the feature.

### Streaming That Does Not Cancel On Disconnect

Continuing to generate after the user navigates away, paying for output no one reads. Wire cancellation to the client connection lifecycle.

## Self-Check

- [ ] Requests are routed by task complexity: easy tasks go to a smaller or distilled model, the flagship model is reserved for genuine reasoning, and the cheaper model was validated on the evaluation set before routing to it.
- [ ] The escalation rate of any small-then-large cascade is measured and low; if it is high, the small model is not used for that task.
- [ ] Caching is correct: static prefixes are structured cache-first for provider prompt caching, exact-match caching is limited to deterministic tasks, and semantic caching uses conservative thresholds scoped by task with a measured wrong-answer rate.
- [ ] Cache entries are versioned by (model, prompt) and bounded by TTL or invalidation keyed to correctness-affecting inputs, so stale answers are not served after a change.
- [ ] Non-interactive workloads use batch APIs for the discount, with per-item validation and capping before submission, a dry-run sample, and per-item failure handling rather than full re-runs.
- [ ] Context is pruned to relevance (not stuffed "for safety"), history is summarized rather than re-sent in full, and the dominant cost component (input vs reasoning) was identified before optimizing.
- [ ] Streaming is used only where time-to-first-token matters to UX, TTFT is measured separately from total time, and generation cancels on client disconnect.
- [ ] Cost and latency are enforced as budgets with teeth: per-feature and per-tenant spend caps degrade or stop the feature (not just alert), and latency SLOs target p95/p99.
- [ ] The highest-risk cases were verified — largest-model-by-default, semantic-cache wrong answers, cache-invalidating prompt structure, batch amplification, context stuffing, always-escalating cascades, alert-only budgets, and disconnect token burn — not only the single cheap development call.
