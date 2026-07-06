---
name: translation-and-format-deliverables.md
description: Use when the agent is preparing final masters for release, exporting deliverables for streaming, CD, vinyl, or sync, managing file formats and sample rates, creating DDP images or ISRC codes, or evaluating whether masters translate correctly across all target playback formats and meet distribution specifications.
---

# Translation and Format Deliverables

A finished master is not a single file; it is a set of deliverables, each tailored to a distribution format with its own technical specifications, and a failure at the deliverable stage can delay a release or cause it to be rejected. The judgment problem is that engineers often focus on the creative mastering and treat the deliverable export as a mechanical final step, then discover that the streaming master is the wrong sample rate, the CD master has incorrect gaps, or the vinyl pre-master was not prepared for the cutting engineer. Each format — streaming, CD, vinyl, download, sync — has specific requirements for sample rate, bit depth, loudness, file naming, metadata, and sequencing, and a master that sounds perfect can still fail to ship if the deliverables do not meet the spec. This skill covers the decisions that govern translation and deliverables: format-specific preparation, file specifications, metadata, sequencing, and verification against distribution requirements.

## Core Rules

### Prepare Format-Specific Masters Rather Than One File for All

Different formats require different mastering approaches, not just different exports. A streaming master targets platform loudness and preserves dynamics; a CD master may be louder and is delivered as 16-bit/44.1 kHz; a vinyl pre-master requires mono low end, no ultrasonic content, and careful sequencing for side lengths; a sync master may need alternate versions (instrumental, stems, clean edits). The decision is to prepare each format's master with its specific requirements in mind, rather than assuming one master suits all. The criterion is that each deliverable is optimized for its target medium and meets its technical spec.

### Match Sample Rate, Bit Depth, and Format to the Deliverable Spec

Each distribution format specifies sample rate, bit depth, and file format, and non-conforming files are rejected or auto-converted (often degrading quality). Streaming platforms accept various sample rates but have preferences; CD requires 16-bit/44.1 kHz WAV; high-resolution downloads specify higher rates. The decision is to deliver files at the exact specification required, using high-quality sample rate conversion and dithering when reducing bit depth. The trap is delivering a 24-bit/96 kHz file where 16-bit/44.1 kHz is required, or applying bit depth reduction without dither, introducing quantization distortion. Confirm the spec, convert properly, and verify the final file's properties.

### Apply Correct Dither When Reducing Bit Depth

When reducing bit depth (e.g., from 24-bit master to 16-bit CD), quantization error introduces distortion; dither adds low-level noise that randomizes the error and preserves the sense of dynamic detail. The decision is to apply appropriate dither (typically TPDF dither or a noise-shaped variant) as the very last process before bit depth reduction, and to apply it only once. The trap is applying dither multiple times (e.g., in the master and again in the export), or skipping dither and accepting quantization distortion on quiet material. Set dither as the final stage and ensure it is not duplicated in the signal chain.

### Sequence Tracks With Correct Gaps for the Format

Album sequencing includes the gaps between tracks, which differ by format: CD uses gaps measured in seconds (often 2 seconds default, adjustable); streaming platforms treat tracks as independent files with no crossfades unless designed as such; vinyl requires side breaks and consideration of groove spacing. The decision is to sequence deliberately for each format, with gap lengths that serve the album flow and meet the format's constraints. The trap is applying CD-style gaps to streaming masters (creating silence that streaming does not expect) or ignoring vinyl side length limits (causing the cutting engineer to compress dynamics to fit). Plan sequencing per format and verify gap lengths in the deliverable.

### Embed Metadata and Identification Correctly

Metadata — ISRC codes, track titles, artist names, album title, UPC/EAN — identifies the release for royalty tracking and distribution, and missing or incorrect metadata causes lost royalties or distribution rejection. The decision is to embed the required metadata in the deliverable files and to verify it against the distribution aggregator's requirements. ISRC codes are unique per track and must be correct for royalty collection; UPC/EAN identifies the release. The trap is omitting metadata, entering it inconsistently across formats, or assuming the distributor will add it. Embed metadata in the master files and confirm it reads correctly in a metadata viewer.

### Prepare Alternate Versions Required for Sync and Licensing

Sync placements (film, TV, advertising) and many licensing deals require alternate versions: instrumental, clean (no profanity), radio edit, stems (separated instrument groups). The decision is to prepare these alternates as part of the deliverable package, mastered to match the main version, so they are available when requested without emergency rework. The trap is delivering only the main master and scrambling to produce alternates when a sync opportunity arises on deadline. Produce the standard alternates (instrumental, clean, stems) alongside the main master, and verify they match in level and tone.

## Common Traps

### Delivering One Master for All Formats

The engineer exports the same master file for streaming, CD, and vinyl, assuming it suits all. The false signal is that the master sounds good. The harm is a streaming master that is too loud and gets turned down, a CD that lacks the format's expected loudness, or a vinyl cut that suffers from ultrasonic content and stereo low end. The mechanism is ignoring format-specific requirements. The remedy is per-format mastering.

### Wrong Sample Rate or Bit Depth for the Spec

The deliverable is exported at the master's native rate rather than the format's required rate. The false signal is that the file plays. The harm is rejection by the distributor or auto-conversion that degrades quality, or a CD that cannot be replicated. The mechanism is not confirming the spec. The fix is to verify the required rate and depth and convert properly.

### Skipping or Duplicating Dither

Bit depth reduction is done without dither, or dither is applied at multiple stages. The false signal is that the file sounds fine on loud material. The harm is quantization distortion on quiet material and fades, or accumulated noise from duplicated dither. The mechanism is treating dither as optional or redundant. The remedy is single, correct dither as the final stage.

### Ignoring Sequencing Differences Across Formats

CD gaps are applied to streaming files, or vinyl side lengths are not considered. The false signal is that the sequence is "the album." The harm is streaming files with unexpected silence, or vinyl sides that exceed length and force dynamic compromise at cutting. The mechanism is treating sequencing as universal. The fix is per-format sequencing.

### Omitting or Entering Incorrect Metadata

Metadata is left blank or entered inconsistently, assuming the distributor handles it. The false signal is that the audio is what matters. The harm is lost royalties (no ISRC), distribution rejection, or inconsistent identification across platforms. The mechanism is treating metadata as administrative overhead. The remedy is to embed and verify metadata.

### Failing to Prepare Alternate Versions

Only the main master is delivered, and sync or licensing requests require alternates that do not exist. The false signal is that the main master is complete. The harm is missed opportunities or emergency rework under deadline when alternates are requested. The mechanism is not anticipating downstream needs. The fix is to produce standard alternates with the main master.

## Self-Check

- Did I prepare format-specific masters for each target (streaming, CD, vinyl, sync), optimized for that medium rather than exporting one file for all?
- Did I deliver files at the exact sample rate, bit depth, and format required by each distribution spec, using proper conversion where needed?
- Did I apply correct dither as the final single stage before bit depth reduction, without duplication?
- Did I sequence tracks with gap lengths appropriate to each format, considering streaming independence, CD gaps, and vinyl side limits?
- Did I embed complete and correct metadata (ISRC, UPC/EAN, titles, artist) and verify it reads correctly in a metadata viewer?
- Did I prepare the standard alternate versions (instrumental, clean, stems) mastered to match the main version, so sync and licensing requests can be met without rework?
- If I submitted these deliverables to a distributor or cutting engineer, would they pass technical inspection, or would they be rejected for spec violations?
