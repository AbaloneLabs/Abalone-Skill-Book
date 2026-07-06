---
name: library_website_ux_accessibility.md
description: Use when the agent is designing, redesigning, evaluating, or auditing the user experience and accessibility of a library website, planning navigation and information architecture, choosing components and page layouts, writing for the web, or reviewing a library site for usability, mobile responsiveness, cognitive load, plain language, and conformance with accessibility expectations such as WCAG and Section 508.
---

# Library Website UX And Accessibility

A library website is usually the most-used branch of the library. Most patrons will never walk through the physical doors, but nearly all will touch the website to search, renew, register, book a room, read a guide, or find hours. When the site is confusing, slow, hard to read, or inaccessible, the practical effect is service denial: a patron who cannot find the database list, cannot complete renewal on a phone, or cannot operate a form with a screen reader has been turned away. UX and accessibility work is therefore not decoration or polish applied at the end of a redesign; it is the core of equitable service delivery.

The judgment problem is that library websites are built and maintained by people who already know the library. Staff instinctively design for expert mental models, internal vocabulary, and desktop workflows. They underestimate how hard the site is for first-time users, mobile users, older adults, users with disabilities, users on slow connections, and users who do not read English fluently. Accessibility is frequently treated as a compliance checklist completed near launch rather than a continuous design constraint, which produces sites that technically pass an automated scan but still fail real people. This skill exists to push the agent to design from the outside in, to treat accessibility as a first-class requirement, and to verify usability with evidence rather than opinion.

Use this skill when designing or evaluating any part of a library website: homepage, navigation, search placement, service pages, forms, account portals, event calendars, research guides, or mobile experience. The goal is to prevent the agent from designing for staff mental models, hiding key tasks, treating accessibility as an afterthought, relying on automated tools alone, or shipping without testing with real users.

## Core Rules

### Start From Top Patron Tasks, Not Internal Structure

Patrons arrive with goals, not with an org chart. The site must surface the small set of tasks that most users actually perform.

Common top tasks for a library site:

- search the catalog and discovery layer;
- log into the account, renew, and pay fines;
- find hours, locations, and contact options;
- access research databases and guides;
- register for events or book study rooms;
- get help via chat, email, or appointment.

Implications for design:

- the homepage should expose these top tasks within the first viewport, not bury them under news, director messages, or hero images;
- navigation labels should use patron vocabulary, for example "Hours & Locations," not "Facilities Operations";
- avoid reflecting departmental structure in menus, which confuses users who do not know which department owns a service;
- validate the task list with search analytics, help-desk tickets, and brief user interviews rather than staff assumption.

A site organized around internal structure forces patrons to learn the library before they can use it. A site organized around tasks lets them act immediately.

### Design Navigation For Findability, Not Completeness

Navigation is the wayfinding system of the site. Its job is to get users to the right place quickly, not to list everything the library does.

Navigation principles:

- keep primary navigation to five to seven top-level items; more than that overwhelms working memory;
- use labels that are mutually exclusive and collectively exhaustive at the top level; overlap causes hesitation;
- provide multiple paths to the same destination because users think differently;
- support search as a primary navigation method, since many users search rather than browse;
- include a persistent utility menu for account, hours, and help;
- use descriptive link text, not "click here," so screen reader users and scanners understand destinations.

Test navigation with a tree test or first-click test: can a representative user reach a key destination from the homepage in a few clicks without backtracking? If not, the structure is wrong regardless of how logical it looks on paper.

### Make Accessibility A Design Constraint From The Start

Accessibility must be designed in, not bolted on. Retrofitting is expensive and produces inferior results.

Accessibility fundamentals:

- structure content with proper headings, lists, and landmarks so screen reader users can navigate semantically;
- ensure all interactive elements are keyboard operable with visible focus indicators;
- provide text alternatives for images that convey information, and use empty alt text for decorative images;
- label all form fields explicitly and associate labels with inputs;
- do not rely on color alone to convey meaning such as errors or required fields;
- ensure sufficient color contrast, generally at least 4.5:1 for normal text;
- make sure the page works at 200% zoom and reflows without horizontal scrolling;
- avoid content that auto-plays, flashes, or traps keyboard focus;
- caption videos and provide transcripts for audio.

Treat conformance with WCAG 2.1 or 2.2 Level AA as the baseline, not the ceiling. Remember that legal accessibility and usable accessibility are different: a page can pass a checklist and still be frustrating for a real assistive-technology user.

### Write For The Web Using Plain Language

Library sites are full of jargon that patrons do not understand: "OPAC," "ILL," "discovery layer," "bibliographic instruction," "proxy." Web writing must translate.

Plain language practices:

- use short sentences and common words;
- define unavoidable technical terms on first use;
- front-load the most important information;
- use descriptive headings and bullet lists for scanning;
- write link text that describes the destination, not the action verb;
- write instructions as clear steps, not paragraphs;
- consider readability targets around a grade 8 to 9 level for general public pages, while subject guides can be more advanced for their audience.

Plain language is an equity issue. Patrons with limited English proficiency, cognitive disabilities, or low literacy are excluded by dense bureaucratic prose.

### Design Forms That People Can Actually Complete

Forms are where patrons prove the site works or fail. Account login, event registration, room booking, and interlibrary loan requests are high-stakes form interactions.

Form design rules:

- ask only for information you actually need; every extra field reduces completion;
- group related fields and use clear section headings;
- show validation inline and clearly, with errors linked to the specific field;
- write error messages that explain how to fix the problem, not just that it failed;
- preserve user input when validation fails so they do not retype everything;
- allow autofill and use standard input types for mobile keyboards;
- make required fields clear without relying on color alone;
- provide a confirmation and a clear next step after submission.

A form that resets on error or gives a vague "something went wrong" message causes abandonment and drives support load.

### Optimize For Mobile And Slow Connections

A large share of patrons, especially in public libraries, access the site only on a phone, often on a slow or metered connection.

Mobile and performance considerations:

- design mobile-first, then enhance for larger screens;
- ensure tap targets are at least 44 by 44 pixels and spaced to avoid mis-taps;
- avoid fixed-width layouts that force horizontal scrolling;
- minimize page weight, large images, and render-blocking scripts;
- lazy-load images and defer non-critical scripts;
- test on real mid-range Android devices and slow network throttling, not just on a fast desktop.

Performance is accessibility: a page that takes ten seconds to load on a budget phone is effectively inaccessible to the user who has two minutes.

### Use Evidence, Not Opinion, To Guide Decisions

Library websites accumulate opinions from every stakeholder. Decisions should be grounded in evidence.

Evidence sources:

- web analytics showing entry pages, exit pages, search terms, and task flows;
- heatmaps and session recordings showing where users struggle;
- help-desk and chat transcripts revealing recurring confusion;
- usability tests with five to eight representative users per round;
- accessibility audits combining automated scans and manual testing;
- feedback forms and survey data.

When stakeholders disagree, run a quick test. Evidence resolves arguments that opinions perpetuate.

### Maintain Continuously, Not Just At Redesign

A website is never done. Treating it as a project with a launch date guarantees decay.

Continuous maintenance:

- assign clear ownership for content and sections;
- schedule regular content audits for accuracy, broken links, and outdated information;
- review analytics monthly and act on findings;
- retest accessibility after major changes;
- keep a backlog of UX improvements informed by evidence;
- document design patterns and standards so additions stay consistent.

The most common failure mode is launch-and-neglect, where a beautiful new site slowly rots until the next expensive redesign.

## Common Traps

### Designing For Staff Mental Models

Staff know the library, so they build navigation and labels that make sense internally. This excludes patrons who do not share that knowledge. Validate with real users and use patron vocabulary.

### Treating Accessibility As A Launch Checklist

Running an automated scan at the end catches some issues but misses keyboard traps, screen reader problems, and cognitive barriers. Accessibility must be designed in and tested manually with assistive technology.

### Relying On Automated Accessibility Tools Alone

Automated tools detect roughly a third to forty percent of issues. They miss context-dependent problems like heading order, link purpose, and reading order. Always supplement with manual and user testing.

### Hiding Key Tasks Behind Hero Images And News

Homepages dominated by rotating banners, news carousels, and promotional content push real tasks below the fold. Carousels are also an accessibility and usability problem. Prioritize tasks over promotion.

### Jargon And Acronym-Heavy Writing

Terms like "OPAC," "discovery," "proxy," and "ILL" are meaningless to most patrons. Translate to plain language or define on first use.

### Desktop-Only Design And Testing

Testing only on a large monitor hides mobile failures. A large share of patrons use phones exclusively. Test on real mobile devices and slow networks.

### Forms That Reset Or Give Vague Errors

A form that clears on error or says "submission failed" causes abandonment. Preserve input and give field-specific, actionable error messages.

### Opinion-Driven Design Without Evidence

Stakeholder opinions are not data. When teams argue over layout, run a usability test or check analytics. Let evidence decide.

### Launch-And-Neglect

A site that is not maintained decays. Assign ownership, audit content regularly, and treat the site as a living service.

## Self-Check

- Does the homepage expose the top patron tasks, such as search, account, hours, databases, and help, within the first viewport?
- Is navigation limited to a small number of clearly labeled, non-overlapping top-level items using patron vocabulary?
- Are all interactive elements keyboard operable with visible focus, proper headings, labeled forms, and text alternatives for informative images?
- Has the site been tested manually with screen readers and keyboard-only navigation, not just scanned automatically?
- Is body copy written in plain language, with jargon defined and link text describing destinations?
- Do forms ask only for necessary information, validate inline with field-specific errors, and preserve user input on failure?
- Does the site work on mid-range mobile devices and slow connections, with adequate tap targets and no horizontal scrolling?
- Are decisions grounded in analytics, usability tests, and help-desk evidence rather than stakeholder opinion?
- Is there clear ownership and a maintenance plan so the site does not decay after launch?
- Has accessibility been verified against a recognized standard such as WCAG 2.1 or 2.2 AA, with manual testing supplementing automated scans?
