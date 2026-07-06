---
name: middleware-and-implementation.md
description: Use when the agent is implementing game music in middleware such as Wwise or FMOD, setting up interactive music systems, configuring events and parameters, working with audio programmers, or preparing adaptive music assets for a game engine.
---

# Middleware and Implementation

Composing adaptive music is only half the work; the music must be implemented in middleware so the game can trigger and control it. Agents advising on game music often focus on composition while treating implementation as someone else's problem. But the way music is structured for implementation determines whether it works interactively, and composers who understand middleware write music that implements cleanly, while those who do not produce assets that fight the system. This skill covers the judgment needed to structure and implement music for interactive systems.

## Core Rules

### Structure Music Assets for the Implementation System

Middleware such as Wwise or FMOD organizes music into segments, layers, and playlists that the game controls. Recommend structuring the composed music to match this organization from the start: defining clear segments for each game state, separating layers for vertical mixing, and naming and organizing files consistently, because music structured without regard for implementation requires costly rework and produces a fragile system.

### Understand the Target Middleware Before Composing

Each middleware system has specific capabilities and constraints for adaptive music: how it handles transitions, what sync options exist, how layers interact, and what file formats are required. Recommend learning the target system's capabilities before composing, because music written without understanding the implementation system may use techniques the system cannot support, forcing compromises or rewrites.

### Define the Game Parameters That Drive the Music

The music responds to game parameters, such as intensity, location, combat state, or player health, and these must be defined and mapped to musical changes. Recommend working with the audio programmer to define the parameters, their ranges, and how they map to musical states early, because unclear or changing parameter definitions force reimplementation and destabilize the music system.

### Plan Transition Behaviors and Sync Points

Transitions between musical states can happen at various sync points, immediately, at the next bar, at the next phrase, or at a custom marker, and the choice affects how smooth the transition feels. Recommend planning the transition behavior for each state change, specifying the sync point and any crossfade, because default transition settings often produce abrupt or mistimed changes that the player notices.

### Budget Memory and Streaming Considerations

Game audio operates under memory and streaming constraints: too many simultaneous layers or too much music loaded at once exceeds budgets and causes performance problems. Recommend considering the memory footprint of the music system, using streaming for longer pieces and memory for short interactive elements, and limiting simultaneous layers, because an over-budget music system causes technical problems that may force cutting content.

### Iterate With the Audio Team Throughout Development

Implementation is not a one-time handoff; it requires iteration as the game evolves. Recommend maintaining communication with the audio programmer and designers, testing changes in context, and revising both music and implementation as the game's needs become clear, because a music system built without ongoing collaboration drifts from the game's actual requirements. Test the music on the target platform early and often, since behavior that works in the authoring tool can perform differently under real memory and streaming pressure in the shipped game.

## Common Traps

### Composing Without Understanding the Middleware Capabilities

This trap arises when composers write music assuming any structure can be implemented. The harm is that the music uses techniques the middleware cannot handle, forcing rewrites or compromises. The false signal is that the music is complete; the result is assets that cannot be implemented as intended, because the composition ignored the system's actual capabilities.

### Disorganized Assets That Make Implementation Fragile

Composers deliver music files without consistent naming, clear segmentation, or logical organization. The trap is that disorganization seems like a minor workflow issue. The harm is an implementation that is hard to maintain, prone to error, and costly to revise, because the implementer cannot reliably identify and route the correct assets.

### Undefined or Shifting Game Parameters

The parameters driving the music are not clearly defined, or they change during development without updating the music system. The trap is that parameter definitions seem like a technical detail. The harm is a music system that responds incorrectly or breaks, because the music was mapped to parameters that no longer match the game's actual state.

### Ignoring Memory and Streaming Budgets

Composers build rich, multi-layered adaptive systems without considering the memory cost. The trap is that more layers feel like better adaptivity. The harm is performance problems, crashes, or forced content cuts, because the music system exceeded the platform's memory or streaming capacity, and the richness that seemed valuable becomes a liability.

## Self-Check

- Did I structure the music assets to match the middleware's organization?
- Did I learn the target middleware's capabilities and constraints before composing?
- Did I define and map the game parameters that drive the music with the audio team?
- Did I plan the transition behaviors and sync points for each state change?
- Did I consider memory and streaming budgets when designing the music system?
- Did I plan to iterate with the audio team throughout development?
