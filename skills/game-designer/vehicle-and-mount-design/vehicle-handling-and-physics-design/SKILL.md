---
name: vehicle-handling-and-physics-design.md
description: Use when the agent is designing vehicle handling and physics, tuning driving or flight feel, building vehicle mechanics, or evaluating whether vehicle handling feels responsive, skill-expressive, and fun or feels floaty, twitchy, punishing, and disconnected from player input in ways that undermine the vehicle fantasy.
---

# Vehicle Handling and Physics Design

Vehicle handling — the feel of driving, flying, or riding — is the core of any vehicle-focused game, and it is also one of the most difficult feels to get right, because handling exists in a narrow band between responsive and twitchy, between weighty and floaty, between skill-expressive and punishing. The judgment problem is that handling must serve the vehicle fantasy (a sports car should feel different from a truck), must be readable and consistent (so the player can predict and express skill), and must be tuned to the input and physics model, and agents tend to miss this because handling that feels fine to the designer (who has internalized it) feels wrong to a new player, and because the temptation to add realism can produce handling that is technically accurate but not fun. The harm is vehicles that fight the player's input, that do not express the fantasy, or that are punishing in ways that exclude all but the most patient players. This skill covers how to design vehicle handling that serves the fantasy, is readable and skill-expressive, and avoids the floaty-twitchy-punishing traps. The agent has latitude in the physics, but the obligation to make handling serve the fantasy and the player is not optional.

## Core Rules

### Match Handling to the Vehicle Fantasy and Player Expectation

Vehicle handling must match the fantasy the player brings to the vehicle — a sports car should feel fast and precise, a truck should feel heavy and powerful, a fighter jet should feel agile and responsive — because handling that betrays the fantasy breaks the engagement the vehicle was meant to deliver. The decision rule: for each vehicle, define the fantasy and the handling qualities that serve it, and tune the physics to express those qualities, avoiding handling that contradicts the fantasy. Handling that contradicts the fantasy disappoints, because the vehicle did not deliver the experience the player came for.

### Tune Handling to Be Readable and Predictable

The player must be able to read and predict the vehicle's behavior — how it will respond to input, how it will behave at the limit, how it will recover — so driving is about skill and expression rather than about fighting unpredictable physics. The decision rule: tune handling for readability and predictability, ensure the vehicle responds consistently to input, and avoid physics that behave unpredictably at the limit. Unpredictable handling produces fighting the vehicle rather than driving it, because the player could not predict the response.

### Balance Realism With Fun and Accessibility

Vehicle physics must balance realism (which serves authenticity and depth) with fun and accessibility (which serves engagement), because pure realism can produce handling that is technically accurate but punishing and exclusionary, while pure arcade handling can feel weightless and shallow. The decision rule: decide the realism-fun balance based on the game's audience and fantasy, tune physics to that balance, and avoid realism that excludes or arcade simplicity that shallows the experience. Realism-first handling can exclude all but specialists; arcade-first handling can lack depth, because the balance was not chosen for the audience.

### Make Handling Skill-Expressive With a High Skill Ceiling

Vehicle handling should be skill-expressive — allowing better players to drive better, with techniques to master and a high skill ceiling — so driving rewards investment and produces mastery, rather than being a binary that anyone can do equally. The decision rule: design handling with a skill expression curve (techniques to learn, limits to master, optimization to pursue), and ensure the ceiling is high enough to reward dedicated players. Handling without skill expression is shallow, because the player cannot improve and the driving does not reward investment.

### Ensure Input Responsiveness Matches the Handling Model

The input responsiveness — how quickly and precisely the vehicle responds to player input — must match the handling model, because sluggish input undermines precision handling and overly sensitive input produces twitchiness, and the mismatch produces fighting the controls. The decision rule: tune input responsiveness to the handling model's demands (precise handling needs responsive input, weighty handling tolerates slight delay), and avoid mismatches that fight the player. Input-handling mismatch produces a vehicle that fights the controls, because the responsiveness did not serve the handling.

### Provide Recovery Mechanics So Mistakes Are Survivable

Vehicle handling should include recovery mechanics — ways to recover from mistakes, spins, or loss of control — so mistakes are survivable and learning can occur, rather than every mistake being terminal and punishing. The decision rule: design recovery mechanics (counter-steer, stabilization, reset options), ensure mistakes are recoverable, and avoid handling where every error is fatal. Terminal-mistake handling punishes learning, because the player could not recover and retry from errors.

## Common Traps

### Handling That Betrays the Vehicle Fantasy

The team tunes vehicle handling without aligning to the fantasy, and a sports car handles like a boat or a truck handles like a go-kart, betraying the player's expectation. The trap is that the handling is internally consistent. The false signal is that the vehicle drives. The harm is that the vehicle does not deliver the fantasy the player came for, the engagement the vehicle was meant to provide is absent, the player is disappointed by a vehicle that feels wrong for what it is, and the vehicle fantasy is broken, because the handling was not matched to the expectation.

### Unpredictable Handling That the Player Fights

The team tunes physics that behave unpredictably at the limit — the vehicle snaps, grips inconsistently, or recovers randomly — and the player cannot predict the behavior, and drives defensively to avoid the unpredictability rather than expressively. The trap is that the physics are realistic. The false signal is that the handling has depth. The harm is that the player cannot predict the vehicle's response, driving becomes about avoiding the unpredictable limit rather than exploring it, the skill expression that predictable handling would enable is absent, and the player fights the vehicle rather than drives it, because the handling was not readable.

### Realism-First Handling That Excludes Players

The team tunes vehicle physics for maximum realism, assuming realism equals quality, and the handling is technically accurate but punishing and exclusionary, accessible only to specialists. The trap is that realism feels authentic. The false signal is that the handling is sophisticated. The harm is that the realistic handling excludes the majority of the audience, the learning curve is too steep for the game's intended players, the engagement that accessible handling would sustain is absent, and the game serves a niche rather than its audience, because realism was prioritized over fun and accessibility.

### Handling Without Skill Expression

The team designs vehicle handling that anyone can do equally — no techniques to master, no limit to explore — and the driving is shallow and does not reward investment. The trap is that accessible handling serves everyone. The false signal is that the vehicle is easy to drive. The harm is that the handling has no skill expression curve, the player cannot improve, the driving does not reward investment, the mastery that a high skill ceiling would sustain is absent, and the vehicle engagement is shallow, because the skill expression was not designed.

### Input-Handling Mismatch That Fights the Controls

The team tunes input responsiveness independently of the handling model, and the mismatch produces a vehicle that fights the controls — sluggish input on a precision vehicle, twitchy input on a weighty vehicle. The trap is that the input and handling were tuned separately. The false signal is that both are adjusted. The harm is that the responsiveness does not serve the handling, the player fights the controls to get the vehicle to respond as the handling demands, the precision or weight the handling intends is undermined by the input, and the vehicle feels wrong, because the input and handling were not matched.

### Terminal Mistakes With No Recovery

The team designs handling where every mistake — a spin, a loss of control — is terminal, with no recovery mechanics, and the player is punished for every error with no chance to recover and learn. The trap is that terminal mistakes add stakes. The false signal is that the handling is punishing. The harm is that the player cannot recover from errors, learning cannot occur through retry, the frustration of terminal punishment accumulates, and the player who would have learned the handling instead abandons it, because the recovery that would enable learning was absent.

## Self-Check

- Does the vehicle handling match and express the fantasy the player brings to the vehicle?
- Is the handling readable and predictable, so the player can drive expressively rather than fight unpredictability?
- Have I balanced realism with fun and accessibility, chosen for the audience rather than for authenticity alone?
- Does the handling have a skill expression curve with techniques to master and a high skill ceiling?
- Does the input responsiveness match the handling model's demands, without fighting the controls?
- Are there recovery mechanics so mistakes are survivable and learning can occur through retry?
- Did I confirm the handling serves the fantasy and the player, rather than the physics simulation?
