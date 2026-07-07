---
name: designing_for_diverse_users.md
description: Use when the agent is designing or reviewing an interface for a broad range of users, accounting for varying abilities, ages, devices, environments, literacy levels, cultural backgrounds, and temporary or situational limitations, or when deciding who counts as a valid user and whose needs to prioritize.
---

# Designing For Diverse Users

Designing for diverse users is not adding extra modes after a "normal" user is served. The assumed default user is itself a design decision, and one of the most consequential ones. When a product is built around a narrow archetype, every real user who differs from that archetype encounters friction that was never anticipated, and that friction is invisible to the team because the archetype was never named.

Use this skill before scoping personas, defining the "primary user," setting device or environment assumptions, choosing input methods, deciding which edge cases to defer, or reviewing whether a design serves the people it claims to serve. The goal is to prevent the agent from designing for a convenient average, treating accessibility as a separate audience, or shipping a design that works only under ideal conditions for ideal users.

## Core Rules

### Name The Assumed User, Then Challenge It

Every design carries an implicit archetype: the device they hold, the speed of their connection, the steadiness of their hands, their language, their reading level, their environment, their time pressure. Make that archetype explicit before designing, then ask who it excludes.

Ask whether the assumed user:

- always has two free hands and a stable surface;
- reads the product's primary language fluently;
- has typical vision, color perception, hearing, and motor control;
- is indoors, seated, calm, and undistracted;
- uses a current flagship device on a fast network;
- has used similar products before and knows the conventions.

When the archetype is named, the gaps become design problems instead of post-launch surprises. Designing for the people the archetype leaves out often improves the product for everyone.

### Distinguish Permanent, Temporary, And Situational Limitations

Disability is one category of human variation, not a separate population. The same interface barrier affects a permanent arm injury, a temporary wrist sprain, and a parent holding a sleeping baby. Treating these as the same design problem leads to better solutions than carving out a "disability mode."

Map limitations across three axes:

- permanent: low vision, color blindness, deafness, motor impairment, cognitive difference;
- temporary: injury, fatigue, illness, medication, eye strain;
- situational: bright glare, one-handed use, noisy room, gloves, poor network, distraction.

Design for the union of these, not the permanent cases alone. A large touch target helps a tremor, a sprained wrist, and a bumpy bus ride equally.

### Design For The Environment, Not Only The Person

Users interact in conditions designers rarely experience: outdoor glare on a phone screen, a kitchen with wet hands, a factory floor with gloves, a moving vehicle, a dim bedroom, a noisy café. A design that tests well in a quiet office can fail in its real context.

Consider:

- contrast that survives sunlight and screen brightness at low settings;
- audio that is not required where sound is impractical or private;
- touch targets that work with gloves, wet fingers, or imprecise pointers;
- offline or degraded-network behavior that does not lock users out;
- glanceable layouts for users who can only look for a second at a time.

### Respect Different Literacy, Language, And Expertise Levels

Users vary in reading level, domain knowledge, digital fluency, and familiarity with the product's conventions. Jargon that feels precise to experts can exclude newcomers; oversimplified labels can frustrate experts. Neither is universally correct.

Prefer:

- plain language for labels, errors, and instructions;
- progressive disclosure so newcomers are not overwhelmed and experts are not blocked;
- recognizable patterns over invented ones, so transferable knowledge helps users;
- avoiding idioms, metaphors, and culturally specific references in core flows;
- consistent terminology so the same concept always uses the same word.

### Avoid Forced Normalcy And Token Representation

Including diverse users means more than adding a stock photo. Forced normalcy appears when the design assumes everyone wants to behave like the archetype and penalizes deviation: password rules that reject names common in some cultures, name fields that reject single names or long names, address forms that assume a fixed structure, or date formats that confuse half the audience.

Audit forms, inputs, and validation for assumptions about names, addresses, family structure, gender, dates, phone numbers, and units. Where the system cannot avoid a structure, explain why and offer the most flexible option.

### Prioritize By Harm, Not By Frequency

Not every edge case deserves equal investment. Rank needs by the severity of exclusion, not by how often they occur. A rare but blocking failure, such as a screen reader user unable to complete checkout, matters more than a frequent but cosmetic inconvenience. Conversely, do not treat rare cases as irrelevant; a low-frequency, high-harm failure can define the product for the people it affects.

## Common Traps

### Designing For Yourself Or The Team

Designers, engineers, and stakeholders are not representative users. Their fluency, devices, and tolerance for complexity are usually far above the audience's. A design that feels intuitive internally can be opaque to real users.

### Treating Accessibility As A Separate Audience

Building a "normal" flow and an "accessible" flow guarantees that the accessible flow decays. Inclusive design means one flow that works for more people, not a parallel product.

### Assuming The Flagship Device

Testing only on the newest hardware hides performance, viewport, input, and rendering problems that affect the majority of users on older or cheaper devices.

### Confusing Averages With Individuals

Averages obscure the tails. A design optimized for the statistical average can fail everyone at the edges, and the edges are where harm concentrates.

### Overgeneralizing From One Persona

A single persona becomes a constraint rather than a guide. Real audiences are distributions, and any one persona hides the variation that causes real problems.

### Ignoring Situational And Temporary Cases

Because these users are "normally abled," teams assume they need no accommodation. In practice, situational limitations affect nearly every user at some point.

## Self-Check

- [ ] The assumed default user was named explicitly and then challenged, rather than left implicit.
- [ ] The design accounts for permanent, temporary, and situational limitations, not only permanent disability.
- [ ] The design was considered under realistic environments: glare, noise, one-handed use, gloves, poor network, and distraction.
- [ ] Labels, errors, and instructions use plain language and avoid unexplained jargon, idioms, or culturally specific references.
- [ ] Forms and inputs accept varied name, address, date, phone, and family structures rather than forcing one format.
- [ ] The design was tested or reviewed on devices and conditions beyond the team's own, including older and lower-end hardware.
- [ ] Needs were prioritized by severity of exclusion, not only by frequency.
- [ ] Representation is functional and structural, not limited to imagery.
- [ ] The design serves a range of expertise levels without forcing newcomers or experts into the wrong depth.
- [ ] No user group is pushed into a separate, lower-quality flow as a substitute for one inclusive flow.