---
name: community-feedback-and-product-signal.md
description: Use when the agent is extracting product, policy, documentation, launch, bug, sentiment, or support-quality signals from community forums, peer discussions, public threads, user groups, beta communities, creator communities, or customer-to-customer support spaces.
---

# Community Feedback And Product Signal

Community spaces contain product signals that do not look like tickets. Customers may compare workarounds, teach each other hidden steps, complain about missing features, report bugs informally, or describe confusion before contacting support. Agents can miss these signals because community discussion feels noisy and unofficial. This skill helps the agent turn community patterns into credible product and support feedback without overgeneralizing.

## Core Rules

### Separate signal from noise

Community posts may include venting, speculation, peer guesses, duplicate complaints, jokes, spam, or highly technical detail. Identify the actionable signal: blocked workflow, unclear policy, regression, documentation gap, unsupported workaround, feature demand, onboarding friction, or trust issue.

Do not forward a thread simply because it is loud. Extract the pattern and evidence.

### Capture context and representativeness

Record who appears affected, product area, plan or role if known, region, device, version, integration, timing, workaround, and whether the issue is new or recurring. State whether the sample is a few vocal users, a broad thread, or a repeated pattern across posts.

Community data is valuable but biased toward participants.

### Preserve customer language

Community wording can reveal how customers name the problem. Capture representative phrases or paraphrases that show expectation mismatch, not just internal taxonomy.

Redact private data and avoid copying inflammatory language unless it is necessary to understand safety, harassment, or reputational risk.

### Verify before escalating as fact

Peer reports may be accurate, outdated, regional, caused by user configuration, or tied to an incident. Check against known issues, release dates, support tickets, telemetry, help center content, or product owner guidance where available.

Escalate uncertainty clearly rather than presenting community speculation as confirmed defect.

### Watch unofficial workaround adoption

When many users share a workaround, it may indicate missing product capability, documentation failure, or support distrust. It may also create risk if it bypasses security, billing, data integrity, or policy.

Flag risky workarounds and route them for official guidance.

### Track sentiment and trust

Community signal includes emotional pattern: customers feel ignored, confused, punished, excited, betrayed, or dependent on peers because official support is slow. Sentiment can indicate adoption risk, churn risk, or communication failure.

Do not reduce community feedback to feature counts only.

### Link to product or knowledge owners

Route signals to the right owner: product, design, engineering, docs, localization, accessibility, trust and safety, billing, policy, or support operations. Include a concise summary, evidence links, severity, and suggested next check.

### Close the public loop where appropriate

If product confirms a bug, docs are updated, or official guidance is created, return to the community with safe public information. Do not share internal roadmap or private customer data.

Closing the loop improves trust and reduces repeated threads.

### Prioritize public-risk signals and measure response effect

Some community patterns require faster attention because they can spread publicly: unsafe workaround, security misconception, billing rumor, data-loss claim, discriminatory experience, accessibility barrier, legal allegation, outage speculation, or viral complaint. These signals may deserve escalation even before ticket volume is high.

The public nature of the channel changes the risk. A misleading workaround viewed by many customers can create more harm than a larger number of private tickets.

After official guidance is posted, watch whether duplicate threads decrease, customers continue to misunderstand, risky workarounds stop spreading, or sentiment improves. If the same confusion continues, the answer may be incomplete, hard to find, or not trusted.

Community feedback work is not finished when the escalation is sent.

## Common Traps

- Treating loud threads as representative without context.
- Ignoring community signal because it is not a formal ticket.
- Forwarding raw threads without extracting the actionable pattern.
- Presenting peer speculation as confirmed product defect.
- Missing risk in widely shared unofficial workarounds; removing customer language until product cannot understand expectation mismatch
- Focusing only on feature requests and ignoring sentiment or trust damage; failing to route signals to the correct owner
- Sharing internal product decisions publicly before approval; not returning with official guidance after the company acts
- Underestimating low-volume public signals that can spread harmful workarounds or rumors; posting official guidance but not checking whether it reduced confusion or duplicate threads

## Self-Check

- Is the actionable signal separated from venting, speculation, duplicates, and spam?
- Does the summary capture affected context such as product, plan, role, region, device, version, integration, timing, and workaround?
- Is representativeness stated honestly?
- Are customer phrases or paraphrases preserved without private data?
- Are known issues, releases, tickets, telemetry, docs, or product guidance checked before presenting claims as fact?
- Are unofficial workarounds assessed for security, billing, data, policy, and support-risk impact?
- Does the signal include sentiment, trust, adoption, and churn context where relevant?
- Is the right owner identified across product, design, engineering, docs, localization, accessibility, trust, billing, policy, or operations?
- Is the escalation packet concise with links, severity, and next check?; is there a plan to close the public loop with approved information where appropriate?
- Are public-risk signals such as unsafe workarounds, security misconceptions, billing rumors, data loss, accessibility barriers, legal allegations, and outage speculation prioritized?; is the effect of official guidance monitored through duplicate threads, sentiment, continued confusion, and workaround spread?
