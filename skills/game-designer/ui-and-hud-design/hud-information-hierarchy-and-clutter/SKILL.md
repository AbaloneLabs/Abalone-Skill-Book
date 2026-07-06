---
name: hud-information-hierarchy-and-clutter.md
description: Use when the agent is designing a heads-up display, deciding which information to show persistently versus on demand, organizing HUD element hierarchy, or evaluating whether a HUD communicates essential information clearly or clutters the screen, competes for attention, and obscures the game.
---

# HUD Information Hierarchy and Clutter

A HUD is a constant competitor with the game for the player's attention, and every element on it must justify its presence against the cost of the screen real estate and cognitive load it consumes. The judgment problem is that each piece of information someone wants displayed is individually reasonable — health matters, ammo matters, the minimap matters, the objective matters — and the accumulation of individually-reasonable elements produces a cluttered HUD that obscures the game and overwhelms the player. Agents tend to miss this because each element is added to solve a real need and removing any feels like losing information, and because clutter accumulates gradually through additions rather than in a single decision. The harm is a HUD that competes with the gameplay, that the player must parse before they can play, and that obscures the very action it exists to support. This skill covers how to establish HUD hierarchy, decide what earns persistent display, and resist clutter accumulation. The agent has latitude in the HUD's content, but the obligation to make every element earn its place is not optional.

## Core Rules

### Display Persistently Only What the Player Must Monitor Continuously

Persistent HUD elements should be reserved for information the player must monitor continuously to play — typically health, a critical resource, the objective marker — because each persistent element is a permanent tax on attention and screen space. The decision rule: for each persistent element, confirm the player must track it in real time to engage the core loop; if the information is needed only occasionally, relegate it to on-demand display. A HUD that persists everything the player might want clutters the screen with information that is relevant only sometimes, drowning the always-relevant information in noise.

### Relegate Occasional Information to On-Demand or Contextual Display

Information the player needs sometimes but not always — detailed maps, full inventory, secondary objectives, lore — should appear on demand (a button press) or contextually (when relevant), not persistently. The decision rule: classify each piece of information by how often the player needs it, and move anything below "continuously" to on-demand or contextual presentation. This frees the persistent HUD for the essential and prevents the occasional from competing with the constant.

### Establish a Clear Visual Hierarchy Matching Information Importance

Within the HUD, elements must be visually ordered by importance: the most critical information largest, most contrasted, most centrally placed; secondary information smaller and more peripheral. The decision rule: define the importance hierarchy of all displayed information and apply a consistent visual treatment (size, contrast, position) that lets the player's eye find the critical information first. A HUD without hierarchy forces the player to scan everything to find anything, which is cognitive load the gameplay should not require.

### Locate Elements to Avoid Obscuring Gameplay-Critical Action

HUD elements occupy screen space the gameplay also needs, and placing them over the gameplay's focal area — the crosshair's sightline, the character's feet, the approaching threat's path — forces the player to read the HUD through the game or the game through the HUD. The decision rule: map the screen's gameplay-critical zones for each situation and place HUD elements in the peripheral zones that do not compete. A HUD that overlaps the action it should frame produces a game the player cannot clearly see, regardless of how well-organized the HUD itself is.

### Make HUD Elements Fade or Adapt to Gameplay Context

A HUD that displays all elements at full intensity at all times competes with the game even when the information is not currently relevant; a HUD that fades non-essential elements during low-information moments and intensifies them when relevant reduces the constant tax. The decision rule: implement contextual HUD behavior where elements appear, intensify, or fade based on gameplay state, showing full detail only when the player needs it. Static full-intensity HUDs tax attention even in calm moments when the information is irrelevant, draining focus from the experience.

### Validate HUD Clutter With Eye-Tracking or Attention Observation

The player's experience of clutter is perceptual, and a HUD that looks organized in a static review may overwhelm in motion when the player must also track gameplay. The decision rule: validate the HUD with attention observation — where do testers' eyes go, what do they miss, what do they report as distracting — and revise based on where the HUD competes with rather than supports perception. HUDs reviewed only in static screenshots pass review while failing the player, because the competition for attention only appears in dynamic play.

## Common Traps

### Displaying Everything Persistently Because Each Element Is Justified

The team adds each HUD element to serve a real need, each is individually justified, and the accumulation produces a cluttered HUD where no single element can be removed without losing information, yet the whole is overwhelming. The trap is that each element has a defensible purpose. The false signal is that every displayed element is useful. The harm is that the player must parse a dense HUD before engaging the game, the essential information is buried among the occasionally-useful, the screen feels cluttered and the game obscured, and the HUD that was built to inform instead overwhelms, because the discipline of relegating occasional information to on-demand was never applied.

### Uniform Visual Treatment With No Hierarchy

All HUD elements receive similar visual treatment — similar size, similar contrast, similar position weight — so the player cannot distinguish critical from secondary at a glance and must scan the whole HUD to find what matters. The trap is that uniform treatment looks clean and consistent. The false signal is that the HUD is well-designed visually. The harm is that the player's eye cannot find the critical information without scanning, the scan time is cognitive load the gameplay imposes, and in time-critical moments the player cannot locate the health or ammo they need because it was not visually prioritized, because the hierarchy that would guide the eye was never established.

### HUD Elements Overlapping the Gameplay Focal Area

HUD elements are placed over the screen's center or the gameplay's focal zone — the crosshair sightline, the character's immediate surroundings — and the player must read the game through the HUD or dismiss the HUD to see the game. The trap is that the screen edges feel too peripheral for important information. The false signal is that the HUD is visible and central. The harm is that the HUD competes with the gameplay for the same screen space, the player cannot clearly see the action the HUD should frame, and the information that should support the game instead obscures it, because the placement prioritized HUD visibility over gameplay visibility.

### Static Full-Intensity HUD That Taxes Attention Constantly

The HUD displays all elements at full intensity at all times, including during calm moments when most information is irrelevant, and the player's attention is continuously taxed by information they do not currently need. The trap is that a static HUD is simpler to implement. The false signal is that all information is always available. The harm is that the constant display of irrelevant information trains the player to ignore the HUD, so when critical information appears it is missed, and the HUD that should direct attention instead produces habituation, because it never adapted to what the player needed when.

### Validating HUD Only in Static Screenshots

The team reviews the HUD in static screenshots and approves it as organized and clear, but in dynamic play the HUD competes with moving gameplay for attention in ways the screenshot did not reveal. The trap is that screenshots are the easy review artifact. The false signal is that the HUD looks clean in review. The harm is that the clutter and competition only appear in motion, the player experiences the HUD as overwhelming where the screenshot showed it as organized, and the HUD ships with attention problems the static review could not detect, because the HUD was validated in a medium that removed the factor that governs its quality.

## Self-Check

- Is every persistent HUD element justified by the player's need to monitor it continuously, with occasional information moved to on-demand or contextual display?
- Have I classified all displayed information by frequency of need and relegated anything below continuous to on-demand presentation?
- Does the HUD have a clear visual hierarchy (size, contrast, position) that lets the player find critical information without scanning?
- Are HUD elements placed in peripheral zones that do not overlap the gameplay-critical focal area?
- Does the HUD fade or adapt element intensity based on gameplay context, showing full detail only when relevant?
- Did I validate the HUD with attention observation in dynamic play, not only in static screenshots?
- Did I resist adding each individually-justified element without considering the cumulative clutter cost?
