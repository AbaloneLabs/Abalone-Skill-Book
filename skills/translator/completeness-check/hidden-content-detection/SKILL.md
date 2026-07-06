---
name: hidden_content_detection.md
description: Use when the agent is finding source content that is easy to overlook because it lives outside the main text flow, including hidden layers conditional content alt text tooltips metadata embedded comments non printing characters locked segments and text in images, ensuring nothing translatable is missed.
---

# Hidden Content Detection

A surprising amount of translatable content is invisible to a normal read of the source. It lives in places the eye skips: the alt text behind an image, the tooltip that appears on hover, the metadata in a document's properties, the comment a reviewer left in the margin, the text in a hidden slide layer, the conditional content that shows only for certain users, the non-printing characters that carry labels, the text baked into a graphic. Translators who work from what they can see on screen routinely miss this content, and the localized product ships with a mixture of translated prose and untranslated fragments that betray it as partially localized. The harm is not only aesthetic. Hidden content often carries function: an alt text that screen readers depend on, a tooltip that explains a button, metadata that search engines index, a comment that contains a critical instruction. Missing it can break accessibility, usability, discoverability, and even compliance. Detection of hidden content is a distinct skill because the content is, by nature, not in the main flow; you have to know where to look and make a deliberate effort to surface it. This skill covers the categories of hidden content, where each hides, how to extract it, and how to confirm nothing translatable was left behind.

Use this skill when localizing documents, presentations, software, websites, e-learning, or any content with layers, metadata, or embedded text, or when confirming that all translatable content, including the non-obvious parts, has been captured. The goal is to surface and translate content that a surface read will always miss.

## Core Rules

### Inventory The File Beyond Its Visible Text

Before translating, inventory what the file actually contains beyond the text you can read on screen. The visible prose is often a fraction of the translatable material.

For office documents, check document properties and metadata, headers and footers, footnotes and endnotes, comments and tracked changes, text boxes and shapes, and embedded objects. For presentations, check slide notes, hidden slides, slide masters and layouts, and text in grouped or layered objects. For software and websites, check resource files for strings not shown in the main UI, tooltips, alt text, aria labels, placeholder and error messages, and content loaded dynamically. For e-learning, check closed captions, transcripts, alt text, and branching scenario text. For graphics and PDFs, check text baked into images, form fields, and bookmarks. Build the habit of asking, for every file type, what else is in here besides the body text.

### Capture Alt Text Tooltips And Accessibility Content

Accessibility content is translatable content, and it is frequently missed because it never appears in the visible layout.

Alt text describes images for screen readers and must be translated so that users with visual impairments receive the information in their language. Tooltips and title attributes provide context on hover and carry meaning. Aria labels and roles label interactive elements for assistive technology. Long descriptions for complex images carry substantial information. These elements are content, not decoration, and leaving them untranslated means the localized product is inaccessible to part of its audience. Extract them, translate them with the same care as visible text, and verify they still make sense in context, because alt text that describes the wrong image is worse than none.

### Translate Metadata That Users Or Systems See

Metadata is often overlooked because it sits in properties dialogs, but some of it reaches users or systems and must be translated.

Document titles, subject, keywords, and description fields appear in search results, file previews, and accessibility tools, and they affect discoverability. SEO metadata on websites, page titles and meta descriptions, directly shapes whether localized pages are found. Software metadata such as app names, store descriptions, and permission explanations faces end users. Translate the metadata that users or systems will see, and coordinate with whoever owns search and indexing so that translated metadata is actually deployed. Metadata that is purely internal, such as author names or version tags, may not need translation; distinguish the two deliberately rather than ignoring all metadata by default.

### Surface Comments Tracked Changes And Reviewer Notes

Comments and tracked changes contain text that may need translation, depending on the deliverable's purpose, and they are easy to miss because they sit in margins or revision views.

In collaborative documents, comments may contain instructions, questions, or context that a target reader or reviewer needs. Tracked changes show edits whose wording may matter. Decide based on the brief whether to translate these: a deliverable meant to preserve the review trail needs them translated; a clean final deliverable may strip them. Do not silently leave them in the source language in a document otherwise translated, because that creates a jarring mixed-language result. If the decision is to remove them for the clean version, do so deliberately and note it.

### Find Conditional Dynamic And Branching Content

In software, websites, and e-learning, content often appears only under conditions: error messages that show on failure, branching text that depends on user choices, locale-conditional content, and dynamically loaded strings. This content is translatable but invisible in a static view.

Work from the resource files or source-of-truth, not from screenshots, because screenshots show only one state. Extract all strings, including those for rare states and error paths. For e-learning, walk every branch of every scenario so that text shown only on a particular path is captured. For software, translate the full string catalog, including debug messages if they are user-facing. Conditional content is where partial localization is most visible to users, because the moment they hit an error or an edge case, the source language suddenly appears.

### Extract Text Embedded In Images And Graphics

Text baked into images, infographics, diagrams, and charts is translatable content that no text-extraction tool will find unless you look for it.

Identify every image that contains text: charts with axis labels and legends, diagrams with callouts, infographics, screenshots with captions, and decorative typography that carries meaning. For each, decide whether to translate the embedded text by editing the source graphic, overlaying translated text, or recreating the image. Coordinate with designers, because translated text may not fit the original layout and may require font and layout adjustments. Leaving image text untranslated is one of the most common and most visible localization failures.

### Confirm Nothing Was Left In The Source Language

After translation, confirm that no translatable content remains in the source language. A localized deliverable with stray source-language fragments is incomplete.

Run a check for untranslated segments, source-language strings in the target, and mixed-language content. This catches the hidden content that was missed: the tooltip still in English, the alt text not updated, the image text left as-is. Treat any remaining source-language content as a defect to resolve, either by translating it or by documenting why it was intentionally left, such as a brand name governed by policy.

## Common Traps

### Translating Only The Visible Body Text

Working from what appears on screen misses alt text, tooltips, metadata, comments, and embedded text that a complete localization requires.

### Forgetting Accessibility Content

Alt text, aria labels, and tooltips are content for part of the audience; leaving them untranslated breaks accessibility.

### Ignoring Metadata That Users See

Document titles, SEO descriptions, and store metadata reach users and systems; untranslated metadata hurts discoverability and user experience.

### Leaving Comments In The Source Language

Mixed-language documents with untranslated comments look unfinished; decide deliberately whether to translate or strip them.

### Missing Conditional And Error-State Strings

Working from screenshots captures only one state; error messages and branching text are missed and appear suddenly to users.

### Overlooking Text Baked Into Images

Image text is invisible to extraction and is one of the most visible localization failures when left untranslated.

### Not Checking For Leftover Source-Language Content

A final scan for untranslated fragments catches the hidden content that was missed during translation.

## Self-Check

Before approving a localization as complete, verify:

- The file was inventoried beyond visible body text, including properties, headers, footers, notes, comments, text boxes, embedded objects, and slide masters as relevant.
- Accessibility content, alt text, tooltips, title attributes, aria labels, and long descriptions, was extracted and translated.
- User-facing metadata such as titles, descriptions, and SEO fields was translated and coordinated for deployment.
- Comments and tracked changes were handled deliberately, translated or stripped, not left silently in the source language.
- Conditional, dynamic, branching, and error-state content was captured from resource files or full scenario walks, not from a single screenshot.
- Text embedded in images, charts, diagrams, and infographics was identified and translated or recreated with design coordination.
- A final check confirmed no translatable content remains in the source language, with any exceptions documented.
- The deliverable would not surprise a user with sudden source-language fragments in tooltips, errors, images, or metadata.
