---
name: lead_scoring_and_routing.md
description: Use when the agent is designing lead scoring models, defining qualification criteria and thresholds, routing leads to sales or nurture, aligning marketing and sales on lead definition, or reviewing whether a scoring model predicts real opportunity rather than rewarding engagement noise.
---

# Lead Scoring And Routing

Lead scoring is how marketing decides which prospects deserve sales attention now. It fails when it rewards engagement noise, clicks and email opens, over buying intent, when it ignores fit so that unqualified leads score high on activity, when the threshold is set by gut rather than by conversion data, or when sales does not trust the model and works around it. A good scoring model predicts which leads will convert and routes them at the moment they are worth the effort.

Use this skill before designing or revising a lead scoring model, setting thresholds, defining routing rules, or aligning marketing and sales on lead quality. The goal is to prevent the agent from building a scoring model that looks precise but does not predict opportunity.

## Core Rules

### Score Both Fit And Intent, Not Engagement Alone

A lead's value comes from who they are and whether they are ready to buy. Score both dimensions.

Score:

- fit, such as role, company, industry, and size;
- intent, such as high-value page views, demo requests, and pricing interest;
- engagement depth, such as repeat visits and content consumption;
- recency and frequency of activity;
- explicit signals such as form responses and requests.

Engagement alone rewards the person who opens every email but never buys. Fit alone ignores whether they are in market. Combine both.

### Base The Model On Conversion Data, Not Assumption

Do not assign points by committee opinion. Calibrate to what actually predicts conversion.

Calibrate by:

- analyzing which attributes and behaviors converted leads share;
- weighting factors by their predictive power;
- testing the model against historical wins and losses;
- validating that high scores correlate with conversion;
- refining as more data accumulates.

A model built on assumption rewards the wrong signals. A model built on conversion data predicts real opportunity.

### Set The Threshold By Sales Capacity And Conversion Rate

The score that triggers a sales handoff is an economic decision, not an arbitrary number.

Set the threshold by:

- sales capacity and the volume of leads it can work;
- the conversion rate of leads at each score level;
- the cost of working a lead that does not convert;
- the risk of missing leads that would convert;
- the acceptable ratio of marketing-qualified to sales-qualified.

A threshold too low floods sales with noise. A threshold too high starves sales of opportunity. Set it where the economics work.

### Route By Rule, With Speed And Ownership

A qualified lead is only valuable if it reaches the right person fast. Routing rules make that happen.

Route by:

- territory, segment, or account assignment;
- round-robin or named owner rules;
- speed requirements for first contact;
- escalation for aged or unworked leads;
- clear ownership so no lead is orphaned.

A lead that sits unworked loses value by the hour. Routing rules with speed and ownership prevent the leak.

### Align Marketing And Sales On Lead Definition

A scoring model fails if sales does not trust or use it. Align on the definition.

Align on:

- what constitutes a marketing-qualified and sales-qualified lead;
- the attributes and behaviors that define each;
- the data captured and shared at handoff;
- the feedback loop from sales on lead quality;
- the review cadence to refine the model together.

When sales works around the model, the model is dead. Joint definition and feedback keep it trusted and used.

### Decay Scores To Reflect Recency

A lead's intent fades. Scores should decay when activity stops.

Decay by:

- reducing score for inactivity over time;
- weighting recent behavior more than old;
- resetting or downgrading leads that have gone cold;
- re-engaging cooled leads through nurture;
- distinguishing evergreen fit from time-bound intent.

A lead that was hot three months ago and silent since is not the same lead. Recency decay keeps the model honest.

### Measure And Refine Against Outcome

A scoring model is a hypothesis. Measure whether it predicts outcome and refine it.

Measure:

- conversion rate by score band;
- sales acceptance and follow-up rates;
- revenue attributed to scored leads;
- the gap between predicted and actual quality;
- feedback from sales on mis-routed leads.

A model that is not measured and refined drifts. Treat it as a living system calibrated to results.

## Common Traps

### Engagement Over Intent

Rewarding clicks and opens over buying signals inflates scores with non-buyers.

### Assumption-Based Weighting

Assigning points by opinion rather than conversion data rewards the wrong signals.

### Arbitrary Threshold

A threshold not tied to sales capacity and conversion rate either floods or starves sales.

### Slow Or Orphaned Routing

Leads that reach no one or sit unworked lose value rapidly.

### Sales Distrust And Workaround

A model sales does not trust gets bypassed, making the scoring effort wasted.

### No Score Decay

Treating old activity as current intent overstates the lead's readiness.

### Set-And-Forget Model

Failing to measure and refine against outcome lets the model silently mislead.

## Self-Check

- [ ] The model scores both fit and intent, not engagement activity alone.
- [ ] Factor weights are calibrated to conversion data, not assigned by assumption.
- [ ] The qualification threshold is set by sales capacity and conversion economics.
- [ ] Routing rules assign clear ownership with speed requirements and escalation.
- [ ] Marketing and sales are aligned on the definition of qualified leads and the feedback loop.
- [ ] Scores decay to reflect recency so stale activity does not overstate readiness.
- [ ] The model is measured against conversion rate by score band and refined over time.
- [ ] Sales feedback on lead quality is collected and used to improve the model.
- [ ] High scores correlate with actual conversion, validated against historical data.
- [ ] The model is reviewed regularly as products, segments, and buying behavior evolve.
