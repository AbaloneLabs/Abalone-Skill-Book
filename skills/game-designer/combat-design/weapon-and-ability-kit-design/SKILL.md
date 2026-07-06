---
name: weapon-and-ability-kit-design.md
description: Use when the agent is designing player weapon kits, ability loadouts, character move sets, cooldown structures, damage profiles, role archetypes, counter-play relationships, or balancing the tradeoff between expressive player choice and dominant-strategy collapse across action, shooter, fighting, MOBA, and RPG combat systems.
---

# Weapon and Ability Kit Design

A weapon or ability kit is the set of verbs a player brings into combat, and it is the single most direct expression of how a game wants to be played. The judgment problem is that a kit is not a list of cool moves; it is a system of tradeoffs, and every option must be worth choosing in some situation without being worth choosing in every situation. Designers tend to miss this because individual abilities look compelling in isolation — a teleport that feels great, a shotgun that hits hard — but the relationships between the abilities in a kit, and between one kit and its opponents, are what determine whether combat is a series of interesting decisions or a single optimal routine repeated forever. The harm of getting it wrong is severe and specific: a kit with one dominant option collapses the entire decision space, making the rest of the kit dead weight; a kit with no clear identity forces the player to invent their own fun and usually leaves them feeling the character is unfocused or weak. Worse, in competitive games, a single over-tuned kit warps the entire meta, bans become mandatory, and the rest of the roster atrophies. This skill covers the structural design of kits — the internal logic that makes a set of abilities cohere, the counter-play that makes them fair, and the tuning discipline that keeps them honest. The agent has latitude to design creative and idiosyncratic kits, but the obligation is to design each kit as a closed decision system with explicit strengths, explicit weaknesses, and verified counter-play.

## Core Rules

### Define a Kit by Its Core Identity Before Adding Options

Every kit needs a central fantasy and a core gameplan that the player can articulate in one sentence: this is the zone-control mage, the burst assassin, the defensive anchor, the mobile skirmisher. The core identity dictates which abilities belong and which are foreign. A kit without an identity becomes a grab bag of unrelated tools, and the player cannot form an intuition about when to use what, so they default to whichever ability has the best raw numbers. The discipline is to write the identity down before designing abilities, then evaluate every proposed ability against it: does this serve the core gameplan, fill a necessary gap, or is it a cool idea that belongs in a different kit? When a kit has eight abilities and no clear throughline, the fix is subtraction, not addition. The decision criterion: can a new player, after one match, describe what this character is for? If the answer is "a bit of everything," the kit has no identity and will play as nothing in particular.

### Give Every Option a Cost, a Situation, and a Failure Mode

An ability that is always correct to use is not a choice; it is a button you press on cooldown. Real choice requires that each option has a context where it shines, a context where it is wasted or punished, and a resource cost that makes using it a commitment. The cost can be cooldown time, mana or ammo, positioning risk, an animation lock, or an opportunity cost against the other abilities in the kit. The situation is the set of conditions under which the ability is the right answer. The failure mode is what happens when the ability is used at the wrong time — whiffed, interrupted, punished, or simply wasted. When an ability has no failure mode, it is free power and it will dominate the kit. The decision criterion: for each ability, name the situation where it is optimal, the situation where it is a mistake, and the cost the player pays for choosing it. If any of these is undefined, the ability is not yet a decision.

### Ensure No Single Option Dominates the Kit

Within a kit, if one ability is so strong or so universally applicable that a rational player uses it in every situation, the other abilities are dead weight and the kit has effectively one button. This collapses the expressive space the kit was meant to provide. The causes are usually raw numbers (one ability does more damage than the others can match), range mismatch (one ability covers all relevant ranges so the others are never needed), or cooldown asymmetry (one ability is always available so the others are never worth waiting for). The fix is rarely to nerf the dominant ability into the ground; it is to sharpen the situations — narrow its effective range, lengthen its cooldown, add a condition — so that the other abilities become the right answer in the contexts the dominant ability no longer covers. The decision criterion: in a typical engagement, does the player cycle through multiple abilities because each is situationally correct, or do they spam one and ignore the rest? If the latter, the kit has a dominant option and the decision space is fake.

### Design Counter-Play Into Every Lethal or Controlling Ability

An ability that the opponent cannot react to, avoid, or play around is not fair, and fairness is the foundation of combat that players respect. Counter-play means the opponent has a window: a telegraph before the hit, a position they can escape to, a resource they can spend, a state they can avoid entering. A sniper round with no tracer and no audio cue gives the victim no information and no agency; the same round with a visible laser sight and a charge sound gives the victim a chance to dodge, and the kill feels earned rather than cheap. The discipline is to ask, for every ability that deals significant damage or imposes control (stun, root, silence), what the opponent can do about it. When the answer is "nothing," the ability needs a telegraph, a counter, a cost, or a narrower application. The decision criterion: when a player is killed or locked down by this ability, can they identify, in retrospect, what they could have done differently? If not, the ability lacks counter-play and will generate frustration regardless of its balance.

### Separate Sustained Damage, Burst Damage, and Utility Into Clear Roles

Damage in combat is not one thing; it is at least three categories with different purposes. Sustained damage is the baseline pressure that wins long engagements and rewards consistent aim or execution. Burst damage is the spike that secures kills against targets who would otherwise escape or heal, and it is the most balance-sensitive because it can remove counter-play if it exceeds reaction time. Utility — mobility, crowd control, healing, vision — does no damage but shapes the conditions under which damage happens. A kit that mixes these without intention becomes impossible to balance, because buffing sustained damage also buffs burst, and nerfing burst guts sustained. The discipline is to assign each kit a primary role and tune within it, then verify that the kit's burst cannot exceed the threshold of human reaction time plus the opponent's counter-play window. The decision criterion: does this kit's time-to-kill allow the opponent to react and respond, or does it kill faster than a human can process the threat?

### Tune Against the Skilled Player, Not the Average

A kit that is balanced for average play will be broken at high skill, and high-skill play propagates downward through guides, streams, and meta adoption within weeks. The discipline is to balance against the player who executes the kit optimally — perfect aim, frame-perfect combos, optimal cooldown cycling — and then verify that the kit remains usable but not dominant for the average player. This usually means the kit has a high skill ceiling (rewarding mastery) without an unusably high skill floor (punishing beginners). The trap is the reverse: balancing for the average and discovering that skilled players have turned the kit into an unstoppable combo the average player never found. The decision criterion: what does this kit look like in the hands of the top percentile of players, and does it still have the weaknesses and counter-play it was designed to have, or has optimal execution erased them?

### Verify the Kit Against the Roster, Not Just in Isolation

A kit that is internally balanced and fun to play can still break the game when placed against the other kits, because matchups reveal interactions that isolation hides. A mobility-heavy kit may be balanced against another mobile kit but oppressive against a slow, grounded kit with no answer for the repositioning. A burst kit may be fair against a tank but lethal against a support with no survivability tools. The discipline is to build a matchup matrix — every kit against every kit — and identify the matchups where one side has no meaningful response. These are not always fixable by number tuning; sometimes they reveal that a kit is missing a tool it needs to participate in certain matchups, or that a kit's design fundamentally cannot coexist with another's. The decision criterion: across the roster, is there any matchup where one player has no agency against the other's core gameplan? If so, that matchup is broken and the kit needs a tool, a counter, or a redesign.

## Common Traps

### The Swiss-Army Kit With No Weakness

A designer gives a kit strong damage, strong mobility, strong defense, and strong utility because each seemed like a reasonable addition, and the result is a character who has no losing matchup and no situation where they must disengage or swap. The trap is that a kit with no weakness is a kit with no decisions, because the answer to every situation is the same: engage and win. The false signal is that the kit "feels complete and versatile." The harm is that the kit dominates the meta, every other kit is measured against it and found wanting, and the game collapses toward a single optimal pick.

### Burst Damage Faster Than Human Reaction

An ability combo deals lethal damage in a window shorter than the opponent's reaction time plus input latency, so the victim is dead before they can recognize the threat, let alone respond. The trap is that the combo "works as designed" on paper — the numbers add up — but it has removed the counter-play that makes the kill feel fair. The false signal is a high kill rate, which reads as the kit being strong rather than unfair. The harm is frustration, perceived cheapness, and a meta where the only defense is to play the same burst kit, hollowing out roster diversity.

### The Situational Ability That Is Never Situational Enough

A designer creates an ability meant for a specific situation — a shield-breaker for use against fortified enemies — but the situation arises so frequently, or the ability is so cheap to use, that it becomes the default action in every engagement. The trap is that the ability was intended to create situational decisions but, because its preconditions are almost always met, it functions as unconditional power. The false signal is that players "use the ability as intended." The harm is that the situational layer of the kit's design evaporates, and the kit collapses to whichever ability has the best unconditional numbers.

### Copying a Real-Game Kit Without Understanding Its Context

A designer ports a kit from a successful game — a character's move set, a weapon's stats — without accounting for the fact that the kit was balanced within that game's specific damage model, movement speed, health pools, map geometry, and roster. The trap is that the kit's numbers were never absolute; they were relative to a system, and transplanting them into a different system produces a kit that is either wildly overpowered or useless. The false signal is that the kit "worked in the reference game." The harm is balance chaos and a kit that cannot be fixed by tweaking numbers because its fundamental assumptions no longer hold.

### Mobility Without Commensurate Cost

A kit has strong repositioning — a dash, a teleport, a wall-run — with a short cooldown and no resource cost, allowing the player to engage and disengage at will. The trap is that mobility is the most universally useful tool in combat, because it solves both offense (closing distance) and defense (escaping), and when it is cheap it erases the positioning decisions that combat is built around. The false signal is that the mobility "feels great." The harm is that immobile kits become unviable, engagements have no commitment, and the game loses the spatial tension that makes positioning meaningful.

### The Combo That Requires Frame-Perfect Execution to Function

A kit's intended gameplan depends on a combo or ability chain that only works with frame-perfect timing, and the designer treats this as a skill expression rather than a fragility. The trap is that under real network conditions — latency, packet loss, inconsistent frame rates — the combo becomes unreliable, and the kit's effectiveness swings wildly based on connection quality rather than player skill. The false signal is that the combo "works in local testing." The harm is a kit that is overpowered on a perfect connection and unplayable on an average one, making balance impossible and alienating players without top-tier connections.

## Self-Check

- Can I state the kit's core identity in one sentence, and does every ability in the kit serve that identity rather than diluting it into a directionless grab bag?
- For each ability, have I defined the situation where it is optimal, the situation where it is a mistake, and the cost the player pays — so that no ability is always correct to use?
- Has the kit been checked for a dominant option that crowds out the others, and if one exists, have I sharpened its situationality rather than leaving the decision space collapsed?
- Does every lethal or controlling ability have identifiable counter-play — a telegraph, a window, an escape, a counter-resource — so that a killed player can name what they could have done differently?
- Does the kit's time-to-kill allow for reaction and response, and is burst damage separated from sustained damage so that tuning one does not accidentally break the other?
- Has the kit been balanced against optimal (top-percentile) execution, and does it retain its intended weaknesses and counter-play when played perfectly rather than having those erased by skill?
- Has a matchup matrix been run across the roster, and are there any matchups where one side has no meaningful agency against the other's core gameplan?
