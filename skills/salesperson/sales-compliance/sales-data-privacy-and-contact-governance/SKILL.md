---
name: sales-data-privacy-and-contact-governance.md
description: Use when the agent is collecting, enriching, importing, exporting, sharing, cleaning, or using prospect and customer contact data in sales systems, CRM, enrichment tools, spreadsheets, event lists, partner lists, account research, AI tools, or routing workflows where risks include excessive personal data, stale or unverified enrichment, unauthorized sharing, hidden sensitive attributes, poor data lineage, privacy request mishandling, or sales decisions based on data the organization cannot lawfully or ethically use.
---

# Sales Data Privacy And Contact Governance

Sales teams depend on contact data, but not all available data should be collected or used. Prospect records, enrichment fields, event lists, partner files, CRM notes, and AI-generated research can contain stale, sensitive, inferred, or unauthorized information. Agents often treat data quality as a productivity issue and miss privacy, lineage, consent, retention, and fairness risks. This skill helps the agent govern sales data so outreach and account work are useful without becoming invasive or noncompliant.

Use this skill when importing leads, enriching accounts, researching contacts, updating CRM, using AI tools for prospecting, sharing lists, segmenting buyers, handling privacy requests, or deciding which contact information belongs in sales systems. The agent should apply data minimization and route uncertain privacy questions to approved owners.

## Core Rules

### Collect only data with a sales purpose

Every field should support a legitimate sales, service, routing, compliance, or account-management purpose. Role, company, work email, business phone, region, industry, account relationship, consent status, and buying context may be relevant. Personal details, protected characteristics, family information, health, politics, religion, inferred financial distress, or private social content usually do not belong in sales records.

Do not collect data merely because an enrichment tool offers it. Excess data increases privacy risk and can make outreach feel invasive.

### Preserve source and consent lineage

When importing or enriching contacts, record where the data came from, when it was obtained, what disclosure or consent applied, who provided it, and any use restrictions. Data lineage determines whether outreach, sharing, or retention is allowed.

Avoid mixing list sources in a way that erases restrictions. A customer list, event opt-in list, purchased list, partner referral, and scraped public data may have different permitted uses.

### Validate enrichment before relying on it

Enrichment data can be stale, inferred, mismatched, or attached to the wrong person. Before using it for personalization, routing, territory assignment, or qualification, check confidence and source. Do not make sensitive or high-impact decisions based only on enrichment.

If a buyer corrects information, update the system and consider whether the source should be distrusted. Bad data creates bad outreach and privacy complaints.

### Keep sensitive inferences out of CRM

Sales notes should not include speculative or sensitive inferences about a person's health, family, religion, politics, age, ethnicity, disability, immigration status, financial hardship, or personal life. Even when a detail is publicly visible, storing and using it may be inappropriate.

Use professional, role-based context. "Owns security evaluation for cloud tools" is useful. "Likely under pressure after personal event" is not.

### Control list sharing and exports

Lead lists and CRM exports can spread quickly through spreadsheets, agencies, partners, and AI tools. Share only with approved recipients and tools, under permitted purpose and access controls. Avoid downloading broad lists when a smaller segment is enough.

If a partner or vendor receives contact data, check agreement, transfer rules, suppression requirements, deletion expectations, and whether the contacts consented to partner use. Do not casually send lists over email or chat.

### Respect privacy requests and data corrections

Opt-out, deletion, correction, access, objection, and do-not-sell or do-not-share requests may arrive through sales channels. The seller should recognize them and route to the privacy process promptly. Do not argue, continue selling, or silently delete only local notes while other systems retain data.

If a privacy request affects active opportunity communication, coordinate with legal, privacy, revenue operations, and account owners. Respecting the request does not mean losing all business context; it means using the approved path.

### Use AI tools with data boundaries

AI tools can help summarize accounts or draft outreach, but they may not be approved for personal data, confidential account data, customer notes, call transcripts, or proprietary lists. Check tool approval and data handling rules before uploading sales data.

Do not ask AI to infer sensitive traits or produce targeting based on protected characteristics. Keep prompts and outputs within professional business context.

### Periodically clean and retire data

Stale contacts, bounced emails, departed employees, old opportunity notes, obsolete consent, and inactive leads create poor outreach and privacy risk. Define cleanup rules for invalid data, retention periods, duplicate records, suppression updates, and ownership of stale accounts.

Data governance is ongoing. A CRM that only grows and never retires data becomes less useful and more risky.

## Common Traps

- Importing or enriching contacts without knowing source, consent, or use restriction.
- Keeping sensitive personal details because they might help personalize outreach.
- Using enrichment confidence as if it were verified buyer truth.
- Exporting broad lead lists to spreadsheets, partners, or AI tools without access and purpose controls.
- Treating privacy requests as sales objections instead of compliance signals.
- Deleting data from one local file while leaving it active in CRM, sequencing tools, and partner systems.
- Segmenting or prioritizing prospects using protected characteristics or sensitive inferences.
- Letting stale contacts remain in active outreach because cleanup lowers list size.

## Self-Check

- Does each data field have a legitimate business purpose?
- Are sensitive personal details and protected-characteristic inferences excluded from CRM and outreach?
- Is source, date, consent, disclosure, and use restriction preserved for imported or enriched data?
- Has enrichment been verified before use in personalization, routing, or qualification?
- Are list exports limited, access-controlled, and shared only with approved recipients or tools?
- Are partner and vendor data transfers governed by agreement and permitted use?
- Are privacy requests recognized and routed through the approved process?
- Are AI tools approved for the type of sales data being used?
- Are stale, bounced, duplicate, departed, or inactive records cleaned under defined rules?
- Would the data use still look appropriate if the prospect asked how the company obtained and used their information?
