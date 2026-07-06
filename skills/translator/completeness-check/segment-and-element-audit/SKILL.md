---
name: segment_and_element_audit.md
description: Use when the agent is auditing a translation at the segment and element level to confirm one-to-one correspondence between source and target, checking that segments list items table cells tags placeholders and inline elements align correctly with nothing merged split misplaced or orphaned.
---

# Segment And Element Audit

Modern translation happens in segments: units defined by a tool, a file format, or a markup structure. Segments are supposed to map cleanly to target units, but in practice they break in ways that corrupt the deliverable even when every word is correct. Two source segments get merged into one target segment, losing a boundary that mattered. One segment splits into two, breaking references and translation memory. A list item shifts position so that item four in the source aligns with item five in the target. A table cell is translated into the wrong cell. A tag or placeholder gets dropped, moved, or duplicated, breaking the formatting or the code that consumes the text. An inline element, a bold marker, a hyperlink, a variable, ends up orphaned or wrapped around the wrong words. These are segment and element level defects, and they are dangerous because they are invisible to a fluent read of the target text. The prose may be perfect while the structure is broken. An audit at this level is the discipline of confirming that every source segment and element has exactly one correctly placed counterpart in the target, that boundaries are preserved, and that markup is intact. This skill covers how to audit segments and elements, what defect types to look for, and how to verify structural integrity without relying on the target text reading well.

Use this skill when auditing a translated file for structural integrity, when verifying segment and element correspondence, when checking tag and placeholder integrity, or when confirming that lists, tables, and inline elements align correctly. The goal is to prove the target is structurally sound, not merely well-written.

## Core Rules

### Verify One-To-One Segment Correspondence

The foundation of a segment audit is confirming that source and target segments correspond one to one, in order, with no merges, splits, or losses.

For each source segment, confirm there is a target segment, that the target segment corresponds to that source and not an adjacent one, and that the boundary is preserved. Merged segments, where two source segments become one target, lose segmentation that translation memory, search, and reference systems depend on, and they often collapse distinct meanings. Split segments, where one source becomes two targets, break alignment and can orphan tags. Confirm segment counts match unless a documented, authorized resegmentation occurred. In files where segmentation is tool-defined, be alert to cases where the tool segmented poorly and the translator worked around it by merging or splitting silently; these need verification, not just acceptance.

### Check List And Enumeration Integrity

Lists are where alignment defects are most common and most damaging, because order and count carry meaning. Audit lists element by element.

Confirm that the target list has the same number of items as the source, in the same order, with each item corresponding to the correct source item. Watch for items dropped because they seemed redundant, items merged because they were short, items reordered to sound better, and numbering that drifted. In numbered lists, verify the numbering itself is intact and consecutive. In nested lists, verify the nesting level of each item is preserved, because a sub-item promoted to a top-level item changes the structure's meaning. In lists where items cross-reference each other or are referenced elsewhere, misalignment breaks the references. Count the items and verify position, do not trust the visual impression.

### Audit Tables Cell By Cell

Tables concentrate alignment risk because content sits in a grid where correct words can land in wrong cells. Audit tables cell by cell, not row by row.

For each source cell, confirm the target cell in the same row and column contains the corresponding translation. Watch for cells left untranslated because they held numbers or codes, cells whose content shifted during editing, merged cells that absorbed neighbors, and header cells that define the table's meaning. Verify that row and column headers, which often hold short labels, are translated consistently with how they are used in the body. Confirm that any units, totals, or formulas reference the correct cells after translation. A table where every cell is translated but two cells are swapped is structurally defective even though no content is missing.

### Preserve Tag And Placeholder Integrity

Tags, formatting markers, and placeholders are structural elements that must survive translation intact and in the right place. Audit them deliberately.

Confirm that every tag in the source segment appears in the target segment, that no tag was dropped, duplicated, or corrupted, and that tags wrap the correct span of text. Placeholders, variables, and interpolation markers must remain valid: a placeholder renamed or reformatted will break the code that substitutes it. Inline formatting markers such as bold, italic, and link tags must enclose the equivalent target text, not orphan fragments or wrap the wrong words. In software strings, confirm that placeholders preserve their position relative to grammar, because moving a placeholder can produce ungrammatical output in languages with different word order. Use the tool's tag-validation if available, but also read the segments to confirm tags sit correctly around meaning, because validation checks syntax, not semantics.

### Guard Against Orphaned And Misplaced Inline Elements

Inline elements such as hyperlinks, footnotes references, index markers, and emphasis spans belong to specific words or phrases. Audit that they attach to the correct target text.

A common defect is the orphaned element: a footnote marker that ends up at the end of a sentence instead of after the term it marks, a hyperlink that wraps the wrong phrase, an index entry pointing to a translated term that no longer matches. These defects change meaning and break navigation. For each inline element, confirm it sits at the equivalent position in the target, attached to the equivalent content. Where word order changes between languages, reposition the element deliberately to preserve its reference, and note where repositioning was necessary.

### Detect Silent Resegmentation And Boundary Changes

Translators sometimes change segment boundaries to make drafting easier, merging short segments or splitting long ones. Detect these silent changes because they affect alignment, memory, and references.

Compare source and target segment counts at the file level first; a mismatch signals resegmentation. Then investigate the mismatched regions to confirm whether the change was authorized and whether it preserved meaning. Unauthorized resegmentation can corrupt translation memory matches for future work and break cross-references that assume the original segmentation. Where resegmentation is legitimate, for example to handle a segmentation error in the source file, document it so downstream tools and reviewers understand the deviation.

### Re-Verify Structure After Any Edit

Segment and element integrity is fragile; a late edit can reintroduce a defect the audit had cleared. Re-verify structure after corrections.

When a reviewer requests a wording change, the correction can move a tag, drop a placeholder, or shift a list item. When post-editing machine translation, an editor may delete a segment boundary or merge cells. Treat the structural audit as the final gate, run after all linguistic edits, so that the last change did not break what the earlier audit confirmed.

## Common Traps

### Trusting Fluent Target Text Over Structure

Perfect prose with broken segmentation, misaligned cells, or dropped tags is a defective deliverable; structure must be audited separately.

### Merging Or Splitting Segments Silently

Unauthorized resegmentation corrupts alignment, memory, and references; document any boundary changes.

### Reordering Or Dropping List Items

List order and count carry meaning; verify item count and position, do not trust visual impression.

### Swapping Cells In Tables

Correct words in wrong cells is a structural defect; audit cell by cell in the same row and column.

### Dropping Or Misplacing Tags And Placeholders

Tags must be intact and correctly placed; validation checks syntax but not whether tags wrap the right meaning.

### Orphaning Inline Elements

Footnote markers, links, and index entries must attach to the equivalent target text, not drift to sentence ends.

### Skipping Re-Verification After Edits

Late corrections can reintroduce structural defects; run the element audit as the final gate.

## Self-Check

Before approving a translated file on structural grounds, verify:

- Source and target segments correspond one to one, in order, with no unauthorized merges, splits, or losses.
- Segment count matches the source unless a documented, authorized resegmentation occurred.
- Lists have the correct item count, order, numbering, and nesting level, verified element by element.
- Tables are audited cell by cell, with each target cell matching its source row and column and no swapped content.
- All tags, formatting markers, and placeholders are intact, uncorrupted, and correctly placed around equivalent text.
- Placeholders preserve valid syntax and grammatically correct position, especially across differing word orders.
- Inline elements such as links, footnote markers, and index entries attach to the correct target text.
- Silent resegmentation and boundary changes were detected, investigated, and documented where legitimate.
- Structure was re-verified after the final linguistic edit, so late changes did not reintroduce defects.
- The audit proves structural integrity rather than assuming it from the target text reading well.
