---
name: asset_pipeline_and_memory.md
description: Use when the agent is building or auditing a game asset pipeline, content import workflow, or build system; importing, compressing, or transcoding textures, meshes, audio, or animation; choosing a loading strategy (synchronous, asynchronous, background streaming); setting GPU VRAM or system RAM memory budgets; designing LOD chains or mip streaming; tracking asset dependencies and reference graphs; deciding between hot-reloadable and cooked/packed assets; ensuring build determinism and reproducibility; or diagnosing oversized binaries, mid-level loading hitches, VRAM thrashing, missing-asset crashes, or "works on my machine" build divergence between developer machines and shipping builds.
---

# Asset Pipeline And Memory

An asset pipeline is not "a folder of PNGs and WAVs the game reads." It is the machinery that turns source art into the exact bytes the runtime uploads to the GPU and audio decoder, and the decisions in that machinery — compression formats, loading timing, memory budgets, LOD policy, dependency tracking — are almost impossible to retrofit once the game is playable. The pipeline defines the shipping binary's size, the frame-time cost of entering a level, whether a mid-range GPU runs out of VRAM, and whether two builds from the same source produce the same bytes. These are architectural decisions, not polish tasks.

Agents tend to treat assets as opaque inputs and reach for the defaults: import at full resolution, load everything up front, compress "when we ship." This works on the developer's machine with its 24 GB GPU and NVMe drive, and it fails everywhere else — a 50 GB binary, a level that stutters for 800 ms on a door open, textures that thrash in and out of VRAM on a 4 GB card, and a build that produces different hashes on two machines because a timestamp leaked into a cache key. The judgment problem is deciding budgets and formats up front, choosing a loading strategy per asset class, tracking what depends on what, and keeping the build reproducible — *before* the game is large enough that changing any of it is a multi-week migration.

## Core Rules

### Set Memory And Disk Budgets Before Importing Anything

Compression, resolution, and LOD decisions are meaningless without a target. Decide, per platform, the VRAM budget (how many MB of textures and vertex/index buffers can be resident), the RAM budget (audio, decompressed meshes, asset metadata), and the disk/install budget (the shipped binary size). These budgets must be allocated by category — e.g., "characters get 512 MB VRAM, environment 1.5 GB, UI 128 MB" — and enforced by the pipeline, not hoped for at the end. An asset imported at 4K without a budget will be 4K at ship; an asset imported against a 2 MB-per-texture budget will be compressed to fit. Budgets are the only thing that turns "we should compress" into a concrete, verifiable decision. Without them, the pipeline optimizes for fidelity on the developer's machine, not for the target hardware.

### Match The Loading Strategy To The Access Pattern

There are three strategies, and each asset class should use the one that fits its access pattern.

- **Synchronous load.** Blocks the frame until done. Acceptable only for tiny assets (a UI icon, a sound effect) or during an explicit loading screen where blocking is invisible. Any synchronous load on the gameplay path is a frame hitch waiting to happen.
- **Asynchronous load.** Requested in advance and completed over several frames via background I/O and decompression. The standard for level assets, character models, and anything the game knows it will need soon. Requires the gameplay code to tolerate the asset being not-yet-ready (placeholder, fallback, or gating).
- **Streaming.** Assets are loaded on demand, often in chunks tied to player position (open-world terrain, streaming sectors). Demands a streaming system that predicts what will be needed (ahead of the player's movement) and evicts what is no longer needed, with strict VRAM budgeting to avoid thrash.

The failure is applying one strategy everywhere: loading the entire game synchronously at startup (long load, huge RAM), or streaming everything including a small UI texture (unnecessary complexity and latency). Name the strategy per asset class, and make sure gameplay code handles the not-ready case rather than assuming the asset is present.

### Compress For The Target Hardware, Not The Authoring Format

Source art is uncompressed and huge; the runtime almost never uses it raw. Compression is chosen per asset type and per target platform, and the formats have real tradeoffs.

- **Textures.** GPU block-compression formats (BCn on desktop, ASTC on mobile) reduce VRAM 4–8x with controlled quality loss and are sampled directly by the GPU without decompression. Choose the format and quality per texture *class* — a normal map tolerates different compression than a diffuse map, and a UI element needs higher fidelity than a distant prop. Generate mipmaps always (they cut bandwidth and prevent aliasing); failing to is a common silent perf loss.
- **Meshes.** Compress vertex data (quantize positions/normals to 16-bit where precision allows), strip or cache-optimize index buffers, and consider mesh LOD chains rather than one high-poly model.
- **Audio.** Compressed streaming (Vorbis/Opus) for long music and dialogue; uncompressed or short-loop PCM for low-latency one-shots (gunfire, UI) where decode latency matters. The mistake is compressing a gunshot and hearing the decode delay, or leaving a 5-minute music loop as PCM.

Decide formats in the import settings, not at ship, because changing them later means re-importing everything and re-tuning quality.

### Track Asset Dependencies Explicitly

Assets reference other assets: a material references textures and shaders; a prefab references meshes, materials, and scripts; a level references hundreds of assets. This reference graph is load-bearing. The pipeline must know it, because (a) loading an asset means transitively loading its dependencies, (b) unloading an asset is only safe when nothing references it (reference counting or garbage collection), and (c) a missing or broken reference is a crash or a pink-missing-texture. Build a dependency graph during import, validate it in CI (no dangling references, no cycles), and use it to drive both loading (load the dependency closure) and packing (bundle an asset with its dependencies so they stream together). Untracked dependencies produce the classic "works in editor, crashes in cooked build" failure, because the editor loads everything speculatively while the cooked build loads only what was packed.

### Keep Hot-Reload And Cooked Builds In Sync

Hot-reload (the editor re-imports a changed asset without restart) is a developer productivity feature, not a shipping path. The trap is building and testing only against hot-reload, then discovering at ship that the cooked/packed build behaves differently — different asset ordering, different default compression, missing assets that the editor loaded speculatively, or shader variants that were stripped. The rule: the cooked build must be a first-class, regularly-tested artifact, not a final step. Run the game from cooked builds in CI or nightly; any asset or shader feature that only works under hot-reload is a shipping bug. Similarly, ensure the cook is deterministic: same source plus same tool versions produce the same bytes. Determinism breaks when timestamps, machine-local paths, or non-reproducible compression seeds leak into the output — hash the build and diff against a known-good hash in CI.

## Common Traps

### Shipping Uncompressed Or Dev-Resolution Assets

Importing at full source resolution "to preserve quality," never revisiting it, and shipping a binary 10x larger than necessary with textures that thrash a 4 GB GPU. Compression and resolution are decisions made at import against a budget; deferring them means shipping the dev-machine defaults.

### Synchronous Loading On The Gameplay Path

Calling a blocking load when the player opens a door or triggers a cutscene, producing a visible hitch. The fix is async loading triggered early enough that the asset is resident by the time it is needed; if that is not possible, a loading screen is more honest than a stutter.

### Dangling Or Circular Dependencies

An asset that references a deleted texture (crash or pink texture in cooked build), or two assets that reference each other (infinite load or pack failure). The dependency graph must be validated in CI; the editor's speculative loading hides these until cook.

### Testing Only Under Hot-Reload

Developing for months against the editor's hot-reload path, then discovering at ship that shader variants were stripped, assets were missing from the pack, or compression changed the look. The cooked build is a separate artifact that must be tested continuously, not at the end.

### Non-Deterministic Cook Output

A build pipeline that embeds timestamps, absolute source paths, or a non-seeded compression step, producing different bytes from the same source on different machines. Reproducible builds are a correctness property — they let you prove a shipped binary came from a known source state and make delta-patching feasible.

## Self-Check

- [ ] Per-platform VRAM, RAM, and disk/install budgets exist and are allocated by asset category, and the pipeline enforces them (e.g., imports fail or warn when a texture exceeds its class budget).
- [ ] Each asset class uses a loading strategy (sync, async, streaming) matched to its access pattern, and gameplay code handles the not-yet-ready case rather than assuming presence.
- [ ] Textures use GPU-native block-compression formats (BCn/ASTC) with mipmaps; meshes use quantized vertex data and LOD chains; audio is split between compressed-streaming and low-latency PCM by use case.
- [ ] An asset dependency graph is built at import, validated in CI (no dangling references, no cycles), and used to drive both load-closure and packing.
- [ ] The cooked/packed build is built and tested regularly (CI or nightly), not only at ship, and any feature that works only under hot-reload is treated as a shipping bug.
- [ ] The cook is deterministic: same source and tool versions produce identical bytes, verified by hashing and diffing against a known-good build in CI.
- [ ] Streaming systems have a hard VRAM budget and an eviction policy that respects currently-visible assets, verified not to thrash under movement.
- [ ] A low-spec target device (minimum-spec GPU/RAM/storage speed) was tested end-to-end, and the game neither exceeded VRAM, hitched on level transitions, nor shipped an oversized binary.
