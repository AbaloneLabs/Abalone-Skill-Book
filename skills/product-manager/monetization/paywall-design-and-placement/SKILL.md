---
name: paywall-design-and-placement.md
description: Use when the agent is designing paywalls, deciding where and when to place paywalls in the user journey, balancing free value with conversion pressure, writing paywall messaging, choosing what to surface on the paywall, or evaluating the tradeoff between conversion and trust in gating decisions.
---

# Paywall Design And Placement

The paywall is one of the most consequential and most mistreated surfaces in a product. It is the moment where the value the user has experienced meets the ask to pay, and its design determines whether that meeting produces conversion, resentment, or churn. A well-designed paywall, placed at the right moment, converts users who have been convinced of value and preserves the trust of those who have not. A poorly designed paywall — too early, too aggressive, too opaque — converts a few through pressure while alienating many, damaging the relationship and often the brand. The product manager who treats the paywall as a careful negotiation rather than a forced gate will convert more users and keep them longer.

This skill covers the judgment needed to design and place paywalls: when to gate, what to show, how to ask, and how to balance conversion against trust.

## Core Rules

### Place the paywall at the moment of demonstrated value, not at the moment of maximum pressure

The single most important paywall decision is placement, and the most common failure is placing it for pressure rather than for value. A paywall placed before the user has experienced value asks for payment against an unproven promise and converts poorly while generating resentment. A paywall placed at the moment the user has just experienced meaningful value asks for payment against demonstrated worth and converts far better.

- Identify the moment in the user journey where value has been demonstrated, not merely promised. This is the natural placement.
- Resist the temptation to gate earlier to "force" conversion. Early gating converts the desperate few and alienates the majority, who leave without the value that would have converted them.
- For products where value is ongoing rather than moment-based, place the paywall after a meaningful taste of value, not after an arbitrary time or action count.

The paywall is a value exchange. It works when the user has already received enough to judge the exchange as fair.

### Gate based on the value gradient, not based on feature inventory

What to put behind the paywall is as important as where to place it, and the common failure is gating based on what features exist rather than on what value they represent. The gating should follow the value gradient: free users receive genuine value, and the paid tier offers value that is clearly greater and that the user naturally wants once they have outgrown the free experience.

- Gate the capabilities that represent the escalation of value, not arbitrary features. The upgrade should be motivated by the user's growing need, not by artificial scarcity.
- Avoid gating core value so tightly that the free experience is useless; it serves as a poor advertisement and suppresses both acquisition and conversion.
- Avoid gating so loosely that the free experience satisfies most users, leaving no motivation to upgrade.
- Do not split a coherent workflow across the gate, where some steps are free and some paid. This creates frustration, not motivation.

The gate should feel to the user like a natural threshold where more value becomes available, not like a wall erected across their path.

### Make the paywall communicate value, not pressure

The paywall surface itself should be designed around communicating the value of the paid tier, not around applying pressure to convert. Pressure-based design (urgency timers, fake scarcity, manipulative framing) produces short-term conversion and long-term distrust, and increasingly carries legal and regulatory risk.

- Lead with the value the user will receive, in concrete terms connected to what they have already experienced. "Continue with X" beats "Upgrade now."
- Show the options clearly and let the user choose, rather than pre-selecting or obscuring the lower-priced option.
- Use honest urgency only when it is real (a genuine promotional period, a real limit). Fabricated urgency destroys credibility the moment the user recognizes it.
- Make the path to purchase simple and the path to decline respectful. A user who declines should be able to return to the free experience without being punished or trapped.

Users who convert through value communication become long-term customers. Users who convert through pressure churn and warn others.

### Treat the paywall as a negotiation that must preserve the relationship for non-converters

Most users who see a paywall will not convert on first exposure, and the paywall's design determines whether those non-converters become future converters or permanent losses. A paywall that traps, punishes, or alienates non-converters destroys the future conversion opportunity. A paywall that respects the user's choice keeps the door open.

- Ensure that declining the paywall returns the user to a usable free experience, not to a degraded or punishing state.
- Do not repeatedly re-gate the same user with the same paywall in the same session; this is harassment, not conversion.
- Allow the user to continue receiving free value, so that when their need escalates, the paid tier is a natural choice rather than a grudge purchase.
- Design for the user who converts on the fifth exposure, not only the first. The relationship across multiple exposures determines whether that fifth exposure ever happens.

The paywall is not just for the users who convert today; it is for the users who will convert later, if the relationship is preserved.

### Test paywall design on conversion and on retention, not just on conversion

Paywall changes are typically measured on conversion rate, and this is where the most dangerous mis-optimization occurs. A change that increases conversion but harms retention — because it converts users through pressure who then churn, or because it damages trust — is a net loss that conversion metrics hide.

- Measure the effect of paywall changes on downstream retention and lifetime value, not just on immediate conversion.
- Watch for changes that increase conversion of low-intent users who churn quickly. These inflate the conversion metric while degrading the customer base.
- Test the effect on the non-converters too. A paywall change that increases conversion but alienates non-converters may reduce the future conversion pool more than it gains in immediate conversions.

A paywall optimized only for conversion will, over time, optimize toward pressure and manipulation, because those move the immediate metric. Retention and lifetime value are the metrics that keep the optimization honest.

### Make pricing and terms transparent on the paywall

The paywall is where the user makes a financial decision, and opacity at this moment destroys trust. Hidden fees, unclear billing cycles, obscured auto-renewal, and hard-to-find cancellation create conversions that feel like tricks and generate churn, complaints, and chargebacks.

- State the price, the billing cycle, and what is included clearly and prominently.
- Disclose auto-renewal and cancellation terms at the point of purchase, not buried in terms of service.
- Do not use confusing pricing units (credits, tokens, points) that obscure the real cost unless the model genuinely requires them, and even then, make the real-world cost clear.
- Make cancellation as easy as subscription. Hard cancellation is a retention tactic that produces churn-with-resentment, regulatory risk, and brand damage.

Transparency at the paywall is trust-building; opacity is trust-destroying, and the trust damage outlasts any conversion gain.

### Adapt the paywall to context and segment

A single paywall design for all users, at all moments, is suboptimal. The paywall that works for a user deep in the product differs from one that works for a new user, and the paywall that motivates one segment may not motivate another.

- Adapt paywall messaging and offer to the user's context: what they were doing, what value they have experienced, what their likely need is.
- Adapt to segment where willingness to pay and value perception differ. A paywall that fits a power user may alienate a casual one.
- Adapt to timing. A paywall presented after a peak value moment differs from one presented during a friction moment.
- Be cautious with personalization that feels surveillant. The adaptation should feel helpful, not creepy; the line is real and user-perception-dependent.

## Common Traps

### Paywall placed before value is demonstrated

Gating early to force conversion converts the desperate few and alienates the majority, who leave without the value that would have converted them. Place the paywall at the moment of demonstrated value.

### Gating based on feature inventory rather than value gradient

Arbitrary gating splits coherent workflows and creates frustration rather than motivation. Gate based on the natural value gradient.

### Pressure-based paywall design

Urgency timers, fake scarcity, and manipulative framing produce short-term conversion and long-term distrust, with growing legal risk. Communicate value, not pressure.

### Trapping or punishing non-converters

A paywall that degrades the free experience for users who decline destroys the future conversion opportunity. Preserve the relationship for non-converters, who may convert later.

### Measuring only conversion, not retention

A change that increases conversion but harms retention is a net loss hidden by the conversion metric. Measure downstream retention and lifetime value.

### Opaque pricing and hard cancellation

Hidden fees, unclear billing, and hard cancellation produce conversions that feel like tricks and generate churn, complaints, and regulatory risk. Be transparent and make cancellation easy.

### Re-gating the same user repeatedly

Showing the same paywall repeatedly in the same session is harassment, not conversion, and trains the user to dismiss all paywalls.

### One paywall for all users and moments

A single paywall design is suboptimal across segments and contexts. Adapt messaging, offer, and timing to context, while avoiding personalization that feels surveillant.

## Self-Check

- Is the paywall placed at a moment of demonstrated value, or at a moment of maximum pressure that converts few and alienates many?
- Does the gating follow the value gradient, motivating upgrade through the user's growing need rather than artificial scarcity?
- Does the paywall communicate value and respect the user's choice, free of manipulative pressure tactics?
- Does declining the paywall return the user to a usable, non-punishing free experience that preserves the future conversion opportunity?
- Am I measuring the effect of paywall changes on retention and lifetime value, not just on conversion?
- Are pricing, billing, auto-renewal, and cancellation terms transparent at the point of purchase?
- Is cancellation as easy as subscription, or am I using friction to retain users who want to leave?
- Am I avoiding re-gating the same user repeatedly, which trains dismissal of all paywalls?
- Have I considered adapting the paywall to context and segment, while staying on the right side of the line between helpful and surveillant?
- If a user who declined my paywall described the experience to a friend, would they describe a fair offer or a manipulative trap?
