---
name: information_hierarchy.md
description: Use when the agent is designing a screen, page, dashboard, form, landing section, or product surface and must decide information hierarchy, visual priority, grouping, scanning order, and what users should notice first.
---

# Information Hierarchy

Design is not only making things look acceptable. It is deciding what a person can understand first, what they can safely ignore, what action they can take next, and how much effort it takes to compare information. A screen with all the correct content can still fail if priority, grouping, and scanning order are unclear.

Use this skill before designing or reviewing pages, dashboards, forms, settings screens, lists, profile views, product pages, reports, onboarding flows, landing pages, or any interface where multiple pieces of information compete for attention. The goal is to prevent the agent from arranging elements by decoration, equal weight, or implementation convenience instead of user decision flow.

## Core Rules

### Define The Primary User Question

Every screen should answer at least one primary question. Before choosing layout, typography, or components, state what the user is trying to learn or decide.

Examples:

- "What is the current status?"
- "Which item needs attention first?"
- "What changed since last time?"
- "Can I trust this recommendation?"
- "What action should I take next?"
- "Is this the product, place, document, or record I was looking for?"

If the screen has no primary question, it often becomes a pile of content. If it has too many primary questions, it needs stronger grouping, navigation, or separate views.

### Create A Clear Priority Ladder

Not every element can be first. Establish a ladder of importance:

1. identity and context;
2. current state or main value;
3. primary action or decision;
4. supporting details;
5. secondary actions;
6. metadata, history, or advanced controls.

Use size, weight, spacing, position, contrast, alignment, and grouping to express this order. Avoid giving headings, cards, buttons, badges, icons, and statistics the same visual force. Equal emphasis makes the user do the prioritization work.

### Group By Task Meaning, Not Data Shape

Do not group content only because it comes from the same API object, table, model, or CMS field. Group by how the user makes sense of the task.

Useful grouping patterns:

- current status and next action;
- identity and trust signals;
- summary and supporting evidence;
- required fields and optional fields;
- risk indicators and mitigation actions;
- comparison criteria;
- historical activity and current configuration.

When a group exists, give it a clear boundary through spacing, heading, alignment, or layout. Avoid nested boxes unless the product's design system uses them carefully. Too many containers create visual noise without improving understanding.

### Design For Scanning Before Reading

Most users scan before they read. A good hierarchy lets the user form an accurate first impression in a few seconds.

Support scanning through:

- predictable placement of titles, status, actions, and metadata;
- concise labels;
- aligned columns;
- consistent number formats;
- meaningful section headings;
- repeated patterns for repeated objects;
- visible status and exception states;
- enough whitespace to separate ideas.

Do not rely on long explanatory text to rescue a confusing layout. Explanatory copy can help, but the structure should carry meaning.

### Match Density To Context

The right density depends on use case. Operational tools, CRM views, admin dashboards, and developer consoles often need compact, comparable information. Editorial pages, portfolios, venue pages, and brand pages may need more immersive spacing and imagery. Mobile layouts need clear tap order and reduced simultaneous complexity.

Avoid applying one visual style everywhere. A dashboard should not behave like a marketing hero. A landing page should not hide the product identity. A dense table should not use oversized decorative cards that slow comparison.

### Make State And Change Obvious

Interfaces often fail because they show data but not state. Users need to know whether something is active, pending, failed, outdated, selected, disabled, risky, recommended, or incomplete.

For important state, choose visible treatments:

- status labels with plain text;
- color plus shape or icon, not color alone;
- timestamps with context;
- before/after comparisons;
- progress and completion indicators;
- empty, loading, error, and partial states.

Do not bury critical state in small metadata or rely on color alone.

### Keep Actions Close To Their Consequences

Actions should appear near the objects or decisions they affect. A primary page action belongs in a predictable high-priority position. Row-level actions belong near the row. Destructive actions need separation, confirmation, or clear labeling when consequences are serious.

If a screen has several possible actions, rank them. Primary, secondary, tertiary, and destructive actions should not look interchangeable.

## Common Traps

### Decorating Before Structuring

Gradients, cards, illustrations, icons, and shadows cannot fix unclear hierarchy. Start with information order and grouping, then style.

### Making Everything A Card

Cards are useful for repeated independent items. They are poor as a default wrapper for every section. Nested cards and floating sections can make the page feel fragmented and reduce scanability.

### Overusing Hero Scale

Large display type belongs to true hero moments or brand identity. In dashboards, sidebars, forms, and compact tools, oversized headings can waste space and push important controls away.

### Hiding The Primary Object

On product, venue, profile, document, or object-focused pages, the object must be visible immediately. If the user cannot tell what they are viewing in the first viewport, the hierarchy is failing.

### Treating Empty Space As Polish Alone

Whitespace should clarify relationships. Too little creates clutter; too much can separate related controls, slow repeated work, and hide important content below the fold.

### Letting Implementation Order Become Reading Order

Data often arrives in a convenient technical order. Users need decision order. Reorder content for comprehension, not for the shape of the backend response.

## Self-Check

- [ ] The primary user question for the screen is explicit.
- [ ] The visual hierarchy clearly distinguishes primary identity, current state, main action, supporting details, secondary actions, and metadata.
- [ ] Content is grouped by task meaning rather than database, API, or CMS shape.
- [ ] A user can scan the screen and understand the main state or decision before reading paragraphs.
- [ ] Density matches the product context: operational, editorial, marketing, transactional, mobile, or comparison-heavy.
- [ ] Important state, change, risk, and status are visible and not communicated by color alone.
- [ ] Primary, secondary, tertiary, and destructive actions have distinct priority and placement.
- [ ] Repeated objects use consistent structure so users can compare them quickly.
- [ ] The primary object, product, person, place, or record is visible in the first viewport when relevant.
- [ ] The layout avoids decorative structure that does not improve comprehension.
