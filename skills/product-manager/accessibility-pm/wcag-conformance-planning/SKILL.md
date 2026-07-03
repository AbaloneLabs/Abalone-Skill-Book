---
name: wcag_conformance_planning.md
description: Use when the agent is planning WCAG conformance, interpreting success criteria, deciding how to meet specific WCAG requirements, scoping accessibility audits, or determining what Level A, AA, or AAA conformance requires for a product surface.
---

# WCAG Conformance Planning

The Web Content Accessibility Guidelines are the most widely referenced accessibility standard, and they are also widely misunderstood. Teams treat WCAG as a binary, a product either passes or fails, when conformance is actually defined per page and per success criterion, with nuances around partial conformance, third-party content, and the difference between meeting a criterion's letter and actually serving users. Other teams cite WCAG without reading the success criteria, assuming that good intentions and automated scans amount to conformance. Both misunderstandings produce products that claim compliance they do not have, or that miss compliance they could achieve.

WCAG conformance planning is the work of translating the standard into concrete, scoped, verifiable work for a specific product. It requires reading the success criteria, understanding what each requires in practice, mapping them to the product's surfaces, identifying where conformance is hard and why, and producing an honest assessment of current state and the path to the target level. A product manager who plans conformance well can give the team a realistic, prioritized path; one who does not produces either false confidence or paralyzing scope.

Use this skill before claiming WCAG conformance, before scoping an accessibility audit, before interpreting what a success criterion requires, or before planning the work to reach a conformance level. Ask: have the relevant success criteria actually been read and understood, or is conformance assumed? Is the conformance claim scoped to specific pages and surfaces, or asserted globally? Is the gap between current state and target level known, or is it a guess?

## Core Rules

### Read And Understand The Success Criteria

WCAG conformance is defined by specific, numbered success criteria, each testable, each with official documentation explaining intent and techniques. Conformance planning that does not engage with the actual criteria is guesswork. Before claiming or planning conformance, read the success criteria at the target level, understand what each requires, and note where the requirement is ambiguous for your product.

The criteria are organized under four principles: content must be perceivable, operable, understandable, and robust. Within each, the criteria range from Level A (lowest) through AA to AAA (highest). Conformance at a level means meeting all criteria at that level and below. Do not assume you understand a criterion from its title; the detail matters. For example, the criterion on target size has specific minimum dimensions and exceptions that a title-level reading misses. Engage with the actual text, and where it is unclear, consult the official techniques and failures documentation.

### Scope Conformance To Specific Pages And States

WCAG conformance is evaluated per web page, not per product. A claim of conformance must specify which pages are covered, and conformance requires that the full page, in all its states, meets the criteria. This matters because products are dynamic: a page that conforms at load may fail after a user opens a modal, submits a form with an error, or expands a section. Conformance must account for all reachable states, not only the initial render.

Define the conformance scope explicitly. List the pages and flows covered. For each, identify the states that must be tested, including modals, errors, dynamic content, and authenticated versus unauthenticated views. A conformance claim that covers only the landing page's initial state is narrow and potentially misleading if presented as product-wide. Honest scoping makes clear what is covered, what is not, and what remains.

### Map Criteria To Concrete Product Decisions

Each success criterion translates into concrete decisions about design, content, and code. Color contrast requirements dictate the color system and text sizing. Keyboard operability dictates interaction design and focus management. Text alternatives dictate content workflows for images and media. Label associations dictate form construction. Error identification dictates validation design. Conformance planning connects each applicable criterion to the specific work it implies.

Produce a mapping that, for each criterion at the target level, describes what the product must do to conform and where the current design or implementation stands. This converts the abstract standard into a work list the team can execute. It also surfaces conflicts early, such as a design choice that cannot meet a contrast requirement without redesign, which is far cheaper to discover before implementation than after.

### Distinguish Conformance From Genuine Usability

WCAG conformance is necessary but not sufficient for genuine accessibility. A product can meet every success criterion and still be difficult or unpleasant for users of assistive technology, because the criteria capture a floor, not the full experience. Conversely, a product can serve users well in practice while technically failing a criterion that, in its specific context, does not cause real harm.

Plan for both. Pursue conformance as the baseline obligation, and pursue genuine usability as the higher goal. This means testing with real assistive technology and, where possible, with users who depend on it, not only checking criteria. Be honest about the distinction in communications: a conformance claim says the product meets the standard; it does not, by itself, say the product is a good experience for users with disabilities. Conflating the two overclaims and invites correction.

### Handle Third-Party And Legacy Content Honestly

Many products include content the team does not fully control: embedded widgets, third-party integrations, user-generated content, or legacy components. WCAG addresses this through partial conformance statements, which acknowledge that a page conforms except for specified third-party content. Misrepresenting these situations, either by claiming full conformance that the third-party content breaks, or by ignoring the content entirely, undermines the credibility of the conformance claim.

For each piece of content you do not fully control, assess its conformance, document the gap, and decide on the honest representation. Options include requiring the third party to conform as a condition of integration, providing an accessible alternative, or issuing a partial conformance statement that names the excepted content. The goal is an accurate claim, not a claim that looks clean by omitting inconvenient reality.

### Sequence Conformance Work By Risk And Impact

Achieving conformance across a large product is substantial work, and attempting everything at once stalls. Sequence by a combination of legal risk, user impact, and effort. High-traffic, high-stakes flows, such as checkout, account creation, and core task completion, carry the most risk and impact and should come first. Low-traffic internal tools or rarely visited pages can follow. Within a flow, fix the criteria that block users most severely before those that cause minor friction.

Produce a roadmap that reflects this sequencing, with clear milestones. This lets the team make incremental progress and lets stakeholders see a credible path, rather than facing an undifferentiated mass of accessibility work that never gets prioritized. Sequencing also lets you make honest interim claims about which surfaces conform, rather than waiting for a future total conformance that may be far off.

### Verify Conformance With Appropriate Methods

Conformance verification requires methods matched to what each criterion tests. Some criteria are machine-testable, such as whether an image has an alt attribute, and automated tools catch a subset of these. Many criteria require human judgment, such as whether a text alternative is meaningful or whether an error message is understandable. Some require testing with specific assistive technology, such as whether content is properly announced to a screen reader or operable with a keyboard alone.

Do not rely on automated scanning alone; it catches perhaps a third of issues and gives false confidence. Combine automated scanning for the machine-testable subset, manual expert review for the judgment-based criteria, and testing with assistive technology for the criteria that depend on real interaction. Document what was tested, how, and by whom, so the conformance claim is defensible and repeatable.

## Common Traps

### Citing WCAG Without Reading The Criteria

Assuming conformance from good intentions or automated scans, without engaging the actual success criteria. The trap is a conformance claim untethered from the standard's requirements.

### Global Conformance Claims

Asserting product-wide conformance when conformance is defined per page and state. The trap is a claim that hides non-conforming surfaces.

### Conformance Confused With Usability

Meeting criteria while delivering a poor experience for users of assistive technology. The trap is overclaiming that conformance equals genuine accessibility.

### Ignoring Third-Party Content

Claiming full conformance while embedded or legacy content breaks criteria. The trap is a clean claim maintained by omitting inconvenient reality.

### Automated-Scan False Confidence

Treating a clean automated scan as conformance. The trap is shipping products that pass scans but fail real users of assistive technology.

### Undifferentiated Roadmaps

Presenting accessibility as one large unsequenced effort. The trap is work that never gets prioritized because it has no credible path.

## Self-Check

- [ ] The success criteria at the target conformance level have been read and understood, not assumed from titles.
- [ ] The conformance claim is scoped to specific pages, flows, and states, including modals, errors, and dynamic content.
- [ ] Each applicable criterion is mapped to concrete design, content, and code decisions for the product.
- [ ] Conformance is pursued as a baseline, with genuine usability for assistive technology users as the higher goal, and the two are not conflated.
- [ ] Third-party, legacy, and user-generated content is assessed and represented honestly, with partial conformance statements where applicable.
- [ ] Conformance work is sequenced by legal risk, user impact, and effort, with a credible roadmap and milestones.
- [ ] Verification combines automated scanning, manual expert review, and testing with assistive technology, not scanning alone.
- [ ] The methods, scope, and results of verification are documented so the conformance claim is defensible.
- [ ] Interim claims reflect which surfaces actually conform, rather than asserting future total conformance.
- [ ] No conformance claim is made for surfaces or states that have not been tested.
