---
name: macro-design-and-template-safety.md
description: Use when the agent is creating, editing, reviewing, or using support macros, templates, canned responses, snippets, bot replies, escalation templates, policy language, or reusable customer messages and must ensure they are accurate, adaptable, safe, humane, and not creating false promises or privacy risk.
---

# Macro Design And Template Safety

Macros speed support, but they also scale mistakes. A bad template can send irrelevant questions, promise unauthorized remedies, mishandle emotional cases, leak private assumptions, or make customers feel unseen. Agents often treat macros as completed answers. This skill helps the agent design and use templates as controlled starting points that still require case-specific judgment.

## Core Rules

### Define the macro's allowed use

A strong macro states the customer intent, issue type, required preconditions, exclusions, approval needs, and risk cases where it should not be used. Without boundaries, agents apply templates to cases that only look similar.

Do not create a macro that can be used safely only by someone who already knows all the hidden rules.

### Preserve case-specific adaptation

Macros should include prompts for customer name, specific issue, evidence checked, account state, policy reason, next action, and timing. They should not sound as if the agent ignored the customer's message.

Reusable language should support personalization, not erase it.

### Avoid unauthorized commitments

Check for promises about refunds, credits, replacements, timelines, bug fixes, account restoration, legal outcomes, escalation, or policy exceptions. Use conditional or process language where approval is required.

Templates create obligations at scale, so their wording must be disciplined.

### Protect privacy and security

Macros must not reveal account existence, security configuration, fraud signals, internal queues, private notes, third-party data, or sensitive policy details. They should never ask for passwords, full payment numbers, tokens, private keys, or unnecessary identity documents.

If sensitive evidence is needed, route to approved secure channels.

### Handle tone and emotion variants

A macro for routine information should not be used unchanged for harmed, angry, anxious, safety-critical, or repeat-contact customers. Provide variants or guidance for high-impact cases, apologies, denials, and escalation updates.

Efficiency should not make support sound indifferent.

### Keep policy and product references current

Macros touching pricing, plans, UI, refund rules, privacy, security, legal language, warranty, subscriptions, and known issues need owner and review cadence. Link them to source articles or policy references.

Stale macros are dangerous because agents trust them.

### Design for agent decision points

Good templates include internal notes or placeholders that force checks: verify authority, confirm eligibility, choose correct timeline, remove irrelevant section, add evidence, or route if risk appears. Avoid macros that let agents send without thinking.

The macro should guide judgment, not bypass it.

### Monitor performance and failure and design safe failure modes

Track macro edits, reopen rate, CSAT comments, escalations, complaints, misuse, and cases where agents frequently delete sections. These signals show where the macro is too broad, too vague, or unsafe.

Macro maintenance is part of quality control.

Macros should make it hard to send the wrong thing. Use visible placeholders, internal-only instructions, required decision points, and warnings for sensitive cases. If a macro has sections that must be deleted, make the deletion obvious before sending.

Do not rely on agents noticing subtle template conditions during high-volume work.

### Retire macros deliberately and require review before sensitive publication

Old templates continue to circulate through saved snippets, bot flows, copied text, and agent habit. When replacing a macro, remove or disable stale versions, communicate the change, and check that related automation uses the new version.

A retired policy is still active if agents keep sending the old wording.

Macros touching legal, privacy, security, billing, refunds, safety, regulated claims, accessibility, or public incidents should have a named reviewer and approval state before rollout. Draft convenience should not become production language accidentally.

## Common Traps

- Building macros without clear inclusion and exclusion criteria.
- Letting template friendliness replace case-specific acknowledgement.
- Promising approval-dependent outcomes in reusable language.
- Asking for unsafe sensitive data through a canned reply; revealing internal routing, security, or fraud logic
- Using routine macros for emotional, high-impact, or repeat-contact cases; leaving old pricing, UI labels, policy, or legal language in circulation
- Creating placeholders that agents can accidentally leave in customer replies; treating low handle time as proof the macro is good
- Ignoring evidence that agents constantly edit or misuse the template; leaving optional sections or placeholders easy to send accidentally
- Updating a macro but leaving old snippets, bot replies, or copied versions in circulation; rolling out sensitive macros before legal, policy, security, billing, or operations review

## Self-Check

- Does the macro define intent, preconditions, exclusions, approvals, and risk cases?
- Does it require case-specific issue, evidence, account state, policy reason, action, and timing?
- Are promises about money, replacement, access, timeline, bug fix, escalation, legal outcome, or exception authorized or conditional?
- Does it avoid account enumeration, security configuration, fraud signals, internal notes, and third-party data?
- Are passwords, full payment data, tokens, keys, and unnecessary documents avoided?
- Are sensitive evidence paths secure and approved?
- Are tone variants or guidance available for high-impact, emotional, safety, denial, and repeat-contact cases?
- Are product and policy references current with owner and review cadence?; do placeholders and internal notes reduce misuse without leaking to customers?
- Are edits, reopen rate, CSAT, escalations, complaints, misuse, and deletion patterns monitored?; does the macro have safe failure modes through clear placeholders, decision points, and sensitive-case warnings?
- Were stale snippets, bot flows, copied templates, and old versions removed or retired when the macro changed?; does any sensitive macro have named review and approval before use?
