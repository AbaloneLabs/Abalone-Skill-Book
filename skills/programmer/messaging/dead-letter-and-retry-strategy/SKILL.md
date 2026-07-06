---
name: dead_letter_and_retry_strategy.md
description: Use when the agent is designing or reviewing the retry and dead-letter strategy for an asynchronous messaging system, choosing retry counts and backoff policies for queue consumers, configuring dead-letter queues (DLQ) and their retention and enrichment, detecting and quarantining poison messages, deciding what happens after retries are exhausted (DLQ versus discard versus alert), defining DLQ monitoring and triage ownership and replay tooling, or reasoning about retry storms and cascading failure when a downstream dependency is degraded. Also covers exponential backoff with jitter, per-topic versus shared DLQ design, preserving original metadata and error context, circuit-breaking a failing consumer, and the interaction between retries and consumer idempotency.
---

# Dead Letter And Retry Strategy

Retries and dead-letter queues are the failure-handling layer of an asynchronous messaging system, and they are too often treated as configuration to fill in at the end rather than as a first-class design concern. The defaults look harmless — "retry a few times, send failures to a DLQ" — but the wrong policy turns a transient blip into a sustained outage, turns a single bad message into an infinite loop that starves the queue, or turns the DLQ into a silent graveyard where business-critical messages disappear forever. The judgment problem is not whether to retry, but how to retry, when to stop, where failures go, and who is responsible when they land there.

Agents tend to under-invest here because the happy path hides every decision: a message fails once, retries succeed, life goes on. The harm appears only under failure, and it is disproportionately large. An aggressive retry policy against a degraded downstream multiplies the downstream's load manyfold and prevents its recovery — a retry storm that turns a partial outage into a total one. An unbounded retry loop on a poison message blocks a partition and stalls every message behind it. A DLQ with no alerts and no owner accumulates failed orders, failed payments, failed notifications until someone notices weeks later, by which point the business impact is real and the original context is gone. The strategy must be designed against these specific failure modes, not configured against a template.

This skill is the messaging-specific treatment of retry and dead-letter strategy. It goes deeper than the failure-handling section of the message-queue-design-and-delivery skill (which establishes the need for bounded retries and a DLQ) and is distinct from the reliability retries-timeouts-circuit-breakers skill (which covers synchronous HTTP retry policy). Here the questions are specific to asynchronous consumers: retry policy for queue messages, DLQ design and enrichment, poison-message quarantine, and the operational lifecycle of a failed message from first failure to resolution.

## Core Rules

### Treat Retry Policy As A Per-Message-Type Decision, Not A Global Default

Different messages fail for different reasons and tolerate different retry behavior, so a single global retry policy is almost always wrong somewhere. The policy must be chosen per message type against the failure modes that type actually experiences.

- **Transient failures warrant retries.** A downstream that is briefly unavailable, a database connection blip, a rate-limit response — these will likely succeed on retry. Retry them, with backoff, a bounded number of times.
- **Permanent failures do not warrant retries.** A message that cannot be parsed, references a missing record, or triggers a consumer bug will fail every time. Retrying it forever burns capacity and blocks the queue. These belong in a DLQ or quarantine immediately, possibly after a single attempt to confirm the failure is not transient.
- **The retry count must be bounded.** Decide the maximum attempts per message type, and make the bound explicit. Unbounded retries are never correct; they convert every persistent failure into an infinite loop. A typical range is three to ten attempts for transient failures, fewer for low-latency workloads.
- **Fail-fast where latency matters.** For time-sensitive messages (a real-time notification, a fraud check that must complete in seconds), a long retry tail is worse than a fast failure to the DLQ. Tune the policy to the message's useful lifetime, not to a generic "be resilient" instinct.

The judgment call is classifying each message type's failures as transient or permanent, which requires knowing the failure modes. A policy designed without that knowledge will retry permanent failures and DLQ transient ones — both wrong.

### Use Exponential Backoff With Jitter, And Cap It

When you retry, the spacing between attempts matters as much as the count. Fixed-interval retries concentrate load, and naïve exponential backoff without jitter causes synchronized retry storms.

- **Exponential backoff spaces retries increasingly far apart**, so transient failures get a few quick chances and then progressively longer waits, avoiding hammering a recovering dependency. Without a cap, however, the last retry can be scheduled hours or days later, which is useless for most workloads.
- **Cap the backoff** at a maximum delay that is meaningful for the message's lifetime. A cap of seconds to low minutes is typical; a multi-hour backoff on a message that should be resolved in seconds is a sign the policy does not match the workload.
- **Jitter is mandatory, not optional.** When many consumers fail simultaneously against the same downstream (a common case during a partial outage), deterministic backoff causes them all to retry at the same computed intervals, producing synchronized spikes that prevent the downstream from recovering. Jitter spreads retries randomly across the backoff window.
- **Full versus equal jitter.** Full jitter picks a random delay between zero and the exponential value (maximum spread, slowest individual recovery). Equal jitter picks half the exponential value plus a random half (less spread, faster average recovery). Full jitter is the safer default for protecting a fragile downstream; equal jitter trades some synchronization risk for faster recovery. Choose based on whether protecting the downstream or recovering quickly is the priority.

The default for an unknown workload: exponential backoff with full jitter, a cap in the seconds-to-minutes range, and a bounded retry count. Deviate only with a reason grounded in the message's failure modes and lifetime.

### Design The Dead-Letter Queue As An Operational Artifact, Not A Trash Can

A DLQ is where messages go when retries are exhausted, and its design determines whether failed messages are recoverable or effectively lost. The trap is treating the DLQ as a place to put things and forget them.

- **Decide when to DLQ.** The trigger is exhaustion of retries for a transient failure, or immediate detection of a permanent failure. The DLQ is for messages that cannot be processed now and need intervention — not for messages that were deliberately skipped.
- **Enrich the message with failure context.** A raw dump of the original message is nearly useless for triage. The DLQ entry must preserve the original message and metadata (original topic, partition, offset, timestamp, message id, correlation id) and add the failure context (error type, error message, stack trace or code, attempt count, timestamps of each attempt, the consumer that failed). Without this enrichment, triage requires reconstructing context that may no longer exist.
- **Choose per-topic versus shared DLQ deliberately.** A per-topic DLQ keeps failures scoped and easy to route to the owning team, but multiplies the number of queues to monitor. A shared DLQ centralizes monitoring but requires routing and risks one team's failures drowning out another's. Per-topic (or per-consumer-group) is usually better for ownership clarity; shared is acceptable only with strong routing and alerting.
- **Set retention deliberately.** DLQ messages must live long enough to be triaged and replayed, but not so long that the DLQ grows unbounded. Time-based retention in days to weeks is typical; the right value depends on how quickly the team responds. A DLQ with no retention limit and no alerts will eventually exhaust broker storage.

The DLQ is an operational artifact with an owner, a retention policy, an alerting threshold, and a replay path. If any of those is missing, the DLQ is a graveyard.

### Detect And Quarantine Poison Messages Quickly

A poison message is one that will always fail — a malformed payload, a reference to a missing record, a message that triggers a consumer bug. Retrying it is pure waste, and the longer it sits in the retry loop the more it blocks the queue behind it.

- **Detect poison messages by failure signature.** Parse errors, schema validation failures, missing-reference errors, and repeated identical failures across attempts are signatures of a poison message, not a transient failure. A consumer that distinguishes these can route them to quarantine immediately rather than retrying.
- **Quarantine is distinct from DLQ in intent.** A poison-message quarantine is for messages that are structurally unable to be processed as-is and need either a code fix, a data fix, or human review; a retry-exhaustion DLQ is for messages that might succeed after the downstream recovers. Conflating them mixes messages needing different responses. Where the infrastructure supports it, separate them; where it does not, at least tag them distinctly in the DLQ.
- **Apply a circuit breaker to a failing consumer.** If a consumer is failing on every message — not just one poison message but all of them — the consumer itself is broken (a bad deploy, a dependency outage). Continuing to pull, fail, and DLQ floods the DLQ and wastes capacity. A circuit breaker that pauses consumption after a sustained failure rate lets the system degrade gracefully rather than amplifying the failure.

The judgment call is distinguishing "this one message is bad" from "this consumer is broken," because the responses are opposite: quarantine the message versus stop the consumer.

### Define The Decision And Ownership After Exhaustion

Retries exhaust, and the message lands in the DLQ. What happens next is an operational decision that must be designed, not improvised.

- **Define the post-exhaustion options explicitly.** A failed message can be replayed after a fix (the common case for transient-caught-late or code-bug-then-fix), discarded as invalid (genuinely malformed or duplicated data), routed elsewhere (a different handler or a manual workflow), or escalated to a human. The system must support all of these, not silently assume replay.
- **Assign DLQ ownership.** Every DLQ must have an owning team responsible for triage, and that ownership must be discoverable from the DLQ itself (in metadata or naming). A DLQ with no owner is a DLQ no one monitors.
- **Build or provide replay tooling.** Replay is the most common DLQ resolution, and it must be safe: replay must preserve the original message identity so consumers deduplicate (see the idempotency rule below), and it must be able to target a subset of messages rather than blindly re-injecting the whole DLQ. Without replay tooling, the DLQ is a one-way destination.
- **Alert on DLQ depth and age, not just existence.** A DLQ with one old message is normal; a DLQ growing by the minute is an incident. Alert on depth thresholds and on the age of the oldest message, and page the owning team. A DLQ with no alerts is effectively data loss.

### Make Retries Safe Through Consumer Idempotency

Retries deliver duplicates. A message that fails after its side effect partially completed, then retries, will re-perform the side effect unless the consumer is idempotent. This is covered in depth in the message-queue-design-and-delivery skill, but it has specific implications for retry and DLQ strategy that belong here.

- **Every retried consumer must be idempotent.** Charge, email, inventory decrement, external API call — each must deduplicate by a stable key derived from the message, so a retry or a replay from the DLQ does not double the effect. This is not optional and not the broker's responsibility.
- **Replay from the DLQ must preserve message identity.** When you replay a failed message, it must carry the same id it originally carried, so the consumer's deduplication treats it correctly. A replay that generates a new id defeats idempotency and risks duplicate effects.
- **Side effects and acknowledgement must be coupled.** Commit the side effect and the message acknowledgement together (transactionally or via the outbox pattern), so a crash between them does not leave a performed effect to be re-performed on retry. This is the single most common source of duplicate effects under retry.

### Reason About Retry Storms And Cascading Failure

The most dangerous failure mode in a retrying system is the retry storm: a downstream dependency degrades, every consumer's requests start failing, and the retry policy multiplies the load against the already-struggling downstream, preventing its recovery and spreading the outage. This is the asynchronous analog of the synchronous cascading-failure problem (covered in the retries-timeouts-circuit-breakers skill), and it must be designed against explicitly.

- **Aggressive retries amplify downstream load.** If every consumer retries three times with short backoff, a downstream outage triples or quadruples the request rate against it at the worst possible moment. The multiplication factor is roughly the retry count; the synchronization from lack of jitter can make it worse.
- **Backoff and jitter are the primary defense**, but they may not be enough if the downstream is fully down. Combine them with a circuit breaker that pauses retries when the downstream is consistently failing, so the system backs off as a whole rather than each consumer independently hammering.
- **Bounded concurrency protects the consumer.** A consumer that retries unboundedly in parallel can exhaust its own database connections or thread pool while retrying, turning a downstream outage into a self-inflicted consumer outage. Bound the consumer's concurrency and treat retry as part of that budget.
- **Consider fail-fast under sustained failure.** When a downstream has been failing for a sustained period, it is often better to DLQ new messages immediately (fail-fast) than to retry them, preserving capacity for when the downstream recovers. This is the operational expression of "do not amplify the outage."

## Common Traps

### Global Retry Policy Applied To All Messages

Configuring one retry count and backoff for the whole system, so transient and permanent failures are treated identically — permanent failures loop, transient failures are DLQed too early, and time-sensitive messages wait through a retry tail that outlives their usefulness. The trap is the convenience of a single knob. Tune per message type against its failure modes.

### Exponential Backoff Without Jitter

Using deterministic exponential backoff, so when many consumers fail at once they all retry at the same computed intervals, producing synchronized load spikes that prevent a recovering downstream from recovering. The trap is that the backoff looks correct in isolation. Always add jitter, and prefer full jitter when protecting a fragile downstream.

### Unbounded Retries On A Poison Message

Retrying a structurally unprocessable message forever because the retry policy has no bound or because the failure is not classified as permanent. The message loops, blocks its partition, and starves the messages behind it. The trap is assuming "more retries = more resilient." Bound retries and detect poison messages early.

### DLQ With No Enrichment

Routing failed messages to a DLQ as raw payloads, losing the error context, attempt history, and original metadata, so triage requires reconstructing facts that no longer exist. The trap is that the DLQ receives the message but not the diagnosis. Enrich every DLQ entry with failure context at the moment of failure.

### DLQ With No Owner, Alerts, Or Retention Limit

Creating a DLQ because the framework provides one, then never monitoring it, never assigning an owner, and never setting retention — so failed messages accumulate invisibly until broker storage fills or someone notices weeks of lost orders. The trap is treating the DLQ as a destination rather than a process. Every DLQ needs an owner, depth and age alerts, and a retention policy.

### Replay That Generates New Message Ids

Building replay tooling that re-publishes DLQ messages with fresh ids, defeating consumer idempotency and causing duplicate side effects on replay. The trap is treating replay as a fresh publish. Replay must preserve the original message identity so consumers deduplicate correctly.

### Side Effect And Acknowledgement Decoupled

Performing the side effect and acknowledging the message in separate steps, so a crash between them causes the side effect to be re-performed on retry. The trap is assuming the broker's redelivery is the only duplicate source. Couple the side effect and the acknowledgement transactionally.

### Retry Storm Against A Degraded Downstream and conflating Quarantine And Retry-Exhaustion DLQ

Aggressive retries with short backoff and no circuit breaker against a downstream that is degraded, multiplying the load and preventing recovery — turning a partial outage into a total one. The trap is that each consumer's retries look reasonable in isolation but combine destructively. Use backoff with jitter, bound concurrency, and circuit-break under sustained failure.

Mixing structurally-unprocessable poison messages with messages that simply exhausted retries against a transient outage, so the DLQ contains messages needing opposite responses (fix-the-data versus wait-and-replay). The trap is a single DLQ that no one knows how to triage. Separate them or at least tag them distinctly.

## Self-Check

- [ ] Retry policy is chosen per message type against its actual failure modes — transient failures retry with a bounded count and backoff, permanent failures (parse errors, missing references, consumer bugs) are detected and routed to quarantine or DLQ quickly rather than looped, and time-sensitive messages fail-fast within their useful lifetime.
- [ ] Retries use exponential backoff with a meaningful cap and mandatory jitter (full jitter by default to protect fragile downstreams, equal jitter where faster recovery is the priority), and there are no fixed-interval or jitterless policies that could synchronize retries across consumers.
- [ ] The dead-letter queue is designed as an operational artifact: every entry is enriched with the original message plus metadata (topic, partition, offset, ids, timestamps) and failure context (error type, message, attempt count, attempt timestamps, failing consumer), and per-topic versus shared DLQ is a deliberate choice for ownership clarity.
- [ ] Poison messages are detected by failure signature and quarantined separately from (or distinctly tagged within) the retry-exhaustion DLQ, and a sustained consumer failure triggers a circuit breaker that pauses consumption rather than flooding the DLQ.
- [ ] The post-exhaustion lifecycle is designed: replay, discard, reroute, and escalate are all supported; every DLQ has a discoverable owning team; replay tooling exists, targets subsets of messages, and preserves original message identity; and DLQ depth and oldest-message-age alerts page the owner.
- [ ] Every retried consumer is idempotent by a stable key derived from the message, replay preserves that identity so deduplication holds, and side effects are committed transactionally with the acknowledgement so a crash between them cannot cause a duplicate effect on retry.
- [ ] Retry storms are designed against: backoff with jitter bounds synchronized load, consumer concurrency is bounded so retries cannot exhaust the consumer's own resources, and a circuit breaker or fail-fast policy prevents amplifying a downstream outage into a total one.
- [ ] The highest-risk cases were specifically verified — a poison message looping forever, a DLQ silently accumulating with no owner or alerts, a replay causing duplicate side effects, and a degraded downstream amplified into total failure by aggressive synchronized retries — rather than only the single-retry-succeeds happy path.
