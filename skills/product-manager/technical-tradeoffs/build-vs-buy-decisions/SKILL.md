---
name: build_vs_buy_decisions.md
description: Use when the agent is deciding whether to build a feature internally or buy, license, or integrate a third-party solution, evaluating vendors, weighing total cost of ownership, or assessing the strategic and operational tradeoffs of relying on external dependencies.
---

# Build Vs Buy Decisions

The build-versus-buy decision is one of the most consequential and most frequently botched choices a product team makes. The common failure is reasoning from a single dimension. Teams build because engineering wants to, framing buy as a lack of craft. Or they buy because a vendor demo looked impressive, framing build as reinventing the wheel. Both framings collapse a multidimensional decision into a slogan, and both lead to predictable regret: built solutions that drain years of maintenance for a non-core capability, and bought solutions that lock the product into a dependency that diverges from its needs.

A sound build-versus-buy decision evaluates the capability against strategic importance, total cost, time to value, differentiation, risk, and the long-term implications of each path. It recognizes that the decision is rarely permanent and that the right answer can change as the product and market evolve. The product manager's job is to make the tradeoffs explicit, resist the pull of default biases, and choose based on what serves the product and business over the relevant horizon.

Use this skill before deciding to build or buy a capability, before evaluating vendors, before committing to a dependency, or when reassessing a past decision that no longer fits. Ask: is this capability core to differentiation, or commodity infrastructure? Have I counted the full lifecycle cost of each option, or only the upfront build cost? What dependencies, risks, and lock-in does each path create? Am I choosing based on evidence, or on a team preference dressed as analysis?

## Core Rules

### Assess Whether The Capability Is Core Or Commodity

The first and most important question is whether the capability differentiates the product or is commodity infrastructure. Core capabilities, the ones that make your product better than alternatives in ways users value, are usually worth building, because controlling them lets you compete. Commodity capabilities, the ones every product needs but where being marginally better confers little advantage, are usually better bought, because building them diverts scarce capacity from the work that matters.

Be honest about which is which. Teams overestimate how many capabilities are core, because every feature feels essential to the people building it. A useful test: would a user choose your product over a competitor because of how this capability works? If yes, it may be core. If the capability is table-stakes that users expect but do not choose on, it is likely commodity, and buying a mature solution frees the team to differentiate elsewhere. Misclassifying commodity as core leads to years of maintenance for no competitive gain.

### Count Total Cost Of Ownership, Not Just Build Cost

The cost comparison between build and buy is usually done badly, because build cost is counted upfront and incompletely, while buy cost is counted as the license fee. A fair comparison counts total cost of ownership over a realistic horizon for both options. For build, this includes not only initial development but ongoing maintenance, bug fixing, scaling, security, compatibility, and the opportunity cost of the team not doing other work. For buy, it includes license or usage fees, integration cost, configuration, ongoing vendor management, and the cost of working around the vendor's limitations.

Make the comparison explicit and honest. A build that looks cheaper in month one often costs more over three years once maintenance accumulates. A buy that looks expensive in license fees may be cheaper once the avoided maintenance and faster time to value are counted. Neither direction is universally cheaper; the answer depends on the capability, the team, and the horizon. Refuse to let the comparison be decided by whichever cost is easiest to measure, which is usually the upfront build estimate.

### Weigh Time To Value Honestly

Time to value often favors buy, because a vendor solution can be integrated in weeks while a build takes months or years. When the capability is needed soon to validate a hypothesis, meet a commitment, or capture a market window, the speed of buy can outweigh its other costs. When the capability is foundational and the team has time to build it well, the slower path may be worth it for the control and fit it provides.

Be explicit about the time pressure and what it is worth. If shipping the capability in two months unlocks revenue or learning that a ten-month build would miss, that is a real factor, not a shortcut. But do not let time pressure become a permanent rationale for buying everything, because accumulated dependencies have their own long-term costs. Distinguish between time pressure that is real and strategic, and time pressure that is manufactured by default urgency.

### Evaluate Differentiation And Fit

A bought solution imposes its model on your product. The vendor designed it for a general market, and your needs are specific. Where your needs diverge from the vendor's assumptions, you either accept the misfit, work around it with customization that negates the buy benefit, or wait for the vendor to prioritize your request. For capabilities close to the user experience or the core value proposition, this misfit can damage the product.

Assess fit before buying. How closely does the vendor's model match your users' needs and your product's conventions? Where it diverges, can you live with the gap, or will it create friction users feel? For commodity capabilities far from the core, some misfit is tolerable. For capabilities users interact with directly, misfit is costly. The more a capability shapes the user experience, the stronger the case for building it to fit, even at higher cost.

### Map Dependencies, Risks, And Lock-In

Every buy creates a dependency, and dependencies carry risk. The vendor may raise prices, change direction, degrade quality, get acquired, or shut down. The integration may prove harder than expected. The vendor's roadmap may diverge from your needs. Security or compliance obligations may attach to the third-party code. These risks are not reasons to never buy, but they must be counted in the decision and managed after it.

For each buy option, assess the dependency risk and the lock-in it creates. How hard would it be to replace the vendor if needed? What data, workflows, or user expectations would be disrupted? Is there a viable alternative, or does the integration become a one-way door? For capabilities where switching would be catastrophic, weigh the lock-in heavily, and prefer options with portability, export, or multiple vendors. For capabilities where switching is feasible, lock-in matters less. Treat dependency risk as a first-class factor, not an afterthought.

### Consider The Team's Capacity And Competence

The build option is only viable if the team has the capacity and the competence to build and maintain the capability well. A build assigned to a team without the relevant expertise produces a fragile solution that becomes a long-term burden. A build that competes with higher-priority work delays that work. Honest assessment of the team's capacity and skills is essential, because the build option's cost and risk depend entirely on who will do it and what else they will not do.

Assess whether the team can build this capability to an acceptable standard without sacrificing more important work. If the capability requires expertise the team lacks, the build cost and risk rise sharply, and buy becomes more attractive. If the team has deep relevant expertise and available capacity, build becomes more feasible. Do not let enthusiasm for building override a realistic assessment of whether the team can deliver and sustain the capability.

### Make The Decision Reversible Where Possible

Some build-versus-buy decisions are one-way doors: once integrated deeply, a vendor is hard to remove, or once built, a custom system is hard to replace. Others are two-way doors: a vendor can be swapped, or a build can be replaced, without catastrophic disruption. The reversibility should influence how much analysis and caution the decision warrants.

For one-way doors, invest in thorough analysis, because the cost of a wrong choice is high and persistent. For two-way doors, decide more quickly, accept some risk, and plan to revisit. Where possible, design the choice to be reversible: choose vendors with export and portability, build with abstraction layers that allow swapping components, and avoid deep integration before the capability's long-term role is clear. Reversibility reduces the stakes and lets the team learn before committing fully.

## Common Traps

### Building To Satisfy Engineering Preference

Choosing build because the team finds it more interesting, framed as craftsmanship. The trap is years of maintenance for a non-differentiating capability.

### Buying Because The Demo Was Impressive

Choosing buy based on a polished demo without assessing fit, cost, or lock-in. The trap is a dependency that diverges from the product's needs.

### Counting Only Upfront Build Cost

Comparing build's initial estimate to buy's license fee, ignoring maintenance and opportunity cost. The trap is a build that looks cheap and costs dearly over time.

### Treating Commodity As Core

Overestimating how many capabilities differentiate the product. The trap is scarce capacity spent on table-stakes instead of competitive advantage.

### Ignoring Dependency Risk

Buying without assessing what happens if the vendor raises prices, degrades, or disappears. The trap is a dependency that becomes a liability.

### Permanent Urgency As Rationale For Buy

Using time pressure, real or manufactured, to justify buying everything. The trap is accumulated dependencies and eroded core competence.

## Self-Check

- [ ] The capability was classified as core (differentiating) or commodity (table-stakes), with honest reasoning rather than defaulting to core.
- [ ] Total cost of ownership over a realistic horizon was compared for both build and buy, including maintenance, opportunity cost, integration, and vendor management.
- [ ] Time to value was weighed explicitly, distinguishing real strategic urgency from manufactured default urgency.
- [ ] The fit of the vendor's model to the product's needs and user experience was assessed, especially for user-facing capabilities.
- [ ] Dependency, lock-in, switching cost, and vendor risk were mapped and counted in the decision.
- [ ] The team's capacity and competence to build and maintain the capability were honestly assessed.
- [ ] The reversibility of the decision was considered, and one-way doors received more analysis than two-way doors.
- [ ] The decision was made on evidence across multiple dimensions, not on a single factor or team preference.
- [ ] A plan exists to manage the chosen dependency or maintain the chosen build over time.
- [ ] The decision includes a review point, recognizing that the right answer can change as the product and market evolve.
