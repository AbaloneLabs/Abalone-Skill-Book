---
name: progress_mapping_and_outcomes.md
description: Use when the agent is mapping the progress a customer is trying to make, defining desired outcomes under a Jobs-To-Be-Done lens, sequencing the steps of progress, or using outcome statements to prioritize what to build and measure.
---

# Progress Mapping And Outcomes

Progress mapping is the operational core of Jobs-To-Be-Done. Once the job is defined, the work is to map the full arc of progress the customer is trying to make, from the initial struggle through the steps to the desired outcome, and to identify where that progress is currently blocked. This map becomes the basis for prioritization, measurement, and design, because it shows exactly where the customer is underserved and what better progress would look like. Done well, it produces outcome statements that are measurable, stable, and directly actionable for roadmap decisions. Done poorly, it produces a journey diagram that looks comprehensive but never changes what the team builds or tracks.

The harm this skill prevents is optimizing individual steps while the customer's overall progress remains poor. A product can improve every feature and still lose, because the binding constraint on the customer's progress sits in a step the product does not address, or in the handoff between steps. Agents often map the journey from the product's perspective rather than the customer's, producing a diagram of product usage rather than a map of customer progress.

Use this skill before answering questions such as "how do we map the customer journey", "what outcomes should we optimize", "where are customers underserved", or "how do we use JTBD to prioritize". The goal is to prevent the agent from producing a journey map that describes the product instead of the customer's progress.

## Core Rules

### Map Progress From The Customer's Perspective, Not The Product's

A progress map describes what the customer is doing, thinking, and feeling as they try to make progress, not the screens they click through in your product. The map must include everything the customer does, including steps that happen outside the product: research, comparison, setup, integration, training their team, and abandoning the effort. If the map only covers in-product flows, it misses the steps where progress most often breaks down.

Start the map before the customer encounters the product, at the moment the need arises, and extend it past the moment of first use, through adoption, integration into routine, and the outcomes that follow. The boundaries matter: a map that starts at sign-up and ends at first use captures only a fraction of the progress arc and misses where customers actually struggle.

### Write Outcome Statements That Are Measurable And Solution-Independent

An outcome statement describes what the customer wants to achieve as a direction of improvement: minimize the time to do something, increase the likelihood of a result, reduce the effort required, or reduce the variability of an outcome. Well-formed outcome statements contain a direction, a metric, and a context, and they make no reference to any specific feature or solution. They are stable, because the desired outcome persists even as technology changes.

Poor outcome statements smuggle in solutions or remain unmeasurable. "A better dashboard" is a solution, not an outcome. "Easier reporting" is unmeasurable. "Minimize the time to produce an accurate monthly expense summary" is an outcome: it has a direction, a metric, and a context, and it does not prescribe how. Prioritization against outcome statements forces the team to consider many possible solutions rather than refining one.

### Identify Underserved Outcomes By Importance And Satisfaction

Not every outcome is equally worth addressing. The most valuable opportunities are outcomes that customers rate as highly important and currently poorly satisfied. Outcomes that are important and well satisfied are table stakes; outcomes that are unimportant need no investment. Mapping outcomes on a two-axis view of importance and satisfaction reveals where the product can create the most value, which is usually in the high-importance, low-satisfaction region.

This analysis depends on gathering real importance and satisfaction signals, not guessing. Use structured research, ranking exercises, or behavioral proxies to estimate where each outcome sits. The combination of importance and satisfaction is more informative than either alone, because it separates opportunities from obligations.

### Locate The Binding Constraint On Progress

Across the map of progress, some step or outcome is the binding constraint, the place where progress breaks down most often and most severely. This is rarely obvious and often sits outside the product. Customers may struggle at the step of getting internal buy-in, migrating data, training colleagues, or trusting the result. Improving steps downstream of the binding constraint creates no value, because customers never reach them.

Diagnose the binding constraint by looking at where customers drop off, where they build workarounds, where they report the most friction, and where progress stalls in the data. Concentrate improvement effort there before optimizing steps that are not currently limiting.

### Connect Outcomes To Metrics That Reflect Progress

Each important outcome should map to a metric that indicates whether the customer is making better progress. These metrics differ from feature-usage metrics, because they measure outcomes rather than activity. A product can drive heavy feature usage while customers fail to make progress, which shows up as churn despite engagement. The right metrics answer whether the customer is succeeding at the job, not whether they are clicking.

Resist the temptation to track only what is easy to measure. If the true outcome is hard to instrument, use a proxy but label it as a proxy and sanity-check it against retention and qualitative signals. A metric that moves the right way while retention moves the wrong way is measuring the wrong thing.

### Use The Map To Prioritize, Not Just To Illustrate

A progress map earns its cost only if it changes prioritization. Once outcomes are mapped, important, and satisfied levels are estimated, and the binding constraint is located, the roadmap should look different than it would have without the map. If the same features would have been prioritized regardless, the map was decorative. Test the map's value by asking what it caused the team to stop doing or start doing.

The map should also reveal outcomes the product currently does not serve at all, which are candidates for expansion, and outcomes it serves poorly relative to their importance, which are candidates for investment.

## Common Traps

### Mapping The Product Flow Instead Of Customer Progress

Drawing the steps as screens or features rather than as what the customer is trying to accomplish. The trap is a map that confirms the current product rather than revealing where progress breaks.

### Outcome Statements That Smuggle In Solutions

Writing outcomes as feature wishes. The trap is that prioritization collapses back into feature debate, losing the solution-independence that makes outcomes useful.

### Starting And Ending The Map At Product Boundaries

Beginning at sign-up and ending at first use. The trap is missing the steps before and after the product where progress most often fails.

### Treating All Outcomes As Equally Important

Prioritizing whichever outcome is loudest or most recent. The trap is investment in low-importance outcomes while the binding constraint goes unaddressed.

### Improving Downstream Of The Binding Constraint

Polishing steps customers never reach. The trap is visible improvement that produces no change in overall progress or retention.

### Metrics That Track Usage Instead Of Progress

Measuring clicks and sessions rather than whether the customer succeeded at the job. The trap is dashboards that look healthy while churn rises.

## Self-Check

- [ ] The progress map is drawn from the customer's perspective and includes steps outside the product, from need to sustained outcome.
- [ ] Outcome statements are measurable, solution-independent, and stable across technology change.
- [ ] Outcomes are evaluated on both importance and current satisfaction to reveal underserved opportunities.
- [ ] The binding constraint on progress has been located and prioritized over downstream improvements.
- [ ] Each important outcome maps to a metric that reflects customer progress, not only feature usage.
- [ ] The map changed at least one prioritization decision; if not, it was decorative.
- [ ] Outcomes the product does not serve or serves poorly are identified as expansion or investment candidates.
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
