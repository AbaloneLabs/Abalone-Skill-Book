---
name: chatbot-handoff-and-containment-risk.md
description: Use when the agent is reviewing chatbot containment, bot-to-human handoff, automation failure, repeated bot loops, customer frustration with bots, chatbot routing, bot transcript quality, or whether automation should stop and a human support owner should take over.
---

# Chatbot Handoff And Containment Risk

Chatbots can answer simple questions and gather context, but they can also trap customers, misclassify urgency, ask unsafe questions, or hand off without usable information. Containment is not quality if customers are blocked from resolution. This skill helps the agent decide when automation should continue, when it should hand off, and what context must transfer.

## Core Rules

### Define what the bot may safely handle

Bots are best for low-risk, structured, repeatable tasks: article suggestions, order lookup, simple status, basic troubleshooting, and guided forms. They should be conservative around billing disputes, refunds, account access, safety, legal, privacy, fraud, harassment, regulated complaints, and high-emotion cases.

Do not let bot coverage expand into sensitive judgment work without controls.

### Detect handoff triggers early

Handoff triggers include repeated failure, customer asks for human, high impact, unclear intent, sensitive data, safety language, legal or privacy terms, account takeover, fraud, urgent deadline, accessibility barrier, abusive behavior, or bot confidence below threshold.

The bot should not require customers to fail many times before reaching a human.

### Preserve context during handoff

Human agents need customer intent, bot path, answers given, account identifiers where safe, articles shown, actions attempted, errors, sentiment, escalation triggers, and unresolved questions. The customer should not have to repeat everything.

A handoff without context is another support failure.

### Avoid unsafe bot questions

Bots should not ask for passwords, full card numbers, private keys, tokens, recovery codes, unnecessary documents, or sensitive personal data through unsafe channels. They should also avoid asking customers to perform risky technical or safety steps.

Automation can scale bad evidence requests.

### Measure resolution, not containment

Track whether customers solved the issue, reopened, repeated contact, escalated, abandoned, expressed frustration, or had high-risk issues contained incorrectly. A high containment rate can hide harm.

Bot quality should be judged by customer outcome and risk accuracy.

### Make human path visible and respectful

If the customer needs a human, do not shame them or force irrelevant self-service. Make the path clear, explain what will happen next, and transfer collected context.

The bot should lower effort, not defend itself.

### Review bot language for policy and tone

Bot responses must avoid false promises, outdated policy, unauthorized refunds, legal advice, security disclosure, and tone mismatch. High-impact cases need more careful language than routine help.

Bots can sound confident while being wrong.

### Feed bot failures back to owners and preserve customer agency during automation

When handoff fails or containment is harmful, capture failure mode, customer phrase, bot route, missing content, wrong answer, escalation trigger, and recommended fix. Route to bot, knowledge, policy, product, or operations owner.

Bot improvement needs evidence from failed conversations.

Customers should know when they are interacting with automation where policy requires it or where confusion would affect trust. They should be able to correct bot misunderstanding, change intent, or request a human through an available path. The bot should not pretend certainty when it is only guessing.

Automation should assist the customer, not make the customer negotiate with a system.

### Review language and localization failures and audit handoff samples

Bots often fail when customers use non-standard phrasing, multilingual messages, slang, local billing terms, accessibility tools, or speech-to-text errors. Monitor these failures separately from ordinary "unknown intent" and route to localization, accessibility, or knowledge owners.

Containment metrics can hide language inequity.

Regularly review real bot handoffs for missing context, wrong intent, unsafe data requests, and customer frustration. Aggregate metrics alone rarely show the conversation-level failure.

## Common Traps

- Treating containment rate as the primary success metric.
- Letting bots handle high-risk billing, access, safety, legal, privacy, fraud, or regulated cases.
- Requiring customers to repeat failed bot steps before human help.
- Handing off with no transcript or context summary; asking for unsafe sensitive data through bot flows
- Ignoring accessibility barriers in bot interaction; giving bot language that overpromises or misstates policy
- Hiding the human option to protect deflection; failing to analyze abandoned bot sessions
- Not routing bot failure evidence to owners; making customers prove repeatedly that the bot misunderstood them before human handoff
- Treating multilingual, accessibility, slang, or speech-to-text failures as generic low-confidence noise; relying only on aggregate bot metrics and never reading failed handoff transcripts

## Self-Check

- Is the bot task low-risk, structured, repeatable, and appropriate for automation?
- Are sensitive cases excluded or routed conservatively?
- Are handoff triggers defined for repeated failure, human request, impact, unclear intent, sensitive data, safety, legal, privacy, takeover, fraud, deadline, accessibility, abuse, and low confidence?
- Does handoff preserve intent, bot path, answers, identifiers, articles, actions, errors, sentiment, triggers, and unresolved questions?
- Are unsafe data requests and risky actions avoided?
- Are resolution, repeat contact, reopen, escalation, abandonment, frustration, and high-risk miscontainment measured?
- Is human help visible and respectful?
- Are bot responses reviewed for policy, tone, security, legal, refund, and escalation accuracy?; are bot failures routed with concrete evidence and recommended fix?
- Would the customer experience less effort after the bot, not more?; can the customer correct misunderstanding, change intent, or request a human without excessive friction?
- Are language, localization, accessibility, slang, and speech-to-text failures tracked and owned?; are real handoff samples audited for missing context, wrong intent, unsafe asks, and frustration?
