---
name: rendering_optimization.md
description: Use when the agent is optimizing a renderer for frame time, reducing overdraw, implementing batching or instancing or GPU instancing, building LOD systems, adding frustum or occlusion culling, choosing texture compression formats, setting a frame time budget, or profiling GPU performance to find why a frame is over budget. Covers overdraw, draw call reduction, level of detail, culling strategies, texture compression tradeoffs, and the discipline of profiling against a fixed frame budget.
---

# Rendering Optimization

Rendering optimization is governed by a single fact: a frame has a fixed time budget (16.6 ms for 60 FPS, 6.9 ms for 144 FPS, including everything — simulation, rendering, present), and exceeding it produces visible stutter, dropped frames, or thermal throttling that cascades into worse performance. The work is not "make it faster"; it is "spend the budget deliberately across the things that matter, and prove the frame fits." The difficulty is that a renderer has many independent costs — draw calls, fill rate (overdraw), vertex processing, texture bandwidth, shader ALU, state changes — and optimizing the wrong one (the one that is easy to measure or easy to change) has no effect on the frame time because the bottleneck is elsewhere. A week spent cutting draw calls on a fill-rate-bound frame changes nothing; an hour spent on overdraw fixes it.

Agents tend to optimize by intuition and by checklist ("reduce draw calls, add LOD, compress textures") without measuring where the time actually goes, because the happy path runs at 60 FPS on the dev machine and the failures only appear on weaker hardware or under specific scenes. The judgment problem is deciding, against measured profile data, which cost is the bottleneck, what the second-order effects of each optimization are (culling reduces draw calls but adds CPU work; LOD reduces vertices but can pop), and how to keep the frame within budget across the full range of target hardware and scenes. Getting this wrong produces a renderer that benchmarks well and stutters in real play.

## Core Rules

### Set A Frame Time Budget And Optimize Against The Weakest Target

Optimization without a target is activity without progress. Define the frame budget explicitly (e.g., 16.6 ms for 60 FPS), define the target hardware (the weakest device you support), and define the worst-case scene (the one with the most geometry, lights, overdraw, and effects). Every optimization decision is then a question: does this keep the worst-case frame on the weakest target within budget? An optimization that improves the dev machine's frame from 5 ms to 4 ms is worthless if the target device is at 25 ms.

Measure on the target hardware, not the dev machine. A renderer that hits 60 FPS on a desktop GPU and 20 FPS on the target mobile GPU has a target-device problem that no amount of desktop profiling will reveal. Keep a representative low-end device on the desk and profile on it routinely.

### Profile Before Optimizing; The Bottleneck Is Not Where Intuition Says

A GPU frame is bottlenecked by one of: draw call submission (CPU-bound), vertex processing (geometry-bound), fill rate (overdraw-bound), texture bandwidth (memory-bound), shader ALU (compute-bound), or state changes. The first optimization step is to identify which, because optimizing a non-bottleneck has zero effect on frame time. GPU profilers (RenderDoc, PIX, NVIDIA Nsight, Xcode GPU frame capture) report which stage dominates; use them.

Heuristics help triage: if reducing resolution dramatically improves frame time, you are fill-rate- or bandwidth-bound; if reducing draw call count dramatically improves frame time, you are CPU submission-bound; if reducing triangle count helps, you are geometry-bound; if simplifying shaders helps, you are ALU-bound. Confirm with the profiler, then optimize the actual bottleneck. The most common error is optimizing the cost that is easiest to change rather than the cost that dominates.

### Attack Overdraw, Because Fill Rate Is A Finite Resource

Overdraw is the number of times a pixel is shaded per frame. A pixel shaded six times (six overlapping layers of geometry) consumes six times the shading and depth-test work of a pixel shaded once. Heavy overdraw is the most common cause of fill-rate-bound frames, and it is invisible in the scene graph — you see it only in an overdraw visualization or a profiler showing fragment shading dominates.

Reduce overdraw by: rendering opaque front-to-back (so the depth test rejects occluded fragments early, before shading — depth pre-pass with a depth-only render can help further), culling occluded geometry (occlusion culling, not just frustum culling), reducing transparent-layer stacking (particles, foliage, UI), and using LOD to reduce the triangle density of distant objects that overdraw each other. A particle system that renders a full-screen quad per particle is an overdraw disaster; a particle system that bounds each particle to its sprite is fine.

### Reduce Draw Calls Through Batching, Instancing, And Atlasing

Draw call submission is CPU-bound and per-call expensive (see the shader/pipeline skill). The standard reductions: static batching (combine static geometry that shares a material into one mesh at build or load time), dynamic batching (small meshes combined per frame — useful only for very small meshes, as the CPU combine cost can exceed the savings), GPU instancing (one draw call for many copies of the same mesh with per-instance attributes), and texture atlasing / array textures (so objects that differ only by texture share one material and one draw call).

The discipline is to measure the draw call count per frame, set a budget based on the target CPU's submission throughput, and batch until the budget is met. Note that batching has limits: dynamically batched meshes must be small (the combine cost scales with vertex count), instanced meshes must share a mesh and shader, and atlased textures introduce UV-bleeding and mip-map complications. Apply the right technique per case, not all of them everywhere.

### Use LOD To Scale Triangle Cost With Screen Size

Level of detail (LOD) swaps lower-resolution meshes for distant objects so vertex processing scales with the object's contribution to the screen, not its absolute complexity. An object that covers 4 pixels should not be a 50,000-triangle mesh; it should be a 50-triangle billboard. LOD is one of the highest-leverage optimizations for geometry-bound frames, and it must be tuned to avoid popping (visible switches between LODs) and to cover the full distance range.

Discipline: generate LODs down to the smallest screen-space size that matters, switch based on screen-space coverage (not raw distance, so aspect ratio and FOV are handled), and use geomorphing or dithering to hide transitions where popping is visible. Do not stop at three LODs if the scene has distant objects; a billboard or impostor LOD for the far distance is often the largest single win.

### Cull Aggressively, But Account For The Cull's Own Cost

Culling removes geometry that does not contribute to the frame — outside the frustum (frustum culling), occluded by other geometry (occlusion culling), or behind the camera. Frustum culling is cheap and mandatory; occlusion culling is more expensive (it requires a depth hierarchy or a software rasterization pre-pass) and pays off only when the scene has significant occlusion (interiors, dense urban scenes). Culling reduces both draw calls and the vertex/fragment work downstream.

The trap is forgetting that culling is itself CPU (or compute) work. A culling system that tests every object against a complex occlusion hierarchy per frame can become CPU-bound on culling alone. Size the culling structure to the scene, use spatial acceleration (BVH, octree, hash grids), and run culling in a job system or on the GPU (compute-based culling with indirect draw) when the object count is large. Culling that costs more than it saves is a regression.

### Compress Textures With Hardware-Native Formats, And Budget VRAM

Uncompressed textures (RGBA8 or worse, RGBA32F) consume bandwidth and VRAM far in excess of their visual contribution. Hardware-native compressed formats — BCn (DXT) on desktop, ASTC on mobile and modern desktop, ETC2 as a baseline mobile format — are sampled directly by the GPU without decompression, saving 4x to 8x bandwidth and VRAM for a small quality cost. Every texture that is not compressed is wasting bandwidth on a bandwidth-bound renderer.

Choose the format against the data: BC1/ETC2 for opaque color (4 bits/pixel), BC3/BC5 for normals (two-channel compressed), BC7/ASTC for high-quality color with alpha, ASTC with a higher block size for mobile where quality at low bitrate matters. Budget VRAM explicitly: a scene with 4 GB of uncompressed textures will not run on a 2 GB mobile GPU; the same scene compressed fits comfortably. Mipmap everything that is sampled at varying scales; an unmipmapped texture causes cache thrashing and aliasing.

### Account For Second-Order Effects Of Every Optimization

Every optimization changes the cost model, sometimes regressively. Culling reduces draw calls but adds CPU work; LOD reduces vertices but can pop and requires LOD generation; batching reduces draw calls but increases per-draw memory and can prevent per-object frustum culling; texture compression saves bandwidth but can introduce artifacts that need higher base resolution. Track these tradeoffs explicitly: an optimization that helps the bottleneck but introduces a new cost elsewhere is only a net win if the new cost is not itself the next bottleneck.

The meta-rule: measure before and after every optimization, on the target hardware, on the worst-case scene. An optimization that improves a synthetic benchmark but regresses the real scene is not an optimization.

## Common Traps

### Optimizing Without Measuring The Bottleneck

Applying draw call batching, LOD, and texture compression to a frame that is fill-rate-bound on overdraw, and seeing no improvement. Profile first; optimize the actual bottleneck.

### Profiling On The Dev Machine, Not The Target

Hitting 60 FPS on the desktop and 20 FPS on the target mobile GPU, then optimizing for the desktop and shipping a stuttering product. Profile on the weakest target hardware routinely.

### Ignoring Overdraw In Particle-Heavy Or Foliage-Heavy Scenes

Rendering full-screen quads per particle or many overlapping foliage cards, producing 10x+ overdraw and a fill-rate-bound frame. Bound sprites to their extent, reduce layer stacking, render opaque front-to-back.

### Batching Everything Without Measuring Draw Call Cost

Spending effort batching on a frame that is GPU-bound (not CPU submission-bound), where the draw call count was never the problem. Draw call optimization helps only CPU-submission-bound frames.

### Stopping LOD At Three Levels With No Far-Distance Billboard

Using three LOD meshes and letting distant objects render at the lowest LOD's full triangle count, missing the largest win (billboard/impostor for far distance). Generate LODs down to the smallest visible screen size.

### Culling That Costs More Than It Saves

Running a complex occlusion cull per object per frame and becoming CPU-bound on culling work. Size the culling structure to the scene; move heavy culling to a job system or GPU compute.

### Uncompressed Or Oversized Textures

Shipping RGBA8 or RGBA32F textures, or textures far larger than their screen contribution, wasting bandwidth and VRAM. Compress with hardware-native formats; mipmap everything sampled at varying scales.

### Treating An Optimization As A Win Without Measuring

Assuming batching/LOD/culling helped because it "should," then shipping a regression when the second-order cost (CPU cull work, LOD popping, lost per-object cull) exceeded the gain. Measure before and after, on target hardware, on the worst case.

## Self-Check

- [ ] A frame time budget is defined (e.g., 16.6 ms for 60 FPS), with a named weakest target device and a named worst-case scene; every optimization is justified against keeping that frame within budget.
- [ ] The frame's bottleneck (CPU submission, vertex, fill rate, bandwidth, ALU, state changes) was identified with a GPU profiler before optimizing; non-bottlenecks were not optimized.
- [ ] Overdraw is bounded: opaque geometry renders front-to-back with depth-test early rejection, occlusion culling is applied where the scene benefits, and transparent-layer stacking is controlled.
- [ ] Draw call count is measured and within a budget set by the target CPU's submission throughput; static batching, instancing, and atlasing are applied where they actually reduce calls.
- [ ] LOD covers the full distance range down to a billboard or impostor for far objects; transitions are based on screen-space coverage and dithered/geomorphed where popping is visible.
- [ ] Culling is aggressive but its own cost is accounted for; heavy culling runs in a job system or GPU compute, and culling is not the new bottleneck.
- [ ] Textures use hardware-native compressed formats (BCn, ASTC, ETC2) chosen against the data type; everything sampled at varying scales is mipmapped; VRAM is budgeted to fit the target.
- [ ] Every optimization was measured before and after on the target hardware and worst-case scene; second-order costs (cull CPU, LOD pop, lost per-object cull, compression artifacts) were checked and found not to regress the frame.
