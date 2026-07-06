---
name: ar-and-mixed-reality-game-design.md
description: Use when the agent is designing AR and mixed reality games, planning location-based and context-aware mechanics, building for shared physical-digital spaces, or evaluating whether an AR experience uses the real world meaningfully or treats it as a mere backdrop, with safety, privacy, and the unique affordances of spatial computing handled well or handled poorly.
---

# AR and Mixed Reality Game Design

Augmented and mixed reality games overlay digital content on the real world, and this creates a design space unlike any other — the game's mechanics can use the real environment (surfaces, locations, objects), the player's physical context becomes part of the play, and the shared physical-digital space introduces safety, privacy, and social considerations that flat games never face. The judgment problem is that AR must use the real world meaningfully (mechanics that depend on the environment) rather than as a backdrop (a flat game projected onto a camera feed), must keep the player safe in physical space while distracted by digital content, and must respect the privacy and social norms of the spaces it enters. Agents tend to miss this because the AR that demos well (cool overlay) often fails as a game (the overlay is not a mechanic), and because the safety and privacy risks are invisible until a player is harmed or a space is violated. The harm is AR games that are flat games with a camera gimmick, that endanger players in physical space, or that violate the privacy and norms of the spaces they enter. This skill covers how to design AR that uses the real world meaningfully and safely, and avoid the backdrop, safety, and privacy traps. The agent has latitude in the AR design, but the obligation to use the real world meaningfully and safely is not optional.

## Core Rules

### Design Mechanics That Use the Real World Meaningfully

AR mechanics should use the real world meaningfully — mechanics that depend on the environment (surface detection, location, real objects) — rather than treating the real world as a backdrop (a flat game projected onto a camera feed), because meaningful use of the real world is the point of AR, and backdrop AR is a flat game with a gimmick. The decision rule: design mechanics that depend on and use the real environment, and avoid mechanics that ignore the environment. Backdrop AR wastes the medium, because the real world was not used.

### Keep the Player Safe in Physical Space While Distracted

AR players are distracted by digital content while moving in physical space, and the design must keep them safe — warnings, boundary awareness, safe-movement mechanics — because a distracted player in physical space can be harmed (walking into traffic, tripping, trespassing). The decision rule: design for physical safety (warnings, boundaries, safe movement), and avoid mechanics that demand unsafe physical movement. Unsafe AR movement endangers players, because the distraction led to physical harm.

### Respect the Privacy and Norms of Entered Spaces

AR games enter real spaces (public, private, shared), and the design must respect the privacy and norms of those spaces — not capturing or sharing private information, not violating social norms, not trespassing — because AR that violates privacy or norms faces backlash, regulation, and harm. The decision rule: respect the privacy and norms of entered spaces (no private capture, no norm violation, no trespassing), and avoid invasive or norm-violating design. Invasive AR faces backlash, because the privacy or norms were violated.

### Design for Shared Physical-Digital Social Play

AR exists in shared physical space, and the design can leverage this for social play — players in the same physical space playing together, shared digital content visible to co-located players — because shared physical-digital play is a unique AR affordance that flat games cannot replicate. The decision rule: design for shared physical-digital social play where it serves the experience, and avoid ignoring the shared-space affordance. Ignoring shared space wastes the medium, because the co-located social play was not used.

### Account for Environmental Variability and Unreliability

AR depends on the real environment, which is variable and unreliable (lighting, surfaces, GPS, space), and the design must account for this — graceful degradation when the environment cannot support a mechanic, fallback experiences — because a mechanic that requires ideal conditions fails in real environments. The decision rule: account for environmental variability (graceful degradation, fallbacks), and avoid mechanics that require ideal conditions. Ideal-condition mechanics fail in reality, because the environment could not support them.

### Consider Battery, Heat, and Device Constraints of AR

AR is demanding on the device (camera, processing, sensors), and the design must consider battery, heat, and device constraints — pacing sessions, optimizing load — because demanding AR drains the battery, heats the device, and ends sessions, and the device constraints limit what AR can sustain. The decision rule: consider battery, heat, and device constraints (pacing, optimization), and avoid unsustainable load. Unsustainable AR ends sessions, because the device could not sustain the load.

## Common Traps

### Backdrop AR Wasting the Medium

The team designs AR as a flat game projected onto a camera feed, and the real world is a backdrop rather than a mechanic. The trap is that the overlay looks cool. The false signal is that the AR works. The harm is that the mechanics do not use the real environment, the AR is a flat game with a camera gimmick, the unique affordance of AR (using the real world) is wasted, the experience has no reason to be AR, and the game fails to justify its medium, because the real world was not used meaningfully.

### Unsafe Movement Endangering Players

The team designs mechanics that demand unsafe physical movement (walking while distracted, entering dangerous locations), and the distraction endangers players. The trap is that the movement is engaging. The false signal is that the movement works. The harm is that the distracted player moves unsafely in physical space, the player walks into traffic, trips, or trespasses, the physical harm or legal consequence occurs, the game is held responsible, and the experience endangers the players it should protect, because the safety was not designed.

### Invasive AR Violating Privacy and Norms

The team designs AR that captures or shares private information or violates social norms, and the invasiveness faces backlash. The trap is that the capture enables the mechanic. The false signal is that the AR functions. The harm is that the invasive AR captures private spaces or information, the privacy violation is discovered, the backlash and regulation attach to the game, the spaces and communities reject the game, and the experience faces the harm that respect would have prevented, because the privacy and norms were not respected.

### Ignoring Shared Space Wasting the Affordance

The team designs AR for solo play, ignoring the shared physical space, and the unique affordance is wasted. The trap is that solo play is simpler. The false signal is that the game works solo. The harm is that the shared physical-digital social play (a unique AR affordance) is not used, the co-located players play separately rather than together, the medium's strength is wasted, the experience has no reason to be AR over a flat game, and the game fails to leverage its medium, because the shared space was ignored.

### Ideal-Condition Mechanics Failing in Reality

The team designs mechanics that require ideal environmental conditions, and the mechanics fail in real environments. The trap is that the mechanics work in testing. The false signal is that the AR detects surfaces. The harm is that the ideal-condition mechanics fail when the environment is variable (poor lighting, no surfaces, bad GPS), the player cannot play in their real environment, the experience breaks or is unavailable, and the game fails for players whose environment could not support the mechanics, because the variability was not accounted for.

### Unsustainable Load Ending AR Sessions

The team designs AR with unsustainable device load, and the load ends sessions. The trap is that the AR is rich. The false signal is that the AR runs. The harm is that the demanding AR drains the battery, heats the device, the session ends when the battery dies or the device throttles, the session length (and engagement) is limited by the device constraints, the player avoids the game to avoid the drain, and the experience is unsustainable, because the load was not managed.

## Self-Check

- Do mechanics use the real world meaningfully (depending on the environment) rather than treating it as a backdrop?
- Is the player kept safe in physical space while distracted by digital content (warnings, boundaries, safe movement)?
- Are the privacy and norms of entered spaces respected (no private capture, no norm violation, no trespassing)?
- Is shared physical-digital social play leveraged where it serves the experience?
- Does the design account for environmental variability (graceful degradation, fallbacks) rather than requiring ideal conditions?
- Are battery, heat, and device constraints considered, with load managed for sustainable sessions?
- Did I confirm the AR experience uses the real world meaningfully and safely rather than as a backdrop gimmick?
