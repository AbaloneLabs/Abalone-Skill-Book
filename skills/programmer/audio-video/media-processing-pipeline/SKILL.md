---
name: media_processing_pipeline.md
description: Use when the agent is building media processing pipelines, transcoding farms, thumbnail and preview generation, metadata extraction, subtitle and caption processing, chapter segmentation, distributed media workers, or designing processing graphs and job orchestration for video and audio content at scale.
---

# Media Processing Pipeline Design

A media processing pipeline is the invisible backbone of any platform that ingests user-uploaded or professionally produced media. Unlike playback, processing happens asynchronously, often at large scale, and its failures are quiet: a missing thumbnail, a subtitle track that never rendered, a transcode that silently produced a corrupt file. Agents tend to treat processing as "just call ffmpeg" and discover later that the real problems are resource bounding, partial-failure recovery, format variance, and the cost of reprocessing when a step changes.

The judgment problem is that a processing pipeline is a distributed system disguised as a media problem. Each stage (ingest, probe, transcode, thumbnail, metadata, subtitle, package) can fail independently, consume unbounded resources on malformed input, and produce outputs whose correctness cannot be verified by a simple "did it exit zero." The agent must design for idempotency, observability, partial outputs, and reprocessing without re-running expensive steps. The pipeline is judged not by how it handles a clean MP4 but by how it handles a corrupt upload, a 0-byte file, a 4-hour 8K video, or a codec the transcoder has never seen.

## Core Rules

### Treat each processing stage as an independent, idempotent, retryable unit

A pipeline stage should take a deterministic input (source artifact plus parameters plus version) and produce a deterministic output that can be cached and re-used. Design stages so that re-running them with the same input yields the same output and is safe. This lets you retry failed stages without re-running expensive upstream work, and lets you add new outputs (e.g., a new thumbnail size) by re-running only the relevant stage. Store stage outputs keyed by (input hash, stage version, parameters) so the cache is meaningful.

### Probe and validate input before committing resources

Never hand an untrusted upload directly to a transcoder. Probe first (duration, codec, resolution, frame rate, audio layout, container validity) and reject or quarantine malformed input early. Malformed or adversarial media can cause transcoders to consume unbounded CPU, memory, or time. Enforce hard limits on input duration, resolution, bitrate, and file size at the probe stage, and run transcoders with memory and CPU limits plus a wall-clock timeout. A probe that reports a 10,000-frame-per-second video or a negative duration is a signal of a corrupt or hostile file, not a feature to support.

### Bound every stage with resources and a timeout

Transcoding is the classic denial-of-service vector in a media pipeline. A single pathological input can pin a worker for hours. Every stage must run with: a memory limit, a CPU limit, a wall-clock timeout, and ideally a maximum output size. Kill stages that exceed limits and mark the input as failed (or needs-review), not as silently successful. Track per-input resource consumption so you can detect inputs that consistently approach limits and may be malformed.

### Model the pipeline as a graph with explicit dependencies, not a script

Represent the pipeline as a directed graph of stages with declared dependencies (thumbnail depends on probe, all encodes depend on probe, packaging depends on encodes, etc.). A graph representation lets an orchestrator run independent stages in parallel, retry a failed stage without re-running siblings, and reason about partial completion. A shell script that chains commands loses all of this; a failed step in the middle forces a full re-run. Choose or build an orchestrator that supports per-stage retry, parallelism, and dead-letter handling.

### Handle subtitles, chapters, and metadata as first-class, not afterthoughts

Subtitles (SRT, WebVTT, CEA-608/708 embedded captions), chapter markers, and structured metadata are easy to forget because they are small and seem trivial. They are not. Caption timing drift, missing forced narratives, language mislabeling, and broken chapter boundaries are all common production defects that users notice. Extract and validate each track explicitly, normalize formats (e.g., to WebVTT for web delivery), preserve language tags correctly, and test that captions actually display in the target player. Treat a video with broken captions as a defective product, not a minor issue.

### Make reprocessing cheap and targeted

Pipeline logic, codec choices, and output specs change over time. You will need to reprocess existing content (new resolutions, better encoders, new thumbnail style, fixed caption extraction). Design so reprocessing can target a single stage or output without re-running the whole pipeline. Version your stage logic (encoder version, parameter set) and store it with each output so you can identify which outputs are stale and selectively regenerate only those. A pipeline that requires re-running expensive transcodes to fix a thumbnail bug is a pipeline that will never get its thumbnails fixed.

### Verify outputs, do not trust exit codes

A transcoder that exits zero can still produce a truncated, corrupt, or silent output. Verify outputs structurally: probe the output (duration matches input within tolerance, expected streams present, no decode errors on a sample of frames), check output file size is within a sane range, and for critical paths decode a few frames to confirm the output is actually playable. For audio, confirm the output is not silent when the input was not. Output verification catches the class of failures where the tool "succeeded" but produced garbage.

### Plan for distributed processing and backpressure

At scale, processing is distributed across many workers. Design workers to be stateless and horizontally scalable, pull jobs from a durable queue, and tolerate being killed mid-job (the job is retried by another worker). Implement backpressure so a flood of uploads does not overwhelm workers or storage. Be aware that media processing is CPU/GPU-bound, not I/O-bound; scaling workers requires scaling compute, and a single transcode can saturate a core for the full duration of the media.

## Common Traps

### Trusting the transcoder's exit code as proof of success

Exit code zero means the process ended, not that the output is correct or complete. Truncated outputs, silent audio, and corrupt frames can all come from a "successful" run. Always structurally verify outputs.

### Letting one pathological input exhaust a worker

A corrupt or adversarial file can cause a transcoder to consume all available memory or run for hours. Without hard limits and a timeout, a single bad upload can degrade the entire pipeline. Bound every stage and quarantine inputs that repeatedly fail.

### Re-running the entire pipeline on every failure or spec change

If a stage fails or a spec changes, re-running upstream transcodes wastes enormous compute. Design stages to be independently retryable and cacheable so reprocessing targets only what changed.

### Forgetting embedded captions and secondary audio tracks

Embedded captions (CEA-608/708), secondary audio languages, and chapter metadata are easy to drop during transcoding if not explicitly preserved. A "successful" transcode that loses the accessibility captions is a regression. Explicitly extract and validate every track.

### Assuming duration matching means correctness

Probing that input and output duration match is necessary but not sufficient. A video can be the right length but have corrupt frames, silent audio, or wrong resolution. Duration checks are one verification, not the whole verification strategy.

### Treating the pipeline as a script rather than a graph

A linear script cannot retry a single stage, run independent stages in parallel, or recover from partial failure. By the time the limitation is felt (during an incident or a spec change), retrofitting a graph model is expensive. Model the pipeline as a graph from the start.

## Self-Check

- Does each pipeline stage take a deterministic input and produce a cacheable, idempotent output keyed by input hash, stage version, and parameters?
- Is every untrusted input probed and validated (duration, codec, resolution, validity) before any expensive processing, with hard limits on size, duration, and resolution?
- Does every stage run with a memory limit, CPU limit, wall-clock timeout, and maximum output size, with inputs that exceed limits quarantined rather than marked successful?
- Is the pipeline modeled as a dependency graph that supports per-stage retry, parallelism, and partial completion, rather than a linear script?
- Are subtitles, chapters, and all metadata tracks explicitly extracted, validated, normalized, and tested for correct display in the target player?
- Can reprocessing target a single stage or output without re-running expensive upstream work, and is each output stamped with the stage version that produced it?
- Are outputs structurally verified (probe, stream presence, frame decode sample, audio non-silence) rather than trusted by exit code alone?
- Are workers stateless, horizontally scalable, pulling from a durable queue, and resilient to mid-job termination with retry by another worker?
- Is there backpressure so a flood of uploads cannot overwhelm workers or storage, and is compute (not just I/O) recognized as the scaling bottleneck?
- Are corrupt or adversarial inputs detected and quarantined rather than silently consuming unbounded resources?
