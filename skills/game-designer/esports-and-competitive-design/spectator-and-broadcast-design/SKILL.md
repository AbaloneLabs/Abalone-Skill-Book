---
name: spectator-and-broadcast-design.md
description: Use when the agent is designing spectator and broadcast features for a competitive game, building replay and observer systems, planning esports presentation, or evaluating whether the game is watchable and understandable to spectators or is opaque, cluttered, and fails to convert viewers into the audience that sustains an esports scene.
---

# Spectator and Broadcast Design

An esports scene depends on spectators — viewers who watch the game — and the game's spectator and broadcast features determine whether it is watchable (clear, engaging, understandable to viewers) or opaque (confusing, cluttered, inaccessible to non-players), and watchability determines whether the game can build the viewer audience that sustains competitive play. The judgment problem is that a game that is great to play can be terrible to watch (the information the player has is not conveyed to the spectator), the spectator's needs differ from the player's (overview, context, clarity over immersion), and the broadcast must serve both the knowledgeable fan and the newcomer. Agents tend to miss this because the game looks clear to the team (who understand it) while being opaque to spectators (who lack the team's knowledge), and because spectator features are often an afterthought added post-launch rather than designed in. The harm is games that cannot build a viewer audience, limiting their esports potential regardless of competitive depth. This skill covers how to design spectator and broadcast features that make the game watchable, serve diverse audiences, and avoid the opacity traps. The agent has latitude in the features, but the obligation to make the game watchable is not optional.

## Core Rules

### Design Spectator View to Convey the Information Viewers Need

The spectator view must convey the information viewers need to understand the match — scores, objectives, key states, player positions — that the player sees through their own interface, because the spectator lacks the player's first-person context and needs it provided through the broadcast view. The decision rule: identify the information a viewer needs to follow the match, ensure the spectator view conveys it, and avoid spectator views that omit critical information. Information-poor spectator views confuse, because the viewer lacked the context the player had.

### Serve Both the Knowledgeable Fan and the Newcomer

The broadcast must serve both the knowledgeable fan (who wants depth, advanced analysis) and the newcomer (who needs explanation, context), because a broadcast that serves only the fan excludes newcomers (limiting audience growth) and one that serves only newcomers bores fans (limiting retention). The decision rule: layer the broadcast to serve both (depth for fans, explanation for newcomers), and avoid serving one at the other's expense. Single-audience broadcasts limit the viewership, because one audience was excluded.

### Provide Observer Tools for Dynamic, Informative Coverage

Broadcast observers (camera operators) need tools — player cameras, overview cams, instant replay, slow motion, tactical overlays — to provide dynamic and informative coverage, because a fixed or limited observer view produces boring or confusing broadcasts. The decision rule: provide robust observer tools (multiple cameras, replays, overlays), and avoid limited observer capabilities. Limited observer tools produce poor broadcasts, because the observer lacked the tools to cover dynamically.

### Make the Game State Legible at a Glance for Spectators

The spectator should be able to grasp the game state at a glance — who is winning, what the objective is, what just happened — through clear visual design, HUD, and presentation, because spectators who cannot quickly grasp the state lose the thread of the match. The decision rule: design the spectator presentation for at-a-glance legibility (clear HUD, visual state indicators), and avoid presentations that require study to understand. Illegible spectator presentations lose viewers, because the state could not be grasped quickly.

### Build Replay and Highlight Systems for Analysis and Content

Replay and highlight systems — reviewing key moments, analyzing plays, creating shareable clips — serve both broadcast analysis (casters explaining what happened) and community content (clips shared on social media), and robust replay systems extend the game's reach through content. The decision rule: build robust replay and highlight systems (full match replay, moment analysis, clip creation), and avoid limited or absent replay. Limited replay restricts analysis and content, because the moments could not be reviewed or shared.

### Design for Spectator Pacing and Dramatic Arc

The broadcast should account for spectator pacing and dramatic arc — building tension, highlighting climaxes, providing analysis in lulls — because a broadcast that does not pace for the viewer (just showing raw gameplay) can be boring even when the match is exciting. The decision rule: design the broadcast for spectator pacing (tension, climax, analysis lulls), and avoid raw-feed broadcasts that ignore viewer pacing. Pacing-ignorant broadcasts bore, because the viewer's experience was not shaped.

## Common Traps

### Information-Poor Spectator Views

The team designs a spectator view that omits the information viewers need, and the spectators cannot follow the match. The trap is that the spectator view shows the game. The false signal is that the match is viewable. The harm is that the viewer lacks the context the player has, the match is confusing, the spectator cannot follow the action or understand the stakes, and the viewers leave because the broadcast was incomprehensible, because the critical information was not conveyed.

### Single-Audience Broadcasts Excluding Viewers

The team designs the broadcast for one audience (knowledgeable fans or newcomers), and the other audience is excluded. The trap is that the broadcast serves its target well. The false signal is that the broadcast is clear. The harm is that the excluded audience (newcomers if fan-focused, fans if newcomer-focused) cannot engage, the viewership is limited to the served audience, the audience does not grow (newcomers excluded) or does not retain (fans bored), and the esports scene's potential is capped, because one audience was excluded.

### Limited Observer Tools Producing Poor Broadcasts

The team provides limited observer tools, and the broadcasts are boring or confusing because the observer cannot cover dynamically. The trap is that the observer view exists. The false signal is that the match can be watched. The harm is that the observer lacks the tools to show key moments, provide overview, or replay action, the broadcast is flat or confusing, the viewers are not engaged, and the broadcast quality undermines the esports presentation, because the observer tools were limited.

### Illegible Spectator Presentation Losing Viewers

The team designs a spectator presentation that requires study to understand, and the spectators who cannot grasp the state at a glance lose the thread. The trap is that the presentation shows the game. The false signal is that the state is displayed. The harm is that the spectator cannot quickly grasp who is winning or what is happening, the thread of the match is lost, the viewer disengages from confusion, and the broadcast fails to retain viewers, because the presentation was not legible at a glance.

### Limited Replay Restricting Analysis and Content

The team provides limited or no replay systems, and the broadcast analysis and community content are restricted. The trap is that the live broadcast is the product. The false signal is that the match is watchable live. The harm is that the casters cannot analyze key moments, the community cannot create shareable clips, the analysis that would deepen engagement is absent, the content that would extend the game's reach is not created, and the esports ecosystem is weaker, because the replay was limited.

### Pacing-Ignorant Raw Broadcasts

The team broadcasts raw gameplay without shaping the spectator pacing, and the broadcast is boring even when the match is exciting. The trap is that the gameplay is the content. The false signal is that the match is being shown. The harm is that the raw feed does not build tension or highlight climaxes, the viewer's experience is not shaped, the exciting match becomes a boring broadcast, and the viewers leave because the pacing was not designed for them, because the broadcast ignored spectator pacing.

## Self-Check

- Does the spectator view convey the information viewers need to follow the match, that players have through their interface?
- Does the broadcast serve both knowledgeable fans (depth) and newcomers (explanation)?
- Are robust observer tools provided (multiple cameras, replays, overlays) for dynamic coverage?
- Is the game state legible at a glance through clear HUD and visual design?
- Are replay and highlight systems robust, supporting both broadcast analysis and community content?
- Is the broadcast designed for spectator pacing and dramatic arc, not just raw gameplay feed?
- Did I confirm the game is watchable and understandable to spectators, not just playable?
