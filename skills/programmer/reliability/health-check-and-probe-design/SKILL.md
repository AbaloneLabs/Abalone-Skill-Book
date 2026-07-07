---
name: health_check_and_probe_design.md
description: Use when the agent is designing health checks, liveness probes, readiness probes, startup probes, or deep health checks for a service; configuring Kubernetes/orchestrator probes; deciding what "healthy" means for a service with dependencies; avoiding probe cascading failures and thundering herds; or debugging a service that flaps between healthy and unhealthy. Covers liveness vs readiness vs startup semantics, dependency-aware deep checks, probe failure modes (cascading, thundering herd, self-inflicted load), probe thresholds and timing, and the distinction between a service being alive and being able to serve.
---

# Health Check And Probe Design

A health check is how an infrastructure platform decides whether a service instance is fit to receive traffic or fit to continue running, and getting it wrong produces two characteristic failures. The first is the cascading failure: a service's health check depends on a downstream dependency, the dependency blips, the health check fails, the orchestrator removes or restarts the instance, and now the service itself becomes unavailable to its own callers — who then fail their health checks, propagating the outage outward. A health check that includes dependencies turns a downstream blip into an upstream outage. The second is the thundering herd: an orchestrator restarts many instances, they all hit their dependencies simultaneously to pass their startup health checks, the dependency cannot handle the burst, the checks fail, the instances restart again, and the system never converges. Health checks are load-generating callers of the service they observe, and under failure conditions they can amplify the failure they are meant to detect.

Agents tend to design health checks as "ping all the dependencies and report healthy only if all respond," conflating liveness (the process is running) with readiness (the service can serve), and not accounting for the load the checks themselves generate or the cascading effect of dependency-aware checks. The judgment problem is recognizing that health check semantics must match their purpose (a liveness probe that fails restarts the process; a readiness probe that fails removes traffic — very different consequences), that a deep check's dependencies determine its blast radius, and that probe timing and thresholds determine whether the system detects real failures or flaps on noise. This skill covers the discipline of health check and probe design: liveness/readiness/startup semantics, dependency-aware deep checks, avoiding cascading failures and thundering herds, and probe timing and thresholds.

## Core Rules

### Distinguish Liveness, Readiness, And Startup Semantics

Different probes answer different questions, and their failure has different consequences. Conflating them causes incorrect orchestration behavior.

- **Liveness ("is the process alive and able to make progress?"): failure restarts the process.** A liveness probe checks whether the process is running and not deadlocked. If it fails, the orchestrator kills and restarts the instance. Use it to detect hung, deadlocked, or wedged processes that cannot recover without a restart. Do not use it to check dependencies — a liveness failure that depends on a downstream blip causes unnecessary restarts.
- **Readiness ("can this instance serve traffic right now?"): failure removes the instance from the load balancer.** A readiness probe checks whether the instance is ready to handle requests (warmed up, connected to dependencies, not overloaded). If it fails, the orchestrator stops sending traffic but does not restart. Use it to remove unhealthy or overloaded instances from rotation without restarting them.
- **Startup ("has the process finished initializing?"): gates liveness/readiness during slow startup.** A startup probe checks whether the service has completed its initialization (loaded caches, established connections). Until it passes, liveness probes are disabled, preventing the orchestrator from killing a slow-to-start process. Use it for services with slow startup (large cache warmup, heavy initialization).
- **Match the probe type to the consequence.** A liveness probe failure restarts (disruptive); a readiness probe failure removes traffic (less disruptive); a startup probe failure extends the startup window. Choose the probe type whose failure consequence matches the condition being detected.

### Keep Liveness Shallow And Dependency-Light

A liveness probe that depends on downstream services turns their failures into restarts of this service, amplifying outages. Liveness should be shallow.

- **Liveness checks whether the process itself is alive, not whether its dependencies are.** A liveness probe that queries a database and fails when the database is down causes the orchestrator to restart the service — which does not fix the database and disrupts the service's recovery. Liveness should verify the process is running and not deadlocked, not the health of dependencies.
- **Avoid dependency calls in liveness probes.** A liveness probe that makes a network call to a dependency generates load on that dependency (every instance probing at intervals) and couples the service's restart behavior to the dependency's health. Keep liveness local and fast.
- **Detect deadlocks and wedges, not transient errors.** The legitimate liveness failure is a process that is hung, deadlocked, or in an unrecoverable state — conditions a restart fixes. Transient errors (a slow response, a temporary dependency failure) are not liveness failures and should not trigger restarts.

### Make Readiness Dependency-Aware But Cascade-Safe

A readiness probe should reflect whether the instance can serve, which depends on its dependencies — but it must not cascade failures outward.

- **Readiness can check dependencies, because an instance that cannot reach its dependencies cannot serve.** Unlike liveness, readiness legitimately includes dependency health: if the database is unreachable, the instance cannot serve requests, so removing it from rotation is correct.
- **But bound the cascade: readiness failure removes traffic, it does not propagate the failure upward as a restart.** The key safety property is that a readiness failure stops traffic to this instance without making this instance's callers fail their own probes. The load balancer routes to other instances; if all fail, the service is unavailable, but it has not cascaded via restarts.
- **Distinguish "dependency down" from "dependency slow."** A readiness check that fails on a slow dependency (timeout) may remove healthy instances that could still serve some requests. Consider whether a degraded dependency warrants removing the instance or just shedding some load.
- **Cache dependency state to avoid probe load.** A readiness probe that queries dependencies on every check generates constant load on them. Cache the dependency health (updated by a background check) and have the probe return the cached state, reducing probe load.

### Prevent The Thundering Herd On Startup And Recovery

When many instances start or recover simultaneously, their health checks and dependency queries can overwhelm the dependencies, preventing convergence.

- **Stagger startup and recovery.** Rolling deploys, staggered restarts, and jitter on probe intervals prevent all instances from hitting dependencies simultaneously. Orchestrators support rolling updates with max-unavailable and max-surge settings to control the pace.
- **Use a startup probe for slow-starting services.** A startup probe gives the service time to initialize without liveness probes killing it, preventing restart loops during slow startup. Configure the startup probe threshold to accommodate the expected startup time.
- **Warm caches gradually, not all at once.** A service that warms its entire cache on startup hits the backing store with a burst of requests. Warm gradually (lazy population, background warming with rate limits) to avoid overwhelming the backing store.
- **Add jitter to probe intervals.** If all instances probe at the same interval starting from the same time, their probes synchronize, creating periodic bursts. Jitter (randomized probe timing) spreads the load over time.

### Configure Probe Timing And Thresholds To Detect Real Failures Without Flapping

Probe timing (interval, timeout, failure threshold) determines whether the system detects real failures or flaps on noise. Misconfigured timing causes false restarts or missed failures.

- **Set the failure threshold to tolerate transient blips.** A failure threshold of 1 restarts on the first failed probe, which may be a transient blip. A threshold of 3 (three consecutive failures) tolerates brief blips while still detecting sustained failures. Match the threshold to the failure mode (restart on sustained failure, not on a single blip).
- **Set the timeout to distinguish failure from slowness.** A probe timeout that is too short fails on slow responses (which may still be serving); too long delays detection of real failures. Set the timeout to the duration beyond which a response is genuinely a failure, not just slow.
- **Set the interval to balance detection speed against load.** A short interval detects failures fast but generates more probe load; a long interval reduces load but delays detection. Typical intervals are 10-30 seconds; balance detection speed against the load probes generate.
- **Set the success threshold for readiness appropriately.** A readiness success threshold of 1 returns an instance to rotation on the first success (fast recovery); a higher threshold requires sustained success (more conservative, avoids flapping). Use a higher threshold for readiness than for liveness, since returning a not-yet-ready instance to traffic can cause errors.

## Common Traps

### Dependency-Aware Liveness Probe

A liveness probe that queries dependencies, causing restarts when dependencies blip and amplifying outages. Keep liveness shallow (process alive, not deadlocked).

### Conflating Liveness And Readiness

Using a single health check for both, so a dependency blip causes restarts (liveness behavior) when it should only remove traffic (readiness behavior). Separate the probes by semantics.

### Cascading Health Check Failures

A deep health check whose dependency failure propagates outward as restarts or traffic removal, turning a downstream blip into an upstream outage. Bound the cascade: readiness removes traffic, does not restart.

### Thundering Herd On Startup

Many instances starting simultaneously, overwhelming dependencies with health-check and warmup traffic, preventing convergence. Stagger startup, use startup probes, warm gradually, add jitter.

### Probe Load On Dependencies

Health checks that query dependencies on every interval, generating constant load. Cache dependency health and have the probe return cached state.

### Failure Threshold Of 1 (Flapping)

Restarting or removing traffic on a single failed probe, which may be a transient blip. Use a failure threshold of 3+ to tolerate blips while detecting sustained failures.

### Timeout Too Short (Failing On Slowness)

A probe timeout that fails on slow responses, removing or restarting instances that are slow but functional. Set the timeout to distinguish failure from slowness.

### No Startup Probe For Slow-Starting Services

A slow-starting service killed by liveness probes before it finishes initializing. Use a startup probe to gate liveness during initialization.

## Self-Check

- [ ] Liveness, readiness, and startup probes are distinguished by semantics and consequence — liveness (process alive, failure restarts) is shallow and dependency-light; readiness (can serve, failure removes traffic) is dependency-aware but cascade-safe; startup (initialized, gates liveness) is used for slow-starting services.
- [ ] Liveness probes do not call downstream dependencies — they verify the process is running and not deadlocked — so a dependency blip does not cause unnecessary restarts that amplify outages.
- [ ] Readiness probes may check dependencies (an instance that cannot reach dependencies cannot serve), but failure removes traffic rather than restarting, bounding the cascade so a downstream blip does not propagate outward as restarts.
- [ ] Dependency health is cached (updated by a background check, returned by the probe) rather than queried on every probe, so probes do not generate constant load on dependencies.
- [ ] Startup and recovery are staggered (rolling deploys, jitter on probe intervals, startup probes for slow starters, gradual cache warming) to prevent a thundering herd of health checks and warmup traffic from overwhelming dependencies.
- [ ] Probe timing is configured to detect real failures without flapping: failure threshold tolerates transient blips (3+ consecutive failures), timeout distinguishes failure from slowness, interval balances detection speed against probe load, and readiness success threshold is conservative to avoid returning not-yet-ready instances to traffic.
- [ ] The health check endpoint itself is lightweight (fast response, low resource use, no side effects) and does not perform expensive operations or mutations, treating the probe as a frequent caller whose load must be bounded.
- [ ] Deep health checks (if used for monitoring or dashboards, distinct from orchestrator probes) are clearly separated from liveness/readiness probes, report dependency status as information without triggering orchestration actions, and have their own rate limiting to avoid dependency load.
