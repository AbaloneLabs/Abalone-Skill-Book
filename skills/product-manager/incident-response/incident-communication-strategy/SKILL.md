---
name: incident-communication-strategy.md
description: Use when the agent is deciding incident communication strategy, writing customer-facing incident messages, determining notification audiences and timing, managing public relations during an incident, or crafting post-incident communications and disclosures after resolution.
---

# Incident Communication Strategy

What you say during and after an incident shapes customer trust as much as the incident itself. Customers do not expect software to be perfect; they expect to be informed, respected, and treated honestly when it fails. Communication that is timely, truthful, and human preserves trust through serious incidents. Communication that is delayed, evasive, legalistic, or self-protective destroys trust that took years to build, and the destruction is often permanent. The product manager frequently owns or strongly shapes this communication, and the judgment involved is distinct from the technical response.

This skill covers the strategy and craft of incident communication: who to tell, when, what to say, and how to say it, both during the incident and after resolution.

## Core Rules

### Make the communication strategy a first-class part of incident response

Incident response plans that focus only on technical mitigation and omit communication produce incidents that are technically resolved but reputationally damaging. Communication must be planned, owned, and resourced alongside the technical response, not improvised when customers start complaining.

- Name a communication owner for every significant incident, distinct from the technical incident commander, so neither role is shortchanged.
- Pre-build templates and decision criteria for common incident types (outage, data incident, security incident, degraded performance) so the first message is not written from scratch under pressure.
- Define the audiences and channels in advance: status page, direct customer email, in-product notification, support scripting, executive briefing, public statement for major incidents.

Improvised communication under pressure is consistently worse than communication prepared in advance and adapted to the specifics.

### Prioritize timeliness over completeness in the first message

The first communication sets the tone. Its purpose is to acknowledge that something is wrong, that the team is aware and responding, and to set expectations for updates. It does not need to be complete; in fact, waiting for completeness delays it dangerously.

- Send the first acknowledgment as soon as a significant incident is confirmed, even if scope and cause are unknown.
- State what is confirmed, what is being investigated, and when the next update will come.
- Resist the pressure to wait until you have a full picture. Customers who are experiencing the problem already know something is wrong; silence reads as ignorance or cover-up.

The credibility of all subsequent communication depends on the first message being timely. A late first message, no matter how complete, has already lost ground.

### Be honest about severity, scope, and uncertainty

The central discipline of incident communication is honesty, especially when honesty is uncomfortable. Downplaying severity, narrowing scope prematurely, or stating confident ETAs that are guesses all feel protective in the moment and destructive afterward.

- Use plain, direct language about severity. If the service is down, say it is down, not "experiencing degraded performance."
- Distinguish confirmed from suspected scope. "We have confirmed impact on X and are investigating whether Y is affected" is honest and credible; "a small number of users are affected" that later becomes everyone is not.
- State uncertainty explicitly. "We do not yet have a reliable estimate for recovery" is more trustworthy than a specific time that is a guess.

Customers can handle bad news; they cannot handle being misled. Every incident where the communication was honest but painful preserves more trust than one where the communication was reassuring but wrong.

### Tailor the message to each audience

Different audiences need different information, and a single message rarely serves them all. Map the audiences and craft appropriate messages for each.

- **End users** need to know whether they are affected, whether they should take action, and roughly when to expect resolution. They do not need technical detail.
- **Technical contacts and admins** need enough detail to assess their own exposure, take protective action, and answer questions from their stakeholders.
- **Support** needs consistent messaging, the current status, any workaround, and the boundaries of what they can promise, so they do not contradict the official channels.
- **Executives and account managers** need the business stakes, the customer impact, and the planned communication, so they can handle escalated conversations consistently.
- **The public**, for major incidents, needs a statement that acknowledges impact, demonstrates response, and avoids both minimization and over-dramatization.

Inconsistent messaging across these audiences is noticed and interpreted as disorganization. Coordinate so that all channels tell a consistent story at the appropriate level of detail.

### Set and meet a communication cadence

During an active incident, the cadence of updates matters as much as their content. A promised update that arrives on time reassures; a missed update alarms. Set a cadence appropriate to severity and meet it.

- For severe, fast-moving incidents, updates every 30 to 60 minutes may be appropriate.
- For slower incidents, every few hours may suffice.
- Always state when the next update will come, and deliver it at that time even if the content is "still investigating."

The cadence is a demonstration of control. Breaking it, even with good reason, undermines the perception that the team is on top of the situation.

### Handle the post-incident communication deliberately

The incident is not over when service is restored; it is over when customers understand what happened, what it meant for them, and what will change. Post-incident communication closes the trust loop and is often more important for long-term trust than the during-incident updates.

- Send a resolution communication that confirms service is restored, summarizes what happened at an appropriate level of detail, acknowledges the impact honestly, and describes what will change to reduce recurrence.
- Avoid both the non-apology ("we apologize for any inconvenience") that minimizes and the over-apology that implies more harm than occurred. Match the tone to the actual impact.
- For significant incidents, follow up with a more detailed post-incident review shared with affected customers. The level of technical detail should match the audience; technical customers value transparency about cause and fix.

The post-incident communication is what customers remember. Investing in it is investing in the relationship.

### For data and security incidents, follow disclosure obligations precisely

Incidents involving data exposure, security breaches, or compliance-relevant failures carry legal and regulatory disclosure obligations that override normal communication preferences. These must be identified and followed precisely.

- Involve legal and compliance early to determine what must be disclosed, to whom, and on what timeline.
- Do not let a desire to control the narrative delay or shape disclosures that are legally required.
- Coordinate the required disclosures with the broader communication so they are consistent rather than contradictory.

Failing disclosure obligations converts a technical incident into a legal and regulatory crisis that far exceeds the original damage.

### Match the channel to the severity

Not every incident warrants a direct email to every customer; over-communication of minor incidents trains customers to ignore the channel, so the major incident message is missed. Match the channel to the severity.

- Minor, short incidents may warrant only a status page entry for those who look.
- Moderate incidents affecting a segment warrant targeted notification to the affected segment.
- Major incidents warrant broad notification through multiple channels, including proactive outreach to key accounts.

Calibrating the channel preserves the signal value of each channel for when it matters most.

## Common Traps

### Waiting for completeness before the first message

The first message is delayed until the full picture is known, by which time customers have already concluded the team is unaware or hiding the problem. Prioritize timely acknowledgment over complete information; completeness comes in later updates.

### Euphemism that minimizes severity

"Degraded performance," "intermittent issues," "suboptimal experience" — euphemisms feel protective and read as dishonest to customers experiencing the real impact. Plain language about severity preserves trust; euphemism destroys it.

### Confident ETAs that are guesses

Pressure for a recovery time produces optimistic estimates that are then missed, sometimes repeatedly. Each missed ETA is a credibility loss. Prefer an honest "we do not have a reliable estimate yet" over a specific time that is a guess.

### Inconsistent messaging across channels

The status page, the support team, the account manager, and the executive all say different things, and customers notice. Coordinate messaging across all channels and treat support as a first-class recipient of the communication plan.

### The non-apology that minimizes impact

"We apologize for any inconvenience" in response to a serious outage reads as dismissive and damages trust. Match the acknowledgment to the actual impact, and be specific about what users experienced.

### Over-communicating minor incidents

Treating every minor incident as a major communication trains customers to ignore the channel. When the major incident arrives, the notification is lost in the noise. Calibrate channel and reach to severity.

### Post-incident silence

Service is restored and the team moves on without a resolution communication, leaving customers uncertain whether it is truly over and what to make of it. The post-incident message closes the trust loop and is essential, not optional.

### Disclosure shaped by narrative control rather than obligation

For data and security incidents, the desire to minimize reputational damage shapes or delays required disclosures, converting a contained incident into a compliance crisis. Follow disclosure obligations precisely, coordinated with but not overridden by communication preferences.

## Self-Check

- Is communication a planned, owned, resourced part of incident response, with templates and audience definitions prepared in advance?
- Was the first message timely, acknowledging the incident before customers had to chase for status?
- Is the communication honest about severity, scope, and uncertainty, using plain language rather than euphemism?
- Have I tailored messages to each audience (end users, technical contacts, support, executives, public) while keeping the story consistent?
- Did I set and meet a communication cadence, delivering updates at the promised time even when content was "still investigating"?
- Is there a deliberate post-incident communication that confirms resolution, acknowledges impact honestly, and describes what will change?
- For data or security incidents, have legal and compliance obligations been identified and followed precisely, coordinated with broader communication?
- Is the channel calibrated to severity, so that minor incidents do not train customers to ignore the major-incident channel?
- Did I avoid non-apologies, premature confident ETAs, and minimization that erode trust?
- After this incident, would an affected customer say they were informed, respected, and treated honestly throughout?
