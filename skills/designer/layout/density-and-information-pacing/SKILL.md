---
name: density_and_information_pacing.md
description: Use when the agent is deciding how much information to show at once, choosing between dense and spacious layouts, pacing content reveal, managing cognitive load, grouping for scanability, or balancing expert efficiency against first-time comprehensibility on dashboards, lists, and detail views.
---

# Density And Information Pacing

Density is the decision of how much to put in front of the user at once, and pacing is the decision of when and how to reveal it. These are not styling choices; they are comprehension choices. Too dense and the user is overwhelmed, missing the one thing that matters in a wall of data. Too sparse and the expert cannot get work done, clicking and scrolling to assemble information that should sit together. Agents tend to default to whatever the reference product does, or to fill every pixel because space feels wasteful, without analyzing who the user is, what they already know, and what they are trying to find. The harm is a layout that is either exhausting for newcomers or inefficient for experts, and usually both at once.

Use this skill before deciding how much to show on a screen, how to group and pace it, or how to balance density across user types. The goal is to prevent the agent from choosing density by aesthetic preference, from showing everything for fear of hiding something, or from over-pacing a simple task into needless steps.

## Core Rules

### Match Density To User Expertise And Task Frequency

The right density depends on who is using the surface and how often. An expert who lives in a tool all day wants density: more on screen, less clicking, comparison without navigation. A first-time or infrequent user wants space: fewer choices, clear labels, room to understand each item. There is no universally correct density.

Decide density by asking:

- is this a daily expert workflow, an occasional task, or a one-time action?
- does the user already know what they are looking for, or are they exploring?
- is speed or comprehension the priority for this surface?
- will users tolerate a learning curve to gain efficiency?

An admin console used eight hours a day should be far denser than a public form used once. Optimizing the expert tool for first-time clarity wastes their time; optimizing the one-time form for density overwhelms the user.

### Let Hierarchy, Not Whitespace Alone, Manage Density

Density is not solved by adding or removing whitespace. A dense layout can be highly scannable if hierarchy is strong, and a spacious layout can be confusing if hierarchy is weak. The real lever is how clearly the eye is told what matters, what groups together, and what is secondary.

Build hierarchy to carry density:

- use size, weight, color, and position to mark primary, secondary, and tertiary information;
- group related items so the eye reads clusters, not a flat field;
- align columns and labels so the eye can scan a row without re-parsing;
- repeat the structure so users learn where to look.

Whitespace supports hierarchy, but it cannot substitute for it. Adding space to a poorly structured layout just spreads the confusion out.

### Pace Reveal Based On Need, Not On Hiding For Tidiness

Pacing is revealing information when the user needs it. Good pacing reduces what the user must process up front while keeping the next step visible. Bad pacing either dumps everything at once or hides things so deeply that users cannot find them. The test is whether each reveal answers a question the user has at that moment.

Pace by intent:

- show summary and primary action first, detail on demand;
- reveal fields progressively as earlier answers make them relevant;
- keep secondary and expert options accessible but not foregrounded;
- avoid forcing a user through steps to reach information that should be visible.

Hiding the total on a checkout to keep the page tidy is bad pacing; the user needs the total. Progressive disclosure that matches the user's questions is good pacing.

### Group For Scanability, Not For Database Structure

How information groups on screen shapes how fast users find what they need. Grouping should reflect how users think about the task, not how the data is stored. When grouping mirrors internal data models, users hunt through irrelevant clusters.

Group by user mental models:

- cluster fields by the decision they support, not by their database table;
- put related read-together values adjacent so the eye scans once;
- separate groups that serve different intents (identification, configuration, billing);
- label groups with the user's vocabulary, not internal names.

A settings page grouped by backend service forces users to learn the architecture. The same fields grouped by user intent (profile, notifications, security) scan instantly.

### Use Progressive Disclosure Without Burying Critical Paths

Progressive disclosure is powerful for managing density, but it is also the most misused pacing tool. It helps when it reveals depth that most users do not need; it harms when it hides steps that everyone needs behind extra clicks. The distinction is whether the hidden content serves the majority or a minority.

Apply progressive disclosure when:

- the hidden detail serves advanced or infrequent needs;
- revealing it would overwhelm first-time users;
- the entry point is clearly labeled and discoverable;
- the path to the hidden content is short and reversible.

Do not use it when the hidden content is part of every user's core path, when it makes a frequent task slower, or when the entry point is ambiguous. Burying a required field under "advanced" guarantees support tickets.

### Balance Consistency With Contextual Density

A product often needs different density in different places, and that is correct, but it must not feel arbitrary. Density should shift for clear reasons (a dense data table versus a spacious settings form) while shared patterns stay consistent. Inconsistent density with no rationale feels broken.

Maintain coherence while allowing contextual density:

- keep shared elements (headers, spacing scale, type ramp) consistent across dense and sparse surfaces;
- let density vary by content type and task, with the variation explainable;
- document why certain surfaces are denser so the choice is deliberate;
- avoid sudden density jumps between adjacent views that share a workflow.

### Account For Reading Versus Scanning

Some surfaces are read carefully (an article, a contract, a detail page); others are scanned (a list, a table, a dashboard). Density and pacing should serve the dominant mode, and the two modes need opposite treatments.

Match treatment to mode:

- reading surfaces: comfortable measure, generous line height, space between paragraphs, minimal distraction;
- scanning surfaces: tight rows, aligned columns, emphasis on differentiators, density that rewards quick comparison;
- hybrid surfaces: decide which mode dominates and design for it, then support the secondary mode on demand.

A dashboard designed like an article is slow to scan; an article designed like a dashboard is exhausting to read.

### Test Density Against The Full Range Of Data

Density chosen against typical data fails at the edges. A dense table that looks balanced with ten rows collapses with five hundred; a spacious card layout that looks generous with three items becomes endless scrolling with fifty. Test density against the range the surface will actually hold.

Stress-test density with:

- the smallest plausible data set (does it look empty or broken?);
- the largest plausible data set (does it become unusable?);
- edge cases: very long names, many empty fields, extreme values;
- localized and translated content that changes length.

## Common Traps

### Defaulting To The Reference Product's Density

Copying another product's density without analyzing its audience and task produces a layout that fits them, not your users.

### Showing Everything To Avoid Hiding Anything

Filling the screen to ensure nothing is missed overwhelms users and hides the one thing they need behind a wall of data.

### Adding Whitespace To Fix Poor Hierarchy

Space cannot fix a layout whose hierarchy is weak; it only spreads the confusion out and makes scanning slower.

### Progressive Disclosure That Hides Core Paths

Burying steps every user needs behind "advanced" or extra clicks turns a simple task into a frustrating hunt.

### Grouping By Database Structure

Clusters that mirror internal data models force users to learn the architecture before they can find their fields.

### One Density For The Whole Product

Applying uniform density to reading and scanning surfaces underserves both; density should vary by content type and task.

### Ignoring Data Range

Density validated only against typical data breaks at the extremes: empty, very large, long-text, and localized states.

### Over-Pacing Simple Tasks

Breaking a straightforward task into many revealed steps to feel "guided" adds clicks without aiding comprehension.

## Self-Check

- [ ] Density was chosen based on user expertise, task frequency, and whether speed or comprehension dominates, not by aesthetic preference.
- [ ] Hierarchy (size, weight, color, position, grouping) carries the density, and whitespace supports rather than substitutes for it.
- [ ] Pacing reveals information when the user needs it, keeping the next step visible without dumping everything up front.
- [ ] Grouping reflects user mental models and decision intent, not internal database structure.
- [ ] Progressive disclosure hides only advanced or infrequent needs, never core paths, with discoverable and reversible entry points.
- [ ] Density varies by content type and task for explainable reasons while shared patterns stay consistent.
- [ ] Reading and scanning surfaces are treated differently, with each designed for its dominant mode.
- [ ] Density was stress-tested against the smallest, largest, edge-case, and localized data, not only typical content.
