---
name: genre-convention-selection.md
description: Use when the agent is choosing which genre conventions to honor versus subvert, deciding how closely to follow player expectations for a given game category, or evaluating whether breaking a convention will read as innovation or as a broken promise to the audience.
---

# Genre Convention Selection

A genre is a contract: players approach a game with expectations formed by every similar game they have played, and those expectations are both a resource and a trap. Honoring conventions gives players an on-ramp, lets marketing communicate the game in a sentence, and reduces the teaching burden. Subverting conventions is where distinctive identity and memorable moments live. The judgment problem is that conventions are invisible until violated, and agents tend to either over-honor them (producing a competent but indistinguishable game) or over-subvert them (producing a game that reads as buggy or hostile because it broke promises the team did not realize it was making). Agents miss this because conventions feel like neutral defaults rather than active choices, and because subversion feels creative in the studio but reads as defect in the wild. The harm is a game that either fails to find an audience because it is generic, or fails to retain one because players feel deceived by a core system that behaves unlike its peers. This skill covers how to audit the conventions of a target genre, decide which are load-bearing promises, and subvert only where the payoff justifies the friction. The agent has latitude in which conventions to keep, but the obligation to make each choice deliberate and communicated is not optional.

## Core Rules

### Audit the Genre's Load-Bearing Conventions Before Designing Against Them

Every genre has a small set of conventions that players treat as promises: a first-person shooter aims with the mouse or right stick, a Metroidvania gates progress behind acquired abilities, a deck-builder shuffles a draw pile. Before subverting anything, enumerate these conventions explicitly and classify each as load-bearing (violating it breaks the genre contract) or stylistic (violating it is merely unusual). The decision rule: you may only subvert a load-bearing convention if you can name the specific experience the subversion creates and the friction it costs, and if you have a plan to teach the new behavior. Teams that subvert without this audit break promises they never knew existed.

### Distinguish Convention Subversion From Convention Ignorance

A deliberate subversion signals to the player that the rule is being broken on purpose and rewards them for noticing. Ignorance simply omits a convention, and the player reads the omission as a bug or oversight. The decision rule: every subversion must be telegraphed — through tutorial framing, visual emphasis, or a moment where the old expectation is visibly denied — so the player understands the new contract. If the player cannot tell whether a missing convention is intentional, the design has failed regardless of the underlying idea.

### Match Convention Fidelity to Audience Familiarity and Market Position

The degree to which you honor conventions should depend on who you are targeting. A game aimed at genre veterans can subvert aggressively because the audience knows the rules being broken; a game aimed at newcomers or a broad market should honor conventions more fully because the conventions are doing the teaching. The decision rule: map your target audience's prior genre experience, and calibrate convention fidelity to the lowest-experience player you must reach. A niche hardcore game that honors every convention will feel safe but redundant; a mass-market game that subverts core conventions will feel alien.

### Use Conventions to Reduce Onboarding Cost, Then Spend the Saved Budget on Innovation

Conventions are borrowed teaching: every player who has played a prior game in the genre arrives already knowing the controls, the HUD grammar, and the failure states. The strategic move is to honor the conventions that are expensive to teach, and spend the saved onboarding budget on the one or two systems where you actually innovate. The decision rule: list the conventions you are keeping, estimate the tutorial time each saves, and redirect that budget to teaching your genuinely novel mechanic well. Teams that innovate everywhere teach nothing well.

### Verify Subversions Survive Contact With Marketing and Store Page Framing

A subversion that works in the studio can fail in the market if the game is sold as a conventional genre entry. Players who bought "a soulslike" and find the stamina system removed will feel betrayed even if the removal is brilliant. The decision rule: review the store page, trailer, and key art against the convention choices, and ensure the marketing either sets the subversive expectation or does not over-promise the convention. The design and the marketing must agree on which contract the player is signing.

### Plan for the Reviewer and Streamer Who Skips the Tutorial

Reviewers and streamers often skip tutorials, play fast, and form opinions in minutes. A subversion that depends on careful teaching will be misread as a defect by this audience, and their impressions shape the launch window. The decision rule: stress-test every subversion against a player who ignores all framing and plays on prior-genre instinct, and ensure the consequence of the subversion is either gentle enough to survive misunderstanding or dramatic enough to force re-engagement. Do not assume the player read the sign.

## Common Traps

### Subverting a Load-Bearing Convention to Be Clever

The team removes a core genre system — stamina, permadeath, the turn structure — because it feels fresh in the studio, without recognizing that the system was load-bearing for that audience's sense of the game. The trap is that the removal produces immediate novelty and internal excitement. The false signal is that the prototype feels distinctive and the team is energized. The harm is that the target audience experiences the removal as a broken promise, reviews frame it as a flaw, and the game fails to convert the very players its genre was supposed to attract, because the team optimized for originality over the contract that genre selection implied.

### Keeping Every Convention Out of Fear

The team, aware that breaking conventions is risky, retains every standard system, control scheme, and progression shape, producing a game that is competent and indistinguishable. The trap is that honoring conventions is low-risk and feels responsible, and each retained convention is individually defensible. The false signal is positive playtests where players say "this is solid." The harm is that the game has no distinctive identity, the marketing cannot articulate why a player should choose it over the genre leader, and it dies in a crowded market not because it is bad but because it gives no one a reason to switch.

### Assuming the Audience Knows the Convention You Are Subverting

A subversion only lands if the player recognizes the rule being broken, but teams often subvert conventions their actual audience never learned, because the audience skews younger or crossed over from a different genre. The trap is that the team's reference pool is deep, so the subversion feels meaningful to them. The false signal is that genre-literate internal testers respond strongly to the subversion. The harm is that the real audience experiences the "subverted" system as simply the game's normal behavior, receiving none of the intended frisson while still paying the friction cost, so the cleverness is wasted and the budget spent on it is unrecovered.

### Marketing the Game as the Convention It Subverts

The trailer and store page sell the game using the imagery and language of the convention the design team deliberately removed, because that language is what converts to wishlists. The trap is that convention-flavored marketing performs better in pre-launch metrics. The false signal is strong wishlist growth and positive trailer engagement. The harm is a launch mismatch: players arrive expecting the convention, find it absent, and review-bomb or refund, and the team is forced to either patch the convention back in under pressure or watch retention collapse, either way eroding the design intent.

### Treating Genre Blending as Automatically Innovative

The team combines two genres — a deck-builder and a farming sim — assuming the blend is inherently fresh, without auditing which conventions from each parent survive and whether they cohere. The trap is that "genre mashup" reads as creative in a pitch. The false signal is that the combination has no direct competitor. The harm is that the inherited conventions from each genre often conflict — the pacing of one fights the loop of the other — and the shipped game feels like two half-games stitched together, satisfying neither audience, because no one reconciled the convention sets before building.

## Self-Check

- Have I explicitly enumerated the load-bearing conventions of the target genre and classified each as a promise I must keep or a candidate for deliberate subversion?
- For every convention I chose to subvert, can I name the specific experience the subversion creates, the friction it costs, and the teaching plan that telegraphs it as intentional?
- Did I calibrate convention fidelity to the prior genre experience of my actual target audience, rather than to the team's own genre literacy?
- Have I redirected the onboarding budget saved by honored conventions toward teaching my genuinely novel systems well, instead of innovating diffusely across every system?
- Do the store page, trailer, and key art set expectations consistent with my convention choices, so no player buys a contract the game does not honor?
- Have I stress-tested each subversion against a player who skips the tutorial and plays on prior-genre instinct, to confirm it survives misunderstanding?
- For any genre blend, have I reconciled the conflicting conventions inherited from each parent rather than shipping both sets unresolved?
