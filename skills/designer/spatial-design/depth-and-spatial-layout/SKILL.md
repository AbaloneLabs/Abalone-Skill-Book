---
name: depth_and_spatial_layout.md
description: Use when the agent is arranging content across depth in a spatial or 3D interface, layering panels, positioning objects in Z-space, designing parallax, occlusion, shadows, scale, or deciding how near and far elements relate to the user and to each other.
---

# Depth And Spatial Layout

In a flat interface, layout is a two-dimensional problem: where things sit left, right, above, and below. In a spatial interface, layout becomes a three-dimensional problem where every element also has a depth, and depth changes how the element is perceived, reached, and understood. Depth is not a visual effect. It is a structural dimension that carries hierarchy, focus, grouping, and physical plausibility.

The judgment problem is that depth is powerful but treacherous. Used well, it makes a spatial interface feel natural: closer objects feel more important, layers separate cleanly, and the eye understands what is foreground and what is reference. Used poorly, depth causes eye strain, ambiguous grouping, unreachable controls, and a sense that the interface is floating arbitrarily rather than resting in a coherent world. The agent's job is to treat depth as a layout discipline with rules, not as a decoration to sprinkle on top of a flat design.

Use this skill before arranging panels, windows, objects, menus, or annotations across depth in augmented reality, mixed reality, virtual reality, or any 3D interface. The goal is to prevent the agent from producing depth layouts that are uncomfortable, ambiguous, physically implausible, or that fight the task the user is trying to accomplish.

## Core Rules

### Treat Depth As Hierarchy, Not Just Distance

In spatial layout, nearer reads as more important, more immediate, and more actionable. This is the deepest structural tool the medium offers, and it should be used deliberately.

Use depth to express:

- the active or focused object versus available alternatives;
- a summoned modal or dialog versus the base environment;
- primary content versus supporting reference material;
- an action about to be taken versus context behind it.

When depth aligns with importance, the user understands the interface at a glance. When it contradicts importance, for example a background panel closer than the active tool, the layout feels wrong even if the user cannot say why. Audit the depth order against the intended hierarchy and resolve contradictions.

### Establish A Coherent Depth Frame

A spatial interface needs a frame of reference for depth, just as a flat interface needs a grid. Without it, elements float at arbitrary distances and the scene feels chaotic.

Define:

- a resting depth where primary content lives;
- a near zone for focused, interactive work;
- a mid zone for reference and supporting panels;
- a far zone for ambient or background context;
- consistent spacing between layers so relationships read clearly.

Users do not need to know the frame explicitly, but they feel its absence. A coherent frame makes the space legible; an incoherent one makes every element feel randomly placed. Reuse the same depth frame across the product so users build expectations.

### Respect Comfortable Focal Distances

The human visual system converges and focuses at a real distance. Spatial displays create a conflict between where the eyes converge and where they focus, and this conflict causes strain when content sits at uncomfortable depths or jumps between depths rapidly.

Reduce strain by:

- keeping primary content within a comfortable focal range, typically near arm's length to a bit beyond;
- avoiding very close placement of dense text or fine detail;
- avoiding rapid, large jumps in depth that force the eyes to reconverge;
- limiting the number of simultaneous depth layers the user must track;
- using scale and motion to reduce the need to physically refocus constantly.

Comfort is cumulative. Small strains over a long session become real pain and exclusion. Treat focal comfort as a layout constraint, not a polish step.

### Use Occlusion, Shadow, And Scale Consistently

Depth is communicated through cues: occlusion, shadow, scale, and parallax. These cues must be consistent, or the user's depth perception breaks.

Maintain consistency:

- nearer objects occlude farther ones predictably;
- shadows fall in a consistent direction and soften with distance;
- scale follows perspective, so distant objects appear smaller unless intentionally world-sized;
- parallax during head movement reinforces, not contradicts, the depth order.

Inconsistent cues, a near object that fails to occlude, a shadow pointing the wrong way, or scale that ignores distance, make the scene feel broken even when each element looks fine in isolation. Define the lighting and perspective model once and apply it everywhere.

### Group By Spatial Proximity In All Three Dimensions

Grouping in flat design relies on two-dimensional proximity. In spatial design, grouping must consider depth too. Two panels that are close in X and Y but far apart in Z do not read as a group.

When grouping:

- keep related elements close in all three dimensions;
- separate unrelated groups with clear gaps in depth as well as position;
- avoid interleaving unrelated layers, which destroys grouping;
- use consistent depth offsets within a group so it reads as a unit.

A common failure is stacking many panels at similar screen position but slightly different depths, which makes the layering ambiguous and the grouping unclear. Either commit to clear depth separation or collapse the layers.

### Match Depth Treatment To Physical Plausibility

Spatial interfaces feel more comfortable when they respect physical intuition, even when they are not strictly realistic. Objects that rest on surfaces, have weight, cast shadows, and occupy believable space are easier to accept than objects that float arbitrarily.

Aim for plausibility:

- let panels rest on or near detected surfaces when available;
- give objects consistent scale relative to the room and the user;
- avoid content that clips through walls, floors, or furniture without intent;
- use shadows and contact to ground floating objects;
- reserve impossible physics for deliberate moments, not as a default.

Plausibility is not realism for its own sake. It reduces cognitive load because the user's existing spatial instincts apply. The more the interface behaves like a believable space, the less the user has to learn.

### Design For Movement Through Depth

Users move in spatial interfaces, and movement changes which elements are near, far, visible, and occluded. A layout that works from one standing position may break when the user takes a step.

Consider:

- what happens to depth relationships when the user approaches or retreats;
- whether near content becomes overwhelming or far content becomes unreachable;
- how occlusion changes as the user moves around world-locked objects;
- whether the user can comfortably reach interactive targets from plausible positions;
- how to guide the user back to the comfortable volume if they wander.

Movement is part of the medium. Design the layout as a space to be inhabited, not just a picture to be viewed from one spot.

### Provide Clear Depth Transitions

When content changes depth, for example a panel summoning forward or a modal appearing, the transition should communicate the change. Sudden depth jumps disorient; smooth, signaled transitions help the user track what moved and why.

For depth transitions:

- animate the movement so the eye can follow;
- keep transitions brief but not instantaneous;
- signal the cause, a selection, a summon, a focus change;
- return content to its resting depth when the moment passes;
- avoid many simultaneous depth transitions, which overload the user.

Depth transitions are a tool for focus. Use them sparingly so they retain meaning.

## Common Traps

### Arbitrary Floating Layers

Panels placed at random depths without a frame of reference make the scene feel chaotic and make grouping ambiguous. Establish and reuse a coherent depth frame.

### Depth That Contradicts Hierarchy

A background reference panel placed closer than the active tool confuses the user even when nothing is technically wrong. Align depth with importance.

### Too Many Simultaneous Layers

Forcing the user to track many depth layers at once causes strain and ambiguity. Collapse or merge layers until the depth order reads clearly.

### Inconsistent Lighting And Perspective

Shadows, occlusion, and scale that do not follow a single model break depth perception. Define the model once and apply it everywhere.

### Clipping Through Real Surfaces

Content that intersects walls, floors, or furniture without intent destroys plausibility and can block interaction. Respect detected surfaces or handle clipping deliberately.

### Rapid Depth Jumps

Fast, large changes in focal depth force the eyes to reconverge and cause discomfort. Animate transitions and keep primary content in a stable focal range.

### Ignoring Movement And Reach

A layout that works from one position may put content out of reach or out of view when the user moves. Design for inhabitation, not just viewing.

## Self-Check

- [ ] Depth is used deliberately to express hierarchy, focus, and grouping, and the depth order aligns with intended importance.
- [ ] A coherent depth frame defines resting, near, mid, and far zones, and is reused consistently across the product.
- [ ] Primary content sits within a comfortable focal range, dense detail is not placed too close, and rapid large depth jumps are avoided.
- [ ] Occlusion, shadow, scale, and parallax follow a single consistent lighting and perspective model.
- [ ] Grouping considers all three dimensions, and related elements are close in depth as well as position.
- [ ] Depth treatment respects physical plausibility, with objects grounded by surfaces, shadows, and consistent scale.
- [ ] The layout accounts for user movement, changing occlusion, reachability, and guidance back to the comfortable volume.
- [ ] Depth transitions are animated, signaled, brief, and used sparingly so they retain meaning.
- [ ] No unrelated layers interleave at similar depths in a way that makes grouping ambiguous.
- [ ] The scene has been considered as an inhabited space, not only as a static view from one position.
