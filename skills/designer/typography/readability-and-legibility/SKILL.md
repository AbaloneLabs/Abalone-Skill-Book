---
name: readability_and_legibility.md
description: Use when the agent is choosing typefaces, sizes, line lengths, line heights, weights, or text treatments for body copy and long-form reading, optimizing legibility of small text and labels, selecting measure and column width, or ensuring text remains readable across devices, zoom, low vision, and degraded rendering conditions.
---

# Readability And Legibility

Readability and legibility are often used interchangeably, but they name different problems, and confusing them produces text that looks refined yet exhausts the reader. Legibility is whether individual characters can be told apart; readability is whether extended text can be read comfortably and for long enough to serve its purpose. A typeface can be highly legible in a single word and poor for a paragraph, or vice versa. Most interface text failures come from optimizing the look of type in a design tool while ignoring the conditions under which people actually read: small sizes, long sessions, low vision, zoom, glare, and fatigue.

Use this skill before choosing body typefaces, setting reading sizes, deciding line length and line height, treating small labels, or reviewing whether text will remain readable in real use. The goal is to prevent the agent from selecting type for its aesthetic character at the expense of sustained reading, squeezing text into sizes that look clean but strain the eye, or ignoring the users who depend on larger, clearer type to use the product at all.

## Core Rules

### Choose Body Typefaces For Sustained Reading, Not Character

Display typefaces chosen for personality often make poor body text. Their distinctive letterforms, tight spacing, or low contrast between strokes slow reading and increase fatigue over paragraphs. Body type should be quiet: even in color, generous in spacing, with clearly differentiated characters that the eye can process quickly. Personality belongs in headings and display sizes, where the reader encounters it briefly, not in body copy, where the reader lives with it.

Evaluate body candidates by reading a full paragraph, not by admiring a specimen. If the typeface draws attention to itself after a few lines, it is wrong for body text.

### Ensure Characters Are Distinguishable

Legibility fails when characters look alike. Common confusions include capital I, lowercase l, and the digit 1; the letters O and zero; and pairs such as rn resembling m. At small sizes or low resolution, these distinctions collapse, and readers slow down or misread. A typeface whose characters stay distinct at small sizes is more legible than one whose elegance depends on fine detail that disappears.

Check a body typeface for:

- clear differentiation of commonly confused characters;
- an open counter, the enclosed space in letters like o and e, that stays open at small sizes;
- sufficient stroke contrast that survives low-resolution rendering;
- a tall x-height, which generally improves small-size legibility.

### Set A Reading Size That Serves Real Users

Body text sized for visual elegance is often smaller than comfortable reading requires. Designers, working on large calibrated screens, tolerate sizes that strain users on smaller laptops, phones, or aging eyes. A body size that looks refined at 14px can fatigue readers within a paragraph. Comfortable reading usually wants a slightly larger body size than the aesthetic default, and users frequently increase it further through browser zoom or system settings.

Choose a body size that reads comfortably for sustained periods, and design the layout to absorb the larger size, including wrapped lines and taller blocks, rather than shrinking type to fit a dense layout.

### Control Line Length, The Measure

Line length affects reading more than most designers realize. Lines that are too long force the eye to track far across and make it hard to find the start of the next line; lines that are too short break rhythm and chop the text into fragments. A comfortable measure lets the eye return easily and keeps the reading flow smooth.

Aim for a moderate character count per line for body text, and use max-width or column constraints to hold it. Let the layout reflow rather than stretching text edge to edge across a wide screen, where the measure becomes unreadable regardless of size or typeface.

### Set Line Height For The Size And Measure

Line height must match both the text size and the line length. Small text needs more generous line height so the eye can separate lines; large text needs tighter line height to avoid gaping. Long measures benefit from slightly more line height to help the eye track; short measures can tighten. A single line height across all contexts guarantees that some text is overcrowded and some is gapped.

Treat line height as a per-context decision, tuned to the size and measure of each text block, not as a global constant.

### Preserve Readability Under Zoom, Low Vision, And Degraded Conditions

Many users do not read at the designer's chosen size. They zoom in, increase the system font size, or rely on assistive technology. Text that breaks, overflows, or clips when enlarged excludes these users entirely. Designs that lock text to a fixed pixel size, prevent zoom, or truncate when enlarged fail the readers who need clarity most.

Ensure text:

- scales with user zoom and system font settings without breaking layout;
- uses relative units so it grows and shrinks with user preferences;
- does not clip or overlap when enlarged to 200 percent or more;
- remains readable on low-resolution screens and under glare.

### Treat Small Text And Labels As A Legibility Problem

Labels, captions, metadata, and helper text are often the smallest text on the screen, yet they carry operational information. Small text magnifies every legibility weakness: confused characters, low stroke contrast, and tight spacing all worsen as size drops. Text that is merely small but high-contrast and well-spaced stays legible; text that is small, thin, and low-contrast becomes unreadable.

For small text, prefer slightly larger sizes, heavier weights, stronger contrast, and more letter-spacing than body text would need. Avoid using tiny text as a way to make dense layouts appear clean.

### Avoid Treatments That Sacrifice Reading For Style

All-caps body, very thin or very heavy weights at small sizes, low-contrast gray text, tight letter-spacing, and text over busy backgrounds all reduce readability while looking stylish in isolation. Reserve such treatments for short labels and headings where style can dominate, and keep body text in conditions known to support sustained reading.

## Common Traps

### Body Typeface Chosen For Personality

A distinctive typeface that delights in a specimen fatigues the reader across paragraphs of body text.

### Body Size Sized For Aesthetics

A refined small body size strains readers on smaller screens and aging eyes, when comfort should govern the choice.

### Lines Too Long Or Too Short

An uncontrolled measure forces the eye to track too far or chops text into fragments, both of which break reading flow.

### One Line Height Everywhere

A global line height overcrowds some text and gaps other text, when each block needs tuning to its size and measure.

### Small, Thin, Low-Contrast Labels

The operational text that users act on is made the least legible, often to keep a dense layout looking clean.

### Locking Text Size And Blocking Zoom

Fixed pixel sizes and disabled zoom exclude users who depend on larger text to read at all.

### Stylistic Treatments On Body Text

All-caps, thin weights, or low-contrast gray body text sacrifices reading for a look that works only on short labels.

## Self-Check

- [ ] The body typeface was evaluated by reading a full paragraph, not only admiring a specimen, and stays quiet during sustained reading.
- [ ] Commonly confused characters remain distinguishable at the chosen body size and on low-resolution rendering.
- [ ] Body size prioritizes comfortable sustained reading over visual refinement, and the layout absorbs the larger size.
- [ ] Line length is constrained to a comfortable measure, and text does not stretch edge to edge across wide screens.
- [ ] Line height is tuned per text block based on size and measure, not set as a single global value.
- [ ] Text scales with user zoom and system font settings without clipping, overlapping, or breaking layout.
- [ ] Small labels, captions, and metadata use larger sizes, heavier weights, stronger contrast, and more spacing than body text.
- [ ] Stylistic treatments such as all-caps, thin weights, and low-contrast gray are confined to short labels and headings, not body text.
- [ ] The type was reviewed on real devices, including phones and lower-resolution screens, not only on a large calibrated monitor.
- [ ] Reading comfort was tested over a sustained passage, not a single line, to catch fatigue that short samples hide.