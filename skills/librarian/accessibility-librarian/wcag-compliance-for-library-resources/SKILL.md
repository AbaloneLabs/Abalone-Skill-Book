---
name: wcag_compliance_for_library_resources.md
description: Use when the agent is evaluating, remediating, or building library digital resources for accessibility conformance against WCAG 2.1 or 2.2 Levels A and AA, auditing websites, discovery layers, catalogs, LibGuides, digital collections, PDFs, forms, and multimedia, interpreting success criteria, combining automated and manual testing with assistive technology, producing accessibility conformance reports or VPATs, or planning remediation of library web content.
---

# WCAG Compliance For Library Resources

Accessibility conformance is the difference between a library digital resource that serves all patrons and one that silently excludes people with disabilities. WCAG, the Web Content Accessibility Guidelines, is the recognized standard for measuring that conformance, and in many jurisdictions meeting it is a legal obligation under laws such as the Americans with Disabilities Act, Section 508, and equivalent statutes elsewhere. But legal compliance is the floor, not the ceiling. The deeper purpose is equity: a patron who uses a screen reader, who navigates by keyboard, who needs captions, who relies on high contrast, or who needs enlarged, reflowed text has the same right to the catalog, the databases, the research guides, and the digital collections as any other patron. When those resources fail WCAG, the practical effect is denial of service.

The judgment problem is that accessibility is frequently treated as a one-time scan or a launch checklist, and both approaches fail. Automated tools catch only a fraction of issues, roughly a third to forty percent, because many success criteria depend on context and human judgment: is the heading order logical, does the link text describe its destination, is the reading order correct, does the video have accurate captions. A page can pass an automated scan and still be unusable with a screen reader. Libraries also face a compounded problem: their digital footprint includes not just their own website but vendor platforms, licensed databases, third-party widgets, and legacy PDFs, much of which they do not control directly. The agent's job is to test thoroughly with both tools and assistive technology, interpret WCAG criteria accurately, document conformance honestly, and drive remediation, while escalating when legal risk or unresolved vendor barriers are involved.

Use this skill when evaluating, remediating, or building any library digital resource for accessibility, auditing conformance, interpreting WCAG criteria, producing conformance reports, or planning remediation. The goal is to prevent the agent from relying on automated tools alone, conflating compliance with usability, accepting vendor claims without verification, or treating accessibility as a checkbox rather than an ongoing equity practice.

## Core Rules

### Understand WCAG As A Standard, Not A Suggestion

WCAG is organized around four principles, Perceivable, Operable, Understandable, Robust, known by the acronym POUR, and structured into success criteria at Levels A, AA, and AAA.

Foundational understanding:

- Level A is the minimum; Level AA is the common legal and policy target; AAA is the highest, not always achievable for all content;
- success criteria are testable statements, not vague aspirations;
- conformance is assessed per page or per set of pages, not as a vague overall impression;
- "substantially conforms" is not conformance; either a page meets the criteria or it does not;
- WCAG versions evolve; clarify which version and level a requirement targets, commonly 2.1 or 2.2 AA.

Interpreting WCAG accurately is the foundation of credible conformance work. Know the principles, the levels, and the specific criteria.

### Test With Both Automated Tools And Manual Methods

No single method catches everything. Combine automated scanning, manual review, and assistive technology testing.

Testing methods:

- automated tools, such as axe, WAVE, or Lighthouse, catch clear defects like missing alt text or contrast failures;
- manual review catches context-dependent issues like heading order, link purpose, and reading order;
- keyboard-only testing reveals operability barriers;
- screen reader testing, with products such as JAWS, NVDA, or VoiceOver, reveals real-world usability;
- testing on mobile and with zoom and reflow reveals responsive barriers.

Automated tools are a starting point, never the whole assessment. A page that scans clean can still fail real users.

### Address The Most Common And High-Impact Failures

Some WCAG failures appear repeatedly across library resources and have outsized impact. Prioritize them.

Common high-impact failures:

- missing or unhelpful alt text on informative images;
- improper heading structure, used for styling rather than hierarchy;
- non-descriptive link text such as "click here" or "read more";
- form fields without proper labels and associated instructions;
- keyboard inaccessibility, including traps and missing focus indicators;
- insufficient color contrast;
- content that does not reflow or scale at 200% zoom;
- multimedia without captions or transcripts;
- dynamic content and custom widgets without proper ARIA roles and states.

Fixing these recurring issues removes the majority of real-world barriers quickly.

### Treat Perceivable, Operable, Understandable, Robust As A Checklist

Use the POUR principles as an organizing framework for assessment and remediation.

POUR application:

- Perceivable: content is presentable in ways users can perceive, including text alternatives, captions, contrast, and adaptability;
- Operable: interface components are operable, including keyboard access, no traps, enough time, and no seizure-inducing content;
- Understandable: content and operation are understandable, including readable language, predictable behavior, and input assistance;
- Robust: content works with assistive technologies, including valid markup and compatibility.

Assessing against POUR ensures coverage of the full standard, not just the easy parts.

### Do Not Conflate Compliance With Usability

A resource can meet WCAG criteria and still be difficult or frustrating for users with disabilities. Compliance is necessary but not sufficient.

Beyond compliance:

- test with real users who have disabilities where possible;
- consider cognitive load, reading level, and consistency;
- ensure the experience is not just possible but reasonable;
- gather feedback from patrons who use assistive technology;
- treat complaints and difficulty reports as priority signals.

The goal is usable access, not a passing score. Usability testing with real users reveals what checklists miss.

### Hold Vendors And Third-Party Platforms Accountable

Much library content lives on vendor platforms, databases, discovery layers, and embedded widgets that the library does not directly control. Accessibility must be a procurement and contract requirement.

Vendor accountability:

- request and review VPATs or Accessibility Conformance Reports;
- test vendor platforms independently rather than trusting claims;
- include accessibility as a contract requirement with remediation timelines;
- track known vendor accessibility issues and workarounds;
- escalate persistent vendor failures to procurement and leadership;
- prefer vendors with demonstrated accessibility commitment.

Vendor-hosted does not mean vendor-absolved. The library remains responsible for the access it provides.

### Document Conformance Honestly

Accessibility statements and conformance reports must reflect reality, not aspiration.

Honest documentation:

- state which WCAG version and level is targeted;
- report actual conformance based on testing, not intent;
- disclose known gaps and provide alternatives or timelines;
- provide contact information for accessibility issues;
- avoid claiming full conformance that testing does not support;
- update documentation as remediation progresses.

Overstating conformance creates legal risk and erodes trust. Honest reporting, including known limitations, is more defensible and more useful to patrons.

### Remediate Strategically And Continuously

Accessibility is rarely fixed in one project. Prioritize and sustain remediation.

Remediation strategy:

- prioritize high-traffic and high-stakes pages, such as the homepage, catalog, account, and key services;
- fix recurring issues across templates to gain leverage;
- build accessibility into the content workflow so new content conforms by default;
- maintain a backlog and track remediation progress;
- retest after changes, since regressions are common;
- allocate ongoing resources, not just project budgets.

Accessibility debt accumulates like technical debt. Continuous attention prevents recurrence.

### Build Accessibility Into The Content Lifecycle

The cheapest accessibility fix is the one never needed because content was created accessibly.

Lifecycle integration:

- train content authors on accessible practices: headings, alt text, link text, tables;
- provide templates and checklists that enforce accessibility;
- review accessibility as part of publishing workflow;
- include accessibility in style guides and governance;
- audit periodically to catch drift.

Accessibility built into daily content work prevents the cycle of create-then-remediate.

### Plan For Ongoing Monitoring

Accessibility degrades as content and platforms change. Monitor continuously.

Monitoring:

- run automated scans on a schedule across the site;
- track and trend issues over time;
- review analytics for pages with high abandonment that may signal barriers;
- respond to patron-reported issues promptly;
- retest critical workflows after major changes or upgrades.

Accessibility is an ongoing service, not a completed project.

## Escalation Guidance

Accessibility failures can carry legal risk and equity consequences. Escalate in these situations:

- Any formal accessibility complaint, legal demand, or OCR complaint: escalate immediately to leadership, legal counsel, and the disability services office; document and respond per policy.
- Any vendor platform with persistent, unresolved accessibility barriers affecting core services: escalate to procurement and leadership to renegotiate, require remediation, or replace.
- Any resource that is legally required to be accessible, such as course reserves or required readings, and is not: escalate for immediate remediation or provision of an accessible alternative.
- Any claim of full conformance that testing does not support: escalate to correct the claim before it creates liability.
- Any situation where a patron is denied service due to inaccessible content: escalate to provide an immediate accommodation and fix the underlying barrier.

When legal risk or service denial is involved, escalate promptly and document the response.

## Common Traps

### Relying On Automated Tools Alone

Automated tools miss the majority of real barriers. Always supplement with manual and assistive technology testing.

### Conflating Compliance With Usability

Meeting criteria does not guarantee a usable experience. Test with real users and respond to difficulty reports.

### Accepting Vendor Claims Without Verification

VPATs and marketing claims often overstate conformance. Test vendor platforms independently.

### Overstating Conformance

Claiming full conformance without testing creates legal risk and erodes trust. Document honestly, including gaps.

### One-Time Project Mentality

Accessibility degrades without ongoing attention. Monitor continuously and remediate as part of routine work.

### Ignoring Third-Party And Embedded Content

Widgets, databases, and vendor platforms carry barriers the library must still address. Include them in scope.

### Styling Over Semantics

Using headings, lists, and color for visual effect rather than meaning breaks navigation for assistive technology. Use semantic structure.

### No Patron Feedback Loop

Patrons who encounter barriers are the best source of real issues. Provide and respond to feedback channels.

## Self-Check

- Is the target WCAG version and level, commonly 2.1 or 2.2 AA, clearly defined for each resource?
- Does testing combine automated scanning, manual review, keyboard testing, and screen reader testing, not tools alone?
- Are the most common and high-impact failures, including alt text, headings, link text, labels, keyboard access, contrast, and captions, addressed?
- Is assessment organized around the POUR principles to ensure full coverage of the standard?
- Is the resource tested for usability with real users and assistive technology, not just checklist compliance?
- Are vendor and third-party platforms tested independently, with accessibility required in contracts and tracked for remediation?
- Does the accessibility statement or conformance report reflect actual testing, disclose known gaps, and avoid overstated claims?
- Is remediation prioritized by traffic and stakes, built into content workflows, and sustained over time?
- Is accessibility integrated into the content lifecycle through training, templates, and publishing review?
- Is there ongoing monitoring, with escalation paths for complaints, legal risk, vendor failures, and service denial?
