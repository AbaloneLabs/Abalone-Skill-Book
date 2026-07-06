---
name: structure_and_navigation_for_accessibility.md
description: Use when the agent is building headings, lists, and document structure for screen readers and skim-reading, using semantic structure as an accessibility practice, designing navigation, or ensuring content is perceivable and navigable for readers using assistive technology or scanning.
---

# Structure And Navigation For Accessibility

Structure is invisible to the eye until it is missing. For a sighted reader skimming a page, headings and lists are visual cues that can be faked with bold text and spacing. For a reader using a screen reader, there are no visual cues; there is only the document's semantic structure, which the assistive technology reads as a map. When the structure is real, the reader can navigate by heading, jump to sections, and understand the hierarchy. When the structure is fake or absent, the reader is trapped in a stream of text with no way to find what they need.

The judgment problem is that visual structure and semantic structure are different things, and most writers think only in visual terms. Bold text that looks like a heading may not be a heading to a screen reader; a list formatted with dashes may be read as a paragraph; a logical reading order on the page may be scrambled in the code. The harm is direct: readers who rely on structure to access content are excluded by structure that only looks right. This skill prevents the rework of rebuilding document structure after publication and prevents the quiet exclusion of readers who depend on semantic navigation.

Use this skill when structuring documents, web content, instructions, or long-form text; when designing headings and navigation; or when ensuring content works with assistive technology. The agent has freedom to design visually, but must build semantic structure that carries the same meaning the visuals suggest.

## Core Rules

### Build Semantic Structure, Not Just Visual Layout

Semantic structure is meaning encoded in the document's structure, independent of how it looks. A real heading tells the document, and any tool reading it, that this text is a heading and at what level. Visual styling can produce the same appearance without the meaning. Accessibility requires that structure be semantic, so that the meaning survives when the visuals are removed.

Prefer semantic elements:

- use true heading levels, not bold or large text that mimics them;
- use true lists, not dashes or numbers typed into a paragraph;
- use structural elements for tables, quotes, and asides rather than visual approximations;
- ensure reading order in the code matches the intended logical order.

If the only thing conveying structure is appearance, the structure fails for readers who do not experience the appearance.

### Use A Logical Heading Hierarchy

Headings form an outline that readers use to navigate and to understand the relationship between sections. A logical hierarchy descends through levels without skipping, so that a level-two heading is followed by level-three subheadings, not a sudden jump to level four. Skipped levels and misused headings break the outline and disorient readers navigating by structure.

Heading practices:

- use one level-one heading per page or document, usually the title;
- descend through levels without skipping (two, then three, not two then four);
- do not choose heading level for visual size; choose it for hierarchy;
- keep headings descriptive enough to convey the section's content;
- avoid using headings purely for styling text that is not a heading.

The heading outline should make sense as a table of contents read on its own.

### Make Headings Descriptive And Predictable

Headings are navigation aids. A reader using a screen reader may pull up a list of headings to decide where to go; a sighted reader scanning the page uses headings to locate content. Descriptive headings tell the reader what each section contains; vague or clever headings force the reader to enter the section to find out.

Write headings that:

- name the section's actual content or purpose;
- are specific enough to distinguish from other sections;
- avoid being clever at the cost of being clear, in informational content;
- follow a predictable pattern within the document;
- front-load the key word so scanners find it fast.

In informational and instructional content, clarity beats cleverness. Reserve ambiguity for genres where it serves the reader.

### Use Lists Semantically For Parallel Items

Lists signal that items are parallel and countable, which helps both scanning readers and assistive technology. A real list lets a screen reader announce the number of items and navigate between them. A fake list, typed as dashes in a paragraph, loses all of that. Use lists when items are genuinely parallel; avoid them when prose flows better.

Use lists when:

- items are parallel steps, options, conditions, or examples;
- order matters (numbered lists) or does not (bulleted lists);
- the reader benefits from seeing items as discrete units.

Do not fragment prose into lists purely for visual effect; lists that should be paragraphs harm comprehension and clutter navigation.

### Ensure Logical Reading Order

Reading order is the sequence in which content is encountered, which for screen readers is determined by the document's structure, not its visual layout. Content that is visually arranged in columns, sidebars, callouts, or tables may be read in an order that scrambles meaning if the structure does not enforce the intended sequence.

Verify reading order by:

- checking that the code order matches the intended logical order;
- ensuring sidebars and callouts do not interrupt the main flow confusingly;
- testing tables so that header and data cells associate correctly;
- confirming that visually positioned elements do not detach from their context;
- reading the content in source order to confirm it still makes sense.

If the meaning depends on visual arrangement that the structure does not preserve, the structure is broken.

### Make Navigation Survive Without Visuals

Robust structure is structure that works when the visuals are stripped away. A reader using a screen reader, a reader with low vision using custom styles, and a reader on a device that reflows content all depend on structure that does not rely on appearance. Test structure by imagining the content with no styling.

The test:

- can the reader navigate by headings alone?
- are lists recognizable as lists without bullets?
- is the reading order correct without layout?
- are tables understandable as data, not as visual grids?
- does emphasis survive without bold or color?

If structure collapses without visuals, rebuild it semantically.

### Provide Wayfinding For Longer Content

In longer documents, readers need wayfinding: landmarks that help them know where they are and how to move. Tables of contents, section summaries, "back to top" links, and consistent navigation all serve wayfinding. Without them, readers using assistive technology or scanning long content can become lost.

Wayfinding elements:

- a table of contents with links to sections, for long documents;
- descriptive section headings that act as landmarks;
- summaries at the start of long sections;
- consistent placement of navigation and controls;
- clear indication of the reader's location in a sequence.

Wayfinding is not decoration; it is the difference between a long document that is usable and one that is exhausting.

### Associate Labels, Controls, And Context

Interactive elements and reference content must be properly associated with their labels and context. A form field needs a label a screen reader can announce; an image needs a description; a table needs headers associated with its cells; an expandable section needs a programmatically related label. Without association, the reader encounters controls and content without knowing what they are.

Ensure association by:

- linking form labels to their fields programmatically;
- providing alt text or descriptions for images (see the dedicated skill);
- associating table headers with data cells;
- giving buttons and links descriptive names, not "click here";
- grouping related controls with a shared label where appropriate.

Association makes structure machine-readable, which is what makes it reader-readable through assistive technology.

## Common Traps

### Faking Headings With Bold Or Size

Text styled to look like a heading but not marked as one is invisible to screen readers. Use true heading levels.

### Skipping Heading Levels For Visual Effect

Choosing heading levels for size breaks the outline and confuses navigation. Choose levels for hierarchy.

### Using Typed Characters Instead Of Real Lists

Dashes or numbers typed into a paragraph are not lists to assistive technology. Use semantic list structures.

### Assuming Visual Order Equals Reading Order

Visually arranged content may be read in a scrambled sequence. Verify the source order carries the meaning.

### Vague Or Clever Headings In Informational Content

Headings that prioritize style over clarity force readers to enter sections blind. In instructional content, be descriptive.

### Forgetting That Color And Style Carry No Meaning Alone

Information conveyed only by color or visual styling is lost to readers who do not perceive it. Encode meaning in text and structure.

### Treating Accessibility As A Styling Afterthought

Adding structure after the content is written is harder and often incomplete. Build semantic structure from the start.

## Self-Check

Before treating the structure as accessible, verify:

- Structure is semantic, not merely visual; headings and lists are real elements.
- The heading hierarchy is logical, with one level-one heading and no skipped levels.
- Headings are descriptive enough to serve as a navigation outline on their own.
- Lists are used for genuinely parallel items and formatted as real lists.
- Reading order in the source matches the intended logical order.
- The structure survives when visuals are removed.
- Longer content provides wayfinding: tables of contents, summaries, landmarks.
- Form fields, buttons, links, and images are properly labeled and associated; tables associate headers with data cells and remain meaningful without visual layout
- No meaning is conveyed by color or styling alone; a reader using a screen reader could navigate to any section and understand their location
- A reader scanning visually and a reader using assistive technology encounter the same structure; structure was built in from the start, not retrofitted at the end
