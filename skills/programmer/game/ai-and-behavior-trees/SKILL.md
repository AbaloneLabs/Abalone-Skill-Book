---
name: ai_and_behavior_trees.md
description: Use when the agent is building game AI, NPC decision-making, enemy behavior, or agent coordination; choosing between behavior trees, utility AI, finite state machines, GOAP, or hierarchical task networks (HTN); implementing navigation meshes, pathfinding (A*, Dijkstra, JPS), or steering behaviors; setting AI tick budgets or update scheduling; tuning difficulty, rubberbanding, or AI fairness; handling partial observability and imperfect information for AI; coordinating squads or flocks of agents without quadratic cost; or diagnosing AI that is trivially exploitable, impossibly perfect, frozen, jittering between states, or consuming excessive CPU. Covers the tradeoffs of reactive vs planning architectures, perceptual realism, and scalable multi-agent coordination.
---

# AI And Behavior Trees

Game AI is not "an algorithm that plays the game well." It is a system that produces believable, tunable, performant behavior under hard constraints — a per-frame CPU budget measured in single-digit milliseconds, imperfect information about the world, dozens to hundreds of simultaneous agents, and a design goal of *fun* rather than *optimality*. The same NPC that is trivially exploitable (stands still while shot) is also, with a parameter flip, impossibly perfect (tracks the player through walls and headshots instantly), and both are failures. Agents tend to reach for one architecture (usually a behavior tree or a state machine), wire it to perfect world knowledge, and tune difficulty by feel — then discover the AI is either boring, unfair, or melts the CPU when fifty enemies are on screen.

The judgment problem is choosing a decision architecture that fits the kind of behavior the game needs (reactive reflexes vs planned sequences vs strategic goals), giving the AI *deliberately imperfect* perception so it behaves believably rather than omnisciently, bounding the per-agent and per-frame cost so it scales, and tuning difficulty as a designed curve rather than a difficulty-multiplier slider. The harm of getting this wrong is experiential and silent: the game runs, but the enemies feel dumb, feel cheap, or drop the frame rate the moment a big fight starts.

## Core Rules

### Match The Decision Architecture To The Kind Of Behavior Needed

There are several architectures, and they solve different problems. Picking one by familiarity rather than fit produces brittle AI.

- **Finite state machines (FSM).** Good for entities with a small number of distinct, mutually exclusive modes (idle/patrol/chase/attack/flee). Simple to author and debug, but scale poorly — a complex NPC's state graph becomes a tangled web of transitions, and adding a behavior means rewiring many states.
- **Behavior trees.** Good for modular, hierarchical reactive behavior: a tree of selectors (try children until one succeeds) and sequences (run children in order) with condition and action nodes. Composable, reusable, and the industry default for combat and NPC behavior. The cost: they are *reactive* — they re-evaluate from the root each tick, which is great for responsiveness but means they do not inherently commit to multi-step plans.
- **Utility AI.** Scores candidate actions by weighted considerations (health, distance, ammo) and picks the highest. Excellent for emergent, context-sensitive behavior and smooth priority shifts. The cost: tuning is opaque (a web of weights and curves), and behavior can be hard to predict or debug.
- **Goal-oriented action planning (GOAP) / hierarchical task networks (HTN).** The AI reasons about a goal and assembles a sequence of actions to reach it. Powerful for strategic, multi-step behavior (flank, set up an ambush). The cost: planning is expensive (search over action space), harder to author, and harder to tune for consistent feel.

Strong choice: a cover shooter's grunts use behavior trees for reactive combat with utility-scored target selection. Weak choice: using a flat FSM for a complex NPC and ending up with a 40-state transition graph, or using GOAP for enemies that only need reflex reactions and paying the planning cost every tick. Name the behavior class — reflexive, hierarchical, or strategic — before choosing.

### Give The AI Deliberately Imperfect Perception

Omniscient AI feels unfair and is unplayable. Real game AI does not query the world for the player's exact position; it runs a perception model that models what the NPC *could* know. This means: line-of-sight checks (with a vision cone and range), hearing (events emit audio signals that decay with distance), memory (the NPC remembers the last known position, not the current one, once the player breaks line of sight), and reaction delay (the NPC does not react on the frame it first sees the player — there is a detection-to-response latency). The last-known-position pattern is essential: an NPC that loses sight of the player should search the last known location, not beeline to the player's true position. Perception is where "believable" is won or lost; an AI with perfect knowledge is either a turret (if it acts on it) or obviously faking (if it ignores it).

### Budget AI Cost Per Agent And Per Frame

AI is a per-frame cost that scales with agent count, and unbounded it will drop the frame rate in the exact moments (big battles) where performance matters most. The levers:

- **Tick scheduling.** Not every agent needs to decide every frame. Stagger decisions: agent A thinks on frame 1, agent B on frame 2, etc., with the decision held until the next think. "Think every N frames" or "think every N ms" cuts decision cost by N with no visible loss for all but the most twitch-reactive enemies.
- **Work sharing.** Expensive queries (pathfinding, perception sweeps) are amortized: pathfinding is queued and solved across frames (never A* on the main thread for 50 agents at once), and perception can be tiered (cheap distance check first, expensive LOS check only for candidates that pass).
- **LOD for AI.** Distant or off-screen agents run cheaper logic — simpler perception, slower think rate, prebaked patrol instead of live pathfinding. The player cannot tell that a guard 200 m away is using a simplified brain.

Decide the per-agent and total AI budget up front (e.g., "AI gets 3 ms/frame, 0.1 ms/agent"), and profile against the worst case (max simultaneous agents). An AI system that runs fine on 5 enemies and tanks at 50 has no budget.

### Tune Difficulty As A Designed Curve, Not A Multiplier

Difficulty is not "enemy health × 1.5." It is a designed experience: what the player is expected to feel at each stage, and what levers create that. Good difficulty levers include perception acuity (reaction time, vision range, accuracy), aggression (how often the AI pushes vs holds position), coordination (whether enemies flank or attack one at a time), and resource pressure (enemy health/ammo as a coarse dial). Rubberbanding — dynamically adjusting AI performance to keep the race/fight close — is a legitimate tool but a dangerous one: too obvious and it feels like the game is cheating, too subtle and it does nothing. Tune difficulty by playing and measuring, and keep the levers data-driven (designer-editable curves, not hardcoded constants) so the curve can be shaped without engineering. The failure is difficulty that is either flat (every enemy identical, no arc) or a raw stat multiplier that makes late-game enemies into bullet sponges without making them smarter.

### Coordinate Multiple Agents Without Quadratic Cost

Squad behavior, flanking, and flocking are what make combat feel designed rather than chaotic. The trap is implementing coordination as all-to-all communication: every agent checks every other agent, costing O(N²) and collapsing at scale. The scalable patterns are: a shared blackboard or squad manager that agents read from and write to (O(N) per frame, one source of truth), spatial partitioning for neighbor queries (grid, quadtree) so an agent finds nearby allies without scanning all, and role assignment (one agent is the spotter, others suppress) that emerges from a few shared rules rather than explicit pairwise negotiation. Flocking (boids) is the canonical example: three local rules (separation, alignment, cohesion) evaluated against spatially-local neighbors produce flock behavior at near-linear cost. Coordinate via shared state and local rules, not pairwise communication.

## Common Traps

### Omniscient AI That Tracks The Player Through Walls

Querying the player's true position directly, so enemies beeline to the player regardless of line of sight, producing "they knew exactly where I was" complaints. Perception must model what the NPC can actually perceive; last-known-position and reaction delay are mandatory for believability.

### Behavior Tree That Never Commits

A reactive tree that re-evaluates from the root every tick, so a condition flip mid-action aborts a half-finished plan and the NPC jitters between behaviors (starts to flank, sees player, abandons flank, starts to shoot, loses sight, starts to search). Use running nodes and commit/hysteresis where a behavior, once started, should complete or fail cleanly rather than be preempted by a marginally-higher-priority sibling.

### A* On The Main Thread For Every Agent

Calling synchronous pathfinding per agent per frame, so 50 enemies finding paths simultaneously drops the frame to 15 FPS. Pathfinding is expensive and must be queued, time-sliced across frames, and cached (re-use a path until the target moves significantly or a timeout elapses).

### Difficulty As A Stat Multiplier

Doubling enemy health and damage for "hard mode," producing bullet sponges that are no smarter, just slower to kill. Real difficulty tunes perception, aggression, and coordination; stat scaling is the coarsest and least satisfying lever and should not be the only one.

### N² Coordination

Each agent scanning all others to find squadmates or avoid collisions, costing O(N²) and failing past a few dozen agents. Use spatial partitioning and shared blackboards; coordination should be near-linear.

## Self-Check

- [ ] The decision architecture (FSM, behavior tree, utility, GOAP/HTN) was chosen to match the behavior class needed — reactive, hierarchical, or strategic — not by familiarity, and complex NPCs are not trapped in a tangled flat FSM.
- [ ] The AI uses a perception model with vision/hearing, line-of-sight checks, last-known-position memory, and reaction delay, rather than querying the player's true position directly.
- [ ] A per-agent and per-frame AI budget exists; decisions are tick-staggered, pathfinding is queued/time-sliced and cached, and AI LOD reduces cost for distant or off-screen agents.
- [ ] Difficulty is tuned via designed levers (perception, aggression, coordination, accuracy) as data-driven curves, not solely a health/damage multiplier, and rubberbanding, if present, is subtle and bounded.
- [ ] Multi-agent coordination uses shared blackboards and spatially-local neighbor queries (near-linear), not all-to-all pairwise communication (quadratic).
- [ ] Reactive architectures (behavior trees) commit to started behaviors or fail them cleanly, rather than jittering between actions on every root re-evaluation.
- [ ] A worst-case load test (maximum simultaneous agents in a big fight) was profiled, and AI stayed within its frame budget without dropping the frame rate.
- [ ] Playtesting confirmed the AI feels neither trivially exploitable (frozen, dumb) nor impossibly perfect (omniscient, instant, unfair) across the difficulty curve.
