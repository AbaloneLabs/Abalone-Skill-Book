---
name: real_options_valuation.md
description: Use when the agent is valuing flexibility, optionality, and decision rights such as the option to expand, defer, abandon, or scale a project or investment, valuing patents, undeveloped resources, growth platforms, staged investments, or strategic optionality, deciding when real-options methods add value over NPV, or reviewing the inputs (volatility, decision points) and the risk of overvaluing flexibility with optimistic assumptions.
---

# Real Options Valuation

Real options valuation applies option-pricing logic to real (non-financial) assets and decisions, treating managerial flexibility (the ability to expand, defer, abandon, contract, or switch a project) as a set of options that have value beyond the static net present value (NPV) of expected cash flows. The insight is genuine: a project with a negative NPV today may be valuable if it includes the option to proceed only if conditions improve, and a growth platform that looks speculative may be worth a great deal because it preserves the right (not the obligation) to scale. The judgment problem is that real options are powerful in concept and dangerous in practice. The inputs (volatility of underlying value, the timing and cost of decision points) are hard to estimate for real assets, and option-pricing models are extremely sensitive to them. Agents tend to invoke real options to justify almost any speculative investment, plugging in optimistic volatility and understating the cost and risk of exercise, producing valuations that flatter whatever they wanted to fund anyway. The skill is using real options where flexibility is real and material, with honest inputs, and not as a license to value hope.

This skill is for valuing optionality in real decisions with awareness of where it helps and where it misleads.

## Core Rules

### Identify Whether Genuine Optionality Exists

Real options valuation is only appropriate when there is real, valuable flexibility. Many investments described as "options" are not options at all because the flexibility is illusory, too costly to exercise, or not truly at the holder's discretion.

Confirm the option is real by checking:

- does management genuinely have the right, not the obligation, to act (expand, defer, abandon)?
- is the decision point discrete and identifiable, or is it a continuous drift with no clear trigger?
- is the option exclusive, or can competitors preempt it (a shared option is worth far less)?
- is the flexibility protected by a barrier (patent, lease, contract, first-mover position) that preserves the right until exercise?

A "growth option" that requires immediate commitment to preserve, or that competitors can seize at any time, is not a free option. It is a cost-bearing bet. Reserve real-options methods for cases of genuine, protected, discretionary flexibility.

### Match The Option Type To The Decision

Real options come in several forms, each valuing a different kind of flexibility. Naming the option correctly determines the model.

Common types:

- option to defer (wait and invest only if conditions improve): valuable under uncertainty and irreversibility;
- option to expand (scale up if outcomes are favorable): the classic growth option;
- option to contract or abandon (shut down or sell if outcomes are poor): limits downside;
- option to switch (alternate inputs, outputs, or operating modes): valuable with volatile input or output prices;
- compound options (staged investments, where each stage is an option on the next): typical of R&D and exploration.

Mislabeling the option (treating a switch option as a growth option, or ignoring abandonment value) misstates the result. Identify the actual decision structure before choosing the model.

### Estimate Volatility Honestly And Show Sensitivity

Volatility of the underlying asset's value is the single most important and most uncertain input in real options. Option value rises with volatility, which creates a powerful incentive to plug in optimistic numbers and inflate the valuation.

Estimate volatility from:

- historical volatility of comparable assets, projects, or commodity prices;
- the dispersion of scenario outcomes (a simple proxy);
- market-implied volatility from traded options on related assets, where available;
- expert or simulation-based estimates of the range of project values.

Always show the option value across a range of volatilities. A real-options valuation that collapses if volatility is 20% lower is not robust; it is an artifact of an aggressive assumption. Treat volatility as a wide range, not a point.

### Account For The Cost And Timing Of Exercise

An option's value depends on the exercise price (the cost to act) and the time to the decision. Both are often mis-specified.

Specify:

- the exercise price (capital cost to expand, build, or proceed) and its uncertainty;
- the decision date or window and whether it can be deferred;
- any ongoing cost of holding the option (lease, maintenance, R&D spend to preserve the right);
- the risk that the option expires worthless if not exercised in time (competitive preemption, patent expiry).

An option with a high holding cost or a short, uncertain window is worth far less than a textbook option. The cost of preserving flexibility is a real drag that agents frequently omit.

### Do Not Use Real Options To Justify Negative-NPV Speculation

The most common abuse is invoking real options to transform a bad project into a good one. A negative-NPV project with a vague "growth option" attached is usually still a bad project.

Guard against this by:

- valuing the base NPV honestly first, without the optionality;
- valuing the option separately and transparently, with its own inputs;
- requiring the option value to exceed the negative NPV by a clear margin, not a hair;
- being skeptical when the option value conveniently flips the project from reject to accept.

If the optionality is the only thing making the project look good, scrutinize the option inputs doubly hard. Real options should refine decisions at the margin, not rescue projects that fail on fundamentals.

### Recognize The Limits Of The Models

Option-pricing models were built for traded financial options with continuous, frictionless markets. Real assets violate most of those assumptions, and the models are approximations at best.

Acknowledge:

- real assets are not continuously tradeable, so replication and arbitrage arguments break down;
- the underlying value is often unobservable and estimated, not market-priced;
- decision rules are discrete and path-dependent, not the smooth exercise of financial options;
- model risk is high: small input changes produce large value changes.

Use decision-tree and scenario methods alongside closed-form or lattice models, and treat the real-options value as one input among several, not a definitive number. Where the model says an option is worth a fortune but intuition says the project is marginal, trust the structural skepticism.

### Combine Real Options With Traditional Valuation

Real options complement, they do not replace, NPV and scenario analysis. The strongest analysis integrates all three.

Integrate:

- base-case NPV as the anchor;
- scenario and decision-tree analysis to map the range of outcomes and decision points;
- real-options value to capture flexibility that NPV and even decision trees underweight;
- a sanity check that the total value (NPV plus optionality) is plausible relative to market comparables.

A valuation that rests entirely on real-options value, with no NPV or market support, is speculative. Real options are most credible as an adjustment to a sound fundamental valuation.

## Common Traps

### Labeling Any Speculative Investment A Growth Option

A vague growth story is not an option. Real options require genuine, protected, discretionary flexibility with identifiable decision points.

### Plugging In Optimistic Volatility To Inflate Value

Option value rises with volatility, so aggressive volatility assumptions manufacture value. Always show sensitivity across a range.

### Ignoring The Cost Of Holding The Option

Preserving flexibility costs money (lease, R&D, maintenance). Omitting the holding cost overstates the option value.

### Treating Shared Options As Exclusive

An option competitors can preempt is worth far less than an exclusive one. Exclusivity and barriers matter.

### Using Real Options To Rescue Negative-NPV Projects

Invoking optionality to flip a bad project to good is the classic abuse. Real options should refine marginal decisions, not justify speculation.

### Over-Trading Model Precision For Poor Inputs

Lattice and closed-form models imply precision the inputs do not support. Model risk is high for real assets.

### Forgetting That Real Assets Are Not Tradeable

Financial option models assume continuous markets and replication. Real assets violate these, so the models are rough approximations.

### Confusing Decision Trees With Real Options

A decision tree maps choices; real options value flexibility formally. They overlap but are not identical, and each has blind spots.

## Self-Check

- [ ] The flexibility being valued is genuine, discretionary, protected by a barrier, and not freely preemptable by competitors, not merely a speculative label.
- [ ] The option type (defer, expand, abandon, contract, switch, compound) is correctly identified and matched to the actual decision structure.
- [ ] Volatility is estimated from comparable data or scenario dispersion and shown across a range, not plugged in as an optimistic point estimate.
- [ ] The exercise price, decision timing, holding cost, and risk of expiry or preemption are explicitly included.
- [ ] Real options are not used to rescue a negative-NPV project; the base NPV is valued honestly first and the option value is shown separately and transparently.
- [ ] The limits of option-pricing models for non-tradeable real assets are acknowledged, and decision-tree and scenario methods are used alongside.
- [ ] Real-options value is integrated with NPV, scenario analysis, and market comparables as one input, not the sole basis for the valuation.
- [ ] The recommendation states that real options valuation is highly sensitive to uncertain inputs, that model risk is substantial for real assets, that optionality may never be realized, and that this is not investment advice and professional valuation expertise may be warranted for complex strategic decisions.