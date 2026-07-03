---
name: make_vs_buy_vs_partner.md
description: Use when the agent is deciding whether to build a capability internally, buy or license it, partner with another company, or use an open-source or third-party solution, weighing strategic, cost, risk, and time tradeoffs.
---

# Make Vs Buy Vs Partner

The make-versus-buy-versus-partner decision determines where a product's capabilities come from and, over time, what kind of company it becomes. It is not a procurement question. It is a strategic question about which capabilities are core to differentiation, which are context that should be borrowed, and which tradeoffs between time, cost, control, and risk the business is willing to accept. Agents often default to building because it feels safer and more impressive, or default to buying because it feels faster, without weighing total cost of ownership, lock-in, vendor viability, or strategic control.

The harm this skill prevents is a capability decision made on instinct that becomes expensive or irreversible. Building the wrong thing wastes scarce engineering capacity and delays the market. Buying the wrong thing creates dependency, lock-in, and switching costs that compound for years. Partnering with the wrong company creates incentive misalignment that surfaces only when stakes are highest. These decisions are hard to reverse, so they deserve explicit reasoning rather than a default.

Use this skill before answering broad questions such as "should we build or buy this", "is this capability core or commodity", "what are the risks of using this vendor", "should we partner with this company", or "when does it make sense to start with a third party and migrate later". The goal is to prevent the agent from recommending a path based on a single dimension such as speed or cost while ignoring control, lock-in, and optionality.

## Core Rules

### Classify Each Capability As Core Or Context

The first question is not how to acquire a capability but whether it differentiates the product. Core capabilities are the ones customers would miss if removed and competitors cannot easily copy; they are where the product earns its position and should usually be built and owned. Context capabilities are necessary but commodity: authentication plumbing, payments rails, email delivery, data infrastructure. Context capabilities should usually be bought, licensed, or adopted from open source so that scarce talent concentrates on core work.

Be honest about the classification. Teams overestimate what is core because building feels more meaningful than integrating. Ask whether this capability, done exceptionally well, would change a customer's choice of product. If not, it is probably context, however much engineers enjoy building it.

### Compare Total Cost Of Ownership, Not Build Cost

Build cost is only the first invoice. A fair comparison includes total cost of ownership across the full life of the capability. For build, that means design, implementation, testing, documentation, ongoing maintenance, bug fixing, security patching, scaling, on-call coverage, and eventual replacement. For buy or partner, it means license or revenue-share fees, integration cost, implementation effort, ongoing configuration, support contracts, upgrade effort, and the cost of working around the vendor's roadmap.

Model the multi-year cost, not the first quarter. A build that looks cheaper in year one can cost more by year three once maintenance accumulates, while a buy that looks expensive upfront can be cheaper once the avoided maintenance is counted. Make the time horizon explicit because it changes the answer.

### Weigh Time-To-Market Against Strategic Control

Buying or partnering usually wins on speed because the capability exists today. Building usually wins on control because you own the roadmap, the data, the quality, and the ability to change direction. The tradeoff is real and must be named rather than resolved by default. A fast-to-market buy that surrenders control over a core capability can win the quarter and lose the company.

Ask how much time pressure is genuine and how much is manufactured urgency. When a deadline is real, buying or partnering to meet it while planning a later build can be the right sequence. When the deadline is arbitrary, sacrificing strategic control to hit it is a poor trade. Separate the two before deciding.

### Evaluate Integration, Lock-In, And Switching Costs

Every external dependency creates integration work and, over time, lock-in. Lock-in is not always bad, but it must be visible. Assess how deeply the capability will be woven into the product, how much proprietary data format or API it imposes, how hard it would be to replace in two years, and what the exit cost would be. Switching cost includes data migration, retraining, reintegration, and the risk that no equivalent alternative exists at acceptable terms.

Prefer dependencies that expose standard interfaces, allow data export, and have credible alternatives. When lock-in is unavoidable, make sure it is compensated by enough strategic value to justify the loss of optionality, and document the decision so the lock-in is conscious rather than accidental.

### Assess Vendor Viability And Support Risk

A dependency is only as reliable as the organization behind it. For any buy or partner option, evaluate vendor viability: financial stability, funding trajectory, customer base, roadmap commitment to this capability, and the likelihood it still supports the product in three years. A cheap dependency on a vendor that folds or sunsets the product becomes an expensive emergency build under duress.

Also assess support quality: responsiveness, escalation paths, security incident handling, documentation, and the health of the user community. Open-source options shift support risk onto your team or onto a community whose responsiveness you do not control. Match the support expectation to how mission-critical the capability is.

### Protect Strategic Control And Data Ownership

Some capabilities carry strategic value beyond their function because they own the customer relationship, the data, or the differentiation. Handing these to a vendor or partner can hollow out the product's long-term advantage. Identify which data, workflows, and customer touchpoints are strategically load-bearing and keep them under your control even when the surrounding capability is bought.

Partnerships are especially risky here because a partner with misaligned incentives may eventually compete with you, withhold data, or change terms once you are dependent. Clarify data ownership, exclusivity, and termination terms up front, because the moment to negotiate control is before the dependency exists, not after.

### Check Partner Alignment And Incentive Compatibility

A partnership works only while both parties' incentives point the same way. Map each party's incentives: revenue, data, brand, customer access, market position. Where incentives diverge, the partnership will eventually strain, because each party will optimize for itself when stakes rise. Incentive compatibility is more predictive of partnership success than contract terms, because contracts are enforced only after damage is done.

Ask what happens to the partner if the partnership succeeds wildly, and what happens if it fails. If the partner's upside depends on your weakness, or if the partner can profit from your dependency, the alignment is fragile. Prefer partners whose success rises with yours.

### Preserve Option Value For Reversible Versus Irreversible Decisions

Not all make-or-buy decisions are equally reversible. Building creates an asset you own and can keep changing; it is relatively reversible in direction. Buying creates integration and lock-in that make reversal expensive; it is less reversible. Partnering can create the deepest lock-in when it involves data, co-branding, or exclusive channels. Weight irreversibility heavily, because the cost of a wrong irreversible decision compounds.

When uncertain, prefer the reversible first move. Starting with a buy or partner to learn fast, then building once the requirements are understood, often beats building speculatively and discovering the requirements were wrong. Conversely, if a build is the only way to protect a core capability, do not delay it behind a temporary buy that creates dependency you will be reluctant to abandon.

## Common Traps

### Defaulting To Build Because It Feels Safer

Engineering teams prefer building because it maximizes control and avoids dependency, and because building is more interesting than integrating. The trap is that this default spends scarce capacity on context capabilities and delays the core work that actually differentiates the product. Always test whether the capability is truly core before building.

### Comparing Only First-Year Cost

A build that is cheaper in year one can be far more expensive once maintenance, on-call, and security burden accumulate. The trap is presenting a narrow cost window that flatters the preferred option. Always model total cost of ownership across the realistic life of the capability.

### Underestimating Integration And Switching Cost

A buy that looks fast can become slow once deep integration, data migration, and workarounds for the vendor's gaps are counted. The trap is treating integration as trivial because the vendor's demo looked easy. Assess switching cost before committing, not after the dependency is entrenched.

### Ignoring Vendor Viability Until It Fails

Choosing the cheapest or most convenient vendor without checking stability creates a ticking risk. The trap is that vendor failure or product sunset converts a cheap dependency into an emergency build under the worst possible timeline. Vet viability for anything mission-critical.

### Surrendering Strategic Control For Speed

Buying or partnering to hit a deadline can feel pragmatic, but if the capability is strategically load-bearing the speed wins the quarter and erodes the foundation. The trap is treating a strategic decision as a tactical one. Separate genuine urgency from manufactured urgency before trading control for time.

### Assuming Incentives Stay Aligned

A partnership that looks aligned at signing can diverge as markets shift or as the partner's strategy changes. The trap is trusting the current alignment to persist without structural protection. Clarify data, exclusivity, and termination terms while leverage exists, and revisit incentive compatibility as conditions change.

### Forgetting The Migration Path

Starting with a buy and planning to build later is often wise, but only if the migration path is real. The trap is treating the temporary solution as permanent because the migration never gets prioritized once the dependency works. Define the triggers and conditions for migration, or accept the buy as the long-term choice deliberately.

## Self-Check

- [ ] Each capability is explicitly classified as core, context, or strategic-control-bearing before the build-or-buy choice is made.
- [ ] The cost comparison uses total cost of ownership across a multi-year horizon, not first-year build cost alone.
- [ ] Time-to-market pressure is separated into genuine versus manufactured urgency before trading control for speed.
- [ ] Integration depth, lock-in, and switching cost are assessed, including data formats, APIs, and exit cost.
- [ ] Vendor or project viability is evaluated for anything mission-critical, including funding, roadmap, and support quality.
- [ ] Strategically load-bearing data, workflows, and customer touchpoints remain under your control even when surrounding capability is bought.
- [ ] Partner incentives are mapped for success and failure scenarios, and incentive compatibility is checked rather than assumed.
- [ ] Reversibility of the decision is explicit, and irreversible choices carry proportionately stronger evidence.
- [ ] Where a buy-to-build migration is planned, the triggers and conditions for migration are defined, not left implicit.
- [ ] The final recommendation names the main risk of the chosen path and the condition under which it should be revisited.
