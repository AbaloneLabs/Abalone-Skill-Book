---
name: load-and-performance-testing.md
description: Use when the agent is designing or running load, stress, soak, spike, or capacity tests, validating throughput and latency SLOs under load, sizing infrastructure before launch, interpreting p95/p99 latency and error-rate results, or diagnosing whether a performance issue is real or a test artifact.
---

# Load And Performance Testing

Load and performance testing is not "run the endpoint in a loop and look at the average response time." It is an experiment designed to answer a specific question: *what happens to this system under a defined, realistic demand profile, and where does it stop meeting its promises?* The most common failure is producing a confident-looking report that answers the wrong question — because the load was unrealistic, the environment was not production-like, the warmup was skipped, or a bottleneck was hidden by an averaging metric.

Agents tend to under-invest in test design and over-invest in tooling. The hard part is deciding what load shape to generate, what to measure, how to isolate the system under test, and how to interpret results without being fooled by coordinated omission, client-side bottlenecks, or cold-start noise. A test that passes but does not resemble production demand provides false confidence that is worse than no test at all.

## Core Rules

### Start From A Specific Question And A Hypothesis

Before choosing a tool, write down the question the test must answer. "Is the system fast?" is not a question. Good questions are specific and falsifiable:

- Can the service sustain 500 requests per second with p99 latency under 300ms and zero 5xx errors for 30 minutes?
- At what offered load does the p95 latency cross our SLO threshold?
- Does the system recover gracefully when load drops back to normal after a spike?
- How many concurrent users can the cluster serve before the database connection pool exhausts?

Every later decision — load profile, duration, metrics, pass/fail criteria — should trace back to this question. If you cannot state the hypothesis, you are not testing, you are generating numbers.

### Model Realistic Load, Not Convenient Load

The single biggest source of meaningless load tests is unrealistic traffic. Real production traffic has properties that a naive script destroys:

- **Read/write mix and data distribution.** A test that always reads the same cached row will report throughput a production system can never achieve. Vary keys across a realistic cardinality and include the long tail of cold data.
- **Think time and arrival patterns.** Real users do not fire requests back-to-back with zero think time. Closed-model tests (fixed number of virtual users hammering continuously) and open-model tests (requests arriving at a target rate regardless of completion) produce very different behavior. Match the model to your real arrival process.
- **Payload size and shape diversity.** A test using tiny identical payloads hides serialization, network, and memory costs that dominate at scale.
- **Geographic and network realism.** Testing from inside the same datacenter as the service hides real-world latency, TLS overhead, and cross-AZ costs.

### Distinguish Load Profiles By Intent

Different questions require different load shapes. Do not conflate them:

- **Load test:** Sustained expected peak load over a representative duration. Validates normal-operation SLOs.
- **Stress test:** Increase load beyond expected peak until the system breaks. The goal is to find the breaking point and observe *how* it fails, not to confirm it survives.
- **Soak / endurance test:** Moderate load over a long duration (hours). Surfaces memory leaks, connection leaks, log file growth, cache warm-up drift, and GC pressure that short tests miss.
- **Spike test:** Sudden large jumps in load. Tests autoscaling, queue depth handling, and backpressure behavior.
- **Capacity / headroom test:** Determines how much load the current provisioned capacity can absorb while still meeting SLOs, informing scale-up decisions.

State which profile you are running and why. A "load test" that runs for 60 seconds is neither a load test nor a soak test.

### Make The Environment Production-Like, Or Say What Is Different

A load test is only as trustworthy as the environment's fidelity to production. Document explicitly what differs and assess whether each difference invalidates the result:

- **Hardware and instance types.** A laptop or shared CI runner cannot represent a production cluster. CPU steal, limited file descriptors, and container CPU throttling distort results.
- **Dataset size.** Testing against 1,000 rows when production has 50 million hides index, scan, and join costs.
- **Topology.** Test the full path including load balancer, TLS termination, cache, database, and downstream dependencies. Testing a service in isolation with mocked dependencies tells you nothing about the real bottleneck.
- **Configuration parity.** Connection pool sizes, thread counts, cache TTLs, and feature flags must match production. A test against debug-mode or dev-configured services is meaningless.

Where full fidelity is impossible, test at the component level but be explicit that you are measuring a component, not the system.

### Control Warmup, Steady State, And Measurement Windows

Results from the first seconds of a test are dominated by JIT compilation, cache population, connection establishment, and pool warmup. Always include a warmup phase and discard it from measurements. Conversely, do not run so long that background noise accumulates without representing a steady state you care about.

Define the measurement window in advance. Cherry-picking the best 10-second slice, or reporting a window that includes shutdown, produces misleading numbers.

### Measure Distributions, Not Averages

Latency is a distribution, and averages hide the tail that determines user experience and SLO compliance. Report percentiles — p50, p90, p95, p99 — and be aware that p99.9 over a short test is statistically unstable. Also report:

- **Error rate**, broken down by status code class. A "successful" test with a rising 5xx rate is a failure.
- **Throughput** as a function of offered load, to find the knee where latency degrades.
- **Resource saturation**: CPU, memory, GC time, connection pool utilization, queue depth, disk I/O. Latency degradation without resource saturation often indicates a client-side or network bottleneck, not a server problem.

### Use Independent Generation And Measurement

The machine generating load should not be the bottleneck. If the load generator saturates its own CPU, network, or file descriptors, you are measuring the generator, not the system under test. Monitor the generator's own resources and confirm it has headroom. Prefer an out-of-process, independently-scaled load source, and ideally collect server-side metrics separately from client-side observed latency, because they answer different questions.

### Account For Coordinated Omission

When a load generator waits for a response before sending the next request (closed model), stalls in the system cause the generator to send fewer requests during the stall, hiding the true latency of requests that would have arrived. This biases latency measurements dramatically downward during slowdowns. Use open-model arrival where appropriate, or use tools that account for the intended schedule rather than the achieved one. If your tool does not handle this, at minimum be aware that reported latency during degradation is optimistic.

### Define Pass/Fail Criteria Before Running

Decide what constitutes success before seeing results, otherwise confirmation bias will redefine success to match whatever happened. Criteria should reference the hypothesis: specific SLO thresholds, error rates, and saturation points. Record both the criteria and the actual outcome.

### Plan For Reproducibility

Load test results are noisy. A single run proves little. Record the seed, the load script version, the environment configuration, the dataset snapshot, and the time. Run enough iterations to distinguish signal from noise, and when comparing two configurations (before/after an optimization), run them back-to-back under identical conditions, ideally interleaved, to control for environmental drift.

## Common Traps

- **Reporting average latency as the headline number.** Averages mask tail latency and outliers. A system with a 50ms average and a 5-second p99 is broken by most SLO definitions.
- **Testing from inside the same network/VPC as the service.** This hides real network latency, TLS cost, and load-balancer behavior, inflating throughput and deflating latency.
- **Using a closed load model with high think time but interpreting results as open-system throughput.** The number of concurrent users, the arrival rate, and the achieved throughput are different quantities that get conflated.
- **Mocking downstream dependencies and concluding the service is fast.** You measured the service plus mocks, not the service plus its real dependency chain. The real bottleneck is often a downstream call.
- **Ignoring client-side saturation.** When the load generator hits 100% CPU or exhausts connections, throughput plateaus and you falsely conclude the server is maxed out.
- **Running for too short a duration.** Memory leaks, cache drift, and connection exhaustion only appear over time. A 60-second test cannot validate a soak scenario.
- **Cherry-picking the measurement window.** Selecting the smoothest-looking slice after the fact is confirmation bias, not measurement.
- **Treating a passing stress test as a green light.** The value of stress testing is observing failure modes, not surviving. If nothing broke, you did not stress it enough.
- **Comparing runs across different times of day or shared environments.** Noisy neighbors, background jobs, and autoscaling state vary. Comparisons must control for environment.
- **Confusing offered load with achieved load.** If the server rejects or queues requests, achieved throughput is lower than what you asked for. Report both.
- **Scaling load linearly with a single dimension and missing nonlinear failure.** Systems often degrade suddenly past a threshold (lock contention, pool exhaustion, cache eviction storms). Step-load tests reveal the knee; pure ramp tests can blur it.

## Self-Check

- Can I state, in one sentence, the specific question this load test is designed to answer, and the pass/fail criteria I defined before running?
- Does my load profile (read/write mix, data distribution, think time, arrival model) match real production traffic, or am I testing convenient synthetic traffic?
- Did I discard a warmup phase, and is my measurement window defined in advance rather than selected after seeing results?
- Am I reporting latency as percentiles (p95/p99) plus error rate and resource saturation, not just an average and a throughput number?
- Is the environment production-like? If not, have I documented every difference and assessed whether it invalidates the conclusion?
- Are downstream dependencies real or mocked? If mocked, am I careful not to claim end-to-end performance?
- Is my load generator itself unsaturated, with headroom on its own CPU, network, and connections?
- Did I run enough iterations to distinguish signal from noise, and did I control the environment when comparing configurations?
- For stress tests: did I actually find a breaking point and observe the failure mode, or did I stop too early?
- For soak tests: did I run long enough to surface leaks, drift, and slow resource exhaustion?
- Am I reporting offered load and achieved load separately, so queuing and rejection are visible?
