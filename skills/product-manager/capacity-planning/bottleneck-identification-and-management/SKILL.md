---
name: bottleneck_identification_and_management.md
description: Use when the agent is identifying bottlenecks in a product delivery flow, analyzing what constrains throughput across teams and roles, deciding how to exploit or elevate a bottleneck, or improving overall delivery by managing the constraint rather than optimizing non-bottleneck steps.
---

# Bottleneck Identification And Management

A bottleneck is the resource, role, or process step whose capacity limits the throughput of the entire system. In any flow of work, the bottleneck determines how much the system can produce, no matter how fast the other steps are. Optimizing non-bottleneck steps produces no improvement in overall throughput, because the bottleneck still constrains the system. Done well, bottleneck management identifies the true constraint, exploits it to maximize its output, and elevates it only when exploitation is exhausted. Done poorly, teams optimize the steps they control or understand, feel productive, and wonder why overall delivery does not improve. Agents often fail to identify the bottleneck because it frequently sits outside the team being optimized, in a shared role, a dependent team, or an approval process.

The harm this skill prevents is local optimization that produces no global improvement. A team that doubles its own throughput while the bottleneck sits elsewhere delivers the same total output, because the bottleneck still limits the system. The effort spent optimizing non-bottlenecks is wasted from a system perspective, even though it feels productive.

Use this skill before answering questions such as "why is our delivery slow", "what is our bottleneck", "how do we improve throughput", or "why doesn't optimizing our team speed things up". The goal is to prevent the agent from optimizing non-bottleneck steps while the true constraint goes unaddressed.

## Core Rules

### Identify The Bottleneck By Where Work Accumulates

The bottleneck is found by observing where work accumulates, because the step that cannot keep up causes a queue to form before it and a starvation of work after it. Look for the role or process step with the longest queue of waiting work, the most over-time, the most complaints about being a blocker, and the step after which work flows freely. The accumulation pattern reveals the constraint more reliably than anyone's opinion about what is slow, because opinions are biased toward the steps each person interacts with.

Map the flow of work end to end and identify where queues build. The bottleneck is rarely where people assume, because each team feels its own pressure most acutely. A team that feels overloaded may be a non-bottleneck working hard on work that is queueing elsewhere; the actual bottleneck may be a shared role that everyone waits on but no one owns. Data on queue length and wait time reveals what opinions obscure.

### Exploit The Bottleneck Before Elevating It

Once the bottleneck is identified, the first response is to exploit it: maximize the output of the existing bottleneck capacity before adding more. This means ensuring the bottleneck is never idle, never working on low-value or rework tasks, never blocked by avoidable dependencies, and never consumed by work that a non-bottleneck could do. Every hour of bottleneck capacity wasted is an hour of system throughput lost permanently, because non-bottleneck capacity cannot compensate.

Exploitation often involves counterintuitive changes: prioritizing the bottleneck's work above others, removing meetings and overhead from the bottleneck role, ensuring quality upstream so the bottleneck does not spend time on rework, and feeding the bottleneck only with work it can actually process. These changes can significantly increase throughput without adding any capacity, simply by ensuring the constraint is fully utilized on its highest-value work.

### Subordinate Non-Bottlenecks To The Bottleneck

Non-bottleneck steps should be subordinated to the bottleneck, meaning they operate at a pace and in an order that serves the bottleneck, not their own local efficiency. A non-bottleneck that produces work faster than the bottleneck can consume it merely builds inventory that waits, adding no throughput and increasing coordination cost. A non-bottleneck that optimizes its own utilization at the expense of the bottleneck's flow reduces system throughput. Subordination means the non-bottlenecks adapt to keep the bottleneck fed with the right work at the right time.

This is often the hardest change to make, because it requires non-bottleneck teams to work in ways that look locally inefficient, such as idling when the bottleneck is full or prioritizing work that unblocks the bottleneck over work that uses their own capacity fully. The discipline of subordinating to the constraint is what converts local optimization into system improvement.

### Elevate The Bottleneck Only After Exploitation Is Exhausted

If exploitation and subordination are insufficient, the next step is to elevate the bottleneck: add capacity to the constrained resource, through hiring, training, tooling, automation, or process redesign. Elevation is expensive and slow, so it should follow exploitation and subordination, which are cheaper and faster. Adding capacity to a bottleneck that is not fully exploited wastes the investment, because the new capacity is also underutilized.

When elevating, be aware that resolving one bottleneck reveals the next. The system's constraint moves to the next-most-limited resource, and the optimization cycle begins again. This is expected and healthy: continuous improvement means continuously identifying and addressing the current constraint, not solving one bottleneck and declaring victory.

### Check Whether The Bottleneck Is Policy Rather Than Capacity

Not all bottlenecks are capacity constraints; some are policy constraints. An approval process that requires sign-off from a busy executive, a definition of done that mandates review by a scarce role, a release process that batches work into infrequent windows: these are policy bottlenecks, where the constraint is a rule rather than a resource. Policy bottlenecks can sometimes be resolved by changing the rule, which is cheaper than adding capacity. Examine whether the bottleneck is a real capacity limit or a self-imposed policy that could be relaxed.

Distinguishing policy from capacity bottlenecks matters because the responses differ. A capacity bottleneck is addressed through exploitation, subordination, and elevation. A policy bottleneck is addressed by questioning whether the policy serves its purpose and whether it can be redesigned to reduce constraint without sacrificing its intent.

### Monitor For Bottleneck Migration

As bottlenecks are addressed, the constraint migrates. The role that was the bottleneck last quarter may no longer be, and a different role may now limit throughput. A capacity assessment or allocation strategy built around the old bottleneck becomes stale and may optimize the wrong thing. Monitor for bottleneck migration by periodically re-examining where work accumulates, and update the exploitation and subordination strategy to target the current constraint.

Failing to notice migration is a common failure: a team invests heavily in elevating a bottleneck, succeeds, and then continues to over-invest in that role long after it ceased to be the constraint, while the new bottleneck goes unaddressed. The discipline is to always optimize the current constraint, not the previous one.

## Common Traps

### Optimizing Non-Bottlenecks

Improving steps that are not the constraint. The trap is local productivity gains that produce no system-level throughput improvement.

### Assuming The Bottleneck Is Where It Feels Slow

Identifying the constraint by opinion rather than queue data. The trap is optimizing the step that complains loudest rather than the one that actually limits flow.

### Elevating Before Exploiting

Adding capacity to a bottleneck that is not fully utilized. The trap is expensive investment that is itself underutilized.

### Non-Bottlenecks Optimizing Local Efficiency

Working at full local utilization regardless of the bottleneck's needs. The trap is inventory buildup and reduced system throughput.

### Ignoring Policy Bottlenecks

Treating a rule-based constraint as a capacity problem. The trap is adding resources when changing the policy would have resolved it.

### Optimizing A Stale Bottleneck

Continuing to focus on a former constraint after it migrated. The trap is investment in a role that no longer limits the system.

## Self-Check

- [ ] The bottleneck is identified by where work accumulates and queues build, not by opinion.
- [ ] The bottleneck is exploited, never idle and never on low-value work, before capacity is added.
- [ ] Non-bottleneck steps are subordinated to the bottleneck's pace and needs, even at local inefficiency.
- [ ] Elevation, adding capacity, follows exploitation and subordination, not precedes them.
- [ ] Policy bottlenecks are distinguished from capacity bottlenecks and addressed by rule change where possible.
- [ ] The constraint is monitored for migration, and optimization targets the current bottleneck, not a previous one.
- [ ] Improvements are measured by system throughput, not by local step productivity.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
