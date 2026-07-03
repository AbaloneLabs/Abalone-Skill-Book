---
name: float_and_schedule_flexibility_analysis.md
description: Use when the agent is analyzing total float and free float across a schedule network, interpreting schedule flexibility, deciding where slack protects the plan, assessing near-critical paths, or deciding how to use float without silently consuming schedule protection.
---

# Float And Schedule Flexibility Analysis

Float, also called slack, is the amount of time an activity can slip without causing harm. It is the schedule's shock absorber, and reading it correctly is what separates a schedule that can absorb delay from one that collapses on the first slip. Total float measures slip relative to the project end; free float measures slip relative to the next activity; and the distinction matters because they protect different things. The craft is not just computing float but interpreting it: knowing where flexibility genuinely exists, where it is fragile, and where it has already been silently consumed. Agents tend to treat float as idle time to be filled, to confuse total and free float, and to ignore near-critical paths that are one slip away from becoming critical.

Use this skill before interpreting a schedule's flexibility, deciding where to apply resources or buffer, or diagnosing why a schedule with apparent slack still keeps slipping. The goal is to prevent the agent from mistaking fragile float for real protection and from consuming the schedule's reserves without knowing it.

## Core Rules

### Distinguish Total Float From Free Float

Total float is how much an activity can slip before the project end date is affected; it is computed as latest finish minus earliest finish. Free float is how much an activity can slip before the next activity's earliest start is affected; it is computed as the earliest successor start minus this activity's earliest finish. Total float protects the project; free float protects the successor. An activity can have large total float and zero free float, meaning it can slip without delaying the project but will immediately delay the next task.

Always state which float you mean. Decisions based on the wrong float misallocate protection.

### Understand Where Float Comes From

Float arises from the structure of the network: parallel paths that are shorter than the critical path create float on the shorter paths. It is a property of the network logic, not a buffer someone added. This means float can disappear when logic changes, when durations change, or when resource leveling is applied. Do not treat float as a stable reserve; it shifts as the schedule evolves.

Recompute float after every network or duration change, because yesterday's flexibility may be gone.

### Treat Float As Protection, Not Idle Time

Float exists to absorb uncertainty, resource conflicts, and minor delays. It is not spare capacity to be filled with more work or pulled forward to please a target. When float is consumed by low-priority work or optimistic re-planning, the previously non-critical path becomes critical and the project loses its ability to absorb shocks. The most common schedule failure is silent consumption of float followed by surprise when everything becomes critical.

Make float visible and treat deliberate consumption as a decision with a recorded reason.

### Protect The Critical Path Using Float Elsewhere

Because the critical path has zero float, delays there extend the project directly. Use float on non-critical paths to protect the critical path: pull resources from floating activities to support critical ones, sequence critical work to start as early as possible, and avoid starting floating work so early that it competes for shared resources. The discipline is to spend flexibility where it buys the most protection.

Do not let floating activities drift early and consume the resources the critical path needs.

### Watch Near-Critical Paths Closely

A path with only a small amount of float is near-critical. A short delay turns it critical, and if the original critical path is also tight, the project can suddenly have two critical paths and no flexibility. Rank paths by float and monitor the top few, not just the single critical one. Near-critical paths are where surprises come from.

Set a float threshold below which a path is treated as critical for management attention.

### Distinguish Negative Float As A Real Alarm

Negative float means the schedule cannot meet its imposed constraint: the calculated dates are later than a required milestone. This is not flexibility; it is a forecast of certain slippage unless scope, logic, resources, or the constraint itself change. Negative float should trigger action, not be hidden by softening durations or removing logic to make the numbers look better.

Investigate the cause and resolve it through real changes, not cosmetic ones.

### Account For Resource Float Versus Logic Float

Logic float assumes infinite resources; resource float accounts for the fact that shared resources constrain when work can actually happen. A path with logic float may have no resource float if the people it needs are committed elsewhere. Resource leveling often converts logic float into resource-critical chains. Analyze float on the resource-leveled schedule to see real flexibility.

Do not promise flexibility that disappears the moment a shared specialist is double-booked.

### Use Float Deliberately In Compression And Recovery

When the schedule must be compressed or recovered, float is the first place to look. Crashing and fast-tracking often work by borrowing float from non-critical paths to relieve the critical one. But this consumes the protection, so do it knowingly and update the risk picture. A compressed schedule with no float anywhere is brittle and will slip on the first disruption.

## Common Traps

### Confusing Total Float With Free Float

Decisions based on total float when free float is zero will delay the successor even if the project end holds. State which float you use.

### Treating Float As Idle Time To Fill

Consuming float with extra work removes the schedule's shock absorption and turns near-critical paths critical. Treat float as protection.

### Ignoring Near-Critical Paths

Watching only the critical path misses paths that become critical after a small slip. Monitor the top few paths by float.

### Assuming Float Is Stable

Float shifts with logic, duration, and resource changes. Recompute after every update rather than relying on stale numbers.

### Hiding Negative Float Cosmetically

Softening durations or removing logic to eliminate negative float hides a real forecast of slippage. Resolve it with real changes.

### Promising Logic Float Without Resource Check

Logic float that vanishes under resource leveling is not real flexibility. Analyze the resource-leveled schedule.

### Starting Floating Work Too Early

Floating activities started early compete for shared resources and starve the critical path. Sequence to protect critical work.

### Consuming Float Without Recording The Decision

Silent consumption removes protection invisibly. Make deliberate consumption a visible, recorded decision.

## Self-Check

- [ ] Are total float and free float both computed and clearly distinguished in analysis and communication?
- [ ] Is float treated as protection against uncertainty rather than idle capacity to be filled?
- [ ] Are near-critical paths below a defined float threshold monitored alongside the critical path?
- [ ] Is float recomputed after every network, duration, or resource change?
- [ ] Is negative float treated as an alarm requiring real resolution, not cosmetic adjustment?
- [ ] Is float analyzed on the resource-leveled schedule so resource constraints are reflected?
- [ ] Are resources from floating activities used to protect the critical path rather than compete with it?
- [ ] Is deliberate consumption of float during compression or recovery made visible and recorded?
- [ ] Are floating activities sequenced so they do not starve the critical path of shared resources?
- [ ] When multiple paths share float, is the shared nature recognized so one path cannot exhaust it silently?
