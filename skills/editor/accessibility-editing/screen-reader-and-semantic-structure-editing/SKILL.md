---
name: screen_reader_and_semantic_structure_editing.md
description: Use when the agent is editing content for screen-reader usability, fixing heading hierarchy and reading order, writing semantic markup and ARIA roles, crafting meaningful alt text and image descriptions, clarifying link text, or ensuring content makes sense when read aloud linearly by assistive technology.
---

# Screen Reader And Semantic Structure Editing

A screen reader reads content linearly, sequentially, and without the visual layout that sighted readers rely on to understand structure. Editors miss accessibility problems here because the visual page looks fine: headings appear in the right place, images convey meaning at a glance, and links are obvious from color and position. But for a screen-reader user, a decorative heading tagged as bold text is invisible to navigation, an image without a text alternative is a silent gap, and a link that says "click here" announces nothing about its destination. Poor semantic editing excludes readers, violates accessibility obligations, and produces content that is structurally broken for a significant audience. The editor's task is to ensure the content communicates its full meaning when read aloud, in order, without the visual cues the sighted take for granted.

## Core Rules

### Build A Logical Heading Hierarchy That Works As Navigation

For screen-reader users, headings are not styling; they are the primary means of navigating a page. The editor must verify that the page uses true heading tags in a logical, gap-free hierarchy: one H1 describing the page, followed by H2s for major sections, H3s for subsections, and so on, with no skipped levels. Decorative text styled to look like headings but tagged as paragraphs or bold is invisible to heading navigation. The editor reads the heading outline alone and asks whether it conveys the page's structure and whether a user could navigate by it. Each heading must accurately summarize the section that follows, because users jump to headings to decide what to read.

### Establish A Meaningful Reading Order Independent Of Layout

Visual layout can imply a reading order that the underlying markup does not support. Content arranged in columns, sidebars, callouts, or grids may be read by a screen reader in the wrong sequence, producing nonsense. The editor must verify that the document source order, the order in which content appears in the markup, presents information logically when read linearly. Where the visual order and source order diverge, the editor works with design and development to align them, because forcing a reading order against source order is fragile and often fails. The test is reading the content aloud top to bottom in source order and confirming it makes sense without visual positioning.

### Write Meaningful Alt Text And Image Descriptions

Images that convey information must have text alternatives that convey the same information; images that are purely decorative should be marked so that screen readers skip them. The editor's job is to write or vet alt text based on the image's purpose in context, not on a generic description of the picture. A chart's alt text must convey the data or trend, not say "a chart"; a functional image, like a search icon, must convey its action, such as "search." The editor avoids redundant alt text that repeats adjacent text, and avoids placeholder phrases like "image of" that add noise. For complex images like infographics or diagrams, a short alt text plus a longer text description in the page may be necessary to convey full meaning.

### Make Link Text Descriptive And Independent Of Context

Screen-reader users often navigate by pulling a list of all links on a page, stripped of surrounding text. In that list, links like "click here," "read more," or "learn more" are meaningless, because the context that gave them meaning is gone. The editor must ensure link text describes its destination or purpose on its own, such as "read our accessibility policy" rather than "read more." The editor also avoids redundant repetition when many links appear together and ensures that links with different destinations have different text, while links with the same destination use consistent text. Descriptive link text benefits all users, not only screen-reader users, because it is scannable and unambiguous.

### Use Semantic Markup And ARIA Roles Appropriately

Semantic HTML, such as lists, tables, and form elements used for their intended purpose, conveys structure to assistive technology automatically. The editor encourages semantic markup and resists the temptation to build structure from generic elements styled to look right. Where custom widgets or dynamic content require ARIA roles, states, and properties to communicate behavior, the editor verifies they are applied correctly and not overused. ARIA is powerful but dangerous: misapplied ARIA can break accessibility worse than no ARIA at all, for example by hiding content or announcing the wrong role. The editor's principle is to use native semantics wherever possible and to treat ARIA as a targeted intervention, not a blanket overlay.

### Edit Content To Make Sense When Read Aloud Linearly

Beyond structure, the prose itself must hold up when heard rather than seen. The editor reads content aloud, or imagines doing so, and checks for dependencies on visual cues: references to "the box on the right," "as shown below," or "the red text" that fail without sight. Instructions that rely on color or position must be rephrased to rely on text. Abbreviations and acronyms must be expanded on first use so they are announced correctly. Punctuation matters, because it affects how screen readers pause and inflect. The editor ensures the aural experience is coherent, not just the visual one.

### Verify, Do Not Assume, Accessibility

Accessibility claims are easy to make and easy to get wrong. The editor does not accept that content is accessible because it looks right or because a tool reported no errors. Automated checkers catch some issues but miss many, including meaningless alt text, confusing link text, and illogical reading order. The editor verifies by reading the content as a screen-reader user would, by checking the heading outline, by reviewing alt text in context, and by confirming semantic markup. Where the editor cannot test directly, the editor flags content for expert review rather than signing off on assumptions.

## Common Traps

### Using Headings For Visual Styling

Tagging text as a heading because it looks the right size corrupts the navigation outline. The trap is thinking visually. The editor enforces semantic heading use and routes styling through CSS.

### Skipping Heading Levels

Jumping from H2 to H4 breaks the hierarchy screen-reader users rely on. The trap is casual nesting. The editor verifies a complete, gap-free outline.

### Alt Text That Describes Rather Than Informs

Writing "image of a chart" instead of conveying the chart's data leaves the meaning inaccessible. The trap is generic description. The editor writes alt text based on the image's purpose.

### Non-Descriptive Link Text

"Click here" and "read more" are meaningless out of context. The trap is convenience. The editor rewrites links to describe their destination.

### Relying On Color Or Position To Convey Meaning

Instructions like "click the red button" fail for users who cannot see color or layout. The trap is visual dependence. The editor rephrases to rely on text labels.

### Misusing ARIA To Fix Problems Native Semantics Solve

Over-applying ARIA can hide content or announce wrong roles, worsening accessibility. The trap is treating ARIA as a cure-all. The editor prefers native semantics and uses ARIA surgically.

### Trusting Automated Checkers Alone

Automated tools miss context-dependent issues like unhelpful alt text and illogical order. The trap is equating "no errors" with "accessible." The editor verifies through human review.

## Self-Check

- Does the page have a logical, gap-free heading hierarchy with one H1, accurate section headings, and no decorative misuse of heading tags?
- Does the source reading order present content logically when read linearly, without relying on visual layout to imply sequence?
- Does every informative image have purpose-driven alt text that conveys its meaning, with decorative images marked to be skipped?
- Is every link text descriptive of its destination on its own, with no generic phrases like "click here" or "read more"?
- Has content that relies on color, position, or visual cues been rephrased to rely on text so it works without sight?
- Are lists, tables, and forms built with semantic markup rather than styled generic elements, with ARIA used only as a targeted intervention?
- When read aloud in order, does the content remain coherent, with abbreviations expanded and references not dependent on sight?
- Has the content been verified through human review of the heading outline, alt text, link text, and reading order, not only automated checks?
- Where the editor cannot verify accessibility directly, has the content been flagged for expert review rather than assumed compliant?
- Would a screen-reader user be able to navigate, understand, and act on this content as effectively as a sighted user?
