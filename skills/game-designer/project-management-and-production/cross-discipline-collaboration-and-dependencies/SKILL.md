---
name: cross-discipline-collaboration-and-dependencies.md
description: Use when the agent is coordinating design with art, engineering, audio, and production, resolving cross-discipline dependencies, managing handoffs between teams, aligning conflicting creative visions, or preventing integration failures caused by unmanaged interdependencies.
---

# Cross-Discipline Collaboration and Dependencies

A game is built by disciplines that each optimize for a different good, and the integration of those goods is where most production pain lives. The judgment problem is that design, art, engineering, audio, and production each hold a partial view of the game, and the dependencies between them are invisible until a handoff fails and the team discovers that two disciplines built incompatible assumptions into the same feature. Agents miss the critical work because collaboration feels like communication rather than dependency management, the conflicts that matter are structural and not interpersonal, and the cost of an unmanaged dependency appears at integration when it is most expensive to fix. This skill covers how to map dependencies before they bite, how to run handoffs that transfer accountability rather than blame, and how to resolve creative conflicts without either suppressing a discipline's expertise or abdicating the design's coherence. The designer leads the creative vision but must do so as a coordinator of experts, not a dictator of preferences.

## Core Rules

### Map Dependencies Before They Become Integration Emergencies

Every feature is a web of dependencies: art needs the design spec to know what to build, engineering needs the art format to know how to load it, audio needs the gameplay context to know when cues fire, and localization needs the final text before it can translate. The rule is that these dependencies must be mapped explicitly and early, with the consumer of each dependency named and the delivery date negotiated with the producer. The trap is treating dependencies as things that resolve themselves through proximity and good will, when in fact they resolve themselves through rework and delay. When a dependency is mapped, it can be sequenced; when it is not, it becomes the integration bug that blocks three teams for a week.

### Make Handoffs Transfer Accountability, Not Just Artifacts

A handoff is not the moment a file changes systems; it is the moment responsibility for correctness passes from one discipline to another. The rule is that every handoff must include not just the asset or spec but the acceptance criteria, the known constraints, and the open questions, so that the receiving discipline can verify what they got rather than discovering its limits later. When a handoff is just a file transfer, the receiver inherits hidden problems and the sender believes the work is done. When the handoff includes the criteria for done, both sides share a definition of success and the inevitable gaps surface during the handoff rather than during integration. Treat the handoff as a contract with a verification step, not a drop.

### Resolve Creative Conflicts at the Level of Player Experience

When art wants a slower, more cinematic combat and design wants a faster, more responsive one, the conflict is real and cannot be resolved by either side winning. The rule is that creative conflicts between disciplines must be escalated to the question of what serves the player experience, evaluated against the game's stated intent, rather than resolved by seniority or volume. This requires the designer to articulate the experience goal specifically enough that tradeoffs become visible: if the intent is tension, the slower animation may serve it; if the intent is flow, it may not. When conflicts are resolved at the level of preference, the losing discipline disengages and the winning decision is weaker for lacking their expertise. When resolved at the level of player experience, the conflict becomes a shared problem.

### Give Each Discipline Authority Over Its Expertise, Not Veto Over the Game

A healthy collaboration gives each discipline genuine authority over the things they know best: art over visual composition, audio over mix and cue placement, engineering over architecture and performance. The rule is that this authority is over their domain's execution, not over whether a feature exists or what experience it targets, which are design decisions. The failure at one extreme is the designer who dictates animation timing and shader choices, alienating the experts; at the other extreme is the designer who lets any discipline veto a feature on taste, losing coherence. The judgment is to delegate execution authority fully while retaining the authority to define what the game is trying to do, and to challenge a discipline's execution only against the agreed experience goal, never against personal preference.

### Surface the Implicit Assumptions Each Discipline Carries

Each discipline carries assumptions the others cannot see. Engineering assumes the feature will be data-driven; art assumes the camera will frame the asset a certain way; design assumes the audio will sell the impact. The rule is that cross-discipline reviews must explicitly surface these assumptions, because the integration failures come from the assumptions that were never stated and therefore never reconciled. The practice is to ask, for each feature, what each discipline is assuming about the others' work, and to write those assumptions down so they can be confirmed or corrected. An assumption that is written can be wrong and fixed; an assumption that is implicit becomes a bug.

### Sequence Work to the Critical Path, Not to Disciplinary Comfort

Different disciplines work at different cadences and prefer to work in different orders, but the schedule must follow the dependencies, not the preferences. The rule is that the critical path, the longest chain of dependent tasks, determines the order, and each discipline must sometimes do work in an order that is locally inconvenient to unblock another team. The trap is allowing a discipline to optimize its own workflow in a way that serializes dependencies and lengthens the critical path. When the producer and the designer understand the critical path, they can ask each discipline to absorb a local inconvenience to shorten the project, and that request is fair because it serves the whole.

### Protect the Spaces Where Disciplines Must Iterate Together

Some problems cannot be solved by handoff; they require designers, artists, and engineers in the same room iterating on the same build until the feel is right. Combat feel, camera behavior, and moment-to-moment pacing are examples. The rule is that these collaborative iteration spaces must be protected on the schedule, not treated as overhead, because they are where the game's quality is actually made. When these sessions are skipped in favor of asynchronous handoffs, the feature ships with each discipline having optimized its own layer and the integrated feel being nobody's responsibility. Reserve time for the work that only happens when the disciplines are present together.

## Common Traps

### The Handoff With No Definition of Done

Design hands a spec to engineering, engineering builds something, and design discovers it does not match intent, but neither side can say whether the build is wrong or the spec was ambiguous. The trap is that without an agreed definition of done at handoff, every gap becomes a dispute about fault rather than a problem to solve. The false signal is that the handoff occurred, which feels like progress. The harm is rework, blame, and the erosion of trust between disciplines that makes every future handoff slower and more defensive.

### Resolving Conflict by Seniority or Loudness

A creative disagreement arises, and it is resolved by whoever has the higher title or the stronger personality prevailing, with the substance of the conflict never examined. The trap is that this produces decisions optimized for internal politics rather than player experience, and the losing discipline contributes less thereafter because its expertise was overruled without reason. The false signal is the apparent resolution and the absence of ongoing argument. The harm is a coherent-looking but experientially weaker game, plus a team that has learned that expertise does not matter as much as rank.

### Assuming Proximity and Good Will Will Resolve Dependencies

The disciplines sit near each other, talk daily, and assume that dependencies will sort themselves out through informal coordination. The trap is that informal coordination handles the easy dependencies and silently drops the hard ones, because the hard dependencies are the ones nobody wants to own and nobody has been assigned. The false signal is the constant communication, which feels like coordination. The harm is that the critical dependency surfaces at integration as a blocker, and the team scrambles to resolve under deadline what should have been sequenced months earlier.

### Letting One Discipline's Optimization Serialize the Project

A discipline reorganizes its workflow for internal efficiency in a way that forces other teams to wait, and because the change is locally rational it goes unchallenged. The trap is that local optimization can lengthen the global critical path, and without anyone owning the path, the serialization is invisible until the schedule slips. The false signal is that each team is working efficiently. The harm is that the project takes longer despite every team feeling productive, because the dependencies have been arranged suboptimally and nobody has the authority or the visibility to reorder them.

### Suppressing a Discipline's Expertise to Avoid Conflict

The designer, wanting to keep the peace, accepts an art or audio direction that contradicts the experience goal rather than pushing back, and the shipped feature is incoherent as a result. The trap is that conflict avoidance feels like collaboration but actually produces a weaker game, because the disciplines were not aligned on intent and their outputs do not reinforce each other. The false signal is the smooth, conflict-free process. The harm is a feature where the art says one thing, the audio another, and the design a third, and the player experiences the mismatch as a game that does not know what it wants to be.

### The Integration Bug That Belongs to Nobody

At integration, a feature breaks, and each discipline can plausibly argue its own layer is correct, so the bug sits unfixed because no single team owns the gap between the layers. The trap is that integration failures are often cross-disciplinary by nature and fall outside any team's defined responsibility, so they persist. The false signal is that each team has met its own acceptance criteria. The harm is that the gap ships as a visible flaw, and the absence of ownership means it is never properly resolved, only patched around.

## Self-Check

- Have the dependencies for each feature been mapped explicitly, with the consumer, the deliverable, and the date named for each?
- Does every handoff include acceptance criteria, known constraints, and open questions, so the receiver can verify rather than inherit hidden problems?
- When a creative conflict arises, is it resolved by reference to the player experience and the game's stated intent, rather than by seniority or volume?
- Does each discipline hold authority over its own execution while the designer retains authority over what experience the game targets, with neither overreach nor abdication?
- Have the implicit assumptions each discipline carries about the others' work been surfaced and written down so they can be confirmed or corrected?
- Is the work sequenced to the critical path of dependencies, with local inconveniences accepted where they shorten the project, rather than to each discipline's comfort?
- Are protected collaborative iteration sessions scheduled for the features, like combat feel and camera, that can only be solved with disciplines present together?
