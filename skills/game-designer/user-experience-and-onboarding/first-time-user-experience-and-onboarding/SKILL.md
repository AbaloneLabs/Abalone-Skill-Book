---
name: first-time-user-experience-and-onboarding.md
description: Use when the agent is designing first-time user experiences, onboarding flows, tutorial sequences, intro cinematics, early-game pacing, contextual teaching, or reviewing whether new players reach the core gameplay loop without confusion, churn, or tutorial fatigue.
---

# First-Time User Experience and Onboarding

The first-time user experience (FTUE) is the single most consequential span of any game, because it determines whether a player ever reaches the experience the rest of the team spent years building. The judgment problem is that onboarding is not the act of explaining the game; it is the act of getting a player to care enough to keep playing while they learn. Designers routinely miss this distinction because they conflate teaching with telling, producing tutorial flows that front-load text, lock the player into rigid linear sequences, and test comprehension rather than enjoyment. The harm is measurable and severe: funnel analytics in nearly every genre show the steepest drop-off in the first session, and the majority of players who abandon a game do so before reaching the mechanics that would have retained them. A weak FTUE wastes the entire downstream investment. Agents tend to err in two directions: over-explaining out of fear that players will be confused, or under-scaffolding out of a desire to respect player intelligence. Both produce churn — the first from boredom and condescension, the second from frustration and the feeling that the game is unfair. The freedom here is considerable, because onboarding style is genuinely genre- and audience-dependent, but the obligation is non-negotiable: the FTUE must be treated as a designed experience that is playtested, measured, and iterated, not as a text dump bolted on at the end of development.

## Core Rules

### Teach Through Action, Not Through Text

The strongest onboarding never tells the player something it can instead let them discover by doing. Reading a prompt that explains a jump mechanic is categorically weaker than placing a small gap the player cannot cross without jumping, because the obstacle creates a motivation to act, the action produces feedback, and the feedback teaches the mechanic as a side effect of solving a real problem. Text-based tutorials fail because they ask the player to hold an abstract instruction in working memory and then apply it later, which most players do not do accurately. The decision criterion is strict: for every concept you intend to teach, ask whether there is a situation you can construct in which the player will naturally perform the correct action to achieve a goal they already have. When such a situation exists, use it and cut the text. When it does not, use the shortest possible contextual prompt, and only after the player has already attempted and failed, so the prompt answers a question they are actively asking.

### Front-Load the Core Fantasy, Defer the Systems

A new player's first ten minutes should deliver the promise of the game — the feeling that made the team want to build it — not a tour of its menus, currencies, and progression systems. The trap is that designers, proud of their systems, want to showcase them immediately, and the result is an FTUE that feels like a product demonstration rather than an experience. The discipline is to identify the single emotional core of the game (being a powerful warrior, exploring a mysterious world, outsmarting opponents) and engineer the opening so the player feels it within the first session. Systems that support that fantasy — skill trees, crafting, economy, social features — should be introduced only once the player has a reason to want them, which usually means after they have encountered a limitation the system resolves. When a system is introduced before the player feels its absence, it reads as homework.

### Respect Returning and Experienced Players With Skippable, Adaptive Onboarding

Onboarding designed only for complete novices punishes the significant fraction of players who are genre veterans, returning after a break, or playing a sequel to a game they already know. Forcing an experienced shooter player through a mandatory "move with WASD" sequence communicates that the game does not respect their time. The discipline is to build onboarding that is adaptive: detect or offer prior experience, allow skipping of foundational tutorials, and provide a condensed refresher rather than a full re-teach. The decision criterion: can a player who already knows the basics reach engaging content within two minutes of starting, without being forced through instruction they do not need? When the answer is no, the FTUE is optimized for the designer's comfort, not the player's.

### Introduce Mechanics at the Moment of Need, Not in Advance

Pre-emptive teaching — explaining a mechanic the player will not use for twenty minutes — is one of the most common and damaging onboarding mistakes, because the explanation is forgotten by the time the mechanic is needed, and the player must be re-taught anyway. The principle of just-in-time teaching is that a concept should be introduced at the exact moment the player has a use for it, when motivation and attention are highest and retention is strongest. This requires mapping the FTUE as a sequence of needs: what problem does the player face, and what mechanic resolves it, in chronological order. When you find yourself explaining something "for later," cut it and move the explanation to the moment of need. The exception is mechanics that are dangerous to discover by trial and error, such as permadeath or irreversible commitments, which warrant an early warning.

### Measure the Funnel, Not Just the Feedback

Onboarding cannot be evaluated by whether playtesters say they understood the tutorial; it must be evaluated by whether they actually progress, and by where they drop off. Funnel analytics — the percentage of players who reach each milestone in the first session — reveal the true shape of the FTUE and expose the specific moments where players quit. Self-reported comprehension is unreliable because players who are confused often blame themselves and report that the tutorial was fine, while players who understood it may still have been bored into leaving. The discipline is to instrument the FTUE with event tracking at every step, run it with real new players (not colleagues), and treat any step with an abnormally high drop-off as a design defect requiring iteration, regardless of how clear the team believes it is.

### Protect the Player's Sense of Agency From the Tutorial

A tutorial that wrests control from the player — locking movement, forcing camera angles, disabling buttons, gating progression behind a single correct action — teaches a meta-lesson that the game is rigid and the player is a passenger. This is especially damaging in games whose core appeal is agency and expression. The discipline is to minimize hard locks: prefer soft guidance (lighting, level geometry, NPC direction, objective markers) over hard constraints, allow multiple valid solutions to tutorial challenges, and never disable a control the player has already learned to use. When a hard lock is genuinely necessary to prevent the player from breaking the sequence, keep it as brief as possible and release it the moment its purpose is served.

## Common Traps

### The Wall of Text Opening

The game opens with multiple panels of instructional text, lore exposition, or control diagrams before the player touches the controls. The trap is that designers assume players read this content, but telemetry across the industry consistently shows that the majority skip or skim tutorial text, meaning the instruction never lands. The false signal is that the text "covers everything," giving the team confidence that players have been informed. The harm is that players enter the game uninformed anyway, now also irritated by the delay, and the churn happens at the text screen itself rather than in gameplay.

### Teaching Everything Before Letting the Player Play

The designer, anxious that players might encounter an unexplained system, front-loads the entire mechanic set into the first level. The trap is cognitive overload: a new player can internalize roughly one to three new mechanics per session before retention collapses, and dumping ten systems on them in twenty minutes ensures none of them stick. The false signal is the feeling of completeness — the designer can check off that every system was "covered." The harm is that players feel overwhelmed, retain nothing, and must relearn everything organically later, at which point the tutorial was wasted effort that also damaged the first impression.

### Mandatory Unskippable Cutscenes Before Gameplay

The game forces the player through a long cinematic or dialogue sequence before granting control, often to establish story or tone. The trap is that the player has not yet been given a reason to care about the characters or world, so the exposition lands on an uninvested audience and reads as a delay. The false signal is that the cinematic is "high quality," which reassures the team that it is earning its length. The harm is that the most churn-prone players — those deciding in the first minutes whether to continue — are spending that decision window watching instead of playing, and many leave before the cinematic ends.

### Tutorial Fatigue From Linear Hand-Holding

The onboarding is a single rigidly scripted corridor where every action is dictated, every door locked until the prescribed step is complete, and no deviation is allowed. The trap is that this format is easy to build and easy to validate (every player has the same experience), so it becomes the default, but it actively trains players that the game is mechanical and joyless. The false signal is that playtests "complete successfully" because players follow the script. The harm is that the FTUE communicates the wrong thing about the game's tone, and players who would have loved the open experience leave believing the game is a constrained chore.

### Ignoring the Second-Day Return Experience

The FTUE is designed and tested for the first session, but the player's second session — when they return, have forgotten half of what they learned, and face a more complex game state — receives no design attention. The trap is that retention is a multi-session phenomenon, and the experience of returning to a half-understood game is a major churn point that first-session onboarding does not address. The false signal is that first-session metrics look healthy. The harm is silent attrition between sessions that never appears in the first-day funnel but devastates long-term retention.

### Over-Reliance on UI Prompts as a Substitute for Level Design

When players get lost or confused, the reflexive fix is to add more UI — objective markers, glowing trails, button prompts, arrows — rather than to improve the level design or feedback that should have guided them organically. The trap is that UI prompts are cheap to add and visibly "solve" the confusion in playtests, so they proliferate until the screen is cluttered with instruction. The false signal is that players stop getting lost. The harm is a game that plays itself, where the player follows markers rather than engaging with the world, and where the underlying navigational and feedback problems are never actually fixed.

## Self-Check

- Does the FTUE teach its core mechanics through player action and contextual obstacles rather than through text, and have I cut every explanation that could instead be discovered by doing?
- Does the opening session deliver the core fantasy and emotional promise of the game before introducing supporting systems, rather than front-loading menus and progression?
- Can an experienced or returning player reach engaging content quickly, with skippable or adaptive onboarding that does not force them through basics they already know?
- Is each mechanic introduced at the moment of need rather than in advance, with no pre-emptive teaching of systems the player will not use imminently?
- Have I instrumented the first-session funnel with event tracking and identified every step with abnormally high drop-off as a defect to iterate, rather than relying on self-reported comprehension?
- Have I minimized hard control locks and rigid scripting, preferring soft guidance that preserves the player's sense of agency?
- Have I designed and tested the second-session return experience, not just the first session, so that returning players are re-oriented rather than lost?
