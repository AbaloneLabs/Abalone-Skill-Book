---
name: bilingual_and_side_by_side.md
description: Use when the agent is producing bilingual or side-by-side deliverables for review legal comparison or client verification, aligning source and target segments row by row, handling expansion and column layout in two-language documents, managing directionality when source and target use different scripts, or ensuring bilingual files support review without leaking content or breaking correspondence.
---

# Bilingual And Side-By-Side

A bilingual or side-by-side deliverable presents source and target together so that a reviewer, a client, or a legal authority can compare the translation against the original segment by segment. This format is essential for review workflows, for certified and legal translations where correspondence must be auditable, and for clients who do not read the target language but must verify completeness and fidelity. It is also a format where many things go wrong, because presenting two languages together introduces alignment, layout, directionality, and confidentiality problems that a monolingual deliverable never faces. Agents miss these issues because a bilingual file looks straightforward, two columns of text, and the problems are structural rather than linguistic: misalignment that breaks the source-target correspondence, expansion that destroys the column layout, mixed script directionality that visually collides, and source content that leaks into a deliverable meant to be target-only. The harm this skill prevents is bilingual deliverables that fail their purpose: misaligned rows that make comparison impossible, broken layouts that make the file unusable, or correspondence errors that undermine the legal or review value the format was created to provide.

Use this skill when producing bilingual or side-by-side deliverables, aligning source and target for review or comparison, handling two-language layout and expansion, managing mixed-script directionality, or ensuring bilingual files support their intended review or legal purpose without leaking content or breaking correspondence. The goal is a bilingual deliverable where every target segment is unambiguously paired with its source and the layout supports accurate comparison.

## Core Rules

### Maintain Strict Segment-Level Source-Target Alignment

The entire value of a bilingual deliverable rests on the correspondence between each source segment and its target. If the alignment drifts, a reviewer comparing row by row sees the wrong source against the wrong target, and the comparison becomes worse than useless because it generates false findings. Alignment must be established through consistent segmentation: the same segmentation rules applied to source and target, so that each row contains one source segment and its corresponding target segment. Where the target requires structural changes that break one-to-one correspondence, a source sentence split into two target sentences or two source sentences merged into one, the alignment must be explicitly managed, either by repeating the source segment, by using empty cells with a note, or by a merge or split marker that makes the non-one-to-one relationship visible. Never let alignment drift silently, because a misaligned bilingual file is structurally defective even if every target segment is individually correct. Verify alignment after translation and again after any layout adjustment, because layout operations can disturb row correspondence.

### Design The Layout To Accommodate Expansion And Contraction

Two languages in one document means two different text lengths, and the layout must accommodate both. A side-by-side table with fixed column widths will break when the target expands beyond the source length, overflowing cells, wrapping badly, or pushing content out of alignment. Design the layout with expansion in mind: use proportional or flexible column widths, allow rows to grow vertically, and avoid fixed-height containers. Consider which language is longer and give that column appropriate space; in many pairs the target is longer, but in some, such as English to Chinese, the target is shorter and the source column needs more room. Test the layout with representative long segments, not just average-length text, because the worst-case expansion is what breaks the layout. A bilingual layout designed for average text will fail on the first long segment, and the failure is a layout defect that reflects on the whole deliverable.

### Handle Mixed Script Directionality Explicitly

When source and target use different script directions, such as an English source with an Arabic or Hebrew target, the side-by-side layout faces a directionality conflict. The source reads left-to-right and the target reads right-to-left, and placing them in adjacent columns without explicit directionality handling produces visual chaos: text aligned to the wrong edge, punctuation in the wrong position, and reading order that confuses the eye. Handle directionality explicitly by setting the correct text direction for each column, aligning each column to the edge its script reads from, and using directionality marks where mixed-direction content, such as Latin terms inside RTL text, requires them. In some traditions, RTL target text is placed in the right-hand column and LTR source in the left, so that each reads inward from its natural edge; follow the convention appropriate to the audience. Test the rendered output, because directionality problems are invisible in a plain-text or segment view and only appear in the assembled layout.

### Preserve The Review And Audit Purpose Of The Format

A bilingual deliverable exists for a purpose: review, legal comparison, certification, or client verification. The format must serve that purpose, which means it must make comparison easy and correspondence auditable. Use clear visual separation between source and target, consistent row structure, and segment identifiers or numbers that let a reviewer cite a specific segment. For legal or certified translations, follow the format conventions the receiving authority expects, which may include specific column headers, statement of accuracy, or notary-friendly layout. For review workflows, include space or a mechanism for reviewer comments. Do not add elements that undermine the purpose: avoid styling that obscures the correspondence, avoid merging cells in ways that break row-by-row reading, and avoid omitting the source where the purpose requires it. A bilingual file that does not serve its review or legal purpose fails regardless of the quality of the translation within it.

### Prevent Source Content Leakage Into Target-Only Deliverables

A subtle risk of bilingual workflows is that source content leaks into a deliverable meant to be target-only. A bilingual working file is used during translation and review, but the final deliverable for the end audience may need to be monolingual target, with no source visible. If the bilingual file is delivered by mistake, or if source segments remain in a file that should be target-only, the deliverable leaks content the audience should not see, which may include internal notes, draft text, or commercially sensitive source material. Manage the transition from bilingual working format to target-only deliverable explicitly: confirm which deliverables are bilingual and which are target-only, produce the target-only version through a verified export that strips source, and verify the final file contains no residual source content. The bilingual file is a working and review artifact, not necessarily the deliverable, and confusing the two is a confidentiality and completeness error.

### Verify Correspondence After Every Layout Operation

Alignment and correspondence can be disturbed by any layout operation: sorting, filtering, column resizing, page break insertion, or export to a different format. After any such operation on a bilingual file, re-verify that each source segment still corresponds to its target in the same row. A common failure is a sort or filter applied to one column but not the other, which silently breaks the correspondence while leaving both columns individually intact. Treat correspondence as a property that must be verified after every operation, not assumed to persist. For high-stakes bilingual deliverables such as legal or certified translations, perform a final automated or manual correspondence check that confirms every row contains a matched source-target pair with no orphans, duplicates, or misalignments.

## Common Traps

### Letting Alignment Drift Through Structural Changes

Splitting or merging segments during translation without managing the alignment leaves rows where source and target no longer correspond. This is a trap because each segment is individually correct, so the error is invisible at the segment level. Manage splits and merges explicitly and verify alignment.

### Designing Bilingual Layout For Average Text Length

Fixed column widths sized to average text break on the first long segment, overflowing or misalignning the layout. This is a trap because most segments fit and the failure seems exceptional. Design for worst-case expansion and test with long segments.

### Ignoring Directionality In Mixed-Script Side-By-Side

Placing LTR and RTL text in adjacent columns without explicit direction handling produces visual chaos and wrong reading order. This is a trap because the text content is correct. Set direction per column and test the rendered output.

### Undermining The Review Purpose With Poor Structure

Merged cells, inconsistent rows, or missing segment identifiers make a bilingual file hard to review and unsuitable for legal or certified use. This is a trap because the translation is good. Structure the file to serve its review or audit purpose.

### Delivering The Bilingual Working File As The Target-Only Deliverable

Confusing the bilingual working artifact with the target-only final deliverable leaks source content to an audience that should see only the target. This is a trap because the bilingual file looks complete. Confirm which deliverables are target-only and verify source is stripped.

### Assuming Correspondence Survives Layout Operations

A sort, filter, or export applied unevenly breaks source-target correspondence while both columns look intact. This is a trap because the segments are correct individually. Re-verify correspondence after every layout operation.

## Self-Check

Before delivering a bilingual or side-by-side file, verify:

- Every source segment is aligned with its corresponding target segment in the same row, with any splits or merges explicitly managed and visible rather than silently drifted.
- The layout accommodates realistic text expansion and contraction, with flexible column widths and rows tested against worst-case long segments rather than average text.
- Mixed script directionality, where source and target use different directions, is handled explicitly per column with correct alignment, direction marks, and audience-appropriate column placement, verified in rendered output.
- The file structure serves its review, legal, or certification purpose, with clear separation, consistent rows, segment identifiers, and any authority-required format conventions followed.
- The deliverable type, bilingual or target-only, is confirmed, and any target-only deliverable has been produced through a verified export that strips source, with no residual source content.
- Correspondence was re-verified after every layout operation, sort, filter, or export, confirming no orphaned, duplicated, or misaligned rows.
- For legal or certified bilingual deliverables, a final correspondence check confirms every row is a matched source-target pair.
- The rendered bilingual file supports accurate segment-by-segment comparison without layout, directionality, or correspondence defects.
