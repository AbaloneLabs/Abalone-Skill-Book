---
name: north_star_metric_selection.md
description: Use when the agent is selecting or evaluating a North Star metric, defining what single metric best represents product value, choosing between candidate metrics, or assessing whether a proposed North Star will align the team and resist gaming.
---

# North Star Metric Selection

A North Star metric is the single metric chosen to represent the value the product creates for customers, the number that, if it goes up sustainably, means the product is succeeding. Its purpose is to focus a diverse organization on one shared definition of success, so that decentralized decisions all push the same direction. Done well, it creates genuine alignment and makes tradeoffs clearer. Done poorly, it selects a metric that can be gamed, that rewards the wrong behavior, or that captures activity rather than value. Selecting the North Star is one of the highest-leverage and most easily botched decisions in product, because the wrong metric misdirects years of effort. Agents often propose a North Star by picking the metric that is easiest to measure or that already moves, rather than the metric that represents value.

The harm this skill prevents is organizational effort pointed at the wrong target. A team optimizing a flawed North Star will dutifully move it while the product stagnates or harms customers, because the metric and the value have come apart. The cost is enormous because the misdirection is invisible until the consequences, churn, stagnation, brand damage, become undeniable.

Use this skill before answering questions such as "what should our North Star metric be", "is this a good North Star", "how do we choose between candidate metrics", or "why does our metric keep moving while the product struggles". The goal is to prevent the agent from selecting a North Star on convenience rather than on a rigorous theory of what value the metric represents.

## Core Rules

### Choose A Metric That Represents Value Delivered, Not Activity Performed

The North Star must measure the value customers receive from the product, not the work the product or the customers do. Sessions, logins, page views, and features shipped are activity; they can rise while value falls. The right metric reflects whether customers are succeeding at the job the product exists to help with. A product whose North Star is engagement can drive engagement that harms users, while a product whose North Star reflects successful outcomes drives the team toward genuine value.

Express the North Star in terms of the customer outcome: customers who reached a result, value created per customer, problems solved, progress made. The test is whether a sustained increase in the metric, holding everything else constant, means customers are better off. If you can imagine the metric rising while customers are worse off, it is measuring activity, not value.

### Ensure The Metric Cannot Be Easily Gamed

Any metric that becomes a target will be gamed, intentionally or through well-meaning local optimization. A good North Star is hard to game because moving it requires genuinely creating value, not just changing behavior to flatter the number. Examine each candidate metric for how it could be inflated without creating value: counting low-quality actions, broadening definitions, incentivizing unwanted behavior, or sacrificing long-term value for short-term movement. The more easily a metric can be gamed, the worse it is as a North Star.

Pair the North Star with guardrail metrics that detect gaming and unintended harm. If the North Star is engagement, a guardrail on satisfaction or retention catches the case where engagement rises because the product is manipulative rather than valuable. The guardrails do not replace the North Star; they keep it honest.

### Decompose The North Star Into A Clear Formula

A North Star that is a single abstract number is hard to act on. Decompose it into a formula whose components the team can influence, so that everyone understands the levers that move it. A typical decomposition multiplies a count of customers by a frequency by a depth of value, or similar factors that correspond to distinct strategic levers. The decomposition turns the North Star from a scoreboard into a map of where the team can intervene.

Each component of the formula should correspond to a real strategic question: are we acquiring more customers, are they using more often, are they getting more value per use. The decomposition makes clear which lever is moving and which is stuck, focusing effort where it can have effect.

### Test The Metric Against The Product's Specific Value Theory

There is no universal North Star. The right metric depends entirely on what value the specific product creates and for whom. A marketplace, a productivity tool, a content platform, an enterprise system, and a consumer app all create value differently and need different North Stars. Do not copy another product's metric; derive yours from your own theory of value. State what value the product creates, then ask what metric would prove that value is being delivered.

A common failure is to choose a North Star that is plausible for the category but wrong for the specific product. Two products in the same category can have different value theories and therefore different North Stars. The metric must fit the product, not the category label.

### Verify The Metric Actually Correlates With Long-Term Success

Before committing to a North Star, test whether it historically correlates with the outcomes the business cares about: retention, revenue, longevity, low churn. A metric that moves independently of retention may be measuring something other than value, no matter how plausible it sounds. Look at the data: do customers who score higher on the metric retain better, expand more, churn less? If the correlation is weak or absent, the metric is not capturing value, and choosing it will point the team away from what actually drives success.

Where historical data is unavailable, because the product is new, treat the North Star as a hypothesis and revisit it once retention data accumulates. Do not enshrine an untested metric as permanent truth.

### Ensure The Metric Is Communicable And Meaningful Across Functions

A North Star that only the data team understands cannot align an organization. It must be expressible in plain language, meaningful to engineering, design, marketing, sales, and support, and connected to the work each function does. If the metric requires a paragraph of caveats to explain, it is too complex to serve as a shared target. Simplicity and clarity are properties of a good North Star, because alignment depends on comprehension.

The tradeoff is that the most rigorous metric may be less communicable than a slightly less rigorous but far more intuitive one. Sometimes the right choice is the metric the whole organization can rally around, provided it still represents value and resists gaming.

## Common Traps

### Choosing Activity Over Value

Selecting sessions, logins, or actions because they are easy to measure. The trap is a metric that rises while value falls, driving the team to optimize the wrong thing.

### A Metric That Is Easily Gamed

Picking a number that can be inflated without creating value. The trap is dutiful optimization that produces metric movement and customer harm simultaneously.

### Copying Another Product's North Star

Adopting a famous company's metric without examining fit. The trap is a metric that fits their value theory, not yours.

### No Guardrail Metrics

Optimizing the North Star without monitoring for harm. The trap is gaming and unintended consequences that go undetected until they damage the product.

### A Metric That Does Not Correlate With Retention

Choosing on plausibility rather than evidence. The trap is a metric that moves while churn rises, because the metric and value have come apart.

### An Incomprehensible North Star

A metric so complex that only specialists understand it. The trap is an alignment tool that fails to align because no one outside data knows what it means.

## Self-Check

- [ ] The North Star measures value delivered to customers, not activity performed by the product or users.
- [ ] The metric has been examined for gaming potential and paired with guardrail metrics that detect harm.
- [ ] The metric is decomposed into a formula whose components correspond to strategic levers the team can influence.
- [ ] The metric is derived from the product's specific value theory, not copied from another product or category.
- [ ] Historical data shows the metric correlates with retention, expansion, and long-term success, or it is treated as a hypothesis pending data.
- [ ] The metric is communicable in plain language and meaningful across functions.
- [ ] A sustained increase in the metric, holding all else constant, genuinely means customers are better off.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
