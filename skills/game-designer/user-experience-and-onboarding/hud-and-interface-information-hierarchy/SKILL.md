---
name: hud-and-interface-information-hierarchy.md
description: Use when the agent is designing HUDs, UI layouts, menu systems, iconography, notification systems, damage indicators, minimaps, objective trackers, diegetic versus non-diegetic interfaces, or reviewing whether on-screen information is prioritized, legible, and non-occluding during gameplay.
---

# HUD and Interface Information Hierarchy

The heads-up display is the layer through which the player perceives the game state, and its central problem is not what to show but what to withhold. The judgment problem is that every team member can justify adding their system's information to the screen — the combat designer wants ammo, the narrative designer wants dialogue subtitles, the economy designer wants currency, the quest designer wants objective markers — and without a ruthless editor, the HUD accretes until it competes with the gameplay for the player's attention. Designers miss this because each addition seems reasonable in isolation and because removing information feels like reducing functionality, when in fact clarity is the functionality. The harm is a cluttered screen that obscures the game world, buries urgent information among trivia, increases cognitive load during the moments that demand the player's full attention, and produces a game that feels visually noisy and stressful even when the underlying systems are sound. The opposite failure — an under-informed HUD — leaves the player guessing at state they need, producing frustration and the sense that the game is withholding. Agents tend to err toward addition, because adding a widget is easier than arguing for its removal, and because the cost of clutter is diffuse and slow to manifest while the cost of missing information is immediate and localized. The freedom here is wide — HUD style is genre-defining — but the obligation is to treat every on-screen element as a claim on the player's attention that must justify its presence against everything else competing for that same attention.

## Core Rules

### Treat Screen Space as a Scarce Resource With an Attention Budget

The player's visual attention is finite, and every HUD element spends some of it. The discipline is to conceptualize the HUD as having an attention budget: there is a limited amount the player can parse in the heat of gameplay, and every widget, number, icon, and marker draws from that budget before the player can attend to the actual game world. The decision criterion: at the moment of peak gameplay intensity, which elements must be visible, and does the sum of their attention cost leave room for the player to perceive the world? When the answer is that the HUD consumes more attention than the action, the interface is failing its purpose. Audit every element against the question "would the player be worse off without this during combat?" and cut or contextualize everything that does not survive that test.

### Prioritize by Urgency, Frequency, and Consequence of Ignorance

Not all information deserves equal prominence. The correct hierarchy places the most urgent, most frequently needed, and most consequential information in the most perceptually prominent positions, and demotes the rest. Health, when death is possible in the next second, is urgent and belongs at the periphery of the focal area where it is always visible. Currency balance, needed only when shopping, is not urgent and belongs in a menu. The decision criterion: for each piece of information, ask how quickly the player must react to it (urgency), how often they need it (frequency), and what happens if they miss it (consequence). Rank elements by the combination of these three, and assign screen position and visual weight accordingly. When a low-urgency element occupies a high-prominence position, it crowds out the information that actually matters.

### Make Critical State Peripheral and Always-On, Make Detail On-Demand

The strongest HUD pattern separates always-on summary state from on-demand detail: the player should be able to glance at the screen edge at any moment and know their vital status (health, ammo, objective direction) without focusing on it, and should be able to summon deeper information (full inventory, detailed map, stat breakdowns) only when they choose to. The trap is inverting this — making the player open a menu to check health, or permanently displaying exhaustive detail that cannot be ignored. The discipline is to design two tiers: a minimal, glanceable, always-present layer for state the player continuously needs, and a summoned, dismissible layer for state the player needs intermittently. The decision criterion: can the player play the core loop without ever opening a menu, and can they access any deeper information within one or two inputs when they want it?

### Use Diegetic and Contextual Presentation to Reduce Clutter

Not every piece of information needs a permanent widget. Diegetic presentation — health as a glowing crystal on the character's back, ammo count on the weapon model, objective direction as a sign in the world — communicates state through the game world itself and costs no HUD space. Contextual presentation — a damage indicator that appears only when the player takes damage, then fades — shows information only at the moment it is relevant. The discipline is to prefer diegetic and contextual over permanent non-diegetic widgets wherever the game's tone supports it, because these approaches inform without cluttering. The decision criterion: for each permanent widget, ask whether the same information could be conveyed by the world or shown only when relevant. When it can, remove the widget. Reserve permanent non-diegetic elements for state that is continuously needed and cannot be reasonably diegetic.

### Ensure Legibility Against the Full Range of Game Backgrounds

A HUD element designed and tested against a neutral background will be illegible against the bright skybox, the dark dungeon, the chaotic explosion, or the high-contrast foliage that the game actually contains. The discipline is to validate every text element, icon, and indicator against the full range of backgrounds the player will encounter, using contrast ratios, outlines, drop shadows, and color-blind-safe palettes to guarantee legibility regardless of what is behind the UI. The decision criterion: is this element readable against the brightest, darkest, busiest, and most color-similar backgrounds in the game? When legibility depends on a specific background, the design is broken. Health bars that disappear against red explosions, white text that vanishes against white snow, and red-green coding that is invisible to color-blind players are all consequences of testing UI in isolation from the world it overlays.

### Design Notification Systems to Inform Without Hijacking Attention

Notifications — level ups, quest updates, achievement pops, item pickups, system messages — are necessary but dangerous, because each one demands a moment of the player's attention and a screen region, and uncontrolled they stack into a cascade that obscures gameplay. The discipline is to design notifications with explicit rules: where they appear (periphery, never focal), how long they persist (short, with graceful fade), how they queue and coalesce (multiple pickups summarized, not stacked), and whether they pause gameplay (almost never, unless the information is safety-critical). The decision criterion: during a thirty-second window of intense combat, could the notification system stack enough messages to obscure the focal area, and if so, what rule prevents that? Notifications that can interrupt or occlude at the worst moment are a design defect, not a feature.

## Common Traps

### Feature Creep Into a Cluttered Screen

Each system owner successfully argues that their information deserves screen space, and over development the HUD accumulates widgets until it frames the gameplay rather than supporting it. The trap is that no single addition caused the problem; the clutter emerged from the sum of reasonable individual decisions, and by the time it is visible, removing anything requires a political fight. The false signal is that every element is "justified." The harm is a screen that overwhelms new players, obscures the world, and makes the game feel busy and stressful, with the root cause — the absence of a ruthless editor — never addressed.

### Permanent Display of Detail the Player Needs Only Occasionally

The full stat readout, the complete inventory grid, the detailed minimap with every NPC — all permanently visible because the designer feared the player might want them. The trap is that permanent detail cannot be ignored and forces the player to continuously filter, which is cognitively expensive during gameplay. The false signal is that the information is "available when needed." The harm is constant low-grade attention drain that makes the game feel harder to play than it is, because the player is doing the UI's filtering job mentally.

### HUD Elements That Occlude the Focal Area at Critical Moments

A notification, prompt, or effect overlay appears in the center of the screen — exactly where the player is aiming — during the moment they most need to see the world. The trap is that the element was placed centrally because it "needed to be seen," without considering that it blocks what the player is trying to see. The false signal is that the player noticed the notification. The harm is that the player also failed to notice the enemy, the projectile, or the hazard that killed them, producing deaths that feel unfair and a game that feels like it is fighting the player.

### Color-Only State Coding With No Redundant Cue

Health, status effects, or threat levels are communicated exclusively through color — green is safe, red is danger — with no shape, icon, or position redundancy. The trap is that color-blind players (a significant fraction of any audience) cannot distinguish the states, and even color-normal players struggle when the colors blend into a colorful game background. The false signal is that the coding "works for most players." The harm is a subset of players who cannot read critical state and who experience the game as broken or unfair, alongside accessibility-failure reputational damage.

### Static UI Validated Only Against Neutral Backgrounds

The HUD is designed and reviewed in a UI editor with a gray background, never tested against the actual game's varied environments. The trap is that elements that are perfectly legible in the editor vanish or become unreadable against real backgrounds — white text on snow, dark icons in shadow, thin outlines lost against detailed textures. The false signal is that the UI "looks good in mockup." The harm is in-game illegibility that is discovered only in playtest or post-launch, requiring emergency outline and contrast passes that could have been designed in from the start.

### Notification Cascades That Stack and Obscure

Multiple notifications trigger near-simultaneously — a level up, an achievement, three item pickups, a quest update — and each spawns its own on-screen element that stacks into a column covering a quarter of the display. The trap is that the notification system has no coalescing or queue-management rule, so volume scales linearly with events. The false signal is that each notification "worked correctly" in isolation. The harm is that during reward-heavy moments, which are also often combat moments, the screen fills with popups that the player must wait through or that obscure the action, souring the very moments meant to feel rewarding.

## Self-Check

- Have I audited every HUD element against an attention budget, confirming that the sum of always-on elements leaves room for the player to perceive the game world during peak intensity?
- Is each element positioned and weighted according to its urgency, frequency, and consequence-of-ignorance, with no low-urgency information occupying high-prominence space?
- Is there a clear two-tier structure: minimal always-on glanceable state for continuous needs, and summoned on-demand detail for intermittent needs, so the core loop never requires a menu?
- Have I replaced permanent non-diegetic widgets with diegetic or contextual presentation wherever the game's tone supports it?
- Is every text element, icon, and indicator legible against the brightest, darkest, busiest, and most color-similar backgrounds in the game, with contrast validated rather than assumed?
- Does the notification system have explicit rules for position, duration, queueing, and coalescing that prevent stacking from obscuring the focal area during high-event moments?
- Is all color-based state coding redundant with shape, icon, or position so that color-blind players can read critical state?
