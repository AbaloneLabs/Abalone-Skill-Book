---
name: diegetic-and-contextual-ui-design.md
description: Use when the agent is deciding whether UI should be diegetic or non-diegetic, designing in-world interfaces and contextual prompts, integrating UI into the game world, or evaluating whether diegetic UI enhances immersion or produces readability problems, hidden information, and player confusion.
---

# Diegetic and Contextual UI Design

Diegetic UI — interface elements that exist within the game world, like a wristwatch health display or a holographic ammo counter — promises immersion by removing the artificial HUD layer, but it introduces a new set of failure modes centered on readability, consistency, and the conflict between immersion and clarity. The judgment problem is that diegetic UI that looks beautiful and immersive in a screenshot can be unreadable in motion, can hide critical information behind animation or perspective, and can be inconsistent with non-diegetic UI it must coexist with. Agents tend to miss this because the immersion benefit is visible and celebrated while the readability cost is silent, and because the line between diegetic and non-diegetic is often blurred in ways that confuse the player about what is interface and what is world. The harm is UI that is immersive but unusable, that hides information the player needs, or that breaks immersion by being inconsistent. This skill covers how to choose diegetic versus non-diegetic, ensure readability, and manage the immersion-clarity tradeoff. The agent has latitude in the UI's presentation, but the obligation to ensure the player can perceive the information is not optional.

## Core Rules

### Choose Diegetic Only When Readability Can Be Guaranteed

Diegetic UI should be used only when the information can be presented readably within the world element — clearly visible, consistently oriented, not occluded by animation or perspective. The decision rule: for each diegetic element, confirm it is as readable as an equivalent non-diegetic element across all gameplay situations, including movement, camera angles, and animation states; if it is not, use non-diegetic. Diegetic UI that sacrifices readability for immersion produces a game where the player cannot perceive the information they need, which defeats the UI's purpose regardless of its aesthetic success.

### Maintain Consistency Between Diegetic and Non-Diegetic Layers

Most games mix diegetic and non-diegetic UI, and the player must be able to distinguish which is which and trust both, so the layers must be consistent in what they present and how. The decision rule: define which information is diegetic and which is non-diegetic, apply the distinction consistently, and avoid presenting the same information in both layers in conflicting ways. Inconsistency — health diegetic in one screen, non-diegetic in another — confuses the player about where to look and breaks the immersion the diegetic layer was meant to create.

### Ensure Contextual Prompts Are Unambiguous and Timely

Contextual prompts — the "press X to interact" that appears near objects — must appear when the action is available, disappear when it is not, and clearly indicate what will happen, or they mislead the player. The decision rule: for each contextual prompt, confirm it appears exactly when the action is possible, describes the action accurately, and does not persist when the action is no longer available. Prompts that appear when the action is impossible, or that describe the wrong action, train the player to distrust the prompts, which destroys their usefulness.

### Avoid Hiding Critical Information Behind Diegetic Animation

Diegetic UI often animates — a wrist display flips up, a hologram projects — and if critical information is available only after the animation completes, the player is delayed in moments when they need the information immediately. The decision rule: ensure critical information in diegetic UI is available instantly or near-instantly, and reserve animation for non-critical or contextual information. Hiding health or ammo behind a flip animation produces deaths and failures attributable to the UI withholding information the player needed now, not in 1.5 seconds.

### Design Diegetic UI to Be Legible Across Camera and Perspective Changes

Diegetic UI is fixed in the world, so as the camera moves, the UI's orientation, scale, and occlusion change, and a display readable from one angle becomes unreadable from another. The decision rule: test diegetic UI across the full range of camera positions and perspectives the player will experience, and ensure it remains legible or provides an alternative. Diegetic UI that is readable only from the ideal angle fails whenever the camera deviates, which is constantly in dynamic gameplay.

### Use Diegetic UI to Reinforce World Cohesion, Not to Eliminate All Non-Diegetic UI

The goal of diegetic UI is to reinforce world cohesion where it serves the experience, not to eliminate all non-diegetic UI in pursuit of purity, because some information (objectives, settings, accessibility options) cannot be diegetically presented without harming usability. The decision rule: apply diegetic UI where it enhances immersion without sacrificing function, and accept non-diegetic UI where function requires it, rather than forcing purity. Games that pursue total diegetic purity sacrifice usability for an aesthetic principle the player does not benefit from.

## Common Traps

### Diegetic UI That Is Immersive but Unreadable

The team implements diegetic UI — a health readout on the character's wrist, ammo on the weapon model — that looks immersive and is celebrated for removing the HUD, but in motion the information is small, angled, or occluded, and the player cannot read it when they need it. The trap is that the diegetic UI looks great in screenshots. The false signal is that the HUD is gone and the world is immersive. The harm is that the player cannot perceive critical information, they die or fail because they could not read their health or ammo, and the immersion that was gained is lost to the frustration of an unusable interface, because readability was sacrificed for aesthetic purity.

### Inconsistent Diegetic and Non-Diegetic Layers

The game presents some information diegetically and other information non-diegetically, and the distinction is inconsistent — health is diegetic, stamina is non-diegetic — so the player does not know where to look for any given information. The trap is that each element was placed where it fit aesthetically. The false signal is that the UI is varied and interesting. The harm is that the player cannot build a reliable model of where information lives, every search for information requires scanning both layers, and the immersion the diegetic layer was meant to create is broken by the inconsistency, because the boundary between the layers was never defined or enforced.

### Contextual Prompts That Mislead

Contextual prompts appear when an action is not actually available, or describe an action different from what occurs, and the player presses the button expecting one result and gets another. The trap is that prompts are triggered by proximity rather than availability. The false signal is that prompts appear near interactive objects. The harm is that the player learns the prompts are unreliable, they begin ignoring them or second-guessing them, and the contextual UI that was meant to guide interaction instead produces distrust and errors, because the prompt's availability and accuracy were not tightly coupled to the actual action state.

### Critical Information Hidden Behind Animation

Diegetic UI animates before revealing information — a display boots up, a visor lowers — and critical information is delayed until the animation completes, so the player who needs the information immediately must wait or act blind. The trap is that the animation reinforces the diegetic feel. The false signal is that the UI is immersive and detailed. The harm is that the player is denied information in time-critical moments, deaths and failures result from information withheld by animation, and the diegetic detail that was meant to enhance immersion instead causes the frustration of acting without information, because the animation was prioritized over availability.

### Diegetic UI That Fails Across Camera Angles

The diegetic display is readable from the default camera angle, but as the camera moves — during combat, during traversal, during cutscenes — the display becomes angled, occluded, or too small, and the player cannot read it. The trap is that the default angle is the one tested. The false signal is that the UI is readable in the standard view. The harm is that the player cannot read the information during exactly the dynamic moments when they need it most, the diegetic UI that worked in static review fails in motion, and the player is denied information because the UI was validated only from the ideal perspective.

### Pursuing Total Diegetic Purity at the Cost of Usability

The team, committed to diegetic purity, forces all UI into the world, including information that cannot be diegetically presented without harming usability — settings menus, accessibility options, objective trackers — and the result is unusable or absurd diegetic contortions. The trap is that purity reads as a strong artistic vision. The false signal is that the game has no HUD. The harm is that usability suffers for an aesthetic principle, the player cannot access functions they need, and the purity that was meant to enhance immersion instead produces a game that is harder to use for no player benefit, because non-diegetic UI was eliminated where it was the right tool.

## Self-Check

- For each diegetic UI element, have I confirmed it is as readable as a non-diegetic equivalent across all gameplay situations, including motion and camera changes?
- Is the distinction between diegetic and non-diegetic layers consistent, with no information presented in conflicting ways across layers?
- Do contextual prompts appear exactly when the action is available, describe the action accurately, and disappear when it is not?
- Is critical information in diegetic UI available instantly, with animation reserved for non-critical or contextual displays?
- Have I tested diegetic UI legibility across the full range of camera positions and perspectives, not only the default angle?
- Did I use diegetic UI where it enhances immersion without sacrificing function, accepting non-diegetic UI where function requires it?
- Did I avoid pursuing total diegetic purity at the cost of usability for information and functions that cannot be diegetically presented?
