---
name: physics-feel-tuning-and-gameplay-readability.md
description: Use when the agent is tuning the feel of a game's physics simulation, balancing simulation realism against gameplay readability, deciding how physical objects should behave to serve the game, or reviewing whether physics supports or fights the player's intent; trigger contexts include physics feel, game feel, physics tuning, simulation realism, gameplay readability, physics-based gameplay, ragdoll, momentum, weight, physics interaction, object behavior, simulation parameters, gravity tuning, friction tuning; important risks include physics that feels floaty or sluggish, simulation that obscures intent, emergent chaos, and realism that undermines fairness.
---

# Physics Feel Tuning And Gameplay Readability

A game's physics simulation is not a realism dial; it is a feel instrument, and the agent is usually asked to tune it while the intended game feel, the readability requirements, and the tradeoff between simulation fidelity and player intent are all in tension. The judgment problem is that realistic physics can produce emergent chaos that obscures what the player meant to do, while tuned physics can feel arbitrary or gamey, and the correct setting is rarely the most realistic — it is the setting that makes the physics legible, responsive, and in service of the gameplay the player is trying to perform.

The agent tends to miss this because physics is tuned by chasing realism or by copying reference games, rather than by defining the feel the gameplay requires, and because physics problems are often misattributed to controls or animation. The harm is games where objects feel floaty or leaden, where the player's intended action produces an emergent mess, where the simulation introduces unfair variance, or where the physics that was meant to enable gameplay instead fights it. Use this skill to slow the tuning down enough to expose what feel the gameplay requires, then make the recommendation appropriately conservative when readability, fairness, and intent are at stake.

## Core Rules

### Define The Feel The Gameplay Requires Before Tuning
Before adjusting any parameter, state the feel the gameplay demands: weighty or floaty, snappy or momentum-driven, predictable or chaotic, grounded or exaggerated. The agent should derive the feel from the core loop (a precision platformer needs readable, low-variance physics; a sandbox may benefit from emergent chaos) and tune toward that target. Tuning without a defined feel target produces physics that is technically parameterized but experientially aimless.

### Favor Readability Over Realism Where They Conflict
When realistic simulation and gameplay readability conflict, readability wins. The agent should simplify or constrain physics where realism obscures intent (auto-righting overturned objects, snapping to grabbable states, limiting chaotic bounces), because the player must be able to predict what their action will produce. Realism that makes the game unreadable is a defect, regardless of its fidelity; the player interacts with the game's physics, not with nature's.

### Tune Momentum And Weight To Match Intent
Momentum and weight are the primary carriers of physics feel. The agent should tune acceleration, deceleration, mass, and friction so objects move with the sense of weight the gameplay requires, ensure momentum does not carry the player past their intended stop, and avoid both floaty (no sense of mass) and leaden (unresponsive) extremes. Momentum that fights the player's stopping or turning intent is perceived as fighting the player, regardless of its realism.

### Bound Emergent Chaos And Variance
Physics simulations produce emergent behavior, and unbounded emergence creates variance the player cannot predict or control. The agent should bound chaotic interactions (resting positions, stacking, multi-body collisions), cap or dampen extreme outcomes, and ensure the same action produces reliably similar results. Unbounded chaos is entertaining in a sandbox and destructive in a precision game; the bounds must match the genre.

### Ensure Physics Interactions Are Fair And Consistent
Physics-driven outcomes must be fair: the same action should produce the same result, and results should be attributable to player skill, not simulation lottery. The agent should test for variance in repeated interactions, eliminate non-deterministic ordering that changes outcomes, and ensure the player can learn the physics and improve. A physics interaction whose outcome varies unpredictably is unfair, because the player cannot develop skill against a moving target.

### Manage Determinism For Competitive And Reproducible Play
Where the game is competitive, speedrun-friendly, or replay-based, physics must be deterministic. The agent should ensure fixed timesteps, deterministic solver ordering, and consistent floating-point behavior, because non-deterministic physics breaks competitive fairness, speedrun records, and replay reproducibility. Determinism is a designed property of physics, not a default, and it must be enforced where the game depends on it.

### Coordinate Physics With Animation And Audio
Physics feel is co-authored with animation and audio, and inconsistency between them breaks the sense of weight. The agent should ensure animation blends with physical momentum (no snapping that contradicts mass), impacts produce audio proportional to physical energy, and the three systems present a unified sense of material and force. Physics that says "heavy" while animation says "light" produces a dissonance the player feels but cannot name.

## Common Traps

### Chasing Realism At The Expense Of Readability
The team tunes physics toward realism, and the simulation produces emergent chaos the player cannot read or control. The trap is that realism feels sophisticated. The false signal is that the simulation is accurate. The harm is the player's intended actions produce unpredictable outcomes, skill cannot develop, and the physics that was meant to enable gameplay instead obscures it.

### Floaty Or Leaden Weight From Untuned Parameters
Default or untuned mass, friction, and gravity produce objects that feel floaty (no sense of mass) or leaden (unresponsive), and neither matches the gameplay's feel target. The trap is that the parameters are physically plausible. The false signal is that the simulation runs. The harm is the game feels wrong in a way the player attributes to vague "game feel," because the weight was never tuned to intent.

### Unbounded Emergent Chaos In Precision Contexts
The simulation is allowed to produce full emergent chaos in a game that requires precision, and repeated actions produce wildly different results. The trap is that emergence is entertaining. The false signal is that the physics is rich. The harm is unfair variance, frustrated players, and a game whose outcomes feel like a lottery rather than a test of skill.

### Non-Deterministic Physics Breaking Fairness
Threading, solver ordering, or floating-point variation introduces non-determinism, and the same action produces different results. The trap is that the simulation is correct on average. The false signal is that the physics works. The harm is competitive unfairness, broken speedrun records, and replays that do not match, because determinism was assumed rather than enforced.

### Physics Fighting The Player's Stopping And Turning
Momentum carries the player past their intended stop or turn, and the physics is perceived as fighting the player. The trap is that momentum is realistic. The false signal is that the movement has weight. The harm is the player feels out of control, attributes failures to the physics rather than their skill, and the game feels unresponsive even when it is technically correct.

### Dissonance Between Physics, Animation, And Audio
Physics, animation, and audio disagree about an object's weight or energy, producing a dissonant sense of material. The trap is that each system is tuned in isolation. The false signal is that each looks or sounds correct alone. The harm is the unified sense of weight collapses, and the player feels the game is off without being able to identify why.

### Presenting Feel Preference As Readability Rule
Physics feel decisions are often judgment calls, but the agent should not present a personal feel preference as a readability requirement. State what is known (the feel target, the variance, the determinism needs), what is inferred (player perception), and what is a creative choice, so the team can decide with the tradeoffs visible.

## Self-Check

- [ ] Is the required feel defined and derived from the core loop before any parameter is tuned?
- [ ] Does the tuning favor readability over realism where they conflict, simplifying or constraining physics that obscures intent?
- [ ] Are momentum and weight tuned so objects carry the intended sense of mass without fighting the player's stopping or turning?
- [ ] Is emergent chaos bounded to match the genre, with extreme outcomes capped or dampened?
- [ ] Are physics interactions fair and consistent, with the same action producing reliably similar, skill-attributable results?
- [ ] Is physics deterministic where the game is competitive, speedrun-friendly, or replay-based, with fixed timesteps and consistent solver ordering?
- [ ] Do physics, animation, and audio co-author a unified sense of weight and material, without dissonance?
- [ ] Does the output distinguish physics decisions that serve readability and fairness from those that serve realism or feel preference?
- [ ] Are the feel target, parameter ranges, and determinism requirements specific enough for engineering to implement without re-deciding?
- [ ] Is uncertainty about player perception of feel named, with the tradeoffs that would change the recommendation made explicit?
