---
name: rendering_and_shader_performance.md
description: Use when the agent is building or optimizing a game renderer, shader, or GPU pipeline; reducing draw calls via batching, instancing, or merging; choosing a lighting model (forward, deferred, forward+ / clustered forward); diagnosing GPU-bound vs CPU-bound frames; profiling GPU work and frame time; reducing overdraw, fill rate, or texture bandwidth; managing shader complexity and ALU cost; writing or optimizing HLSL, GLSL, or MSL shaders; implementing render passes, G-buffers, or depth pre-passes; or diagnosing low frame rate, heat/throttling, visual artifacts, or the gap between "looks correct" and "runs at frame rate." Covers the tradeoffs of forward vs deferred rendering, draw-call minimization, shader instruction cost, and profiling methodology to find the real bottleneck.
---

# Rendering And Shader Performance

A renderer is correct when the image looks right, and a renderer ships when the image looks right *at the target frame rate on the target hardware*. These are different problems, and agents routinely solve the first and assume the second. The gap is where most rendering work actually lives: a scene that renders beautifully at 30 FPS on a desktop GPU and falls to 12 FPS on the target console, a shader that is mathematically correct but spends 40 ALU instructions on a calculation that could be two, a draw-call count that is fine for a static scene and collapses when every object is instanced separately. Rendering performance is dominated by a small number of bottlenecks — draw calls, overdraw, shader ALU, texture bandwidth — and the entire skill is identifying *which* one is the limit before optimizing, because optimizing the wrong one does nothing.

Agents tend to optimize what they can see in the code (CPU-side render submission) or what they remember as a rule of thumb ("reduce draw calls"), without measuring whether the frame is GPU-bound or CPU-bound. The result is effort spent cutting draw calls on a frame that was already GPU-bound by shader cost, or rewriting a shader when the bottleneck was texture bandwidth. The judgment problem is profiling first to locate the bottleneck, then applying the technique that targets that bottleneck, and never confusing visual correctness with shipping performance.

## Core Rules

### Profile Before Optimizing: Find The Real Bottleneck

A frame is either CPU-bound (the CPU spent the frame preparing and submitting render work) or GPU-bound (the GPU spent the frame executing that work), and the two require opposite fixes. Optimizing CPU submission on a GPU-bound frame produces zero FPS gain; optimizing a shader on a CPU-bound frame produces zero FPS gain. Use a GPU profiler (RenderDoc, PIX, Nsight, Xcode GPU capture) to measure GPU time per pass, and a CPU profiler to measure submission time. The decisive test: if the GPU finishes before the CPU submits the next frame's work, you are CPU-bound; if the CPU is idle waiting on the GPU, you are GPU-bound. Within GPU-bound frames, identify *which* stage is the limit — vertex processing, rasterization/fill (overdraw), pixel shader ALU, or texture bandwidth — because each has a different remedy. Profiling is not optional; "I think it's the draw calls" is how rendering work gets wasted.

### Reduce Draw Calls Via Batching And Instancing

Draw calls are CPU-side overhead: each one costs the driver time to validate state, set up buffers, and issue the command. At hundreds to thousands of draws, CPU submission becomes the frame bottleneck even when the GPU could render the geometry trivially. The techniques:

- **Static batching / merging.** Combine static geometry that shares material into one large vertex/index buffer, rendered in a single draw. Best for environment props that never move.
- **Dynamic batching.** For small meshes, transform vertices on the CPU and submit together (cheap meshes only; the CPU transform cost must be less than the draw-call savings).
- **Instancing.** The GPU renders many copies of the same mesh in one draw call, with per-instance attributes (transform, color) supplied via an instance buffer. The standard for repeated geometry — trees, rocks, particles, crowd. Far more scalable than dynamic batching because the GPU does the per-instance work.

The rule: if many objects share a mesh and material, instance them; if they are static and unique but share a material, batch them; reduce material/variant switches (each shader or texture change can force a new draw-call submission). But note: batching and instancing are CPU optimizations. If the frame is GPU-bound, they will not help — measure first.

### Choose The Lighting Model Knowing Its Failure Modes

Forward, deferred, and forward+ (clustered) have fundamentally different cost structures, and the choice constrains the whole renderer.

- **Forward rendering.** Each object is shaded with all lights that affect it in one pass. Simple, supports transparency and MSAA naturally, and works with material diversity. The cost scales with `objects × lights per object` — a scene with many lights hitting many objects explodes in shader cost. Best for scenes with few lights or limited light-object overlap.
- **Deferred rendering.** Shade geometry first into a G-buffer (positions/normals/albedo/material in multiple render targets), then run lighting in screen space against the G-buffer. Lighting cost scales with `lights × screen pixels covered`, independent of object count — excellent for many lights. The costs: high memory bandwidth (large G-buffers), no native transparency (transparent objects need a separate forward pass), no native MSAA, and material diversity is constrained by the G-buffer format. Also struggles with many distinct material models (they must fit the G-buffer schema).
- **Forward+ / clustered forward.** A hybrid: a depth pre-pass tiles the screen into clusters, lights are assigned to the clusters they overlap, then a forward pass shades each pixel using only the lights in its cluster. Gets deferred's "many lights, per-pixel light list" benefit while keeping forward's material flexibility and MSAA support. More complex to implement.

Strong choice: an indoor scene with dozens of dynamic lights uses deferred or forward+. Weak choice: using deferred for a scene that is mostly transparent (foliage, particles, water) and paying for a separate forward pass on most of the screen. The choice is a system-level commitment; it shapes the G-buffer, the material authoring, and the transparency path.

### Control Overdraw And Fill Rate

Overdraw is how many times each pixel is shaded per frame. A scene where the GPU shades the average pixel five times (four layers occluded behind the front one) burns five times the pixel-shader and bandwidth cost for no visible benefit. The fixes: a depth pre-pass (render the opaque scene to depth first with a cheap shader, then the real shading pass early-outs on depth-equal), front-to-back submission order (so occluders reject later objects in the depth test early), and careful use of alpha-tested/transparent geometry (which defeats early-z and forces full shading). Transparency is the classic overdraw amplifier: every transparent layer shades and blends fully, with no early-out. Particle systems and foliage are frequent culprits. Profile overdraw explicitly (many profilers visualize it); a scene that looks simple can be shading each pixel ten times.

### Manage Shader Complexity And ALU Cost

A pixel shader's cost is roughly its ALU instruction count times the number of pixels it runs on times the number of times those pixels are overdrawn. Complex math in a hot shader — nested loops, expensive transcendentals (`sin`, `pow`, `log`), per-pixel noise — multiplies across the screen. The discipline: move per-frame or per-object constants to the CPU or vertex shader (don't recompute the same value per pixel), use cheaper approximations where precision allows (half-precision where supported, polynomial approximations of transcendentals), avoid dependent texture reads and loops with non-uniform control flow on GPUs that penalize them, and profile the shader's actual cycle count rather than guessing. Texture bandwidth is the silent partner: a shader sampling eight textures per pixel at 4K is bandwidth-bound regardless of ALU. The goal is not the shortest shader, but the cheapest one that produces the required image — and "required" is defined by the art direction, not by mathematical elegance.

## Common Traps

### Optimizing CPU Submission On A GPU-Bound Frame

Spending a week cutting draw calls on a frame that was GPU-limited by shader cost, producing zero FPS gain. Always profile to find the bottleneck first; CPU and GPU optimizations target different problems.

### Treating "Looks Correct" As "Performs Well"

Shipping a shader because the image matches the reference, then discovering it runs at 20 FPS on the target GPU because of per-pixel transcendentals or 12 texture samples. Correctness is necessary, not sufficient; profile the shader's cycle count on target hardware.

### Deferred Rendering For A Mostly-Transparent Scene

Choosing deferred for "many lights" support, then building a foliage-heavy or particle-heavy game where most of the screen needs the separate forward pass anyway, paying the G-buffer bandwidth cost without the lighting benefit.

### Ignoring Overdraw From Transparency And Particles

A particle system or foliage layer that shades and blends every pixel multiple times with no early-z, burning fill rate on occluded work. Profile overdraw; it is often the hidden cost behind a "simple-looking" scene that runs slowly.

### Judging Performance On A Desktop Instead Of Target Hardware

Profiling on a high-end desktop GPU with massive bandwidth and concluding the renderer is fine, then shipping and finding the target console or mobile GPU is memory-bandwidth-bound. Profile on the minimum-spec target; relative bottleneck rankings differ across GPUs.

## Self-Check

- [ ] The frame was profiled to determine whether it is CPU-bound or GPU-bound before any optimization, and GPU-bound frames identify the limiting stage (vertex, fill/overdraw, pixel-shader ALU, or texture bandwidth).
- [ ] Draw-call count is managed via instancing for repeated meshes, static batching for shared-material static geometry, and minimized material/variant switches — but only pursued when the frame is actually CPU-bound on submission.
- [ ] The lighting model (forward, deferred, forward+) was chosen against the scene's light count, transparency needs, material diversity, and MSAA requirements, and its known failure modes (transparency for deferred, light-object scaling for forward) were considered.
- [ ] Overdraw is measured and controlled: a depth pre-pass and front-to-back opaque submission reduce wasted shading, and transparent/particle-heavy areas are profiled, not assumed cheap.
- [ ] Hot pixel shaders are profiled for ALU and texture-sample cost; invariant work is hoisted to the CPU or vertex shader; approximations are used where precision allows; bandwidth from many texture samples is accounted for.
- [ ] Rendering was profiled on the minimum-spec target hardware, not only on a desktop, because GPU bottleneck rankings differ across devices.
- [ ] A GPU profiler (RenderDoc/PIX/Nsight/Xcode) was used to attribute GPU time per pass, and the most expensive pass — not the most convenient one — was the target of optimization.
- [ ] The final image was verified to meet art-direction requirements *and* frame budget on target hardware — correctness and performance were checked as separate criteria, and neither was assumed from the other.
