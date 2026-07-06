---
name: control-schemes-and-input-mapping.md
description: Use when the agent is designing control schemes, input mappings, default keybinds, controller layouts, rebinding systems, context-sensitive actions, multi-platform input, accessibility input options, or reviewing whether controls feel intuitive, discoverable, and conflict-free across keyboard, mouse, and gamepad.
---

# Control Schemes and Input Mapping

Controls are the single point of contact between the player's intent and the game's response, and they are evaluated in milliseconds of muscle memory, not in design documents. The judgment problem is that a control scheme is not a list of button assignments; it is a spatial and ergonomic argument about where the player's hands rest, which actions must be simultaneous, which actions are urgent, and how much cognitive load each mapping imposes. Designers miss this because they treat input mapping as a late-stage task — assigning functions to whatever buttons remain — rather than as a foundational decision that shapes feel, accessibility, and the entire interaction vocabulary. The harm is severe and often invisible until playtest: controls that fight the player produce a game that feels "janky" or "clunky" regardless of how good the underlying systems are, and players cannot articulate why, only that it does not feel right. Worse, poor defaults that cannot be rebound lock out players with motor limitations, non-standard hardware, or simply different preferences, shrinking the addressable audience. Agents tend to err by optimizing for the designer's own hands, by overloading buttons with context-sensitive stacks that the player cannot predict, or by treating remapping as a nice-to-have rather than a baseline requirement. The freedom here is real — there is no universally correct layout — but the obligation is to design controls as an ergonomic and cognitive system, validate them on the target hardware with real hands, and never ship a scheme that has only been tested by the person who designed it.

## Core Rules

### Map by Frequency, Urgency, and Simultaneity, Not by Category

The correct placement of an action is determined by how often it is used, how urgently it must be reachable, and whether it must be performed at the same time as other actions — not by which logical category it belongs to. Movement and camera, the most frequent and most urgent actions, claim the primary digits: the thumbs on sticks, WASD and mouse on keyboard. Combat actions that must be performed while moving claim the fingers that remain free during movement. The trap is assigning actions by theme (all "inventory" actions on one button cluster) which produces layouts where a rarely used action occupies a prime position and a frequent one is buried. The decision criterion: list every action with its frequency (per minute), its urgency (can the player wait half a second), and its simultaneity constraints (must it co-occur with movement, aiming, or other actions), then assign the most frequent, urgent, and simultaneous actions to the most accessible inputs first. When two actions conflict for the same input, the one with higher combined frequency-urgency-simultaneity wins.

### Respect Platform Conventions Unless You Have a Defended Reason to Break Them

Players arrive with thousands of hours of muscle memory from other games in the genre, and violating conventions imposes a relearning tax that reads as poor design even when the alternative is internally superior. On gamepad, the face buttons have genre-established meanings: A/X to jump or confirm, X/A to interact, RT to fire, LT to aim. On keyboard, WASD for movement, mouse for camera and primary action, Shift for sprint, Ctrl or C for crouch, E for interact, R for reload. The discipline is to default to convention and to treat any deviation as a decision requiring an explicit, written justification that outweighs the relearning cost. When you do break convention, the benefit must be substantial and demonstrable in playtest, not merely a preference. A game that remaps jump to a shoulder button for internal consistency but forces every player to fight their reflexes has made a poor trade.

### Make Context-Sensitive Actions Predictable, Not Surprising

Context-sensitivity — one button doing different things based on situation — is a powerful tool for fitting many actions onto few buttons, but it becomes a trap when the player cannot predict what will happen before they press. The discipline is that a context-sensitive action must be legible: the game must communicate, through prompt, highlight, or consistent rule, what the button will do in the current context before the player commits. The deeper rule is that context-sensitive actions should never map fundamentally different verbs to the same input in adjacent contexts, because the player will press expecting one and get the other. The decision criterion: can the player, looking at the current situation, state with confidence what pressing the button will do? When the answer depends on invisible state, the mapping is broken. Group context-sensitive actions by verb family (all "interact" variants on one button), never by accident.

### Provide Full Remapping as a Baseline, Not a Feature

Full input remapping is an accessibility requirement, not a premium feature. Players have different hand sizes, motor limitations, hardware configurations, and accumulated habits, and a fixed scheme excludes a measurable fraction of the audience. The discipline is that every action must be independently rebindable, with no hard-coded inputs, no actions locked to specific devices, and support for secondary bindings so a player can map an action to two inputs. The decision criterion: can a player using one hand, a custom adaptive controller, or a non-QWERTY layout configure the game to be playable? When the answer is no, the game fails an accessibility baseline that is increasingly a legal and market expectation. Treat default schemes as opinionated starting points, never as the only option.

### Handle Input Conflicts and Simultaneous Presses Deliberately

When two actions share an input through modifiers or context, or when the player presses inputs the game does not expect simultaneously, the result is often undefined behavior: the wrong action fires, inputs are dropped, or the game enters an unintended state. The discipline is to explicitly define the resolution order for every input conflict: which action wins when two are pressed, how modifiers combine, what happens on simultaneous press of conflicting actions, and how the system recovers from a held input when context changes. The decision criterion: for every pair of actions that can be triggered near-simultaneously, is the outcome defined and tested? Unhandled conflicts manifest as "the game ate my input" complaints that players attribute to lag or bugs, when the real cause is an undefined resolution rule.

### Account for the Full Range of Target Hardware and Input Devices

A scheme that feels perfect on an Xbox controller may be unplayable on a keyboard, agonizing on a touch screen, or impossible on an adaptive switch device. The discipline is to design and validate input separately for every supported platform, recognizing that the constraints differ fundamentally: a gamepad has analog sticks and a fixed button count; a keyboard has many keys but no analog input by default; a touch screen has no tactile feedback and obscures the display; motion and voice inputs have reliability limits. The decision criterion: for each platform, have I designed a scheme that is ergonomic on that platform's native hardware, rather than porting one platform's logic to another's constraints? A keyboard scheme that is a literal translation of a gamepad layout (eight directions, no mouse look) fails the player on that platform.

## Common Traps

### The Designer's-Hands Default

The control scheme is tuned by the designer who built it, played with it for hundreds of hours, and can no longer perceive its friction. The trap is that the scheme feels perfect to its creator and awkward to everyone else, because the creator has adapted to quirks that new players experience as obstacles. The false signal is that internal testing reports the controls "feel good." The harm is a game that reviews poorly for "clunky controls" despite the team's confidence, because the only hands that validated it were biased ones.

### Overloading a Single Button With a Context Stack

To fit many actions onto a controller, the designer stacks four or five context-sensitive functions onto one button — interact, revive, pick up, swap weapon, contextual special — resolved by invisible state. The trap is that the player presses expecting one outcome and gets another, especially in tense moments where the context changed faster than the player perceived. The false signal is that "it works on paper" because every context has a defined resolution. The harm is misfires that feel like the game betrayed the player's input, producing frustration that players cannot diagnose and attribute to the game being unfair.

### Hard-Coding Inputs That Cannot Be Rebound

Core actions — jump, attack, interact — are bound to fixed inputs in code, with no remapping path exposed, because exposing it was scoped out or deemed unnecessary. The trap is that this locks out players whose hardware, disability, or preference requires a different mapping, and it is often discovered only post-launch through accessibility complaints. The false signal is that "most players use the defaults." The harm is a preventable exclusion of players who would otherwise be able to play, and reputational damage in accessibility communities that track these failures.

### Ignoring Simultaneous-Input Edge Cases

The scheme works when actions are pressed in sequence but breaks when the player presses movement, camera, and three actions at once — a common situation in combat. The trap is that the resolution logic was never stress-tested under simultaneous load, so inputs are dropped or the wrong action fires under realistic play. The false signal is that sequential testing passes. The harm is "input dropping" in the moments that matter most, which players experience as the game failing to respond and which is nearly impossible to reproduce in isolated testing.

### Porting One Platform's Logic to Another's Constraints

A game designed for gamepad is ported to keyboard by literally mapping buttons to keys, preserving on-rails camera and discrete movement that the mouse and keyboard could handle far better. The trap is that the port respects the original scheme's logic rather than the target platform's affordances, producing a controls experience that feels like a compromise. The false signal is feature parity — "all functions are mapped." The harm is a platform version that feels worse than it should and reviews that penalize the port specifically.

### Inconsistent Action-to-Button Mapping Across Modes

In gameplay, X interacts; in a menu, X cancels; in a vehicle, X accelerates; in a minigame, X does nothing. The trap is that the same input means opposite things in adjacent contexts, and the player's muscle memory betrays them at every transition. The false signal is that each context is internally consistent. The harm is constant relearning and frequent mispresses at mode boundaries, which accumulate into a pervasive feeling that the controls are unreliable.

## Self-Check

- Did I assign inputs based on frequency, urgency, and simultaneity constraints rather than logical category, so that the most-used and most-urgent actions occupy the most accessible inputs?
- Did I default to platform conventions for every standard action, and document a defended justification for each convention I chose to break?
- Is every context-sensitive action predictable — can the player state what a button will do before pressing it — with no fundamentally different verbs mapped to the same input in adjacent contexts?
- Is full, independent remapping available for every action, including secondary bindings and no hard-coded inputs, meeting accessibility baselines?
- Have I explicitly defined and tested the resolution order for every input conflict and simultaneous-press scenario, with no undefined behavior under realistic load?
- Have I designed and validated a distinct scheme for every supported platform's native hardware, rather than porting one platform's logic onto another's constraints?
- Did I validate the default scheme with hands other than the designer's, including players unfamiliar with the game, to surface friction the creator can no longer perceive?
