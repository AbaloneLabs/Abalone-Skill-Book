---
name: assumption_mapping_and_prioritization.md
description: Use when the agent is mapping product assumptions, identifying which assumptions are riskiest, sequencing which belief to test first, or deciding where uncertainty is highest and most consequential before building.
---

# Assumption Mapping And Prioritization

Every product bet rests on assumptions, beliefs about users, markets, technology, or behavior that must be true for the product to succeed. Most of these assumptions are never made explicit, and the team builds as if all of them were equally certain. The ones that actually kill products are the ones no one examined until it was too late.

The judgment problem is that teams test the assumptions that are easy to test rather than the ones that matter, and they build on the assumptions that feel familiar rather than the ones that are uncertain. A comfortable assumption that users want the feature gets validated repeatedly, while the lethal assumption that users will pay, or that the unit economics work, or that the distribution channel exists, goes untested until launch. Agents tend to list assumptions superficially, rank them by gut, and then test in an order driven by convenience or enthusiasm rather than by risk. The harm is products that ship having de-risked the easy parts and left the fatal parts untouched, and teams that are surprised by failure in areas they assumed were settled. The discipline of assumption mapping exists to force the lethal assumptions to the surface and to test them first, before the investment makes them too expensive to abandon.

Use this skill before committing to a build, when sequencing discovery work, and when deciding what to test first. The goal is to surface every load-bearing belief, rank by risk and consequence, and resolve the riskiest assumption before any other.

## Core Rules

### Surface Every Load-Bearing Assumption Explicitly

Before prioritizing, you must enumerate the assumptions the product depends on, because you cannot prioritize what you have not named. Most teams hold these assumptions implicitly and never write them down, which guarantees the most dangerous ones stay invisible. The act of listing is the first de-risking.

Walk the product concept end to end and extract the beliefs it requires: desirability assumptions about whether users want this, viability assumptions about whether the business works, feasibility assumptions about whether it can be built, and usability assumptions about whether users can succeed with it. Write each as a single statement that must be true for the product to work. Aim for completeness over elegance; a long list you can prioritize beats a short list that omits the killer.

### Rank By Risk And Consequence, Not By Familiarity

The right ordering for testing is risk and consequence: how uncertain is the assumption, and how damaging is it if it is wrong. An assumption that is both highly uncertain and highly consequential is the riskiest and must be tested first. An assumption that is certain or low-consequence can wait.

Use a simple two-axis view. Uncertainty is how little evidence you have; consequence is how much failure of this assumption breaks the product. The riskiest assumptions sit in the high-uncertainty, high-consequence corner. Resist ranking by what the team is good at testing or by what feels exciting; the goal is to attack the assumptions whose failure would kill the investment, even when those are the uncomfortable ones.

### Apply The Riskiest-Assumption-First Principle

The riskiest assumption should be tested before any build that depends on it, because building on an untested lethal assumption commits resources to a path that may be invalid. Resolving the riskiest assumption first is the single highest-leverage move in product discovery, because it prevents the largest possible waste.

Ask, of all the assumptions, which one, if false, makes the rest irrelevant. That is the assumption to test first, with the cheapest possible method. If users do not have the problem, no amount of feature work matters; if they will not pay, no amount of polish fixes the business. Sequence so that each test either removes a lethal assumption or reveals the next one to attack.

### Separate Desirability, Viability, Feasibility, And Usability

Different assumption types fail differently and need different tests, and conflating them muddies the prioritization. Desirability assumptions, does the user want this, are tested with users. Viability assumptions, does the business work, are tested with economics and market data. Feasibility assumptions, can we build it, are tested with engineering. Usability assumptions, can users succeed, are tested with prototypes.

Label each assumption by type so the team can see where the risk concentrates. A product can have strong desirability evidence but lethal viability risk, and treating the two as one bucket hides that. Surfacing the type also points to the right test method for each, which prevents testing a viability assumption with a usability prototype and getting a meaningless answer.

### Make The Cost Of Being Wrong Explicit

Consequence is not just whether the assumption matters, but how much it costs to be wrong, in time, money, and strategic position. An assumption whose failure costs a week is lower priority than one whose failure costs a year, even if both are uncertain. Quantifying the cost sharpens the ranking and makes the tradeoff legible to stakeholders.

For each assumption, estimate what is lost if it is false and discovered late: sunk build cost, opportunity cost, reputational risk, or strategic delay. The cost-of-wrong converts a fuzzy sense of importance into a number that can be compared. Assumptions with high cost-of-wrong deserve testing even when uncertainty is moderate, because the downside is severe.

### Sequence Tests To Compound Learning

Testing in the right order compounds learning, because early tests reshape which later assumptions even matter. If the riskiest assumption fails, several downstream assumptions become irrelevant and need no testing. If it succeeds, the next riskiest moves to the front. Sequencing is dynamic, not a fixed roadmap.

After each test, re-rank the remaining assumptions in light of what was learned. An assumption that was medium-risk may become lethal once a related assumption is confirmed, or may disappear entirely. Treat the assumption map as a living artifact that is re-prioritized after every test, not a one-time exercise.

### Match The Test Cost To The Assumption's Risk

The amount spent testing an assumption should be proportional to its risk, not uniform across all assumptions. A lethal, uncertain assumption deserves a real, well-resourced test. A low-risk assumption deserves the cheapest check that can resolve it, or no test at all if the cost of being wrong is small.

Resist the symmetry of testing every assumption with equal effort. Over-testing low-risk assumptions wastes time; under-testing high-risk assumptions courts disaster. Allocate test effort by risk, and accept that some assumptions will be validated by a five-minute check while others warrant a multi-week experiment.

## Common Traps

### Testing What Is Easy Instead Of What Matters

Teams gravitate to assumptions they know how to test, such as usability, and avoid the hard ones, such as willingness to pay. The trap is a product de-risked in the easy dimensions that fails in the dimension no one examined. Prioritize by risk, not by testing convenience.

### Keeping Assumptions Implicit

Assumptions that are never written down cannot be prioritized, and the lethal ones stay hidden. The trap is building on beliefs no one articulated, then being surprised when one was false. Surface every load-bearing assumption explicitly.

### Ranking By Enthusiasm Or Familiarity

The assumptions the team is excited about get tested first, regardless of risk. The trap is sequencing driven by preference rather than by where uncertainty and consequence concentrate. Rank by risk and cost-of-wrong.

### Ignoring Viability And Feasibility Assumptions

Desirability gets all the attention because it involves users, while the business and engineering assumptions go untested. The trap is a desirable product that cannot make money or cannot be built. Test across all four types.

### Treating The Assumption Map As Fixed

Once mapped and ranked, the list is filed and never revisited. The trap is that early tests change which later assumptions matter, and a stale map tests the wrong things. Re-rank after every test.

### Uniform Test Effort Across All Assumptions

Spending the same effort on every assumption wastes time on low-risk beliefs and under-invests in lethal ones. The trap is a discovery process that is thorough in effort but misallocated in impact. Match test cost to risk.

### Confirming Comfortable Assumptions Repeatedly

The assumption that users want the feature gets validated again and again because positive results feel good. The trap is re-confirming what is settled while the unsettled lethal assumption waits. Move on once an assumption is sufficiently de-risked.

## Self-Check

- [ ] Every load-bearing assumption is written down explicitly, covering desirability, viability, feasibility, and usability.
- [ ] Assumptions are ranked by uncertainty and consequence, not by familiarity, enthusiasm, or testing ease.
- [ ] The single riskiest assumption, the one whose falsity makes the rest irrelevant, is identified and tested first.
- [ ] Each assumption is labeled by type so the team can see where risk concentrates and choose the right test method.
- [ ] The cost of being wrong is estimated for each assumption and feeds the ranking.
- [ ] Tests are sequenced so early results reshape which later assumptions matter, and the map is re-ranked after each test.
- [ ] Test effort is proportional to each assumption's risk, not uniform across all assumptions.
- [ ] Comfortable assumptions are not re-confirmed repeatedly while lethal assumptions remain untested.
