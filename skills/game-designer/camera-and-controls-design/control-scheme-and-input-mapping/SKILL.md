---
name: control-scheme-and-input-mapping.md
description: Use when the agent is designing a game's control scheme, mapping actions to inputs, deciding between context-sensitive and dedicated controls, handling multi-platform input, or evaluating whether a control scheme is learnable, comfortable, and does not conflict with player expectations or cause input errors.
---

# Control Scheme and Input Mapping

A control scheme is the physical interface between the player's intent and the game's response, and every mapping decision trades learnability against expressiveness against comfort. The judgment problem is that there is never enough input bandwidth for every action to have a dedicated, comfortable button, so actions must share inputs, stack under modifiers, or become context-sensitive — and each of those choices introduces failure modes (mis-presses, forgotten mappings, context errors) that only appear in play. Agents tend to miss this because a control scheme that works in a guided demo hides the conflicts that emerge in tense play, and because the scheme's quality is judged by whether it functions rather than whether it survives the cognitive load of real gameplay. The harm is controls that players fight, actions that misfire under pressure, and a game whose mechanics are sound but whose interface prevents the player from expressing them. This skill covers how to allocate input bandwidth, manage context and conflicts, and design controls that survive real play. The agent has latitude in the specific mappings, but the obligation to make the scheme learnable, comfortable, and error-resistant is not optional.

## Core Rules

### Allocate the Most Accessible Inputs to the Most Frequent Actions

Input real estate is unequal: face buttons and triggers are more accessible than D-pad directions or chord combinations, and the most frequent core actions must occupy the most accessible inputs. The decision rule: rank actions by frequency and criticality, and assign the highest-ranked actions to the most accessible inputs, demoting rare actions to less accessible or modifier-stacked inputs. A scheme that puts a frequent action on an awkward input produces chronic discomfort, because the player performs it thousands of times and each time pays the accessibility tax.

### Prefer Context-Sensitivity for Low-Bandwidth Platforms, Dedicated Inputs for High-Stakes

Context-sensitive controls (one button does different things by situation) solve bandwidth scarcity but introduce context errors; dedicated controls (one action per input) are learnable but demand bandwidth the controller may not have. The decision rule: use context-sensitivity for platforms with limited inputs (mobile, controller-heavy games) where the benefit exceeds the error risk, and use dedicated inputs for high-stakes actions where a context error is catastrophic (combat criticals, life-saving abilities). The choice depends on whether the error cost or the bandwidth cost is higher for each action.

### Avoid Modifier Chords for Time-Critical Actions

Actions that must be performed under time pressure — dodges, parries, emergency heals — should not require modifier chords (hold a trigger plus press a face button), because chords are slower and more error-prone than single inputs under stress. The decision rule: map time-critical actions to single, accessible inputs, and reserve chords for non-time-critical actions like menus or emotes. A chord-mapped dodge that fails under pressure because the player mis-times the chord is a control scheme killing the player, not the gameplay.

### Ensure the Scheme Matches Platform and Genre Expectations

Players arrive with platform and genre expectations — WASD for PC movement, the right stick for console aiming, specific face button conventions for platformers — and violating them produces immediate friction even if the scheme is internally logical. The decision rule: map the scheme onto established platform and genre conventions where they exist, and diverge only with clear justification and strong onboarding. A scheme that reinvents standard mappings forces the player to unlearn before they learn, and the unlearning tax costs engagement.

### Provide Remapping and Validate the Default Against Diverse Hands

Players have diverse hand sizes, mobilities, and motor needs, and a single default scheme cannot serve all of them. The decision rule: offer full input remapping, design the default scheme for the broadest comfort, and validate the default with testers of diverse hand sizes and motor abilities. A non-remappable scheme that fits the team excludes players it could have served, and remapping is a low-cost accessibility feature with high inclusion return.

### Stress-Test Controls Under Cognitive and Time Load

A control scheme that is comfortable in a calm demo may fail when the player is also tracking threats, managing resources, and reacting to events, because cognitive load degrades input precision. The decision rule: playtest the scheme in the game's most demanding moments — intense combat, time pressure, multi-task situations — and confirm the mappings hold when the player is stressed. Schemes validated only in calm conditions break in the conditions that matter most, because the cognitive load that produces mis-presses was never present in testing.

## Common Traps

### Frequent Action on an Awkward Input

The team maps a frequent core action to an awkward input — a often-used ability on a D-pad direction, a primary interaction on a chord — because the accessible inputs were taken by less frequent actions, and the player pays a comfort tax on every use. The trap is that the mapping is functional. The false signal is that the action is reachable. The harm is chronic discomfort, the player's hand fatigues, the action feels sluggish because the input is slow to reach, and the core loop's feel degrades because a frequent action was demoted to make room for a rare one that did not need the prime real estate.

### Context Errors in High-Stakes Actions

A high-stakes action is context-mapped — the same button revives a teammate in one context and attacks in another — and under pressure the player triggers the wrong context, performing the attack when they meant to revive, with catastrophic consequences. The trap is that context-sensitivity saved a button. The false signal is that the mapping works in each context. The harm is that context errors in high-stakes moments feel like the game betrayed the player's input, the consequence (a death, a failed objective) is attributed to the controls rather than the player, and trust in the control scheme collapses, because a context-mapped action was used where a dedicated input was required.

### The Chord-Mapped Time-Critical Action

A dodge or parry is mapped to a modifier chord, and under time pressure the player mis-times the chord or fails to hold the modifier, and the action does not fire when the player needed it. The trap is that the chord freed a button. The false signal is that the chord is reachable in calm testing. The harm is that the action fails precisely when it matters most, the player dies to a dodge that did not fire, the failure is attributed to the controls, and the combat that depended on reliable time-critical inputs is undermined by a scheme that asked the player to execute a chord faster than the situation allowed.

### Reinventing Standard Mappings Without Justification

The team maps controls in a non-standard way — jump on a non-standard button, aim on the opposite stick — because the internal logic appealed, and players fight the scheme because their muscle memory expects the standard. The trap is that the internal logic is coherent. The false signal is that the scheme is self-consistent. The harm is that every player must unlearn the standard before learning the new, the unlearning tax produces early frustration and churn, and the innovation that felt clever in the studio reads as perversity in the wild, because the standard mappings are standard for reasons the team overrode without compensating justification.

### No Remapping on a Scheme That Excludes Players

The scheme is not remappable and is tuned for the team's hands, excluding players with different hand sizes, mobilities, or motor needs who cannot comfortably reach the default mappings. The trap is that the default works for the team. The false signal is that internal playtests report no comfort issues. The harm is that players who cannot use the default have no recourse, they churn in the first session without articulating that the controls excluded them, and the audience the game could have served is reduced by a missing feature whose cost is low and whose inclusion return is high, because remapping was treated as optional rather than as the accessibility baseline it is.

### Validating Controls Only in Calm Conditions

The team evaluates the control scheme in calm, guided demos where the player is not stressed, and the scheme passes, but in real gameplay's cognitive load the mappings produce mis-presses and errors. The trap is that calm tests are easy to run and review. The false signal is that the controls feel fine in the demo. The harm is that the scheme breaks under the load of real play, the mis-presses cluster in intense moments where they are most costly, and the controls that passed review fail the player in the conditions the demo never reproduced, because cognitive load was not part of the validation.

## Self-Check

- Are the most frequent and critical actions mapped to the most accessible inputs, with rare actions demoted to less accessible or modifier-stacked inputs?
- Have I used context-sensitivity only where the bandwidth benefit exceeds the error risk, and dedicated inputs for high-stakes actions where context errors are catastrophic?
- Are time-critical actions mapped to single accessible inputs, with chords reserved for non-time-critical functions?
- Does the scheme match established platform and genre expectations, diverging only with justification and strong onboarding?
- Is full input remapping available, and was the default validated with testers of diverse hand sizes and motor abilities?
- Did I stress-test the controls under the cognitive and time load of the game's most demanding moments, not only in calm demos?
- Did I confirm the scheme survives real play, where mis-presses cluster, rather than only functioning in guided conditions?
