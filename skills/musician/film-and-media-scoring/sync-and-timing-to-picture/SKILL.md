---
name: sync_and_timing_to_picture.md
description: Use when the agent is synchronizing music to film or video, writing to hit points or timecode, deciding on tempo and meter for a cue, using click tracks or tempo maps, conforming a cue to a picture edit change, or solving problems where music must align precisely with on-screen events.
---

# Sync and Timing to Picture

Film music must align with the picture — entering on a specific frame, hitting a dramatic moment exactly, and exiting cleanly — and this alignment is engineered, not left to feel. The judgment problem is that music has its own time (tempo and meter) and film has its own time (frames and seconds), and reconciling them requires mathematical and musical planning. A composer who writes freely and hopes it "lands" will miss hit points; a composer who plans tempo, meter, and barline placement against timecode can hit any frame precisely while keeping the music natural. The agent must master tempo calculation, click tracks, tempo maps, and the art of making engineered sync sound organic.

## Core Rules

### Identify Hit Points and Engineer the Cue Around Them

A hit point is a specific frame where the music must align with an on-screen event — a door opening, a punch landing, a reveal. Before composing, identify all hit points in the cue and their timecode. The cue's tempo, meter, and phrase structure are then chosen so that barlines and strong beats fall on or near the hit points. This is often a process of working backward: if a hit is at a known timecode, what tempo places a downbeat there? There are standard techniques (tempo calculation from timecode, metric modulation, barline insertion and deletion) for solving this. Engineering the cue around hit points is the foundation of film scoring craft.

**Decision criteria:** Are all hit points in the cue identified with frame-accurate timecode? Has the tempo and meter been chosen so that strong beats or barlines align with the hits? Is the alignment verified mathematically, not by feel?

### Use Tempo Maps and Click Tracks to Maintain Sync

A tempo map is a document (in the DAW or notation software) that defines the tempo and meter at every point in the cue, synchronized to the picture's timecode. The click track derived from it gives the performer or sequencer the precise beat to follow. Building an accurate tempo map is essential: it ensures that live players and MIDI sequences stay in sync with the picture from start to finish. The tempo map must account for tempo changes, meter changes, and rallentandi/accelerandi, all calculated to land hit points. Without a tempo map, sync drifts and hit points are missed.

**Decision criteria:** Is there a tempo map for the cue, built in the DAW or notation software, synchronized to picture timecode? Does the click track derived from it keep live and sequenced elements in sync throughout?

### Choose Tempi That Serve Both the Drama and the Math

A tempo must satisfy two constraints: it must feel right for the scene's emotion and energy, and it must place beats on hit points. Sometimes these conflict — the "right" feel does not land the hit. The resolution involves techniques like choosing a related tempo, using a metric modulation to shift, inserting or deleting a bar, or using a tempo change (rall. or accel.) that is both musically motivated and mathematically precise. The art is finding a solution that serves both constraints without compromise. Avoid tempi that feel wrong just to hit a point; find a musical solution.

**Decision criteria:** Does the chosen tempo feel right for the scene AND land the hit points? If there is a conflict, has a musical technique (modulation, bar adjustment, tempo change) been used to resolve it rather than forcing a wrong-feeling tempo?

### Make Engineered Sync Sound Organic

The danger of click-track scoring is that it sounds mechanical — rigidly tied to the grid in a way that drains musical life. The craft is to engineer the sync while preserving the music's natural ebb and flow. Techniques include using expressive timing within the bar (rubato against the click), choosing meters and phrase lengths that feel natural, hiding tempo changes inside musically motivated moments, and allowing the music to breathe between hit points rather than marching rigidly. The audience should feel that the music fits the picture naturally, not that it was forced onto a grid.

**Decision criteria:** Does the cue sound musical and organic despite being engineered to hit points? Are expressive timing, natural phrase lengths, and hidden tempo changes used to preserve musicality?

### Conform Cues to Picture Edits Quickly and Accurately

When the picture is re-edited after a cue is composed (which happens constantly), the cue must be conformed — adjusted to the new cut. This may involve extending or shortening sections, moving hit points, or recomposing transitions. The faster and more accurately this is done, the more viable the composer is in the production. Conforming requires a clear tempo map (so changes can be calculated), modular composition (so sections can be moved), and a workflow that allows rapid re-timing. A cue that cannot be conformed efficiently becomes a liability when the cut changes.

**Decision criteria:** When the cut changes, can the cue be conformed quickly using the tempo map and modular structure? Is the workflow set up to handle re-timing without recomposing from scratch?

### Account for Frame Rates and Timecode Standards

Sync depends on the frame rate of the picture (24, 23.976, 25, 29.97, 30 fps) and the timecode standard (drop-frame or non-drop). A tempo map built against the wrong frame rate will drift out of sync. The composer must confirm the frame rate and timecode standard with the post-production team before building any tempo map, and must ensure the DAW and any notation software are set to the same standard. Errors here cause sync problems that are hard to diagnose because everything appears correct in isolation.

**Decision criteria:** Is the frame rate and timecode standard confirmed with post-production and matched across all software? Are there any frame-rate mismatches that would cause drift?

## Common Traps

### Writing Freely and Hoping It Lands

A composer who writes a cue by feel, without engineering the tempo and meter against the hit points, will miss the hits and have to redo the cue. The trap mechanism is that the music sounds good in isolation. The harm is wasted effort and cues that do not serve the picture. The corrective is identifying hit points and engineering the tempo map before or during composition.

### Rigid Click-Track Scoring That Sounds Mechanical

A cue locked rigidly to a click, with no expressive timing, sounds lifeless and telegraphs that it was engineered. The trap mechanism is that the grid feels safe and correct. The harm is music that fits the picture technically but lacks the emotional flow that makes a score effective. The corrective is preserving musicality through expressive timing and natural phrasing within the engineered structure.

### Wrong Frame Rate or Timecode Standard

Building a tempo map against 24 fps when the picture is 23.976, or using non-drop when the project is drop-frame, causes the cue to drift out of sync over its duration. The trap mechanism is that everything looks correct in the composer's setup. The harm is sync errors discovered late, often at the mix. The corrective is confirming frame rate and timecode standard before starting and matching all software.

### Inability to Conform When the Cut Changes

A cue composed as a continuous, non-modular piece cannot be easily adjusted when the picture is re-edited, forcing recomposition. The trap mechanism is that the cue works for the current cut, so conformability is not considered. The harm is costly rewrites when the cut changes. The corrective is modular composition and a clear tempo map that support rapid conforming.

### Forcing a Wrong-Feeling Tempo to Hit a Point

Choosing a tempo that lands a hit but feels wrong for the scene produces music that serves the math but betrays the drama. The trap mechanism is that hitting the point feels like success. The harm is a cue that is technically aligned but emotionally off. The corrective is using musical techniques (modulation, bar adjustment) to find a tempo that serves both.

## Self-Check

- Are all hit points in each cue identified with frame-accurate timecode and engineered into the tempo and meter?
- Is there a tempo map synchronized to picture timecode, with a click track that keeps all elements in sync?
- Does the chosen tempo serve both the dramatic feel and the mathematical requirement to land hit points?
- Does the cue sound organic and musical despite being engineered, with expressive timing preserved?
- Can cues be conformed quickly when the cut changes, using the tempo map and modular structure?
- Is the frame rate and timecode standard confirmed with post-production and matched across all software?
- When a cue is played against picture, do all hit points align precisely on the intended frames?
