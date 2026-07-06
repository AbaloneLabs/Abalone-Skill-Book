---
name: team_capacity_assessment.md
description: Use when the agent is assessing how much work a team can realistically deliver, estimating team capacity accounting for overhead and interruptions, sizing work against available capacity, or avoiding the chronic over-commitment that comes from planning at theoretical rather than effective capacity.
---

# Team Capacity Assessment

Capacity assessment is the discipline of estimating how much work a team can realistically deliver in a period, accounting for everything that consumes time besides planned feature work. It is not counting heads and multiplying by hours. Real capacity is far lower than theoretical capacity, because meetings, support, incidents, code review, planning, context switching, onboarding, vacations, and the steady flow of unplanned work consume a large fraction of every team's time. Done well, capacity assessment produces estimates the team can actually hit, which builds trust and enables realistic planning. Done poorly, it produces chronic over-commitment, missed plans, and burnout, because the plan assumed capacity that never existed. Agents often estimate capacity by summing ideal hours, ignoring the gap between theoretical and effective capacity that every operating team experiences.

The harm this skill prevents is the planning failure loop. A team that plans against theoretical capacity commits to more than it can deliver, misses, and is then pressured to commit to even more to catch up, missing again. The cycle erodes trust, quality, and morale. Honest capacity assessment is the foundation of plans that are both ambitious and achievable.

Use this skill before answering questions such as "how much can the team deliver", "what is our capacity", "why do we always miss our plan", or "how do we estimate what we can commit to". The goal is to prevent the agent from planning against theoretical capacity and setting the team up to fail.

## Core Rules

### Base Capacity On Historical Delivery, Not Theoretical Hours

The most reliable predictor of how much a team will deliver is how much it has delivered in similar past periods, not a calculation of available hours. Historical delivery already incorporates all the overhead, interruptions, and friction that reduce theoretical capacity, so it reflects the real operating rate. Use velocity or throughput history as the starting point for capacity assessment, and adjust for known changes in team size, composition, or context.

When historical data is unavailable, because the team is new or the work is unfamiliar, be conservative and refine the estimate as the team establishes a track record. Do not substitute optimistic theoretical calculations for missing history; they will overestimate capacity every time. The first few periods of a new team should be treated as calibration, with commitments held deliberately low until the real rate becomes clear.

### Account For Overhead And Unplanned Work Explicitly

A large fraction of team time is consumed by work that is not planned feature delivery: meetings, support, incident response, code review, planning and refinement, bug fixing, technical debt maintenance, onboarding, and administrative tasks. Additionally, unplanned work arrives continuously, and if no capacity is reserved for it, it will displace planned work and break the plan. Estimate the historical rate of overhead and unplanned work, and subtract it from theoretical capacity to arrive at effective capacity for planned work.

Be honest about overhead rather than pretending it does not exist. A team that spends forty percent of its time on meetings and support has sixty percent available for planned work, not one hundred percent. Planning as if the full capacity were available guarantees that planned work is displaced by the overhead that was never accounted for.

### Adjust For Team Composition And Context Factors

Capacity is not fixed; it changes with team composition and context. A team with new members who are ramping up has lower effective capacity, because onboarding consumes both the new members' time and the experienced members' time who mentor them. A team in the middle of a migration or a major incident has reduced capacity for new feature work. A team that has just lost members or is anticipating turnover is destabilized. Adjust capacity estimates for these factors rather than treating headcount as a static multiplier.

Also account for the compounding effect of context switching. A team split across many initiatives delivers less total than a team focused on few, because switching between contexts has a cost. The more parallel work, the lower the effective capacity, even at the same headcount. Sequencing to reduce context switching effectively increases capacity.

### Distinguish Effective Capacity From Theoretical Capacity In All Planning

The chronic planning error is presenting theoretical capacity, the sum of all team hours, as if it were available for planned work. Effective capacity, what is actually available for planned feature delivery after overhead and unplanned work, is always lower, often dramatically so. Plans must be built against effective capacity, and the distinction must be visible to stakeholders so that commitments reflect reality.

When stakeholders demand more than effective capacity allows, the response is a visible tradeoff: either reduce scope, extend timeline, add resources, or accept that the plan will miss. Silently accepting demands that exceed effective capacity produces a plan that fails by construction. Making the capacity constraint visible is how the team protects its commitments.

### Account For Non-Uniform Capacity Across Skills And Roles

Team capacity is not a single number; it is a set of capacities by skill and role. A team may have sufficient engineering capacity but insufficient design capacity, creating a bottleneck in design even when overall capacity appears adequate. Assess capacity by the constrained roles and skills, not only in aggregate, because the bottleneck determines what can actually be delivered. A plan that fits aggregate capacity but overloads the bottleneck role will stall at the bottleneck.

Map the work to the roles and skills it requires, and check capacity for each. Where a role is the bottleneck, the plan is constrained by that role's capacity, regardless of how much surplus exists elsewhere. Capacity planning that ignores role-specific constraints systematically overestimates what can be delivered.

### Build In Buffer For Uncertainty And Protect It

Even effective-capacity estimates carry uncertainty: work may prove harder than expected, more unplanned work may arrive, and team members may become unavailable. Build in buffer to absorb this uncertainty, and protect the buffer from being consumed by additional commitments. A plan at one hundred percent of effective capacity has no room for the inevitable surprises and will miss when they arrive. A plan at eighty to eighty-five percent has room to absorb normal variation and hit its commitments.

The buffer is not slack to be filled with more commitments; it is the reserve that makes the plan robust. Protecting it requires discipline, because the temptation is always to add one more item since capacity appears unused. The buffer's value is realized when the unexpected arrives and the plan still holds.

## Common Traps

### Theoretical Capacity Treated As Available Capacity

Planning against summed hours without subtracting overhead. The trap is chronic over-commitment and missed plans.

### Ignoring Historical Delivery Rate

Substituting optimistic calculations for actual track record. The trap is estimates that bear no relationship to what the team can do.

### No Reservation For Unplanned Work

Assuming all capacity goes to planned features. The trap is planned work displaced by the unplanned work that inevitably arrives.

### Ignoring Role-Specific Bottlenecks

Planning in aggregate while one role is overloaded. The trap is a plan that stalls at the bottleneck despite appearing feasible overall.

### Filling The Buffer With More Commitments

Treating reserve capacity as available for additional scope. The trap is a plan with no resilience, that misses whenever normal variation arrives.

### Not Adjusting For Team Changes

Treating headcount as a static capacity multiplier. The trap is estimates that ignore onboarding, turnover, and context-switching costs.

## Self-Check

- [ ] Capacity estimates are based on historical delivery rate, not theoretical hour calculations.
- [ ] Overhead, meetings, support, and unplanned work are subtracted explicitly to arrive at effective capacity.
- [ ] Capacity is adjusted for team composition changes, onboarding, turnover, and context-switching costs.
- [ ] Plans are built against effective capacity, with the distinction from theoretical capacity visible to stakeholders.
- [ ] Capacity is assessed by role and skill, with the bottleneck role's capacity as the real constraint.
- [ ] Buffer is built in for uncertainty and protected from being consumed by additional commitments.
- [ ] When demands exceed effective capacity, the tradeoff is made visible rather than silently accepted.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
