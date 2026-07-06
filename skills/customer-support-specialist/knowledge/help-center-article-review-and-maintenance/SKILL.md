---
name: help-center-article-review-and-maintenance.md
description: Use when the agent is reviewing, editing, auditing, or maintaining a help center article for accuracy, user intent, steps, screenshots, policy fit, localization, accessibility, searchability, product changes, stale content, or whether customers can safely complete the task from the article.
---

# Help Center Article Review And Maintenance

Help-center articles are customer-facing operational tools. A stale or unclear article can create failed setup, billing disputes, unsafe troubleshooting, privacy exposure, or repeated contacts. Agents often edit articles for style while missing whether the content still matches the product, policy, and customer intent. This skill helps the agent review help content for usable, safe, current guidance.

## Core Rules

### Review against real customer intent

Identify what customers are trying to do, what search terms they use, what confusion appears in tickets, and where they get stuck. The article should answer the task customers have, not only describe the feature from an internal perspective.

Do not optimize an article for internal terminology that customers do not know.

### Verify current product and policy

Check UI labels, screenshots, navigation, permissions, plan availability, region differences, pricing, refund policy, privacy language, security steps, warranty terms, and known limitations. Product and policy drift are the main causes of bad help content.

Do not rely on old screenshots or memory.

### Make steps complete and safe

Steps should include prerequisites, permissions, expected outcome, irreversible effects, warnings before risky actions, fallback path, and what to do if the step fails. For technical steps, include environment differences where they matter.

Do not give destructive or security-sensitive instructions without clear safeguards.

### Separate explanation from action

Customers often need a concise action path first and conceptual explanation second. Place prerequisites and warnings before steps; place background where it helps decisions. Avoid long articles that hide the actual task.

A correct article can still fail if the customer cannot find the next action.

### Maintain accessibility and localization readiness

Use clear headings, descriptive links, alt text for meaningful images, plain language, and terms that can be translated consistently. Avoid screenshots as the only source of critical information. Watch for region-specific policy, currency, date, legal, and support-channel differences.

Help content must work for more than one ideal reader.

### Protect sensitive information

Do not include internal tool screenshots, private account data, security bypass details, fraud thresholds, unreleased features, or employee-only procedures in customer-facing articles. Use sanitized examples and approved language.

Public help content can become an attack surface.

### Connect article maintenance to support signals

Use search failures, high exit rate, repeat contacts, macro edits, escalations, CSAT comments, and agent feedback to prioritize updates. After changes, monitor whether behavior improves.

Maintenance is an ongoing feedback loop, not a one-time edit.

### Version and sunset deliberately

When products, plans, UI, or policies change, decide whether to update, redirect, archive, or leave version-specific content. Mark legacy guidance clearly. Remove stale content from search where it misleads.

Old articles can be more harmful than missing articles.

### Validate the complete customer path and check analytics without worshiping them

An article may be accurate but still fail if links are broken, buttons are hidden behind permissions, forms reject valid inputs, embedded videos are outdated, or the final confirmation does not match the article. Walk through the path as the target customer where possible.

Do not treat article review as complete until the linked self-service flow, contact path, or escalation path still works.

Search exits, page exits, low helpfulness votes, repeated contacts, and high bot fallback can identify weak content. But analytics can also hide underserved customers, accessibility barriers, localization issues, or low-volume high-risk failures. Combine metrics with ticket reading and agent feedback.

Numbers are signals, not the whole review.

## Common Traps

- Reviewing grammar while ignoring whether the steps still work.
- Writing around internal terminology instead of customer search intent.
- Leaving old screenshots after UI or permission changes.
- Omitting prerequisites, warnings, expected result, or failure path.
- Giving destructive instructions before warnings; relying on screenshots for critical steps without accessible text
- Publishing one region's policy as universal; including internal tools, risk controls, unreleased features, or private data
- Updating the article but not related macros, bots, or translations; leaving stale legacy articles searchable without context
- Leaving broken links, dead forms, hidden permission requirements, or outdated videos untouched; trusting page views and helpfulness votes while ignoring ticket evidence and accessibility failures

## Self-Check

- Does the article match real customer intent, search language, and ticket confusion?
- Were UI labels, screenshots, navigation, permissions, plan, region, pricing, refund, privacy, security, warranty, and limitations verified?
- Are prerequisites, permissions, expected outcome, irreversible effects, warnings, fallback, and failure path clear?
- Are warnings placed before risky steps?
- Is the action path easy to find before background explanation?
- Are headings, links, image descriptions, plain language, and localization needs handled?
- Are region, currency, date, legal, and channel differences accounted for?
- Are internal tools, private data, security bypasses, fraud thresholds, unreleased features, and employee procedures excluded?
- Are support signals used to prioritize and verify article maintenance?; are versioning, redirects, archiving, search visibility, macros, bots, and translations aligned?
- Was the full linked customer path tested or reviewed for permissions, forms, confirmations, links, and media?; were analytics combined with ticket evidence, accessibility checks, localization feedback, and agent observations?
