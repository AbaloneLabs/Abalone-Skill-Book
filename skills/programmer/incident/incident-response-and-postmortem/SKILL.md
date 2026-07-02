---
name: incident_response_and_postmortem.md
description: Use when the agent is responding to a production incident or outage, acting as incident commander, coordinating responders, deciding whether to roll back or roll forward, writing or reviewing a postmortem, running a blameless retrospective, triaging severity and escalation, drafting customer or public communications during an incident, or tracking incident action items to completion. Also covers SEV classification, war-room coordination, restore-vs-investigate tradeoffs, hotfix risk, and deciding what an incident should actually change about the system. Use when an incident is live, when a postmortem is due, or when reviewing whether past incidents produced lasting improvements.
---

# Incident Response And Postmortem

An incident is a period where the system is harming users and a group of humans must coordinate, under time pressure and incomplete information, to stop the harm. The technical work of finding the broken component is only one part; the rest is deciding who does what, what to communicate, when to stop investigating and start restoring, and — after the system is healthy again — what the organization must actually learn so the same harm does not recur. An incident handled technically well but coordinated poorly lasts longer, scares customers, and burns out responders. An incident followed by a postmortem that assigns blame and produces no change is an incident that will repeat.

Agents tend to miss the coordination and learning layers because they are less concrete than the bug. The instinct is to dive into logs and fix the code, which is necessary but insufficient: without an incident commander the responders work in parallel and contradict each other; without deliberate communication the silence is filled by rumor and customer panic; without a blameless postmortem that produces tracked action items, the outage becomes a story rather than a fix. The judgment problems are: during the incident, what do we do *first* and who decides; after the incident, what do we genuinely change. This skill covers the operational and organizational discipline of incidents. The technical method of isolating a root cause is covered by the systematic-debugging skill, and detecting that an incident is happening is covered by the alerting skill.

## Core Rules

### Assign Roles Before You Need Them, And Separate Them During The Incident

An incident needs at least three distinct roles, and conflating them is the most common coordination failure:

- **Incident Commander (IC).** Owns the process and the decisions, not the keyboard. The IC tracks timeline, chooses between restore and investigate, declares severity changes, calls for escalation, and decides when the incident is resolved. The IC should not be the one typing the fix, because someone deep in a terminal cannot also hold the whole picture.
- **Communications / Scribe.** Owns what the outside world and the rest of the org hear. Posts status updates on a fixed cadence, drafts customer messaging, updates the status page, and keeps a running timeline of decisions and evidence. This role exists because if nobody owns it, communication happens late, inconsistently, or not at all.
- **Investigator(s) / Responder(s).** The people reading logs, running commands, and applying mitigations. They form hypotheses and propose actions, but the IC decides which action to take.

The separation matters because the pressures conflict. The investigator wants to keep digging; the IC must decide when digging stops and restoring starts. The scribe wants accurate updates; the responder wants to not be interrupted. Without a named IC, the team defaults to consensus under stress, which means the loudest or most senior voice wins, decisions are not logged, and nobody is accountable for the timeline. Assign the IC explicitly at the start of every incident, even a small one — the cost is one sentence, and the benefit is a coherent response.

### Classify Severity By User Impact, Not By Internal Alarm

Severity decides who gets woken, how fast escalation happens, and how much risk is acceptable to restore service. Classify by what users experience, not by how loud the alert is or how scary the log looks:

- **SEV1 / critical.** Users are actively harmed at scale: the service is down, data is being lost or corrupted, payments are failing, a security breach is in progress, or a core workflow is broken for all or most customers. Wake people, form a war room, escalate to leadership, and accept meaningful risk to restore service.
- **SEV2 / major.** A significant feature or a large subset of users is affected, with degraded but available service, or a workaround exists but is painful. Page the owning team; do not necessarily escalate to leadership.
- **SEV3 / minor.** Limited impact, a clean workaround exists, or the issue affects a small number of users. Handle in business hours; track but do not interrupt.

The discipline is to classify honestly. Inflating everything to SEV1 burns out responders and desensitizes leadership; downgrading a real outage to SEV3 leaves customers hanging. Reclassify as the situation changes — an incident that starts as SEV3 and spreads must be upgraded, and the IC should say the new severity out loud so everyone recalibrates. Tie severity to the SLOs and error budgets the system already defines: burning the budget fast enough to miss the window is a high-severity signal regardless of the underlying cause.

### Restore Service Before You Understand The Cause (Usually)

The central tension of incident response is that the fastest way to stop user harm and the fastest way to understand the bug are usually different actions. During an active outage, the priority order is: stop the harm, then restore service, then understand the cause, then fix it permanently.

- **Stop the harm first.** If data is being corrupted, stop the corrupting process even before you know why it corrupts. If a deploy is poisoning traffic, cut it off. Reversing a harmful action almost always takes priority over explaining it.
- **Restore service by the cheapest reliable means.** Roll back, fail over, increase capacity, route around the broken component, or serve a degraded path. The goal is a healthy system, not a correct explanation.
- **Then investigate.** Once users are no longer failing, take the time to find the real cause. An incident that was "fixed" by a rollback but never root-caused will recur, because the broken code is still in the repository waiting for the next deploy.

The exception is when restoration *requires* understanding: you cannot safely roll back a migration that ran halfway, you cannot fail over if you do not know which replica is consistent, and you cannot roll forward if you do not know which change broke things. In those cases investigation is the restoration path. The judgment is recognizing which situation you are in. Default to restore-first; switch to investigate-first only when every restore option is unsafe without more knowledge, and say that decision explicitly so the team aligns.

### Treat Rollback As The Default, Roll Forward As A Deliberate Choice

When a recent change caused the incident, the instinct is often to "just fix it forward" — write a patch and deploy it. Under time pressure this is usually the wrong call:

- **Rollback is fast and known.** You are returning to a state that was working. Its behavior is understood. It stops the harm now.
- **Roll forward is slow and unverified.** A hotfix written during an outage has not been tested, was authored under stress, and is being shipped into a system that is already misbehaving. It can introduce a second, independent failure on top of the first, turning a single incident into a compound one.

Prefer rollback unless rolling back is impossible or more dangerous than the alternative. Roll back is impossible when: the change included an irreversible migration, the old version is incompatible with current data, or external state (a third-party API change, a deleted resource) cannot be undone. Roll back is more dangerous when: the old version has a known worse bug, or the forward fix is trivial and already validated. When you choose roll forward, name why rollback was not viable, and apply the same release safety (canary, gradual rollout) you would in peacetime — an outage is not a license to skip it.

### Communicate On A Fixed Cadence, Internally And Externally

Silence during an incident is interpreted as either incompetence or cover-up. Decide a cadence and hold it:

- **Internal.** The IC posts a status update to the incident channel on a regular interval (e.g., every 15–30 minutes for a SEV1), even if the update is "still investigating, no change." Regular no-change updates prevent the channel from filling with "any updates?" noise and let stakeholders stand down.
- **External / customer.** Post a status page update early — as soon as the incident is confirmed and scoped, not after it is resolved. Customers tolerate "we are aware of elevated errors and are investigating" far better than they tolerate discovering the outage themselves. Update on the same cadence, and post a resolution update when service is restored.

Good incident communication states what is known, what is unknown, what is being done, and when the next update will come. It avoids speculation about cause before the cause is confirmed, avoids promising resolution times you cannot guarantee, and never minimizes the impact customers are experiencing. A status update that says "minor degradation" while customers cannot log in destroys trust faster than the outage itself.

### Escalate Early When The Incident Exceeds The Current Responders

Escalation is not failure; it is routing. The cost of an unnecessary escalation is a few minutes of someone's attention. The cost of a missing escalation is an incident that runs hours longer than it should because the responders lacked authority, access, or knowledge.

Escalate when:

- the responders do not have access to the affected system or data;
- the incident crosses team or service boundaries and no one owns the whole;
- the mitigation requires a decision the responders are not authorized to make (spending money, taking a dependency offline, disclosing externally);
- the incident is not improving after a reasonable window, or is getting worse;
- the impact is severe enough that leadership needs to know regardless of progress.

The rule of thumb: if you are wondering whether to escalate, escalate. Define escalation paths in advance — who pages the on-call for another team, who reaches leadership, who handles legal or security disclosure — so that during the incident the path is "follow the runbook," not "figure out who owns this."

### Run A Blameless Postmortem Whose Output Is Change, Not Narrative

A postmortem is not a report; it is a mechanism for the organization to learn. Its purpose is to find the systemic conditions that made the failure possible and to change them, so the same class of incident cannot recur. Two principles govern it:

- **Blameless.** The postmortem focuses on systems and processes, not on which individual made a mistake. When a human took an action that caused the incident, the question is not "who is at fault" but "what about the system made that action possible, likely, or reasonable at the time." People act rationally given the information, tools, and incentives they have. Blame produces defensiveness, hidden incidents, and zero improvement; blameless analysis produces psychological safety and real fixes. This does not mean ignoring negligence or performance problems — it means handling those through separate people processes, not through the postmortem.
- **Action-oriented.** A postmortem without tracked action items is a story. Every contributing factor should produce one or more concrete actions: a code change, a test, an alert, a runbook, a guardrail, a process change, a documentation update. Each action has an owner and a due date and is tracked to completion, not filed and forgotten.

The strongest postmortems capture the timeline (what happened and when, with evidence), the impact (who was affected, for how long, measured against the SLO), the root cause and contributing factors (not just the trigger), what went well in the response, what went poorly, and the action items. They distinguish the *trigger* (the deploy that started it) from the *root cause* (why that deploy could cause an outage, why it was not caught, why rollback was hard) — fixing only the trigger leaves the root cause intact.

### Track Action Items To Completion, And Verify They Actually Prevent Recurrence

The most common postmortem failure is the action item that is filed, assigned, and never done — or done in a way that does not actually address the risk. Treat action items like real work:

- Each has a single owner, a due date, and a tracking issue.
- They are reviewed in a recurring forum until closed.
- "Fixed" means the contributing factor is addressed, not that a ticket was moved. An action item "add a test for X" is only closed when the test exists, runs in CI, and would have caught the original failure.
- Prioritize by how much recurrence risk they remove. The action that closes the root cause matters more than the action that polishes a runbook.

Periodically audit whether past incidents produced real change. If the same failure mode keeps recurring, the postmortem process is producing documents, not learning — and the response is to fix the learning loop, not to write a harsher postmortem next time.

## Common Traps

### Everyone Investigates, Nobody Commands

The responders all dive into terminals, form separate mental models, and pursue different fixes in parallel. Decisions are not coordinated, two people apply contradictory mitigations, and nobody is recording the timeline. This feels productive because everyone is busy, but it extends the incident. Assign an IC whose only job is to coordinate and decide.

### Investigating Instead Of Restoring Because "We're Close"

The team is on the trail of the root cause and keeps digging instead of rolling back, because understanding feels like progress and rolling back feels like giving up. Meanwhile users keep failing. During an active outage, restoration beats understanding unless restoration is impossible without understanding. Roll back, stop the harm, then investigate at leisure.

### The Heroic Hotfix That Causes A Second Incident

Under pressure, someone writes a one-line fix, ships it directly to production without tests or gradual rollout, and it interacts with the already-broken system to create a new failure. The original incident is now compounded. Treat hotfixes with the same release discipline as normal changes, scaled to the urgency; prefer rollback when the hotfix is not yet validated.

### Silence On The Status Page Until It's Over

Waiting until the incident is resolved to post any external update. Customers have already noticed, opened tickets, and lost trust; the eventual "resolved" post reads as cover-up. Post early, post on a cadence, and be honest about impact.

### The Postmortem That Finds "Human Error"

The root cause is recorded as "the engineer deployed the wrong config" and the action item is "be more careful." This explains nothing and prevents nothing, because it does not ask why a wrong config was deployable, why it was not caught, why the system was fragile to it. Blameless analysis asks what system change would have made the error impossible or caught it; "be more careful" is not a system change.

### Action Items Filed And Forgotten

The postmortem lists eight action items, they are assigned to tickets, and six months later half are still open and two have recurred as incidents. Without tracking to completion and verifying the fix actually closes the risk, the postmortem was ceremony. Action items are debt that must be paid down.

### Inflating Severity To Force Action

Marking every issue SEV1 so that people respond, because lower severities are ignored. This desensitizes responders, burns out on-call engineers, and means a real SEV1 does not stand out. If low severities are ignored, fix the response process for them; do not relabel them as critical.

### Treating The Trigger As The Root Cause

The postmortem concludes "a bad deploy caused the outage" and the action is "the deploy was reverted." But the deploy was only the trigger; the root cause is why that deploy could take down the system, why tests did not catch it, why rollback was slow, why there was no canary. Fixing only the trigger guarantees recurrence with a different trigger next time.

### No Decision Owner, So The Loudest Voice Directs

Without a named IC, the most senior or most assertive person in the channel ends up making calls, regardless of whether they have the best information. Decisions are not logged, dissent is suppressed, and the response is driven by hierarchy rather than evidence. Name the IC and let them decide based on the investigators' input.

## Self-Check

- [ ] Roles are assigned at the start of the incident: a named Incident Commander who is not typing the fix, a communications owner who posts on a cadence, and investigator(s) who propose actions the IC decides on.
- [ ] Severity is classified by user impact (and measured against SLOs/error budgets), not by alert volume; it is reclassified out loud as the situation changes, and escalation happens early when the incident exceeds the responders' authority, access, or knowledge.
- [ ] During the incident, the priority was to stop the harm and restore service (rollback, failover, degrade) before fully understanding the cause — unless restoration was impossible without investigation, in which case that decision was stated explicitly.
- [ ] Rollback was the default; roll-forward hotfixes were chosen only when rollback was impossible or more dangerous, and hotfixes still received release safety (tests, gradual rollout) rather than being shipped blind into a broken system.
- [ ] Internal and external communication happened on a fixed cadence starting early, stated knowns/unknowns/next-update time, avoided unconfirmed speculation and impact minimization, and did not wait for resolution before the first customer-facing update.
- [ ] A blameless postmortem was produced that focused on systems and processes, distinguished the trigger from the root cause and contributing factors, and generated concrete action items — not "be more careful."
- [ ] Every action item has a single owner, a due date, a tracking issue, and is reviewed to completion; "closed" means the contributing factor is actually addressed (e.g., a test that would have caught the failure exists and runs), not that a ticket was moved.
- [ ] The postmortem captured timeline with evidence, measured impact against the SLO, recorded what went well and poorly in the response, and the learning loop is verified by checking that past incidents did not recur with the same failure mode.
