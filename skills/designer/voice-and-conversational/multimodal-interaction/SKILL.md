---
name: multimodal_interaction.md
description: Use when the agent is designing multimodal interfaces that combine voice, touch, gesture, keyboard, mouse, pen, camera, or gaze input, or coordinating input and output across multiple devices, screens, and modalities including handoff, continuity, and modality switching.
---

# Multimodal Interaction

Multimodal interfaces let users move between voice, touch, gesture, keyboard, mouse, pen, camera, and gaze, sometimes within a single task. The difficulty is not adding a second input method; it is making the methods cooperate, hand off cleanly, and never contradict each other. When modalities are designed in isolation, users hit dead ends: a task started by voice cannot be finished by touch, a gesture conflicts with a button, or the system responds to the wrong input at the wrong moment.

Use this skill before designing products that span multiple input or output methods, cross-device experiences, voice-plus-touch flows, gesture interfaces, camera or gaze input, or continuity between phone, tablet, wearable, and desktop. The goal is to prevent the agent from bolting modalities together without coordination, leaving users trapped when one method fails or when context shifts.

## Core Rules

### Let Modalities Complement, Not Compete

Each input method has strengths. Voice is hands-free but public and error-prone. Touch is direct but occupies the hands. Keyboard and mouse are precise but require attention and a surface. Gesture is expressive but fatiguing and hard to discover. Gaze is fast but imprecise for activation.

Design so each modality handles what it is best at:

- voice for quick commands and hands-free contexts;
- touch for direct manipulation and selection;
- keyboard for precision, speed, and accessibility;
- gesture for spatial and expressive actions;
- gaze for targeting with another modality confirming.

When two modalities fight for the same action, users get confused about which the system will honor.

### Make Modality Switching Seamless Mid-Task

Users switch modalities without announcing it. They start a command by voice, refine it by touch, confirm with a key. The system must carry state across the switch:

- preserve partial input and selection;
- keep context so the new modality continues the same task;
- avoid forcing the user to restart when they change method;
- recognize the same intent expressed through different modalities.

A task that dies when the user switches from voice to touch has failed at multimodality.

### Define Which Modality Leads In Each Context

Context determines the right primary modality. A driver should lead with voice; a designer at a desk should lead with pointer and keyboard; a cook should lead with voice or large touch targets. Decide the default modality per context and make others available as alternatives rather than equals competing for attention.

Document the lead modality for each environment so the design does not default to whatever is easiest to build.

### Handle Conflicting And Simultaneous Input

When multiple modalities are active, inputs can collide. A user speaks while tapping; a gaze target overlaps a touch target; a gesture triggers during a voice command. Define resolution rules:

- which modality takes precedence when inputs overlap;
- whether simultaneous input is combined or ignored;
- how to cancel an in-progress action from another modality;
- how to prevent accidental activation, especially from passive inputs like gaze or motion.

Unresolved conflicts produce unpredictable behavior that users cannot learn.

### Provide Clear Feedback Across All Active Modalities

Feedback must match the input the user just gave. A voice command needs audio or visual confirmation; a touch needs tactile or visual response; a gesture needs immediate spatial feedback. When modalities are mixed, feedback should be consistent in meaning even if different in form, so the user always knows whether their input registered.

Avoid feedback that only works for one modality, leaving users of another unsure whether the system responded.

### Design Graceful Fallback When A Modality Fails

Every modality fails sometimes: voice in noise, touch with wet hands, camera in low light, gesture when fatigued, gaze with glasses. A multimodal product must degrade gracefully:

- offer an alternative modality for the same action;
- never trap the user in a flow that requires the failed modality;
- signal when a modality is unavailable and how to proceed;
- preserve the task so the user can continue another way.

A product that hard-requires one modality is not truly multimodal.

### Coordinate Across Devices And Screens

Multimodal often means multi-device: phone, tablet, watch, laptop, smart speaker, TV. Design continuity:

- hand off active tasks between devices;
- sync state so a task started on one finishes on another;
- let each device lead with its strongest modality;
- handle disconnects and reconnection without losing progress.

Users expect to move between devices without thinking; the system must track the task, not the device.

### Respect Privacy And Social Context Per Modality

Modalities differ in privacy. Voice and camera are public and observable; touch and keyboard are private; gaze can reveal attention. Match the modality to the sensitivity of the task:

- avoid voice for private data in shared spaces;
- avoid camera capture without clear consent;
- let users choose a discreet modality for sensitive input.

Forcing a public modality for a private task is a privacy failure.

### Make Modalities Discoverable Without Overwhelming

Users cannot use a modality they do not know exists. Surface available input methods contextually, through hints, onboarding, and progressive disclosure, without cluttering the interface with every option at once. Show the relevant alternatives for the current task and context.

### Ensure Accessibility Across Modality Combinations

Multimodal designs can help or harm accessibility. A user who cannot speak needs non-voice paths; a user who cannot see needs audio and touch; a user with motor differences needs alternatives to gesture and gaze. Never make a feature reachable through only one modality. Provide equivalents so multimodality expands access rather than narrowing it.

## Common Traps

### Bolting Modalities Together Without Coordination

Adding voice to a touch app without shared state produces two interfaces that cannot finish each other's tasks.

### Trapping Users In One Modality

A flow that requires voice end-to-end fails the moment voice is unavailable, with no fallback.

### Conflicting Inputs With No Resolution Rule

When touch, voice, and gaze can all trigger, undefined precedence makes behavior unpredictable.

### Feedback Mismatched To Input

Visual-only feedback leaves voice users unsure; audio-only feedback leaves deaf users out. Feedback must span active modalities.

### Forcing A Public Modality For Private Tasks

Requiring voice or camera for sensitive data ignores the social context of use.

### Assuming A Single Device

Designing per device without handoff breaks the moment a user moves from phone to laptop mid-task.

### Discoverability Failure

Hidden modalities that users never learn about add complexity without value.

### Single-Modality-Only Features

Features reachable only through gesture or only through voice exclude users who cannot use that modality.

## Self-Check

- [ ] Each modality is assigned to the actions it handles best, and competing modalities for the same action have clear precedence.
- [ ] Users can switch modalities mid-task without losing state, selection, or context.
- [ ] A lead modality is defined for each usage context, with alternatives available rather than competing equally.
- [ ] Simultaneous and conflicting inputs have explicit resolution and cancellation rules.
- [ ] Feedback is provided in a form appropriate to each active modality and is consistent in meaning across them.
- [ ] Every modality-dependent action has a fallback path when that modality is unavailable.
- [ ] Tasks hand off and sync across devices, with each device leading with its strongest modality.
- [ ] Private or sensitive tasks offer discreet modalities and avoid forcing public input like voice or camera.
- [ ] Available modalities are discoverable through contextual hints without overwhelming the interface.
- [ ] No feature is reachable through only one modality; equivalents exist for users who cannot use a given input method.
