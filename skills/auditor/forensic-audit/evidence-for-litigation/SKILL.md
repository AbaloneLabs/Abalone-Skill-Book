---
name: evidence_for_litigation.md
description: Use when the agent is preparing audit or forensic evidence for litigation, arbitration, regulatory action, government investigation, or commercial dispute, building chain of custody, assessing admissibility, handling privileged material, coordinating with counsel, or preparing expert testimony.
---

# Evidence For Litigation

Evidence gathered for an internal report is not automatically usable in court. Litigation and regulatory proceedings impose rules of admissibility, authentication, privilege, and preservation that ordinary audit workpapers were never designed to meet. Agents commonly assume that a well-organized file will speak for itself, then discover at deposition or hearing that the evidence is excluded because chain of custody is broken, metadata is destroyed, the file is over-privileged, or the work product was shared too broadly. The cost of that discovery is high: a strong case can collapse, sanctions can follow, and the auditor's credibility can be permanently damaged.

Use this skill whenever audit or forensic findings may be used in litigation, arbitration, a regulatory inquiry, or a contractual dispute. The goal is to gather, preserve, and present evidence so that it survives cross-examination, remains admissible, and does not create collateral risks through spoliation or waived privilege.

## Core Rules

### Establish And Maintain Chain Of Custody From The First Moment

Chain of custody proves that the evidence presented is the same evidence collected, unchanged. Once broken, opposing counsel will argue alteration, fabrication, or contamination. Chain of custody must begin at collection, not when the matter "gets serious."

For each item of evidence, record:

- a unique identifier for the item;
- date, time, and location of collection;
- the person who collected it and their authority;
- the source system, device, custodian, or location;
- the method of collection and tools used;
- hash values for digital files where feasible;
- every transfer, with from/to, date, and purpose;
- storage location and access controls;
- any handling, copying, or conversion performed;
- the current custodian.

Gaps must be explained, not papered over. Prefer forensic images and original media over re-exported copies.

### Preserve Broadly And Early To Avoid Spoliation

Spoliation is the destruction or alteration of relevant evidence, including automatic deletion. Courts and regulators can impose sanctions, adverse inferences, or fines even when destruction was negligent. The duty to preserve arises when litigation is reasonably foreseeable, which is often earlier than agents assume.

Take preservation actions:

- issue a written litigation hold to all relevant custodians;
- suspend auto-deletion of email, chat, logs, and backups;
- identify all potentially relevant data sources, including personal devices and cloud;
- preserve structured data, databases, and system logs, not just documents;
- capture metadata by imaging rather than copying files;
- document the hold, acknowledgements, and any gaps;
- reissue the hold periodically and when scope changes;
- preserve physical evidence in tamper-evident storage;
- record what could not be preserved and why.

Acting after a trigger event but before preservation can itself become a finding.

### Protect Privilege And Work Product Deliberately

Privilege and work product shield sensitive analysis from disclosure, but they are easily waived. The protections depend on who directs the work, who receives it, and how it is documented. Treating investigative analysis as ordinary business communication destroys the protection.

Manage privilege by:

- conducting the work at the direction of legal counsel where protection is intended;
- marking documents consistently and accurately;
- limiting distribution to need-to-know;
- separating privileged analysis from underlying facts;
- avoiding cc lists, broad email, and shared drives;
- controlling drafts and version history;
- not embedding mental impressions in fact-gathering documents;
- documenting the basis for privilege claims;
- reviewing for waiver before any external disclosure;
- distinguishing attorney-client privilege from work product and from in-house counsel's business roles.

When in doubt, route through counsel and ask before sharing.

### Authenticate Evidence So It Can Be Admitted

Authentication proves that evidence is what its proponent claims. Screenshots, spreadsheets, and exports often fail authentication unless a witness with knowledge or a system custodian can testify to their origin and integrity. Plan authentication before collection, not at trial.

For each evidence type, plan:

- who can authenticate it (system owner, custodian, records keeper);
- the business records foundation and any hearsay exceptions;
- certification requirements for foreign or electronic records;
- the system that produced it and its normal retention;
- whether a hash, log, or audit trail corroborates integrity;
- the original versus a duplicate and any Rule on duplicates;
- the need for a sworn declaration or affidavit;
- translation and certification for non-English records;
- the impact of any redactions on authenticity.

Evidence that cannot be authenticated is evidence that cannot be used.

### Gather For Admissibility, Not Just For Understanding

Investigators naturally collect what helps them understand the scheme. Litigation demands more: relevance, reliability, and compliance with evidentiary rules. A document that explains the fraud may still be excluded if it is hearsay without an exception or if its prejudice outweighs its probative value.

For each item, assess:

- relevance to a claim, defense, or element;
- reliability and corroboration;
- hearsay status and available exceptions;
- potential for exclusion on prejudice, confusion, or cumulativeness;
- whether it is being offered for the truth of the matter asserted;
- the need for live testimony versus documentary proof;
- whether it opens damaging collateral issues;
- whether it is the best evidence or a secondary copy;
- the effect of redactions on meaning.

Collect with the rules of evidence in mind, not just investigative instinct.

### Coordinate The Expert Witness Role Carefully

When audit or forensic conclusions depend on specialized knowledge, an expert may be required, and the expert's work is subject to separate rules, disclosures, and scrutiny. The investigator and the expert must align on methodology, assumptions, and the boundaries of opinion. Poor coordination produces opinions that are excluded or impeached.

Coordinate with the expert on:

- the question the opinion must answer;
- the data, assumptions, and limitations relied upon;
- the methodology and its acceptance in the field;
- alternative explanations considered and rejected;
- consistency with prior testimony and publications;
- the scope of the opinion and what it does not cover;
- report requirements and disclosure deadlines;
- deposition and cross-examination preparation;
- the distinction between fact witness and expert witness roles;
- retention, conflicts, and independence.

An expert who cannot defend the methodology at deposition adds risk rather than value.

### Document So Findings Survive Cross-Examination

Workpapers prepared for litigation will be examined line by line by hostile counsel. Conclusions stated without support, contradictions left unresolved, and overstatements become impeachment material. Documentation should be precise, contemporaneous, and free of unnecessary opinion.

Ensure documentation includes:

- the procedure performed and its objective;
- the population and how it was defined;
- the data source, extraction, and reconciliation;
- assumptions and their basis;
- exceptions, anomalies, and how they were resolved;
- the reasoning linking evidence to conclusion;
- limitations and what was not tested;
- who performed each step and when;
- reviewer sign-off and challenge notes;
- corrections with dates and reasons, never silent edits.

Assume every working note could be read aloud at deposition.

### Control Communications And Avoid Creating Harmful Records

Casual emails, draft conclusions, and speculative notes can become the most damaging exhibits in the case. Investigators often create records that overstate certainty, speculate about motive, or opine on legal conclusions. Discipline in communication is part of evidence handling.

Apply discipline to:

- avoiding legal conclusions in working papers;
- not speculating about intent or guilt beyond the evidence;
- refraining from jokes, sarcasm, or editorial comments in writing;
- not drafting conclusions before the evidence supports them;
- limiting internal emails to facts and next steps;
- routing analysis through privileged channels;
- not forwarding work product to non-privileged recipients;
- correcting, rather than deleting, errors;
- separating fact gathering from opinion formation;
- reviewing outgoing communications for unintended admissions.

## Common Traps

### Copying Files Instead Of Imaging

Copying changes metadata, drops deleted-file remnants, and breaks the integrity story. Use forensic imaging and work from the image, preserving the original read-only.

### Issuing A Hold But Not Enforcing It

A hold letter that custodians ignore, while deletion continues, can be treated as bad faith. Track acknowledgements, monitor compliance, and confirm suspension of deletion routines.

### Over-Collecting Without A Collection Plan

Sweeping entire mailboxes or servers increases cost, review burden, and the chance of exposing privileged or irrelevant material. Scope collection to relevance and document the criteria.

### Treating In-House Counsel Communications As Automatically Privileged

Not everything involving in-house counsel is privileged; business advice is not. Identify the purpose of each communication and structure it to support the privilege claim.

### Letting The Investigator Become An Unprepared Expert

If the investigator's analysis is offered as opinion, they may be deposed as an expert without the protections and preparation that role requires. Clarify roles with counsel early.

### Redacting Without Documenting The Basis

Unexplained redactions invite challenges and can be ordered produced. Record what was redacted, why, and the legal basis, and keep a non-redacted version under privilege.

### Relying On Self-Authenticating Assumptions

Screenshots and exports are not self-authenticating just because they look official. Line up the custodian, the system record, or the certification before relying on them.

### Failing To Preserve Metadata

Stripping metadata by saving into a new format can destroy dates, authors, and edit history that authenticate the document. Preserve native files and metadata wherever possible.

## Self-Check

- Does each item of evidence have a complete chain of custody from collection to present, with any gaps explained?
- Was a litigation hold issued, acknowledged, and enforced before any relevant deletion could occur?
- Is privilege and work product protected through counsel direction, controlled distribution, and consistent marking?
- Can each key exhibit be authenticated by a competent witness or record, with the foundation identified in advance?
- Has each item been assessed for relevance, hearsay, reliability, and exclusion risk under the applicable evidentiary rules?
- Is the expert role defined, with methodology, assumptions, limitations, and disclosure obligations coordinated with counsel?
- Do workpapers show procedure, population, data source, assumptions, exceptions, reasoning, and limitations without silent edits?
- Are communications free of legal conclusions, speculation, and editorializing that could become impeachment material?
- Are originals and metadata preserved through imaging, with copies used for working analysis?
- Are redactions documented with a stated basis and a privileged non-redacted version retained?
