---
name: wearable_interface_design.md
description: Use when the agent is designing interfaces for smartwatches, fitness trackers, rings, smart glasses, hearables, or other wearables, and must handle tiny screens, wrist posture, always-on display, battery limits, sensor-driven UI, and the constraints of body-worn devices.
---

# Wearable Interface Design

Wearables are worn, not held. That single fact changes almost every design assumption inherited from phone design. The screen, if there is one, is tiny and often partially obscured by the user's own wrist posture. Attention is measured in fractions of a second, not minutes. Battery, thermal, and always-on constraints limit what can be shown and how often it updates. And the device sits against the body, which means comfort, motion, and sensor reliability all shape what the interface can responsibly do.

The judgment problem is that wearables are constantly tempting to over-design. It is easy to imagine a rich, phone-like experience on the wrist and very hard to deliver one that does not drain the battery, overwhelm the user, or demand more attention than the wrist can give. The agent's job is to design for the wearable as a distinct medium with its own physics, not as a miniature phone. The strongest wearable interfaces do less, more reliably, and at exactly the right moment.

Use this skill before designing or reviewing smartwatch apps, fitness tracker screens, smart glasses overlays, hearable feedback, ring or band interactions, complication systems, or sensor-driven wearable features. The goal is to prevent the agent from shipping wearable UI that is too dense, too demanding, too battery-hungry, or too reliant on interactions the wrist and the body cannot comfortably sustain.

## Core Rules

### Design For The Wrist And The Glance, Not The Screen

A wearable is viewed in brief glances while the user is doing something else: walking, exercising, talking, carrying a bag. The interface must be legible and useful in the first moment, often without the user fully raising or focusing on the device.

Design for glances by:

- leading with the single most important value or status;
- using large, high-contrast type that reads in motion and outdoors;
- minimizing the number of taps or rotations needed to reach the answer;
- avoiding dense lists, small charts, and walls of text;
- making the current state obvious without animation or exploration.

If the user must stop, raise the wrist fully, scroll, and read to understand the screen, the design has already failed for the most common use. Treat the glance as the primary surface and everything deeper as optional.

### Respect Wrist Posture And Reachability

The wrist is not a stable, centered surface. Users view wearables at angles, often partially, while the arm is in motion or occupied. Reachability depends on which hand wears the device, the user's dexterity, and what else the hand is doing.

Account for posture by:

- placing primary content and actions where they are visible at a natural glance angle;
- avoiding controls in screen regions the thumb cannot comfortably reach one-handed;
- supporting both wrist orientations and dominant hands;
- not requiring precise taps on tiny targets during movement;
- keeping destructive actions away from easy accidental contact.

A button that is technically on screen but unreachable by the operating thumb is not really there. Design the touchable region around real wrist anatomy.

### Treat Battery, Thermal, And Always-On Display As Design Constraints

Wearables live or die by battery and thermal limits, and the interface directly drives both. Bright, frequently updating, always-on content drains power and heats the device against the skin. These are not engineering afterthoughts; they are design constraints that shape what can be shown.

Design within power and thermal limits by:

- minimizing refresh frequency for non-critical information;
- using dim, low-power always-on states that show only the essentials;
- avoiding continuous bright animation, video, or live backgrounds;
- batching sensor updates rather than streaming constantly;
- letting the screen sleep or dim aggressively when not glanced;
- considering heat against the skin for sustained workloads.

A beautiful always-on animation that halves battery life and warms the wrist will be disabled by users or by the system. Design the low-power state as carefully as the active state.

### Match Information To The Body-Worn Context

A wearable knows things a phone does not: heart rate, movement, orientation, proximity, sleep, and other body signals. This is its reason to exist, but it also means the interface can surface intimate, real-time information that must be handled with care and relevance.

Use body context responsibly by:

- surfacing sensor data only when it is actionable or meaningful;
- avoiding alarming the user with noisy or imprecise readings;
- providing context for health and fitness numbers, not bare values;
- respecting privacy for sensitive data visible on the wrist to others;
- defining clear thresholds before prompting, so the user is not nagged.

A heart-rate spike during a horror film is not a medical event. A wearable that treats every sensor wiggle as an alert trains the user to ignore all alerts. Calibrate the relationship between signal and prompt.

### Keep Interactions Short, Recoverable, And Forgiving

Wearable interactions happen in motion, in poor lighting, with wet or gloved hands, and under time pressure. Precision is low and mistakes are common. Interactions must be short, forgiving, and easy to recover from.

Make interactions robust by:

- keeping flows to one or two steps wherever possible;
- using large, forgiving tap targets and simple gestures;
- allowing easy undo for accidental actions;
- avoiding long text entry on the device; defer to phone or voice;
- preserving state so an interrupted flow can resume;
- never trapping the user in a flow they cannot escape.

If a wearable flow requires typing, precise dragging, or many sequential taps, it belongs on the phone. Use the wearable for what it does well and hand off the rest.

### Coordinate With The Companion Device

Most wearables are paired with a phone, and the two form a system. The wearable is best for glanceable, immediate, body-aware moments; the phone is best for depth, configuration, history, and text. Design the division deliberately.

Coordinate by:

- making clear which device owns which part of the task;
- allowing the wearable to defer long or complex actions to the phone;
- syncing state instantly so the user is never confused about what is current;
- surfacing on the wrist only what the wrist does well;
- handling the case where the phone is absent, offline, or out of battery.

Do not duplicate the entire phone experience on the wrist. Do not force the user to pull out the phone for things the wrist could handle. Find the boundary that plays to each device's strength.

### Design For Interruption And Ambient Awareness

Wearables interrupt the body directly, through haptics against the skin and sounds near the ear. This intimacy is powerful and easily abused. Every notification is a physical sensation the user cannot ignore.

Treat interruption as a scarce resource by:

- limiting the number and frequency of haptic and audio alerts;
- differentiating alert types so the user can learn what matters;
- allowing the user to mute, focus, or sleep without losing critical alerts;
- reserving strong patterns for genuinely urgent events;
- never using attention-grabbing patterns for marketing or low-value prompts.

A wearable that buzzes constantly becomes a wearable the user silences or removes. Protect the integrity of the interruption channel.

## Common Traps

### Shrinking A Phone App Onto The Wrist

Porting a phone interface directly produces dense, unreadable, tap-heavy screens that drain battery and frustrate users. Redesign for the wrist as a distinct medium.

### Too Much Information Per Glance

Packing multiple metrics, charts, and actions into one screen defeats the glance. Lead with one value and let the rest be optional.

### Ignoring Always-On Battery Cost

Bright, frequently updating always-on content drains battery and heats the skin. Design the dim, low-power state as a first-class surface.

### Tiny Targets And Long Flows

Precise taps and multi-step flows fail in motion and under time pressure. Keep interactions short, forgiving, and easy to undo.

### Over-Alerting From Noisy Sensors

Treating every sensor fluctuation as an alert trains users to ignore all alerts. Calibrate thresholds and reserve strong patterns for real urgency.

### Forgetting The Companion Boundary

Duplicating the phone on the wrist, or forcing the phone for wrist-suitable tasks, wastes both devices. Define the division of labor deliberately.

### Sensitive Data Visible To Bystanders

Health, financial, or personal information shown large on the wrist is visible to anyone nearby. Consider privacy in what the glance reveals.

## Self-Check

- [ ] The primary surface is a glance that delivers the most important value or status in the first moment without scrolling or exploration.
- [ ] Type, contrast, and layout remain legible in motion, outdoors, and at natural glance angles.
- [ ] Primary content and actions sit within the comfortably reachable and visible region for real wrist posture and either hand.
- [ ] Always-on and active states are both designed, with low-power dim states that preserve battery and avoid heating the skin.
- [ ] Sensor and body data is surfaced only when actionable or meaningful, with context, calibrated thresholds, and privacy for sensitive values.
- [ ] Interactions are short, forgiving, recoverable, and avoid text entry or precise dragging on the device.
- [ ] The division of labor between wearable and companion phone is deliberate, with instant sync and graceful behavior when the phone is absent.
- [ ] Haptic and audio interruptions are scarce, differentiated, mutable, and reserved for genuinely urgent events.
- [ ] Destructive actions are separated from accidental contact and are easy to undo.
- [ ] The design treats the wearable as a distinct body-worn medium, not as a miniature phone.
