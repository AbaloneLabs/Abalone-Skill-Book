---
name: vr-presence-and-comfort-design.md
description: Use when the agent is designing VR game presence and comfort, planning immersion and embodiment mechanics, addressing motion sickness and comfort options, or evaluating whether a VR experience sustains presence or breaks it through comfort-violating motion, poor embodiment, and interactions that remind the player they are in a headset rather than inside the world.
---

# VR Presence and Comfort Design

Virtual reality's defining promise is presence — the felt sense of being inside the world — and presence is fragile, easily broken by motion that causes sickness, by embodiment that reminds the player of the headset, and by interactions that fight the player's body rather than responding to it. The judgment problem is that VR design must sustain presence (through convincing embodiment, responsive interaction, and world consistency) while managing the comfort constraints that VR imposes (motion sickness, fatigue, accommodation), and agents tend to miss this because the design that maximizes immersion in a non-VR sense (fast motion, rapid camera changes) is exactly the design that destroys VR presence by making the player sick. The harm is VR experiences that break presence through comfort violations, that sicken the player, or that remind the player of the headset rather than letting them forget it. This skill covers how to design VR presence and comfort together, and avoid the sickness, embodiment, and presence-breaking traps. The agent has latitude in the VR design, but the obligation to sustain presence without harming the player is not optional.

## Core Rules

### Design Motion to Minimize Sickness and Sustain Presence

VR motion is the primary cause of motion sickness (when the visual motion does not match the vestibular motion), and motion must be designed to minimize sickness — teleportation, vignetting, snap-turn, cockpit framing — so the player can move without nausea, because sickening motion destroys presence (the player is aware of the sickness, not the world). The decision rule: design motion to minimize sickness (comfort locomotion options), provide alternatives for sensitive players, and avoid sickening motion. Sickening motion destroys presence, because the player felt the sickness, not the world.

### Build Convincing Embodiment That Lets the Player Forget the Headset

Presence depends on embodiment — the player's body in the world responding to their real body — and embodiment must be convincing (the virtual hands track the real hands, the body matches the player's actions) so the player forgets the headset and inhabits the body, because embodiment gaps (hands that do not track, bodies that do not respond) remind the player of the mediation. The decision rule: build convincing embodiment (responsive tracking, matching actions), and avoid embodiment gaps that remind the player of the headset. Embodiment gaps break presence, because the player noticed the mediation.

### Make Interactions Respond to the Player's Body and Intention

VR interactions should respond to the player's body and intention — reaching, grabbing, manipulating with the hands — so the world feels responsive and present, because interactions that do not respond to the body (pre-canned animations, button-press abstractions) remind the player of the mediation. The decision rule: design interactions to respond to the player's body and intention, and avoid abstractions that replace bodily interaction. Abstracted interactions break presence, because the interaction did not respond to the body.

### Provide Comprehensive Comfort Options for Varying Tolerance

VR players vary widely in motion sickness tolerance, and the game must provide comprehensive comfort options — locomotion style, vignette strength, snap-turn angle, seated/standing modes — so each player can configure the experience to their tolerance, because a single motion design excludes the sensitive or bores the tolerant. The decision rule: provide comprehensive comfort options for varying tolerance, default to comfortable settings, and avoid forcing one motion design. Forced motion excludes sensitive players, because the design did not accommodate the tolerance range.

### Design for VR Fatigue and Session Length Constraints

VR is physically demanding (holding arms up, moving the body), and the design must account for fatigue — avoiding extended arm-held actions, providing rest, pacing sessions — because fatiguing VR ends sessions and harms the experience, and VR sessions are naturally shorter than flat-screen sessions. The decision rule: design for fatigue (avoid extended strain, provide rest, pace sessions), and avoid fatiguing mechanics. Fatiguing mechanics end VR sessions, because the physical strain exceeded the player's tolerance.

### Maintain World and Interaction Consistency to Sustain Presence

Presence depends on consistency — the world behaves as expected, interactions respond as expected — and consistency must be maintained, because inconsistencies (objects passing through each other, interactions failing) break presence by reminding the player of the simulation. The decision rule: maintain world and interaction consistency (solid objects, reliable interactions), and avoid inconsistencies that break presence. Inconsistencies break presence, because the player noticed the simulation's failure.

## Common Traps

### Sickening Motion Destroying Presence

The team designs motion that causes motion sickness (smooth locomotion, rapid acceleration, camera manipulation), and the sickness destroys presence. The trap is that the motion is immersive in flat-screen terms. The false signal is that the motion is smooth. The harm is that the visual-vestibular mismatch sickens the player, the player is aware of the sickness rather than the world, the presence is destroyed by the discomfort, the sensitive player cannot play, and the experience is remembered as nauseating rather than present, because the motion was not designed for comfort.

### Embodiment Gaps Reminding the Player of the Headset

The team builds embodiment with gaps — hands that do not track, bodies that do not respond — and the gaps remind the player of the headset. The trap is that the embodiment is present. The false signal is that the player has a body. The harm is that the embodiment gaps (untracked hands, unresponsive bodies) remind the player of the mediation, the player notices the headset rather than inhabiting the body, the presence is broken by the gap, and the experience fails to convince, because the embodiment was not convincing.

### Abstracted Interactions Replacing Bodily Engagement

The team abstracts interactions (button-press instead of reach, pre-canned animation instead of manipulation), and the abstraction replaces bodily engagement. The trap is that the abstraction is convenient. The false signal is that the interaction works. The harm is that the abstracted interaction does not respond to the player's body, the player presses a button instead of reaching, the bodily engagement that sustains presence is replaced, the world does not respond to the body, and the presence is weakened, because the interaction was abstracted.

### Forced Motion Excluding Sensitive Players

The team forces one motion design without comfort options, and the sensitive player is excluded. The trap is that the motion design is the team's preference. The false signal is that the motion works for the team. The harm is that the sensitive player (a large portion of the VR audience) cannot tolerate the forced motion, the player is excluded or sickened, the audience is narrowed to the tolerant minority, and the game fails to reach the players whose tolerance the design did not accommodate, because the motion was not configurable.

### Fatiguing Mechanics Ending VR Sessions

The team designs fatiguing mechanics — extended arm-held actions, constant movement — and the fatigue ends sessions. The trap is that the mechanics are active. The false signal is that the mechanics are engaging. The harm is that the physically demanding mechanics fatigue the player, the fatigue accumulates and ends the session, the session length (and engagement) is limited by the physical strain, the player avoids the game to avoid the fatigue, and the experience is remembered as exhausting rather than engaging, because the mechanics were not designed for fatigue.

### Inconsistencies Breaking Presence

The team allows world and interaction inconsistencies — objects passing through each other, interactions failing — and the inconsistencies break presence. The trap is that the inconsistencies are minor. The false signal is that the world is functional. The harm is that the inconsistencies (clipping objects, failed interactions) remind the player of the simulation, the player notices the mediation rather than the world, the presence is broken by each inconsistency, and the experience fails to sustain the felt sense of being inside, because the consistency was not maintained.

## Self-Check

- Is motion designed to minimize sickness (comfort locomotion, vignetting, snap-turn) rather than sickening the player?
- Is embodiment convincing (responsive tracking, matching actions) so the player forgets the headset?
- Do interactions respond to the player's body and intention rather than abstracting them into button presses?
- Are comprehensive comfort options provided for varying motion sickness tolerance, defaulting to comfortable settings?
- Is the design accounting for VR fatigue, avoiding extended strain and providing rest?
- Are world and interaction consistency maintained to avoid presence-breaking inconsistencies?
- Did I confirm the VR experience sustains presence rather than breaking it through comfort violations and embodiment gaps?
