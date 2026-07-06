---
name: alt_text_and_descriptive_media.md
description: Use when the agent is writing alt text for images, describing charts and diagrams for screen readers, deciding how to convey visual information to blind users, handling complex or decorative images, or ensuring visual content is not invisible to part of the audience.
---

# Alt Text And Descriptive Media

Alt text and descriptions are how visual content reaches blind and low-vision users who navigate with screen readers, and how images become searchable and meaningful when they cannot be seen. The judgment problem is that alt text is usually treated as a compliance checkbox, produced as a generic label or skipped entirely, when in fact it is a writing task that requires real judgment about what an image is for. Creators fail by writing alt text that is too vague to convey meaning, too verbose to be useful, identical to a caption and therefore redundant, or absent altogether. They also fail at the harder cases: charts, diagrams, and complex images that need more than a short phrase to be genuinely accessible.

The harm is that visual content becomes invisible to a segment of the audience, and the meaning of the piece is lost for them. This skill helps the agent decide what to describe, write alt text that conveys purpose, and handle complex visuals responsibly.

## Core Rules

### Describe The Purpose, Not Just The Pixels

Good alt text conveys what the image is meant to communicate in its context, not a literal inventory of pixels. A photo of a team may exist to show camaraderie, scale, or expertise, and the description should serve that purpose.

Write to the image's function:

- state the subject and the relevant action or context;
- include detail that carries the meaning of the image;
- omit irrelevant description that does not serve the purpose;
- match the level of detail to how much the image matters to the content.

Ask what a sighted reader gains from the image, and ensure the alt text delivers an equivalent gain.

### Be Concise But Complete

Alt text that is too short fails to convey meaning; alt text that is too long becomes exhausting to hear and often goes unread. The right length depends on the image's role.

Calibrate length:

- a short phrase for simple, functional images;
- a sentence or two for meaningful photographs;
- longer structured descriptions for charts, diagrams, or complex visuals;
- never a wall of text where a sentence would do.

Conciseness is respect for the reader's time. Completeness is respect for their access. Both matter.

### Never Duplicate The Caption As Alt Text

A common mistake is to copy the visible caption into the alt text, forcing screen reader users to hear the same information twice. Alt text and captions serve different roles and should not be redundant.

Coordinate the two:

- if the caption already says what the image is, alt text can be brief or marked decorative;
- if the caption adds commentary, alt text should describe the image itself;
- ensure the combination gives a screen reader user the full picture without repetition.

Treat caption and alt text as complementary, not interchangeable.

### Mark Decorative Images Correctly

Not every image carries information. Decorative images that exist only for visual styling should be marked as decorative so screen readers skip them, rather than narrating ornament.

Handle decorative images:

- use empty or decorative alt for purely stylistic images;
- do not write alt text like image or divider that adds no meaning;
- reserve descriptive alt for images that convey information or mood relevant to the content.

Forcing descriptions onto decorative images clutters the experience without adding access.

### Handle Charts And Data Visualizations With Real Description

Charts and diagrams are among the hardest images to make accessible because their meaning is in the data, not the picture. A short alt phrase is almost never enough. These need structured, meaningful description.

Describe data visuals properly:

- state what the chart shows and its main takeaway;
- include the key values, trends, or comparisons;
- provide the data in an accessible format, such as a table, where possible;
- avoid describing only the visual form, such as a blue bar and a red bar.

The goal is equivalent access to the information, which for a chart means access to the data and its meaning.

### Write Descriptions For Complex And Instructional Images

Diagrams, maps, infographics, and how-to illustrations often cannot be conveyed in a short alt attribute. They need longer, structured descriptions that walk the reader through the content.

Provide extended descriptions:

- use a long description or adjacent text for complex visuals;
- structure the description in a logical reading order;
- cover regions, steps, or relationships sequentially;
- ensure a reader who cannot see the image could act on the information.

A complex image with only a one-line alt text is functionally inaccessible no matter how polite the line is.

### Place Descriptive Text Where It Is Discoverable

Alt text only helps if it is actually attached and reachable. Descriptions buried in inaccessible formats or omitted from the markup do not reach the audience.

Ensure descriptions are attached:

- put alt text in the image's alt attribute or platform equivalent;
- provide long descriptions in linked or adjacent accessible text;
- avoid putting essential descriptions only inside the image as burned-in text;
- verify the publishing platform actually exposes the alt text to assistive tech.

A description that exists in the creator's notes but not in the published asset helps no one.

### Write In Plain, Predictable Language

Screen reader users hear alt text read aloud, often rapidly. Dense, clever, or unpredictable phrasing is harder to parse by ear. Clear, direct language serves better.

Write for listening:

- lead with the most important information;
- use straightforward sentence structure;
- avoid jargon unless it is essential and defined;
- spell out or expand terms that screen readers may mispronounce.

Alt text is spoken media. Write it to be heard, not just read.

## Common Traps

### Generic Or Vague Alt Text

Labels like image or photo convey nothing. Describe the purpose and meaning.

### Copying The Caption Into Alt Text

Duplicate text forces screen reader users to hear everything twice. Coordinate the two.

### Overlong Descriptions For Simple Images

Walls of text exhaust the reader. Match length to the image's importance.

### Forcing Alt Onto Decorative Images

Stylistic images should be marked decorative, not narrated. Reserve alt for meaning.

### Short Alt For Charts And Diagrams

Data visuals need real description of the data and takeaway, not the visual form.

### Descriptions Not Attached Or Discoverable

Alt text in notes but not in the published asset helps no one. Attach it properly.

### Dense, Unscannable Phrasing

Clever or jargon-heavy text is hard to hear. Write plainly for listening.

## Self-Check

Before approving alt text and descriptive media, verify:

- Each image's alt text conveys its purpose in context, not just a literal pixel inventory.
- Length matches the image's importance, concise for simple images and fuller for meaningful ones.
- Alt text is not a duplicate of the visible caption.
- Purely decorative images are marked decorative rather than given meaningless alt text.
- Charts and data visualizations have descriptions covering the data, trends, and main takeaway, not just visual form.
- Complex or instructional images have extended, structured descriptions in accessible text.
- Descriptions are attached in the alt attribute or platform equivalent and reachable by assistive tech.
- Essential information is not locked inside the image as burned-in text alone.
- Language is plain, direct, and written to be heard by a screen reader.
- A user who cannot see the image would gain an equivalent understanding of the content from the description.
