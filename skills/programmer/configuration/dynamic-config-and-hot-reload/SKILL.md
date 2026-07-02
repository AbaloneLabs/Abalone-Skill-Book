---
name: dynamic_config_and_hot_reload.md
description: Use when the agent is implementing runtime configuration changes, hot reload of config without redeployment, watching config sources for updates, maintaining consistency across instances during a config change, rolling back a bad config update, caching configuration, or handling the failure modes of dynamic configuration systems such as stale configs, partial propagation, and observation lag.
---

# Dynamic Config and Hot Reload

Dynamic configuration, the ability to change application behavior at runtime without redeployment, is powerful and dangerous in equal measure. The power is obvious: tune a threshold, enable a feature, or change a limit without a deploy. The danger is less obvious and is the source of most incidents involving config: a dynamic config change is a global, instantaneous behavior change to a running system, with no build, no review gate, and often no rollback plan. A bad config value can break every instance simultaneously, faster than any code deploy, because config propagation is fast and code rollout is slow. Agents who treat dynamic config as a convenient knob underestimate that it shares all the risks of a production deploy with none of the ceremony, and that a config mistake is a production change made directly by a human under time pressure.

The judgment problem is that dynamic config trades deploy safety for speed, and the trade must be made deliberately. A config value baked into the build can only change with a deploy (slow, reviewed, tested), which is safe but inflexible. A config value fetched at runtime can change instantly (fast, flexible), which is responsive but can break the system in seconds. The agent must decide which config warrants dynamism (values that must change faster than deploy cadence: kill switches, throttles, feature flags), ensure changes propagate consistently or with a defined partial-rollout model, make every change observable and reversible, and design for the failure modes unique to dynamic config: stale values, partial propagation, watch-mechanism failures, and the cache coherence problem across many instances. A dynamic config system without these properties is a loaded gun pointed at production.

## Core Rules

### Reserve dynamism for config that must change faster than deploy cadence

Not all config should be dynamic. The default should be static config baked into the build or deployment, which changes only through the deploy pipeline (reviewed, tested, rolled out gradually). Make a config value dynamic only when it must change faster than deploy cadence allows: kill switches, circuit-breaker thresholds, rate limits, feature flags, and operational tuning that responds to live conditions. Each dynamic value is a permanent operational risk surface (it can be changed to a breaking value at any time by anyone with access), so the value must justify the risk. Resist making everything dynamic for convenience; the convenience is paid for in incident risk.

### Make every dynamic config change observable, versioned, and reversible

A dynamic config change is a production behavior change and must be treated with the same seriousness as a deploy. Every change should be: observable (logged with who changed what, when, and why, and visible in a change history), versioned (each config version is identifiable, so you know which version each instance is running), and reversible (you can roll back to the previous value quickly, ideally with one action). Without versioning and history, a bad change cannot be diagnosed ("what was the value before?"). Without fast rollback, a bad change becomes an incident with no recovery. Treat the config store as a versioned, audited system, not a free-form value editor.

### Define the propagation model: eventual consistency, partial rollout, or atomic

When config changes, how does it reach instances? The model matters and must be explicit. Eventual consistency (each instance picks up the new value within some lag) is simplest but means instances run different values simultaneously, which can cause inconsistent behavior or split-brain if the config controls routing or compatibility. Partial rollout (roll the new value to a subset first, like a canary) is safer for risky changes but requires the config system to support targeted rollout. Atomic (all instances change at once) is rarely achievable and usually an illusion. Know which model your system uses, design the application to tolerate the resulting inconsistency (e.g., a value that changes compatibility should be rolled out with both old and new supported during transition), and never assume all instances see the same value at once.

### Handle watch and fetch failures explicitly, with a safe fallback

The mechanism by which instances learn of config changes (polling, watch/stream, push) will fail: the config service goes down, the watch connection drops, the network partitions. Define what happens on failure. The safest default is usually to keep running with the last-known-good value (fail static) rather than to fail closed (crash or refuse to serve) or fail open (revert to a compiled default that may be wrong). Log the failure so it is detectable. Never design the system so that a config-service outage takes down the application; the application must run on its cached config when the source is unavailable. Test this by cutting the config service in a staging or chaos exercise and confirming the application continues.

### Bound the cache and the staleness, and know the lag

Instances cache config for performance (fetching on every read is too slow), and caching introduces staleness: an instance may run an old value for some time after a change. Know the maximum staleness (cache TTL plus fetch latency plus watch propagation) and ensure the application tolerates it. If a change must take effect within seconds, the cache TTL must support that. If the application cannot tolerate instances running different values for the staleness window, the config must not be dynamic, or the change must be coordinated (e.g., expand-contract: deploy code supporting both values, change config, then deploy code requiring the new value). Stale config that causes inconsistency is a common dynamic-config incident.

### Avoid config values that change compatibility without an expand-contract transition

Some config changes are not just value changes; they change how the application interprets data or communicates, and mixing old and new across instances breaks the system. For example, changing a serialization format or a data-schema version via config means instances running different values cannot understand each other's data. These changes must not be made as a simple config flip; they require an expand-contract (parallel-change) transition: deploy code that can handle both old and new, change the config, then deploy code that uses only the new. Treat any config value that affects compatibility as a deployment coordination problem, not a runtime toggle.

### Make dynamic config changes testable, including the transition

A config value is tested at the value it shipped with, but dynamic config means the value changes at runtime to values that were never tested. This is a gap. Where feasible, test the application against a range of plausible config values, including boundary and adversarial values, so a runtime change to an untested value does not break the system. More importantly, test the transition itself: changing a value at runtime while the application is serving traffic must not corrupt state, leak resources, or cause errors. A config reload that is safe at rest but breaks under load is a latent incident. Include config-change-during-load in your test and chaos exercises.

## Common Traps

### Making everything dynamic for convenience

Each dynamic value is a permanent risk surface. Reserve dynamism for values that must change faster than deploy cadence; keep the rest static and deploy-gated.

### No versioning, history, or rollback for config changes

A bad change with no record of the prior value and no fast rollback becomes an incident with no recovery. Version, audit, and make every change reversible in one action.

### Assuming all instances see the new value at once

Propagation is eventually consistent; instances run different values during the lag. Design the application to tolerate this, or use expand-contract for compatibility-affecting changes.

### The config-service outage takes down the application

If the application cannot run without the config service, the config system is a critical single point of failure. Fail static to last-known-good and keep running; test this by cutting the config service.

### Unbounded staleness causing inconsistency

A long cache TTL means instances run old values long after a change. Know the maximum staleness and ensure the application tolerates the window, or use coordinated transitions.

### Changing compatibility-affecting config with a flip

Serialization, schema, or protocol changes made via a simple config flip break instances running different values. Use expand-contract transitions for these.

### Untested runtime values breaking the system

A value that works at ship time may break when changed at runtime to an untested value, or the transition itself may break under load. Test a range of values and test the change under load.

## Self-Check

- Is dynamism reserved for config that must change faster than deploy cadence (kill switches, throttles, flags), with the rest kept static and deploy-gated?
- Is every dynamic config change versioned, audited (who/what/when/why), and reversible in a single fast action, with full change history?
- Is the propagation model (eventual consistency, partial rollout, or atomic) explicit, and does the application tolerate instances running different values during the lag?
- Does the application fail static (keep running on last-known-good) when the config service or watch mechanism fails, rather than failing closed or open, and has this been tested?
- Is the maximum staleness (cache TTL plus fetch plus propagation) known and bounded, and does the application tolerate the resulting inconsistency window?
- Are compatibility-affecting config changes (serialization, schema, protocol) made via expand-contract transitions rather than simple flips?
- Are dynamic config values and the runtime transition itself tested (a range of values, change-during-load), so an untested runtime value or a reload under load does not break the system?
- Is the config-service outage scenario exercised (chaos exercise cutting the config source), confirming the application continues on cached config?
