---
name: voice-of-customer-pattern-synthesis.md
description: Use when the agent is synthesizing customer feedback patterns from tickets, chats, calls, reviews, surveys, community posts, social comments, escalations, complaints, churn notes, support tags, or qualitative evidence for product, operations, policy, or leadership decisions.
---

# Voice Of Customer Pattern Synthesis

Voice-of-customer synthesis is not counting complaints and writing themes. Support data is biased by who contacts support, which channels are visible, which tags agents choose, and which customers have enough time or power to complain. Weak synthesis overstates anecdotes, hides severe minority harms, or strips away context that product teams need. This skill helps the agent turn support feedback into credible patterns and decision-ready insight.

## Core Rules

### Define the decision the synthesis serves

Clarify whether the output is for roadmap prioritization, bug triage, policy change, launch review, executive update, customer success action, QA improvement, or operational staffing. Different decisions need different evidence.

Do not produce a generic theme list if the audience needs severity, affected segment, root cause, revenue impact, or suggested intervention.

### Preserve customer context

Themes should include who is affected, what they were trying to do, where they got stuck, what workaround they used, what emotion or cost appeared, and what outcome they wanted. A theme like "confusing billing" is too vague to act on.

Include representative paraphrases or short examples without exposing private data.

### Balance frequency and severity

High-volume complaints matter, but low-volume high-severity issues may matter more: accessibility barriers, data loss, safety risk, fraud, legal exposure, VIP impact, regulated customers, or churn from strategic accounts.

Report both volume and severity so decision makers do not optimize only for the loudest category.

### Account for data bias

Support data reflects channel access, language, region, customer segment, plan, agent tagging habits, survey response bias, and whether customers are willing to complain. Compare tickets with reviews, social posts, community threads, churn notes, product analytics, and account feedback where possible.

State limitations instead of pretending the synthesis is a complete customer census.

### Separate symptoms from causes

Customers may report "bug," "bad support," "too expensive," or "confusing UI," but the cause might be policy language, plan mismatch, missing onboarding, localization, performance, documentation, or billing design. Synthesis should avoid treating customer labels as final diagnosis.

Use evidence to propose likely drivers and what would confirm them.

### Show trend and change

A useful pattern includes whether the issue is new, growing, seasonal, launch-related, region-specific, tied to an incident, or stable background noise. Compare against prior period if possible.

A small spike after a release may deserve more attention than a larger but steady category.

### Convert insight into action paths

For each major pattern, identify likely owner, suggested next investigation, customer-facing mitigation, knowledge update, product fix, policy review, or metrics to monitor.

Synthesis that ends at "customers are unhappy" is not operationally useful.

### Protect privacy and trust

Remove personal data, sensitive account details, and unnecessary verbatim quotes. Use enough detail to preserve meaning while respecting confidentiality.

If feedback involves vulnerable customers, abuse, security, legal claims, or data incidents, apply the right escalation and redaction.

### Keep minority segments visible and distinguish evidence from recommendation

Some customer groups generate fewer tickets because they have less access to support, use another language, churn silently, rely on partners, avoid complaining publicly, or cannot complete the workflow at all. Low volume does not always mean low importance.

When synthesizing patterns, call out segment silence or underrepresentation where likely. Include accessibility, localization, small-market, reseller, marketplace, and low-plan signals if they show disproportionate harm.

A synthesis can include recommendations, but it should make clear which statements are observed evidence, which are interpretations, and which are proposed actions. This helps product, operations, or leadership challenge assumptions without dismissing the customer signal.

Use confidence labels when the pattern is emerging, directional, or well-supported.

## Common Traps

- Reporting themes without knowing what decision the report should support.
- Counting ticket tags as objective reality.
- Letting high-volume low-severity issues crowd out severe minority harms.
- Removing all context until the insight is too generic to act on.
- Treating customer-proposed causes as proven root cause; ignoring non-ticket sources such as reviews, community, social, churn notes, and account teams
- Failing to separate new spikes from long-standing baseline issues; using private customer details or raw quotes unnecessarily
- Producing a summary with no owner, next step, or confirmation path; overgeneralizing from one region, language, plan, or customer segment
- Treating quiet or underrepresented segments as unaffected; blending observation, interpretation, and recommendation until the evidence basis is unclear

## Self-Check

- Is the decision purpose clear: roadmap, bug, policy, launch, executive, success, QA, or operations?
- Does each theme preserve customer goal, blocker, context, workaround, cost, emotion, and desired outcome?
- Are frequency and severity both represented?
- Are high-severity low-volume issues such as accessibility, safety, data loss, fraud, legal, VIP, regulated, and churn risk included?
- Are data-source biases and coverage limits stated?
- Are tickets compared with reviews, social, community, surveys, churn, analytics, or account feedback where available?
- Are symptoms separated from likely causes and confirmation needs?
- Does the synthesis show trend, spike, seasonality, launch relation, region, and segment where relevant?
- Does each major pattern have an owner or next action path?; is private or sensitive customer information redacted appropriately?
- Are quiet or underrepresented segments considered where support data may undercount harm?; are observations, interpretations, recommendations, and confidence levels separated?
