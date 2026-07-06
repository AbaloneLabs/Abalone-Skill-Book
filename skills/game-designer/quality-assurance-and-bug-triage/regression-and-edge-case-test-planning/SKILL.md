---
name: regression-and-edge-case-test-planning.md
description: Use when the agent is planning regression test suites for a game, designing edge case coverage, defining what to retest after a change, structuring QA test plans for systems and content, or ensuring fixes do not break adjacent features during production and live patches.
---

# Regression and Edge Case Test Planning

Regression is the failure mode where a fix or a new feature breaks something that used to work, and edge cases are the inputs at the boundary where systems behave unpredictably. The judgment problem is that testing the happy path proves nothing about the boundaries, and the team's confidence after a fix is usually based on the fix working, not on the absence of new breakage elsewhere. Agents miss the critical work because a test plan that covers the main flow looks thorough, the regressions appear only at integration when many changes interact, and the cost of an untested edge case is paid by players who find it after ship. This skill covers how to design regression coverage that actually catches breakage, how to find the edge cases that matter rather than the ones that are easy to enumerate, and how to scope retesting so that it is affordable rather than abandoned under deadline. The designer collaborates with QA but is accountable for identifying the risky boundaries that testing must cover, because the designer knows where the systems interact.

## Core Rules

### Design Regression Around the Dependency Graph, Not the Feature List

A change to one system can break any system that depends on it, so regression coverage must be driven by the dependency graph, not by the list of features that were directly touched. The rule is that for any change, the regression set includes the changed feature plus every feature that reads its state, shares its data, or chains from its output. The trap is regressing only the changed feature, which proves the fix works but proves nothing about the collateral damage. When the dependency graph is mapped, regression becomes targeted: a change to the inventory system triggers retests of crafting, shops, quests that grant items, and saves, because all of them consume inventory state. When the graph is not mapped, regression is either omitted or applied indiscriminately, and both fail to catch the real regressions.

### Find Edge Cases at System Boundaries, Not in the Middle of Valid Ranges

Edge cases cluster at the boundaries: zero, one, and maximum of a resource; the first and last item in a list; simultaneous inputs; state transitions during loading; the moment a timer expires during another action. The rule is that test cases should be designed at the boundaries of every variable and every state transition, because the middle of the valid range is where the code is most tested by normal play and least likely to fail. The trap is enumerating many test cases in the comfortable middle and calling it coverage, when the defects live at the edges the comfortable cases never reach. For each system, list its variables, identify each variable's boundaries, and generate cases at those boundaries plus the transitions between states.

### Treat Concurrency and Timing as First-Class Edge Cases

The hardest bugs come from timing: two events resolving in an unexpected order, an input arriving during a state transition, a save triggered while an inventory operation is mid-flight, a network packet arriving during a load. The rule is that any system with asynchronous events, network communication, or shared state must have test cases that exercise the timing boundaries, not just the sequential flow. The trap is testing only the sequential happy path and assuming timing will sort itself out, when timing bugs are the most common source of save corruption, duplication, and desync. Design cases that interrupt actions mid-progress, trigger events simultaneously, and stress the ordering assumptions, because these are the cases players will hit through latency and rapid input.

### Scope Retesting to What Changed and What Touches It, Then Verify the Boundary

Full regression of the entire game after every change is unaffordable, so retesting must be scoped, but scoping too narrowly misses regressions and scoping too broadly wastes the budget. The rule is to scope to the changed feature and its dependency neighborhood, then to define an explicit boundary beyond which retesting is not performed, and to make that boundary a conscious decision rather than an accident. The boundary matters because it defines the risk being accepted: everything inside the boundary is retested, everything outside is assumed stable, and that assumption should be justified by the change's blast radius. When the boundary is implicit, regressions escape it silently; when it is explicit, a missed regression is a known risk that was weighed, not an oversight.

### Make Regression Repeatable and Automated Where the Cost Justifies It

A regression suite that must be re-run manually every build will not be run every build, because manual regression is slow and expensive and gets skipped under pressure. The rule is that the highest-value regression cases, the ones that catch frequent or severe regressions, should be automated so they run on every build without human effort. The trap is attempting to automate everything, which produces a brittle suite that costs more to maintain than it saves, or automating nothing, which leaves regression to manual testing that cannot keep pace. The judgment is to automate the stable, high-frequency, high-severity cases and leave the exploratory and feel-based testing to humans, because humans find the regressions automation cannot predict.

### Include the Failure and Recovery Paths, Not Just the Success Path

Most test coverage targets the case where things go right. The rule is that equal coverage must target the cases where things go wrong: what happens when a network connection drops mid-action, when a save fails, when a required asset is missing, when the player runs out of a resource mid-craft. The trap is that failure paths are often unimplemented or untested, so the game behaves unpredictably exactly when the player is already frustrated. For each system, enumerate its failure modes and test that each fails gracefully, recovers correctly, and communicates the failure to the player. A system that works when nothing goes wrong is not finished; it is finished when it degrades gracefully when everything goes wrong.

### Test the Content, Not Just the Code, at Scale

Code-level testing proves the systems work in isolation, but games fail at scale when hundreds of content items interact. The rule is that content must be tested in aggregate: every weapon against every enemy type, every quest in sequence with every other quest, every level loaded after every other level to catch memory leaks. The trap is testing a representative sample and assuming the rest hold, when content authored by different people under different assumptions will contain combinations the code never anticipated. Automated content validation catches schema errors; only aggregate play and stress testing catches the interaction failures that emerge at scale.

## Common Traps

### Happy Path Coverage Mistaken for Thorough Coverage

The test plan lists many cases, all of them follow the intended player flow, and the team believes the system is well-tested because the case count is high. The trap is that happy path cases prove the system works when used exactly as intended, which is the scenario least likely to fail and least likely to be how players actually behave. The false signal is the large case count. The harm is that the boundaries and failure paths ship untested, and the defects appear in player hands at the edges the test plan never reached.

### Regression Scoped Only to the Changed Feature

A fix is made to the inventory system, the inventory is retested and works, and the fix ships, only to break crafting because crafting reads inventory state in a way the fix changed. The trap is that regression was scoped to the changed feature alone, ignoring the dependency neighborhood. The false signal is that the changed feature passes its tests. The harm is collateral regressions in adjacent systems that were never retested, discovered by players or at cert when they are most expensive.

### Ignoring Timing and Concurrency Until It Ships

The team tests sequential flows and assumes the game is solid, but the shipped game suffers duplication bugs, save corruption, and desync because timing cases were never exercised. The trap is that timing bugs are invisible in sequential testing and appear only under real latency and concurrent input, which the test plan did not simulate. The false signal is that the sequential tests pass consistently. The harm is the most severe bug category, corruption and desync, shipping to players because the hardest cases were never designed into the plan.

### Automating Everything Until the Suite Becomes Unmaintainable

The team automates aggressively, the suite grows large, and then every content change breaks dozens of automated tests that must be updated, until maintaining the suite consumes more effort than the regressions it catches. The trap is that automation has a maintenance cost that scales with volatility, and automating volatile content produces a suite that breaks constantly and erodes confidence. The false signal is the high automation coverage percentage. The harm is that the team starts ignoring or disabling failing tests, the suite becomes noise, and the real regressions slip through the disabled checks.

### Testing Code in Isolation but Never Stressing Content at Scale

Each system passes its unit tests, each content item passes its validation, and the team concludes the game is solid, but at scale the combination of content reveals memory leaks, balance cliffs, and interaction failures. The trap is that code-level and item-level testing cannot reveal aggregate failures, which only appear when many items coexist. The false signal is that every component passes in isolation. The harm is ship-time discoveries of scale failures that require broad rebalancing or optimization, when there is no time left to do it properly.

### Assuming the Boundary of Retesting Is Obvious

The team retests "around" a change without defining what around means, and regressions escape the undefined boundary because nobody agreed where retesting stopped. The trap is that an implicit boundary is really no boundary, and the scope of regression drifts based on who is available and how much time remains. The false signal is that retesting occurred. The harm is inconsistent coverage where some changes get broad regression and others get none, and the regressions slip through the gaps.

## Self-Check

- Is regression coverage driven by the dependency graph, so that a change triggers retests of every system that reads its state or chains from its output?
- Are test cases designed at system boundaries, state transitions, and variable extremes rather than concentrated in the comfortable middle of valid ranges?
- For every system with asynchronous events, network communication, or shared state, are there test cases exercising timing, concurrency, and mid-transition interruption?
- Is the retest scope defined as an explicit boundary around the changed feature and its neighborhood, with the accepted risk beyond that boundary stated consciously?
- Are the highest-value regression cases automated for every build, while volatile and exploratory testing remains human, avoiding both over- and under-automation?
- Does the test plan cover failure and recovery paths, verifying the system degrades gracefully when connections drop, saves fail, and resources run out?
- Is content tested at scale, in aggregate combinations, rather than only in isolation, to catch the interaction failures that emerge only when many items coexist?
