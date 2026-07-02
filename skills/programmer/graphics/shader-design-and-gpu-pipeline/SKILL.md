---
name: shader_design_and_gpu_pipeline.md
description: Use when the agent is writing vertex, fragment, compute, geometry, or tessellation shaders; designing render passes and pipeline state; structuring uniform and vertex data uploads; debugging GPU performance cliffs, draw call overhead, shader divergence, bandwidth-bound frames, or pipeline stalls; or choosing between rendering techniques (forward, deferred, forward+, visibility buffers) against target hardware. Covers GPU architecture constraints, memory bandwidth, warp divergence, draw call costs, and the tradeoffs of pipeline and shader structure.
---

# Shader Design And GPU Pipeline

A GPU is not a fast CPU with more cores. It is a throughput machine built around hiding memory latency with massive thread parallelism, and its performance is governed by constraints that have no analogue in CPU programming: memory bandwidth is the dominant cost (not instruction count), threads execute in lockstep groups so divergence serializes them, draw calls cross a CPU-GPU boundary that is expensive regardless of how little work the GPU does, and the fixed-function pipeline stages impose structure that the programmable stages must respect. Code that looks efficient by CPU intuition — branchless, minimal arithmetic, few function calls — can be a disaster on the GPU because it diverges, thrashes the texture cache, or issues a thousand draw calls that each do one triangle. The difficulty is that the GPU's cost model is invisible from the shader source and is only visible in a profiler, by which point the architecture is fixed.

Agents tend to write shaders as if they were CPU functions and treat the pipeline as plumbing, because the happy path renders correctly and the failures only appear under profiling or on lower-tier hardware. The judgment problem is deciding, for each piece of rendering work, what pipeline technique it belongs in (forward vs deferred vs compute), how to express the shader so threads stay coherent and memory access stays cached, and how to structure draw calls and state so the CPU-GPU boundary is not the bottleneck. The harm of getting this wrong ranges from a frame that misses its budget on the target hardware to a pipeline that cannot be salvaged without a rewrite because the fundamental technique was wrong.

## Core Rules

### Pick The Rendering Technique Against Bandwidth, Material Diversity, And Hardware

The choice between forward, deferred, forward+, and visibility-buffer rendering is a tradeoff among bandwidth cost, material flexibility, transparency handling, and hardware support — and it is hard to reverse.

- **Forward rendering** shades each fragment as the geometry is rasterized, looping over lights in the shader. Simple, handles material diversity and transparency naturally, but scales linearly with light count (every fragment pays for every overlapping light) and can waste shading on occluded fragments. Best for few lights, complex materials, or transparent geometry.
- **Deferred rendering** renders geometry to a G-buffer (positions/normals/material in multiple render targets), then shades in a full-screen pass that reads the G-buffer once per light. Decouples shading cost from geometry complexity and handles many lights efficiently, but consumes large bandwidth for the G-buffer, struggles with transparency (no G-buffer for transparent geometry), and limits material diversity (every surface writes the same G-buffer layout). Best for many lights, uniform-ish materials, opaque scenes.
- **Forward+ / clustered** builds a light grid (compute pass), then forward-shades each fragment using only the lights in its cluster. Combines forward's material and transparency flexibility with deferred's light-count scaling, at the cost of the cluster-build pass. Often the modern default for engines that need both.
- **Compute-based / visibility buffer** rasterizes a visibility buffer (which triangle hit each pixel), then shades in compute. Maximizes material flexibility and shading reuse, but is more complex and depends on hardware features.

Name the governing constraint — light count, material diversity, transparency needs, target bandwidth budget — before picking. There is no universally best technique; the wrong choice caps performance for the life of the project.

### Minimize Bandwidth, Because Bandwidth — Not Instructions — Usually Governs GPU Performance

Modern GPUs execute arithmetic far faster than they can fetch memory. A shader that does ten extra ALU operations to avoid one texture fetch is usually a net win; a shader that adds a texture fetch to save arithmetic is usually a loss. The first performance question about any shader is "what memory does it touch, and is that memory cached," not "how many operations does it do."

Concretely: pack vertex attributes tightly (fewer attributes = less bandwidth per vertex), use the smallest texture formats that hold the data (a normal map in BC5, not RGBA32F), access textures with cache-friendly coherence (mipmapped, contiguous UVs, not random samples), and avoid reading G-buffer or render targets more than necessary. Bandwidth-bound shaders are the common case; an instruction-count optimization that increases bandwidth makes things slower. Profile with a GPU profiler that reports bandwidth utilization before assuming an "optimization" helped.

### Keep Warps/Wavefronts Coherent; Divergence Serializes

GPU threads execute in lockstep groups — warps (NVIDIA, 32 threads) or wavefronts (AMD, 32 or 64). Within a group, all threads execute the same instruction at the same time. When threads in a group take different branches (`if`/`else`), the GPU executes both branches and masks off the threads that don't take each — effectively serializing. This is divergence, and it is one of the most common and most invisible performance cliffs.

Minimize divergence by structuring shaders so neighboring threads (which share a warp) take the same path. Prefer uniform branches (conditions that are the same for the whole draw call, hoisted to the CPU or a uniform) over per-fragment branches. Prefer math over branching where a branch would diverge (`mix`/`step` instead of `if`). Prefer texture-driven variation (a texture lookup that varies per fragment) over shader-branch variation. A dynamic loop whose trip count varies across a warp is also divergence: the warp iterates until the longest-running thread finishes, with finished threads idle. Keep loop bounds uniform where possible.

### Batch Draw Calls; The CPU-GPU Boundary Is Expensive Per Call

Each draw call is a CPU-side cost: validating state, setting uniforms, uploading constants, submitting to the command buffer, and crossing the driver boundary. This cost is largely independent of how much geometry the draw submits — a draw of one triangle costs almost as much as a draw of ten thousand. A frame with five thousand draw calls is often CPU-bound on the submission, not GPU-bound on the rendering, and no shader optimization will help.

Reduce draw calls by batching: static batching (combine static geometry into one mesh at build time), instancing (draw many copies of the same mesh with per-instance attributes in one call), and texture atlasing / array textures (so different objects can share one material and thus one draw). Target a draw call budget per frame based on the weakest target hardware, and measure. A scene that submits a draw call per object per material will not scale; one that batches aggressively will.

### Match Data Layouts To The Fixed-Function Stages And Cache Lines

The vertex fetch stage, the rasterizer, and the texture units are fixed-function hardware with expectations about data layout. Vertex attributes should be packed in the order the shader reads them, in the smallest format that holds the range (e.g., normals as signed 8-bit packed, positions as float16 where precision allows), and arranged so a single cache line fetch serves multiple vertices (interleaved attributes, or de-interleaved for specific access patterns — benchmark both). Textures should be mipmapped (so distant texels fetch from smaller mip levels with better cache behavior), compressed with a block format the hardware samples natively (BCn on desktop, ASTC/ETC on mobile), and laid out in the GPU-preferred tiling (the driver usually handles this, but be aware it exists).

Do not fight the hardware with naive layouts. A vertex buffer of `Vec3 position; Vec3 normal; Vec2 uv; Vec4 color` as 32-byte structs when the data fits in 16 bytes doubles the bandwidth and halves the vertex fetch throughput.

### Structure Pipeline State To Avoid Redundant State Changes

Pipeline state (shaders, blend modes, depth/stencil state, render targets, vertex layouts) is expensive to switch on the GPU, and the driver may stall to recompile programs when state combinations change. Group draws by shared state: sort draws by shader, then by material/textures, then by blend state, so the pipeline state changes as few times as possible per frame. A frame that interleaves opaque, transparent, and post-process draws triggers constant state changes; one that batches all opaque draws, then all transparent, then post-process, changes state a handful of times.

Use pipeline state objects (Vulkan, DirectX 12, modern APIs) or sort by state hash (older APIs) to make state changes deliberate and minimal. A render pass that flips between two shaders per draw is paying the state-change cost on every draw.

### Use Compute Where The Graphics Pipeline Is The Wrong Tool

Not all GPU work fits the rasterization pipeline. Culling, light clustering, particle simulation, image filtering, and prefix sums are often better expressed as compute shaders, which have flexible workgroup sizes, shared memory, and explicit synchronization. Compute lets you express algorithms that the fixed-function rasterizer would fight (scattering, reductions, irregular work).

Reach for compute when the work is not "shade these triangles" — but be aware that compute and graphics share the same bandwidth and divergence constraints, and that the handoff between compute and graphics passes has its own cost (barriers, cache flushes). A compute pass that produces data the graphics pass consumes requires a memory barrier; too many barriers stall the pipeline.

## Common Traps

### Optimizing Instruction Count While Ignoring Bandwidth

Spending days shaving ALU operations from a shader that is bandwidth-bound, so the optimization has no effect. Profile to identify the bottleneck (bandwidth, ALU, texture, or fill rate) before optimizing; bandwidth is the common case on modern GPUs.

### Per-Fragment Branching That Diverges Across The Warp

Using `if (condition) { ... } else { ... }` where the condition varies per fragment, so the warp executes both branches and masks half the threads. Prefer uniform conditions, math (`mix`, `step`), or texture-driven variation over divergent branches.

### One Draw Call Per Object

Submitting a draw call per mesh per material, then hitting a CPU-bound frame on draw submission. Batch (static batching, instancing, atlas/array textures) to reduce draw calls; set and measure a per-frame draw call budget.

### Choosing Deferred Rendering For A Scene With Heavy Transparency

Picking deferred for "many lights," then discovering transparent geometry (particles, glass, foliage) cannot be deferred and must be forward-rendered in a separate pass, doubling the work. Match the technique to the scene's transparency and material needs.

### Oversized Or Wrong-Format Textures And Attributes

Using RGBA32F for normals, uncompressed textures, or unpacked vertex attributes, doubling bandwidth and halving throughput. Pack data into the smallest format the precision allows; use hardware-native compressed texture formats.

### Flipping Pipeline State Every Draw

Interleaving draws with different shaders, blend modes, or render targets, so the pipeline state changes on every draw and the driver stalls. Sort draws by state to minimize changes; use PSOs or state-hash sorting.

### Dynamic Loops With Per-Thread Trip Counts

A `for` loop whose bound varies per fragment, so the warp iterates to the longest bound with finished threads idle. Hoist loop bounds to uniforms where possible, or restructure so the work is uniform across the warp.

### Treating Compute As Free Because It's "Not The Graphics Pipeline"

Adding compute passes for culling, clustering, or simulation without accounting for the barrier cost between compute and graphics, then stalling on synchronization. Barriers are not free; minimize handoffs between compute and graphics passes.

## Self-Check

- [ ] The rendering technique (forward, deferred, forward+, compute/visibility) was chosen against the scene's light count, material diversity, transparency needs, and target bandwidth budget — not by default or fashion.
- [ ] The frame's bottleneck was identified with a GPU profiler (bandwidth, ALU, texture, fill rate, or CPU draw submission) before optimizing; instruction-count optimizations were not applied to a bandwidth-bound shader.
- [ ] Branching in hot shaders is uniform where possible; divergent per-fragment branches were replaced with math, texture-driven variation, or hoisted conditions.
- [ ] Draw calls are batched (static batching, instancing, atlas/array textures) to meet a per-frame draw call budget measured on the weakest target hardware.
- [ ] Vertex attributes and textures are packed into the smallest formats the precision allows; textures use hardware-native compressed formats (BCn, ASTC, ETC) and are mipmapped.
- [ ] Draws are sorted by pipeline state (shader, material, blend, render target) to minimize state changes per frame; PSOs or state-hash sorting make changes deliberate.
- [ ] Compute passes are used where the graphics pipeline is the wrong tool, and the barrier cost between compute and graphics passes is accounted for, not assumed free.
- [ ] A profile on the target (or lowest-target) hardware confirmed the frame meets its time budget, and no single shader or pass dominates unexpectedly.
