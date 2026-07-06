---
name: mobile-controls-and-touch-ux.md
description: Use when the agent is designing mobile touch controls and UX, building touch input schemes, optimizing for one-handed and portrait play, or evaluating whether mobile controls feel precise and comfortable or feel imprecise, cramped, and fatiguing, with occlusion and reach problems that undermine the experience on small touchscreens.
---

# Mobile Controls and Touch UX

Touch controls are the primary input for mobile games, and they are also one of the most difficult input designs to get right, because touch lacks the tactile feedback and precision of physical controls, the player's fingers occlude the screen, and the range of devices (screen sizes, one-handed use) is vast. The judgment problem is that touch controls must be precise (despite touch's imprecision), comfortable (despite extended holding), and adaptive (across devices and grips), and agents tend to miss this because controls that work in the lab (two-handed, focused testing) fail in the real world (one-handed, on the bus), and because the occlusion and reach problems are invisible until played on a real device in a real context. The harm is controls that fight the player's input, that fatigue the hands, that are occluded by the player's own fingers, or that cannot reach across the screen. This skill covers how to design touch controls that are precise, comfortable, and adaptive, and avoid the occlusion, fatigue, and reach traps. The agent has latitude in the controls, but the obligation to make touch input serve the player is not optional.

## Core Rules

### Design Touch Controls for Precision Despite Touch Imprecision

Touch input is inherently imprecise (no tactile feedback, large contact area), and controls must be designed to compensate — large tap targets, generous hit boxes, input assistance — so the controls feel precise despite touch's imprecision, because precise-feeling controls on an imprecise input require design compensation. The decision rule: design controls that compensate for touch imprecision (large targets, generous hit boxes, assist), and avoid controls that demand precision touch cannot provide. Precision-demanding controls frustrate, because the input could not meet the demand.

### Account for Finger Occlusion of the Screen

The player's fingers occlude the screen (covering content and action with the controlling fingers), and the control layout must account for this — placing controls where occlusion is acceptable, ensuring critical information is not under the fingers — because occluded content or action harms the experience. The decision rule: map the control layout to account for occlusion (controls where occlusion is fine, critical info not under fingers), and avoid layouts that occlude important content. Occluding layouts hide content, because the fingers covered what the player needed to see.

### Optimize for Comfort and Avoid Hand Fatigue

Mobile games are often played for extended periods while holding the device, and the controls must be optimized for comfort — reachable controls, relaxed grips, minimal strain — because fatiguing controls produce discomfort that ends sessions and drives churn. The decision rule: optimize controls for comfort (reachable positions, relaxed grips, minimal strain), and avoid layouts that fatigue the hands. Fatiguing controls end sessions, because the discomfort exceeded the player's tolerance.

### Support One-Handed and Portrait Play Where Appropriate

Many mobile players play one-handed (on the go, holding with one hand) or in portrait orientation, and the controls should support these modes where appropriate, because controls that require two hands or landscape exclude the one-handed or portrait player. The decision rule: support one-handed and portrait play where the game allows, and avoid requiring two-handed or landscape where one-handed could work. Two-handed-required controls exclude one-handed players, because the controls could not be used in the one-handed context.

### Provide Adaptive Control Sizing and Layout Across Devices

Mobile devices range widely in screen size, and the controls must adapt — scaling sizing and layout to the device — so the controls are usable on a small phone and a large tablet, because fixed controls that fit one device are wrong on others. The decision rule: make controls adaptive to screen size (scaling, layout adjustment), test across device range, and avoid fixed controls. Fixed controls fail across devices, because the sizing and layout were not adaptive.

### Use Gestures and Multi-Touch Intentionally, Not Excessively

Gestures and multi-touch can enrich mobile input, but they must be used intentionally (where they serve the experience) not excessively (where they add complexity without value), because excessive or unintuitive gestures burden the player's memory and execution. The decision rule: use gestures and multi-touch where they serve the experience, ensure they are intuitive and discoverable, and avoid excessive or unintuitive gestures. Excessive gestures burden the player, because the complexity exceeded the value.

## Common Traps

### Precision-Demanding Controls on Imprecise Touch

The team designs controls that demand precision touch cannot provide (tiny targets, exact taps), and the controls feel imprecise and frustrating. The trap is that the controls are precise on a stylus. The false signal is that the controls work in testing. The harm is that the touch input cannot meet the precision demand, the controls feel unresponsive and frustrating, the player's inputs do not register as intended, the experience is fighting the input, and the player churns from the imprecise controls, because the design did not compensate for touch imprecision.

### Occluding Layouts Hiding Critical Content

The team places controls without accounting for finger occlusion, and the player's fingers cover critical content or action. The trap is that the controls are placed. The false signal is that the controls are accessible. The harm is that the controlling fingers occlude the content the player needs to see, the action is hidden under the fingers, the player cannot perceive what is happening in the occluded area, and the experience is harmed by the layout, because the occlusion was not accounted for.

### Fatiguing Controls Ending Sessions

The team designs controls that require strain (stretched reaches, tense grips, constant holding), and the controls fatigue the hands and end sessions. The trap is that the controls are usable. The false signal is that the controls function. The harm is that the fatiguing controls produce discomfort, the discomfort accumulates over the session, the player ends the session to relieve the fatigue, the session length (and engagement) is limited by the control design, and the player churns from the discomfort, because the controls were not comfortable.

### Two-Handed or Landscape Requirements Excluding Players

The team designs controls that require two hands or landscape orientation, and the one-handed or portrait player is excluded. The trap is that two-handed controls are more capable. The false signal is that the controls are full-featured. The harm is that the one-handed player (on the go, holding with one hand) cannot use the controls, the portrait player is forced into landscape, the players whose context requires one-handed or portrait are excluded, and the game loses the audience whose context the controls do not support, because one-handed and portrait were not supported.

### Fixed Controls Failing Across Devices

The team designs fixed controls that fit one device size, and the controls are wrong on other devices. The trap is that the controls fit the test device. The false signal is that the controls work. The harm is that the fixed controls are too small on large tablets or too large on small phones, the controls are wrong across the device range, the players on mismatched devices have a poor control experience, and the game fails on devices the controls were not designed for, because the controls were not adaptive.

### Excessive or Unintuitive Gestures Burdening Players

The team uses excessive or unintuitive gestures, and the player is burdened by the complexity. The trap is that gestures feel mobile-native. The false signal is that the controls are rich. The harm is that the excessive gestures burden the player's memory and execution, the unintuitive gestures are not discoverable, the player struggles to remember and perform the gestures, the complexity exceeds the value, and the controls are resented, because the gestures were not intentional.

## Self-Check

- Do controls compensate for touch imprecision (large targets, generous hit boxes, assist) rather than demanding precision?
- Does the control layout account for finger occlusion, keeping critical content visible?
- Are controls optimized for comfort, avoiding hand fatigue from stretched reaches or tense grips?
- Are one-handed and portrait play supported where appropriate, not requiring two-handed or landscape?
- Are controls adaptive to screen size, tested across the device range, not fixed to one size?
- Are gestures and multi-touch used intentionally and intuitively, not excessively or unintuitively?
- Did I confirm the touch controls feel precise, comfortable, and adaptive rather than imprecise, cramped, and fatiguing?
