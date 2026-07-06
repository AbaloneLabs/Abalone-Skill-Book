---
name: multilingual-support-quality.md
description: Use when the agent is handling support across multiple languages, reviewing translated customer replies, deciding whether to use machine translation, routing to bilingual agents, preserving meaning across languages, or maintaining quality for customers who are not using the team's primary language.
---

# Multilingual Support Quality

Multilingual support is not simply translating words. Customers may be explaining urgent problems in a second language, using region-specific product terms, or depending on a translated answer to make billing, safety, access, or legal decisions. Agents often miss meaning, tone, eligibility, or escalation signals when language feels like a tooling problem. This skill helps the agent preserve support quality when language introduces uncertainty.

## Core Rules

### Identify the language need and risk level

Determine whether the customer needs full support in another language, a translated summary, help understanding a policy, or simple navigation. Then assess risk: billing, identity, privacy, safety, legal, data loss, accessibility, account access, abuse, or contractual issues require higher accuracy.

Low-risk translation can tolerate more tooling support. High-risk translation needs verified language quality or specialist review.

### Preserve the customer's meaning

Do not over-smooth unclear language until the customer's actual uncertainty disappears. If a phrase could mean refund, chargeback, cancellation, account deletion, or subscription pause, clarify before acting.

Keep important terms such as dates, amounts, account names, error codes, product labels, and policy categories intact. Misreading one term can change the whole case.

### Use machine translation with limits

Machine translation can help triage and draft, but it can mistranslate tone, negation, idioms, legal terms, medical or safety references, names, gender, and product-specific language. Treat it as assistance, not proof.

For sensitive or high-impact cases, route to a qualified language speaker, localization reviewer, or approved escalation path. If no such path exists, acknowledge the language limitation and avoid irreversible action until meaning is clear.

### Keep customer-facing language plain

Use short sentences, concrete next steps, and avoid idioms, humor, slang, sarcasm, culturally specific references, and dense policy phrasing. Plain language improves both human comprehension and translation accuracy.

If the customer uses formal language, match respect without becoming vague.

### Confirm critical understanding

For actions involving money, cancellation, deletion, identity, security, address, order, warranty, or legal process, confirm the customer's intent in clear language before proceeding. Ask targeted questions, not broad "please explain again" prompts.

Confirmation is especially important when the customer is using a translated interface or communicating through an interpreter.

### Maintain consistent terminology

Product names, plan names, error messages, policy categories, and legal names should match approved localization glossaries where available. Inconsistent terminology can make customers think features, plans, or rights differ when they do not.

If a feature is not localized, show the customer-visible term and explain it.

### Respect channel and staffing limits

If the organization does not provide full support in a language, be transparent about available options while still helping safely. Do not pretend fluency or leave the customer with unusable English-only instructions for a critical issue.

Track demand for unsupported languages as an operations signal.

### Document language handling

Record the customer's preferred language, whether translation was used, whether a bilingual reviewer handled the case, and any uncertainty that affected decisions. Future agents need to understand why clarification or escalation happened.

### Use confirmation loops for critical meaning and handoff language context between agents

When translation uncertainty remains, use a confirmation loop: restate the understood request in simple language, list the intended action, and ask the customer to confirm before the team proceeds. This is better than asking the customer to "clarify everything" because it narrows the risk.

For example, distinguish "cancel renewal," "cancel immediately," "delete account," and "stop marketing emails" before action. Distinguish "I was charged twice" from "I see an authorization hold." Critical meaning should be confirmed in the language the customer can understand.

Multilingual cases often move between queues, shifts, outsourced teams, and specialists. Preserve language preference, translation uncertainty, terms already clarified, and any customer frustration caused by language friction. Do not make the customer restart the language negotiation with every transfer.

If a bilingual agent has interpreted a key phrase, document the interpretation and confidence. Later agents should not overwrite it casually through machine translation.

## Common Traps

- Treating translation as sufficient without checking case risk.
- Acting on ambiguous translated intent in billing, deletion, access, privacy, or safety cases.
- Removing uncertainty while rewriting the customer's message.
- Using idioms, jokes, or long sentences that translate poorly.
- Assuming machine translation captured negation, dates, amounts, names, gender, or policy terms correctly; sending English-only steps for a critical workflow because no localized macro exists
- Mixing localized and internal product terms; forgetting to confirm the customer's intent before irreversible action
- Failing to record that translation uncertainty affected the case; ignoring repeated requests for unsupported languages as staffing or localization signals
- Asking the customer to re-explain broadly instead of confirming specific high-risk meaning; dropping language context during handoff so the next agent repeats the same confusion

## Self-Check

- Is the customer's language need and risk level identified?
- Are billing, identity, privacy, safety, legal, data loss, access, abuse, accessibility, and contractual risks treated with higher translation caution?
- Has ambiguous intent been clarified before action?
- Are dates, amounts, names, account identifiers, error codes, product labels, and policy categories preserved accurately?
- Is machine translation used within its limits and reviewed for sensitive cases?
- Is customer-facing language plain, concrete, and free of idioms or sarcasm?
- Are critical actions confirmed in the customer's understood language?
- Are approved localized terms or customer-visible terms used consistently?
- Are unsupported language limitations communicated honestly with a safe path?; is language handling documented for future agents and operations review?
- Was critical meaning confirmed through a clear restatement before high-risk action?; does the handoff preserve language preference, clarified terms, translation uncertainty, and reviewer confidence?
