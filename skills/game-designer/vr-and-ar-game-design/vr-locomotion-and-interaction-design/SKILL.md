---
name: vr-locomotion-and-interaction-design.md
description: Use when the agent is designing VR locomotion and interaction systems, choosing between teleportation, smooth locomotion, and room-scale movement, building hand-based grab and manipulation, or evaluating whether VR interactions feel natural and physical or feel abstracted, imprecise, and disconnected from the player's bodily intention in virtual space.
---

# VR Locomotion and Interaction Design

Locomotion and interaction are the two foundational systems of VR design, and both are hard because VR inherits neither the camera control of flat-screen games (which would sicken) nor the button abstraction of controllers (which would break presence), requiring new approaches built on the player's body and physical space. The judgment problem is that locomotion must move the player through the world without sickness, interaction must let the player manipulate the world with their hands, and both must respect the physical room the player occupies, and agents tend to miss this because the locomotion and interaction that feel natural in testing (large room, fresh player) fail in real homes (small room, fatigued player). The harm is locomotion that sickens or traps, and interaction that feels abstracted or imprecise rather than natural and physical. This skill covers how to design VR locomotion and interaction that serve the player's body and space, and avoid the sickness, abstraction, and space traps. The agent has latitude in the systems, but the obligation to make locomotion and interaction serve the player is not optional.

## Core Rules

### Choose Locomotion Based on Comfort, Game Needs, and Space

VR locomotion options — teleportation (most comfortable, breaks presence least), smooth locomotion (most present, most sickening), room-scale (most physical, most space-demanding) — each trade comfort, presence, and space, and the choice must be based on the game's needs (a slow exploration game can use teleportation; a fast action game needs smooth) and the player's comfort and space. The decision rule: choose locomotion based on comfort, game needs, and space, offer alternatives where possible, and avoid forcing one option. Forced locomotion excludes players, because the option did not fit the comfort, game, or space.

### Design Hand-Based Interaction for Physical Manipulation

VR interaction should be hand-based — the player reaches, grabs, and manipulates with virtual hands — so the interaction feels physical and present, because hand-based interaction (responsive to the body) sustains presence while abstracted interaction (button-press) breaks it. The decision rule: design hand-based interaction for physical manipulation, and avoid abstracting interaction into button presses. Abstracted interaction breaks presence, because the interaction did not respond to the hands.

### Handle Object Grab, Physics, and Manipulation Reliably

Grabbing and manipulating objects in VR is technically hard (physics, collision, hand tracking), and the interaction must be reliable — objects grab when intended, physics respond as expected, manipulation is precise — because unreliable interaction (objects that slip, physics that spaz, manipulation that fails) frustrates and breaks presence. The decision rule: ensure grab, physics, and manipulation are reliable (robust grab detection, stable physics, precise manipulation), and avoid unreliable interaction. Unreliable interaction frustrates, because the interaction did not respond as expected.

### Respect the Player's Physical Play Space

VR is played in a physical play space (the room the player occupies), and the design must respect this space — not requiring movement beyond the room, handling boundary encounters gracefully — because designs that exceed the space (requiring a larger room than the player has) or ignore the boundary (letting the player walk into walls) harm the experience and the player. The decision rule: respect the physical play space (design for common room sizes, handle boundaries gracefully), and avoid space-exceeding or boundary-ignoring design. Space-exceeding design excludes players, because the room could not support the design.

### Provide Seated and Standing Modes for Accessibility

VR players play seated or standing, and the game should provide both modes (adjusting height, reach, and interaction for each) so seated players are not excluded, because a standing-only game excludes seated players (accessibility, disability, preference). The decision rule: provide seated and standing modes, adjust the design for each, and avoid standing-only design. Standing-only design excludes seated players, because the mode did not accommodate the seated player.

### Integrate Locomotion and Interaction With the Game's Core Loop

Locomotion and interaction are not standalone systems — they must integrate with the game's core loop (the locomotion serves the exploration, the interaction serves the manipulation the loop requires) — because locomotion and interaction that exist apart from the loop feel disconnected and purposeless. The decision rule: integrate locomotion and interaction with the core loop, ensure they serve the loop's actions, and avoid disconnected systems. Disconnected systems feel purposeless, because the locomotion and interaction did not serve the loop.

## Common Traps

### Forced Locomotion Excluding Players

The team forces one locomotion option without alternatives, and players who cannot tolerate or use the option are excluded. The trap is that the option is the team's preference. The false signal is that the option works for the team. The harm is that the forced option sickens sensitive players (smooth locomotion) or bores tolerant players (teleportation-only), the players who cannot use the option are excluded, the audience is narrowed, and the game fails to reach the players whose comfort or preference the option did not fit, because the locomotion was not chosen for the range.

### Abstracted Interaction Breaking Presence

The team abstracts interaction into button presses (grab with a button, manipulate with a stick), and the abstraction breaks presence. The trap is that the abstraction is convenient. The false signal is that the interaction works. The harm is that the abstracted interaction does not respond to the player's hands, the player presses buttons instead of reaching, the physical manipulation that sustains presence is replaced, the world does not respond to the body, and the presence is weakened, because the interaction was abstracted.

### Unreliable Grab and Physics Frustrating Players

The team ships grab and physics that are unreliable — objects slip, physics spaz, manipulation fails — and the unreliability frustrates. The trap is that the interaction works in testing. The false signal is that the grab detects. The harm is that the unreliable interaction fails in real play (objects slip, physics spaz), the player's intended actions do not register, the frustration accumulates, the presence is broken by each failure, and the experience is remembered as janky rather than physical, because the interaction was not reliable.

### Space-Exceeding Design Excluding Players

The team designs for a large play space, and the player with a smaller room is excluded. The trap is that the design fits the team's space. The false signal is that the design works. The harm is that the space-exceeding design requires a larger room than the player has, the player cannot perform the required movements, the boundary encounters are not handled gracefully, the player is excluded or harmed (walking into walls), and the game fails for players whose space the design exceeded, because the space was not respected.

### Standing-Only Design Excluding Seated Players

The team designs for standing only, and the seated player is excluded. The trap is that standing is the intended mode. The false signal is that the game is playable standing. The harm is that the standing-only design excludes seated players (accessibility, disability, preference), the seated player cannot reach or interact as the design requires, the audience is narrowed, and the game fails to reach the players whose mode the design did not provide, because seated mode was not supported.

### Disconnected Locomotion and Interaction Feeling Purposeless

The team designs locomotion and interaction apart from the core loop, and the systems feel disconnected and purposeless. The trap is that the systems are functional. The false signal is that the systems work. The harm is that the locomotion and interaction do not serve the loop's actions, the systems exist apart from the game's purpose, the player moves and manipulates without integration, the systems feel like tech demo rather than game, and the experience is purposeless, because the locomotion and interaction were not integrated with the loop.

## Self-Check

- Is locomotion chosen based on comfort, game needs, and space, with alternatives offered rather than forced?
- Is interaction hand-based for physical manipulation rather than abstracted into button presses?
- Are grab, physics, and manipulation reliable (robust detection, stable physics, precise manipulation)?
- Does the design respect the player's physical play space, handling boundaries gracefully?
- Are seated and standing modes provided, with the design adjusted for each?
- Are locomotion and interaction integrated with the core loop, serving the loop's actions?
- Did I confirm VR locomotion and interaction feel natural and physical rather than abstracted and disconnected?
