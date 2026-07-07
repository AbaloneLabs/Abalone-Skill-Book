---
name: spatial_interface_design.md
description: Use when the agent is designing spatial interfaces, augmented or mixed reality surfaces, 3D menus, world-anchored UI, room-scale layouts, hand-tracked controls, or any interface that lives in physical space rather than on a flat screen.
---

# Spatial Interface Design

Spatial interfaces live in physical space. Content can be placed at any depth, in any direction, anchored to the real world or floating freely, and reached by gaze, gesture, voice, or controller. That freedom is the core judgment problem. A flat screen constrains the designer in helpful ways: there is one plane, a known edge, and a stable relationship between the user and the content. Spatial computing removes those constraints, and with them removes the safety rails that keep flat-screen design coherent.

The risk is that spatial interfaces become unanchored, overwhelming, or unreachable. Content placed behind the user is invisible. Menus at the wrong depth cause eye strain. World-locked panels collide with furniture. Gestures that feel natural in a demo fail in a cluttered room. The agent's job is to design spatial surfaces that use the extra dimension deliberately rather than carelessly, so that depth, position, and physical context support the task instead of fighting it.

Use this skill before designing or reviewing augmented reality, mixed reality, virtual reality, spatial operating systems, 3D dashboards, world-anchored annotations, hand-tracked tools, or room-scale experiences. The goal is to prevent the agent from shipping spatial UI that is disorienting, physically uncomfortable, inaccessible, or impossible to use in a real environment.

## Core Rules

### Decide What Is Body-Locked, Head-Locked, And World-Locked

The most fundamental spatial decision is how content relates to the user and the room. Each option has distinct tradeoffs and the wrong choice is the most common cause of spatial discomfort.

Body-locked (follows the user's position):

- always available regardless of where the user looks or moves;
- good for persistent tools, navigation, and status;
- risk of feeling cluttered or always in the way.

Head-locked (follows the user's gaze):

- stays in view without the user turning;
- good for transient alerts and brief prompts;
- causes discomfort and disorientation if overused or held too long.

World-locked (anchored to physical space):

- stays put when the user moves, encouraging exploration and spatial memory;
- good for content tied to real locations, multi-panel work, and persistent rooms;
- risk of being out of view, occluded, or unreachable.

Match the lock mode to the content's purpose. Persistent, task-critical tools often belong body-locked. Brief notifications may be head-locked for a moment. Rich, explorable, or location-meaningful content belongs world-locked. Never head-lock dense or long-lived content.

### Place Content Within Comfortable Reach And View

Spatial interfaces have a comfortable volume, and designs that ignore it cause fatigue, eye strain, and unreachable controls. Define where content should live relative to the user.

Consider:

- primary content within the central field of view and at arm's reach or slightly beyond;
- avoid forcing users to hold their arms raised for long periods;
- avoid placing dense content too close, which causes vergence-accommodation strain;
- avoid placing critical content far behind or far above the user;
- keep interactive targets within the reachable interaction volume for the chosen input method.

Comfort is an accessibility requirement, not a nicety. Designs that require sustained awkward posture exclude users with limited mobility, stamina, or range of motion, and tire everyone over time.

### Use Depth To Convey Meaning, Not Decoration

Depth is information in a spatial interface. Objects closer to the user feel more immediate, more important, and more actionable. Depth used randomly creates confusion; depth used deliberately creates hierarchy.

Use depth to express:

- focus and foreground versus reference and background;
- the active object versus available alternatives;
- a modal or summoned surface versus the base environment;
- physical plausibility, so objects rest on surfaces rather than float arbitrarily.

Avoid:

- stacking many layers of content at similar depths, which causes visual noise;
- using depth only for visual flair without informational meaning;
- placing interactive elements at depths that contradict their importance.

When everything has depth, nothing has priority. Reserve spatial layering for moments where the depth carries meaning.

### Respect The Real Environment And Its Constraints

Spatial interfaces share space with the real world, and the real world is messy. Furniture, other people, changing light, reflective surfaces, and limited tracking all affect whether a design works in an actual room.

Design for real environments by:

- avoiding placement assumptions that require a large empty room;
- handling occlusion when a person or object passes between user and content;
- providing fallbacks when surface detection or anchoring fails;
- accounting for low light, glare, and reflective surfaces that break tracking;
- not assuming the user can or will move furniture;
- considering shared spaces where others may walk through the interface.

A design that works only in an idealized empty studio fails in the living room where people actually use it. Test against cluttered, small, and shared spaces, not just clean demos.

### Match Input Method To Precision And Fatigue

Spatial input ranges from precise controllers to indirect gaze to freehand tracking to voice. Each has different precision, learning cost, and fatigue profiles, and each suits different tasks.

Controller or pointer input:

- high precision, low fatigue, reliable;
- good for dense selection, text, and fine manipulation;
- requires the user to hold or find a device.

Direct hand tracking:

- intuitive and device-free;
- lower precision, higher arm fatigue;
- poor for long sessions or fine work without support.

Gaze and pinch or dwell:

- low physical effort, hands-free;
- lower precision and slower for dense targets;
- can feel passive or imprecise for expert users.

Voice:

- excellent for commands, search, and discrete actions;
- poor for precise spatial positioning and private or noisy environments;
- should complement, not replace, spatial selection.

Choose input by task. Do not force a single method onto every interaction, and always provide a reliable fallback for users who cannot use the primary method.

### Preserve Spatial Memory And Orientation

Spatial interfaces are powerful because users can remember where things are in space. That advantage is destroyed if content moves unpredictably. Preserve orientation so users can build habits.

Preserve spatial memory by:

- keeping world-locked content stable across sessions when it makes sense;
- returning panels and tools to where the user left them;
- avoiding reflow that silently relocates content;
- providing landmarks or environmental anchors that orient the user;
- signaling clearly when content is being repositioned and why.

When the user looks away and back, the world should be where they left it, or the change should be obvious and intentional.

### Design For Shared, Multi-User, And Spectator Contexts

Spatial experiences are often social. The same content may be seen by a wearer, a spectator on a flat screen, or multiple users in the same room. Each audience needs consideration.

Consider:

- what spectators see when someone wears a headset and interacts;
- how multiple wearers share or conflict over the same anchored content;
- permission and ownership for world-locked objects others can affect;
- how a user joins or leaves a shared spatial session;
- how to align each participant's coordinate space so shared content lands consistently.

Do not assume a single isolated user. Design the social and ownership model early, because retrofitting it into a spatial system is painful.

## Common Traps

### Head-Locking Dense Content

Fixing panels to the user's view feels safe in a demo but causes nausea, clutter, and disorientation in real use. Reserve head-locking for brief, transient content.

### Placing Content Behind Or Above The User

Content the user cannot see without searching feels missing. Critical content belongs in the central, comfortable volume, not scattered around the room.

### Ignoring Arm Fatigue From Sustained Gestures

Freehand interaction that requires raised arms for minutes at a time is unusable in practice. Design for brief gestures, resting positions, and low-effort alternatives.

### Assuming An Empty, Well-Lit Room

Tracking and placement that depend on ideal conditions fail in real homes. Design fallbacks for clutter, occlusion, low light, and small spaces.

### Using Depth As Decoration

Parallax and floating layers that carry no meaning add visual noise and depth confusion. Every layer should justify its position in space.

### Relocating Content Without Signaling

When world-locked panels silently move, users lose trust in their spatial memory. Stability is a feature; changes must be deliberate and visible.

### Forcing One Input Method Onto Every Task

Hand tracking is not always best, and controllers are not always available. Tasks have different precision and fatigue needs, and users have different abilities.

## Self-Check

- [ ] Each piece of content has a deliberate lock mode (body, head, or world) matched to its purpose, and dense content is not head-locked.
- [ ] Primary content and interactive targets sit within a comfortable field of view and reachable interaction volume, avoiding sustained awkward posture.
- [ ] Depth is used to convey hierarchy and meaning, not as random decoration, and layers do not create visual noise.
- [ ] The design handles cluttered, small, shared, low-light, and occluded real environments, with fallbacks when tracking or anchoring fails.
- [ ] Input methods are matched to task precision and fatigue, with reliable fallbacks for users who cannot use the primary method.
- [ ] World-locked content is stable across sessions, changes are signaled, and users can build reliable spatial memory.
- [ ] Shared, multi-user, and spectator contexts are designed, including ownership, permissions, join/leave, and coordinate alignment.
- [ ] No critical content is placed where the user must search behind, above, or far from their natural orientation.
- [ ] Brief, transient content is distinguished from persistent content in both lock mode and depth treatment.
- [ ] The experience has been considered in a real room, not only an idealized demo space.
