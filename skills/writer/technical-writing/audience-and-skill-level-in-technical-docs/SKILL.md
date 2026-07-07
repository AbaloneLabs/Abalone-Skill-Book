---
name: audience_and_skill_level_in_technical_docs.md
description: Use when the agent is matching technical documentation to audience expertise, calibrating jargon and context for beginner versus intermediate versus expert readers, auditing assumptions, designing onboarding versus reference docs, countering the curse of knowledge, or serving multiple audiences in one documentation set.
---

# Audience And Skill Level In Technical Docs

Every piece of technical documentation is written for a reader at a particular skill level, whether the writer knows it or not. The expert assumes knowledge the beginner lacks; the beginner gets buried under definitions the expert already holds. The central problem of technical writing is that the same topic must be explained differently to readers at different points on the learning curve, and a document that is perfect for one audience is often useless, or insulting, to another. Matching the document to the audience is not a finishing touch; it is the first decision, and it shapes every subsequent choice about context, jargon, depth, pace, and structure.

The common failure is the curse of knowledge. The writer, who deeply understands the system, unconsciously assumes the reader shares that understanding. Prerequisites go unnamed. Jargon goes undefined. Steps that feel obvious to the expert are omitted, leaving the beginner stranded. The reverse failure also occurs: a writer, anxious not to alienate newcomers, over-explains basics to an expert audience, burying the useful information under remedial context and signaling that the document is not for them. Both failures come from not knowing who the reader is, or from trying to serve every reader at once and serving none of them.

Use this skill when deciding the audience level for a technical document, when calibrating jargon and context, when auditing the assumptions a document makes, when designing onboarding versus reference material, or when a documentation set must serve multiple skill levels. The agent has freedom in depth and style, but the obligation to match the document to a real reader at a real skill level is not optional.

## Core Rules

### Name The Audience And Their Skill Level Before Writing

Before writing, define the reader concretely. "Developers" is not an audience; it is a category. A developer on their first day with the framework, a developer integrating it into a production system, and a developer maintaining its internals are three different readers with three different skill levels and three different needs. Name the reader specifically enough that the definition guides choices about jargon, depth, prerequisites, and examples.

Define the audience by asking:

- what is their current skill level with this technology, beginner, intermediate, expert?
- what adjacent knowledge can be assumed, such as general programming, command line, networking?
- what are they trying to accomplish, learn, evaluate, build, debug, maintain?
- what is their context, are they onboarding, in production, under pressure, exploring?
- what will confuse or frustrate them if omitted or over-explained?

A document written for a named audience makes consistent choices. A document written for "everyone" makes inconsistent ones.

### Audit The Assumptions The Document Makes

Every technical document rests on assumptions about what the reader already knows. These assumptions are often invisible to the writer, because the writer holds the knowledge and cannot see the gap. The skill is to make the assumptions visible, evaluate whether they match the named audience, and either state them explicitly as prerequisites or adjust the document to remove them. An unstated assumption that the reader does not meet produces a document the reader cannot use.

Audit assumptions by scanning for:

- terms used without definition, and whether the audience can be expected to know them;
- steps omitted as obvious, and whether they are obvious to this reader;
- concepts referenced without explanation, such as calling something a closure or a monad;
- tools and environments assumed available, such as a specific shell or package manager;
- prior reading assumed completed, such as an earlier guide in a sequence.

For each assumption, decide: state it as a prerequisite, remove it by explaining, or confirm the audience meets it. Do not let assumptions hide.

### Calibrate Jargon To The Audience's Vocabulary

Jargon is not inherently bad. For an expert audience, the precise technical term is the most efficient and accurate way to communicate, and avoiding it produces verbose, imprecise prose that frustrates the expert. For a beginner audience, the same term is an obstacle that stops the reader cold. Jargon calibration means matching the vocabulary to what the audience already commands, defining terms that are necessary but new, and not over-explaining terms the audience already knows.

Calibrate jargon by:

- using the expert term when the audience is expert, because precision and brevity matter to them;
- defining necessary terms when the audience is learning, the first time they appear, briefly and in context;
- avoiding unnecessary jargon that adds no precision, such as corporate or buzzword vocabulary;
- providing a glossary for documents with many new terms, rather than interrupting the flow repeatedly;
- being consistent, using the same term for the same concept throughout, to avoid confusing the reader with synonyms.

The goal is not to eliminate jargon but to match it. Mismatched jargon, too much for beginners or too little for experts, is what damages the document.

### Counter The Curse Of Knowledge Deliberately

The curse of knowledge is the specific failure where the writer's expertise prevents them from seeing the reader's confusion. The writer cannot un-know what they know, so steps that feel obvious get skipped, prerequisites go unstated, and the reader is left to fill gaps they cannot fill. Countering the curse requires deliberate technique, because simple good intentions are not enough; the knowledge is invisible to the person who holds it.

Techniques to counter the curse of knowledge:

- have someone at the target skill level read the document and report where they got stuck;
- read the document as if you have never seen the system, pretending away your knowledge;
- explain each step's why, not just its what, so the reader can reason when steps differ in their context;
- include the error cases and how to recognize them, because the expert's instinct hides these from the page;
- test the instructions by following them exactly as written, doing only what the document says.

The curse of knowledge is the single most common cause of unusable technical documentation. Treat countering it as a core activity, not an afterthought.

### Separate Onboarding Docs From Reference Docs

Beginners and experts need different kinds of documentation, and confusing them serves neither. Onboarding docs, tutorials, getting-started guides, are for readers who need to build a mental model and achieve a first success. They should be linear, opinionated, and success-oriented, with every step explained and the path to a working result clear. Reference docs are for readers who already have the model and need to look up specifics. They should be non-linear, complete, and precise, optimized for quick lookup rather than reading. A document that tries to be both fails at both: too slow for the expert, too sparse for the beginner.

Match document type to audience need:

- onboarding: linear, guided, builds understanding, ends with a working result;
- reference: non-linear, complete, optimized for lookup, assumes the model;
- explanation: builds deeper understanding for readers who have the basics;
- troubleshooting: for readers in a specific failure state who need diagnosis.

Knowing which type you are writing determines structure, depth, and tone. Do not mix types within a single document.

### Decide How To Serve Multiple Audiences

Many documentation sets must serve readers at multiple skill levels. The wrong response is to write a single document that tries to serve everyone, which ends up over-explaining for experts and under-explaining for beginners. The right response is usually to provide multiple documents or paths, each targeted at a specific audience, with clear signposting so each reader finds the entry point meant for them. This is more work, but it is what actually serves readers.

Multi-audience strategies:

- separate documents per audience, with a landing page that routes readers to the right one;
- layered structure within a document, basics first with explicit links to advanced material;
- progressive disclosure, summary up front, detail available for those who want it;
- explicit prerequisites at the top, so readers know what the document assumes;
- cross-linking between audience-specific docs so a reader can move as their skill grows.

The key is that each reader should be able to find a path suited to their level, rather than wading through material aimed at everyone and therefore at no one.

### Match Depth And Pace To The Reader's Goal

A reader's skill level determines not only what they know but what they want from the document and how fast they want to move. A beginner wants careful pace, reassurance, and visible success. An expert wants density, precision, and the ability to skip ahead. A document that paces itself for a beginner will frustrate an expert; one paced for an expert will lose a beginner. Match the depth and pace to the reader's goal and tolerance.

Consider:

- how much hand-holding the audience wants, and how much they resent;
- how dense the prose should be, more explanation for learners, more compression for experts;
- how many examples are needed, more for learners to build intuition, fewer for experts who generalize;
- how much to explain versus link, explaining for beginners, linking for experts who can follow.

## Common Traps

### Writing For Everyone And Therefore No One

A document that tries to serve all skill levels serves none of them well. Name the audience and write for them, providing separate paths for other audiences.

### The Curse Of Knowledge

The writer's expertise hides the reader's gaps. Prerequisites go unstated, obvious steps get skipped, and the beginner is stranded. Counter it with deliberate technique and testing.

### Over-Explaining To Experts

Anxious not to alienate, the writer buries expert-level content under remedial basics. The expert signals this is not for them and leaves. Match depth to the audience.

### Undefined Jargon For Beginners

Necessary technical terms used without definition stop the beginner cold. Define terms the first time they appear, or provide a glossary.

### Avoiding Jargon For Experts

Stripping precise terminology to avoid intimidating experts produces verbose, imprecise prose that frustrates them. Use the expert vocabulary for the expert audience.

### Mixing Onboarding And Reference

A single document that is both tutorial and reference fails at both. Separate the document types and cross-link them.

### Unstated Prerequisites

Assumptions about prior knowledge, tools, or reading that go unstated produce a document the unprepared reader cannot use. State prerequisites explicitly at the top.

## Self-Check

Before treating the audience match as sound, verify:

- The audience is named concretely enough to guide choices about jargon, depth, pace, and prerequisites.
- The assumptions the document makes were audited and either stated as prerequisites or adjusted to match the audience.
- Jargon is calibrated to the audience's vocabulary, defined for beginners and used precisely for experts.
- The curse of knowledge was countered deliberately, through testing, fresh-eyes reading, or reader feedback.
- Onboarding and reference material are separated rather than mixed in a single document.
- A multi-audience strategy, if needed, provides distinct paths with clear signposting rather than one document for all.
- Depth and pace match the reader's goal and tolerance, with appropriate density and example count.
- No section over-explains to experts or under-explains to beginners relative to the named audience.
- Prerequisites are stated explicitly so readers know what the document assumes.
- A reader at the target skill level could use the document without hitting an invisible gap the writer's expertise concealed.
