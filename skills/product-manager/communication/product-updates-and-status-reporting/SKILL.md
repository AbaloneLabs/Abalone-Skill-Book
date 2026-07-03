---
name: product_updates_and_status_reporting.md
description: Use when the agent is writing a product update, status report, or release note, choosing what to communicate to which audience, planning the cadence of product communications, or reporting progress without noise.
---

# Product Updates And Status Reporting

A status update is not a log of activity; it is a tool for creating the right visibility at the right time for the right audience. Done well, it lets leadership trust the work without inspecting it, lets dependent teams plan around you, and lets customers understand what changed. Done poorly, it generates noise that trains every audience to ignore it, hides deteriorating status behind optimistic language, or reports motion as progress. The craft is distinguishing signal from noise and matching the message to what each audience actually needs to know.

Use this skill before writing a weekly update, a leadership status report, a release note, or a cross-team progress message, and before setting the cadence of product communications. The goal is to prevent the agent from producing updates that no one reads, obscuring risk behind green status, or reporting activity instead of outcomes.

## Core Rules

### Tailor Content To Each Audience

A single update does not serve all audiences, because each cares about different things. Sending everyone the same dense engineering status guarantees that leadership skims and customers bounce.

Calibrate by audience:

- the team: blockers, decisions needed, what changes for their work;
- leadership: status against commitments, risks, asks, confidence in dates;
- the company: what shipped, why it matters, what is coming;
- customers: what changed for them, how to use it, what to expect next.

Write each update for the audience that will read it. An update that tries to serve everyone serves no one.

### Choose Cadence Deliberately

Cadence is a tradeoff between visibility and fatigue. Too frequent and audiences stop reading; too rare and you lose the ability to surface issues early. Cadence should follow the decision velocity of the work, not a calendar habit.

Match cadence to need:

- weekly: active delivery with shifting blockers and dependencies;
- biweekly or monthly: steady work where status rarely changes;
- per-release: customer-facing change summaries tied to ship dates;
- event-driven: escalations and status changes the moment they occur.

When status is stable, reduce frequency rather than manufacturing updates. A report that says "no change" three weeks running should become less frequent.

### Distinguish Signal From Noise

Most updates fail by reporting everything that happened rather than what changed the reader's decisions. Signal is information that changes what the audience should do or expect. Noise is activity that does not.

Keep signal by asking of each item:

- does this change a decision the audience owns;
- does this change a date, scope, or risk they depend on;
- does this require action from them;
- would omitting it cause a surprise later.

If the answer to all is no, it is noise. Cut it, or relegate it to a reference appendix. Updates that are mostly noise get ignored, including the signal buried in them.

### Separate Progress From Motion

Motion is activity; progress is movement toward an outcome. "Held four meetings and wrote three tickets" is motion. "Removed the dependency blocking QA, putting us back on the committed date" is progress. Audiences conflate the two only when the update lets them.

Report progress by:

- stating movement against the committed outcome or date;
- naming what is now unblocked or de-risked;
- distinguishing work done from work remaining;
- avoiding hour counts and meeting logs as proxies for results.

If you can only report motion, the work may be stalled, and the update should say so rather than disguise it.

### Report Status Honestly, Including Red And Amber

Status reporting fails most when it drifts optimistic. Green that hides risk destroys trust the moment the risk materializes, and recovery is harder once credibility is gone. Honest amber and red, reported early, are acts of professionalism, not pessimism.

Report status by:

- green: on track to committed outcome, no material risk;
- amber: at risk, with the specific blocker and the mitigation underway;
- red: will miss unless something changes, with the decision or help needed.

Never report green with an undocumented "we think we can recover." If recovery depends on an assumption, status is amber. The rule that protects trust is simple: status reflects reality, not hope.

### Communicate Delays And Changes Without Losing Trust

Delays and scope changes are inevitable; how they are communicated determines whether trust survives. Hiding a delay, burying the cause, or announcing it late compounds the damage. Early, causal, and solution-oriented communication preserves credibility.

When communicating a change:

- state what changed and the new expectation plainly;
- explain the cause without deflecting or over-explaining;
- name the impact on dependent work and audiences;
- describe the mitigation or the decision being made;
- give the next checkpoint so the audience knows when to expect certainty.

Audiences forgive delays they understand and were warned about. They do not forgive surprises.

### Write Release Notes Users Actually Read

Release notes are the rare update customers choose to read, and most are wasted because they describe internals customers do not care about. A release note should answer one question for the user: what is now different for me, and why should I care.

Make release notes effective by:

- leading with the user benefit, not the engineering change;
- writing in the user's language, not internal jargon or ticket IDs;
- grouping by audience or impact, not by team;
- including what to do or what to expect, not just what shipped;
- keeping them short enough to actually be read.

A release note that reads like a changelog serves the writer, not the reader.

### Use Updates To Create Visibility, Not Just Report

The highest-value updates do not merely report state; they shape what happens next. A good update surfaces a decision that needs making, a dependency that needs unblocking, or a risk that needs attention, and prompts the action that resolves it.

Engineer updates to drive outcomes by:

- ending with explicit asks or decisions needed;
- naming owners and due dates for open items;
- flagging risks early enough to act on them;
- making dependencies visible to the people who can resolve them;
- closing the loop by reporting when prior asks were resolved.

An update that changes nothing is a report. An update that prompts a decision or unblocks work is leverage.

## Common Traps

### Reporting Motion As Progress

Listing meetings, tickets, and hours creates the impression of momentum while obscuring whether the outcome moved. Audiences who cannot tell the difference lose trust, and the work may be stalled behind the activity.

### Drifting Optimistic On Status

Reporting green because the team hopes to recover hides risk until it is too late to act. The short-term comfort of optimism costs the long-term credibility of a late surprise. Status must reflect reality, including uncomfortable amber.

### One-Size-Fits-All Updates

Sending the same update to the team, leadership, and customers ensures each gets mostly irrelevant content and starts skipping it. Audience-tailoring is not extra work; it is the work that makes the update read.

### Update Fatigue From Over-Communicating

Reporting on a fixed cadence regardless of whether anything changed trains audiences to ignore the channel. When nothing meaningful moved, reduce frequency or say so plainly rather than manufacturing content.

### Burying The Ask

An update that lists status but never states what decision or help is needed leaves the audience passive. If the update does not prompt an action when one is needed, it has not created visibility, only a log.

### Changelog-Style Release Notes

Release notes written as internal change descriptions, with ticket numbers and engineering terms, are unreadable to customers and waste the one channel they opt into. They serve the team's memory, not the user's understanding.

### Hiding Delays Or Announcing Them Late

Concealing a slip to avoid a difficult conversation makes the eventual disclosure worse, because the audience learns both the delay and that it was withheld. Early honesty is cheaper than late surprise.

## Self-Check

- [ ] Each update is tailored to its specific audience's concerns, not broadcast identically to all.
- [ ] Cadence matches the decision velocity of the work and is reduced when status is stable.
- [ ] The update reports signal that changes decisions or expectations, with noise cut or relegated.
- [ ] Progress is reported against the committed outcome, not as motion or activity logs.
- [ ] Status is honest: amber and red are reported early with specific blockers and mitigations, not hidden behind optimistic green.
- [ ] Delays and scope changes are communicated early, with cause, impact, mitigation, and a next checkpoint.
- [ ] Release notes lead with user benefit in user language, grouped by impact, and are short enough to be read.
- [ ] Updates end with explicit asks, named owners, and due dates where action is needed.
- [ ] Risks and dependencies are surfaced early enough for the audience to act on them.
- [ ] Prior asks are closed the loop, so the update channel is trusted to drive outcomes.
