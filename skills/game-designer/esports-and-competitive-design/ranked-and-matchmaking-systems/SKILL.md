---
name: ranked-and-matchmaking-systems.md
description: Use when the agent is designing ranked and matchmaking systems, building skill rating and ladder systems, tuning match quality and queue times, or evaluating whether ranked and matchmaking produce fair, competitive, engaging matches or produce stomps, long queues, rank anxiety, and the smurf-and-boost problems that degrade competitive integrity.
---

# Ranked and Matchmaking Systems

Ranked and matchmaking systems determine whether competitive players get fair, engaging matches or stomps, long queues, and rank anxiety, and they are the foundation of a competitive game's retention — players stay for good matches and leave for bad ones. The judgment problem is that matchmaking must balance match quality (fair, even matches) against queue time (fast matches), ranked must motivate improvement without producing rank anxiety, and the system must defend competitive integrity against smurfs (skilled players in low ranks) and boosters. Agents tend to miss this because optimizing one dimension (fast queues) degrades another (match quality), and because the ranked anxiety that drives players away is invisible in the metrics (which show engagement) until the players leave. The harm is players who churn from stomps and anxiety, ranked systems that demotivate rather than motivate, and competitive scenes undermined by integrity problems. This skill covers how to design ranked and matchmaking that produce fair, engaging matches and motivate improvement. The agent has latitude in the systems, but the obligation to produce quality matches and defend integrity is not optional.

## Core Rules

### Balance Match Quality Against Queue Time Deliberately

Matchmaking must balance match quality (fair, even matches that require searching for well-matched opponents) against queue time (fast matches that may accept poorly-matched opponents), and the tradeoff must be deliberate, because prioritizing queue speed produces stomps (mismatched matches) while prioritizing quality produces long queues (player frustration). The decision rule: define the quality-time tradeoff deliberately, set the matchmaking to the chosen balance, and communicate queue expectations. Imbalanced tradeoffs produce either stomps or long queues, because the balance was not deliberate.

### Design Ranked to Motivate Improvement Without Anxiety

Ranked systems should motivate improvement — the rank reflecting skill, the climb rewarding growth — without producing rank anxiety (the fear of losing rank that makes play stressful rather than enjoyable), and the design must balance motivation against anxiety, because anxiety drives players away from ranked (and possibly the game). The decision rule: design ranked to motivate (clear skill reflection, rewarding climb) while minimizing anxiety (protection from variance, progress visibility), and avoid rank systems that produce debilitating anxiety. Anxiety-producing ranked drives players away, because the stress exceeded the motivation.

### Defend Competitive Integrity Against Smurfs and Boosters

Smurfs (skilled players in low-rank accounts, stomping legitimate low-rank players) and boosters (playing others' accounts to raise their rank) undermine competitive integrity, and the system must detect and address these (smurf detection, account verification, rank adjustment), because integrity problems make ranked meaningless. The decision rule: implement smurf detection and boosting countermeasures, protect ranked integrity, and avoid letting integrity problems fester. Unaddressed smurfs and boosters destroy integrity, because the ranked outcomes were corrupted.

### Make Skill Rating Accurate and Responsive

Skill rating must be accurate (reflecting actual skill) and responsive (updating appropriately with results), because inaccurate or unresponsive rating produces mismatched matches (the rating does not reflect the player's skill), and the rating system must converge on true skill reasonably quickly. The decision rule: use a robust skill rating system (e.g., Elo, Glicko, TrueSkill variants), ensure it converges on true skill, and avoid systems that misrate players. Inaccurate rating produces mismatches, because the rating did not reflect the skill.

### Provide Progress Visibility So Players See Their Trajectory

Players need to see their progress — where they are, where they are going, whether they are improving — through clear rank display, progress indicators, and season structure, because invisible progress demotivates (the player cannot see their growth or trajectory). The decision rule: provide clear progress visibility (rank, progress to next tier, season rewards), and avoid opaque progress. Invisible progress demotivates, because the player could not see their trajectory.

### Use Seasons and Resets to Maintain Engagement and Freshness

Seasons and rank resets maintain engagement by providing fresh starts (new climbs), clear structure (season rewards, endings), and rating recalibration (correcting drift), and the season structure must balance freshness against the frustration of re-climbing. The decision rule: use seasons and resets to maintain freshness and recalibrate ratings, calibrate the reset to balance freshness against re-climb frustration, and avoid either no resets (stale) or excessive resets (frustrating). No-reset ranked stales; excessive-reset ranked frustrates, because the season structure was not calibrated.

## Common Traps

### Queue-Speed Priority Producing Stomps

The team prioritizes queue speed over match quality, and the fast queues come at the cost of mismatched matches that stomp. The trap is that fast queues satisfy players. The false signal is that queues are short. The harm is that the mismatched matches are stomps (one-sided, uncompetitive), the players in stomped matches have a bad experience, the competitive engagement that even matches would sustain is absent, and players churn from the stomps, because the quality was sacrificed for speed.

### Rank Anxiety Driving Players Away

The team designs a ranked system that produces rank anxiety — high stakes for loss, visible rank loss, punishing demotion — and players leave ranked (and possibly the game) to escape the stress. The trap is that high-stakes ranked feels competitive. The false signal is that the ranked is meaningful. The harm is that the anxiety makes play stressful rather than enjoyable, the players leave ranked to escape the stress, the ranked population shrinks, and the competitive scene that depends on ranked participation weakens, because the anxiety exceeded the motivation.

### Unaddressed Smurfs and Boosters Destroying Integrity

The team does not adequately detect and address smurfs and boosters, and the competitive integrity is undermined. The trap is that smurf detection is hard. The false signal is that the ranked is running. The harm is that smurfs stomp legitimate low-rank players (driving them away), boosters corrupt rank accuracy (making ranked meaningless), the integrity problems fester, the ranked outcomes are not trustworthy, and the competitive players who value integrity leave, because the integrity was not defended.

### Inaccurate Skill Rating Producing Mismatches

The team uses a skill rating system that is inaccurate or unresponsive, and the ratings do not reflect true skill, producing mismatched matches. The trap is that the rating system is simple. The false signal is that players have ratings. The harm is that the inaccurate ratings produce mismatched matches (players of different skill paired), the matches are stomps or unsatisfying, the players do not feel their rating reflects their skill, and the matchmaking fails its purpose, because the rating was not accurate.

### Invisible Progress Demotivating Players

The team designs ranked without clear progress visibility, and the players cannot see their trajectory, and they demotivate. The trap is that the rank is displayed. The false signal is that the players know their rank. The harm is that the players cannot see whether they are improving, the progress feels opaque, the motivation that visible growth would provide is absent, the climb feels aimless, and players disengage from the ranked, because the progress was not visible.

### No Resets (Stale) or Excessive Resets (Frustrating)

The team uses no rank resets (the ranked stales as ratings drift and the climb is finished) or excessive resets (the ranked frustrates as players re-climb constantly), and the season structure harms engagement. The trap is that the reset policy is set. The false signal is that the ranked has seasons. The harm is that no-reset ranked stales (the climb is over, ratings drift, engagement declines) or excessive-reset ranked frustrates (the re-climb is constant, progress feels erased), and the season structure that should sustain engagement instead harms it, because the reset calibration was wrong.

## Self-Check

- Is the match quality–queue time tradeoff deliberate, avoiding both stomp-producing speed and frustration-producing waits?
- Does ranked motivate improvement while minimizing the rank anxiety that drives players away?
- Are smurfs and boosters detected and addressed to defend competitive integrity?
- Is the skill rating system accurate and responsive, converging on true skill to produce well-matched games?
- Is progress visible (rank, trajectory, season rewards) so players see their growth and stay motivated?
- Are seasons and resets calibrated to maintain freshness without frustrating re-climbs?
- Did I confirm ranked and matchmaking produce fair, engaging matches that retain competitive players?
