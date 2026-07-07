---
name: physics-simulation-stability-and-performance.md
description: Use when the agent is designing or reviewing the stability and performance of a game's physics simulation, managing solver behavior, fixed timesteps, resting objects, stacking, and large-body counts, or evaluating whether the simulation remains stable and performant under load; trigger contexts include physics stability, simulation performance, solver tuning, fixed timestep, resting objects, stacking stability, rigidbody explosion, physics optimization, broadphase, continuous collision detection, CCD, simulation step, physics load, framerate coupling; important risks include exploding rigidbodies, tunneling, framerate-coupled physics, stacking collapse, and performance collapse under body count.
---

# Physics Simulation Stability And Performance

A physics simulation that is correct in a demo will fail in a real game, because real games stack objects, accumulate resting bodies, run on variable framerates, and push body counts the demo never reached, and these conditions expose the instabilities and performance cliffs that define shipped physics. The agent is usually asked to design or review a simulation's stability and performance while the solver settings, the timestep strategy, the collision detection mode, and the expected load are under-specified. The judgment problem is that physics stability and performance are in tension (more solver iterations stabilize but cost frames), and the correct configuration is the one that stays stable and performant across the game's actual conditions, not the one that looks good in a clean test.

The agent tends to miss this because stability and performance problems only appear at scale and under edge conditions (tall stacks, dense piles, fast-moving bodies, low framerates), and because the simulation is tuned in clean demos that avoid exactly the conditions that break it. The harm is shipped games where objects explode out of piles, where fast bodies tunnel through walls, where physics framerate couples to rendering and slows the whole game, or where body counts collapse performance. Use this skill to slow the decision down enough to expose the load and edge conditions, then make the recommendation appropriately conservative when stability, determinism, and performance are at stake.

## Core Rules

### Use A Fixed Timestep Decoupled From Render Framerate
Physics must run on a fixed timestep, decoupled from the render framerate, so simulation behavior is consistent across hardware and framerates. The agent should ensure the physics step is fixed (not scaled to frame delta), that the game accumulates time and steps the simulation in fixed increments, and that the render layer interpolates between physics states for smoothness. Framerate-coupled physics produces different game behavior on different hardware and is a fundamental defect.

### Tune The Solver For Stability Under Real Load
Solver settings (iteration count, solver type, constraint tolerances) determine stability, and they must be tuned under the game's real load, not a clean demo. The agent should test with the maximum expected body counts, the tallest stacks, and the densest piles, and increase solver iterations or switch solver type until resting objects and stacks remain stable. A solver tuned on light loads will produce exploding piles and jittering stacks in the real game.

### Manage Resting Objects And Sleeping
Resting objects must be put to sleep to avoid perpetual solver computation, and the sleep system must be tuned to neither wake too easily nor sleep too aggressively. The agent should configure sleep thresholds (velocity, angular velocity, time-at-rest), ensure woken objects re-evaluate correctly when disturbed, and verify that sleeping does not cause objects to ignore legitimate collisions. Poor sleep tuning either wastes performance (nothing sleeps) or causes defects (sleeping objects fall through moving platforms).

### Prevent Tunneling With Continuous Collision Detection For Fast Bodies
Fast-moving bodies tunnel through thin colliders under discrete collision detection, and continuous collision detection (CCD) must be used where tunneling is possible. The agent should identify fast bodies (projectiles, fast-moving characters, swung objects), enable CCD for them, and verify they cannot pass through walls, floors, or other colliders at speed. Tunneling produces objects falling out of the world and is a defect wherever fast bodies and thin geometry coexist.

### Choose Broadphase And Narrowphase Appropriate To Scene Scale
Broadphase and narrowphase algorithms determine performance scaling with body count, and the choice must match the scene. The agent should choose a broadphase (sweep-and-prune, grid, BVH) appropriate to the spatial distribution, ensure the narrowphase handles the game's shape types efficiently, and profile at the expected body counts. A broadphase mismatch produces O(n²) performance cliffs that only appear when the scene fills up.

### Bound Body Counts And Destructible Debris
Physics body counts must be bounded, because every active body costs solver time, and unbounded debris (from destruction, explosions, dismemberment) can collapse performance. The agent should cap concurrent active bodies, merge or freeze debris that settles, and decide when debris is removed or converted to non-physics representation. Unbounded debris is the most common cause of physics-induced performance collapse.

### Profile Under Worst-Case Load, Not Average
Physics performance must be profiled under worst-case load — the most bodies, the densest piles, the fastest bodies, the lowest target framerate hardware — not the average scene. The agent should construct worst-case scenarios, measure frame time and solver time, and ensure the simulation stays within budget under the conditions the game will actually encounter. Average-load profiling hides the cliffs that reach players.

## Common Traps

### Framerate-Coupled Physics Producing Hardware-Dependent Behavior
Physics is scaled to frame delta rather than run on a fixed timestep, and the game behaves differently on different hardware. The trap is that it feels fine on the dev machine. The false signal is that the simulation runs at framerate. The harm is inconsistent game behavior, speedrun invalidation, and bugs that only reproduce on certain hardware, because the physics was never decoupled from rendering.

### Exploding Piles From Under-Tuned Solvers
Stacks and piles explode or jitter because the solver was tuned on light loads and cannot handle the real game's body counts. The trap is that the demo stacks were stable. The false signal is that stacking works. The harm is the signature physics-comedy failure of objects launching out of piles, which undermines seriousness and fairness whenever players stack or pile.

### Tunneling Fast Bodies Through Walls
Fast projectiles or characters pass through thin walls because CCD was not enabled. The trap is that discrete detection works at low speeds. The false signal is that collisions register in testing. The harm is objects fall out of the world, projectiles skip walls, and the game breaks precisely in the fast-action moments where reliability matters most.

### Sleep Tuning That Drops Objects Or Wastes Performance
Sleep thresholds are set too aggressively (objects sleep through collisions, fall through moving platforms) or too loosely (nothing sleeps, performance wastes on resting bodies). The trap is that sleep seems like an optimization detail. The false signal is that the simulation runs. The harm is either falling-through-world defects or performance drain, because the sleep system was never tuned to the game's motion patterns.

### Broadphase Mismatch Causing O(n²) Cliffs
The broadphase does not match the scene's spatial distribution, and performance collapses as body count rises. The trap is that the scene runs fine when empty. The false signal is that the broadphase works. The harm is a performance cliff that appears only when the game fills up, discovered at scale when the broadphase is hard to change.

### Unbounded Debris Collapsing Performance
Destruction or explosions spawn unlimited active debris, and the solver drowns. The trap is that destruction looks great in isolation. The false signal is that the explosion performed well. The harm is performance collapse whenever sustained destruction occurs, because debris was never capped, merged, or cleaned up.

### Average-Load Profiling Hiding The Cliffs
Performance is profiled on average scenes, and the worst-case cliffs are never measured. The trap is that the average runs at framerate. The false signal is that the simulation is performant. The harm is the game collapses on the hardware and scenes players actually encounter, because the worst case was never constructed or measured.

### Presenting Solver Tuning As Neutral Engineering
Solver and performance decisions are often judgment calls, but the agent should not present them as if they were objective. State what is known (the load, the body counts, the target hardware), what is inferred (the stability margin), and what is a budget judgment, so the team decides with the tradeoffs visible.

## Self-Check

- [ ] Does the simulation run on a fixed timestep decoupled from render framerate, with interpolation for smoothness?
- [ ] Is the solver tuned for stability under the game's real maximum load — tallest stacks, densest piles, highest body counts?
- [ ] Is the sleep system tuned to neither wake too easily nor sleep through legitimate collisions?
- [ ] Is continuous collision detection enabled for fast bodies that could tunnel through thin geometry?
- [ ] Are broadphase and narrowphase chosen to match the scene's spatial distribution and shape types, profiled at expected body counts?
- [ ] Are concurrent active bodies bounded, with debris capped, merged, frozen, or cleaned up to prevent solver overload?
- [ ] Is performance profiled under worst-case load on the lowest target hardware, not just average scenes?
- [ ] Does the output distinguish stability and performance decisions that protect the shipped game from those that optimize the demo?
- [ ] Are the timestep, solver settings, CCD scope, body caps, and profiling scenarios specific enough for engineering to implement without re-deciding?
- [ ] Is uncertainty about worst-case load and hardware variance named, with the tradeoffs that would change the recommendation made explicit?
