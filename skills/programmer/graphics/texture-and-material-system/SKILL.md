---
name: texture_and_material_system.md
description: Use when the agent is designing a texture or material system, choosing texture compression formats, building mipmaps or texture atlases, creating a material graph or shader permutation system, implementing texture streaming or LOD, budgeting VRAM for textures, or debugging texture bandwidth, aliasing, streaming pop-in, or material permutation explosion.
---

# Texture And Material System

Textures and materials are where the visible quality of a renderer meets the hard limits of memory and bandwidth. A scene can hold gigabytes of texture data, but the GPU has a fixed VRAM budget and a fixed memory bandwidth, and every texel fetched costs both. The texture and material system is the machinery that decides which texels live in VRAM, in what format, at what resolution, and how the shader selects among material variations — and its design determines whether the renderer hits its quality target within its memory budget or thrashes bandwidth and runs out of VRAM. The difficulty is that the decisions are interlocking: a compression format choice affects quality and bandwidth; a mipmap strategy affects aliasing and memory; a material model affects shader permutations and draw call count; and a streaming system affects load time and pop-in. Optimizing one in isolation often regresses another.

Agents tend to treat textures as "load the PNG, sample it in the shader" and materials as "one shader per object," because the happy path renders correctly and the failures (bandwidth-bound frames, VRAM exhaustion, shader permutation explosion, streaming pop-in) only appear at scale or on constrained hardware. The judgment problem is deciding the compression format against the data type and quality target, the mipmap and atlas strategy against access patterns and memory, the material representation against the permutation count it generates, and the streaming and LOD strategy against the memory budget and the visual cost of transitions. Getting this wrong produces a renderer that looks good in a screenshot and stutters or exhausts memory in a real scene.

## Core Rules

### Choose Texture Compression Formats Against Data Type And Quality Target

Hardware-native block-compressed formats (BCn on desktop, ASTC on mobile and modern desktop, ETC2 as a mobile baseline) are sampled directly by the GPU without decompression, saving 4x to 8x in bandwidth and VRAM for a controllable quality cost. An uncompressed texture is almost always a mistake in a shipped renderer. The format choice depends on what the texture encodes:

- **Color/albedo (opaque):** BC1 (4 bpp) or ETC2 RGB for baseline; BC7 or high-quality ASTC when alpha or higher fidelity is needed.
- **Color with alpha:** BC3 (DXT5, alpha in a separate compressed channel), BC7, or ASTC with alpha.
- **Normals:** BC5 (two-channel, reconstructs the third) for tangent-space normals; BC3 as a fallback. Never compress normals with a color format — the precision loss produces visible banding on smooth surfaces.
- **HDR / float data:** BC6H for HDR color (half-float, compressed); uncompressed RGBA16F/32F only when BC6H's precision is insufficient.
- **Single-channel (masks, height):** BC4 (one compressed channel) or the red channel of a format.

Choose the format against the data and the quality target, and verify with visual inspection on representative content. A format that looks fine on a flat color texture may band badly on a smooth gradient or a normal map. ASTC's configurable block size (from 4x4 high-quality to 12x12 tiny) lets you trade quality for size per texture — use it to fit the VRAM budget by giving important textures higher quality and background textures lower.

### Mipmap Everything Sampled At Varying Scales, And Understand What Mipmaps Cost And Save

A mipmap chain stores pre-filtered downsampled versions of a texture, and the GPU samples the level whose resolution matches the texel's screen-space coverage. Mipmapping serves two purposes: it prevents aliasing (the shimmering and moiré on distant minified textures, which is a correctness problem as much as an aesthetic one), and it dramatically improves texture cache behavior (distant samples fetch from small mip levels that fit in cache, instead of scattered fetches across a huge base texture). An unmipmapped texture sampled at a distance aliases badly and thrashes the cache.

The cost of mipmaps is additional memory — roughly 33% more than the base texture (the sum of the mipmap chain is base + base/4 + base/16 + ... = 4/3 * base). This is almost always worth it. The exception is textures that are never minified (UI textures rendered at 1:1, lookup tables sampled once), which gain nothing from mipmaps and waste the memory. Generate mipmaps for everything sampled at varying scales; skip them only for textures guaranteed to be sampled at native resolution or larger.

### Use Texture Atlases And Array Textures To Reduce Draw Calls And State Changes

When many objects differ only by texture, giving each its own texture forces a separate draw call per object (each material/texture binding is a state change). Texture atlases (packing multiple small textures into one larger texture) and texture array textures (a stack of same-size textures addressable by an index) let many objects share one material and one draw call, with the per-object variation carried in a texture coordinate offset or an array index.

Tradeoffs:

- **Atlases** work with any API and any texture size, but introduce UV-bleeding at atlas boundaries (adjacent textures bleed into each other at mipmap levels, because downsampling blends across the border) and complicate mipmap generation (each sub-texture's mip chain must stay within its region, often via padding). Use half-texel insets or padding to control bleeding.
- **Array textures** avoid bleeding entirely (each layer is independent) and are sampled natively, but require all layers to share the same dimensions and format. They are the cleaner solution when the textures are uniform (a set of character textures, a set of terrain tiles).

Choose the technique against the data: array textures for uniform sets, atlases for irregular packing. The goal is to collapse many texture-only-different draws into few, reducing both draw calls and state changes.

### Design The Material Model To Bound Shader Permutations

A material defines how a surface is shaded — its shaders, its textures, its parameters. The naive approach is one shader per material, but a scene with hundreds of materials then has hundreds of shaders, each with its own pipeline state, and the driver cost of compiling and switching among them dominates. The other naive approach is one "uber-shader" with every feature as a uniform branch, but that produces a slow shader that diverges and cannot be optimized.

The practical middle ground is a material system that generates a bounded set of shader permutations from a material graph or a set of feature flags, compiled once per distinct permutation. Each material selects which features it uses (normal mapping yes/no, emissive yes/no, two layers vs one), and the system compiles a shader variant per unique combination. The discipline is to bound the number of features and combinations, because the permutation count is the product of the feature options and grows combinatorially. A material system with 10 independent boolean features produces 1024 permutations; most are unused, but the compile time and driver overhead scale with the count. Limit features to those that matter, share permutations across materials that use the same feature set, and cull unused permutations at build time.

### Budget VRAM And Stream Textures That Exceed It

A scene's total texture data often exceeds the GPU's VRAM, especially on constrained hardware. The texture system must decide which textures are resident, at what resolution, and when to load or evict them — this is texture streaming. The system maintains a memory budget, loads the mip levels each visible object needs (based on screen-space size), and evicts the mip levels of objects that are distant or off-screen.

Design the streaming system against the memory budget and the access pattern:

- **Prioritize by screen-space contribution.** Load the mip levels an object actually needs at its current distance; a distant object needs only low mip levels, and loading its base texture wastes VRAM.
- **Evict from the least-recently-used.** When the budget is full, evict textures (or their high mip levels) that are off-screen or distant, so VRAM is available for what is visible.
- **Handle the pop-in.** When a texture's needed mip level is not yet resident, the system must show something — a lower mip level, a default texture, a dithered transition. Sudden switches from low to high resolution (pop-in) are visually jarring; preload ahead of camera movement, use trilinear filtering to smooth transitions, or blend between mip levels during load.
- **Budget the streaming I/O.** Streaming from disk or network competes with other I/O; loading too many textures per frame stalls. Cap the per-frame streaming budget and prioritize within it.

### Separate The Material's Data From Its Evaluation

A common architectural error is baking everything into the shader (the textures, the lighting model, the feature flags) so that changing any of it requires a shader recompile. A cleaner separation puts the data (which textures, what parameters, what features) in the material asset and the evaluation (the shader code) in a smaller set of shader permutations. The material selects the permutation and binds the data; the shader evaluates. This separation lets artists iterate on materials without recompiling shaders, lets the engine reuse permutations across materials, and keeps the shader count bounded by the feature combinations rather than by the material count.

## Common Traps

### Uncompressed Or Wrong-Format Textures

Shipping RGBA8 or RGBA32F textures, or compressing normals with a color format, wasting bandwidth and VRAM or producing banding. Match the compression format to the data type; verify quality on representative content.

### No Mipmaps On Minified Textures

Leaving textures unmipmapped, producing aliasing (shimmer, moiré on distant surfaces) and cache thrashing. Mipmap everything sampled at varying scales; the 33% memory cost is almost always worth it.

### Atlas Bleeding At Mipmap Boundaries

Packing textures into an atlas without padding, so mipmap downsampling blends adjacent textures across the border and produces visible seams. Use padding, half-texel insets, or array textures to prevent bleeding.

### Shader Permutation Explosion

A material system with many independent feature flags generating thousands of shader permutations, dominating compile time and driver overhead. Bound the feature set, share permutations, and cull unused variants.

### Loading Full-Resolution Textures For Distant Objects

Streaming the base texture for every object regardless of screen-space size, exhausting VRAM on distant objects that need only low mip levels. Prioritize streaming by screen-space contribution; evict what is not visible.

### Visible Pop-In From Late Streaming

Showing a low-resolution or default texture until the needed mip streams in, with a hard switch when it arrives. Preload ahead of camera movement, smooth transitions with filtering or blending.

### One Shader Per Material

Compiling a separate shader for each of hundreds of materials, paying driver overhead for compilation and state switches. Generate a bounded set of permutations from feature flags; share permutations across materials with the same feature set.

### Ignoring The VRAM Budget

Loading all textures at full resolution and exhausting VRAM, causing the driver to spill to system memory and destroying performance. Budget VRAM explicitly; stream and evict to fit.

## Self-Check

- [ ] Textures use hardware-native compressed formats (BCn, ASTC, ETC2) chosen against the data type (color, alpha, normals, HDR, single-channel) and verified for quality on representative content — no uncompressed or wrong-format textures shipped.
- [ ] Every texture sampled at varying scales is mipmapped; the 33% memory cost is accepted because the aliasing prevention and cache benefit are worth it, and only guaranteed-1:1 textures are left unmipmapped.
- [ ] Texture atlases use padding or half-texel insets to prevent mipmap bleeding, or array textures are used for uniform texture sets to avoid bleeding entirely.
- [ ] The material system generates a bounded set of shader permutations from feature flags, shares permutations across materials, and culls unused variants — permutation count does not scale with material count.
- [ ] VRAM is budgeted explicitly; the streaming system loads mip levels by screen-space contribution, evicts least-recently-used textures under pressure, and stays within the per-frame I/O budget.
- [ ] Streaming pop-in is mitigated (preloading ahead of camera movement, filtered or blended transitions), not shown as a hard resolution switch.
- [ ] Material data (textures, parameters, feature selection) is separated from shader evaluation, so material iteration does not require shader recompiles and permutations are reused.
- [ ] The texture and material system was tested on the full range of target hardware (constrained VRAM, bandwidth limits) and worst-case scenes, not only on the dev machine with headroom.
