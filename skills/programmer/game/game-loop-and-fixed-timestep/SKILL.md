---
name: game_loop_and_fixed_timestep.md
description: Use when the agent is building or fixing a game loop, simulation loop, physics update, animation tick, or real-time render loop; deciding between fixed and variable timestep; implementing an accumulator or sub-stepping pattern; diagnosing frame-rate-dependent behavior, tunneling collisions, unstable physics, determinism loss, desync between clients, or inconsistent simulation at low frame rates; or porting a game between devices with different refresh rates. Covers frame independence, the spiral of death, interpolation vs extrapolation, deterministic simulation, and the tradeoffs of decoupling simulation rate from render rate.
---

# Game Loop And Fixed Timestep

The game loop is the heartbeat of any real-time simulation, and its design is not "run update then render." It is a decision about how time flows through the simulation: whether the physics advances in fixed steps or in proportion to wall-clock time, whether the same inputs produce the same world on a 30 FPS phone and a 240 FPS desktop, and what happens when a frame takes longer to render than the budget allows. Getting this wrong produces symptoms that look like physics bugs, network desyncs, or "it works on my machine" — collisions tunnel through walls at high speed, objects jitter, replays do not reproduce, and two players see different outcomes from the same inputs.

Agents tend to treat the loop as plumbing and reach for the simplest form (`position += velocity * dt`), because the happy path works and the failures only appear under frame drops, high velocities, or cross-device testing. The judgment problem is deciding which update model the game's determinism, stability, and networking requirements demand, and then implementing it correctly — including the edge cases (huge frame gaps, paused state, slow-mo, first frame) that break naive implementations.

## Core Rules

### Choose The Update Model Based On What Must Be Deterministic And Stable

There are three common models, and the choice is a tradeoff, not a default.

- **Variable timestep.** `dt` is measured each frame and applied directly. Simple, smooth animation, but non-deterministic (floating point accumulation differs across runs and devices) and unstable for physics — large `dt` steps make integrators diverge, springs explode, and fast-moving objects tunnel through collisions. Acceptable for presentation-layer animation, camera easing, and particle effects where exact reproduction does not matter.
- **Fixed timestep.** The simulation always advances by a constant `dt` (e.g., 1/60). Deterministic, numerically stable, and reproducible — the same inputs produce the same trajectory regardless of render rate. Required for physics integrity, replays, lockstep networking, and any simulation that must be verified. The cost is that render rate and simulation rate are now decoupled, which requires interpolation or extrapolation to keep visuals smooth.
- **Semi-fixed timestep.** Run the simulation in fixed sub-steps, but size the sub-step based on the frame's `dt` (e.g., cap at 1/120). A compromise that bounds the worst-case `dt` without committing to a single fixed rate. Useful when you need stability but not strict determinism.

Strong choice: a physics-heavy competitive game uses fixed timestep because determinism and stability are non-negotiable. Weak choice: using variable timestep for a physics simulation "because it is simpler," then shipping tunneling and desync bugs. Name the invariant — determinism, stability, smoothness — before picking the model.

### Use An Accumulator To Decouple Simulation Rate From Render Rate

The standard fixed-timestep loop runs the simulation in constant-sized steps driven by an accumulator, not by the frame rate. The shape is: measure wall-clock `dt`, add it to an accumulator, and while the accumulator holds at least one fixed step, run one simulation step and subtract the fixed step. Render once per frame using the leftover accumulator value to interpolate between the previous and current simulation states.

This decoupling is the point. A 240 Hz monitor and a 30 Hz phone run the *same* simulation steps; only the number of steps per frame and the interpolation alpha differ. Without it, "frame independence" is a slogan, not a property. The accumulator also defines what to do with the remainder — it carries forward, so the simulation never loses or invents time.

### Bound The Accumulator To Avoid The Spiral Of Death

If a frame takes longer to render than several simulation steps can cover, the accumulator grows, the next frame runs even more steps to catch up, rendering slows further, and the system spirals into a frozen state where every frame tries to simulate an ever-larger backlog. This is the "spiral of death," and it is the most common failure of a naive accumulator loop.

Defend against it explicitly. Cap the accumulator (or the number of steps per frame) to a maximum — e.g., clamp `dt` to 100 ms before accumulating, or limit to 5 steps per frame and drop the rest. When the cap is hit, the simulation falls behind real time deliberately rather than freezing. Decide the policy consciously: slow down (accept that game time drifts behind wall time), skip ahead (jump the simulation, risking missed collisions), or pause. A game that freezes the moment the GPU hitches has an unbounded accumulator.

### Interpolate, Do Not Extrapolate, For Render Smoothness

When the render rate is higher than the simulation rate, multiple renders occur between simulation steps. Naively rendering the last simulated state produces stutter. The correct fix is interpolation: blend between the previous and current simulation states using the accumulator remainder as the alpha. Interpolation always lags simulation by up to one step, which is acceptable and invisible.

Extrapolation (predicting forward from the current state using velocity) avoids the lag but introduces a different problem: it guesses, and the guess is wrong whenever a collision, input, or state change happens in the next step. Objects visibly snap back when the prediction disagrees with the next simulation result. Prefer interpolation for entities whose motion can change abruptly (players, projectiles), and reserve extrapolation for cases where the one-step lag is genuinely unacceptable and motion is predictable.

### Keep The Simulation Deterministic When Determinism Is Required

For replays, lockstep networking, and recorded tests, the simulation must produce identical output from identical inputs. Determinism is a whole-system property and it breaks silently. The requirements:

- **Fixed timestep** is necessary but not sufficient.
- **Fixed-order iteration** over entities and components — never iterate a hash map or unordered set whose order varies by run.
- **No floating-point operations whose result varies by platform or compiler flags** — be wary of fused multiply-add, transcendentals, and any code path that differs by CPU. Use fixed-point or carefully constrained float math for critical paths.
- **No reliance on wall-clock time, random sources, or thread scheduling inside the simulation** — seed RNGs deterministically and feed them only logged inputs.
- **Same inputs, same order** — record every input with its simulation frame and replay them in order.

Treat any non-determinism as a bug to be found, not a curiosity. A replay that desyncs after 30 seconds has a hidden dependence on ordering, float behavior, or unlogged input.

### Handle Edge Frames Explicitly

The first frame, the frame after a pause, the frame after a window resize, and the frame after the system suspended the process all have pathological `dt` values (near zero, or many seconds). A raw `dt` from the timer in these cases produces a simulation step that is meaningless or destructive. Always clamp `dt` on entry, and special-case the resume frame: either reset the accumulator to zero (accept that time was lost) or advance it by a fixed resume budget. Pausing should freeze the accumulator's growth, not accumulate a giant backlog to process on resume.

## Common Traps

### Using Wall-Clock `dt` Directly For Physics

Multiplying velocity by the measured frame `dt` feels natural and works at 60 FPS, but it makes physics non-deterministic and unstable the moment `dt` spikes. A one-second hitch becomes a giant integration step: springs overshoot, constraints explode, and fast objects pass through walls. Physics must run on a fixed step; `dt` belongs only to interpolation and presentation.

### Forgetting To Cap The Accumulator

Implementing the accumulator loop without a ceiling works in profiling and fails the first time the device thermal-throttles or the player alt-tabs. The loop spends an entire frame catching up, falls further behind, and the game appears to hang. Always bound the catch-up work; "it ran fine on my machine" hides an unbounded loop.

### Rendering The Last Simulated State Without Interpolation

Running fixed-step physics at 50 Hz on a 60 Hz display and rendering the raw state produces a visible 10 Hz stutter, because some frames reuse the same state and some advance. The fix (interpolation) is cheap and mandatory whenever render rate does not evenly divide the simulation rate. Omitting it is the most common cause of "the physics feels jittery."

### Extrapolating Entities That Can Collide

Predicting player or projectile positions forward to smooth rendering, then snapping back when the next step reports a collision, produces a flickering "ghost ahead of the wall" artifact. Interpolate anything whose motion can be interrupted; the one-step lag is invisible and the snap is not.

### Assuming Float Math Is Deterministic Across Devices

Treating IEEE 754 as a guarantee of reproducibility, then shipping a replay that desyncs between an x86 desktop and an ARM phone because of FMA contraction, different `sin` implementations, or compiler reordering. Determinism across hardware requires constraining the math (fixed-point, or floats with documented precision rules) and testing it; it is not free from using floats.

### Iterating Unordered Collections In The Simulation

Looping over a hash-set of entities whose iteration order depends on insertion history or memory allocator, then wondering why replays diverge. Simulation iteration order is part of the contract; use ordered structures or a stable entity list.

### Resetting `dt` To A Constant "To Be Safe"

Clamping `dt` to exactly 1/60 every frame to avoid hitches — which silently turns a fixed-step loop into a variable one and destroys frame independence. Clamping is for *bounding*, not for *normalizing*; the clamp should only trigger on pathological frames, and the steady-state `dt` should reflect real elapsed time feeding the accumulator.

## Self-Check

- [ ] The update model (variable, fixed, or semi-fixed) was chosen explicitly based on whether determinism, stability, or smoothness is the governing invariant — not picked by default.
- [ ] Fixed-step simulation uses an accumulator that decouples simulation rate from render rate, so a 30 FPS device and a 240 FPS device run the same simulation steps.
- [ ] The accumulator is bounded (clamped `dt` or capped step count per frame) to prevent the spiral of death, and the policy on falling behind (slow-down, skip, or pause) is deliberate.
- [ ] Rendering interpolates between previous and current simulation states using the accumulator remainder; entities whose motion can change abruptly are interpolated, not extrapolated.
- [ ] Determinism-critical paths (replays, lockstep netcode, recorded tests) use fixed timestep, fixed iteration order, constrained float math, seeded RNG, and logged inputs — and a cross-device replay test exists.
- [ ] Edge frames (first frame, resume after pause, post-suspend, resize) clamp `dt` and do not feed a pathological value into the simulation.
- [ ] Physics does not integrate against raw wall-clock `dt`; presentation and interpolation are the only consumers of variable `dt`.
- [ ] A deliberate load test (frame drops, low FPS, alt-tab) was run and the loop neither froze, stuttered persistently, nor desynced.
