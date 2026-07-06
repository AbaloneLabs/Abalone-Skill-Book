---
name: digital_reformatting.md
description: Use when the agent is setting digitization specifications, choosing resolution and file formats for masters and derivatives, handling fragile or oversized materials during capture, or ensuring image and audio quality meets archival standards.
---

# Digital Reformatting

Digital reformatting is the technical capture of a physical item into digital files. It looks like operating a scanner, but every specification decision, resolution, color depth, file format, compression, and capture method, determines whether the result can serve access, preservation, and future use. Reformatting done at insufficient quality cannot be recovered later without rescanning, which means rehandling the original. Reformatting done without regard for the item's physical nature can damage fragile materials. Good digital reformatting follows archival standards, captures at quality that supports both present access and future derivation, handles originals safely, and documents the technical choices so the files remain trustworthy over time.

Use this skill when setting digitization specifications, choosing capture methods and formats, handling fragile or unusual materials, or establishing quality standards for a reformatting project. The goal is to prevent the agent from capturing at insufficient quality, using lossy formats for masters, damaging originals through poor handling, or producing files without the technical documentation needed for long-term trust.

## Core Rules

### Capture For The Master, Derive For Access

The foundational principle is to capture once at archival quality, creating a preservation master, and derive access copies from it. The master is the source of truth; access copies are conveniences.

Implications:

- the master must be high enough quality to regenerate any future access copy;
- masters use lossless or uncompressed formats;
- access copies use compressed formats optimized for web delivery;
- never capture directly to a lossy access format as the only copy;
- keep the master untouched and derive from it as needed.

Rescanning is expensive and risks the original. Capture at archival quality the first time.

### Set Resolution By Material Type And Intended Use

Resolution determines how much detail the capture preserves. Too low loses detail; unnecessarily high wastes storage without benefit. Set resolution by material type and use.

Guidelines, adjustable to project standards:

- printed text, 400 to 600 dpi for legibility and OCR;
- photographs and graphics, 600 dpi minimum, higher for small or detailed originals;
- maps and large format, scaled to capture fine detail, often 300 to 600 dpi at original size;
- newspapers and brittle materials, 400 dpi with careful handling;
- film and transparencies, much higher, often 2000 to 4000 dpi equivalent.

Match resolution to the smallest significant detail in the original. A handwritten letter needs enough resolution to read the writing; a fine engraving needs enough to show the lines.

### Choose Color Depth And Color Management Deliberately

Color depth and color management affect whether captured color is accurate and reproducible. Defaults often produce inconsistent or inaccurate color.

Decisions:

- capture masters in 24-bit color or higher, 48-bit for high-end photographic work;
- use grayscale, 8-bit or 16-bit, only when the original is genuinely grayscale;
- calibrate scanners and monitors with color targets;
- embed ICC color profiles in master files;
- capture a color reference target with each item or session for consistency.

Without color management, the same original scanned on different days or devices produces different colors, undermining trust and comparison.

### Use Lossless Or Uncompressed Formats For Masters

Master files must preserve all captured data without lossy compression, because lossy compression discards information that cannot be recovered.

Master formats:

- TIFF for images, the archival standard;
- WAV or FLAC for audio;
- FFV1 or lossless JPEG 2000 for video, depending on capacity;
- PDF/A for text documents when appropriate.

Avoid JPEG, MP3, and other lossy formats for masters. They are acceptable for access derivatives but not for preservation.

### Handle Fragile And Oversized Materials Safely

Reformatting requires physical handling, and poor handling damages originals. The item's safety takes priority over capture speed.

Safe handling practices:

- use book cradles for fragile bindings to avoid stress on spines;
- support oversize items fully to prevent tearing;
- use gloves or clean dry hands per material type, gloves for photographs, hands for paper;
- avoid pressing flat what cannot open flat;
- use transparent overlays for curled items rather than force;
- handle glass plate negatives and film with appropriate supports;
- consult conservation before digitizing severely damaged items.

No digital image is worth damaging the original. If safe capture is not possible, defer until conservation allows it.

### Capture Metadata About The Capture Itself

The technical capture process must be documented as metadata, so future users and preservation systems know how the file was made and can trust or reproduce it.

Capture:

- capture device and settings, scanner model, resolution, color depth;
- capture date and operator;
- software and version used;
- color target and profile;
- any post-capture processing, cropping, rotation, color correction;
- file format and any compression of masters and derivatives.

This technical metadata is administrative metadata, and it is essential for long-term management and migration.

### Implement Quality Control Consistently

Capture without quality control produces errors: cut-off edges, blurred focus, color casts, missing pages, skewed images. QC catches these before they become embedded in the collection.

QC practices:

- review every image at 100 percent for focus and completeness;
- check color against the reference target;
- verify all pages or parts were captured;
- check for skew, cropping errors, and artifacts;
- confirm file naming and metadata match the item;
- sample or fully review derivatives for correct generation.

Decide on full review versus sampling based on the project's value and risk. High-value or one-time captures warrant full review.

### Plan Derivative Generation

Access copies are generated from masters, and the generation process should be consistent and documented.

Plan:

- the derivatives needed, thumbnail, web, zoomable, transcript;
- the formats and compression for each;
- whether OCR text layers are added to text derivatives;
- watermarking or access controls if required;
- naming conventions linking derivatives to masters;
- regeneration triggers, when masters change or formats obsolete.

Automate derivative generation where possible, but verify outputs. Inconsistent derivatives degrade the user experience.

### Handle Audio And Video With Special Care

Audio and video reformatting have additional complexities beyond image capture, including real-time capture, equipment chains, and format obsolescence risks.

For audio:

- capture at high sample rates, 96 kHz, and bit depth, 24-bit, for masters;
- use professional playback equipment calibrated to the original format;
- capture in WAV or FLAC for masters;
- document the playback chain and any noise reduction.

For video:

- capture through professional decks and capture cards;
- use lossless or mezzanine formats for masters;
- document interlacing, frame rate, and color space;
- plan for the heavier storage and preservation burden.

Audiovisual reformatting is specialized and often requires expert staff or vendors. Do not attempt it with consumer equipment.

### Document And Standardize Across The Project

Consistency across a reformatting project depends on documented specifications applied to every item. Drift produces a collection where files cannot be compared or managed uniformly.

Maintain:

- a project specifications document covering all material types;
- capture settings templates for each device;
- naming conventions and folder structures;
- QC checklists and acceptance criteria;
- training for all operators.

Standardization is what turns individual scans into a coherent digital collection.

## Common Traps

### Capturing Only A Lossy Access Copy

JPEG or MP3 as the only copy loses data permanently. Capture lossless masters first.

### Insufficient Resolution

Too-low resolution loses detail that cannot be recovered without rescanning. Match resolution to the original's significant detail.

### No Color Management

Uncalibrated capture produces inconsistent color. Use targets and ICC profiles.

### Damaging Fragile Originals

Poor handling during capture causes permanent damage. Use cradles, supports, and conservation input.

### Skipping Quality Control

Capture errors become embedded without review. Implement QC for every project.

### Missing Technical Metadata

Files without capture documentation cannot be trusted or migrated. Record the capture process.

### Consumer Equipment For Audiovisual

Audio and video need professional capture chains. Consumer gear produces substandard masters.

### Inconsistent Specifications Across The Project

Drift in settings produces an unmanageable collection. Document and enforce standards.

## Self-Check

- Are masters captured at archival quality with lossless or uncompressed formats?
- Is resolution matched to the material type and the smallest significant detail?
- Are color depth and color management applied with calibration and ICC profiles?
- Are fragile and oversized materials handled safely with cradles, supports, and conservation input?
- Is technical capture metadata, device, settings, date, processing, recorded for each file?
- Is quality control implemented, with full review for high-value captures?
- Are access derivatives generated consistently from masters with documented specifications?
- Are audio and video reformatted with professional equipment and lossless master formats?
- Are project specifications documented and enforced across all operators and items?
- Could the masters support regeneration of any future access format without rescanning?
