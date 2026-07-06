---
name: digitization_capture_standards.md
description: Use when the agent is setting technical capture standards for digitization, choosing resolution and bit depth for scanning, establishing master and derivative specifications, defining quality control for digital captures, or selecting equipment and settings for different material types.
---

# Digitization Capture Standards

Digitization capture standards are the technical decisions that determine whether a digital surrogate is fit for purpose, both now and decades from now. Resolution, bit depth, color management, file format, and compression are not interchangeable settings; they determine how much of the original's information the capture preserves, and once a substandard capture is made, the lost information cannot be recovered without re-digitizing. The dominant failure mode is capturing for immediate access only, producing derivatives that look acceptable on screen but lack the fidelity for future uses, scholarly analysis, or preservation. The complementary failure is over-specifying uniformly, wasting storage and effort on materials that do not warrant archival capture. Good capture standards are purpose-driven: they define preservation masters at fidelity appropriate to the material's value and use, derive access copies from those masters, apply consistent quality control, and document the specifications so captures are reproducible and defensible.

Use this skill when setting digitization capture specifications, choosing resolution and bit depth, defining master and derivative standards, establishing capture quality control, or selecting equipment settings for material types. The goal is to prevent the agent from capturing only for access, applying one-size-fits-all specifications, ignoring color management, or skipping quality control on captures.

## Core Rules

### Capture Preservation Masters At Fidelity Appropriate To The Material

The preservation master is the archival capture that preserves the maximum useful information from the original. Its specifications should match the material's value, use, and physical characteristics.

Master specifications by material:

- textual documents, 300 to 600 dpi, 24-bit color or 8-bit grayscale, lossless TIFF;
- photographs and prints, 600 dpi minimum, often higher for small or detailed items, 24-bit color;
- maps and large-format, 300 to 400 dpi at original size, 24-bit color, often requiring stitching;
- negatives and transparencies, 4000 dpi or higher at original size, 16-bit color depth;
- audio, 96 kHz, 24-bit, uncompressed WAV or BWF;
- video, uncompressed or high-bitrate lossless, preserving original interlace and frame rate.

Match the specification to the material. Under-capturing a detailed photograph loses information; over-capturing a plain typescript wastes resources.

### Distinguish Masters From Access Derivatives

Preservation masters and access copies serve different purposes and need different specifications. Never let an access derivative become the only copy.

Separation:

- masters, lossless, high-fidelity, large files, stored for preservation, rarely accessed directly;
- access derivatives, compressed, web-friendly, smaller files, served to users;
- derivatives generated from masters, not captured separately;
- masters protected from editing or degradation.

A JPEG access copy is not a master. If the master is lost or never made, the institution has an access image, not a preservation asset.

### Manage Color And Tone Faithfully

Color and tone fidelity determine whether the digital surrogate represents the original accurately. Poor color management produces captures that look wrong and cannot be trusted for scholarly or comparative use.

Color management:

- use calibrated monitors and color-managed scanners or cameras;
- capture and embed ICC color profiles in masters;
- use color targets, such as a Q13 or ColorChecker, in capture where appropriate;
- capture in a consistent color space, often Adobe RGB or ECI RGB for masters;
- document the color management workflow.

Without color management, captures vary by equipment and operator, and the surrogate cannot be trusted to represent the original.

### Choose File Formats For Longevity And Quality

File format choices at capture determine long-term accessibility and quality retention. Prefer open, well-documented, lossless formats for masters.

Format choices:

- images, TIFF for masters, JPEG or JPEG2000 for derivatives;
- audio, WAV or BWF for masters, MP3 or FLAC for derivatives;
- video, lossless or high-bitrate formats for masters, H.264 for derivatives;
- text, PDF/A for documents with structure, plain text or XML for reflowable content;
- avoid proprietary or undocumented formats for masters.

Format is a preservation decision, not just a capture decision. See the file-format-strategy skill for deeper guidance.

### Apply Quality Control To Every Capture

Capture introduces errors: focus issues, color casts, cropping, stitching artifacts, dust, and skew. Quality control catches these before masters are accepted.

QC checks:

- focus and sharpness at 100 percent zoom;
- color accuracy against the target and original;
- tonal range, no blown highlights or blocked shadows;
- complete capture, no cropping of significant content;
- skew and orientation corrected;
- stitching artifacts for large-format captures;
- dust and scratches documented as present in original versus introduced.

QC at capture prevents accepting masters that fail to preserve the original. A capture that passes QC is the foundation; one that does not must be recaptured.

### Match Equipment And Settings To Material Type

Different materials require different capture equipment and handling. Using the wrong equipment damages materials or produces poor captures.

Equipment by material:

- bound materials, overhead or cradle scanners that protect bindings;
- flat documents, flatbed or planetary scanners;
- photographs and transparencies, film scanners or high-resolution camera setups;
- large-format, oversized scanners or stitched camera captures;
- audio, professional digitization interfaces and decks;
- video, broadcast-quality decks and capture cards.

Match the equipment to the material's physical needs and to the required specifications. A flatbed scanner suited to flat paper is wrong for a fragile bound volume.

### Handle Materials Safely During Capture

Digitization handling can damage fragile materials. Safe handling is part of capture standards, not a separate concern.

Safe handling:

- use book cradles and supports for bound materials;
- avoid pressing glass platens onto fragile or emulsion surfaces;
- handle photographs by edges with clean or gloved hands per institutional policy;
- support large or fragile items fully during capture;
- train operators in safe handling for each material type;
- stop capture if handling risks damage and consult conservation.

A capture that damages the original has failed its purpose. Preservation of the physical original is paramount.

### Document Capture Specifications And Provenance

Capture specifications and provenance must be documented so masters are reproducible and trustworthy over time.

Document:

- the capture specifications, resolution, bit depth, color space, format;
- the equipment and software used;
- the operator and date;
- any post-capture processing, cropping, rotation, color correction;
- the relationship between master and derivatives;
- QC results and any accepted defects.

Documentation turns a capture into a defensible preservation asset. Without it, future users cannot assess the surrogate's fidelity to the original.

## Common Traps

### Capturing Only For Access

Access derivatives lack archival fidelity. Capture preservation masters and derive access from them.

### One-Size-Fits-All Specifications

Uniform specs over-capture some materials and under-capture others. Match specifications to material type and value.

### Ignoring Color Management

Unmanaged color produces untrustworthy captures. Calibrate, use profiles, and capture targets.

### Lossy Formats For Masters

Lossy compression discards unrecoverable information. Use lossless formats for preservation masters.

### Skipping Quality Control

Uncaptured errors, focus, color, cropping, persist in masters. QC every capture.

### Wrong Equipment For The Material

Mismatched equipment damages materials or produces poor captures. Match equipment to physical and specification needs.

### Handling That Damages Originals

Capture that damages the original fails its purpose. Prioritize safe handling and conservation.

### Undocumented Capture

Undocumented masters cannot be assessed for fidelity. Record specifications, equipment, processing, and QC.

## Self-Check

- Are preservation masters captured at resolution, bit depth, and color depth appropriate to the material's value and use?
- Are masters and access derivatives clearly separated, with derivatives generated from masters?
- Is color and tone managed through calibration, ICC profiles, and capture targets?
- Are file formats open, well-documented, and lossless for masters?
- Is quality control applied to every capture, checking focus, color, tonal range, cropping, and artifacts?
- Is capture equipment matched to the material type's physical and specification needs?
- Are fragile materials handled safely with appropriate supports and trained operators?
- Are capture specifications, equipment, processing, and QC results documented for reproducibility and trust?
