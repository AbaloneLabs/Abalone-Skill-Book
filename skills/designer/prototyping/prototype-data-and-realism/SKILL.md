---
name: prototype-data-and-realism.md
description: Use when the agent is choosing what data to put in a prototype, deciding between placeholder and realistic content, handling edge cases and extreme values in prototypes, managing empty and error states, or ensuring prototype realism does not mislead users and stakeholders about real product behavior.
---

# Prototype Data And Realism

The data and content inside a prototype shape what feedback the prototype produces, often more than the visual design does. Placeholder text, generic names, and tidy sample data feel harmless, but they systematically hide the problems that real usage exposes: long names that break layouts, empty states that confuse users, error paths that never get tested, extreme values that overflow containers, and content variations that reveal structural flaws. The judgment problem is that prototypes are too often filled with idealized, cooperative data that makes the design look good while concealing exactly the edge cases that will cause the real product to fail.

Agents tend to populate prototypes with the first plausible content they think of, treating data as filler rather than as a design material. This skill helps the agent treat prototype data as a deliberate choice that determines whether the prototype surfaces real problems or hides them, and whether the realism it presents is honest or misleading.

## Core Rules

### Treat data as a design material, not as filler

The content in a prototype is not neutral decoration; it actively shapes what the prototype reveals. Short, tidy names hide truncation problems; uniform content lengths hide layout failures; always-present data hides empty states. Choose data deliberately to stress the design: include long names, special characters, varying content lengths, missing fields, and extreme values. The goal is not to make the prototype look pretty but to make it behave like the real product will under realistic content variation.

### Use realistic data for user testing, because users behave authentically only with believable content

In usability testing, users interact with a prototype as if it were real only if the content is believable. Generic placeholder text, obviously fake names, and lorem ipsum break the illusion and cause users to behave unnaturally, commenting on the fakeness rather than performing the task. For testing, use realistic, contextually appropriate content: plausible names, realistic product data, believable amounts, and content that matches what users expect. Realistic data is a precondition for trustworthy behavioral feedback.

### Design and populate every state, not only the happy path

Prototypes overwhelmingly show the populated, successful state, and almost never show the states that cause real problems: empty (no data yet), loading, error, partial failure, offline, permission denied, and extreme volume. Each of these states is a design problem as real as the happy path. Populate and design each state in the prototype, because empty and error states are where users most need help and where designs most often fail. A prototype that only ever shows success is testing the easy case.

### Include edge-case and extreme values that stress the layout

Real content varies wildly, and layouts that look fine with average data break with extremes. Deliberately include: the longest possible name, the shortest, names with non-Latin scripts, very large numbers, zero, negative values where relevant, very long text fields, many items, and single items. These edge cases reveal truncation, overflow, alignment, and pagination problems that average data conceals. Make edge-case data a standard part of the prototype, not an afterthought.

### Avoid misleading realism about performance and behavior

A prototype can fake speed, smoothness, and reliability that the real product will not have, because the prototype does not face real network conditions, real data volume, or real backend constraints. This creates false confidence: stakeholders and users experience a snappy, reliable prototype and assume the product will feel the same. Be explicit about what the prototype fakes: if responses are instant because they are hardcoded, if data volume is tiny, if errors never occur, label these as non-representative so that decisions account for the gap between prototype and production.

### Use representative data variety, not a single archetype

Populating a prototype with data that all resembles one user archetype — one region, one name style, one currency, one content type — hides problems of diversity and scale. Use varied data that represents the real user base: multiple regions and scripts, different currencies and formats, varied content types and lengths, and a range of user scenarios. Homogeneous data produces a prototype that works for one kind of user and silently fails others.

### Manage sensitive and personal data in prototypes

Realistic data must not cross into real personal data. Never use real customer records, real names, or real personal information in prototypes that may be shared, screenshot, or shown to external audiences. Use realistic but synthetic data that resembles the real without exposing anyone. For sensitive domains (health, finance), ensure synthetic data is plausible enough to test with but clearly not real.

### Synchronize data across connected screens and flows

When a prototype spans multiple connected screens or flows, inconsistent data breaks the illusion and confuses users. If a user enters a name on one screen, that name should appear correctly on subsequent screens; if an item is added, it should persist; if a selection is made, it should carry through. Data consistency across the flow is what makes a prototype feel like a real product rather than a sequence of disconnected mockups.

## Common Traps

### Using idealized data that hides problems

Tidy, short, uniform content makes the design look good while concealing truncation, overflow, and layout failures that real data will expose.

### Lorem ipsum and obviously fake content in testing

Generic placeholder content breaks the illusion in user testing and produces unnatural behavior and unusable feedback. Use realistic, believable content for testing.

### Showing only the happy path

Prototypes that only ever show populated success never test the empty, error, and edge states where designs most often fail and users most need help.

### Ignoring extreme values

Average-length names and content hide overflow and truncation. Long names, zero, large numbers, and many items reveal structural problems.

### Misleading stakeholders about performance and reliability

A prototype that fakes instant responses and never errors creates false confidence about how the real product will feel. Label what is faked.

### Homogeneous data representing one archetype

Data from one region, script, or content type produces a prototype that works for one user and fails others. Use varied, representative data.

### Using real personal data in shareable prototypes

Real customer data in prototypes that may be shared or shown externally is a privacy failure. Use realistic synthetic data.

### Inconsistent data across connected screens

When data does not carry across a flow, the prototype feels disconnected and users behave unnaturally. Synchronize data across connected screens.

## Self-Check

- Is the prototype populated with data chosen to stress the design, including long, short, special-character, and extreme values?
- For user testing, is the content realistic and believable enough that users interact authentically?
- Are empty, loading, error, partial-failure, and high-volume states designed and populated, not only the happy path?
- Have you deliberately included edge cases that reveal truncation, overflow, and alignment problems?
- Is it explicit what the prototype fakes about performance, volume, and reliability, so decisions account for the gap?
- Does the data represent the diversity of the real user base across regions, scripts, formats, and content types?
- Have you avoided real personal data, using realistic synthetic content instead?
- Does data carry consistently across connected screens and flows in the prototype?
