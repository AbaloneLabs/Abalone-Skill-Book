---
name: accessibility_testing.md
description: Use when the agent is planning or conducting accessibility testing, choosing between automated scans and manual testing, testing with assistive technologies like screen readers and voice control, running keyboard-only testing, validating WCAG conformance, recruiting users with disabilities, or deciding when automated tooling is insufficient and real user testing is required.
---

# Accessibility Testing

Accessibility testing is the practice of verifying that a product can be used by people with disabilities, including those who rely on assistive technologies. It looks like running an automated scan, but it is really a layered evaluation where automated tools catch a fraction of issues, manual testing catches more, and only testing with real users reveals the rest. Agents tend to treat accessibility testing as a scan-and-fix exercise, trust automated tools to certify conformance, or test with assistive technologies they do not understand. The harm is false confidence that a product is accessible when it blocks real users, and legal as well as ethical exposure when the gaps surface.

Use this skill before planning accessibility testing or interpreting its results. The goal is to prevent the agent from equating a clean automated scan with accessibility, from testing assistive technology incorrectly, or from skipping the user testing that only reveals real barriers.

## Core Rules

### Understand That Automated Tools Catch Only A Fraction

Automated accessibility tools are valuable but limited. They reliably detect certain issues (missing alt text, invalid HTML, some contrast failures) but cannot detect most real barriers: whether a screen reader announces content in a sensible order, whether keyboard navigation is logical, whether focus management works in dynamic interfaces, or whether the experience is usable under real conditions. A clean scan does not mean the product is accessible.

Use automation for what it does well:

- run automated scans to catch the mechanical issues they reliably detect;
- treat a clean scan as a starting point, not a certification;
- know that automated tools catch roughly thirty to forty percent of accessibility issues, leaving the majority to manual and user testing;
- never present an automated scan as proof of accessibility to stakeholders.

The most damaging accessibility failures (screen reader traps, keyboard dead ends, focus loss) are invisible to automated tools. Trusting a scan alone certifies problems the scan cannot see.

### Test With The Keyboard Alone

Keyboard-only testing is one of the highest-value manual checks, because keyboard accessibility underlies screen reader, switch, and voice control access. If a user cannot operate the product with a keyboard, multiple assistive technologies fail. Yet keyboard testing is frequently skipped because it is not automated.

Conduct thorough keyboard testing:

- unplug the mouse and attempt every task using only tab, shift-tab, enter, space, arrows, and escape;
- verify the tab order is logical and follows the visual order;
- check that every interactive element is reachable and operable;
- ensure focus is always visible and never trapped in a component;
- verify focus moves sensibly in dynamic interfaces (modals, menus, single-page navigation).

Keyboard failures are common and severe. A product that traps focus or skips controls via keyboard is broken for many users, regardless of how it looks.

### Test With Real Assistive Technologies

Assistive technology testing reveals how the product actually behaves for users of screen readers, screen magnifiers, voice control, and switch devices. Testing with these tools requires understanding how they are used; testing incorrectly produces misleading results. A sighted tester tabbing through with a screen reader running but not understanding its commands learns little.

Test with assistive technology competently:

- test with the most common screen readers (NVDA, JAWS, VoiceOver) across platforms your users use;
- learn the basic commands and navigation patterns of each tool, or partner with someone who uses them daily;
- verify content is announced in a logical order, landmarks and headings are navigable, and dynamic changes are announced;
- test screen magnification, voice control, and switch access where relevant to your users;
- recognize that emulators and testing modes differ from real assistive technology use.

Incompetent assistive technology testing produces false confidence. If no one on the team is proficient, bring in someone who is, including users who rely on these tools daily.

### Validate Against WCAG, But Understand Conformance Limits

WCAG (Web Content Accessibility Guidelines) is the recognized standard, and conformance is often a legal requirement. But WCAG conformance is necessary, not sufficient: a product can meet every criterion and still be difficult or unpleasant for disabled users. Validate against WCAG as a baseline, then test beyond it for real usability.

Use WCAG appropriately:

- map findings to specific WCAG success criteria at the target conformance level (A, AA, AAA);
- understand that WCAG covers many but not all accessibility considerations;
- recognize that novel interfaces (canvas, drag-and-drop, VR) may fall outside what WCAG addresses clearly;
- treat WCAG conformance as a floor, with user testing as the path to genuine usability.

Stating "we are WCAG AA compliant" based on a scan, without manual and user testing, is a misrepresentation. Conformance claims must reflect actual evaluation across all relevant criteria.

### Recruit And Test With Users Who Have Disabilities

The most important and most skipped form of accessibility testing is testing with real users who have disabilities. No amount of automated or expert testing reveals the actual experience of a user navigating with a screen reader they have used for years, or operating with a switch under constraints no simulator captures. Users reveal problems that every other method misses.

Test with disabled users:

- recruit participants who actually use assistive technologies, not simulated or impersonated disability;
- compensate participants fairly and treat them as experts in their own access needs;
- test the real tasks users need to accomplish, not just that elements are technically operable;
- listen to participants' strategies and workarounds, which reveal where the design fails them.

Testing only with automated tools and expert review, never with disabled users, produces a product that may pass checks but fail real people. User testing is not optional for genuine accessibility.

### Test Across The Full Range Of Disability Types

Accessibility is often reduced to blindness and screen readers, but disability is broad: low vision, color blindness, deafness and hard of hearing, motor and dexterity impairments, cognitive and learning disabilities, and seizure sensitivities. Testing only one type leaves other barriers in place.

Cover the disability spectrum:

- test for visual access: blindness, low vision, color blindness, and screen magnification;
- test for auditory access: captions, transcripts, and visual alternatives to audio;
- test for motor access: keyboard, switch, voice control, and tolerance for small targets;
- test for cognitive access: clear language, consistent navigation, error prevention, and reduced cognitive load;
- test for seizure safety: no content that flashes more than three times per second.

A product accessible to screen reader users but inaccessible to keyboard-only users, or to those with cognitive disabilities, is not accessible. Coverage must be broad.

### Integrate Accessibility Testing Throughout, Not At The End

Accessibility treated as a final pre-launch check is too late to fix most issues cheaply. By the time a product is built, structural problems (semantic structure, focus management, dynamic announcements) are expensive to retrofit. Accessibility must be tested throughout design and development.

Test continuously:

- run automated checks in development, integrated into CI, so regressions are caught immediately;
- conduct manual keyboard and assistive technology testing on prototypes and components, not only finished products;
- include accessibility criteria in design and code review;
- track accessibility issues with the same rigor as other defects, not as nice-to-haves.

Late-stage accessibility testing finds problems that are by then expensive or impossible to fix within timeline. Early and continuous testing prevents the buildup of barriers.

### Document And Track Accessibility Issues Rigorously

Accessibility issues are often deprioritized because they are invisible to the majority of the team. Without rigorous documentation, tracking, and advocacy, they slip through release after release. Treating accessibility issues with the same defect-tracking discipline as functional bugs is what ensures they get fixed.

Document and track:

- log every accessibility issue with its WCAG criterion, reproduction steps, and user impact;
- assign severity based on the barrier's effect on users, not just frequency;
- track issues to closure with the same process as other bugs;
- report accessibility status in release readiness, so it is visible in go or no-go decisions.

Accessibility issues that are not tracked are not fixed. Rigor in documentation and tracking is what converts findings into resolved barriers.

## Common Traps

### Equating A Clean Scan With Accessibility

Automated tools catch only a fraction of issues; a clean scan is a starting point, not a certification of accessibility.

### Skipping Keyboard-Only Testing

Keyboard access underlies screen reader, switch, and voice control use; unplug the mouse and test every task.

### Incompetent Assistive Technology Testing

Running a screen reader without understanding its commands produces misleading results; test competently or partner with proficient users.

### WCAG Conformance Claims Without Full Evaluation

Claiming conformance based on a scan, without manual and user testing, misrepresents the product's accessibility.

### Never Testing With Disabled Users

No automated or expert method reveals the real experience of users with disabilities; recruit and test with them.

### Reducing Accessibility To Screen Readers

Disability is broad; test for visual, auditory, motor, cognitive, and seizure-safety access, not only blindness.

### Accessibility As A Final Pre-Launch Check

Late testing finds structural problems too expensive to fix; integrate accessibility testing throughout design and development.

### Untracked Accessibility Issues

Issues that are not logged and tracked with defect rigor slip through release after release; document, assign severity, and track to closure.

## Self-Check

- [ ] Automated scans are used for mechanical issues only, and a clean scan is never presented as proof of accessibility.
- [ ] Keyboard-only testing was conducted for every task, verifying logical tab order, visible focus, no traps, and sensible dynamic focus management.
- [ ] Assistive technology testing used common screen readers competently (or with proficient partners), verifying announcement order, landmarks, and dynamic changes.
- [ ] Findings are mapped to specific WCAG success criteria at the target conformance level, recognized as a floor not a ceiling.
- [ ] Testing included real users with disabilities, recruited fairly and treated as experts in their access needs.
- [ ] Testing covered the disability spectrum: visual, auditory, motor, cognitive, and seizure safety.
- [ ] Accessibility testing is integrated throughout design and development (CI scans, prototype checks, review criteria), not only at the end.
- [ ] Accessibility issues are logged with WCAG criteria, reproduction steps, user-impact severity, and tracked to closure like other defects.
