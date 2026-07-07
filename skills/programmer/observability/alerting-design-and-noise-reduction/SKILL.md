---
name: alerting_design_and_noise_reduction.md
description: Use when the agent is designing alert rules, on-call notifications, paging policies, or alert thresholds and routing; reducing alert fatigue and false positives; deciding what is alert-worthy versus what belongs in a dashboard; tuning sensitivity, severity levels, and escalation; building SLO-based or symptom-based alerting; or reviewing an existing alerting setup that pages too often or misses real incidents. Covers what to alert on, symptom-based vs cause-based alerting, multi-burn-rate and SLO-driven alerts, severity and routing, and the operational discipline of keeping alerts actionable and low-volume.
---

# Alerting Design And Noise Reduction

An alert is an interruption of a human, and interruptions have a cost. An on-call engineer paged at 3am for a non-actionable alert loses sleep, loses trust in the alerting system, and is slower to respond to the next alert — which may be the real incident. The defining failure of alerting is alert fatigue: a system that pages frequently for conditions that do not require human action trains its operators to ignore pages, so when a genuine incident arrives, the page is dismissed or delayed. A system that generates hundreds of alerts a day is worse than one that generates none, because the noise hides the signal and the fatigue degrades response. Good alerting is not "alert on everything that might be wrong"; it is the disciplined selection of a small number of conditions that genuinely require human action, expressed as symptoms a user would notice, with thresholds tuned to catch real problems without firing on normal variation.

Agents tend to alert on causes rather than symptoms (a CPU spike that no user notices, a single error that self-resolves), to set thresholds by guess rather than by data, and to add alerts defensively ("we should know about this") without considering the paging cost. The judgment problem is recognizing that every alert is a tax on human attention, that the value of an alert is measured by whether a human can and should act on it, and that the goal is a small set of high-signal, actionable alerts tied to user-visible symptoms or SLOs, with everything else demoted to dashboards or records. This skill covers the discipline of designing alerting that catches real incidents without the noise: what to alert on, symptom-based design, SLO-driven multi-burn-rate alerting, threshold tuning, severity and routing, and the ongoing maintenance that keeps alerts useful.

## Core Rules

### Alert Only On Conditions That Require Human Action

The first question for any alert is: will a human act on this, and should they? If the answer to either is no, it is not an alert.

- **Alert on conditions that are actionable (a human can do something) and urgent (they should do it now).** A condition that no one can fix (a third-party outage with no workaround) is not actionable; a condition that can wait until business hours is not urgent. Both belong in a dashboard or a ticket, not a page.
- **Alert on symptoms, not causes.** A symptom is what the user experiences (errors, latency, unavailability); a cause is an underlying metric (CPU, memory, queue depth) that may or may not affect the user. Alerting on causes produces false positives (high CPU that does not affect users) and false negatives (users failing while all cause-metrics look normal). Alert on the user-visible symptom; investigate the cause during triage.
- **Do not alert on normal variation or expected events.** A metric that fluctuates (daily traffic cycles, a deploy causing a brief error spike) is not an incident. Alerts must distinguish abnormal from normal, typically via baselines, historical comparison, or thresholds set from data, not guess.
- **Reserve paging for what needs immediate human action.** Lower-severity issues (a degraded-but-functional service, a single failed background job) can notify asynchronously (email, chat, ticket) without paging. Match the urgency to the notification channel.

### Use SLO-Based, Multi-Burn-Rate Alerting For User-Facing Services

For user-facing services with defined SLOs (service level objectives — the error rate or latency targets), SLO-based alerting is the disciplined approach: it alerts based on how fast the error budget is being consumed, which directly reflects user impact.

- **Define SLOs (error rate, latency percentile) that capture user-visible quality.** An SLO is a target (e.g., 99.9% of requests succeed, 99% under 200ms) with a window (e.g., 30 days). It defines the error budget — the allowed amount of bad outcomes before the SLO is breached.
- **Alert on error-budget burn rate, not raw error rate.** A 1% error rate for a minute is not an incident if the 30-day budget absorbs it; a 1% error rate for an hour may burn the month's budget. Multi-burn-rate alerting fires fast alerts for high burn rates (a 2% spike burning the budget in an hour) and slower alerts for sustained moderate burn (0.1% over a day), catching both acute and chronic problems.
- **Use multiple windows and burn rates.** A common pattern: page if the burn rate exceeds 14.4x over 1 hour and 5 minutes (fast burn, acute incident); ticket if it exceeds 3x over 6 hours and 1 hour (slow burn, chronic problem). This catches both the sudden outage and the gradual degradation.
- **Tie alerts to the user-visible SLO, not to internal metrics.** An SLO-based alert fires because users are being affected (the error budget is burning), not because an internal metric looks odd. This keeps alerts tied to user impact.

### Set Thresholds From Data, Not Guesswork

A threshold set by guess (CPU > 80% is bad) fires on normal variation in some systems and misses problems in others. Thresholds must be derived from the system's actual behavior.

- **Baseline the metric before setting the threshold.** Collect data on the metric's normal range, daily/weekly cycles, and variance. Set thresholds relative to the baseline (e.g., 3 standard deviations above the mean, or the 99th percentile of normal behavior), not as absolute values.
- **Use anomaly detection for metrics without a fixed threshold.** For metrics with complex patterns (traffic, latency), anomaly detection (statistical baselines that adapt over time) can fire when behavior deviates from the learned normal, without a fixed threshold.
- **Tune thresholds against historical incidents and non-incidents.** For each candidate threshold, ask: would this have caught the last real incident? Would it have fired on normal operation (false positive)? Tune to maximize detection of real incidents while minimizing false positives.
- **Review and adjust thresholds over time.** A threshold correct at one traffic level may be wrong at another; as the system grows, baselines shift. Periodically review alert performance (precision, recall) and adjust.

### Define Clear Severity, Routing, And Escalation

An alert's severity determines who is notified and how fast they must respond; routing determines who; escalation determines what happens if no one responds. These must be defined before the alert fires, not improvised during an incident.

- **Define a small number of severity levels (typically 2-3).** Too many levels dilute meaning; two or three (e.g., page-immediately, notify-soon, ticket) suffice. Each level maps to a notification channel and response expectation.
- **Route alerts to the team that owns the affected service.** An alert routed to the wrong team wastes time; the on-call for the owning team can act fastest. Service ownership and alert routing must be aligned.
- **Define escalation for unacknowledged alerts.** If the primary on-call does not acknowledge a page within a defined time, escalate (to a secondary, to a manager). This ensures no alert is missed due to a sleeping or unavailable on-call.
- **Include actionable context in the alert.** An alert that says "error rate high" without context forces investigation before action. Include the service, the symptom, the current value, a link to the dashboard, the relevant runbook, and recent changes (deploys, config changes) that may correlate.

### Maintain Alert Quality Over Time

Alerting degrades without maintenance: new alerts are added for transient concerns and never removed; thresholds drift; services change. Periodic review keeps the alert set useful.

- **Review alert volume and signal regularly.** Track how often each alert fires, how many were actionable, and how many were false positives. Alerts with high false-positive rates need threshold tuning or retirement.
- **Retire alerts that are no longer relevant.** Services are deprecated, thresholds become obsolete, and concerns pass. Remove alerts that no longer correspond to a real risk.
- **Treat every false-positive page as a bug to fix.** A page that did not require action is a failure of the alert; investigate why it fired and adjust (threshold, condition, or retirement) so it does not repeat. Accepting false positives trains operators to ignore alerts.
- **Document the alert's intent and action in a runbook.** Each alert should have a runbook: what it means, why it fires, and what to do. A runbook turns an alert into a guided action rather than a mystery.

## Common Traps

### Alerting On Causes Instead Of Symptoms

Alerting on CPU, memory, or queue depth that does not affect users, producing false positives and missing user-impacting problems. Alert on user-visible symptoms (errors, latency).

### Thresholds Set By Guess

A threshold chosen without data, firing on normal variation or missing problems. Baseline the metric; set thresholds from data; tune against historical incidents.

### Alerting On Normal Variation Or Expected Events

Pages for daily traffic cycles, deploy spikes, or self-resolving blips. Distinguish abnormal from normal via baselines and multi-window conditions.

### Too Many Alerts (Alert Fatigue)

Hundreds of alerts a day, training operators to ignore pages and miss real incidents. Alert only on actionable, urgent conditions; demote the rest to dashboards.

### No Severity, Routing, Or Escalation

Alerts with undefined severity or routed to the wrong team, delaying response. Define severity levels, route to the owning team, and escalate unacknowledged alerts.

### Alerts Without Actionable Context

A page that says "high error rate" with no service, value, dashboard link, or runbook, forcing investigation before action. Include context that enables immediate response.

### Alerts Never Reviewed Or Retired

Alerts accumulating over time as services change, with obsolete thresholds and irrelevant conditions. Review volume and signal regularly; retire obsolete alerts.

### Single-Window Thresholds Missing Chronic Problems

A threshold that catches acute spikes but misses slow, sustained degradation that erodes quality over time. Use multi-burn-rate or multi-window alerting for both acute and chronic problems.

## Self-Check

- [ ] Every paging alert corresponds to a condition that is actionable (a human can do something) and urgent (they should do it now) — conditions that are not actionable or not urgent are demoted to dashboards, tickets, or asynchronous notifications, not pages.
- [ ] Alerts are symptom-based (user-visible errors, latency, availability) rather than cause-based (CPU, memory), so they reflect user impact and avoid false positives on internal metrics that do not affect users.
- [ ] For user-facing services with SLOs, alerting is SLO-based with multi-burn-rate and multi-window conditions (fast burn for acute incidents, slow burn for chronic degradation), tied to error-budget consumption rather than raw metric values.
- [ ] Thresholds are derived from data (baselined metrics, historical incident/non-incident comparison, anomaly detection) rather than guesswork, and are reviewed and adjusted as the system evolves.
- [ ] A small number of severity levels (2-3) are defined, each mapping to a notification channel and response expectation; alerts are routed to the owning team; and unacknowledged alerts escalate to a secondary or manager.
- [ ] Alerts include actionable context (service, symptom, current value, dashboard link, runbook, recent changes) so the responder can act immediately rather than investigating from scratch.
- [ ] Alert quality is maintained over time: volume and signal are reviewed regularly, false positives are treated as bugs to fix (threshold tuning or retirement), and obsolete alerts are removed as services change.
- [ ] Each alert has a runbook documenting what it means, why it fires, and what to do, turning the alert into a guided action rather than a mystery for the on-call to decipher.
