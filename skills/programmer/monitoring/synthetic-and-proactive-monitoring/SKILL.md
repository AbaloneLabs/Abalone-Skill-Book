---
name: synthetic_and_proactive_monitoring.md
description: Use when the agent is setting up synthetic checks, probes, or canaries that simulate user traffic to detect outages before users do, monitoring business-critical transactions end-to-end, choosing external vs internal probes, defining availability test thresholds, or reducing false positives in synthetic monitoring. Also covers the failure mode of synthetic checks that pass while real users fail (because the probe does not represent real traffic), probes that test only the happy path, false-positive storms that train responders to ignore synthetic alerts, and the gap between a check that returns 200 and a transaction that actually works.
---

# Synthetic And Proactive Monitoring

Synthetic monitoring is the practice of running scripted probes — simulated user journeys, API calls, or transactions — on a schedule, from outside the system, to detect outages and degradations before real users report them. The judgment problem is that a synthetic check is only as good as its fidelity to real user traffic, and the easy checks (a ping, a health endpoint returning 200) test the wrong thing: they confirm that something is listening, not that a user can accomplish their goal. A checkout endpoint that returns 200 but fails to process payment passes a naive probe and fails the user. The discipline is to model synthetic checks on real user journeys (the critical transactions that matter to the business), to run them from where users are (external probes, multiple regions), to assert on outcomes (the transaction succeeded) rather than liveness (the server responded), and to tune thresholds to suppress false positives — because a synthetic alert that fires on noise trains responders to ignore synthetic alerts, which defeats the purpose of having them.

Agents tend to set up a few shallow probes (a health check, a homepage ping) and declare monitoring done. The harm appears as outages detected first by users (because the probe tested liveness, not the transaction), as checks that pass while real users fail (because the probe does not authenticate, does not traverse the real path, or uses a test account exempt from the failure), and as false-positive storms from brittle probes that page on transient blips until responders mute them. The judgment is to cover the critical business transactions end-to-end, to assert on user-visible outcomes, to probe from the user's vantage (external, multi-region), to distinguish availability from correctness, and to treat false-positive reduction as ongoing maintenance so the signal stays credible. A synthetic check that no one trusts is worse than none, because it provides false assurance.

## Core Rules

### Model Checks On Real User Journeys, Not Health Endpoints

A synthetic check should simulate what a user actually does: the critical transactions that matter to the business (login, search, add-to-cart, checkout, key API calls). A health endpoint that returns 200 confirms the server process is alive; it does not confirm a user can accomplish their goal.

- **Cover critical business transactions end-to-end.** For each journey that matters (the revenue path, the signup path, the core API), run a synthetic that traverses the full path, not just the entry point.
- **Assert on outcomes, not liveness.** The check should verify that the transaction succeeded (the order was created, the search returned results, the API returned the expected data), not merely that the endpoint responded.
- **Use realistic inputs and state.** A probe that uses a static test input may miss failures triggered by real data variation; vary inputs and exercise the paths real users hit.
- **Include the dependencies a real journey touches.** A checkout that depends on a payment service and an inventory service must exercise all three; a probe that mocks the dependencies tests nothing about the integrated system.

### Probe From Where Users Are

The vantage point of the probe determines what it can detect. A probe inside the data center confirms the service works from inside the network; it cannot detect a DNS problem, a CDN outage, a regional network issue, or a certificate failure that affects users outside.

- **Run external probes from multiple regions.** Users are not in your data center; external probes from the regions users inhabit detect problems internal probes cannot (DNS, CDN, regional network, cert).
- **Combine external and internal probes.** External probes detect user-facing problems; internal probes localize where the problem is; both have a role.
- **Probe from real browser runtimes for web journeys.** A web journey depends on JavaScript execution, asset loading, and browser behavior; an HTTP probe that fetches HTML misses client-side failures. Use a real-browser probe for web critical paths.

### Distinguish Availability From Correctness

A service can be available (responding) and incorrect (returning wrong data, partial results, stale content). Probes that check only availability miss correctness failures. Assert on the content and correctness of responses, not only their presence.

- **Validate response content, not only status codes.** A 200 with an empty or wrong body is a failure; assert that the response contains the expected data.
- **Check freshness where it matters.** A service returning stale data (a cache not invalidating, a pipeline not running) is available but wrong; check timestamps or content versions.
- **Check downstream effects for critical transactions.** For a checkout, verify the order was actually created in the system, not only that the API returned success; this catches silent failures where the API lies.

### Tune Thresholds To Suppress False Positives

A synthetic alert that fires on transient blips trains responders to ignore synthetic alerts, which destroys their value. Thresholds and retry logic must suppress noise while preserving the signal of real outages.

- **Confirm before alerting.** A single failed probe may be transient; require confirmation (a retry, or N failures over M attempts) before paging, to suppress flukes.
- **Set thresholds on user impact, not on tight tolerances.** A probe that alerts when latency exceeds 200ms will page constantly; one that alerts when latency exceeds the point users notice (or the SLO breach) pages when it matters.
- **Account for probe-environment noise.** External probes are subject to network variability; a threshold that does not account for that variability will false-positive on slow networks rather than slow services.
- **Treat false-positive reduction as ongoing maintenance.** Probe brittleness accumulates (selectors change, test data drifts, thresholds go stale); review and tune probes regularly so the signal stays credible.

### Treat Synthetic Monitoring As Detection, Not Diagnosis

Synthetic monitoring tells you that something is broken from the user's perspective; it rarely tells you why. Pair it with the observability (logs, metrics, traces) needed to diagnose the cause once the synthetic detects the symptom.

- **Synthetic for detection, observability for diagnosis.** The synthetic alerts that users are failing; the metrics, logs, and traces localize the cause. Both are needed; neither alone is sufficient.
- **Link the synthetic alert to the relevant dashboards and traces.** When the synthetic fires, the responder should land one step from the diagnostic context, not have to hunt for it.

## Common Traps

### Probes That Test Liveness, Not The Transaction

A health-endpoint or homepage ping that confirms the server is alive while the actual user journey (checkout, search) is broken. Model checks on real user journeys; assert on outcomes, not liveness.

### Checks That Pass While Real Users Fail

A probe that uses a test account exempt from the failure, mocks dependencies, or skips authentication, so it passes a path real users fail. Use realistic inputs, state, and dependencies; exercise the integrated path.

### Single-Region Or Internal-Only Probes

Probes run only from inside the data center, missing DNS, CDN, regional network, and certificate problems that affect external users. Run external probes from multiple regions where users are.

### HTTP Probes For Web Journeys

An HTTP probe that fetches HTML for a journey that depends on JavaScript execution, missing client-side failures. Use real-browser probes for web critical paths.

### Status-Code-Only Assertions

A probe that asserts only on the status code, missing a 200 with empty, wrong, or stale content. Validate response content, freshness, and downstream effects.

### False-Positive Storms From Brittle Probes

Probes that alert on every transient blip or tight threshold, training responders to mute or ignore synthetic alerts. Confirm before alerting, set thresholds on user impact, and maintain probes to reduce noise.

### Synthetic Without Diagnostic Pairing

A synthetic that detects the symptom but no metrics/traces to diagnose the cause, so responders know users are failing but not why. Pair synthetic detection with diagnostic observability.

## Self-Check

- [ ] Synthetic checks model real user journeys (login, search, checkout, core API) end-to-end, asserting on outcomes (the transaction succeeded, expected data returned) rather than liveness, with realistic inputs/state and the dependencies a real journey touches.
- [ ] Probes run from where users are: external probes from multiple regions (to detect DNS, CDN, regional network, cert problems), combined with internal probes for localization, and real-browser runtimes for web journeys that depend on client-side execution.
- [ ] Assertions distinguish availability from correctness: response content is validated (not only status codes), freshness is checked where it matters, and downstream effects are verified for critical transactions (the order was created, not only that the API returned 200).
- [ ] Thresholds suppress false positives: confirmation (retry or N-of-M) before alerting, thresholds set on user impact rather than tight tolerances, probe-environment noise accounted for, and false-positive reduction treated as ongoing maintenance.
- [ ] Synthetic monitoring is paired with diagnostic observability (metrics, logs, traces) so detection (users are failing) leads directly to diagnosis (why), with alerts linked to the relevant dashboards.
- [ ] Critical business transactions are prioritized (the revenue path, signup, core API) over shallow coverage of every endpoint, so the most important journeys are protected first.
- [ ] The highest-risk cases were verified — a probe that detected a failure real users would hit (not just liveness), an external probe that caught a problem invisible internally, a content assertion that caught a 200-with-wrong-data, and a tuned threshold that suppressed a false-positive storm — not only the clean happy-path probe.
