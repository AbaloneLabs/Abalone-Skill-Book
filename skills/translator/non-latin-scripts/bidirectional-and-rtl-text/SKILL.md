---
name: bidirectional_and_rtl_text.md
description: Use when the agent is translating or reviewing right-to-left scripts such as Arabic Hebrew Persian or Urdu, handling bidirectional text layout, managing RTL and LTR element mixing, addressing mirroring of UI layouts, or ensuring that numbers Latin words and punctuation display correctly within RTL contexts.
---

# Bidirectional And RTL Text

Right-to-left (RTL) scripts such as Arabic, Hebrew, Persian, and Urdu introduce a layer of complexity that Latin-script translators never encounter: text flows in the opposite direction, and within that RTL flow, numbers, Latin words, and some punctuation run left-to-right, creating bidirectional (BiDi) text. Managing BiDi text is not simply a matter of right-aligning paragraphs. The interaction between RTL and LTR elements, the placement of punctuation and symbols, the behavior of UI layouts, and the rendering in different tools and environments all require deliberate handling. When BiDi is mishandled, the results are visible and embarrassing: punctuation jumps to the wrong side, numbers reverse, mixed-language phrases scramble, UI elements point the wrong way, and the text becomes confusing or unreadable for the target audience. Worse, BiDi issues are often invisible in the translation tool but appear in the final product, so they are missed until late. RTL and BiDi text handling is a specialized discipline that combines linguistic accuracy with typographic and engineering judgment, and it cannot be faked by applying LTR defaults to RTL text.

Use this skill when translating, reviewing, or engineering RTL and BiDi text, handling mixed-direction content, or preparing RTL UI layouts. The goal is RTL text that reads naturally, with correct BiDi handling, and layouts that are properly mirrored and functional for the target audience.

## Core Rules

### Understand Bidirectional Text Behavior

BiDi text is not simply reversed; it follows complex rules. Understand the behavior.

The Unicode Bidirectional Algorithm governs how RTL and LTR characters interact within a paragraph. RTL characters such as Arabic and Hebrew letters flow right-to-left, while LTR characters such as Latin letters and numbers flow left-to-right. The paragraph's base direction, set by the dominant script, determines the overall flow, and the algorithm resolves the order of mixed elements. Punctuation and symbols take their direction from context, which can cause them to appear on the wrong side. Understanding this behavior is essential to diagnosing and fixing BiDi issues, because the "wrong" display often follows the algorithm's rules and requires explicit control to correct.

Do not assume RTL text is just reversed LTR; the BiDi algorithm creates complex interactions.

### Set The Base Direction Correctly

The paragraph base direction determines overall flow. Set it correctly.

An RTL paragraph, with dominant RTL content, should have RTL base direction, so it right-aligns and flows right-to-left. An LTR paragraph should have LTR base direction. The base direction is often inferred from the first strong character, but this can fail when content is mixed or when the first character is neutral. Set the base direction explicitly in markup and tools, using HTML dir attributes, CSS direction properties, or document settings, rather than relying on inference. A wrong base direction causes the entire paragraph to flow incorrectly, with punctuation and alignment errors.

Explicit base direction is more reliable than inference, especially for mixed content.

### Handle Mixed-Direction Segments Carefully

Segments mixing RTL and LTR content, such as Arabic text with an English brand name or a number, require careful handling. Manage them.

Within an RTL segment, LTR runs such as numbers, URLs, or Latin words flow left-to-right, but their position relative to the RTL text and to surrounding punctuation can be ambiguous. The BiDi algorithm resolves this, but the result may not match the intended reading order, especially at boundaries between RTL and LTR runs. Use BiDi control characters or markup to clarify direction where the algorithm produces wrong results. Test mixed segments in the rendering environment, because translation tools may display them differently than the final product. Pay special attention to segments with multiple direction changes, which are most prone to scrambling.

### Manage Punctuation And Symbols At Direction Boundaries

Punctuation and symbols at RTL-LTR boundaries are a leading source of display errors. Manage them.

Characters such as parentheses, brackets, quotes, and commas are directionally neutral and take their behavior from context. At a boundary between RTL and LTR text, they can jump to the wrong side or reverse. For example, a closing parenthesis after a Latin word in Arabic text may display as an opening parenthesis on the wrong side. Use BiDi control characters or restructure the text to place punctuation correctly. Verify punctuation placement in the rendered output, not the source view, because the source may look correct while the render is wrong.

### Mirror UI Layouts For RTL Locales

RTL locales expect mirrored UI layouts. Mirror them.

In an RTL UI, the layout is mirrored: navigation that was on the left is on the right, progress flows right-to-left, icons with directional meaning such as back arrows are flipped, and text alignment is right. Mirroring is not just right-aligning text; it is reflecting the entire layout. Provide mirrored layouts for RTL locales, using CSS logical properties, framework RTL support, or manual mirroring. Test the mirrored UI to ensure that directional elements are flipped and that the layout feels natural to RTL users. Failing to mirror produces a UI that feels foreign and confusing.

A back arrow pointing left in an RTL locale may be correct, since back is conceptually left even in RTL, but other directional elements must be evaluated per their meaning.

### Verify Rendering In The Target Environment

BiDi issues are often invisible in translation tools but appear in the final product. Verify rendering.

Translation tools and text editors may display BiDi text differently than browsers, applications, or published formats. A segment that looks correct in the CAT tool may render with scrambled punctuation, wrong direction, or misplaced elements in the product. Always verify BiDi text in the target rendering environment: open HTML in browsers, render UI in the running application, and check published documents. This is the only reliable way to catch BiDi display errors, because the tool view is not authoritative.

### Handle Numbers Dates And Units In RTL Contexts

Numbers, dates, and units within RTL text follow specific rules. Handle them correctly.

Numbers, including Western digits and Arabic-Indic or Eastern Arabic-Indic digits, flow LTR even within RTL text. Dates and units attached to numbers must be positioned correctly relative to the RTL flow. Decide which digit system to use based on locale conventions: some RTL locales use Western digits, others use Arabic-Indic. Verify that numbers display correctly and that attached elements such as currency symbols or units are positioned per locale convention. Mishandled numbers and dates are a common visible defect in RTL content.

### Use BiDi Control Characters And Markup When Needed

When the BiDi algorithm produces wrong results, use control characters and markup to correct them. Know when and how.

Unicode BiDi control characters such as LRE, RLE, PDF, LRI, RLI, FSI, and PDI, and HTML attributes such as dir and bdi, allow explicit control over direction where the algorithm fails. Use them sparingly and deliberately, because overuse complicates the text and can introduce new issues. The most common needs are isolating embedded LTR content within RTL, such as a Latin word or URL, and correcting punctuation at boundaries. Understand which control to use for which case, and test the result in rendering.

## Common Traps

### Treating RTL As Reversed LTR

RTL text follows complex BiDi rules, not simple reversal; assuming so produces scrambled text.

### Relying On Inferred Base Direction

Inferred base direction fails on mixed content; set it explicitly in markup and tools.

### Ignoring Mixed-Direction Boundaries

Punctuation and symbols at RTL-LTR boundaries jump or reverse; manage them with controls or restructuring.

### Not Mirroring UI Layouts

Right-aligning text without mirroring the layout produces a foreign, confusing UI for RTL users.

### Trusting The Tool View Over Rendered Output

BiDi issues appear in rendering, not in the CAT tool; verify in the target environment.

### Wrong Digit System Or Number Position

Using the wrong digits or mispositioning numbers and dates is a visible defect in RTL content.

### Overusing Or Misusing BiDi Controls

Excessive or wrong BiDi control characters complicate text and introduce new issues; use them deliberately.

### Skipping RTL Testing

Without testing in context, BiDi and mirroring defects ship and embarrass the product.

## Self-Check

Before delivering or approving RTL and BiDi text, verify:

- The bidirectional behavior of the text is understood, recognizing that RTL is not reversed LTR but follows the Unicode BiDi Algorithm.
- Paragraph base direction is set explicitly in markup and tools, not relied upon by inference, for both RTL and mixed content.
- Mixed-direction segments with LTR runs inside RTL text are handled with BiDi controls or markup where the algorithm produces wrong results.
- Punctuation and symbols at direction boundaries are positioned correctly, verified in rendering rather than source view.
- UI layouts are mirrored for RTL locales, with directional elements evaluated and flipped where appropriate.
- All BiDi text is verified in the target rendering environment, such as browsers or running applications, not just the translation tool.
- Numbers, dates, and units use the correct digit system and positioning for the locale.
- BiDi control characters and markup are used sparingly and deliberately, with results tested in rendering.
- No LTR default is applied to RTL layout, direction, or punctuation.
- The RTL text reads naturally, with correct BiDi handling, and the layout is properly mirrored and functional for the target audience.
