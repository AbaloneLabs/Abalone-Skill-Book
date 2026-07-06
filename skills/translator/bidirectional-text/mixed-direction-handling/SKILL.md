---
name: mixed_direction_handling.md
description: Use when the agent is translating or reviewing segments that mix right-to-left and left-to-right runs, embedding Latin words numbers URLs dates email addresses or code inside Arabic Hebrew Persian or Urdu text, diagnosing punctuation and parenthesis reversal at direction boundaries, deciding where to insert Unicode bidirectional control characters, or reviewing strings whose display order differs from their logical storage order.
---

# Mixed Direction Handling

Real localized content is rarely monodirectional. An Arabic sentence contains an English product name, a Hebrew paragraph embeds a date and a currency amount, a Persian interface string wraps a URL, a user-facing message concatenates a translated label with a Latin variable. Whenever right-to-left and left-to-right runs sit inside the same segment, the Unicode Bidirectional Algorithm steps in to decide what displays where, and its decisions do not always match what the reader expects. The hard part of mixed-direction work is not the individual scripts but their boundaries: the points where an RTL run meets an LTR run, and the neutral characters such as parentheses, slashes, hyphens, and quotation marks that sit between them. At those boundaries the algorithm is genuinely ambiguous, and without explicit control the punctuation jumps, the parentheses swap, the number detaches from its label, and a sentence that was correct in storage renders as nonsense on screen. Translators miss these defects because their editing tools often display logical order, while the reader sees resolved visual order, so a string that looks perfect in the CAT tool can ship scrambled.

This skill applies when you are producing, reviewing, or repairing segments that contain both RTL and LTR content, when you are deciding whether and where to insert bidirectional controls, and when you are diagnosing why a stored string renders differently from how it reads. The objective is mixed-direction text whose resolved visual order matches the intended reading order in every rendering environment, achieved with the minimum necessary control and verified against the rendered output.

## Core Rules

### Reason In Logical Order, Verify In Visual Order

Mixed-direction text is stored in logical order, the sequence in which characters are typed and read, but it is displayed in visual order after the bidirectional algorithm resolves it.

You must hold both models in mind. You author and store text in logical order, because that is what databases, search, and source control preserve. But you judge correctness only in visual order, because that is what the reader sees. A common failure is to edit a string until it looks right in the tool's logical view and assume it will render correctly, when in fact the visual order at a boundary is wrong. Establish a habit of rendering every mixed-direction segment and inspecting the visual result before approving it. Never approve mixed-direction content based solely on the logical source view.

### Anticipate Where The Algorithm Becomes Ambiguous

The bidirectional algorithm is deterministic, but at RTL-LTR boundaries its deterministic output frequently diverges from intent, and you should learn to predict those spots.

Ambiguity concentrates at the edges between runs and around neutral characters. A parenthesis between an RTL word and an LTR word can attach to either side. A slash between two Latin terms inside RTL text can be read in the wrong order. A hyphenated Latin phrase can split. A trailing punctuation mark after an LTR run at the end of an RTL sentence can jump to the wrong side. When you encounter a segment with a direction change, mentally simulate the boundary and ask whether the neutral characters will resolve as intended. If the simulation is uncertain, treat the segment as needing explicit control.

### Use Directional Isolation For Embedded Opposite-Direction Runs

When an LTR run such as a number, a date, an email address, a URL, or a Latin term is embedded in RTL text, isolate it so it does not disturb the surrounding flow.

Isolation tells the algorithm to treat the embedded run as a self-contained unit whose own direction is resolved internally, then spliced into the outer flow. The modern mechanism is the isolating controls, LRI, RLI, and FSI, paired with PDI, or the HTML `bdi` element. Isolation is preferable to the older embedding controls because it prevents the embedded content from leaking direction back into its context. Apply isolation around any embedded run whose own direction differs from the paragraph and whose boundaries are ambiguous. Do not isolate everything; isolate only where the algorithm's default resolution is wrong or fragile.

### Fix Boundary Punctuation Deliberately

Punctuation at RTL-LTR boundaries is the single most common source of visible defects, and it must be handled with intent rather than hope.

Neutral characters take their direction from context, and at a boundary the context is split, so a closing parenthesis can render as an opening one, a comma can land on the wrong side, and a quotation mark can mirror incorrectly. When boundary punctuation resolves wrongly, correct it by restructuring the segment, by isolating the adjacent run, or by adding a directional mark that pins the punctuation to the intended side. Avoid reflexively sprinkling control characters; first try reordering the content or isolating a run, which is often cleaner. When you do use a control, choose the minimal one that fixes the specific boundary and document why it is there, because unexplained controls are a maintenance hazard.

### Preserve The Pairing And Balance Of Controls

Bidirectional controls come in pairs, and unbalanced or mispaired controls corrupt the entire paragraph, not just the local run.

Embedding controls such as LRE and RLE must be closed by PDF, and isolating controls must be closed by PDI. A missing or duplicated closing control changes the resolved order of everything that follows it, producing cascading corruption that can reach the end of the segment. When you insert controls, insert them as balanced pairs, verify the pairing by counting opens and closes, and remove controls as a pair when you remove them. Treat a control sequence the way you treat markup tags: never leave one dangling.

### Decide Digit And Number Representation Deliberately

Numbers run left-to-right even inside RTL text, and the choice of digit system and number formatting carries locale meaning that should be decided, not defaulted.

Some RTL locales use Western digits, others use Arabic-Indic or Eastern Arabic-Indic digits, and the convention varies by locale and sometimes by register. A number also carries formatting such as the decimal and thousands separators and the currency symbol position, all of which must be consistent with the chosen locale. Decide the digit system deliberately based on the target locale's convention, apply it consistently across the deliverable, and verify that numbers, separators, and currency render in the correct relative order. Do not assume that a number copied from the source is formatted for the target locale.

### Test Across Renderers, Not Just One

Different rendering engines resolve bidirectional text with subtly different results, and a segment correct in one environment can fail in another.

Browsers, operating system text views, PDF renderers, and mobile frameworks may apply the algorithm with different defaults, different handling of controls, and different base-direction inference. A segment that looks correct in a desktop editor can render differently in a mobile app or a generated PDF. Test mixed-direction strings in the actual target environments, and where environments differ, prefer the most robust solution, typically explicit base direction plus isolation, over a fix that happens to work in only one renderer.

## Common Traps

### Approving Based On The Logical Source View

The CAT tool shows logical order; the reader sees visual order, so a segment that looks right in the tool can ship scrambled. Always verify in rendered visual order.

### Sprinkling Control Characters Reflexively

Adding controls everywhere to feel safe produces fragile, hard-to-maintain text and can introduce new corruption; use the minimal control that fixes the specific boundary.

### Unbalanced Or Mispaired Controls

A missing PDF or PDI corrupts everything after it; insert and remove controls as balanced pairs and verify the count.

### Letting Embedded Runs Leak Direction

An unisolated LTR run inside RTL text can change the resolution of its surroundings; isolate opposite-direction runs at ambiguous boundaries.

### Copying Source Numbers Without Locale Decision

Digits, separators, and currency position carry locale meaning; copying them from the source without deciding the target convention produces inconsistent or wrong formatting.

### Trusting A Single Renderer

Engines resolve bidirectional text differently; a fix that works in one environment can fail in another, so test in the actual target environments.

### Treating Punctuation As Directionally Neutral In All Cases

Neutral characters resolve by context and are unpredictable at boundaries; boundary punctuation needs deliberate handling, not assumption.

## Self-Check

Before delivering or approving mixed-direction segments, verify:

- Each segment has been inspected in resolved visual order, not only in the logical source view of the editing tool.
- Direction-change boundaries have been mentally simulated and any neutral characters whose resolution is ambiguous have been identified.
- Embedded opposite-direction runs such as numbers, dates, URLs, and Latin terms are isolated where the default resolution is wrong or fragile.
- Boundary punctuation such as parentheses, slashes, hyphens, and quotation marks renders on the intended side, corrected by restructuring or minimal controls.
- All bidirectional controls are inserted and removed as balanced pairs, with open and close counts verified.
- The digit system, separators, and currency position reflect a deliberate locale decision applied consistently, not a copy of the source.
- Mixed-direction strings have been tested in the actual target rendering environments, not just one tool or browser.
- Controls are used minimally and each one present is justified, with no reflexive or unexplained directional characters left in the text.
