---
name: search-intent-and-help-content-gap-analysis.md
description: Use when the agent is analyzing help-center search terms, failed searches, no-result queries, article exits, repeated contacts after search, customer language, content gaps, synonym gaps, intent mismatch, or whether self-service content answers the customer's real support need.
---

# Search Intent And Help Content Gap Analysis

Search logs show what customers call their problems before support translates them into internal categories. Poor search analysis leads to articles that exist but cannot be found, content that answers the wrong intent, or self-service paths that hide urgent cases. This skill helps the agent use search behavior to improve help content and routing.

## Core Rules

### Translate customer language to support intent

Customers search in symptoms, emotions, product names, error text, billing descriptors, and guesses. Map terms like "charged twice," "can't get in," "cancel renewal," or "where is package" to the underlying intent, but preserve the original words for search tuning.

Do not force customer language into internal taxonomy too early.

### Separate no-content from no-findability

A failed search may mean no article exists, the article has different wording, search ranking is poor, localization is missing, the content is outdated, or the customer's issue needs human support. Diagnose the gap type before writing new content.

Creating duplicate articles can make findability worse.

### Look at post-search outcomes

Measure whether customers click, exit, contact support, reopen tickets, escalate, or repeat searches. A clicked article is not successful if the customer still contacts support with the same issue.

Search quality is proven by outcome, not clicks alone.

### Identify high-risk search terms

Terms involving self-harm, safety, fraud, account takeover, data deletion, privacy, legal, chargeback, harassment, discrimination, medical, or urgent access should trigger safe routing or prominent escalation paths. These may not be good candidates for pure self-service.

Search can be an early warning system.

### Include synonyms, misspellings, and region language

Customers may use brand nicknames, old feature names, local terms, translated phrases, competitor terms, payment descriptors, or misspellings. Search tuning should connect these to the right article without hiding regional policy differences.

Do not optimize only for internal product labels.

### Use content gaps to route ownership

Some gaps belong to knowledge writers; others belong to product UX, policy, billing, localization, accessibility, bot routing, or support operations. Identify the owner rather than treating every gap as a copy problem.

Search data is product and operations evidence.

### Avoid over-deflection of ambiguous terms

Some queries can mean multiple intents, such as "refund," "locked," "delete," or "not working." Search results should help customers disambiguate and reach human support when stakes are high.

Do not push a single article when intent is ambiguous and risky.

### Document gap evidence

Capture query, volume, affected language or region, clicked articles, exit behavior, downstream tickets, customer phrases, risk level, proposed owner, and recommended change. This makes search feedback actionable.

Search improvement should be traceable to evidence.

### Distinguish search ranking from journey failure and preserve low-volume critical terms

Sometimes the right article ranks well, but the article sends customers into a broken form, outdated account page, inaccessible tool, or policy dead end. In that case the search system is not the only owner. Follow the customer journey after the search result before deciding the fix.

Do not keep tuning keywords when the destination experience is the failure.

Rare searches may be critical if they mention suicide, violence, subpoena, data breach, fraud, child safety, recall, medical urgency, or urgent account lockout. These terms may need prominent safe routing even if volume is too low for normal analytics.

Search governance should include risk lists, not only popularity.

## Common Traps

- Translating customer queries into internal labels and losing real wording.
- Writing a new article when an existing one is simply hard to find.
- Judging search success by clicks rather than resolution.
- Missing high-risk terms that should trigger human or specialized support.
- Ignoring misspellings, old feature names, local terms, and payment descriptors; treating localization problems as generic content gaps
- Sending every ambiguous query to one article; failing to route product, policy, accessibility, billing, or bot gaps to the right owner
- Optimizing for common low-risk searches while ignoring rare high-harm queries; submitting gap feedback without evidence
- Tuning search keywords while the linked flow, form, or tool remains broken; dropping rare safety, legal, fraud, privacy, or urgent-access terms because they do not meet volume thresholds

## Self-Check

- Are customer search terms preserved and mapped to likely support intent?
- Is the gap classified as missing content, poor findability, ranking, localization, outdated content, human-needed issue, or product/policy gap?
- Were post-search outcomes checked: clicks, exits, tickets, reopens, escalations, repeat searches, and same-issue contacts?
- Are high-risk terms routed for safety, fraud, takeover, privacy, legal, harassment, discrimination, medical, chargeback, and urgent access needs?
- Are synonyms, misspellings, old names, local terms, translations, competitor terms, and payment descriptors considered?
- Is the correct owner identified across knowledge, product, policy, billing, localization, accessibility, bot, and support operations?
- Do ambiguous queries offer disambiguation and safe human paths?
- Are query, volume, language, region, clicked articles, exits, tickets, phrases, risk, owner, and recommended change documented?
- Does the change improve resolution rather than only search clicks?; are rare high-harm searches weighted appropriately?
- Was the post-click journey checked for broken forms, inaccessible tools, outdated pages, or policy dead ends?; are low-volume critical terms maintained through risk lists or safe-routing rules?
