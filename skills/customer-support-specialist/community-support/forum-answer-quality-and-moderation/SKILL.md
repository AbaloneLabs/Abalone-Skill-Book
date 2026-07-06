---
name: forum-answer-quality-and-moderation.md
description: Use when the agent is answering in a customer forum, moderating community threads, marking accepted answers, correcting misinformation, handling duplicate posts, enforcing community rules, or balancing peer conversation with official support guidance.
---

# Forum Answer Quality And Moderation

Community forums are public knowledge bases, support channels, and customer spaces at the same time. A weak answer can mislead many future readers, while heavy-handed moderation can suppress useful peer help or make customers feel censored. Agents must answer clearly, protect privacy, correct risky advice, and moderate according to policy. This skill helps the agent maintain forum quality without turning every thread into a private ticket.

## Core Rules

### Identify whether the thread needs official action

Some threads can be answered with public guidance. Others need account review, privacy protection, safety escalation, abuse handling, billing investigation, legal review, or incident routing. Decide before replying.

Do not ask customers to post private data in public. If account-specific action is needed, move to a secure channel and leave a public note that explains the safe path.

### Answer for future readers

Forum answers should be clear, searchable, and durable. Include the product area, conditions where the answer applies, known limits, and the current source of truth. Avoid overly personalized language that only helps the original poster.

If behavior changes by plan, region, version, or role, state that.

### Correct misinformation carefully

Peer answers may be partly useful and partly wrong. Correct the risky part without humiliating the contributor. Explain why a workaround is unsafe if it risks data loss, billing errors, security exposure, policy violation, or unsupported configuration.

If misinformation is harmful, mark or moderate it according to policy.

### Moderate with transparent policy basis

Moderation actions such as editing, hiding, locking, merging, warning, or banning should map to community rules: spam, abuse, harassment, private data, illegal content, impersonation, duplicate threads, off-topic content, or unsafe instructions.

Do not moderate criticism simply because it is uncomfortable.

### Manage duplicates and accepted answers

Merge or link duplicate threads when it helps search quality, but preserve unique impact details that may indicate a new bug or incident. Mark accepted answers only when they are current, safe, and sufficiently scoped.

An accepted answer becomes a public recommendation.

### Escalate emerging incidents and product signals

Multiple forum posts about similar failures, confusion, or workarounds may signal an outage, regression, documentation gap, accessibility issue, or launch problem. Track volume and route patterns to support operations or product.

Community should not be treated as separate from official support telemetry.

### Protect customer dignity

Public corrections should be respectful. Avoid sarcasm, blame, or implying the customer did not search hard enough. For frustrated customers, acknowledge the issue and route them without debating in public.

### Keep forum guidance maintained

Old accepted answers and pinned posts can become wrong after releases, policy changes, pricing updates, or deprecations. Review high-traffic posts and add date or version context where needed.

### Mark known issues and incidents carefully and protect moderators and community members

When a forum thread reflects an active bug, outage, or launch issue, link to the approved known-issue or status source if one exists. If no public source exists, use approved language and avoid overconfirming root cause.

Do not let ten separate forum threads become ten different incident narratives. Centralize updates where possible and make clear what information is official.

Moderation can expose agents and community members to harassment, doxxing, threats, graphic content, or coordinated abuse. Use escalation, restricted tooling, and internal reporting when a thread becomes unsafe.

Do not ask volunteer community members to handle abusive or sensitive content that official teams should own.

## Common Traps

- Answering an account-specific issue publicly.
- Asking for email, order number, payment details, screenshots with private data, or logs in a public thread.
- Writing an answer that helps the original poster but misleads future readers.
- Letting unsafe peer workarounds remain uncorrected.
- Deleting criticism without a moderation policy basis; marking an answer accepted when it only applies to one plan, region, or version
- Merging duplicates and losing evidence of a new incident; treating forum patterns as less important than tickets
- Using sarcastic or scolding language with customers who missed existing answers; leaving old pinned or accepted answers unreviewed after product changes
- Letting active incidents fragment into multiple inconsistent forum explanations; expecting peer moderators to handle harassment, doxxing, threats, or sensitive abuse content alone

## Self-Check

- Is the thread safe for public guidance, or does it need secure account review or escalation?
- Does the reply avoid requesting or exposing private data?
- Is the answer searchable, scoped, and useful for future readers?
- Are plan, region, version, role, and product-state limits stated where relevant?
- Is misinformation corrected respectfully and clearly?
- Are unsafe workarounds moderated or labeled according to policy?
- Are moderation actions tied to spam, abuse, harassment, privacy, illegal content, impersonation, duplicate, off-topic, or unsafe-instruction rules?
- Are duplicates merged or linked without losing incident or bug signals?
- Are accepted answers current, safe, and sufficiently scoped?; are emerging community patterns routed into support or product telemetry?
- Are known issues and incidents linked to approved sources or centralized updates?; are moderator and community safety risks escalated to official teams when needed?
