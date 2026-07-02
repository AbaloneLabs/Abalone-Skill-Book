---
name: data_subject_access_request_handling.md
description: Use when the agent is designing or operating a DSAR or SAR intake process, verifying requester identity, scoping and searching systems, managing GDPR one-month and CCPA 45-day timelines, applying exemptions, or deciding response content and fees.
---

# Data Subject Access Request Handling

The data subject access request is the most operationally demanding privacy obligation. It forces an organization to find, assemble, review, and deliver an individual's data across every system within a fixed clock, while balancing the requester's rights against the rights of others and the organization's legitimate interests. Most failures are not legal misunderstandings; they are operational breakdowns: missed intake channels, slow identity checks, incomplete searches, or responses that omit data the organization actually holds.

Use this skill before building a DSAR workflow, training support staff, configuring intake channels, or responding to a live request. The goal is to make the agent treat the DSAR as a timed, evidence-generating operational process with clear ownership, not as an ad-hoc email reply.

## Core Rules

### Capture Every Intake Channel And Start The Clock Immediately

A DSAR does not require a specific form, magic words, or a fee. It can arrive through any channel: email, support ticket, chat, social media, in person, or via an authorized agent. The clock starts when the request is received, not when it is routed to the privacy team.

Build intake to:

- recognize requests regardless of phrasing or channel;
- log the receipt date and time as the start of the clock;
- route every request to a defined owner within one business day;
- provide a public, accessible way to submit requests;
- accept requests from authorized agents with appropriate verification.

Train customer support, HR, and account management to recognize and escalate requests, because they are usually the first recipients.

### Verify Identity Proportionately

You must confirm the requester is the data subject, but verification must be proportionate to the sensitivity of the data and the risk of disclosure to the wrong person.

Calibrate verification:

- for low-risk data, account-level verification such as logged-in session or standard account credentials may suffice;
- for sensitive or high-volume data, additional verification such as identity documents may be justified;
- avoid collecting more data than necessary for verification, and delete verification artifacts after use;
- handle authorized agent requests by verifying both the agent's authority and the data subject's identity where appropriate.

Over-collection of identity documents for routine requests is itself a privacy issue. Excessive verification can also be treated as an obstruction that breaches the right of access.

### Scope The Request And Clarify Ambiguity

Requests are often broad or vague. You may ask for clarification where the request is manifestly unfounded or excessive, or where clarification genuinely helps, but you cannot use clarification as a delay tactic.

Scope by:

- confirming the data subject and any date range or categories requested;
- identifying all systems, databases, backups, and vendor-held data in scope;
- clarifying whether the request covers special category or employee data;
- pausing the clock only where the law explicitly permits clarification.

Document the scope decision and any clarification, with timestamps.

### Search Comprehensively Across Systems And Vendors

An incomplete search is the most common DSAR defect. The response must cover all data the controller holds, including data held by processors on the controller's behalf.

Search:

- production databases, data warehouses, and data lakes;
- application logs, audit trails, and event streams;
- customer support tools, ticketing systems, and CRM;
- email and collaboration systems where personal data may reside;
- backups and archives, with a documented position on how they are handled;
- vendor and processor systems, by invoking contractual DSAR support obligations.

Use the data inventory and lineage maps to ensure no system is missed. If a system is not searched, record why and whether that is defensible.

### Manage The Timeline Actively

Timelines are short and unforgiving. GDPR generally gives one month from receipt, extendable by two further months for complex cases with notice within the first month. CCPA/CPRA generally gives 45 days, extendable by up to 45 more for complex requests with notice.

Manage the clock by:

- tracking the statutory deadline from receipt, not from completion of identity verification;
- issuing extension notices on time where complexity justifies an extension;
- prioritizing searches across many systems in parallel;
- building a checklist with intermediate milestones, not only the final deadline;
- escalating at-risk requests before the deadline, not after.

A missed deadline is a violation regardless of the quality of the eventual response.

### Apply Exemptions And Redactions Correctly

Not all data must be disclosed. Several exemptions protect other people, legal privilege, and legitimate interests, but they must be applied narrowly and documented.

Apply exemptions carefully:

- redact information that would identify another individual unless that person has consented;
- preserve legal professional privilege where applicable;
- consider exemptions for ongoing investigations, crime, and confidential business information where the law provides them;
- avoid using exemptions broadly to withhold data that should be disclosed.

Over-redaction is as much a failure as under-redaction. Each redaction should have a documented basis.

### Deliver Complete And Understandable Response Content

The response must contain the data in a concise, transparent, intelligible, and easily accessible form, along with prescribed metadata.

Include:

- confirmation that data is being processed;
- a copy of the personal data;
- the purposes of processing;
- the categories of data and recipients;
- retention periods or criteria;
- the right to rectify, erase, restrict, object, and complain;
- the source of the data where not collected from the individual;
- the existence of automated decision-making and meaningful information about the logic.

Deliver the data in a commonly used, machine-readable format where portability is requested. Avoid dumping raw database exports that an individual cannot interpret.

### Respect Fee Limits

Fees are restricted. GDPR access is generally free, with a reasonable fee only for manifestly unfounded or excessive requests or additional copies. CCPA/CPRA prohibits charging fees except for reasonable cost-based fees for additional copies. Do not use fees to deter legitimate requests.

### Document The Entire Process

Every DSAR should generate a defensible record. If a regulator asks what you did, the record should answer without reconstruction from memory.

Record:

- receipt date, channel, and requester identity verification;
- scope and any clarification;
- systems searched and data located;
- exemptions and redactions applied with reasons;
- response date, format, and delivery confirmation;
- any extension and the notice given.

## Common Traps

### Missing Requests In Support Or Social Channels

Requests that arrive via support tickets or social media and are not recognized start a clock the organization does not know it is running.

### Over-Collecting Identity Documents

Demanding passport copies for low-risk data is disproportionate and itself a privacy risk, and can be treated as obstruction.

### Searching Only The Primary Database

Limiting the search to the customer database while ignoring warehouses, logs, support tools, and vendor-held data produces an incomplete and non-compliant response.

### Letting The Clock Run During Slow Identity Checks

Excessive verification delays consume the statutory window and risk a missed deadline.

### Broad Redactions That Defeat The Request

Redacting so heavily that the response is meaningless is treated as a refusal, even though data was technically returned.

### Dumping Raw Exports As The Response

A raw database export is not intelligible or easily accessible and fails the format requirement.

### Ignoring Authorized Agents

Refusing requests from validly authorized agents, or failing to verify the agent's authority, both create compliance gaps.

### Treating Deadlines As Suggestions

Late responses are violations even when the data is eventually correct, and extensions given after the original deadline has passed do not cure the breach.

## Self-Check

- Are all intake channels recognized, with the clock started at receipt and routed to a defined owner within one business day?
- Is identity verification proportionate to data sensitivity, avoiding over-collection of identity documents?
- Is the request scoped and clarified where genuinely needed, with the clock paused only where the law permits?
- Does the search cover databases, warehouses, logs, support tools, collaboration systems, backups, and vendor-held data?
- Is the statutory timeline (GDPR one month, CCPA 45 days) tracked from receipt with extensions noticed on time?
- Are exemptions and redactions applied narrowly, with a documented basis for each?
- Does the response include the prescribed metadata: purposes, categories, recipients, retention, rights, source, and automated decision logic?
- Is the data delivered in an intelligible, easily accessible, and where requested machine-readable format?
- Are fees limited to what the law permits and not used to deter legitimate requests?
- Is the entire process documented with receipt, scope, search, redactions, response, and any extension?
