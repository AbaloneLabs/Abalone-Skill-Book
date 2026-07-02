---
name: serverless_architecture_decisions.md
description: Use when the agent is deciding whether to use serverless compute such as Lambda or Functions, choosing where serverless fits in an architecture, designing event-driven serverless with triggers and idempotency, evaluating serverless databases or message services, or assessing whether a workload fits serverless limits. Also covers cold-start latency and when it matters for user-facing versus background work, execution time and memory limits, the statelessness requirement and external state stores, fan-out and fan-in patterns, retry and at-least-once idempotency, the per-invocation cost model and surprise scale bills, concurrency and deployment-size limits, observability and distributed tracing across functions, and when serverless is the wrong choice such as long-running, steady high load, tight latency, or complex stateful workflows. Use when proposing serverless for a new service, reviewing an event-driven design, or deciding between serverless and always-on compute.
---

# Serverless Architecture Decisions

Serverless is the compute model that most reliably produces both delight and regret, because its strengths and weaknesses are extreme and they apply to different workloads in opposite ways. For event-driven, bursty, sporadic, or glue workloads, serverless functions are nearly ideal: they scale from zero, charge per invocation, remove server operation, and let a small team wire together systems that would once have required a platform team. For steady high-throughput, latency-sensitive, long-running, or stateful workloads, the same model becomes a source of throttling, cold-start latency, surprise bills, and debugging nightmares. The decision is not "is serverless good" but "does this specific workload's shape fit the serverless model's assumptions" — and agents routinely answer that question by pattern-matching on hype or familiarity rather than by checking the workload against the model's actual limits.

The judgment problem is that serverless inverts the usual tradeoffs in ways that are invisible in a prototype and painful in production. A prototype pays nothing, scales magically, and ships in an afternoon, which teaches the team that serverless is always right. The production system then hits the limits the prototype never exercised: cold starts on the user-facing path, a per-invocation bill that explodes when request volume is high, a concurrency cap that throttles a fan-out exactly when throughput matters, a stateless function that cannot hold the session state the workload needs, and a distributed execution across dozens of functions that no one can trace or debug. The harm is delayed and specific: architectures that cannot meet their latency SLO because of cold starts, bills that scale linearly with traffic past the point where always-on compute would be cheaper, and systems whose failure modes are opaque because the execution is scattered across ephemeral invocations. This skill is about deciding whether and where serverless fits, as a compute and architecture decision. It treats cost as one factor among several (not the focus, which is a separate skill) and treats event-driven messaging as a separate concern; here the focus is serverless as a compute choice and the workload-shape judgment it demands.

## Core Rules

### Decide Serverless Against Workload Shape, Not Against Hype Or Familiarity

The first decision is whether the workload's shape fits the serverless model, and the answer depends on characteristics that are easy to state and easy to ignore: invocation pattern, latency sensitivity, runtime duration, statefulness, and traffic profile. Serverless is excellent for some shapes and poor for others, and the cost of a wrong fit is paid in production.

- **Sporadic, bursty, or event-driven workloads fit serverless well.** A function that runs a few times a minute in response to a webhook, a queue message, a file upload, or a schedule pays only for what it does, scales to zero when idle, and removes the burden of keeping a server warm for sporadic traffic. This is the serverless sweet spot.
- **Steady high-throughput workloads often fit serverless poorly.** A function that runs continuously at high invocation rate pays per invocation forever, and at sufficient volume the per-invocation cost crosses over the cost of an always-on compute instance that handles the same load. Serverless rewards spikiness and penalizes steadiness; a 24/7 high-traffic service is frequently cheaper and simpler on always-on compute with autoscaling.
- **Latency-sensitive user-facing workloads are at risk from cold starts.** A function that must respond within tens of milliseconds cannot tolerate the cold-start penalty (the time to provision and initialize an idle instance), which is unpredictable and can be hundreds of milliseconds to seconds depending on runtime, memory, and provider. Background workloads tolerate cold starts; interactive request paths often do not.
- **Long-running workloads exceed serverless time limits.** Functions have a maximum execution time (minutes, not hours); workloads that run longer — large batch jobs, streaming processors, long-lived connections — must be split, checkpointed, or moved to a model without the time ceiling. Designing around the time limit for a workload that fundamentally runs long is a sign serverless is the wrong model.

The strong choice is to map the workload's invocation pattern, latency need, runtime, statefulness, and traffic profile against the serverless model's characteristics, and use serverless where they align. The weak choice is to default to serverless for everything because it removes server operation, or to reject it everywhere because of one bad past experience.

### Treat Cold Starts As A First-Class Latency Constraint On User-Facing Paths

Cold starts are the serverless failure mode that most often invalidates a user-facing design, and they are routinely discovered only after launch because they do not appear under sustained load — they appear exactly when traffic is low or spiky, which is when they are hardest to reproduce.

- **Cold starts happen when a new instance must be provisioned.** The first request after idle, a burst that exceeds the warm instance count, a deployment, or a scale-up all trigger initialization: loading the runtime, the code, and the dependencies, and running any initialization code. The penalty varies by runtime (interpreted languages with large dependency trees are slower), by memory allocation (more memory means more CPU and faster init), and by provider.
- **Assess cold-start sensitivity by the workload's latency budget and traffic pattern.** A user-facing API with a 100 ms SLO and spiky traffic is highly sensitive; a background queue consumer with no human waiting is insensitive. Provisioned concurrency (keeping instances warm) can eliminate cold starts at additional cost, trading the serverless "scale to zero" saving for predictable latency — which may negate the reason serverless was chosen.
- **Minimize cold-start impact in the design.** Smaller deployment packages, fewer dependencies, lazy initialization of expensive resources, and connection reuse across invocations reduce init time. But these are mitigations, not eliminations; if the workload cannot tolerate any cold start, serverless without provisioned concurrency is the wrong choice for that path.
- **Beware the "it was fast in the prototype" trap.** A prototype under sustained load shows no cold starts; the production system at low overnight traffic or on a burst shows them. Test latency under the real traffic pattern, including idle-then-burst, before committing a user-facing path to serverless.

### Respect The Statelessness Requirement And Place State Deliberately

Serverless functions are stateless and ephemeral: each invocation may run on a different instance, instances are reclaimed at any time, and nothing in the function's memory survives between invocations reliably. This is the core architectural constraint, and it determines where state must live and how the system is shaped.

- **All durable state lives outside the function** — in a database, a cache, an object store, a queue, or a session store. The function is compute only. This is a strength (any instance can serve any request, enabling scale) and a burden (every piece of state the workload needs must be fetched and written through an external service, adding latency and failure modes).
- **Do not rely on in-process or module-level state.** Global variables, in-memory caches, and module-level singletons persist only across invocations that reuse the same instance, which the platform controls and which is not guaranteed. Treating them as durable state produces bugs that appear only when the platform reclaims the instance. They are valid only as an opportunistic cache, with correctness that does not depend on them.
- **Session and connection state must be externalized.** A function cannot hold a long-lived connection or a session in memory; sessions live in a shared store, and connections are re-established or pooled through external means. Workloads that depend on connection state (websockets, long polling, stateful protocols) are a poor fit unless paired with a dedicated connection-management service.
- **State placement shapes the architecture.** Because state is external, a serverless system is naturally decomposed into functions plus stateful backing services, which is often cleaner than a monolith but introduces a distributed-system boundary at every state access. Design the state stores and their access patterns deliberately, because they become the system's performance and reliability ceiling.

### Design Event-Driven Serverless For Retries, Idempotency, And Fan-Out

Serverless functions are most powerful when wired to event sources — queues, streams, webhooks, storage events, schedules — and event-driven delivery is almost always at-least-once, meaning the same event can be delivered more than once. Designing as if delivery were exactly-once is the most common correctness bug in event-driven serverless.

- **Assume at-least-once delivery and make handlers idempotent.** A handler that charges a customer, increments a counter, or appends a record must produce the same result whether the event arrives once or three times. Idempotency is achieved with an idempotency key (a unique event identifier checked against a store before processing), natural idempotency (operations that are safe to repeat, like upserts), or deduplication at the handler boundary. A non-idempotent handler under retries produces duplicate charges, inflated counts, and duplicate records.
- **Plan fan-out and fan-in explicitly.** A single event can trigger many parallel function invocations (fan-out), and results may need to be aggregated (fan-in). Fan-out multiplies cost and load and can hit concurrency limits; fan-in across stateless functions requires an external aggregation point (a queue, a store, an orchestrator) because functions cannot coordinate in memory. Design the fan-out boundary and the aggregation mechanism, and watch for the thundering herd where one event triggers a fan-out that triggers further fan-out.
- **Handle retry, failure, and poison messages.** Retries on a failing handler eventually exhaust and must route to a dead-letter queue for inspection; a handler that always fails (a poison message) can otherwise retry forever, multiplying cost and blocking a queue. Define the retry policy, the dead-letter destination, and the operational process for inspecting and replaying failed messages.
- **Be cautious with chaining and long event chains.** A chain of functions triggering functions triggering functions becomes hard to reason about, hard to trace, and hard to debug, with each hop adding latency and a failure mode. Prefer bounded, well-named event flows, and use an orchestration layer (a workflow or step-function service) for multi-step processes that need ordering, compensation, and state, rather than ad-hoc function-to-function chains.

### Understand The Per-Invocation Cost Model And Its Scale Risk

Serverless cost is billed per invocation and per execution time, which is ideal for sporadic workloads and dangerous for high-volume ones. The same property that makes serverless cheap at low volume — you pay only for what runs — makes it expensive at high volume, because the per-invocation price never goes to zero and scales linearly with request count. (Reducing an existing serverless bill is the cost-optimization skill's concern; here the point is that cost shape is a selection factor.)

- **Estimate cost at projected peak request volume, not at prototype volume.** A function that costs fractions of a cent per call is negligible at a thousand calls a day and startling at a million calls a minute. Model the bill at the projected peak and at sustained high load, and compare to the always-on alternative. The crossover where always-on compute becomes cheaper is real and is often lower than teams assume.
- **Watch for request-volume-driven surprise bills.** A misconfigured retry storm, a fan-out that multiplies a traffic spike, a polling loop, or an external caller that hammers an endpoint can generate invocations — and bills — far beyond the expected pattern, with no natural ceiling. Serverless scales without limit in both directions, including the cost direction; set billing alarms and concurrency caps to bound the blast radius.
- **Execution time and memory multiply cost.** Cost is a function of duration times memory allocated; a function that runs longer or is over-provisioned on memory costs proportionally more. Optimizing the handler's runtime and right-sizing memory (more memory can mean faster execution and lower total cost) is part of the design, not an afterthought.
- **The "scale to zero" saving applies only to idle time.** Serverless is cheap when it is mostly idle; a function that runs constantly is paying per invocation for load an always-on instance would handle for a fixed price. Match the model to the workload's idle ratio.

### Plan Observability And Debugging For Distributed, Ephemeral Execution

Serverless execution is distributed across ephemeral instances and event sources, which makes the failure modes that are easy to see in a monolith (a stack trace, a log stream, a single process) hard to see across functions. Agents frequently build serverless systems with per-function logging and then find them undebuggable in production, because the relevant information is scattered across invocations with no correlation.

- **Distributed tracing across functions and event sources is essential, not optional.** Propagate a correlation identifier through every event and every downstream call, and send spans to a tracing backend, so that a single user request or event can be followed across the functions and services it touched. Without tracing, an intermittent failure across a chain of functions is nearly impossible to localize.
- **Structured logs with context beat unstructured print statements.** Logs must carry the correlation identifier, the event identifier, and enough context to reconstruct the invocation, because you cannot attach a debugger to an ephemeral instance after the fact. Aggregate logs centrally and make them queryable by identifier.
- **Cold paths and rare events are the hardest to debug.** A failure that happens only under cold start, only on a specific event shape, or only at scale is invisible in local testing and may appear only in production. Reproducing it requires the production observability; invest in tracing and metrics before the rare failure occurs, not after.
- **Metric and alert coverage must include the serverless-specific signals.** Invocation count, error rate, duration, throttling, concurrency saturation, and dead-letter queue depth are the vital signs of a serverless system. A serverless system without these metrics is operating blind to the failure modes most likely to affect it.

### Know The Limits And When Serverless Is The Wrong Choice

Serverless platforms impose hard limits — on execution time, memory, deployment package size, concurrent executions, environment size, and cold-start behavior — and a workload that needs to exceed any of them is a poor fit, no matter how convenient the model otherwise seems. Designing constantly around a limit is a signal to change models.

- **Concurrency limits throttle fan-out at the worst moment.** Platforms cap the number of simultaneous executions per account or function; a burst or a large fan-out that hits the cap is throttled, queueing or failing invocations exactly when throughput matters most. Requesting a limit increase is possible but not guaranteed and not instant; a design that depends on exceeding the default concurrency limit is fragile.
- **Deployment size and configuration limits constrain the design.** Large deployment packages, many environment variables, or heavy dependencies may exceed limits or slow cold starts. A function that needs a large model, a big dependency tree, or extensive configuration is fighting the model.
- **Serverless is the wrong choice for several workload shapes.** Long-running jobs that exceed the time ceiling; steady high-throughput services where per-invocation cost dominates; latency-critical user-facing paths that cannot tolerate cold starts; complex stateful workflows that need long-lived connections or in-process state; and workloads that require deep system tuning or custom runtimes the platform does not support. For these, always-on compute (containers, VMs, or a managed orchestration platform) is usually simpler, cheaper, and more reliable.
- **Vendor coupling is real.** Serverless functions couple to the provider's event sources, runtime support, deployment tooling, and limits, more tightly than portable compute. For workloads where portability matters, confine the provider-specific surface to a thin handler and keep business logic portable, or choose a more portable compute model.

## Common Traps

### Defaulting To Serverless Because It Removes Server Operation

Choosing serverless for every workload because it eliminates server management, then hitting cold starts, concurrency caps, or per-invocation cost on workloads that fit always-on compute better. Decide per workload against its shape (invocation pattern, latency, runtime, statefulness, traffic profile), not by default.

### Ignoring Cold Starts On A User-Facing Path

Building an interactive API on serverless, finding it fast under sustained load in testing, then discovering intermittent high latency at low traffic or on bursts from cold starts in production. Assess cold-start sensitivity against the latency budget and traffic pattern; use provisioned concurrency or a different model where cold starts are intolerable.

### Relying On In-Process State Across Invocations

Storing session data, caches, or counters in function memory or module-level variables and treating them as durable, then losing or corrupting state when the platform reclaims the instance. Functions are stateless; all durable state lives in external stores, and in-process state is only an opportunistic cache whose correctness must not depend on it.

### Non-Idempotent Handlers Under At-Least-Once Delivery

Writing an event handler that charges, increments, or appends without deduplication, then producing duplicate charges, inflated counts, or duplicate records when the platform redelivers the event. Assume at-least-once delivery and make every handler idempotent via an idempotency key, natural idempotency, or deduplication.

### Surprise Bills From Request-Volume-Driven Scale

Shipping a serverless endpoint without modeling cost at projected peak volume or setting billing alarms, then receiving a large bill from a retry storm, a fan-out multiplier, a polling loop, or sustained high traffic. Model cost at peak, compare to always-on compute, and set billing alarms and concurrency caps to bound the blast radius.

### Hitting The Concurrency Cap During Fan-Out

Designing a fan-out that depends on simultaneous executions beyond the platform's default concurrency limit, then being throttled at the moment throughput matters most. Know the concurrency limit, request an increase with lead time if needed, and treat a design that must exceed the default as fragile.

### Per-Function Logging With No Distributed Tracing

Building a chain of functions with logging only, then being unable to localize an intermittent cross-function failure in production because the logs are scattered across invocations with no correlation. Propagate a correlation identifier and send distributed traces from the start; serverless failure modes are distributed and require distributed observability.

### Forcing A Long-Running Or Stateful Workload Into Functions

Splitting, checkpointing, and chaining functions to work around the execution time ceiling or the statelessness requirement for a workload that fundamentally runs long or holds state, accumulating complexity and failure modes. When a workload fights the model's limits at every turn, switch to always-on compute or an orchestration platform.

### Treating The Prototype's Performance As Production's

Concluding serverless is fast and cheap from a prototype under sustained load and low volume, then hitting cold starts and scale-driven cost in production. Test under the real traffic pattern (including idle-then-burst) and model cost at projected peak before committing.

## Self-Check

- [ ] The decision to use serverless was made against the workload's shape — invocation pattern, latency sensitivity, runtime duration, statefulness, and traffic profile — and serverless was chosen where the shape fits (sporadic, bursty, event-driven) rather than by default or familiarity.
- [ ] Cold-start latency was assessed against the workload's latency budget and traffic pattern: user-facing paths that cannot tolerate cold starts use provisioned concurrency or a different model, cold-start impact is minimized (small packages, lazy init, connection reuse), and latency was tested under idle-then-burst, not just sustained load.
- [ ] The statelessness requirement is respected: all durable state lives in external stores, in-process and module-level state is used only as an opportunistic cache whose correctness does not depend on it, sessions and connections are externalized, and state stores and access patterns are designed deliberately.
- [ ] Event-driven handlers are idempotent under at-least-once delivery (idempotency keys, natural idempotency, or deduplication), fan-out and fan-in boundaries and aggregation mechanisms are designed, retry policies and dead-letter queues handle failures and poison messages, and multi-step flows use an orchestration layer rather than ad-hoc function chains.
- [ ] Cost was modeled at projected peak request volume and sustained high load and compared to always-on compute, the crossover point is understood, billing alarms and concurrency caps bound the blast radius of retry storms and fan-out multipliers, and execution time and memory are right-sized rather than left at defaults.
- [ ] Observability covers the distributed execution: a correlation identifier propagates through every event and downstream call, distributed traces are sent to a backend, structured logs carry enough context to reconstruct an invocation, and metrics include invocation count, error rate, duration, throttling, concurrency saturation, and dead-letter depth.
- [ ] The platform's hard limits (execution time, memory, concurrency, deployment size) were checked against the workload's needs, and workloads that fight the limits — long-running, steady high-throughput, latency-critical, complex stateful, or needing deep tuning — were moved to always-on compute or an orchestration platform rather than forced into functions.
- [ ] The decision considered the operational reality of serverless — that the execution is distributed and ephemeral, that cold paths and rare events are the hardest to debug, and that the team has the observability and operational maturity to run a system whose failure modes are scattered across invocations — not just the prototype's convenience.
- [ ] Vendor coupling is acknowledged and bounded: provider-specific event sources and runtimes are confined to a thin handler where portability matters, with business logic kept portable, or a more portable compute model was chosen where coupling is unacceptable.
