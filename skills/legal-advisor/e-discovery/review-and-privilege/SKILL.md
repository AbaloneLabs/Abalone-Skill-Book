---
name: review_and_privilege.md
description: Use when the agent is designing or managing a document review workflow, selecting review strategies including linear, categorization, or technology-assisted review and predictive coding, conducting privilege and confidentiality review, building privilege logs, managing quality control and sampling, or producing documents in e-discovery.
---

# Review And Privilege

Review is the most expensive phase of e-discovery and the phase where privilege and confidentiality are most at risk. The review strategy determines whether the right documents are produced, whether privileged material is withheld correctly, and whether the production withstands challenge. Agents often default to linear human review of every document, when modern review uses technology-assisted review and predictive coding to manage large volumes, and the defensibility of the chosen method depends on process, validation, and documentation. Privilege review is equally technical: over-withholding delays and invites motions; under-withholding waives privilege and can be irreversible.

The review workflow, quality control, privilege logging, and production must be designed as an integrated system, because errors in one phase propagate to the others. Use this skill before designing a review workflow, before selecting a review technology, before conducting privilege review, before building a privilege log, or before managing production. The goal is to help the agent design a defensible and efficient review, not to provide legal advice, which requires qualified counsel and experienced review professionals for the governing forum.

## Core Rules

### Choose A Review Strategy Matched To Volume And Complexity

Review strategy should reflect the document volume, the complexity of the issues, the available technology, and the need for defensibility. A single strategy rarely fits all matters.

Compare strategies:

- linear review, reading every document, defensible but expensive and slow;
- keyword and concept clustering to group similar documents;
- technology-assisted review (TAR) and predictive coding using machine learning;
- cascading or tiered review, with machine triage and human review of key sets;
- the suitability of each to the volume, complexity, and timeline;
- the defensibility of the chosen method under the governing rules;
- the validation and sampling needed to support the method.

TAR can be far more efficient for large volumes, but its defensibility depends on transparent process, seed sets, validation, and documentation. Linear review remains appropriate for small or highly sensitive sets.

### Design A Review Workflow That Catches Errors

A review workflow should move documents through stages that catch issues before production. Each stage should have defined criteria and quality control.

Design stages:

- initial triage or categorization;
- first-level review for relevance and privilege;
- second-level or senior review of borderline calls;
- privilege review and logging;
- redaction of confidential or privileged content;
- quality control sampling and consistency checks;
- production preparation and final QC;
- exception handling for encrypted, corrupted, or special-format files.

A workflow without QC produces inconsistent calls and privilege errors. Build sampling and escalation into each stage.

### Implement TAR And Predictive Coding Defensibly

Technology-assisted review can dramatically reduce cost, but its use must be defensible. Courts accept TAR when the process is transparent and validated.

Implement defensibly:

- document the TAR protocol and the algorithm's general approach;
- select and validate seed or training sets with subject-matter expertise;
- use recall and precision metrics to measure performance;
- validate results with statistical sampling;
- eliterative training to improve accuracy where appropriate;
- document the decision to use TAR and the validation results;
- be prepared to explain the method to opposing counsel and the court;
- combine TAR with human review of high-priority or borderline documents.

A black-box TAR process without validation is hard to defend. Transparency and documentation are the keys to defensibility.

### Conduct Privilege Review Systematically

Privilege review identifies documents that should be withheld under attorney-client privilege or work product doctrine. It must be systematic to avoid both over- and under-withholding.

Conduct:

- identify privileged communications, attorney-client and work product;
- apply privilege tags consistently across the review team;
- handle common-interest and joint-defense privileges where applicable;
- identify and log each withheld document;
- handle partially privileged documents with redaction;
- screen for inadvertently produced privileged material under clawback;
- separate business communications from legal advice;
- address the privilege implications of embedded or forwarded communications.

Over-withholding delays production and invites motions to compel; under-withholding waives privilege, which can be irreversible. Calibrate the review carefully.

### Build A Defensible Privilege Log

A privilege log substantiates the withholding of privileged documents. An inadequate log invites motions to compel and sanctions.

Build logs with:

- author, recipient, and copied parties;
- date and document type;
- subject matter description sufficient to assess privilege without disclosing content;
- the privilege asserted, attorney-client or work product;
- the basis, such as communication for legal advice or work product prepared in anticipation of litigation;
- consistency across entries;
- handling of voluminous or repetitive documents through categories where permitted;
- timely production of the log with or before the document production.

A log with generic descriptions such as "privileged communication" is routinely rejected. Provide enough detail to assess the privilege claim.

### Manage Confidentiality And Redactions

Beyond privilege, documents may contain confidential information requiring redaction or protective orders. Redactions must be defensible and properly applied.

Manage:

- the basis for each redaction, privilege, confidentiality, or PII;
- the application of redactions so underlying text is not recoverable;
- the documentation of redaction reasons;
- the use of protective orders to limit disclosure of sensitive material;
- the handling of personally identifiable information and regulated data;
- the consistency of redaction across similar documents;
- the avoidance of over-redaction that renders documents meaningless.

Redactions applied improperly, such that text remains recoverable, can expose privileged content. Verify redaction integrity in the produced form.

### Apply Quality Control And Sampling Throughout

QC catches inconsistencies, privilege misses, and relevance errors before production. It is the backbone of a defensible review.

Apply QC:

- sampling at each review stage;
- consistency checks across reviewers;
- second-level review of privilege and relevance calls;
- statistical sampling of production sets;
- tracking of reviewer accuracy and retraining where needed;
- documentation of QC findings and remediation;
- escalation of difficult calls to senior reviewers or counsel.

A review without QC is an undefended process. Document the QC methodology and results.

### Coordinate Production With Deadlines And Form

Production must meet deadlines, follow the agreed form, and include all required components. Last-minute production creates errors.

Coordinate:

- the production schedule against court deadlines;
- the form of production, native, image, or both;
- the load files, fielded data, and text;
- the privilege log accompanying production;
- the redactions and their documentation;
- the Bates numbering and document relationships;
- the resolution of exceptions before production;
- the confirmation that nothing privileged is inadvertently produced.

Production errors, missing attachments or privileged documents, are hard to unwind. Use a pre-production checklist and final QC.

## Common Traps

### Defaulting To Linear Review For Large Volumes

Linear review of every document is often uneconomic. Consider TAR and tiered strategies for large sets.

### Black-Box TAR Without Validation

TAR without documented seed sets, metrics, and sampling is hard to defend. Validate and document.

### Inadequate Privilege Logs

Generic log entries are rejected. Provide author, recipient, date, type, subject, and privilege basis.

### Inconsistent Privilege Calls Across Reviewers

Inconsistent tagging undermines credibility. Use QC, guidelines, and escalation.

### Recoverable Redactions

Redactions that leave text recoverable expose privileged content. Verify redaction integrity.

### No QC Sampling

A review without QC is undefended. Sample at each stage and document findings.

### Last-Minute Production Without Final QC

Rushed production produces errors and privilege waivers. Use a checklist and final QC.

## Self-Check

- Is the review strategy matched to volume, complexity, technology, defensibility, and validation, comparing linear, clustering, TAR, cascading, and tiered approaches?
- Is the review workflow designed with triage, first-level, second-level, privilege, redaction, QC, production, and exception stages?
- Is TAR implemented defensibly with documented protocol, validated seed sets, recall and precision metrics, statistical sampling, iterative training, decision documentation, and explanation readiness?
- Is privilege review systematic for attorney-client and work product, common-interest and joint-defense, consistent tagging, logging, redaction, clawback, business versus legal separation, and embedded communications?
- Is the privilege log defensible with author, recipient, date, type, subject, privilege basis, consistency, voluminous-document categories, and timely production?
- Are confidentiality and redactions managed for basis, proper application, documentation, protective orders, PII and regulated data, consistency, and avoidance of over-redaction?
- Is QC and sampling applied at each stage with consistency checks, second-level review, statistical sampling, reviewer accuracy tracking, documentation, and escalation?
- Is production coordinated with deadlines, form, load files, privilege log, redactions, Bates numbering, exception resolution, and privilege-inadvertent-production prevention?
- Does the output escalate to qualified counsel and experienced review professionals for strategy, privilege, TAR defensibility, and production?
