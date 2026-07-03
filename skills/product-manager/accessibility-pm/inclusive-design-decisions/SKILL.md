---
name: inclusive_design_decisions.md
description: Use when the agent is making inclusive design decisions, considering users beyond the average, designing for situational and permanent impairments, addressing language and cultural inclusion, or ensuring a product works across diverse abilities, devices, contexts, and circumstances.
---

# Inclusive Design Decisions

Inclusive design is broader than accessibility compliance. Where accessibility asks whether people with disabilities can use a product, inclusive design asks whether the product works for the full range of people who might use it, across abilities, languages, cultures, devices, network conditions, literacy levels, and situational circumstances. A product can be technically accessible and still exclude users through assumptions about language, about connectivity, about available time, about prior knowledge, or about the context in which the product is used. The failure mode is designing for a narrow default user and treating everyone else as an edge case.

The product manager's role in inclusive design is to widen the default. This means questioning the assumptions baked into requirements, recognizing that the average user is a fiction that hides real variation, and making decisions that serve more people without treating inclusion as a separate, deferrable workstream. Inclusive design is not about adding features for specific groups; it is about making the core product work for more people by design.

Use this skill before finalizing requirements that embed assumptions about users, before deciding what counts as the default experience, before scoping localization or device support, or when reviewing whether a product excludes users it is meant to serve. Ask: who is the implicit default user, and who does that default leave out? What assumptions about ability, language, device, connectivity, time, and knowledge are embedded in the design? Are inclusion considerations part of the core requirements, or treated as exceptions?

## Core Rules

### Question The Default User Assumption

Every product has an implicit default user, and that default shapes decisions invisibly. The default is often someone fluent in the language, on a modern device, with reliable broadband, with ample time, with domain knowledge, without disabilities, and in a stable environment. Real users deviate from this default in countless ways, and each deviation is a potential point of exclusion.

Make the default explicit and question it. Who are you assuming? What abilities, resources, and contexts does the default imply? Where does the default come from, the team's own characteristics, a specific customer, or genuine research? Once the default is visible, you can decide whether to keep it, broaden it, or design explicitly for variation. A default that goes unexamined excludes by accident; a default that is examined can be chosen deliberately.

### Design For Permanent, Temporary, And Situational Limitations

The same design barrier affects users with permanent disabilities, users with temporary limitations, and users in situational constraints. A user with a permanent arm injury, a parent holding a baby, and someone on a bumpy bus all benefit from an interface that does not require precise mouse control. A user who is deaf, a user in a loud environment, and a user with broken audio all benefit from captions. Designing for these shared needs serves far more people than designing only for the permanent case.

Use this overlap to build the case for inclusive design. Features that support users with disabilities frequently improve the experience for everyone, because the limitations are shared even when their causes differ. This reframes inclusion from a cost borne for a small group to a design quality that benefits the whole user base. Identify the shared limitations your product should address and design for them as core requirements.

### Address Language, Literacy, And Cultural Inclusion

Inclusion extends beyond ability to language, literacy, and culture. A product available only in one language excludes everyone who does not read it fluently. Content written at a high reading level excludes users with lower literacy, cognitive differences, or limited fluency in the product's language. Interactions that assume cultural familiarity, such as specific date formats, name structures, color meanings, or social conventions, exclude users from other contexts.

Consider the full range of users the product is meant to serve and where language, literacy, and culture create barriers. Localization is not just translation; it includes layout that adapts to text length variation, support for right-to-left languages, name and address formats that work across regions, and content that does not rely on culturally specific references. Decide deliberately which languages and regions the product supports, rather than defaulting to one and treating others as future work. Literacy considerations, such as plain language and clear structure, benefit users broadly and should be part of the content requirements.

### Design For Device And Connectivity Variation

The default assumption of a modern device on fast, reliable connectivity excludes large populations. Users on older or low-cost devices, on slow or intermittent networks, on metered data plans, or in regions with poor infrastructure experience a product very differently from the team testing on new hardware in an office. A design that works only under ideal conditions fails precisely the users with the fewest alternatives.

Build device and connectivity resilience into the requirements. This includes performance budgets that account for low-end devices, offline or degraded-mode behavior, progressive loading that delivers value before everything is ready, and data-conscious design that does not assume unlimited bandwidth. Decide which device and network conditions the product must support, test under those conditions rather than only ideal ones, and treat performance under constraint as a requirement, not an optimization.

### Consider Time, Attention, And Context Of Use

Users encounter products in varied contexts that affect their capacity to engage. A user with limited time needs to accomplish their goal quickly, without unnecessary steps. A user who is distracted or interrupted needs an experience that tolerates interruption and resumes gracefully. A user under stress, such as someone managing a health or financial crisis, needs clarity and support, not cognitive load. A user using the product in public needs privacy considerations that a private-use design ignores.

Map the realistic contexts of use and design for them. Where might users be interrupted, and does the product preserve their progress? Where might they be under time pressure, and can they complete the core task efficiently? Where might they be stressed, and does the design reduce rather than add cognitive load? These contextual considerations are inclusion issues, because they determine whether the product works for users whose circumstances differ from the calm, focused, private default.

### Make Inclusion Part Of The Core, Not A Separate Workstream

The most common structural failure is treating inclusion as a separate initiative, a phase, or a set of additional features, rather than as a quality of the core product. When inclusion is separate, it competes for priority and loses, because the core work always seems more urgent. The result is a product whose core experience excludes users, with inclusion work perpetually deferred.

Integrate inclusion into the core requirements, design process, and success criteria. Inclusive considerations appear in the spec, in design review, in user research recruitment, in testing conditions, and in release gates. The question is not whether to do inclusion work in addition to the core work, but whether the core work is inclusive. This shifts inclusion from a cost center to a quality standard that the core work must meet.

### Research With The Users You Risk Excluding

You cannot design inclusively for users you never engage with. Research that recruits only the default user confirms the default and misses the barriers others face. Inclusive design requires research with the full range of users the product should serve, including users with disabilities, users in different regions and languages, users on varied devices and networks, and users in realistic contexts.

Build diverse recruitment into the research plan. When you test only with users who resemble the team, you optimize for the team, not for the user base. When you test with the users most likely to be excluded, you find the barriers that matter most and you design solutions that benefit everyone. The cost of broader recruitment is small compared to the cost of shipping a product that excludes the users it was meant to serve.

## Common Traps

### The Average User Fiction

Designing for an average that hides real variation excludes everyone who deviates. The trap is treating anyone outside the default as an edge case rather than as part of the audience.

### Inclusion As Separate Workstream

Treating inclusion as additional work that competes with the core ensures it loses. The trap is a core product that excludes, with inclusion perpetually deferred.

### Ability-Only Framing

Addressing disability while ignoring language, literacy, device, connectivity, and context leaves many barriers intact. The trap is equating inclusion with accessibility compliance alone.

### Ideal-Condition Testing

Testing on new devices with fast networks hides the experience of users under constraint. The trap is shipping products that fail the users with the fewest alternatives.

### Defaulting To One Language And Culture

Assuming one language and cultural context and treating others as future work excludes most of the potential audience. The trap is a product that works for a narrow population and calls it done.

### Researching Only The Default User

Recruiting only users who resemble the team confirms the default and misses barriers. The trap is optimizing for the team rather than for the people the product should serve.

## Self-Check

- [ ] The implicit default user has been made explicit and questioned, and who it excludes is identified.
- [ ] Design addresses permanent, temporary, and situational limitations, recognizing the shared needs across these.
- [ ] Language, literacy, and cultural inclusion are part of the requirements, not deferred future work.
- [ ] Device and connectivity variation is accounted for, with performance and degraded-mode behavior as requirements.
- [ ] Realistic contexts of use, including time pressure, interruption, stress, and privacy, are considered in the design.
- [ ] Inclusion is integrated into core requirements, design review, research, testing, and release gates, not treated as a separate workstream.
- [ ] Research recruits the full range of users the product should serve, including those most likely to be excluded.
- [ ] Plain language and clear structure are content requirements, not optional polish.
- [ ] The product is tested under constrained conditions, not only ideal ones.
- [ ] Decisions about which users, languages, devices, and contexts are supported are made deliberately and documented, not left to default.
