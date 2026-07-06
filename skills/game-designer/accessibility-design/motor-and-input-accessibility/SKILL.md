---
name: motor-and-input-accessibility.md
description: Use when the agent is designing control schemes, remapping inputs, setting input timing windows, building difficulty options for motor-impaired players, choosing assist mechanics, or reviewing whether a game is playable one-handed, with limited reach, or with alternative input devices.
---

# Motor and Input Accessibility

Motor and input accessibility is the part of accessibility work that decides whether a player's body can actually perform the actions the game requires. Designers tend to treat it as a settings menu problem — add remapping and move on — because the surface features (key bindings, sensitivity sliders) are easy to ship and easy to demonstrate. The real judgment problem is deeper: it is about timing, simultaneity, sustained input, precision, and whether the game's core loop is even reachable for someone whose motor control, range of motion, or stamina differs from the assumed default. Agents miss this because the default player is an able-bodied designer who can hold a button while steering with a stick, react in 200 milliseconds, and execute a combo with frame-tight inputs; every system is tuned against that body, and the tuning feels invisible until a real player cannot pass the tutorial. The harm is that the game ships "accessible" on paper but is unplayable in practice for a large share of the audience, and the team believes the problem is solved because the menu exists. The agent has substantial freedom in how it implements these features, but the default posture must be that motor accessibility is a design constraint on the core loop, not a checkbox added at the end.

## Core Rules

### Make Full Input Remapping the Baseline, Not a Feature

Every actionable input must be remappable by the player, including modifiers, holds, and analog inputs, and remapping must not silently collide with other bindings or break context-sensitive prompts. This is non-negotiable because players using one-handed controllers, mouth sticks, or switch devices construct entirely custom layouts that the designer cannot predict. The trap is shipping "remapping" that only covers a subset of actions, or that hardcodes certain actions to specific buttons because the tutorial text references them. Decision criterion: if any action cannot be moved to any physically available input, the remapping is incomplete and the game is excluding players. Treat controller glyphs in UI as resolved from the binding, never hardcoded.

### Eliminate Mandatory Simultaneous and Sustained Inputs From the Core Loop

Holding a button to sprint while steering, holding a charge while aiming, or pressing two buttons at once for a special action all assume independent, simultaneous, sustained motor control. For many players this is physically impossible or fatiguing within minutes. The judgment is not to ban these mechanics but to provide equivalent alternatives: toggle options for holds, sequential inputs that substitute for simultaneous ones, and assist modes that automate one axis so the player can focus on the other. When the core fantasy of the game depends on simultaneity (e.g., a twin-stick shooter), provide an aim-assist or auto-fire option that reduces the degrees of freedom the player must control at once.

### Loosen or Make Adjustable Every Timing-Based Failure

Quick-time events, parry windows, dodge frames, and reaction gates are the most common motor-accessibility walls. A window that feels generous to a designer with 200 ms reaction time is a brick wall to a player with 500 ms or with tremor. Every timing-based failure must either be generous by default, scale with a difficulty or assist setting, or be skippable. Decision criterion: if a single timing gate blocks all progress for a player who understands the game, that gate is an accessibility failure regardless of intended challenge. Provide options to widen windows, slow time, or auto-succeed on repeated failure, and ensure these do not disable progression or achievements in a way that punishes the player for needing them.

### Support Alternative and One-Handed Input From the Architecture Up

Switch controllers, adaptive controllers, eye trackers, foot pedals, and one-handed layouts are not edge cases to bolt on late; they require that the input system treat inputs as abstract and combinable rather than as fixed controller regions. The decision criterion is architectural: can two inputs be mapped to one physical button via a co-pilot or chord, can one input drive multiple game actions, and does the game function when inputs arrive slowly or intermittently? If the answer requires rearchitecting late in development, the team waited too long. Build the abstraction before content depends on it.

### Provide Aim, Steering, and Movement Assists as Toggles

Auto-aim, aim-assist strength, steering assist, movement snapping, and analog-to-digital input conversion should all be exposed as player-facing toggles or sliders, not buried as hidden difficulty behavior. The judgment is that these assists trade designer-intended skill expression for access, and that tradeoff belongs to the player, not the designer. When a player enables full aim-assist, the game should still be completeable and the ending should not be gated behind disabling it. The trap is hiding assists behind "easy mode" so that players who need them are also forced into a degraded experience in every other dimension.

### Never Lock Progression, Content, or Endings Behind Motor Skill

If a secret area, ending, achievement, or piece of content requires a precise input that a motor-impaired player cannot perform, that content is gated by disability, not by choice. This is the hardest rule because it conflicts with the designer's desire to reward mastery. Decision criterion: any content that is unreachable without a specific motor feat must have an alternative path, an assist that preserves the route, or a skip. Rewarding skill is fine; requiring a specific body to see the game's content is not.

## Common Traps

### Treating Remapping as the Whole Solution

A team ships full remapping, declares the game accessible, and never examines whether the remapped layout is actually usable. The trap is that remapping solves binding but not the underlying demands of simultaneity, timing, and precision; a player who remaps sprint to a reachable button still cannot hold it for ten minutes if the mechanic is exhausting. The false signal is the presence of the menu; the harm is that the deeper motor demands go unexamined and the game remains unplayable despite the checkbox.

### Designing Timing Windows to the Designer's Own Body

Designers tune parry windows and reaction gates against their own trained reflexes, then ship those windows as fixed. The trap is that the designer's body is the worst possible calibration target because they are expert, practiced, and biased toward the challenge feeling "right." The false signal is that the window feels fair in internal playtests populated by the same experts; the harm is that the shipped window excludes everyone slower, and the team never sees the exclusion because their testers are too good.

### Hardcoding Controls in Tutorials and Prompts

The tutorial text says "press X to jump," the prompt renders a fixed glyph, and the moment a player remaps jump to another button the tutorial lies. The trap is that the input system and the UI system are decoupled, so remapping works mechanically but the game's teaching layer breaks. The false signal is that remapping "works" in the options test; the harm is a confusing, hostile onboarding for exactly the players who most need clear instruction.

### Confusing Difficulty With Motor Demand

A designer adds an "easy mode" that reduces enemy health and damage but leaves frame-tight dodge windows intact, believing they have addressed accessibility. The trap is conflating cognitive/numerical difficulty with motor demand; they are independent axes, and lowering one does not lower the other. The false signal is that easy mode exists and is labeled accessibility-friendly; the harm is that motor-impaired players hit the same wall on easy as on hard, and the team believes they were accommodated.

### Punishing Assist Use With Degraded Experience

The game offers auto-aim or auto-dodge but disables achievements, locks the true ending, or flags the save as "assisted," signaling that the player did not really play the game. The trap is that the assist is framed as a concession rather than a legitimate way to play, exploiting the social signal of completion to shame players out of using what they need. The harm is that players who need assists either suffer the stigma or forgo the assist and fail, and either way the game has not actually granted access.

## Self-Check

- Does every actionable input, including holds, modifiers, and analog actions, remap to any physically available input without collision or broken tutorial prompts?
- Have I audited the core loop for mandatory simultaneous or sustained inputs, and provided toggle, sequential, or assist alternatives for each?
- Is every timing-based failure (parries, QTEs, reaction gates) either generous by default, adjustable via a setting, or skippable, with no single gate able to block all progression?
- Does the input architecture support alternative devices, co-pilot/chord mapping, and slow or intermittent input without requiring late rework?
- Are aim, steering, and movement assists exposed as player-facing toggles that preserve full progression, rather than hidden behind a degraded easy mode?
- Is any content, ending, or secret reachable only through a specific motor feat provided with an alternative path or skip?
- Did I verify, with actual motor-impaired players or representative input profiles, rather than assuming the menu is sufficient?
