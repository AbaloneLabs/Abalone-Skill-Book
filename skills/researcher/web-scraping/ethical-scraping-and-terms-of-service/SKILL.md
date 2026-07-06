---
name: ethical-scraping-and-terms-of-service.md
description: Use when the agent is scraping web data for research, interpreting terms of service and robots.txt, assessing the ethics and legality of collecting public data, handling personally identifiable information and platform-owned content, or deciding whether data that is technically accessible may ethically and legally be collected and used for research.
---

# Ethical Scraping And Terms Of Service

Just because data is technically accessible does not mean it may ethically or legally be collected and used for research. This is the central tension of web scraping, and agents frequently resolve it the wrong way, by assuming that accessible equals permissible. Public posts still involve people who may not have anticipated research use; terms of service may prohibit automated collection even of public data; and platform-owned content carries intellectual property constraints. The judgment problem is to interpret terms of service and robots.txt honestly rather than conveniently, to assess the privacy expectations of the people behind the data, and to weigh the research benefit against potential harm. Ethical scraping is not a box to check; it is a sustained analysis of consent, harm, and legality that must be documented. A scrape that ignores terms of service, exposes identifiable individuals, or republishes platform-owned content can cause real harm and invalidate the research.

Use this skill when planning web scraping for research. The goal is to prevent the agent from equating accessibility with permission, from ignoring terms of service and robots directives, and from exposing people or violating platform rights. The agent has freedom in what to study, but the ethical and legal analysis must be explicit, conservative, and documented.

## Core Rules

### Do Not Equate Accessibility With Permission

Data being viewable in a browser is not consent to automated collection and research use. These are separate questions.

Recognize by:

- the gap between technical access and authorized use;
- users' reasonable expectations about how their data will be used;
- the difference between public-to-humans and public-to-agents;
- platform-imposed access conditions as part of the permission analysis.

A user who posts publicly may expect human readers, not a research dataset republished elsewhere. Accessibility lowers some barriers but does not settle consent or authorization.

### Interpret Terms Of Service Honestly, Not Conveniently

Terms of service often restrict automated access. They must be read for what they say, not for what one hopes.

Interpret by:

- reading the actual terms, not assuming they permit research use;
- identifying clauses on automated access, scraping, and redistribution;
- not rationalizing violations as harmless or as too small to matter;
- seeking permission or using sanctioned APIs where terms restrict scraping.

Terms that prohibit scraping mean scraping is a breach of contract, however noble the research aim. Convenience is not an interpretation. Where terms are unclear, err toward caution or seek explicit permission.

### Respect Robots.txt And Access Directives

robots.txt and similar directives express the platform's wishes about automated access. They should be respected.

Respect by:

- reading and honoring robots.txt and meta directives;
- not circumventing access controls or authentication barriers;
- treating a directive as a statement of preference, not a challenge to bypass;
- documenting compliance with these directives.

Circumventing robots directives or login walls to reach data the platform has restricted is ethically and often legally problematic, regardless of technical feasibility.

### Assess Privacy Expectations Of The People Behind The Data

Even public data is about people, who may face harm from its collection, analysis, or publication.

Assess by:

- whether individuals are identifiable, alone or in combination with other data;
- whether the topic is sensitive, stigmatizing, or legally risky for them;
- whether collection and publication could expose or harm vulnerable groups;
- the gap between what users expected and what the research does.

Scraping a support forum for a stigmatized condition, then publishing identifiable quotes, can harm people who sought help in a semi-private space. Privacy analysis must consider the human subjects, not just the data.

### Handle Personally Identifiable Information Carefully

PII, direct or indirect, must be protected through de-identification and secure handling.

Handle by:

- minimizing collection of identifiers not needed for the research;
- de-identifying or pseudonymizing records where possible;
- recognizing the risk of re-identification through combination;
- secure storage, access controls, and a plan for retention and deletion.

Seemingly anonymous records can often be re-identified when combined. PII handling must anticipate linkage attacks and follow data minimization principles.

### Respect Intellectual Property And Platform Ownership

Platform content may be owned by users, the platform, or third parties. Redistribution has IP consequences.

Respect by:

- distinguishing analysis of content from redistribution of it;
- not republishing copyrighted text, images, or media beyond fair use;
- respecting database rights where they apply;
- citing and licensing appropriately when sharing derived data.

Collecting data for analysis may be defensible; republishing full copyrighted posts or media may not be. IP analysis must accompany any plan to share the dataset.

### Weigh Research Benefit Against Potential Harm

Ethical scraping requires a proportionality analysis: does the research benefit justify the risk?

Weigh by:

- the social or scientific value of the research;
- the likelihood and severity of harm to individuals or communities;
- whether less invasive methods could answer the question;
- mitigation measures such as aggregation, delay, or restricted access.

If the same question can be answered with less identifying or less invasive data, the more invasive scrape is hard to justify. Benefit and harm must be weighed explicitly, not assumed.

### Seek Ethical Review And Document The Analysis

Web scraping research with human-subjects implications should undergo ethical review, and the analysis must be documented.

Seek by:

- IRB or ethics board review where human data is involved;
- documenting the terms, privacy, IP, and harm analysis;
- a data management and sharing plan that protects subjects;
- transparency in publication about how the data were ethically handled.

A scrape presented without any ethical analysis hides the reasoning that should justify it. Documentation and review make the ethical reasoning auditable.

## Common Traps

### Accessibility Equated With Permission

Assuming that viewable data may be scraped and used ignores consent, terms, and user expectations.

### Convenient Terms Interpretation

Reading terms to permit what one wants, rather than what they say, is rationalized breach of contract.

### Ignoring Robots And Access Directives

Bypassing robots.txt or authentication to reach restricted data is ethically and often legally wrong.

### Ignored Human Privacy

Treating public data as free of privacy concerns harms identifiable or vulnerable individuals.

### Re-Identification Underestimated

Assuming de-identified records are safe ignores the risk of combination attacks.

### IP And Redistribution Overlooked

Republishing platform-owned or copyrighted content beyond fair use creates legal exposure.

### No Proportionality Or Review

Scraping without weighing benefit against harm or seeking ethical review skips the analysis that justifies the work.

## Self-Check

- Is the distinction between technical accessibility and authorized permission explicitly recognized?
- Are terms of service read honestly for what they say about automated access and redistribution?
- Are robots.txt and access directives respected rather than circumvented?
- Are the privacy expectations and potential harm to the people behind the data assessed?
- Is personally identifiable information minimized, de-identified, and securely handled against re-identification?
- Are intellectual property and platform ownership respected in collection and any sharing?
- Is research benefit weighed against potential harm, with less invasive alternatives considered?
- Is ethical review sought where human-subjects implications exist?
- Is the full ethical analysis, terms, privacy, IP, and harm, documented and auditable?
- Does the publication transparently report how the data were ethically handled?
