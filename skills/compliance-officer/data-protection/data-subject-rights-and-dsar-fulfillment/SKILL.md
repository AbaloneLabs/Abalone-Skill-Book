---
name: data_subject_rights_and_dsar_fulfillment.md
description: Use when the agent is responding to data subject access requests, managing individual rights workflows, evaluating verification of requester identity, handling deletion and portability requests, or ensuring timely and complete fulfillment of privacy rights requests under GDPR, CCPA/CPRA, and other privacy laws.
---

# Data Subject Rights And DSAR Fulfillment

Data subject rights (DSR) and data subject access request (DSAR) fulfillment governs how organizations respond when individuals exercise their privacy rights. The defining feature is that these rights are time-bound, identity-verified, and content-specific, and a failure to respond completely or on time is itself a violation regardless of the underlying data practices. The central difficulty is that locating all of an individual's data across distributed systems is technically hard, that verification must balance security against accessibility, and that exemptions and conflicts between rights must be navigated carefully.

Use this skill before advising on rights request intake, verification, search scope, response content, or deadline management. The goal is to make the agent identify the applicable rights, the verification method, the search scope across systems, and the exemptions before concluding that a response is complete.

## Core Rules

### Map The Applicable Rights By Jurisdiction

Rights differ by law but share common categories.

Cover:

- the right of access (what data you hold);
- the right to rectification (correction);
- the right to erasure (deletion / right to be forgotten);
- the right to data portability;
- the right to object to processing;
- the right to restrict processing;
- rights related to automated decision-making and profiling;
- the right to opt out of sale or sharing (CCPA/CPRA);
- the right to limit use of sensitive data;
- the right to non-discrimination for exercising rights.

GDPR grants the broadest set of rights. CCPA/CPRA focuses on access, deletion, opt-out of sale/sharing, and non-discrimination. The right to portability exists under GDPR but not CCPA. Automated decision-making rights are stronger under GDPR. The applicable rights depend on the requester's jurisdiction and the organization's obligations.

### Establish Intake And Authentication Workpoints

Requests must be captured and identity verified.

Implement:

- multiple intake channels (web form, email, toll-free number, in-app);
- a centralized request management system;
- identity verification proportionate to the request and data sensitivity;
- verification that avoids collecting excessive new data;
- handling of authorized agent requests (with verification of the agent's authority);
- recognition of requests submitted to any part of the organization;
- routing to the appropriate team within required timelines.

Intake must be accessible and not unduly burdensome. Verification must be proportionate: confirming identity for an access request may differ from confirming identity before deletion. Authorized agents (CCPA) must be verified for their authority to act. Requests can come through any channel and must be routed centrally.

### Meet Response Deadlines

Deadlines are strict and short.

Track:

- GDPR: one month, extendable by two months for complex requests (with notice);
- CCPA/CPRA: 45 days, extendable for up to 45 additional days with notice;
- other state laws with varying timelines;
- the clock starts at receipt, not at internal acknowledgment;
- the deadline for opt-out requests (immediate / within 15 business days for some laws);
- the deadline for appeals of denied requests.

The clock starts when the request is received, not when it is assigned internally. Extensions require notice to the requester within the original period. Opt-out requests often have shorter timelines. Failure to meet deadlines is a violation regardless of the merits.

### Define The Search Scope Across Systems

Access and deletion require finding all relevant data.

Cover:

- identification of all systems that may contain the individual's data;
- structured data (databases, CRM, ERP);
- unstructured data (email, documents, shared drives);
- backup and archive systems;
- log files and telemetry;
- data held by processors and vendors;
- data in analytics and marketing platforms;
- the treatment of pseudonymized and anonymized data.

The search scope must be comprehensive. Forgetting a system means an incomplete response. Backup tapes may be excluded from deletion if restoration is not reasonably possible, but must be documented. Data held by processors must be addressed through contractual flow-down. Pseudonymized data that can be re-linked is still personal data.

### Balance Competing Rights And Exemptions

Not all requests must be fulfilled in full.

Assess:

- exemptions for legal obligations (e.g., records required by law);
- the rights of others (not disclosing third-party data in an access response);
- intellectual property and trade secrets;
- legal privilege and litigation holds;
- the "manifestly unfounded or excessive" exemption (GDPR);
- the "reasonable security" and business-to-business exemptions (some state laws);
- conflicts between deletion and retention obligations.

Erasure does not require destroying records that must be kept by law (financial records, employment records, safety records). Access responses must not disclose third-party personal data without justification. Manifestly unfounded or excessive requests can be refused or charged, but the justification must be documented.

### Produce A Complete And Understandable Response

The response must be delivered in a usable format.

Deliver:

- the categories of data held;
- the specific data in a readable format;
- the purposes of processing;
- the recipients and categories of sharing;
- the retention periods;
- the rights available and how to exercise them;
- the source of the data (if not collected from the individual);
- the existence of automated decision-making and meaningful information about the logic;
- the response in the language and format appropriate to the requester.

Access responses must be intelligible, not raw database dumps. Portability must be in a structured, commonly used, machine-readable format. The response should explain the data in context. Translations may be needed for international requesters.

### Handle Deletion And Opt-Out Across The Data Lifecycle

Deletion and opt-out must propagate through downstream systems.

Manage:

- deletion from primary systems;
- propagation to backups, archives, and replicas;
- notification to processors and third-party recipients;
- opt-out propagation to advertising networks and data partners;
- the distinction between deletion and suppression (do-not-contact lists);
- verification that downstream deletion occurred;
- handling of data that cannot be deleted (anonymization as alternative).

Deletion is not complete until all copies are addressed, including those held by processors and shared third parties. Opt-out of sale/sharing must propagate to all recipients of the data. Suppression lists prevent re-collection but are not the same as deletion. Anonymization may be an alternative where deletion is not feasible.

### Manage Authorized Agents And Representative Requests

Authorized agents and legal representatives add complexity.

Address:

- verification of the agent's authority (power of attorney, written authorization);
- CCPA/CPRA rules on authorized agents (verification of consumer identity for registered agents);
- GDPR representative rights;
- requests from individuals lacking capacity (minors, incapacitated adults);
- requests following death (varying by jurisdiction);
- the interaction with power of attorney and guardianship.

Authorized agents must be verified for their authority to act on behalf of the individual. CCPA/CPRA has specific rules: registered agents must have written authorization, and for non-registered agents, the business may verify the consumer's identity directly. Requests from or on behalf of vulnerable individuals require careful handling.

## Common Traps

### Deadline Missed Because Internal Routing Was Slow

The clock starts at receipt; internal delays are not an excuse.

### Incomplete Search Missing Unstructured Data Or Vendor Systems

Forgetting email, shared drives, or processor-held data produces an incomplete response.

### Over-Verification Collecting Excessive Data

Requiring excessive identity documents or collecting new sensitive data for verification is disproportionate.

### Disclosing Third-Party Data In An Access Response

Including another person's personal data in an access response violates their rights.

### Deletion Not Propagated To Processors And Partners

Deleting from the primary system but not from vendors or ad networks is incomplete.

### Treating Opt-Out As Deletion

Opt-out of sale/sharing is not the same as deletion; both must be available separately.

### Refusing Requests Without Documented Exemption Basis

Refusing without citing a specific exemption and documenting the reasoning is non-compliant.

## Self-Check

- Are the applicable rights mapped by jurisdiction including access, rectification, erasure, portability, objection, restriction, automated decision-making, opt-out of sale/sharing, sensitive data limits, and non-discrimination?
- Is intake accessible through multiple channels with centralized management, proportionate identity verification, authorized agent handling, and routing within timelines?
- Are response deadlines met with correct start dates, extensions with notice, opt-out timelines, and appeal deadlines tracked?
- Is the search scope comprehensive across structured data, unstructured data, backups, logs, processor-held data, analytics, and pseudonymized data?
- Are competing rights and exemptions assessed including legal retention obligations, third-party rights, IP, privilege, manifestly unfounded/excessive, and B2B exemptions?
- Is the response complete and understandable with data categories, specific data, purposes, recipients, retention, rights, sources, automated decision-making information, and appropriate language and format?
- Is deletion and opt-out propagated across the lifecycle including primary systems, backups, processors, partners, ad networks, suppression lists, and anonymization alternatives?
- Are authorized agents and representatives handled with authority verification, CCPA/CPRA rules, GDPR representative rights, capacity issues, and post-death requests?
- Are request volumes, response times, and completion rates tracked for regulatory reporting?
- Is the request management system auditable to demonstrate compliance on demand?