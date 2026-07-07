---
name: pcg-authoring-goals-and-authorial-control.md
description: Use when the agent is defining what procedural generation should author in a game, deciding the balance between authored and generated content, setting the generation's goals and constraints, or reviewing whether procedural content serves design intent versus filling space; trigger contexts include procedural generation, PCG, procedural content, authored versus generated, generative systems, algorithmic content, random generation, seed design, generation goals, authorial control, procedural world, procedural levels; important risks include directionless generation, content that fills space without purpose, loss of authorial intent, and procedural systems that contradict the game's design.
---

# PCG Authoring Goals And Authorial Control

Procedural generation is not a way to make content for free; it is a way to trade direct authorship for parameterized authorship, and the agent is usually asked to define what a generative system should produce while the design intent, the constraints, and the boundary between authored and generated content are under-specified. The judgment problem is that procedural systems can produce infinite variation, but variation without intent is noise, and a generator that fills space without serving the game's design produces a world that feels generic, incoherent, or hostile to the experience the team intended.

The agent tends to miss this because procedural generation is often adopted as a solution ("we'll procedural-generate the levels") before the authoring goals are defined, and because the generator's parameters are treated as technical tuning rather than design authorship. The harm is games where the procedural content is technically functional but emotionally and structurally hollow, where the generator contradicts the hand-authored design, or where the team discovers too late that the generator cannot produce the specific experiences the game needs. Use this skill to slow the decision down enough to expose what the generator is actually for, then make the recommendation appropriately conservative when authorial intent, coherence, and player experience are at stake.

## Core Rules

### Define What The Generator Authors Before How It Generates
Before choosing an algorithm, state what the generator is authoring and why: levels, layouts, encounters, items, narratives, textures, or whole worlds, and what design purpose each serves. The agent should map the authoring goal to a player-facing experience (exploration, replayability, surprise, scale) and reject generation that serves no experience beyond "more content." A generator without a defined authoring goal produces content; a generator with one produces designed experiences.

### Decide The Authored-Generated Boundary Explicitly
Every procedural game mixes authored and generated content, and the boundary must be decided deliberately. The agent should specify what is always authored (key narrative beats, signature encounters, critical-path spaces, art landmarks), what is always generated (filler encounters, incidental items, optional areas), and what is hybrid (authored modules assembled by the generator). A blurry boundary produces incoherence; a deliberate boundary produces a game that feels both hand-crafted and expansive.

### Set Generation Goals As Design Constraints, Not Algorithm Outputs
The generator's goals should be expressed as design constraints the player experiences, not as algorithm parameters. The agent should define goals such as "every generated level must have a recognizable landmark," "encounters must escalate over a run," "no generated item should trivialize authored encounters," and translate these into generator constraints. Goals stated only as algorithm outputs (density, probability tables) lose the connection to player experience and drift toward noise.

### Preserve Authorial Intent Through Constraints And Curation
Procedural generation does not remove authorship; it relocates it to the constraints, the asset pool, and the curation. The agent should author the constraints and the asset library as carefully as hand-placed content, curate the generator's output (reject or re-roll outputs that violate intent), and treat the generator as an instrument of authorship rather than a replacement for it. The author who designs the constraints authors the content; the author who lets the generator run free has abdicated.

### Ensure Coherence Across Generated And Authored Content
Generated content must cohere with authored content in tone, difficulty, economy, and fiction. The agent should ensure the generator cannot produce content that contradicts the game's tone (a slapstick item in a grim world), difficulty curve (a trivial encounter on the critical path), economy (a generated item that breaks the crafted economy), or fiction (a generated structure that makes no sense in the lore). Incoherence between generated and authored content is the most common failure of mixed games.

### Match Generation Method To Content Type
Different content types demand different generation methods: grammar-based for structures, noise-based for terrain, constraint-based for solvable levels, simulation-based for ecosystems, and Markov or LLM-assisted for text. The agent should choose the method that serves the content's requirements (a level that must be solvable needs constraint-satisfaction, not raw noise), and avoid using a fashionable method for content it cannot serve. Method chosen for novelty rather than fit produces broken or incoherent content.

### Plan For Evaluation, Curation, And Iteration
A generator is not done when it runs; it is done when its output is evaluated, curated, and iterated against design goals. The agent should establish how generated content will be evaluated (automated metrics, playtest sampling, curation passes), what rejection criteria will re-roll or discard bad output, and how the generator will be tuned over time based on what players actually experience. A generator shipped without evaluation and curation ships the full distribution of its failures.

## Common Traps

### Generation As A Solution Before Goals Are Defined
The team adopts procedural generation to solve a content problem before defining what the content must achieve, and the generator produces variation without purpose. The trap is that generation feels like free content. The false signal is that the system "works." The harm is a game full of content that fills space without serving any experience, and a generator whose output no one curated against intent.

### Directionless Variation Mistaken For Replayability
The generator produces endless variation, and the team equates variation with replayability, but the variation does not create meaningfully different experiences. The trap is that more variation feels like more game. The false signal is that no two runs are alike. The harm is players perceive the variation as superficial, churn after a few runs, and the replayability the generation was meant to provide never materializes.

### The Blurry Authored-Generated Boundary
The boundary between authored and generated content is never decided, and the game lurches between hand-crafted quality and generic filler with no coherent voice. The trap is that mixing is flexible. The false signal is that the game has both authored and generated content. The harm is tonal and structural incoherence, where the player cannot tell what matters and the experience feels stitched from unrelated parts.

### Generator Output Contradicting Authored Design
The generator produces content that breaks the economy, trivializes authored encounters, or contradicts the fiction, because its constraints were not aligned with the authored design. The trap is that the generator runs within its own parameters. The false signal is that the generation is correct. The harm is the authored game is undermined by its own procedural systems, and balance and coherence collapse where generated and authored content meet.

### Method Chosen For Novelty Rather Than Fit
A fashionable generation method is chosen for content it cannot serve (noise for levels that must be solvable, pure randomness for encounters that must escalate), and the output is broken. The trap is that the method is interesting. The false signal is that the method is powerful. The harm is content that fails its fundamental requirements, discovered late when the generator cannot be easily replaced.

### No Evaluation Or Curation Of Generator Output
The generator is shipped without evaluation, curation, or iteration, and its full distribution of failures reaches players. The trap is that the generator passed its unit tests. The false signal is that sample outputs looked good. The harm is players encounter the generator's worst outputs (unsolvable levels, broken economies, incoherent structures) and the game's quality is defined by its procedural tail, not its average.

### Presenting Generator Tuning As Design Authorship
Generator decisions are often judgment calls, but the agent should not present algorithm tuning as if it were neutral engineering. State what is known (the authoring goal, the constraints, the coherence requirements), what is inferred (player experience of the output), and what is a creative choice, so the team retains authorship rather than surrendering it to the algorithm.

## Self-Check

- [ ] Is the generator's authoring goal defined as a player-facing experience before any algorithm is chosen?
- [ ] Is the boundary between always-authored, always-generated, and hybrid content decided explicitly?
- [ ] Are generation goals expressed as design constraints the player experiences, translated into generator parameters?
- [ ] Does the design preserve authorial intent through carefully authored constraints, asset pools, and output curation?
- [ ] Does generated content cohere with authored content in tone, difficulty, economy, and fiction, with constraints preventing contradiction?
- [ ] Is the generation method chosen for fit to the content type's requirements rather than for novelty?
- [ ] Are evaluation, curation, rejection criteria, and iteration established for generator output before shipping?
- [ ] Does the output distinguish generation that serves design intent from generation that fills space?
- [ ] Are the authoring goals, constraints, and boundary specific enough for the engineering and design teams to implement without re-deciding?
- [ ] Is uncertainty about player experience of generated content named, with the tradeoffs that would change the recommendation made explicit?
