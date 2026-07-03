---
name: accessibility_and_inclusive_design.md
description: Use when the agent is evaluating a product or feature for accessibility, defining accessibility requirements, ensuring inclusive design for users with different abilities and contexts, or deciding accessibility scope and compliance level for a release.
---

# Accessibility And Inclusive Design

Accessibility is treated as a finishing touch in too many product teams, something to check before launch if there is time. This is both ethically wrong and practically costly. Accessibility is a product requirement that determines whether real users can use the product at all, and deferring it creates debt that compounds until a redesign, a lawsuit, or a lost enterprise contract forces an expensive remediation.

The deeper frame is inclusive design. Accessibility is often reduced to screen readers and wheelchair users, but the range of human limitation is far wider and most of it is temporary or situational. A parent holding a baby cannot use a second hand. A commuter on a noisy train cannot hear audio. A user on a slow connection cannot load heavy media. A non-native speaker cannot parse dense jargon. Inclusive design asks who is excluded by each decision and treats that exclusion as a product defect, not an edge case.

Use this skill before defining requirements for a feature, setting acceptance criteria, deciding accessibility scope for a release, reviewing a design, or triaging accessibility debt. Ask: who cannot use this if we ship it as designed? What level of accessibility compliance is required and why? Have we tested with assistive technology and real users with limitations, or only assumed it works? Are we accepting accessibility debt consciously with a repayment plan, or letting it accumulate silently?

## Core Rules

### Treat Accessibility As A Product Requirement

Accessibility belongs in requirements and acceptance criteria from the start, not in a QA checklist tacked on at the end. When it is a launch-day afterthought, it is always late, always under-resourced, and usually shipped incomplete. The PM owns making accessibility explicit in the spec, just as they own permissions, data, and rollout.

Define accessibility expectations alongside other requirements. State the compliance level the feature must meet, the assistive technologies that must be supported, and the specific behaviors that must work, such as full keyboard operability, screen reader announcements, and visible focus indicators. If accessibility is scoped down for a release, make that a conscious, documented decision with a repayment plan, not a silent omission.

### Understand The Full Range Of Limitations

Inclusive design looks beyond permanent disabilities to the full spectrum of human limitation. People experience permanent, temporary, and situational limitations, and good design serves all three because the accommodations often overlap.

Consider limitations across these dimensions:

- vision, including blindness, low vision, and color blindness;
- hearing, including deafness and hard of hearing;
- motor, including limited dexterity, injury, and one-handed use;
- cognitive, including attention, memory, reading, and learning differences;
- situational, including bright glare, noisy environments, and slow connectivity;
- linguistic, including non-native speakers and varying literacy;
- device, including older hardware and constrained screens.

Designing for these is not a separate effort for each group. Captions help deaf users and people in noisy rooms. Large touch targets help motor-impaired users and people on bumpy trains. Plain language helps cognitive differences and non-native speakers alike.

### Choose And Understand A Compliance Level

WCAG provides a shared vocabulary and measurable criteria, and choosing a target level forces an honest conversation about scope. Level A is the minimum, Level AA is the common legal and contractual baseline for many products, and Level AAA is the most stringent and not always achievable for all content.

The PM should know what level the product is expected to meet and why. That expectation may come from law, enterprise contracts, organizational policy, or a product principle. Translate the chosen level into concrete requirements the team can build and test against, rather than leaving "make it accessible" as an aspiration. Know that legal compliance and genuine usability are related but not identical; meeting a checklist does not guarantee a smooth experience for real users.

### Write Accessibility Acceptance Criteria

Accessibility must be verifiable, which means it needs observable acceptance criteria like any other requirement. Vague goals like "be accessible" cannot be tested or confirmed.

Useful criteria name specific, checkable behavior:

- all functionality is operable by keyboard alone with no mouse traps;
- focus order and visible focus indicators are logical and visible;
- images and icons have appropriate text alternatives;
- form fields have associated labels and error messages are announced;
- color is not the only means of conveying information;
- text meets the required contrast ratio against its background;
- content reflows and remains usable at 200 percent zoom;
- dynamic content changes are announced to assistive technology.

These criteria let design, engineering, and QA share one definition of done.

### Test With Assistive Technology And Real Users

Automated tools catch a fraction of accessibility issues, perhaps a third, and they miss the problems that matter most. A passing automated scan does not mean the product is accessible. Real validation requires manual testing with assistive technology and, ideally, testing with users who actually rely on it.

Test with screen readers such as VoiceOver, TalkBack, and NVDA. Test keyboard-only navigation. Test at high zoom and with high-contrast settings. Where possible, recruit users with disabilities for usability research, because their experience reveals barriers that checklist testing misses. Treat their feedback as authoritative, not as optional input.

### Weigh Speed Against Exclusion Honestly

There is genuine tension between shipping fast and shipping accessibly. Acknowledging it honestly is better than pretending it does not exist. The question is not whether to ever ship something incomplete, but whether the exclusion is conscious, bounded, and repaid.

When accepting accessibility debt, define the terms. What specifically is deferred? What is the user impact? When and how will it be repaid? Who is accountable? Unbounded, undocumented debt becomes permanent exclusion dressed up as a temporary shortcut. The cost of remediation after launch is almost always higher than building it right initially, because retrofitting accessibility into a finished codebase means reworking components, re-testing, and often redesigning.

### Extend Inclusion Beyond Disability

Inclusive design does not stop at disability. Language, device, bandwidth, literacy, and context all determine who can participate. A product that works only on the newest phone, only in English, only on fast broadband, or only for highly literate users is excluding people even if it passes every accessibility audit.

Ask who the product assumes. Does it assume a fast, stable connection? A large screen? Fluency in one language? Comfort with dense interfaces? Each assumption is a potential exclusion. Expanding the circle of who can use the product is a product decision with real reach and revenue implications, not merely a kindness.

## Common Traps

### Treating Accessibility As Engineering's Problem

When accessibility is handed to engineering as a final QA pass, the design and requirements that caused the barriers are already locked in. The trap is believing a late audit can fix what was broken at the spec and design stage. Accessibility is a product and design responsibility first.

### Trusting Automated Scans As Proof

Automated tools are useful for catching obvious issues, but they miss most real-world barriers, especially in dynamic interfaces. The trap is shipping with confidence because the scan passed, then discovering screen reader users cannot complete the core task. Automated checks are a starting point, not a certificate.

### Defining Scope As "As Accessible As Possible"

"As accessible as possible" is not a scope; it is an escape clause. It gives no one anything to build or test against and guarantees inconsistency across the product. The trap is using vagueness to avoid a hard conversation about what level of compliance the release commits to.

### Letting Accessibility Debt Accumulate Silently

Every deferred accessibility issue is a user who cannot use the product. When debt is not tracked, it compounds invisibly until a forcing event, a complaint, a lost deal, or legal action, makes it urgent and expensive. The trap is treating deferral as free when it is actually a loan with high interest.

### Designing Only For The Permanent Case

Focusing only on permanent disabilities misses the larger population affected by temporary and situational limitations. The trap is narrowing inclusion to a small group when the same design choices serve a much wider audience. Inclusive design is leveraged, not charitable.

### Confusing Compliance With Usability

Meeting WCAG criteria does not guarantee a product is pleasant or efficient to use with assistive technology. The trap is checking boxes and declaring victory while real users still struggle. Compliance is the floor; usability with the people who depend on it is the actual goal.

### Ignoring Contextual Exclusion

Exclusion is not always about ability. Assuming fast bandwidth, new devices, or a single language quietly locks out users who never appear in the accessibility conversation. The trap is equating inclusion solely with disability and missing the broader populations the product could serve.

## Self-Check

- [ ] Accessibility is written into requirements and acceptance criteria from the start, not deferred to a final QA pass.
- [ ] A specific compliance level, such as WCAG 2.1 AA, is chosen and translated into concrete, testable criteria.
- [ ] The design considers permanent, temporary, and situational limitations across vision, hearing, motor, and cognitive dimensions.
- [ ] Acceptance criteria cover keyboard operability, focus, labels, alternatives, contrast, zoom, and dynamic content announcements.
- [ ] Testing includes manual checks with real assistive technology, not only automated scans.
- [ ] Where possible, users who rely on assistive technology were included in research or validation.
- [ ] Any deferred accessibility work is documented with user impact, repayment plan, and owner, not left as silent debt.
- [ ] The product's assumptions about language, device, bandwidth, and literacy were examined for who they exclude.
- [ ] Color is not the sole means of conveying information, and contrast meets the required threshold.
- [ ] Compliance claims are backed by real usability validation, not checklist completion alone.
