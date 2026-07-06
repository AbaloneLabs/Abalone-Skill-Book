---
name: camera-system-and-framing-design.md
description: Use when the agent is designing a game's camera system, choosing between camera perspectives, tuning camera behavior and responsiveness, framing the player's view, or evaluating whether the camera supports the gameplay or obscures action, causes motion sickness, and fights the player's intent.
---

# Camera System and Framing Design

The camera is the player's only window into the game, and every gameplay action is mediated by what the camera shows and how it moves. The judgment problem is that the camera must simultaneously frame the action, communicate spatial relationships, avoid motion sickness, and not fight the player's intent — and these goals conflict. A camera that frames cinematically may obscure threats; a camera that shows everything may feel detached; a responsive camera may induce nausea. Agents tend to miss this because the camera is treated as a technical setup rather than a design system, and because camera problems are often misattributed to controls or difficulty. The harm is a game where players cannot see what they need to see, where the camera fights them during critical moments, or where motion sickness excludes a segment of the audience. This skill covers how to choose and tune a camera for the gameplay, manage framing tradeoffs, and avoid the camera-fights-player trap. The agent has latitude in the perspective choice, but the obligation to make the camera serve the gameplay and the player is not optional.

## Core Rules

### Choose Perspective Based on What the Player Must Perceive and Express

Each perspective — first-person, third-person, top-down, isometric — enables different perceptions and expressions: first-person maximizes immersion and aiming precision but limits spatial awareness; third-person balances awareness and character expression; top-down maximizes tactical perception but reduces immersion. The decision rule: choose the perspective based on what the core loop requires the player to perceive (threats, spacing, environment) and express (precision, character, tactics), and accept the tradeoffs explicitly. Choosing perspective for trend or novelty, rather than for the gameplay's perceptual needs, produces a camera that fights the game.

### Tune Camera Responsiveness to Match Input Intent

The camera must respond to player input at a rate that matches intent: too slow and the camera lags behind the player's turn, feeling sluggish; too fast and the camera whips, causing disorientation and nausea. The decision rule: tune camera acceleration, max speed, and damping so the camera follows input faithfully without overshoot or lag, and playtest for both responsiveness and comfort. A camera that does not match input intent is perceived as fighting the player, regardless of how technically correct its behavior is.

### Frame to Show What the Gameplay Requires, Not What Looks Cinematic

Camera framing must prioritize the gameplay-relevant information — the threats, the platforms, the objective, the path — over cinematic composition. The decision rule: for each gameplay situation, identify what the player must see to act, and frame the camera to show it, even when a more cinematic angle would look better. Cinematic framing that hides a threat or a ledge produces unfair failures, because the player was acting on incomplete information the camera chose not to show.

### Manage Camera Collision and Obstruction Proactively

The camera will collide with walls, terrain, and objects, and unmanaged collision produces a camera that snaps inside geometry, obscures the player, or jumps to disorienting angles. The decision rule: implement robust camera collision handling (pull-in, transparency, repositioning) and playtest extensively in cluttered environments, because collision failures cluster in exactly the tight spaces where gameplay is most intense. A camera that breaks in corners and corridors undermines the gameplay precisely where the player needs it most.

### Provide Player Camera Control Within Bounded Autonomy

The player should have meaningful control over the camera — to look around, to reframe, to check surroundings — but the camera must also maintain the framing the gameplay requires, so the balance is bounded autonomy. The decision rule: give the player camera control that lets them gather information and reframe without allowing them to put the camera in states that break the gameplay (looking through walls, breaking the intended perspective). Total autonomy produces exploitation and bugs; no autonomy produces frustration; bounded autonomy is the design.

### Validate Camera Comfort Across the Motion-Sickness Spectrum

Camera behavior is the primary cause of motion sickness in games, and a camera that is comfortable for the team may sicken a significant portion of players. The decision rule: test camera behavior with motion-sickness-sensitive players, offer comfort options (field of view, camera shake, motion smoothing), and treat motion sickness as an accessibility defect, not a player weakness. A camera that excludes even ten percent of the audience through sickness has failed, regardless of how it performs for the rest.

## Common Traps

### Cinematic Framing That Hides Gameplay-Relevant Information

The team tunes the camera for cinematic composition — dramatic angles, tight shots, low perspectives — and the framing hides the threats, ledges, or objectives the player needs to act on, producing unfair failures. The trap is that cinematic framing looks great in trailers. The false signal is that the game looks filmic. The harm is that the player is hit by threats they could not see, misses jumps they could not judge, and fails for reasons attributable to the camera rather than their skill, and the cinematic camera that was meant to enhance the experience instead undermines it, because framing was prioritized over perception.

### The Sluggish or Whip Camera That Fights Input

The camera responds to player input too slowly (sluggish, lagging behind turns) or too fast (whipping, overshooting), and the player perceives the camera as fighting their intent on every turn. The trap is that extreme responsiveness settings feel decisive in isolation. The false signal is that the camera is responsive on a spec sheet. The harm is that the sluggish camera makes the game feel heavy and unreactive, the whip camera causes disorientation and nausea, and in both cases the player's engagement is consumed by wrestling the camera rather than playing the game, because the camera's response curve was never matched to human input intent.

### Camera Collision Failures in Tight Spaces

The camera collides with geometry in corridors, corners, and cluttered rooms, snapping inside walls or jumping to broken angles, precisely in the tight spaces where combat and navigation are most demanding. The trap is that collision failures only appear in clutter, which demos avoid. The false signal is that the camera works in open spaces. The harm is that the camera breaks in the exact conditions where the player needs it most, the tight encounters become exercises in fighting the camera, and the gameplay that was tuned in open arenas falls apart in the corridors where much of the real game occurs, because collision handling was not robustly implemented and tested.

### No Player Camera Control Where the Gameplay Demands It

The camera is fully automated and the player cannot look around, reframe, or check their surroundings, in a game where the gameplay requires spatial awareness the auto-camera does not provide. The trap is that full automation guarantees consistent framing. The false signal is that the camera always shows the right thing. The harm is that the player cannot gather the information the gameplay requires, cannot check for threats the auto-camera is not showing, and experiences the camera as a constraint on their competence rather than a tool, because the autonomy that would let them perceive was withheld in a game that needed it.

### Ignoring Motion Sickness Until Launch

The team tunes the camera for the developers, who are adapted to it and do not experience sickness, and ships without comfort options or sensitivity testing, discovering at launch that a significant portion of players become nauseated. The trap is that the team does not experience the sickness. The false signal is that internal playtests report no comfort issues. The harm is that motion-sickness-sensitive players churn in the first session, often without articulating why, the reviews mention nausea, and the audience that would have engaged is excluded by a camera that was never tested against them, because motion sickness was treated as a player problem rather than a design defect.

## Self-Check

- Did I choose the perspective based on what the core loop requires the player to perceive and express, accepting the tradeoffs explicitly?
- Is camera responsiveness tuned so it follows input faithfully without sluggish lag or disorienting whip, validated for both response and comfort?
- Does the framing prioritize gameplay-relevant information (threats, ledges, objectives) over cinematic composition?
- Is camera collision handling robust, tested extensively in cluttered and tight spaces where failures cluster?
- Does the player have bounded camera control that allows information-gathering and reframing without enabling camera states that break the gameplay?
- Have I tested camera comfort with motion-sickness-sensitive players and provided comfort options (FOV, shake, smoothing)?
- Did I avoid cinematic framing that hides the information the player needs to act fairly?
