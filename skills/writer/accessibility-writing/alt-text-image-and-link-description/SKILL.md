---
name: alt_text_image_and_link_description.md
description: Use when the agent is writing alt text, image descriptions, link text, captions, or accessible media descriptions, conveying the meaning of images and links to readers who cannot see them, or ensuring text alternatives carry the content's purpose without redundancy or omission.
---

# Alt Text Image And Link Description

Images and links carry meaning that text alone does not always convey, and readers who do not perceive them lose that meaning unless a text alternative exists. Alt text describes an image for a reader who cannot see it; link text tells a reader where a link goes and why it matters. Both are small pieces of writing with outsized consequences. Done well, they restore the meaning the visual or link carries. Done poorly, they either omit the meaning or bury it under noise, leaving the reader with less than sighted readers receive.

The judgment problem is deciding what an image or link is actually for. Many writers treat alt text as a labeling chore and produce descriptions that are technically present but useless: "image," "photo of woman," "click here." Others over-describe, reciting every pixel when only one detail carries the meaning. The same goes for links: "read more" tells a screen reader user nothing about where they would go. The harm is exclusion: readers who depend on text alternatives receive a degraded version of the content, unable to act on images or navigate links that sighted readers use effortlessly. This skill prevents the rework of retrofitting descriptions and prevents the quiet harm of incomplete access.

Use this skill when adding images, icons, charts, diagrams, or links to content; writing captions and descriptions; or auditing a piece for accessible media. The agent has freedom to choose rich visuals, but must provide text alternatives that carry the same purpose the visual serves.

## Core Rules

### Describe The Purpose, Not The Pixels

The central question is what the image is for. An image may illustrate a concept, convey data, set a mood, show a product, or provide instruction. The alt text should convey that purpose, not catalog the image's visual contents. A photograph of a cluttered desk used to illustrate "disorganization" needs alt text about disorganization, not an inventory of the desk's objects.

Determine purpose by asking:

- Why is this image here? What does it add that the text does not?
- If the image disappeared, what would the reader lose?
- Is the image illustrating, proving, decorating, or instructing?
- What is the single most important thing the reader should take from it?

Describe the takeaway the image exists to deliver. Pixels are the medium, not the message.

### Distinguish Informative, Decorative, And Functional Images

Not all images need the same treatment. An informative image carries meaning the reader needs; a decorative image adds atmosphere without information; a functional image acts as a control or link. Each requires different handling, and mistaking one for another produces either missing meaning or useless noise.

Handle each type:

- informative image: alt text that conveys the meaning or data;
- decorative image: empty or null alt text so screen readers skip it;
- functional image (a button or link): alt text that names the action or destination;
- image with text in it: the alt text includes the text, since it cannot be read otherwise.

Mislabeling a decorative image as informative clutters the reader's experience; mislabeling an informative image as decorative deletes meaning.

### Write Links That Name Their Destination And Purpose

Link text is the reader's signpost. Generic links like "click here," "read more," or "link" tell a screen reader user nothing out of context, and they are weak even for sighted scanners. Good link text names where the link goes and what the reader will find, so it makes sense in isolation.

Write link text that:

- names the destination or the content, not the mechanism;
- makes sense when read out of context, as in a list of links;
- is specific enough to distinguish from other links on the page;
- is concise but not so terse it becomes vague;
- avoids URLs as visible text unless the URL itself is the content.

"Read the 2024 accessibility report" beats "click here." The reader should know what they are getting before they commit.

### Convey Data Images Through Equivalent Text

Charts, graphs, and diagrams present information visually, and that information must be available in text for readers who cannot see the image. A data image without a text equivalent excludes readers from the evidence. The text alternative should convey the same findings the chart communicates, not merely say "a bar chart."

For data images:

- state what the chart shows and its main finding or trend;
- provide the key values a sighted reader would extract;
- include the full data in a nearby table when the chart carries detail;
- do not describe the chart type and stop; describe what the chart reveals;
- ensure the text alternative is genuinely equivalent, not a label.

If a sighted reader would draw a conclusion from the chart, the text alternative must let the non-sighted reader draw the same conclusion.

### Avoid Redundancy Between Alt Text And Surrounding Text

Alt text should add what the surrounding text does not, and avoid repeating what it already says. If the caption or body text fully explains the image, the alt text should not duplicate it; it should carry only what is missing. Redundancy forces screen reader users to hear the same information twice.

Balance by:

- checking what the surrounding text already conveys;
- writing alt text that supplies only the additional meaning;
- using brief alt text when the caption carries the rest;
- avoiding "image of" or "picture of" unless the medium itself matters;
- coordinating caption and alt text so they complement, not echo.

The goal is that the reader receives the full meaning once, cleanly, not twice or not at all.

### Keep Descriptions Concise But Complete

Alt text should be as long as necessary and as short as possible. Most images need a sentence; some need more; a few need a full description elsewhere. The discipline is including everything that carries meaning and nothing that does not. Padding ("a beautiful, stunning, gorgeous photograph of...") wastes the reader's time and obscures the point.

Aim for:

- the shortest description that conveys the image's purpose;
- the key detail or takeaway, not an exhaustive inventory;
- plain language a reader can hold in working memory;
- longer descriptions in a separate location only when the image is complex.

Concision respects the reader. Completeness serves them. Both matter; neither alone is enough.

### Handle Complex And Essential Images With Extended Descriptions

Some images carry too much meaning for a short alt text: detailed diagrams, maps, complex infographics, artworks being analyzed. For these, a brief alt text is insufficient, and an extended description is needed elsewhere, linked from the image. Omitting the extended description excludes readers from content that is central to the piece.

For complex images:

- provide short alt text that identifies the image and its role;
- provide an extended description, linked nearby, that conveys the detail;
- structure the extended description so a reader can navigate it;
- for diagrams, describe the relationships, not only the components;
- for art under analysis, convey what the analysis depends on.

The test is whether a reader who cannot see the image could follow the surrounding argument or analysis.

### Write Captions That Work With, Not Against, Alt Text

Captions and alt text serve different readers and different purposes, and they should be coordinated. A caption contextualizes an image for all readers; alt text conveys the image's meaning for readers who cannot see it. When both exist, they should complement each other rather than repeat or contradict.

Coordinate by:

- using the caption for context, source, or commentary;
- using alt text for the image's content and purpose;
- ensuring a reader using a screen reader gets meaning from alt text, and all readers get context from the caption;
- not putting essential meaning only in the caption if the image itself is informative;
- not putting essential meaning only in alt text if sighted readers need it too.

Coordination ensures every reader receives the full meaning without redundancy or gaps.

## Common Traps

### Writing "Image Of" And Stopping

"Image of a chart" conveys nothing the reader can use. Describe what the image shows or means, not that it exists.

### Cataloging Pixels Instead Of Purpose

Listing every visual detail buries the meaning. Describe the takeaway the image exists to deliver.

### Treating Decorative Images As Informative

Adding alt text to purely decorative images clutters the experience without adding meaning. Mark them for skipping.

### Using "Click Here" Or "Read More" As Link Text

Generic link text is meaningless out of context. Name the destination and purpose.

### Forgetting Data Images Need Equivalent Findings

A chart's alt text must convey its findings, not its type. Without the data in text, readers are excluded from the evidence.

### Duplicating Caption And Alt Text

Hearing the same information twice wastes the reader's time. Coordinate the two so they complement.

### Omitting Extended Descriptions For Complex Images

Brief alt text cannot carry a detailed diagram or analyzed artwork. Provide extended descriptions for essential complex images.

## Self-Check

Before treating the media descriptions as complete, verify:

- Each image's alt text conveys its purpose, not a pixel inventory.
- Informative, decorative, and functional images are handled differently and correctly.
- Link text names its destination and purpose and makes sense out of context.
- Data images have text alternatives that convey the same findings a sighted reader would extract.
- Alt text does not redundantly repeat the surrounding text or caption.
- Descriptions are concise but complete, with no padding and no missing meaning.
- Complex or essential images have extended descriptions where short alt text is insufficient.
- Captions and alt text are coordinated to complement rather than duplicate; no image carries essential meaning that is unavailable to readers who cannot see it
- No link is generic ("click here") where a descriptive label is possible; text in images is provided in the alt text, since it cannot be read otherwise
- A reader using a screen reader would receive the same meaning as a sighted reader; descriptions were written for the reader who needs them, not as a compliance label
