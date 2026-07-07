---
name: glanceable_information.md
description: Use when the agent is designing glanceable information for wearables, smartwatches, ambient displays, lock screens, widgets, complications, or any surface where the user must extract meaning in under two seconds without focused attention.
---

# Glanceable Information

Glanceable information is information designed to be understood in a single glance, often under two seconds, often while the user is doing something else. This is a different design problem from readable or interactive information. A screen the user reads assumes they have given it attention. A glanceable surface assumes they have not, and may never give it more than a flick of the eyes. Every decision, what to show, how to show it, and what to omit, must serve that fleeting moment.

The judgment problem is that glanceability is easy to claim and hard to deliver. Designers fill glance surfaces with the information that seems important, add labels and units, include trends and secondary values, and end up with something that takes five seconds and focused attention to parse. The result is a surface that is technically informative but practically useless, because the user cannot extract meaning in the time and attention they actually give it. The agent's job is to ruthlessly prioritize, so that the single most important meaning arrives in the first instant and everything else is either deferred or removed.

Use this skill before designing or reviewing watch faces, complications, lock screen widgets, ambient displays, status rings, notification previews, car dashboard glance surfaces, or any interface meant to be read in passing. The goal is to prevent the agent from shipping glance surfaces that are too dense, too ambiguous, or too dependent on attention the user will not give.

## Core Rules

### Define The Single Answer The Glance Must Deliver

A glanceable surface should answer one question, fast. Before choosing layout, color, or components, state the single answer the user needs in that moment.

Examples of strong glance questions:

- what time is it, and is my next event soon?
- am I on track for my daily movement goal?
- is the alarm set for tomorrow?
- is there anything urgent I need to handle now?
- what is the current heart rate, and is it in a normal range?
- is my flight on time?

If a glance surface tries to answer several unrelated questions, it answers none of them well. Either pick one, or structure the surface so each region answers a distinct question without competing for the same attention.

### Lead With The Conclusion, Not The Data

Glanceable surfaces fail when they show raw data and make the user interpret it. A number alone, a percentage alone, a chart alone, all require the user to recall context, compute meaning, and decide whether to act. That is too much work for a glance.

Lead with meaning by:

- showing status or conclusion before value ("on track", "running late", "within range");
- pairing numbers with their context, goal, or normal range;
- using color, fill, or position to convey state at a glance;
- reserving precision for when the user digs deeper;
- making the need-to-act obvious without reading.

A glance that shows "72" forces interpretation. A glance that shows a filled ring near completion tells the user everything in an instant. Design the conclusion first, then decide how much data supports it.

### Use Shape, Fill, Position, And Color As Primary Carriers

In a glance, recognition beats reading. Shape, fill level, position, and color are parsed faster than text and numbers, and they survive poor lighting, motion, and partial attention better.

Use strong visual carriers:

- rings, bars, and arcs for progress toward a goal;
- position on a dial or scale for where a value sits in a range;
- fill proportion for how much of something remains;
- consistent color states for status, always paired with another cue for accessibility;
- familiar icons for categories the user has learned.

Reserve text and numbers for the precise value, shown smaller or secondarily. The visual carrier delivers the meaning; the text confirms it. Do not let text be the only way to understand the glance.

### Limit The Number Of Independent Elements

Every independent element on a glance surface competes for attention and increases parse time. A watch face with six complications, each with its own value, color, and label, is not glanceable. It is a tiny dashboard.

Limit elements by:

- choosing one primary element and a small number of secondary ones;
- grouping related information so it reads as one unit;
- removing anything that does not serve the primary question;
- avoiding decorative elements that add visual noise;
- using whitespace and hierarchy to separate what remains.

When stakeholders want to add more, the right answer is often a configurable surface where the user chooses, not a surface that shows everything. Glanceability is an act of subtraction.

### Make State And Change Immediately Apparent

Glanceable surfaces are often checked repeatedly to detect change: did the status move, did a goal complete, did something become urgent? The user needs to see what changed without comparing carefully.

Make change visible by:

- using motion or fill completion for goal achievement;
- using clear before and after states for status transitions;
- highlighting what is new, overdue, or abnormal;
- avoiding subtle differences that require careful comparison;
- making completion and threshold crossings unmistakable.

A glance that looks the same whether the user is on track or behind forces the user to remember the previous state. The surface should carry the comparison itself.

### Design For The Real Viewing Conditions

Glances happen outdoors, in motion, at angles, through wet or dirty glass, on dimmed always-on screens, and in a fraction of a second. A design that reads perfectly on a designer's monitor may be illegible on a wrist in sunlight.

Design for real conditions by:

- using high contrast that survives bright ambient light;
- using large enough forms that motion and angle do not blur them;
- ensuring the dimmed always-on state still conveys the essential meaning;
- avoiding fine detail, thin strokes, and low-contrast color pairs;
- testing with the screen at an angle and in motion, not just head-on.

If the glance fails in sunlight or on the dim always-on state, it fails in the conditions where glances actually happen.

### Distinguish Glanceable From Interactive And Detailed Views

A glance surface is not the place for full detail, history, configuration, or interaction. Those belong in a follow-up view the user reaches when they choose to give attention.

Separate the layers by:

- keeping the glance to conclusion and status only;
- linking to a detailed view for values, history, and context;
- avoiding controls and inputs on the glance surface;
- making the boundary between glance and detail clear;
- ensuring the glance is useful even if the user never goes deeper.

The glance is the headline. The detail is the article. Do not try to fit the article into the headline.

## Common Traps

### Showing Data Instead Of Meaning

Bare numbers, percentages, and charts force the user to interpret. Lead with status and conclusion; let data support it.

### Too Many Complications Competing For Attention

Multiple independent values, colors, and labels on one surface destroy glanceability. Subtract until the primary question dominates.

### Precision That Slows The Glance

Showing exact values, decimals, or units prominently makes the user read rather than recognize. Demote precision to the detail view.

### Color As The Only Status Cue

Color-only states fail in sunlight, for color-blind users, and on dim screens. Pair color with shape, fill, position, or icon.

### Ignoring The Dimmed Always-On State

A glance that only works at full brightness fails in the always-on mode where most glances happen. Design the dim state to carry the essential meaning.

### Subtle Change That Requires Memory

If the user must remember the previous state to notice a change, the surface is not doing its job. Make transitions and thresholds unmistakable.

### Treating The Glance As A Tiny Dashboard

Dashboards are for focused comparison; glances are for instant meaning. Do not port dashboard density onto a glance surface.

## Self-Check

- [ ] The glance surface is built around a single primary question the user needs answered in that moment.
- [ ] The surface leads with a conclusion or status, and raw data supports rather than replaces meaning.
- [ ] Shape, fill, position, and color carry the primary meaning, with text and numbers as confirmation.
- [ ] The number of independent elements is strictly limited, grouped, and free of decorative noise.
- [ ] State changes, completions, and threshold crossings are immediately apparent without comparing to memory.
- [ ] Contrast, size, and form remain legible in sunlight, at angles, in motion, and in the dimmed always-on state.
- [ ] Color is never the only carrier of status; it is paired with shape, fill, position, or icon for accessibility.
- [ ] Precision, history, configuration, and interaction are deferred to a detailed view, not crowded onto the glance.
- [ ] The glance is useful even if the user never goes deeper, and the boundary to detail is clear.
- [ ] The surface has been considered in real viewing conditions, not only on an ideal monitor.
