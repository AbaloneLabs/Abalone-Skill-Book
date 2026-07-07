---
name: incident_communication_and_status.md
description: Use when the agent is drafting or managing communication during an incident or outage — writing status page updates, internal stakeholder updates, customer notifications, executive briefings, or social/public statements; deciding cadence and escalation of communication; managing the tension between speed and accuracy; coordinating communication across teams; or handling the communication after an incident (apologies, remediation summaries, regulatory or contractual breach notifications). Also covers the failure modes of silent gaps that breed speculation, premature root-cause claims that get retracted, jargon that confuses non-technical audiences, inconsistent messages across channels, communicating without a single source of truth, and the recurring mistake of treating incident communication as an afterthought to technical mitigation rather than a parallel, planned workstream.
---

# Incident Communication And Status

During an incident, two things happen in parallel: technical mitigation (finding and fixing the cause) and communication (telling the people who are affected what is happening, what to expect, and what to do). The technical work gets the attention; the communication determines whether customers, stakeholders, and the public trust that the situation is under control. The judgment problem is that communication during an incident operates under duress — incomplete information, a moving situation, pressure to say something fast — and the easy choices are the wrong ones. Staying silent to avoid saying something wrong breeds speculation and erodes trust. Claiming a root cause before it is confirmed leads to retractions that look incompetent. Jargon that is precise to engineers is meaningless to customers. Inconsistent updates across channels create confusion about what is actually true. The discipline is to treat communication as a planned, parallel workstream with its own role, cadence, single source of truth, and standards for what to say when you do not yet know the answer.

Agents tend to under-invest here because the technical fix feels like the "real" work and communication feels like it can wait until things are known. The harm is trust erosion that outlasts the incident. A 90-minute silence while engineers debug produces a status page that customers refresh in frustration and a social media backlash. A confident "we found the cause" that is later retracted makes every subsequent update suspect. A customer-facing update written in internal jargon ("the ingress controller's leader election timed out") tells the customer nothing about whether their data is safe. An executive briefed on the wrong severity commits the company to the wrong response. The judgment problem is to communicate early, frequently, and honestly; to separate what is known from what is suspected; to write for the audience that will read it; and to keep all channels consistent through a single source of truth, so communication builds trust rather than destroying it.

This skill covers the communication workstream and roles, cadence and triggers, accuracy under uncertainty, audience-appropriate messaging, channel consistency, and post-incident communication. It complements the incident-response skill (technical handling), the severity-and-escalation skill (classification), and the blameless-postmortem skill (learning). Here the focus is the communication that runs alongside mitigation.

## Core Rules

### Treat Communication As A Parallel Workstream With A Dedicated Role

Communication must not be an ad-hoc task squeezed in between technical steps. It is a parallel workstream with its own owner:

- **Assign a communicator or scribe distinct from the incident commander.** The person driving mitigation cannot also carefully draft customer updates. A dedicated communicator (or comms lead) owns status updates, stakeholder briefings, and channel consistency, freeing the technical lead to mitigate.
- **Stand up the comms workstream at incident start, not when asked.** The first update should go out early (see cadence), which means the comms role and the status page must be ready before the incident. Waiting until customers complain to start communicating is already late.
- **Keep a running timeline and decision log.** The communicator records what happened when, what was communicated, and what decisions were made, so updates are consistent and the post-incident review has an accurate record rather than reconstructed memory.

### Communicate Early And On A Cadence, Even When Information Is Incomplete

The strongest driver of trust during an incident is not perfection; it is presence. A regular cadence of honest updates outperforms sporadic "complete" ones:

- **Acknowledge the incident early.** The first update confirms the issue is known and being worked, even before the cause or ETA is known. "We are investigating reports of elevated errors" is better than silence, because it tells affected people they are heard and stops speculation.
- **Set and keep a cadence.** Commit to updates at a regular interval (every 30 minutes, every hour) and deliver them on time, even if the update is "still investigating." A predictable cadence lets people stop refreshing anxiously; a missed cadence breeds doubt about whether anyone is working it.
- **Update on material change, not only on cadence.** When the situation materially changes (severity escalates, a workaround is found, the fix is deploying), communicate immediately rather than waiting for the next scheduled update.
- **Do not wait for certainty to communicate.** Waiting until the root cause is confirmed to say anything creates a long silence. Communicate what is known now, mark what is suspected, and update as the picture clarifies.

### Separate Known Facts From Suspected Causes, And Never Overclaim

The most damaging communication error is stating a root cause or ETA that is not established, then retracting it. Accuracy under uncertainty requires discipline:

- **State what is known, mark what is suspected.** "Users in region X are experiencing elevated error rates (confirmed). We suspect a downstream dependency; we are verifying." Distinguishing confirmed facts from hypotheses preserves credibility when the hypothesis changes.
- **Do not claim a root cause until it is confirmed.** A premature "the cause was X" that is later retracted ("actually it was Y") makes every claim suspect. If the cause is not yet confirmed, say it is under investigation.
- **Be careful with ETAs.** An ETA that is missed is worse than no ETA. Give a range or a condition ("we expect recovery within the hour, pending verification") rather than a precise time, and update if it slips. A missed precise ETA signals the team does not understand the problem.
- **Correct earlier statements openly when they change.** If an earlier update was wrong, correct it explicitly ("an earlier update attributed this to X; our investigation now indicates Y"). Quietly changing the story looks like cover-up; open correction looks like honest learning.

### Write For The Audience That Will Read Each Update

Different audiences need different information, and the same incident requires multiple versions of the truth, each appropriate to its reader:

- **Customer-facing updates focus on impact and action.** Customers need to know: what is affected, whether their data is safe, what to do (retry, use a workaround, wait), and when to expect resolution. They do not need internal architecture; "some users are unable to log in; your data is not affected; we are working on a fix" is a good customer update.
- **Internal stakeholder updates add context and business impact.** Stakeholders need severity, blast radius, business impact, the mitigation plan, and what they should do or communicate. They need enough to make decisions and to answer questions from their own stakeholders.
- **Executive briefings are concise and decision-oriented.** Executives need the bottom line: severity, customer and revenue impact, estimated resolution, and any decision they need to make (approve a rollback, notify a regulator, approve customer compensation). Avoid technical detail they cannot act on.
- **Avoid jargon in non-technical channels.** Internal terms ("the ingress leader election timed out," "we're OOM-killing the workers") are meaningless or alarming to customers and executives. Translate to impact ("login is intermittently failing") unless the audience is technical.

### Maintain A Single Source Of Truth And Channel Consistency

When an incident is communicated across a status page, customer emails, internal chat, executive briefings, and social media, inconsistency creates confusion and erodes trust:

- **Designate a single source of truth for the current state.** One document or channel holds the authoritative current status, impact, and latest update. All other communications derive from it, so they cannot drift apart.
- **Update all channels consistently.** When the status page changes, ensure customer notifications, internal updates, and social reflect the same information. A status page saying "resolved" while customer support is still telling people "investigating" undermines credibility.
- **Sequence the channels deliberately.** Update the internal source of truth first, then customer-facing channels, so external communication is never ahead of internal awareness.
- **Keep customer support informed and aligned.** Support agents are the front line; they must have the current status and approved language, or they will improvise inconsistent messages.

### Handle Severity, Escalation, And External Obligations Deliberately

Some incidents trigger obligations beyond voluntary communication — escalation to leadership, notification of partners or regulators, and contractual breach handling:

- **Escalate communication as severity rises.** A higher severity means more stakeholders and faster cadence; the communication plan must scale with severity, not stay fixed at the initial level.
- **Meet notification obligations (contractual and regulatory).** Contracts and regulations (SLA breach notifications, data-incident reporting deadlines) may require notifying customers, partners, or regulators within specific timeframes. Track these obligations and meet them; a missed regulatory deadline compounds the incident.
- **Coordinate public and regulatory communication.** What is said publicly must be consistent with what is reported to regulators or partners; coordinate so a public statement does not contradict a regulatory filing.
- **Prepare leadership for external visibility.** For high-severity incidents, leadership may need to communicate publicly (a CEO statement, a customer letter); prepare them with accurate, approved, audience-appropriate content.

### Manage Post-Incident Communication And Trust Repair

The incident does not end when service is restored; the communication that follows shapes whether trust is rebuilt:

- **Send a resolution update and a follow-up summary.** Confirm resolution, describe impact and duration honestly, and commit to a follow-up (a public post-mortem summary or customer communication) once the analysis is complete.
- **Communicate remediation, not just what happened.** Customers and stakeholders want to know what will change to prevent recurrence. A summary that explains the cause and the concrete remediation rebuilds trust; one that stops at "it's fixed" does not.
- **Be honest about impact in the post-incident summary.** Downplaying the impact (duration, affected users, data exposure) in the summary, when the truth emerges later, destroys the trust the incident response built. State the impact accurately.
- **Handle apologies and compensation deliberately.** Where an apology or compensation is warranted, make it genuine, timely, and consistent with the actual impact; a token apology for a severe incident, or a generous one for a trivial one, rings false.

## Common Traps

### Silent Gaps That Breed Speculation

Going silent for an extended period while debugging, so customers refresh the status page and speculate on social media. Communicate early and on a cadence, even when the update is "still investigating."

### Premature Root-Cause Claims That Get Retracted

Stating a confirmed cause or precise ETA before it is established, then retracting it, which makes later claims suspect. Separate known facts from suspected causes, avoid overclaiming, and correct earlier statements openly when they change.

### Jargon That Confuses Non-Technical Audiences

Using internal architecture terms in customer or executive updates, which is meaningless or alarming to the audience. Translate to impact and action for each audience.

### Inconsistent Messages Across Channels

A status page, customer email, internal chat, and social media telling different stories, because there is no single source of truth. Maintain one authoritative current state and derive all channels from it.

### Communicating Without A Dedicated Role

Expecting the incident commander to draft updates between technical steps, producing late and inconsistent communication. Assign a dedicated communicator who owns the comms workstream from incident start.

### Waiting For Certainty Before Saying Anything

Staying silent until the root cause is known, creating a long gap that erodes trust. Acknowledge early and update on cadence; communicate what is known now and mark what is suspected.

### Downplaying Impact In The Post-Incident Summary

Minimizing the duration, affected users, or data exposure in the follow-up, when the truth emerges later and destroys the trust built during the response. State impact accurately and communicate concrete remediation.

## Self-Check

- [ ] Communication is a parallel workstream with a dedicated communicator/scribe distinct from the incident commander, stood up at incident start with a running timeline and decision log.
- [ ] The first update goes out early (acknowledging the issue is known and being worked), and a regular cadence is set and kept, with off-cadence updates on material change, so there are no silent gaps.
- [ ] Known facts are separated from suspected causes; root causes are not claimed until confirmed; ETAs are given as ranges or conditions and updated if slipped; earlier statements are corrected openly when they change.
- [ ] Each update is written for its audience: customer-facing updates focus on impact, data safety, and action; stakeholder updates add context and business impact; executive briefings are concise and decision-oriented; jargon is translated for non-technical channels.
- [ ] A single source of truth holds the current status, impact, and latest update; all channels (status page, customer notifications, internal chat, social, support) derive from it and stay consistent, sequenced internal-first.
- [ ] Severity, escalation, and external obligations are handled: communication scales with severity, contractual/regulatory notification deadlines are tracked and met, public and regulatory communication are coordinated, and leadership is prepared for external visibility.
- [ ] Post-incident communication rebuilds trust: a resolution update and follow-up summary are sent, remediation (not just what happened) is communicated, impact is stated honestly (not downplayed), and apologies/compensation are genuine and proportionate.
- [ ] The highest-risk cases were verified — a long silent gap, a retracted root-cause claim, jargon reaching customers, inconsistent channels, a missed regulatory notification deadline, and a downplayed post-incident summary — not only the clean single update.
