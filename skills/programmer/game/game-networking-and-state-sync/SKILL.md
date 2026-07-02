---
name: game_networking_and_state_sync.md
description: Use when the agent is building multiplayer netcode, synchronizing game state across clients and a server, implementing client-side prediction and server reconciliation, lag compensation, rollback netcode, or lockstep simulation; choosing between state synchronization and event/delta synchronization; deciding authoritative server vs peer-to-peer models; handling packet loss, jitter, latency, and bandwidth budgets; designing anti-cheat for an authoritative server; or diagnosing rubber-banding, teleporting, desync, hit-registration disputes, or "I shot him first" complaints. Covers the tradeoffs of input-delay vs rollback, bandwidth vs accuracy, client-trust vs server-authority, and determinism requirements for lockstep.
---

# Game Networking And State Sync

Game networking is the problem of making players believe they share one world when in fact each holds a slightly different, slightly stale copy of it. Every multiplayer architecture is a set of choices about who is trusted (an authoritative server, or peer clients), what is sent (full state snapshots, or input deltas), how disagreement is resolved (server wins, rollback and re-simulate, or vote), and how much latency the player is asked to absorb as input delay versus how much they are asked to tolerate as prediction error. These choices are not implementation details to defer; they define the feel of the game, the cheat surface, and the bandwidth bill, and they are extremely hard to change after launch.

Agents tend to underestimate networking because the single-player mental model works on a LAN with two players. The failures appear only under real internet conditions — 150 ms ping, 5% packet loss, jitter spikes — and they are severe: rubber-banding as the server corrects divergent predictions, hit-registration disputes because what the attacker saw was not what the server computed, rollbacks that flicker the screen, and cheaters who exploit a client-trusted position. The judgment problem is deciding, for each category of game state, what authority model, sync strategy, and latency-hiding technique fits, and then implementing the reconciliation paths that make divergence survivable rather than game-breaking.

## Core Rules

### Establish Authority Before Choosing A Sync Strategy

The first decision is who is the source of truth, and it determines everything downstream.

- **Server-authoritative (dedicated server).** The server simulates the true state; clients send inputs and render predictions. Cheating-resistant (clients cannot dictate state), but every client action incurs a round trip unless hidden by prediction. Standard for competitive games.
- **Client-authoritative / listen-server.** One client hosts and simulates; others connect. Lower infrastructure cost, but the host can cheat and host-migration is a hard problem. Acceptable for cooperative games, dangerous for competitive ones.
- **Lockstep / peer-to-peer.** All peers run deterministic simulation from the same inputs; only inputs are transmitted. Extremely low bandwidth, but requires bit-exact determinism (see the game-loop skill) and amplifies one slow peer's latency to everyone.

Strong choice: a competitive shooter uses a server-authoritative model with client prediction because anti-cheat and fair hit registration are non-negotiable. Weak choice: trusting client-sent positions in a competitive game "to reduce latency," then shipping an aimbot epidemic. Name the trust boundary explicitly per state category.

### Match The Sync Strategy To The State Category

Different state has different tolerance for staleness, bandwidth, and divergence, and applying one strategy everywhere is either too expensive or too laggy.

- **Fast-changing, player-controlled state (positions, velocities).** Use client-side prediction with server reconciliation: the client predicts its own movement locally and renders immediately, the server authoritatively simulates and sends corrections, and the client re-simulates from the corrected state when prediction diverged. This hides latency at the cost of occasional rubber-banding when the client was wrong.
- **Slow-changing, global state (scores, match phase, inventory).** Use state synchronization: the server sends authoritative snapshots, possibly delta-compressed. Latency is acceptable because the state changes rarely.
- **Discrete events (shots fired, abilities cast, hits landed).** Use event synchronization with reliable-or-unreliable delivery chosen per event. A missed "you took damage" event is worse than a missed "footstep" event; tag events by criticality.

Write down, per state category: who is authoritative, how often it syncs, what happens on packet loss, and how divergence is reconciled.

### Implement Client Prediction And Server Reconciliation Together

Prediction without reconciliation is a lie: the client renders a future the server has not confirmed, and when they disagree, the player sees a jarring snap. The correct pattern is paired. The client predicts, tags each predicted input with a sequence number, and continues simulating. The server processes inputs in order and sends back its authoritative state *with the last-processed input sequence number*. The client compares: if the server's state for that sequence matches its own prediction, nothing to do; if it differs, the client rewinds to the server's authoritative state and re-applies all inputs after that sequence (reconciliation). This keeps the client ahead of the server in feel while converging on the truth. The common bug is predicting but not reconciling, so errors accumulate until a full resnap.

### Use Lag Compensation For Hit Registration In Shooters

When a player with 100 ms ping fires at a moving target, the position they see is 100 ms old from the server's perspective. Authoritatively evaluating the shot against the server's *current* state produces "I shot him but he wasn't there" complaints. Lag compensation rewinds the server's world state to what the attacker saw at the time they fired (within a bounded history window) and evaluates the hit there. This trades fairness for the high-ping attacker (they can hit a target that, from the target's view, had already moved) against fairness for the low-ping target (they get hit around a corner). It is a deliberate tradeoff; document the rewind limit and the "shot around the corner" artifact it permits. Without lag compensation, high-ping players cannot hit anything; with unbounded lag compensation, players are hit seconds after taking cover.

### Choose Rollback Or Delay-Based Netcode For Fighting Games Deliberately

Fighting games and other input-tight genres face a binary choice.

- **Delay-based.** Wait to collect inputs from both peers for a few frames before simulating, smoothing the experience at the cost of visible input lag. Simple, but a lag spike stalls both players.
- **Rollback (GGPO-style).** Simulate immediately on local input, predicting the remote player's inputs; when the real remote inputs arrive, if they differ from the prediction, roll back the simulation to the divergence point and re-simulate forward. No added input lag, but a wrong prediction causes a visible rollback flicker.

Rollback is generally superior for player feel but demands a simulation that can be efficiently saved and rewound (state snapshots every frame, deterministic re-simulation). It is not a drop-in; it shapes the simulation architecture. Choose based on whether the genre tolerates input lag (delay) or rollback flicker (rollback), and budget the rollback window to bound re-simulation cost.

### Budget Bandwidth Against Accuracy And Scale

Bandwidth is a hard constraint, especially for large player counts or mobile networks. The levers, and their costs:

- **Send rate.** How often the server snapshots state. Higher is smoother and more accurate; lower saves bandwidth. 20 Hz is common for large games, 60 Hz for tight competitive ones.
- **Delta compression.** Send only what changed since the last acknowledged snapshot, not full state. Essential at scale.
- **Relevance / interest management.** Only sync entities near a given player; a player on one side of the map does not need state for the other side. Cuts bandwidth quadratically with view distance.
- **Quantization.** Compress positions, angles, and velocities to the precision the game needs. A 32-bit float for an angle is usually wasted; 16 bits or a quaternion quantization suffices.

Decide the budget per player and per entity category, and verify the worst case (all players clustered, all firing) stays under it. A game that works in a 4-player test and collapses at 64 players has no bandwidth budget.

### Treat The Client As Hostile In Competitive Games

Anything the client sends that affects authoritative state is a cheat vector: client-reported positions, client-reported hits, client-reported random outcomes. In a server-authoritative model, the client sends *inputs* (I moved, I fired in this direction) and the server computes the outcome. Validate inputs for plausibility (speed caps, fire-rate caps, line-of-sight checks) to catch speed hacks and trigger bots. Never trust client-reported damage or hit confirmation. The cost of server authority is latency and compute; the cost of client trust is a cheatable game. For cooperative games with trusted peers, client trust may be acceptable — but make that decision explicitly, not by omission.

## Common Traps

### Trusting Client-Sent Positions In A Competitive Game

Accepting "I am at position X" from the client to avoid server simulation cost, then watching speed hacks and teleport cheats proliferate. Authority over movement state must live on the server; the client predicts, it does not dictate.

### Predicting Without Reconciling

Rendering client predictions but never re-simulating from server corrections, so prediction error accumulates until a full resnap that the player perceives as a teleport. Prediction and reconciliation are a pair; ship both or neither.

### Unbounded Lag Compensation

Rewinding the server arbitrarily far to validate a high-ping attacker's shot, so players are hit long after taking cover behind a wall. Bound the rewind window to the latency the game is willing to tolerate, and accept that very-high-ping players will miss.

### Treating Lockstep As "Just Send Inputs"

Adopting lockstep for its low bandwidth without committing to bit-exact determinism, then shipping desyncs where peers diverge after a minute of play. Lockstep requires determinism as a hard prerequisite; without it, the model is broken by construction.

### One Sync Rate For Everything

Sending all state at 60 Hz, including scores and inventory that change once per match, wasting bandwidth; or sending everything at 5 Hz, making player movement choppy. Match send rate and strategy to the state category.

### Ignoring Interest Management At Scale

Syncing every entity to every player, then hitting a bandwidth wall at high player counts. Relevance filtering (only nearby entities) is not an optimization; at scale it is a requirement.

### Sending Full State Every Snapshot

Transmitting complete entity state every tick instead of deltas, multiplying bandwidth by entity count. Delta compression against the last-acknowledged snapshot is the standard fix and should be present from the start, not retrofitted.

### Assuming Reliable Delivery For Everything

Sending all events reliably, so a dropped "footstep" packet stalls the reliable stream and delays the critical "took damage" event behind it. Tag events by criticality; use unreliable for ephemeral state and reliable-ordered only for events whose loss breaks the game.

## Self-Check

- [ ] The authority model (server-authoritative, listen-server, or lockstep P2P) is chosen explicitly per the game's anti-cheat and fairness requirements, and the trust boundary is named for each state category.
- [ ] Each state category (fast player state, slow global state, discrete events) has a documented sync strategy: authority, send rate, loss handling, and reconciliation path.
- [ ] Client prediction is paired with server reconciliation — the client re-simulates from the server's last-acknowledged sequence on divergence, rather than accumulating prediction error.
- [ ] Lag compensation, if used, has a bounded rewind window, and the "shot around the corner" tradeoff is documented and accepted.
- [ ] For fighting/input-tight games, delay-based vs rollback netcode was chosen deliberately, and rollback implementation has efficient state snapshots and bounded re-simulation.
- [ ] A per-player and per-entity bandwidth budget exists; delta compression, interest management, and quantization are in place; and the worst-case load (max players clustered, all active) stays under budget.
- [ ] In competitive games, the client sends inputs only; positions, hits, and outcomes are computed and validated server-side, with plausibility checks for speed and fire-rate hacks.
- [ ] Lockstep, if used, is backed by a proven-deterministic simulation (fixed timestep, ordered iteration, constrained math, seeded RNG) with a cross-peer desync test.
- [ ] A test under realistic network conditions (100+ ms ping, packet loss, jitter) was run, and the game neither rubber-banded persistently, desynced, nor exhibited exploitable client trust.
