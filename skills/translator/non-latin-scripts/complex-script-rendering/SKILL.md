---
name: complex_script_rendering.md
description: Use when the agent is translating or reviewing complex scripts such as Devanagari Thai Arabic or Tamil, handling conjuncts and ligatures, managing character shaping and reordering, addressing font and rendering support, or ensuring that complex scripts display correctly across tools and platforms.
---

# Complex Script Rendering

Complex scripts, including Devanagari, Thai, Arabic, Tamil, and many South and Southeast Asian scripts, present rendering challenges that simple Latin workflows do not anticipate. In these scripts, characters do not simply appear one after another in the order typed. Characters combine into conjuncts and ligatures, vowels and marks reorder around consonants, contextual shaping changes a character's form based on neighbors, and diacritics stack or cluster in ways that require sophisticated font and rendering engine support. When rendering is handled correctly, the text appears as the script should. When it is mishandled, characters separate that should combine, marks land in wrong positions, conjuncts break into visible components, and the text becomes unreadable or garbled for native readers. These problems often stem not from the translation but from the rendering pipeline: fonts that lack the needed features, tools that do not invoke complex text layout, or encoding issues that corrupt character sequences. Translators and reviewers working with complex scripts must understand enough of the rendering mechanics to diagnose problems, specify requirements, and verify that the final output is correct, because a linguistically perfect translation rendered badly is still a failure.

Use this skill when translating, reviewing, or engineering complex script text, handling shaping and reordering, or ensuring correct rendering across tools and platforms. The goal is complex script text that is linguistically accurate and renders correctly for native readers in every environment where it appears.

## Core Rules

### Understand What Makes A Script Complex

Complex scripts require more than one-glyph-per-character rendering. Understand the complexity.

Complex scripts involve character shaping, where a character's glyph changes based on context, as in Arabic's initial, medial, final, and isolated forms; reordering, where vowels or marks render in a position different from their encoding order, as in Devanagari's dependent vowels; conjuncts and ligatures, where sequences of characters combine into single glyphs, as in Indic conjuncts; and stacking or clustering, where marks stack vertically or cluster around a base character, as in Thai or Devanagari. These features require a complex text layout engine to render correctly; without it, the raw character sequence displays as separated, wrong-form, or mispositioned glyphs. Understanding the specific complexities of the target script is essential to diagnosing rendering issues.

A translator who does not understand the script's complexity cannot tell whether a problem is linguistic or rendering.

### Require Complex Text Layout Support

Complex scripts require a complex text layout (CTL) engine to render. Require and verify its support.

Rendering engines such as HarfBuzz, Uniscribe, or CoreText apply the shaping, reordering, and ligation rules that make complex scripts display correctly. Not all tools and environments invoke CTL; some assume simple glyph rendering and produce broken output for complex scripts. Verify that the authoring tools, CAT tools, publishing systems, and target rendering environments support CTL for the relevant script. Test with sample text that exercises the script's complex features, such as conjuncts, stacked marks, and contextual forms. If CTL is not supported, the output will be wrong regardless of the translation quality.

Do not assume a tool handles complex scripts because it accepts the Unicode; rendering support is separate.

### Use Fonts With Complete Shaping Support

Complex scripts require fonts with the needed shaping features. Use and verify complete fonts.

A font for a complex script must include the glyphs for conjuncts and ligatures, the contextual alternates for shaping, and the positioning tables for marks and stacking. Not all fonts are complete: some omit rare conjuncts, lack certain contextual forms, or have incorrect positioning. Use well-supported fonts known to work for the script, and verify that the font covers the characters and features in the actual content. Test with the full range of text, including any rare or specialized characters. A font that renders common text but breaks on conjuncts or stacks will produce visible defects in real content.

Font incompleteness is a leading cause of complex script rendering failure.

### Verify Conjuncts Ligatures And Stacking

The complex features of the script must render correctly. Verify them specifically.

For Indic scripts, verify that conjuncts form correctly, that half-forms and ligatures display, and that the vowel signs and marks position correctly around consonants. For Thai, verify that vowels and tone marks stack and position correctly above or below consonants and that combining sequences do not separate. For Arabic, verify that letters connect in the correct contextual forms and that ligatures such as lam-alef form. Check stacking, where multiple marks cluster, for correct vertical positioning. These features are where rendering engines and fonts most often fail, so verify them with text that exercises them, not just common prose.

A script that renders common words but breaks on conjuncts or stacks is not production-ready.

### Handle Encoding And Character Sequences Correctly

Complex scripts are sensitive to encoding order and normalization. Handle encoding correctly.

Some scripts allow multiple Unicode sequences to represent the same visual result, differing in character order or composition; normalization forms such as NFC and NFD reconcile these, but mixing forms can cause rendering and search issues. For scripts with combining marks, the order of base character and marks matters for rendering. Verify that the text uses a consistent normalization form and that character sequences are in the order the rendering engine expects. Encoding corruption, such as mojibake or lost combining marks, is especially damaging in complex scripts because the visual result is garbled. Verify encoding integrity at every stage.

### Address Reordering And Cursor Movement

Complex script reordering affects editing behavior. Address it where relevant.

Because characters render in a different position than their encoding order, cursor movement and editing in complex scripts can be non-intuitive: the cursor may move visually rather than logically, or text insertion may land in an unexpected place. For translation and review tools, verify that editing behaves predictably for the script. For UI where users input the script, ensure that text entry and cursor behavior feel natural to native users. Reordering issues are most visible during editing and input, so test those interactions.

### Test Rendering Across All Target Platforms

Complex script rendering varies across platforms and applications. Test across all targets.

A script that renders correctly in one browser or operating system may fail in another, due to differences in CTL engines, font availability, or fallback behavior. Test the final content in every platform and application where it will appear: different browsers, operating systems, mobile devices, and document formats. Identify and address platform-specific rendering failures, which may require font embedding, fallback configuration, or content adjustments. Platform variation is a major source of complex script defects that appear only after delivery.

### Verify Complex Scripts In Context

As with all scripts, verify complex script text in context. Verify fit and function.

Complex scripts may have different width and height characteristics than Latin, affecting layout. Stacked marks may require more vertical space, causing overflow in fixed-height containers. Verify that text fits, that line breaks occur at valid points, and that the script renders correctly within the product's UI and layout. Context verification catches issues that isolated text review misses, such as truncation that cuts through a stacked mark cluster.

## Common Traps

### Assuming One-Glyph-Per-Character Rendering

Complex scripts require CTL; without it, characters separate and marks misposition.

### Using Tools Without CTL Support

A tool that accepts Unicode but does not invoke CTL renders complex scripts incorrectly.

### Fonts Missing Shaping Features

Incomplete fonts break on conjuncts, contextual forms, or mark positioning.

### Not Testing Conjuncts And Stacking

Common prose may render while conjuncts and stacks fail; test the complex features.

### Inconsistent Normalization

Mixed normalization forms cause rendering, search, and consistency issues in complex scripts.

### Ignoring Platform Variation

A script correct on one platform may fail on another; test all target environments.

### Skipping Context Verification

Complex scripts may overflow or break in layout; verify fit and function in the product.

### Confusing Linguistic And Rendering Problems

A rendering defect may look like a translation error; understand the script to diagnose correctly.

## Self-Check

Before delivering or approving complex script text, verify:

- The specific complexities of the target script, including shaping, reordering, conjuncts, and stacking, are understood.
- The authoring, CAT, and target rendering environments support complex text layout for the script, verified with sample text.
- Fonts with complete shaping support are used, covering all characters and features in the content.
- Conjuncts, ligatures, contextual forms, and mark stacking render correctly, tested with text that exercises these features.
- Encoding uses a consistent normalization form, with character sequences in the order the rendering engine expects, and no corruption.
- Reordering and cursor movement behave predictably in editing and input contexts for the script.
- Rendering is tested across all target platforms and applications, with platform-specific failures addressed.
- The text is verified in context, with fit, line breaks, and layout confirmed in the product's UI.
- No simple-rendering assumption or incomplete font is allowed to break the script's complex features.
- The complex script text is linguistically accurate and renders correctly for native readers in every environment.
