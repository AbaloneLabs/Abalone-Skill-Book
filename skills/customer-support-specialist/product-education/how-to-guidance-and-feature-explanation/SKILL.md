---
name: how-to-guidance-and-feature-explanation.md
description: Use when the agent is explaining how to use a product feature, writing step-by-step customer guidance, answering configuration questions, clarifying feature behavior, comparing available options, or helping a customer complete a task without overloading, misleading, or overpromising.
---

# How To Guidance And Feature Explanation

How-to support is not only listing steps. The customer is trying to complete a job, avoid a mistake, or understand whether a feature fits their situation. Agents often over-focus on the ideal click path and miss prerequisites, permissions, plan limits, data consequences, accessibility barriers, or safer alternatives. This skill helps the agent explain product use in a way that is accurate, task-oriented, and bounded.

## Core Rules

### Start with the customer's job

Identify what the customer is trying to accomplish, not just the feature name they mention. A question about exports may be about compliance, migration, reporting, backup, billing, or sharing with a partner. A question about settings may be about privacy, security, workflow, or team control.

Guidance should map the feature to the customer's goal. If the feature does not fit, say so and offer the closest supported path.

### Confirm prerequisites and eligibility

Before giving steps, check plan, role, permissions, device, region, account type, feature flag, integration status, product version, data state, and whether the customer is in an admin or end-user context. Many how-to failures come from assuming the customer sees the same UI as the agent.

If prerequisites are unknown, phrase steps conditionally rather than pretending certainty.

### Explain consequences before irreversible actions

Feature guidance may involve deleting data, changing permissions, inviting users, publishing content, charging customers, triggering notifications, disabling security, canceling services, or changing shared settings. Name the consequence before the step.

For shared accounts, explain who will be affected and whether the action can be reversed.

### Use clear and testable steps

Steps should be ordered, concrete, and written in the customer's product language. Avoid internal labels, old UI names, and vague verbs such as "configure it properly." Include expected result after important steps so the customer can tell whether they are on track.

If there are multiple paths for web, mobile, API, or admin console, separate them.

### Keep troubleshooting nearby

Anticipate common blockers: missing button, permission error, disabled integration, browser issue, stale cache, unavailable region, plan limit, validation error, or data not ready. Include safe checks before escalation.

Do not turn every how-to answer into a full troubleshooting tree, but include the first blockers the customer is likely to hit.

### Avoid unsupported best guesses

If product behavior changed recently or differs by customer segment, verify against current source of truth. Do not invent behavior from memory. If the answer depends on product roadmap, beta state, or internal limitation, use approved language.

Uncertain how-to answers create repeated contacts and customer mistakes.

### Teach the principle when useful

For recurring workflows, explain the rule behind the steps: permission inheritance, billing cycle timing, sync direction, approval hierarchy, data ownership, or notification logic. This helps the customer adapt without contacting support again.

Keep the principle practical, not academic.

### Provide a safe next path

If the customer cannot complete the task, give the escalation route, required evidence, and what not to do while waiting. For high-risk workflows, advise against risky retries, duplicate submissions, or manual edits until reviewed.

### Check whether the answer should be personalized and preserve customer confidence without hiding limits

Some how-to questions can be answered generally, but others require account-specific review. Billing changes, account recovery, permission edits, export or deletion steps, enterprise configuration, production integrations, safety settings, and regulated workflows may depend on private state or authority.

Do not give account-specific conclusions from generic documentation. Explain which part is general product behavior and which part requires secure review. This protects privacy and prevents a public or unauthenticated channel from becoming the place where sensitive account facts are confirmed.

If a feature has known constraints, say them early. Customers lose confidence when they follow a long path only to learn the feature cannot meet a key need. It is better to explain "this supports X but not Y" before the customer invests time.

Where a limitation exists, provide the closest supported alternative, escalation route, or feedback capture path. Do not bury the limitation at the end of the answer.

## Common Traps

- Answering the feature name instead of the customer's real goal.
- Giving steps without checking plan, permissions, device, region, version, or account type.
- Omitting consequences of deletion, publishing, billing, permission, or shared-account changes.
- Mixing web, mobile, admin, and API steps in one confusing flow.
- Using internal feature names or outdated UI labels; assuming the button is missing because the customer is confused rather than ineligible
- Providing roadmap guesses or beta behavior as if generally available; overloading a simple task with every possible edge case
- Failing to include what the customer should expect after each major step; giving account-specific conclusions in a public, unauthenticated, or low-verification channel
- Hiding a known limitation until after the customer has followed a long setup path; leaving no escalation path when the feature does not behave as described

## Self-Check

- Does the guidance address the customer's underlying job, not only the feature label?
- Are plan, role, permissions, device, region, version, integration, feature flag, and account context checked or stated as assumptions?
- Are irreversible or broad consequences explained before the relevant step?
- Are shared-account, admin, notification, billing, privacy, and data effects identified where relevant?
- Are steps concrete, ordered, current, and written in customer-visible product language?
- Are web, mobile, API, and admin paths separated if they differ?
- Are likely blockers and safe first checks included without overwhelming the answer?
- Is uncertain, beta, or roadmap behavior avoided unless verified and approved?
- Does the explanation include the underlying product rule when it would help reuse?; is there a clear escalation or next path if the customer cannot complete the task?
- Are general product instructions separated from account-specific conclusions that require verification?; are known limitations, unsupported needs, and closest alternatives stated early enough to prevent wasted effort?
