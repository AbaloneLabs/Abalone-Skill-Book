---
name: animation-and-feedback-collaboration.md
description: Use when the agent is collaborating with animators on gameplay feel, defining animation timing and windows for attacks and interactions, communicating game feel and responsiveness intent, negotiating anticipation versus responsiveness, reviewing animation against gameplay function, or resolving conflicts between animation quality and input responsiveness.
---

# Animation and Feedback Collaboration

Animation is where game feel lives, and collaborating with animators on it is a constant negotiation between two goods that often conflict: animation that looks believable because it obeys weight, anticipation, and follow-through, and gameplay that feels responsive because inputs produce immediate, readable results. The judgment problem is that both sides are right — beautiful animation wants lead-in and recovery, responsive gameplay wants instant reaction — and the compromise is not obvious because it lives in fractions of a second that nonetheless determine whether a game feels good or sluggish. Agents tend to miss the important issues because the conflict is invisible in a static review and emerges only in the hands, when a player presses a button and the character does not do what they expected when they expected it. The harm this prevents is combat that feels floaty or unresponsive despite gorgeous animation, players who cannot time their actions because the visual does not match the gameplay window, or animation that is "fixed" for responsiveness by being gutted of its craft, leaving a game that reacts instantly and feels weightless. Worse, animation-feedback mismatches discovered late force rework of both the animation and the gameplay tuning, because they are interlocked. The agent has freedom in the specific timing choices, but the disciplines of explicit window definition, functional review, and protecting both responsiveness and craft are mandatory. This skill covers the decisions that determine whether animation serves feel or fights it.

## Core Rules

### Define the Gameplay Window Separately From the Visual Animation

Every gameplay action — an attack, a dodge, an interaction — has a functional window (when it deals damage, grants i-frames, or triggers the effect) and a visual animation (what the player sees). These are not the same thing, and conflating them produces either animation that looks right but plays wrong, or gameplay that functions but looks disconnected. Define the functional window explicitly — startup, active, recovery — and then tune the animation to communicate that window clearly, so the player can read when they are committed, when they are effective, and when they can act again. The decision criterion is that the player must be able to learn the timing from the visuals, which requires the animation's key poses to align with the gameplay phases, even if the animation uses tricks (snap, smear, offset) to feel responsive while preserving readability.

### Protect Responsiveness Within the Animation Craft

Responsiveness is not the enemy of good animation; the enemy is animation that ignores the player's input timing. The craft is to preserve anticipation and weight while still making the action begin on input and communicate the active moment clearly. Techniques like input buffering, cancel windows, startup snaps, and recovery cancels let animation keep its believability while gameplay stays snappy. The decision rule is that no animation should make the player wait past the point of frustration before their input has a visible effect, and that the responsiveness budget — how long between press and perceivable reaction — is a design constraint the animator works within, not a casualty of the animation. Negotiate the budget per action class: a heavy attack may tolerate more startup than a dodge, but every class has a ceiling.

### Make Hit and Effect Moments Unambiguously Readable

The moment an attack connects, an interaction triggers, or a state changes must be visually unambiguous, because the player is making split-second decisions based on it. Obscure contact moments — buried in a flourish, hidden by particle effects, or lost in motion blur — make the game feel imprecise even when the logic is correct. Define key poses and frames that telegraph the active window, and ensure effects and camera support rather than obscure the contact. The decision criterion is that a player watching at speed can identify when the action was effective, because if they cannot, they will mistrust the timing and the game will feel unfair regardless of its actual fairness.

### Review Animation in Gameplay Context, Not in Isolation

An animation that looks beautiful in the animator's viewport can feel wrong in the game, because context changes everything: camera angle, distance, surrounding action, input timing, and the sequence of actions before and after. Review every gameplay animation in the actual game, with the actual camera and input, at the actual speed, and in combination with the animations it chains from and into. The decision rule is that an animation is not approved until it plays correctly in context, because isolated approval is the most common source of feel problems that surface only in playtesting. Build a fast iteration loop so animators can see their work in-game without a full build, because context review must be cheap to happen often.

### Co-Own the Feel Rather Than Delegating or Dictating It

Game feel is not the animator's job alone, nor the designer's; it is a shared artifact that emerges from animation, input handling, camera, effects, and audio together. Dictating exact keyframes to animators produces lifeless motion, while delegating feel entirely to animators produces beautiful work that may not serve the gameplay. Establish a shared vocabulary for feel — weight, snap, commitment, recovery — and tune it collaboratively, with the designer owning the functional requirements and the animator owning the craft that expresses them. The decision criterion is that both sides understand the goal and the constraints, because feel problems are almost always cross-discipline and cannot be solved by one side alone.

### Plan for Animation State Combinations and Transitions

Gameplay rarely plays one animation at a time; actions chain, interrupt, blend, and combine with locomotion, and the transitions between states are where feel breaks. A polished attack into a standstill can feel terrible when blended into a sprint or interrupted by a hit. Define the transition rules — what cancels what, how blends preserve momentum, how interruptions read — and test the combinations, not just the individual animations. The decision rule is that transitions are first-class animation work, because players spend more time moving between states than in any single state, and janky transitions undermine the polish of every individual animation around them.

## Common Traps

### Gorgeous Animation That Feels Sluggish

The animator delivers a beautiful attack with full anticipation and follow-through, and it looks great in review, but in play the input-to-effect delay feels unresponsive and players call the combat "clunky." The trap is that the animation obeyed craft principles without obeying the responsiveness budget. The false signal is the beauty of the motion; the harm is a game that looks good and feels bad. This trap causes rework when playtesting finally surfaces the feel problem. The defense is to define and enforce the responsiveness budget up front.

### Functional Window Mismatched to the Visual

The damage is dealt on frame 4, but the visual contact reads on frame 10, so players dodge based on what they see and get hit anyway, then call the game unfair. The trap is that the logic and the visual were tuned separately and never reconciled. The false signal is that both work correctly in their own domain; the harm is a game that is technically fair but perceptually unfair. This trap causes player distrust and balance complaints that no amount of tuning fixes, because the root is the visual-functional mismatch. The defense is to align key poses with functional windows explicitly.

### Responsiveness "Fixed" by Gutting the Animation

Faced with feel complaints, the team shortens or removes anticipation and recovery until the action snaps instantly, solving responsiveness but destroying weight and believability. The trap is that the fix addresses one problem by creating another. The false signal is that inputs now react immediately; the harm is a game that feels twitchy and weightless. This trap causes a swing between sluggish and floaty without ever landing on good. The defense is responsiveness techniques — buffering, cancels, snaps — rather than amputation of the animation craft.

### Isolated Approval That Hides Context Problems

The animation is approved in the viewport, signed off, and integrated, only to discover in playtest that the camera angle hides the key pose, the distance makes the action unreadable, or the blend from locomotion kills the impact. The trap is that approval happened out of context. The false signal is the clean viewport review; the harm is late rework. This trap causes avoidable iteration cycles and erodes trust in the review process. The defense is mandatory in-context review with a fast iteration loop.

### Unplanned Transitions That Break the Chain

Individual animations are polished, but the blends between them — attack into dodge, hit reaction into recovery, sprint into interaction — are janky, and the game feels rough despite each piece being fine. The trap is that transitions were treated as automatic rather than authored. The false signal is the quality of the individual clips; the harm is a game that feels unpolished in exactly the moments players experience most. This trap causes a pervasive "almost there" feel that is hard to pinpoint. The defense is to plan and test transitions as first-class work.

### Effect and Camera Obscuring the Contact

Particles, screen shake, and camera moves are added to enhance impact, but they bury the exact frame the player needs to read, making the action feel muddy. The trap is that the enhancements were added without checking whether they obscured the functional moment. The false signal is the increased spectacle; the harm is reduced readability at the critical instant. This trap causes actions that feel powerful but imprecise. The defense is to review effects and camera against the contact moment and ensure they support, not hide, it.

## Self-Check

- Is every gameplay action defined by an explicit functional window (startup, active, recovery) separate from the visual animation, with key poses aligned to communicate the phases?
- Have I set and enforced a responsiveness budget per action class, using techniques like buffering, cancels, and snaps to preserve both responsiveness and animation craft?
- Are hit, effect, and state-change moments visually unambiguous at speed, with key poses and effects supporting rather than obscuring the contact?
- Was every gameplay animation reviewed in actual game context — real camera, input, speed, and chaining — rather than approved in isolation in a viewport?
- Is game feel co-owned across animation, input, camera, effects, and audio, with shared vocabulary and collaborative tuning rather than dictation or delegation?
- Are transitions and state combinations planned and tested as first-class work, including cancels, blends, interruptions, and hit reactions, not just individual clips?
- Have I confirmed that effects, particles, and camera moves enhance rather than bury the readable contact moment, and avoided fixing feel complaints by gutting animation craft?
