---
name: product-incident-response.md
description: Use when the agent is responding to a product incident or outage, coordinating the product role during incident response, deciding customer communication during a live incident, managing scope and impact assessment, or leading the product perspective while engineering handles technical mitigation.
---

# Product Incident Response

During a major incident, the product manager's role is not to fix the technical problem — that is engineering's job — but to own the parts that determine whether the incident damages customer trust as much as it damages service: impact assessment, customer communication, scope decisions, and the bridge between the technical response and the business and customer reality. A product manager who either disappears (leaving communication to whoever is available) or interferes (trying to direct the technical fix) makes the incident worse. The product manager who understands their specific role makes the difference between an incident customers forgive and one they remember.

This skill covers the judgment needed during a live incident, from the moment it is declared through to stabilization. It complements, but does not replace, the engineering-led technical response.

## Core Rules

### Know your role and do not overstep it

Incident response fails when roles are unclear or when someone tries to do someone else's job. The product manager's role during an incident is specific and bounded.

- **You own impact assessment and customer communication.** You determine who is affected, how severely, and what they need to be told.
- **You own scope and prioritization of non-technical decisions.** Which features can be safely disabled to reduce load, whether to pause a launch, whether to roll back a product change, whether to invoke a business continuity plan.
- **You do not own the technical mitigation.** Do not direct engineers on how to fix the problem. Offer product context they may lack (what a feature does, what data it touches, what depends on it), but defer to engineering on the technical approach.

Overstepping into technical direction slows the response and undermines the incident commander. Disappearing leaves customer communication to whoever has bandwidth, which is usually wrong. Clarity on your role is the foundation of a useful product contribution.

### Assess impact rigorously before communicating scope

The first instinct in an incident is to communicate quickly to appear responsive. But communicating a scope that turns out to be wrong is worse than communicating slightly later with an accurate scope, because each correction erodes credibility. Establish the impact as rigorously as the situation allows before the first broad communication.

- Identify which users are affected: all users, a segment, a geography, a plan tier, users of a specific feature.
- Identify how they are affected: complete outage, degraded performance, data inconsistency, security exposure, incorrect results without an error.
- Identify the business stakes: revenue at risk, contractual SLAs, data integrity, safety, compliance obligations with notification requirements.

Distinguish confirmed impact from suspected impact. In the first communication, say what is confirmed and what is still being determined. A message that says "we have confirmed X and are investigating Y" is more credible than one that confidently states a scope that later expands.

### Communicate proactively, honestly, and on a cadence

Customer communication during an incident is trust-critical. The goal is not to minimize the incident in the customer's eyes but to demonstrate that the team is in control, informed, and acting. Customers can forgive an outage; they struggle to forgive being kept in the dark or misled.

- **Communicate proactively** before customers have to chase for status. A status page update or direct notification that precedes the flood of support contacts is far better than one that follows it.
- **Be honest about severity and uncertainty.** Do not downplay. If the impact is serious, say so. If the root cause or recovery time is unknown, say "we do not yet know" rather than inventing a reassuring estimate.
- **Set and meet a communication cadence.** Promise the next update at a specific time and deliver it, even if the update is "still investigating." The cadence itself reassures; a missed update alarms.
- **Tailor the message to the audience.** Technical customers want technical detail; business customers want impact and ETA; end users want to know if they should take action. One message rarely serves all.

The temptation to soften language ("experiencing degraded performance" when the service is down) is strong and counterproductive. Customers who are experiencing the problem know the truth, and euphemism reads as either incompetence or dishonesty.

### Separate the mitigation track from the root cause track

During an incident, two distinct questions compete for attention: "how do we restore service?" and "what caused this?" Confusing them slows recovery. The priority is restoration; root cause can wait.

- **Mitigation** aims to restore service or reduce impact by any safe means: rollback, feature disablement, capacity increase, traffic rerouting, a manual workaround. This is the urgent track.
- **Root cause** investigation runs in parallel but must not block mitigation. Understanding the cause matters for the post-incident review, not for restoring service in the moment.

A common failure is refusing to mitigate until the cause is understood, on the theory that a fix without understanding might not work or might cause new problems. In practice, restoration through rollback or disablement is usually the right call under customer impact, with root cause pursued afterward.

### Make scope decisions deliberately, especially disabling features

Incidents often require product decisions: disable a feature to reduce load, pause an in-flight launch, roll back a change, or fall back to a degraded but functional mode. These are product decisions because they weigh customer impact against each other, and engineering should not make them in isolation.

- Evaluate the tradeoff: does disabling the feature reduce the incident impact more than it costs the users who depend on it?
- Consider whether a partial or degraded mode is available that preserves critical functionality while shedding load.
- Document the decision and its rationale. Scope decisions made under pressure are scrutinized afterward and should be defensible.

### Coordinate with support before, not after, customer impact lands

Support will bear the brunt of customer contact during the incident. They must be prepared, not surprised. Coordinate with support as part of the response, not as an afterthought.

- Brief support on the impact and the recommended customer messaging before the public communication goes out, so they are not blindsided by customer questions.
- Provide support with the current status, the workaround if one exists, and what to tell customers about ETA. Inconsistent messaging between the status page and the support team destroys credibility.
- Establish a loop so support can feed back what they are hearing from customers, which may reveal impact the internal assessment missed.

### Protect the team's ability to respond effectively

An incident is a marathon, not a sprint, especially a long one. A product manager who contributes to burnout by demanding constant status, scheduling back-to-back meetings, or preventing breaks makes the response worse. Ensure the people doing the technical work have what they need: clear context, freedom from unnecessary interruptions, and the ability to hand off when fatigue sets in. Your job includes protecting the conditions under which others can do their best work.

## Common Traps

### Disappearing or going silent during the incident

Some product managers, unsure of their role, step back entirely and leave communication and scope decisions to engineering or leadership. This creates a vacuum filled by whoever is available, often with inconsistent or wrong messaging. Even if your technical contribution is limited, your role in impact assessment and communication is essential and cannot be abdicated.

### Overstepping into technical direction

The opposite failure is trying to direct the fix, second-guessing engineering decisions, or demanding status so frequently that it interrupts the work. This slows recovery and undermines the incident commander. Contribute product context when asked, defer on technical approach, and channel your energy into the parts you own.

### Downplaying severity in customer communication

The instinct to protect the brand by softening language backfires. Customers experiencing the problem know its severity, and euphemism reads as dishonesty or denial. Honest, direct communication about severity preserves more trust than damage control.

### Communicating scope before it is confirmed

Announcing "a small number of users are affected" that later becomes "all users" destroys credibility. Each expansion of scope makes the next statement less believable. Distinguish confirmed from suspected impact, and resist pressure to state a confident scope prematurely.

### Promising ETAs that cannot be met

Under pressure for a recovery time, teams estimate optimistically, then miss the estimate, then miss the revised one. Each missed ETA is a trust failure. Prefer "we do not have a reliable estimate yet; we will update by [time]" over a specific hour that is a guess. An honest unknown is more credible than a confident wrong number.

### Refusing to mitigate until root cause is known

Waiting for full understanding before taking restoration action extends customer impact. Under pressure, mitigation through rollback or disablement is usually the right call, with root cause pursued in the post-incident review. Do not let the pursuit of understanding block restoration.

### Inconsistent messaging across channels

The status page says one thing, the support team tells customers another, and a executive's social media post says a third. Inconsistency is noticed and interpreted as chaos. Coordinate messaging across all channels and treat support as a first-class recipient of the communication plan.

### No handoff plan for long incidents

In a multi-hour or multi-day incident, the initial responders fatigue and the quality of decisions degrades, but no one wants to "give up" the incident. Without a planned handoff, the incident is run by exhausted people making worse decisions. Plan and execute handoffs deliberately, with full context transfer.

## Self-Check

- Do I understand and stay within my role: owning impact assessment, customer communication, and product scope decisions, while deferring technical mitigation to engineering?
- Did I assess impact rigorously and distinguish confirmed from suspected scope before the first broad communication?
- Is my customer communication proactive, honest about severity and uncertainty, on a promised cadence, and tailored to the audience?
- Am I running mitigation and root cause investigation as separate tracks, prioritizing restoration over understanding during the incident?
- Are scope decisions (feature disablement, launch pauses, rollback) made deliberately with documented rationale, weighing customer tradeoffs?
- Have I coordinated with support before public communication, given them consistent messaging, and established a feedback loop for what they hear?
- Am I protecting the response team's ability to work effectively, including planning handoffs for long incidents?
- Did I avoid downplaying severity, premature scope claims, and optimistic ETAs that erode credibility?
- Is messaging consistent across the status page, support, leadership, and any public channels?
- After stabilization, will I contribute to a learning review focused on systemic improvement rather than blame?
