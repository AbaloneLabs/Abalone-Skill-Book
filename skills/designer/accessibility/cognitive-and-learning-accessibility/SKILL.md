---
name: cognitive_and_learning_accessibility.md
description: Use when the agent is designing or reviewing an interface for cognitive accessibility, including reading level, memory load, attention, decision simplicity, consistent mental models, dyslexia, ADHD, low literacy, language learners, and interfaces that must remain understandable under cognitive strain.
---

# Cognitive And Learning Accessibility

Most accessibility effort focuses on vision and motor access. Cognitive accessibility is harder to see in a screenshot and easier to miss, but it affects far more users: people with dyslexia, ADHD, autism, low literacy, cognitive fatigue, language barriers, age-related decline, and anyone using an interface while distracted, stressed, tired, or in a hurry. An interface that is technically operable can still be unusable if it demands too much memory, attention, decoding, or decision-making.

Use this skill before finishing forms, multi-step flows, settings, onboarding, dashboards, help content, error recovery, and any interface where the user must understand, remember, compare, or decide. The goal is to prevent the agent from producing a design that a sighted, rested, neurotypical expert can use but that excludes users who think, read, or process differently.

## Core Rules

### Minimize Working Memory Load

Working memory is limited. Interfaces that force users to hold information in mind — a code from one screen needed on another, a rule explained once and never repeated, a value hidden behind a toggle — create failures for users with reduced memory capacity and for anyone interrupted mid-task.

Reduce memory load by:

- keeping context visible: show the relevant summary, total, or constraint next to the decision, not on a prior screen;
- persisting selections and progress across steps;
- restating requirements near the field that enforces them;
- avoiding instructions that disappear once the user begins typing;
- showing the result or consequence of a choice before committing to it.

If the user must remember something to succeed, the design is carrying risk.

### Match Reading Level To The Audience

Complex sentences, jargon, abbreviations, and long paragraphs exclude users with low literacy, dyslexia, language differences, and cognitive fatigue. Readability is an accessibility concern, not only a style preference.

Aim for:

- short sentences and common words for general-audience interfaces;
- plain labels instead of internal terminology or clever phrasing;
- one main idea per sentence and per paragraph;
- definitions or tooltips for unavoidable technical terms;
- summaries before detail, so users can decide whether to read further.

For specialized professional tools, the reading level can rise, but even expert users benefit from clarity under stress or fatigue.

### Keep The Mental Model Consistent And Predictable

Users build a mental model of how the interface works and rely on it. Inconsistent placement, unexpected behavior, controls that change meaning by context, and flows that break their own rules force users to rebuild the model repeatedly.

Support a stable mental model by:

- placing primary navigation, search, and key actions in predictable locations;
- using the same control for the same kind of action everywhere;
- keeping terminology stable across screens;
- not changing the meaning of a color, icon, or label between contexts;
- making the current location and available actions obvious.

Predictability is especially important for users with autism or cognitive differences who rely on consistency, and for any user learning a new interface.

### Simplify Decisions And Avoid Overload

Too many options, too much simultaneous information, and too many required decisions create choice paralysis and errors. Cognitive accessibility asks the designer to reduce what the user must process at once.

Techniques:

- progressive disclosure: show the essentials first and reveal detail on demand;
- sensible defaults so the user does not have to decide everything;
- grouping related choices rather than presenting one long flat list;
- ordering options by likelihood or importance;
- separating primary actions from secondary and advanced options;
- breaking long flows into clear, reviewable steps with the ability to go back.

Simplification is not dumbing down. It is respecting the user's attention.

### Support Error Prevention And Easy Recovery

Cognitive load spikes during errors. Users with attention or processing differences are more likely to make slips, and a harsh error experience compounds the difficulty.

Design for gentle recovery:

- prevent errors where possible with constraints, defaults, and confirmation;
- explain errors in plain language with a clear fix;
- preserve entered data when an error occurs so the user does not retype;
- allow undo for reversible actions;
- avoid punitive or blame-oriented error copy;
- make the path back to a correct state obvious.

### Respect Attention And Avoid Overwhelm

Autoplaying media, flashing banners, multiple simultaneous notifications, animated backgrounds, and auto-advancing content compete for attention and can overwhelm users with ADHD, sensory sensitivity, or simply low bandwidth for distraction.

Prefer:

- motion that the user initiates or that supports comprehension;
- the ability to pause, stop, or dismiss attention-grabbing elements;
- calm default states with optional richness;
- avoiding more than one urgent, animated, or modal element at a time.

### Provide Multiple Paths To Understanding

People understand in different ways. Some read, some scan, some rely on examples, some need to try. Offering more than one path to the same understanding helps users with different cognitive styles.

Examples:

- text plus an icon or illustration for key concepts;
- an example alongside a rule;
- a short video or walkthrough alongside written instructions;
- a search and a browse path to the same content;
- a summary plus expandable detail.

## Common Traps

### Testing Only With Expert Users

If the only people who tested the interface are fluent, rested, and familiar with the domain, cognitive barriers remain invisible. Real users include the distracted, the tired, the new, and the processing-diverse.

### Disappearing Instructions

Placeholder text and tooltip-only guidance vanish the moment the user needs them most. Persistent labels and inline help survive typing, errors, and review.

### Assuming Simplicity Means Fewer Features

Removing useful features is not accessibility. The goal is to reduce simultaneous cognitive demand while preserving capability through structure, defaults, and progressive disclosure.

### Clever Or Dense Copy

Witty labels, abbreviations, and dense legal-style sentences may read as sophisticated but exclude users who decode text slowly or translate as they read.

### Forcing Linear Memory Across Screens

Multi-step flows that do not show a summary, do not persist progress, and do not allow review assume perfect memory. They fail under interruption and for users with reduced working memory.

### Overwhelming Error States

A wall of red errors after submission, with no indication of what to fix first, is a cognitive cliff. Field-level, specific, recoverable errors reduce the spike.

## Self-Check

- [ ] The interface keeps necessary context, totals, constraints, and progress visible rather than relying on the user's memory.
- [ ] Reading level, sentence length, and terminology match the audience, with plain labels and definitions for unavoidable jargon.
- [ ] Navigation, controls, colors, icons, and terminology are consistent so the mental model stays stable across screens.
- [ ] Decisions are simplified through defaults, grouping, progressive disclosure, and clear ordering rather than presented as one overwhelming set.
- [ ] Errors are prevented where possible and, when they occur, are explained plainly with preserved data and an obvious recovery path.
- [ ] Attention-grabbing elements, motion, and simultaneous notifications do not overwhelm users who are sensitive to distraction.
- [ ] Key concepts offer more than one path to understanding: text plus icon, rule plus example, or summary plus detail.
- [ ] Instructions, requirements, and help remain available after the user begins interacting, not only before.
- [ ] The flow was considered under realistic cognitive strain: distraction, fatigue, newness, language difference, and interruption.
