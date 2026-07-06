---
name: hallucination-and-policy-risk-in-support.md
description: Use when the agent is checking AI-generated support content for hallucinated facts, invented policies, fabricated links, wrong plan terms, unsafe legal or medical claims, outdated procedures, policy misinterpretation, false eligibility, or confident answers that may not be grounded in approved support sources.
---

# Hallucination And Policy Risk In Support

In support, hallucination is not just a factual error; it can become a customer promise, privacy breach, financial error, or unsafe instruction. AI may invent policy, misremember product behavior, overgeneralize from similar cases, or create a persuasive explanation for a system state it has not verified. This skill helps the agent ground AI-assisted work in approved sources and evidence.

## Core Rules

### Treat unsupported specificity as suspect

Exact dates, amounts, eligibility windows, plan names, refund rules, error causes, release versions, contact routes, and links should be verified. AI often produces plausible specificity because it makes the answer sound helpful.

Specific does not mean sourced.

### Verify against authoritative sources

Use current policy, account system, billing record, order system, security procedure, legal-reviewed language, product release note, status page, or approved knowledge article. If the approved source is missing or stale, escalate rather than letting AI fill the gap.

Do not let AI become the source of truth.

### Check policy boundaries explicitly

AI may merge policies across regions, plans, channels, products, or customer types. Verify refund, warranty, subscription, privacy, tax, security, legal, safety, and regulated-response boundaries before relying on a generated answer.

Policy near-misses can be worse than no answer because they sound official.

### Watch for fabricated links and resources

AI may invent help articles, forms, phone numbers, escalation channels, regulatory resources, or crisis contacts. Confirm every link, contact path, and resource before sharing.

Never share invented emergency, legal, privacy, or security resources.

### Avoid confident root-cause claims

AI often explains why something happened before evidence is available. Separate symptoms, confirmed facts, likely causes, and unknowns. For incidents, bugs, payment failures, delivery exceptions, or account issues, use cautious language unless a source confirms cause.

Unsupported root cause can misroute the case and mislead the customer.

### Detect unsafe procedural advice

Generated instructions may ask customers to disable security, share credentials, delete data, bypass policy, run risky commands, ignore warnings, or use unofficial tools. Review every step for privacy, security, safety, data loss, and supportability risk.

A fluent procedure can still be dangerous.

### Preserve source gaps

If AI surfaces a reasonable answer but no approved source supports it, record the source gap and route to knowledge or policy owner. Do not quietly send the answer because it sounds right.

Repeated source gaps are knowledge management issues.

### Prefer uncertainty over invention

When evidence is missing, say what is known, what needs review, and what the next step is. A grounded partial answer is better than a complete fabricated answer.

Support credibility depends on not pretending.

### Check summaries as carefully as answers and maintain a source requirement for reusable AI output

AI summaries can hallucinate by omission: leaving out customer impact, prior promises, failed verification, policy denial, or sensitive risk. They can also change uncertainty into certainty. Treat summaries, tags, and escalation notes as claims that require review.

A bad summary can mislead every later human even if no customer sees it.

If an AI-generated answer is reused in macros, help content, bot flows, or training, require source traceability and owner approval. A one-off plausible answer should not become institutional guidance without review.

Hallucinations become harder to detect once they are copied into reusable support assets.

## Common Traps

- Trusting exact details because the draft sounds confident.
- Using AI output when the policy source is stale or missing.
- Mixing policies across regions, plans, products, and channels.
- Sharing fabricated forms, links, phone numbers, or escalation paths.
- Giving unsupported root cause for bugs, incidents, payments, or deliveries; sending generated troubleshooting steps without checking safety
- Allowing AI to create legal, medical, tax, or financial advice; treating repeated hallucinations as isolated draft issues instead of knowledge gaps
- Hiding uncertainty to make the response feel complete; forgetting that customers may rely on AI-generated promises
- Trusting AI summaries that omit prior promises, uncertainty, or risk signals; copying one AI answer into reusable content without source and owner approval

## Self-Check

- Are exact dates, amounts, windows, plan names, rules, causes, versions, routes, and links verified?
- Is every substantive claim grounded in account, billing, order, policy, security, legal, product, status, or approved knowledge source?
- Were policy boundaries checked across region, plan, channel, product, customer type, warranty, refund, subscription, privacy, tax, security, legal, safety, and regulated response?
- Are links, forms, phone numbers, escalation paths, regulatory resources, and crisis contacts real and approved?
- Are symptoms, facts, likely causes, and unknowns separated?
- Are generated procedures checked for security, privacy, safety, data loss, and supportability?
- Does the response avoid unauthorized legal, medical, tax, financial, or safety advice?
- Are source gaps routed to knowledge or policy owners?
- Does the response prefer honest uncertainty over invented completeness?; would the customer be protected if they relied on every sentence?
- Were AI summaries, tags, and escalation notes checked for omitted impact, promises, denials, verification, and risk?; if AI output is reused, is source traceability and owner approval present?
