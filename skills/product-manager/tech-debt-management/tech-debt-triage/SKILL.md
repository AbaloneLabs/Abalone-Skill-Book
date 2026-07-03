---
name: tech_debt_triage.md
description: Use when the agent is triaging technical debt, categorizing debt items, assessing severity and impact, deciding which debt to address now versus later, or distinguishing debt that poses real risk from debt that is merely untidy.
---

# Tech Debt Triage

Technical debt is inevitable in any product built under real constraints, and the question is never whether to have it but how to manage it. The two failure modes are symmetric and equally damaging. The first is ignoring debt entirely, letting it accumulate until velocity collapses, bugs multiply, and every change becomes hazardous. The second is treating all debt as equally urgent, flooding the roadmap with refactors that deliver no user or business value while important features wait. Both fail because they refuse to distinguish debt that matters from debt that does not.

Tech debt triage is the discipline of categorizing debt, assessing its real impact, and deciding deliberately what to address, when, and how much. It requires the product manager to engage with technical reality without deferring entirely to engineering, and to weigh debt against feature work using a shared understanding of risk and value. A team that triages debt well sustains velocity over years; one that does not either drowns in debt or starves the product of new value.

Use this skill before triaging a backlog of debt items, before deciding whether to address a specific debt, before balancing debt work against feature work, or when velocity has slowed and debt is suspected. Ask: is this debt causing real harm now, or is it a theoretical concern? What is the cost of leaving it, in velocity, risk, and morale? Is the proposed fix proportional to the harm, or is it gold-plating? Am I distinguishing debt that blocks the product from debt that merely bothers engineers?

## Core Rules

### Categorize Debt By Nature And Origin

Not all debt is the same, and the category determines how it should be handled. Reckless debt was taken knowingly, often to hit a deadline, and is a deliberate tradeoff that should be paid down on a schedule. Prudent debt was taken knowingly because shipping faster was the right call, and is the kind of debt that enabled value delivery. Accidental debt arose from decisions that seemed right at the time but aged poorly, such as an abstraction that did not hold. Bit-rot debt emerged as the environment changed around stable code, such as dependencies that became unsupported.

Categorize each debt item by how it arose. This matters because the response differs. Prudent debt taken for a good reason should be paid down on a planned cadence, not in a panic. Reckless debt taken under pressure signals a process problem to fix, not just a code problem. Accidental debt may require redesign, not just cleanup. Bit-rot debt is ongoing maintenance, not a one-time fix. Treating all debt the same obscures these differences and leads to uniform, often wrong, responses.

### Assess Real Impact, Not Just Aesthetics

Engineers are sensitive to code quality, and thank goodness, but not every imperfection they perceive is debt that matters. The critical triage question is what harm the debt is actually causing. Harm manifests as slowed velocity, when changes take longer because the code resists them; increased bugs, when the debt makes correct behavior hard to maintain; higher risk, when the debt makes outages or data loss more likely; blocked work, when a desired feature cannot be built on the current foundation; and morale damage, when working in the codebase frustrates the team.

For each debt item, identify the concrete harm. If the debt slows every change to a critical area, that is high impact. If the debt is ugly but changes rarely and behaves correctly, the impact is low. Be honest about the difference. Triage that treats every imperfection as urgent overwhelms the roadmap; triage that recognizes which debt actually harms focuses effort where it pays off. The product manager's role is to insist on impact evidence, not just engineer discomfort, while respecting that morale is itself a legitimate impact.

### Distinguish Blocking Debt From Background Debt

Some debt blocks the product: it prevents building a desired feature, makes a critical area unsafe to change, or causes recurring incidents. This debt demands attention because its cost is ongoing and compounding, and because it directly constrains product progress. Other debt is background: it makes the code less pleasant or slightly slower to work in, but it does not block anything specific and behaves correctly. Background debt can be tolerated, paid down opportunistically, or scheduled at low priority.

Separate blocking from background debt, and prioritize blocking debt into the roadmap as a first-class concern. Blocking debt that is left in place continues to tax every related feature and to generate risk; paying it down unblocks future work and reduces incidents, which is a product investment, not a distraction from product work. Background debt, by contrast, rarely warrants dedicated effort and is better addressed when the team is already working in the area for other reasons.

### Evaluate The Fix Against The Harm

A common failure is proposing fixes that far exceed the harm the debt causes. A two-week refactor to address debt that slightly slows an area touched twice a year is disproportionate. A rewrite of a working system to satisfy architectural preferences, when the system behaves correctly and changes rarely, is gold-plating. The fix must be justified by the harm it removes, measured against the opportunity cost of the team's capacity.

For each proposed debt fix, compare the cost of the fix to the cost of the harm it addresses. If the fix costs more than the harm it removes over a reasonable horizon, it is not justified, unless there are strategic reasons like preparing for known future work. Prefer incremental fixes that address the specific harm over grand rewrites that promise cleanliness. The goal is not a pristine codebase; it is a codebase that sustains the product's velocity and safety at acceptable cost.

### Pay Down Debt Opportunistically

Not all debt work needs dedicated roadmap time. Much of it can be addressed opportunistically, when the team is already working in the affected area for feature reasons. If a feature touches a debt-laden component, addressing the nearby debt as part of that work is often cheap, because the context is loaded and the changes are being made anyway. This spreads debt paydown across feature work without requiring constant dedicated sprints.

Build opportunistic paydown into the culture: when working in an area, leave it a little better than you found it. This does not mean scope-creeping every feature into a refactor, but it does mean addressing the debt that is cheaply in reach. Combined with targeted dedicated effort on blocking debt, opportunistic paydown keeps debt manageable without starving feature work. The balance matters: too little opportunistic paydown and debt grows; too much and features never ship.

### Make Debt Visible And Measured

Debt that is invisible cannot be managed. Teams that rely on engineer memory and hallway complaints have no real picture of their debt, its severity, or its trend. Making debt visible requires a lightweight system: a backlog of debt items with category, impact, affected area, and proposed fix. It does not need to be elaborate, but it needs to exist and to be maintained.

Track debt and review it regularly, the way feature work is reviewed. Measure trend: is debt growing, stable, or shrinking? Are blocking items being addressed, or accumulating? A visible, measured debt backlog lets the team make informed tradeoffs and lets leadership see why velocity is what it is. Without visibility, debt decisions are made by whoever complains loudest, which is neither fair nor effective.

### Negotiate Debt Capacity Deliberately

The question of how much capacity goes to debt versus features is a negotiation, and it should be an explicit one, not a default. Some teams default to zero debt work and drown; others default to constant refactoring and ship nothing. The right balance depends on the debt level, the product priorities, and the team's situation, and it should be revisited as those change.

Agree on a debt capacity allocation, such as a percentage of each cycle or a dedicated debt cycle periodically, and adjust it based on debt trend and product needs. When debt is high and blocking, raise the allocation. When debt is manageable, lower it and focus on features. The negotiation should involve both product and engineering, because neither side alone has the full picture. Product understands the feature pressure; engineering understands the debt risk. The decision belongs to both.

## Common Traps

### Ignoring Debt Until Velocity Collapses

Letting debt accumulate without triage until the product becomes hard to change. The trap is a crisis that demands far more disruptive effort than gradual paydown would have.

### Treating All Debt As Urgent

Flooding the roadmap with refactors regardless of impact. The trap is starving the product of new value while cleaning code that was not causing harm.

### Confusing Aesthetics With Impact

Pursuing clean code for its own sake when the debt behaves correctly and changes rarely. The trap is effort that feels productive but delivers no value.

### Disproportionate Fixes

Proposing rewrites that exceed the harm they address. The trap is gold-plating that delays real work.

### Invisible Debt

Relying on memory and complaints instead of a tracked backlog. The trap is decisions driven by volume rather than evidence.

### Default Capacity Allocations

Letting the debt-versus-feature balance be set by habit rather than negotiation. The trap is an allocation that fits no one's actual situation.

## Self-Check

- [ ] Each debt item was categorized by nature and origin (reckless, prudent, accidental, bit-rot), with the response matched to the category.
- [ ] Real impact was assessed for each item: velocity drag, bug rate, risk, blocked work, and morale, not just engineer discomfort.
- [ ] Blocking debt, which constrains product progress or causes incidents, was separated from background debt and prioritized.
- [ ] Each proposed fix was evaluated against the harm it addresses, and disproportionate or gold-plated fixes were rejected.
- [ ] Opportunistic paydown is part of the culture, addressing nearby debt when already working in an area.
- [ ] Debt is tracked in a visible backlog with category, impact, area, and proposed fix, and reviewed regularly.
- [ ] Debt capacity allocation is negotiated explicitly between product and engineering and adjusted based on trend.
- [ ] The debt trend is measured, and the allocation responds to whether debt is growing, stable, or shrinking.
- [ ] Incremental fixes targeting specific harm are preferred over grand rewrites promising cleanliness.
- [ ] The triage distinguishes debt that harms the product from debt that merely bothers engineers, while acknowledging morale as a legitimate factor.
