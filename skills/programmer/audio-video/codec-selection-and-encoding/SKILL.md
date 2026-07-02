---
name: codec_selection_and_encoding.md
description: Use when the agent is choosing a video or audio codec (H.264, H.265/HEVC, AV1, VP9, AAC, Opus, FLAC), setting bitrate, CRF, or encoding presets, configuring hardware acceleration (NVENC, QuickSync, VideoToolbox), transcoding media, building an encoding pipeline, or debugging quality, file size, compatibility, or encoding-speed problems. Covers lossy vs lossless tradeoffs, rate control, preset and tune choices, hardware encoder quality gaps, and the codec compatibility matrix across players and devices.
---

# Codec Selection And Encoding

Codec selection is a decision with permanent consequences: once media is encoded and distributed, the codec, the bitrate, and the encoder choices are baked in, and "we should have used AV1" or "the bitrate was too low" is a costly re-encode or a re-shoot. The decision is governed by three forces that pull against each other — quality at a given bitrate (efficiency), decode compatibility across the target devices and players, and encoding cost (CPU time, or hardware-encoder availability and quality) — and optimizing any one in isolation produces a bad result. AV1 gives the best quality per bitrate but encodes slowly and is not universally decodable; H.264 encodes fast and plays everywhere but is half as efficient; hardware encoders are fast but historically lag software encoders in quality at a given bitrate. The difficulty is that the tradeoffs are quantitative (how much quality for how much size, on which devices, at what encode cost) and the wrong choice is only visible after distribution, when the artifacts are in front of users or the file does not play on a target device.

Agents tend to pick a codec by familiarity ("use H.264, it's standard") and treat encoding as a single CLI invocation with default settings, because the happy path produces a playable file and the failures (banding in gradients, blocking in dark scenes, audio that sounds underwater, files that will not play on Safari or an old Android) only appear on specific content or specific devices. The judgment problem is deciding which codec fits the deployment matrix, what rate control and preset balance quality against size and encode cost, whether hardware acceleration is acceptable for the quality target, and how to verify the result across the target devices before shipping. Getting this wrong produces media that is too large, too low-quality, incompatible, or too expensive to encode at scale.

## Core Rules

### Choose The Codec Against The Compatibility Matrix First, Then Efficiency

A codec that will not decode on the target devices is useless regardless of its efficiency. The first question is: what devices, browsers, and players must decode this media? Then pick the most efficient codec that all of them support.

- **H.264/AVC** is the universal baseline: every modern browser, phone, set-top box, and editing tool decodes it. The safe default when compatibility is paramount. Less efficient than newer codecs (roughly half the compression of HEVC/AV1 at the same quality).
- **H.265/HEVC** is roughly 2x more efficient than H.264 but has uneven browser support (Safari yes, Chrome depends on OS and licensing) and patent-licensing complexity. Strong for app-embedded playback, native apps, and known-device deployments; risky for open web without a fallback.
- **AV1** is the modern royalty-free codec, more efficient than HEVC, with growing hardware decode support and broad browser support (Chrome, Firefox, recent Safari). Encode is slow in software; hardware encoders are emerging. Strong for web video at scale where the decode matrix supports it.
- **VP9** is the predecessor to AV1, royalty-free, broadly supported in browsers, less efficient than AV1. A reasonable fallback.
- **Audio**: AAC is the universal baseline (everywhere, good quality above 128 kbps stereo); Opus is more efficient and lower-latency (strong for real-time and modern streaming, supported in browsers and WebRTC); FLAC and ALAC are lossless for archival.

For audio, do not re-encode lossy to lossy in a pipeline; each transcode loses quality. Keep a lossless master and encode to each target format from it.

### Use Rate Control Deliberately: CRF For Quality, Two-Pass For Size, CBR For Streaming

Rate control decides how the encoder spends bits, and the choice should match the goal.

- **CRF (constant rate factor)** targets a perceptual quality level and lets the bitrate vary with content complexity. Best for archival, single-file delivery, and any case where you want "this looks good" without caring about exact size. Use CRF for the master encode; the right CRF (typically 18-23 for H.264, higher for HEVC/AV1) is chosen by visual inspection against a representative clip.
- **Two-pass VBR** targets a specific average bitrate (or file size) and distributes bits across the whole file based on a first-pass analysis. Best for size-constrained delivery (a fixed file size, a bandwidth budget) where you also want quality-aware bit allocation.
- **CBR (constant bitrate)** keeps the bitrate fixed, which is sometimes required by transport constraints (certain streaming protocols, broadcast) but is quality-inefficient (simple scenes waste bits, complex scenes starve). Use only when the transport demands it.

The common error is using a single bitrate for everything. Content complexity varies enormously — a static talking-head video compresses far better than confetti or water — and a fixed bitrate either wastes bits on the simple parts or starves the complex parts. CRF or two-pass adapts; fixed bitrate does not.

### Tune The Preset Against The Encode-Cost vs Quality Tradeoff

Encoders expose presets (x264's `ultrafast` to `veryslow`, SVT-AV1's 0-13, similar in other encoders) that trade encode speed for quality at a given bitrate. Slower presets use more sophisticated motion estimation and rate-distortion optimization to find a better encode, typically saving 5-20% bitrate at the same quality for a large multiple of the encode time.

Choose the preset against the encode budget and the distribution scale. For a one-off archival encode, use the slowest preset you can afford. For a high-volume transcoding pipeline (user uploads, live-to-VOD), the per-file encode cost matters and a faster preset may be the right economic choice even at a quality cost. The default preset is rarely optimal for either extreme; measure the quality-vs-speed curve for your content and pick deliberately. Note that the slowest preset is not always the best value — the quality gain per unit of extra encode time diminishes, and the knee of the curve is usually a mid-range preset.

### Know The Quality Gap Between Hardware And Software Encoders

Hardware encoders (NVENC on NVIDIA, QuickSync on Intel, VideoToolbox on Apple, AMF on AMD) are dramatically faster than software encoders and are essential for real-time encoding (streaming, live transcoding). Historically they have also been less quality-efficient than software encoders (x264, x265, SVT-AV1) at a given bitrate — older NVENC in particular was visibly worse than x264 `medium` at the same bitrate. Newer hardware encoders (NVENC with two-pass mode, newer Intel QSV) have closed much of the gap, but a quality difference often remains at low bitrates.

Decide explicitly whether hardware acceleration is acceptable for the quality target. For live streaming where encode latency is the constraint, hardware is the answer. For archival or premium VOD where quality per bit matters, software encoding at a slow preset is often worth the cost. Measure both on representative content at the target bitrate; do not assume hardware matches software quality, and do not assume the gap is large — it depends on the generation and settings.

### Keep A Lossless Master And Encode Each Target From It

Every lossy transcode degrades quality, and the degradation compounds: transcoding H.264 to H.264 again, or H.264 to HEVC, loses information that cannot be recovered. A pipeline that re-encodes already-compressed media for each delivery format produces visibly degraded output after a few generations.

Maintain a lossless or near-lossless master (ProRes, DNxHR, FFV1 for video; FLAC/WAV for audio) and encode every delivery format from that master. For user-generated content that arrives already-compressed, store the original as the master and encode delivery variants from it; do not transcode delivery variants from other delivery variants. The master is the source of truth; everything else is a derived lossy copy.

### Verify Quality And Compatibility Before Distribution

Codec and bitrate choices are hypotheses until verified. A CRF that looks fine on a talking-head clip may block badly on high-motion content; a codec that plays in Chrome may not play in Safari or on an old Android; a hardware encode may band in gradients that a software encode handles cleanly.

Verify with: visual inspection of representative clips (including hard content — dark scenes, gradients, high motion, fine detail), objective quality metrics (VMAF, PSNR) against a reference where useful, and playback tests across the target device matrix (the browsers, OSes, and hardware you support). Automated VMAF scoring across a content corpus catches regressions when encoder settings change. A codec/bitrate choice that was not verified on the target devices and hard content is untested.

### Account For The Compatibility Matrix Across The Whole Pipeline, Not Just Decode

Decode compatibility is the obvious constraint, but the pipeline also includes: container format support (MP4 is broadly supported; WebM for VP9/AV1 in browsers; MOV for Apple ecosystems), color space and HDR metadata (BT.709 for SDR, BT.2020/PQ for HDR, with device support varying), audio format pairing (AAC in MP4, Opus in WebM), and subtitle/caption formats. A codec choice that the decoder supports may still fail if the container, color metadata, or audio pairing is wrong for the target.

Specify the full delivery profile — codec, container, audio, color, captions — and verify it end-to-end on each target device, not just "does the video decode."

## Common Traps

### Defaulting To H.264 Without Considering The Compatibility vs Efficiency Tradeoff

Using H.264 for everything because it is familiar, shipping files 2x larger than HEVC or AV1 would produce at the same quality, when the target devices support the more efficient codec. Pick the most efficient codec the device matrix supports.

### Using A Single Fixed Bitrate For All Content

Encoding every video at the same target bitrate, wasting bits on simple content and starving complex content. Use CRF for quality-targeted encodes or two-pass for size-targeted encodes; let the bitrate adapt to complexity.

### Picking The Slowest Preset "For Quality" Without Measuring

Choosing `veryslow` or SVT-AV1 0 for a high-volume pipeline and paying 10x the encode cost for a 5% quality gain that is below the knee of the value curve. Measure the quality-vs-speed curve; the optimal preset is usually mid-range, not the slowest.

### Assuming Hardware Encoder Quality Matches Software

Using NVENC/QSV/VideoToolbox for premium VOD and shipping visibly lower quality than x264/SVT-AV1 at the same bitrate, because the hardware encoder's quality gap was not measured. Measure both at the target bitrate; use hardware where latency demands it, software where quality per bit matters.

### Re-Encoding Lossy To Lossy

Transcoding delivery variants from other delivery variants (or re-encoding uploaded H.264 to HEVC for delivery), compounding quality loss across generations. Keep a lossless or original master; encode every delivery format from the master.

### Verifying Only On Easy Content

Tuning CRF/bitrate on a talking-head clip and shipping banding, blocking, or smearing on dark scenes, gradients, or high motion. Verify on representative hard content, not just the easy clips.

### Checking Decode Support But Not The Full Delivery Profile

Confirming the codec decodes in the target browser but shipping a file that fails because the container, audio format, color metadata, or caption track is unsupported. Verify the full delivery profile end-to-end on each target device.

### Ignoring Audio Codec Pairing

Pairing an efficient video codec with an inefficient or incompatible audio codec (e.g., 256 kbps AAC when Opus at 96 kbps would match the quality), or an audio codec the container does not support. Match audio to video in both efficiency and container compatibility.

## Self-Check

- [ ] The codec was chosen against the target decode compatibility matrix first, then optimized for efficiency; the most efficient codec all target devices support is in use, with a documented fallback where needed.
- [ ] Rate control matches the goal: CRF for quality-targeted encodes (with the CRF chosen by visual inspection of representative content), two-pass VBR for size-constrained delivery, CBR only where transport demands it.
- [ ] The encoding preset was chosen against the encode-cost vs quality curve for the content and distribution scale, measured rather than defaulted to the slowest or fastest.
- [ ] Hardware acceleration is used where encode latency demands it (live) and software encoding where quality per bit matters (archival, premium VOD); the quality gap was measured at the target bitrate, not assumed.
- [ ] A lossless or original master is retained, and every delivery format is encoded from the master — no lossy-to-lossy transcoding in the pipeline.
- [ ] Quality was verified by visual inspection and/or objective metrics (VMAF) on representative hard content (dark scenes, gradients, high motion, fine detail), not only on easy clips.
- [ ] Playback was tested end-to-end across the target device matrix (browsers, OSes, hardware), covering the full delivery profile — codec, container, audio pairing, color/HDR metadata, and captions — not just video decode.
- [ ] Audio codec choice matches the video codec in both efficiency and container compatibility, and audio bitrate was chosen against perceptual quality rather than habit.
