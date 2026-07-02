---
name: cross_platform_rendering.md
description: Use when the agent is building a renderer that targets multiple graphics APIs (Vulkan, Metal, DirectX 11/12, OpenGL, WebGPU), designing a graphics abstraction layer, handling feature differences across platforms, cross-compiling shaders (HLSL, GLSL, MSL, WGSL), or debugging rendering differences, missing features, or performance cliffs across hardware and platforms.
---

# Cross-Platform Rendering

A cross-platform renderer must produce the same image and the same performance character across hardware and operating systems that expose fundamentally different graphics APIs. Vulkan, DirectX 12, and Metal are explicit, low-overhead APIs where the application manages memory, synchronization, and pipeline state directly. DirectX 11 and OpenGL are higher-level, driver-managed APIs with different ergonomics and different performance cliffs. WebGPU is a modern, constrained API designed for the browser sandbox. Each API has its own shader language (HLSL, GLSL/SPIR-V, MSL, WGSL), its own feature set, its own coordinate conventions, and its own performance characteristics. The renderer that pretends these differences do not exist — that builds a thin wrapper and hopes the backends behave identically — ships a product that renders correctly on one platform and subtly wrong, slow, or broken on the others, with bugs that only appear on hardware the developer does not own.

Agents tend to build a cross-platform renderer by abstracting the common API calls into a thin interface and treating the backends as interchangeable, because the happy path renders the same triangle on every platform and the failures (coordinate handedness differences, missing format support, shader compilation divergences, synchronization semantics that differ) only appear under specific features or specific hardware. The judgment problem is deciding how thick the abstraction should be, how to handle genuine feature differences without lowest-common-denominator paralysis, how to share shaders across languages, and how to verify that each platform produces correct and performant output rather than just "something that renders." Getting this wrong produces a renderer that works on the lead platform and becomes a maintenance and bug-fix treadmill on every other.

## Core Rules

### Design The Abstraction Against The Lowest-Overhead API, Not The Highest

The common mistake is to design the abstraction against a high-level API (OpenGL, DirectX 11) where the driver hides memory management and synchronization, then find that the abstraction cannot express the explicit control that Vulkan, DirectX 12, and Metal require. The result is either a leaky abstraction that exposes API-specific details anyway, or a lowest-common-denominator abstraction that cannot use the modern APIs' capabilities and inherits the high-level APIs' performance cliffs.

Design the abstraction against the explicit, low-overhead APIs (Vulkan, D3D12, Metal) as the target, because they are the most demanding: they require the application to manage command buffer recording, pipeline state objects, memory allocation, barriers, and synchronization. An abstraction that can express these can be implemented atop the higher-level APIs (the driver does the work the abstraction would otherwise have to), but the reverse is not true. This does not mean exposing every Vulkan concept in the abstraction; it means the abstraction's model must be compatible with explicit control, so the modern backends are not fighting the abstraction.

### Handle Feature Differences Explicitly, Not By Assumption

Graphics APIs and hardware differ in what they support: texture compression formats (BCn is desktop, ASTC is mobile/modern, ETC2 is mobile baseline), feature levels (tessellation, ray tracing, mesh shaders, variable rate shading), format support (some depth formats, some compressed formats), and limits (max texture size, max buffer bindings). A renderer that assumes a feature exists and falls back silently when it does not produces different output or crashes on unsupported hardware.

Handle feature differences through an explicit capability query system:

- **Query capabilities at startup** (supported formats, feature levels, limits) and store them as a capability set the renderer consults.
- **Define feature tiers** (minimum, standard, high) that map to capability sets, and ensure the renderer degrades gracefully across tiers — a feature unavailable on a tier uses a fallback path, not a crash.
- **Make fallbacks deliberate and tested**, not silent. A fallback path that is never exercised will fail the one time it is needed (on the hardware you do not own). Test the fallback paths on the tiers that use them.
- **Do not lowest-common-denominator everything.** Supporting only the features every platform has cripples the renderer on capable hardware. Support the capable platforms' features and provide tested fallbacks for the rest.

### Reconcile Coordinate Conventions Across APIs

Coordinate conventions differ across APIs and are a persistent source of subtle rendering bugs. The handedness of clip space (left-handed in D3D, right-handed in OpenGL/Vulkan), the Y direction of the framebuffer (Y-down in Vulkan/D3D, Y-up in OpenGL), the Z range of clip space ([0,1] in D3D/Vulkan, [-1,1] in OpenGL), and the UV origin (top-left vs bottom-left) all differ. A renderer that uses one convention in its math and lets each backend apply (or not apply) the correction produces flipped, inverted, or depth-wrong output on some platforms.

Pick one convention internally (the renderer's math uses one handedness, one clip-space Z range, one UV origin) and apply the correction at the boundary where the renderer's convention meets the API's. Centralize the correction (a projection matrix adjustment, a Y-flip in the viewport, a UV flip in the sampler) so it is applied consistently, not scattered through the backend code where one missed correction produces a platform-specific bug. Document the chosen convention so every backend author applies the same correction.

### Share Shaders Through An Intermediate Representation, Not By Hand-Maintaining Per-Language Copies

Maintaining separate shader source for each language (HLSL for D3D, GLSL for OpenGL/Vulkan, MSL for Metal, WGSL for WebGPU) guarantees divergence: a fix in one is forgotten in another, and the platforms render differently. The robust approach is to author shaders in one language (or a high-level DSL) and cross-compile to each target:

- **HLSL as the source**, cross-compiled via DXC to DXIL (D3D) and to SPIR-V (Vulkan), and via SPIRV-Cross to MSL (Metal) and WGSL (WebGPU). HLSL is the most common source because the D3D and Vulkan toolchains both consume it.
- **GLSL as the source**, compiled to SPIR-V, then cross-compiled to MSL, HLSL, and WGSL.
- **A custom DSL or a material-graph system** that generates the target language, giving full control at the cost of building and maintaining the generator.

Whichever source is chosen, the key discipline is that there is one source of truth and the per-platform outputs are generated, not hand-edited. Verify that the cross-compiled shaders produce equivalent output across platforms (a reference image comparison), because the cross-compilers can introduce subtle differences in precision, resource binding, or control flow.

### Account For Synchronization Semantics Differences

Synchronization (ensuring a GPU write is complete before a dependent read) is expressed differently across APIs and is a common source of cross-platform correctness bugs. Vulkan and D3D12 require explicit pipeline barriers and memory barriers with precise access flags; Metal has a simpler but different barrier model; OpenGL and D3D11 hide synchronization in the driver. A renderer that uses one synchronization pattern and assumes it maps to every API produces races on the explicit APIs (missing barriers) or unnecessary stalls on the implicit ones.

Centralize synchronization in the abstraction, expressed as data dependencies (this pass writes this resource, the next pass reads it), and let each backend translate the dependency into the API's barrier mechanism. Do not scatter raw barrier calls through the backend code, because a missing or over-broad barrier is a per-platform bug. Test synchronization under the worst case (many dependent passes, async compute interleaved with graphics) on each platform, because a synchronization bug that does not manifest on one GPU may manifest on another with different execution timing.

### Profile On Every Target, Because Performance Characteristics Diverge

A rendering technique that is fast on one platform can be slow on another, even at the same capability tier. The explicit APIs reward explicit management (pre-built pipeline state objects, descriptor batching, minimal state changes) and penalize naive patterns; the implicit APIs hide some costs but have their own cliffs (driver recompilation on unexpected state, synchronization stalls the application cannot control). Mobile GPUs (tilers) have entirely different cost models than desktop GPUs (immediate-mode), and a technique optimal on one is suboptimal on the other — for example, mobile tilers are sensitive to the number of render passes (each pass forces a tile resolve), so merging passes is high-leverage on mobile and less so on desktop.

Profile on every target platform with its native profiler, not just the lead platform. A frame that hits budget on desktop and misses on mobile has a mobile-specific problem that desktop profiling will never reveal. Keep representative hardware for each target on the desk and profile routinely.

## Common Traps

### Designing The Abstraction Against The High-Level API

Building the abstraction atop OpenGL/D3D11 semantics, then being unable to express the explicit control Vulkan/D3D12/Metal require, producing a leaky or lowest-common-denominator abstraction. Design against the explicit APIs; they are the most demanding.

### Assuming A Feature Exists Across All Platforms

Using a texture format, feature level, or limit without querying, then crashing or rendering wrong on hardware that lacks it. Query capabilities at startup; define feature tiers with tested fallbacks.

### Unreconciled Coordinate Conventions

Using different handedness, clip-space Z, Y direction, or UV origin per backend without centralized correction, producing flipped or depth-wrong output on some platforms. Pick one internal convention and correct at the boundary consistently.

### Hand-Maintaining Shaders Per Language

Keeping separate HLSL, GLSL, MSL, and WGSL source that diverges over time, so platforms render differently. Author in one language and cross-compile; verify output equivalence across platforms.

### Scattered Synchronization Calls

Placing raw barrier calls throughout the backend code, so a missing or over-broad barrier is a per-platform race or stall. Centralize synchronization as data dependencies and let each backend translate them.

### Profiling Only On The Lead Platform

Optimizing for desktop and shipping a mobile build that misses its frame budget, because the mobile-specific costs (tile resolves, bandwidth) were never profiled. Profile on every target with native tooling.

### Lowest-Common-Denominator Feature Support

Supporting only what every platform has, crippling the renderer on capable hardware. Support capable platforms' features with tested fallbacks for the rest.

### Treating Backend Differences As Edge Cases

Dismissing cross-platform rendering differences as rare, when they are the default state of shipping to diverse hardware. Budget for cross-platform verification as a first-class cost, not a final QA step.

## Self-Check

- [ ] The graphics abstraction is designed against the explicit, low-overhead APIs (Vulkan, D3D12, Metal) so it can express explicit control, and the higher-level backends implement atop that model rather than constraining it.
- [ ] Feature differences are handled through an explicit capability query at startup, with defined feature tiers and tested fallback paths — no feature is assumed present, and fallbacks are exercised, not silent.
- [ ] Coordinate conventions (handedness, clip-space Z range, framebuffer Y direction, UV origin) are reconciled to one internal convention with centralized boundary correction, documented so each backend applies it consistently.
- [ ] Shaders are authored in one source language and cross-compiled to each target (HLSL/GLSL/MSL/WGSL), not hand-maintained per language, and cross-platform output equivalence is verified by reference image comparison.
- [ ] Synchronization is centralized as data dependencies in the abstraction and translated per-backend, not scattered as raw barrier calls; synchronization was tested under worst-case pass interleaving on each platform.
- [ ] The renderer was profiled on every target platform with native tooling, including mobile tiler-specific costs (render pass count, bandwidth), not only on the lead desktop platform.
- [ ] Capable platforms use their advanced features (with fallbacks for others), rather than lowest-common-denominator support that cripples capable hardware.
- [ ] Cross-platform rendering differences are treated as the default, and verification across all target hardware and APIs is budgeted as a first-class activity, not deferred to final QA.
