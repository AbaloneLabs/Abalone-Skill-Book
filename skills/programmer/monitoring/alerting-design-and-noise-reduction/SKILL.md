---
name: alerting_design_and_noise_reduction.md
description: Use when the agent is designing, reviewing, or tuning alerts and on-call notifications — deciding what to alert on, choosing thresholds, building SLO/SLI-based alerts, distinguishing page vs ticket alerts, reducing alert fatigue, setting escalation policy, or auditing an existing alerting setup that pages too often or is ignored. Covers symptom-based vs cause-based alerting, burn-rate and multi-window SLO alerts, actionable alert design, runbook linkage, alerting on user impact rather than raw resource usage, page vs ticket vs dashboard distinctions, and measuring alert quality (precision, recall, actionability). Also use when an alert fires but no one acts, when every alert is high severity, or when on-call fatigue is eroding trust in the alerting system.
---

# Alerting Design And Noise Reduction

An alert is a demand for a human's attention. That is its entire definition, and it is also its entire cost. Every alert you send consumes someone's focus, interrupts their sleep, trains them to trust or distrust the system, and competes with every other alert for the scarce resource of human attention. An alert that fires and is ignored is worse than no alert at all: it has paid the full cost of attention while providing zero value, and it has taught the operator that alerts in this system can be safely dismissed — which means the next alert, the real one, will be dismissed too. The judgment problem is not "what can we detect" but "what must a human act on right now, and how do we make sure every page earns that interruption."

Agents tend to over-alert, because the safe-feeling thing to do is to wrap every metric in a threshold and notify on every anomaly. This is exactly backwards. The goal of alerting is not coverage — it is precision. A system that pages on every CPU spike, every error log, every pod restart, every percentile blip produces so much noise that the signal — the one alert that means a user is being harmed — is buried and ignored. Alert fatigue is not an annoyance; it is the failure mode of the alerting system, and it is far more dangerous than missing a metric, because it causes real incidents to be missed. The judgment problem is deciding, for each candidate signal, whether it represents user-visible harm that needs a human now, and ruthlessly refusing to alert on anything that does not.

This skill focuses on alert design and noise reduction. It is distinct from the structured-logging skill (which covers what to record) and the resilience skill (which covers retries, timeouts, and circuit breakers that handle problems automatically). Here the question is: given the signals the system produces, which of them should become a demand for a human's attention.

## Core Rules

### Alert Only On What Requires A Human To Act Now

The defining test of an alert is: "if this fires, is a human required to take action right now?" If the answer is no — the system will self-heal, the metric will recover, the anomaly is interesting but not harmful, or the information is for later analysis — it is not an alert. It is a dashboard, a ticket, a log, or a metric. Candidates that almost never meet this bar: CPU/memory utilization on a single node (autoscale handles it), a single error log line (the error budget handles it), a pod restart (the orchestrator handles it), a percentile latency blip under the SLO. Candidates that usually do: the error rate has exceeded the SLO budget and users are failing, a critical dependency is down with no fallback, data integrity is violated, a queue backlog is growing unbounded and will exhaust capacity.

For every alert, answer explicitly: who acts, what action they take, and what happens if no one acts. If you cannot name the action or the consequence of inaction, the alert is not actionable and should not page.

### Alert On Symptoms (User Impact), Not Causes

The most reliable alerts describe what the user experiences, not what the system is internally doing. A user does not care that CPU is high or that a cache hit rate dropped; they care that requests are failing or slow. Symptom-based alerting is more robust because:

- **Symptoms are stable across causes.** "Error rate above 2%" fires whether the cause is a bad deploy, a dependency outage, a database failover, or a config error. You do not need to predict every cause; you monitor the outcome.
- **Cause-based alerts miss unknown failures.** Alerting on "disk full" catches disk-full; it does not catch the next new failure mode. Alerting on "requests failing" catches everything that makes requests fail.
- **Symptoms map to user-visible SLOs.** The alert directly answers "are users being harmed?" rather than "is something internally unusual?"

Use cause-based signals (CPU, queue depth, connection count) for diagnosis inside the incident and for lower-severity warnings, but make the page fire on the symptom. The hierarchy is: page on symptoms, investigate with causes, warn on causes only when they reliably precede a symptom with enough lead time to act.

### Build Alerts Around SLOs And Error Budgets

The strongest alerting foundation is an SLO (service level objective): a target for user-visible reliability, such as "99.9% of requests succeed within 800ms over 28 days." The SLO defines an error budget — the amount of unreliability you can afford in the window — and alerts fire when the budget is being consumed too fast. This is superior to static thresholds because:

- **It is tied to user impact.** The SLO is defined in terms users experience, so breaching it means real harm.
- **It tolerates normal noise.** A 0.1% error rate is fine if the SLO allows it; you do not page on every error.
- **It supports burn-rate alerting.** You can page when the budget is burning fast enough that the window's objective will be missed (fast-burn, short window) and ticket when it is burning slowly but persistently (slow-burn, long window).

A typical pattern is multi-window multi-burn-rate: page if a short window (e.g., 5m) and a long window (e.g., 1h) both exceed a high burn rate (so you catch acute outages without paging on a single blip), and ticket if a long window exceeds a lower burn rate (so chronic degradation is noticed without paging). This dramatically reduces false pages while preserving detection of real incidents.

### Separate Page, Ticket, And Dashboard By Required Response

Not all signals need the same urgency, and conflating them is the main source of fatigue. Use three tiers with distinct contracts:

- **Page (real-time, interrupts a human).** Reserved for user-impacting, time-critical conditions that need action now. These should be rare, precise, and always actionable. If your page volume is more than a handful per service per week, you are over-paging.
- **Ticket (async, handled in business hours).** Conditions that need human attention but not immediately: slow error-budget burn, degraded-but-functional behavior, capacity trends that will become problems. These get fixed before they become pages.
- **Dashboard (no notification, inspected on demand).** Informational signals: resource utilization, throughput, internal counters. Useful during diagnosis and planning; never wake anyone.

The discipline is putting each signal in the lowest tier that still gets it handled. Defaulting everything to "page, just in case" is the disease; the cure is honest triage of urgency.

### Set Thresholds From Data, Not Intuition

A threshold pulled from thin air ("alert if latency > 500ms") is a guess, and guesses produce either constant false alarms or missed incidents. Set thresholds from observed behavior:

- **Look at the normal distribution first.** What does the metric do on a normal day, at normal peak, during a normal deploy? A threshold inside the normal range will fire constantly; one far outside may never fire until damage is done.
- **Account for seasonality and deploys.** Latency and error rate naturally vary by time of day, day of week, and release. A static threshold that ignores this fires on every Monday morning rush.
- **Prefer relative and burn-rate thresholds over absolute.** "Error rate is 5x the baseline for 10 minutes" adapts as the system grows; "error rate > 1%" rots as traffic changes.
- **Validate against historical incidents.** Would this threshold have fired for the last real outage? Would it have stayed silent during the last quiet week? If you cannot answer, the threshold is untested.

Treat thresholds as hypotheses to validate against data, then tune. A threshold that has never been reviewed since it was written is almost certainly wrong.

### Require Every Alert To Have A Runbook And An Owner

An actionable alert is actionable only if the responder knows what to do and who is responsible. For every alert:

- **Link a runbook** that describes what the alert means, how to investigate it, the likely causes, and the first mitigation steps. An alert with no runbook forces the responder to start from scratch under time pressure.
- **Assign an owning team.** Alerts with no owner get ignored; alerts owned by everyone are owned by no one. Route to the team that can actually fix the underlying system.
- **Include the context needed to start.** The alert should carry the SLO, the current burn rate, the scope (which service, region, tenant), and links to the relevant dashboard and logs — not just "threshold exceeded."

An alert without a runbook and an owner is half an alert. It detects; it does not enable response.

### Make Alerts Self-Healing-Aware: Do Not Page What Automation Handles

If a condition is handled automatically — autoscaling, retries, circuit breakers, failover, queue backpressure — alerting on the raw condition pages for a non-problem. CPU high? If autoscale is working, that is the system doing its job, not an incident. Pod restarting? If the orchestrator reschedules it, that is normal operation. Page only when the automatic response has failed or is insufficient: autoscale has hit its ceiling and latency is now breaching the SLO; restarts are looping and availability is dropping. The alert should fire when the user is harmed despite the automation, not when the automation is merely doing its work.

### Measure And Continuously Tune Alert Quality

Alerting is not a set-and-forget system; it decays as the system changes. Measure its quality and tune it:

- **Track pages per week per service and the trend.** Rising page volume without rising incident quality is fatigue setting in.
- **Track the actionable rate.** What fraction of pages led to a real response (a fix, a rollback, an investigation)? Pages that consistently result in "nothing wrong, closed it" are false positives to be eliminated.
- **Track time-to-detection and time-to-mitigation for real incidents.** If real incidents are detected late or by users first, recall is too low.
- **Hold regular alert reviews.** Every page from the last period is examined: was it actionable, was the threshold right, should it be a ticket instead, should it exist at all? Ruthlessly retire alerts that did not earn their interruption.

The goal is a system where every page is trusted because every page has earned it. Trust is built by deleting bad alerts as aggressively as adding good ones.

## Common Traps

### Alerting On High CPU, Memory, Or Disk By Default

Resource utilization is a cause, not a symptom, and it is usually handled by automation. Paging on "CPU > 80%" fires constantly under normal load, trains operators to ignore it, and says nothing about whether users are harmed. Alert on the symptom (latency, errors, saturation that actually drops availability) and use resource metrics for diagnosis.

### Every Alert Is High Severity

Marking everything "critical" or "page" because each one seems important in isolation. The result is that severity means nothing and everything pages. Reserve the top severity for the small set of conditions that truly need immediate human action; push the rest down.

### Static Thresholds That Ignore Baseline And Seasonality

"Error rate > 1%" on a service whose normal rate is 0.01% is fine; on a service whose normal rate is 2% it pages forever. Thresholds set without looking at the metric's actual behavior produce constant false positives or missed incidents. Derive thresholds from data and revisit them as the system grows.

### Alerting On A Single Sample Or Short Blip

Firing on one data point above threshold, which catches every transient spike and network hiccup. Use windows and sustained conditions (e.g., "above threshold for N consecutive minutes" or multi-window burn rate) so a single blip does not wake someone.

### Cause-Based Alerts That Fire But The System Is Fine

Alerting on "cache hit rate dropped" or "replica lag increased" when users are unaffected because of fallbacks or tolerance. The page fires, the responder investigates, finds nothing wrong, and loses trust. Make the page fire on the user-visible symptom; keep the cause as a diagnostic signal.

### Alert With No Runbook Or Owner

A page that says "queue depth high" with no guidance on what it means, what to check, or who owns it. The responder burns the first minutes of the incident figuring out what the alert even is. Every alert needs a linked runbook and a routed owner.

### Paging On What Automation Already Handles and treating Dashboards As Alerts

CPU-driven autoscale, pod rescheduling, transient retries. These are the system working, not incidents. Page only when the automation is exhausted or failing and users are being harmed.

Building elaborate dashboards and expecting someone to watch them. No one watches dashboards in real time; if a condition needs a human now, it must be an alert, not a chart. Conversely, do not alert on everything that appears on a dashboard — most dashboard data is for inspection, not interruption.

### Never Retiring Alerts and alerting On Error Logs Instead Of Error Rate

Adding alerts over time and never removing them, so the set grows monotonically and noise compounds. Alerts rot: the system changes, the threshold becomes wrong, the owning team moves on. Without periodic review and retirement, the alerting system drowns in stale, low-quality pages.

A single ERROR log triggers a page. Errors are expected within the budget; the alert should fire on the rate or budget burn, not on one line. Page on the aggregate signal that means users are affected, not on individual log entries.

## Self-Check

- [ ] Every alert passes the actionability test: a specific human is required to take a specific action right now, and the consequence of inaction is named — alerts that fail this test are tickets, dashboards, or deleted, not pages.
- [ ] Pages fire on symptoms (user-visible error rate, latency, availability, data integrity), not on internal causes alone; cause-based metrics are used for diagnosis and at most for lower-severity warnings with real lead time.
- [ ] Critical alerts are grounded in SLOs and error budgets (ideally multi-window, multi-burn-rate), so they fire on user impact and tolerate normal noise rather than paging on every blip.
- [ ] Three tiers are used distinctly: page for real-time user-impacting action, ticket for async attention, dashboard for on-demand inspection — and each signal is in the lowest tier that still gets it handled.
- [ ] Thresholds were derived from observed data (baseline, seasonality, deploy variance) and validated against historical incidents and quiet periods; they are not guesses, and relative/burn-rate thresholds are preferred where the system grows.
- [ ] Every alert has a linked runbook (meaning, likely causes, first mitigation steps), a routed owning team, and carries the context (SLO, burn rate, scope, dashboard and log links) needed to start response immediately.
- [ ] No alert pages for conditions that automation handles (autoscale, retries, failover, rescheduling); pages fire only when automation is exhausted or failing and users are harmed.
- [ ] Alert quality is measured (pages/week, actionable rate, time-to-detection, time-to-mitigation) and reviewed periodically, with low-quality alerts retired as aggressively as new ones are added.
- [ ] Alerts use sustained windows or multi-window conditions, so a single sample or transient blip does not page; and severity is not inflated — only a small set of truly time-critical conditions are top severity.
- [ ] No alert fires on individual error logs or single events; the page fires on aggregate rate or error-budget burn that indicates users are affected.
