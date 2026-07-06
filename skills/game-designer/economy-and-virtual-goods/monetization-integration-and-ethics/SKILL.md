---
name: monetization-integration-and-ethics.md
description: Use when the agent is integrating monetization into a game economy, deciding where monetization touches the loop, evaluating whether monetization is fair or predatory, managing the tension between revenue and player trust, or assessing the ethical and reputational risks of a monetization scheme.
---

# Monetization Integration and Ethics

Monetization is not a layer added on top of a finished game; it is a structural choice that reshapes the loop, the economy, and the player's relationship to the game. The judgment problem is that monetization that maximizes short-term revenue often does so by eroding the player trust that sustains long-term revenue, and the metrics that signal the harm (churn, sentiment, lifetime value) lag the design decisions by months. Agents tend to miss this because each monetization mechanic is defensible in isolation and the predatory pattern only emerges in combination, and because revenue is a concrete number while trust is diffuse. The harm is a game that monetizes aggressively at launch, generates a revenue spike, and then collapses as the audience revolts, reviews turn negative, and the lifetime value the monetization was meant to maximize is destroyed by the monetization itself. This skill covers how to integrate monetization without breaking the loop, distinguish fair from predatory schemes, and protect the trust that is the actual long-term revenue source. The agent has latitude in the monetization model, but the obligation to evaluate the ethical and trust consequences is not optional.

## Core Rules

### Integrate Monetization at the Loop's Rest Beats, Not Its Friction Points

Monetization placed at the loop's friction points — where the player is blocked, frustrated, or waiting — reads as a paywall and converts friction into resentment. Monetization placed at rest beats — where the player is choosing, expressing, or aspiring — reads as an option and converts desire into purchase. The decision rule: map the core loop's friction points and rest beats, and place monetization only at the latter. Monetizing friction produces a game that feels like it is punishing non-payers; monetizing aspiration produces a game that feels like it is offering choices.

### Distinguish Accelerators From Paywalls and Never Cross the Line

An accelerator lets a paying player reach a goal faster that a non-paying player can still reach; a paywall lets a paying player reach a goal that a non-paying player cannot. The decision rule: confirm every monetized offering is an accelerator (the free path exists and is reasonable), and never a paywall (the free path is absent or punitive). The line between them is the line between fair and predatory, and crossing it — making content, power, or completion accessible only through payment — converts the audience against the game, because the contract changed from "pay to skip" to "pay to play."

### Preserve a Complete, Satisfying Free Experience

A free-to-play game's free experience must be genuinely complete and satisfying, not a truncated demo that pressures conversion, because the free players are the community, the content, and the word-of-mouth that the paying players exist within. The decision rule: playtest the full free path and confirm a free player can engage with the core loop, progress meaningfully, and experience the game's satisfaction without paying. Truncated free experiences convert poorly and poison the community, because the free players who would have become paying players leave before conversion, and the paying players find a shrinking community.

### Evaluate Each Mechanic Against the Predatory Pattern Checklist

Predatory monetization has recognizable features: obfuscation of real-world cost (premium currency in odd bundles), pressure mechanics (time-limited offers, loss aversion), targeting of vulnerable populations (whale-focused pricing, gambling mechanics), and asymmetry between spend and value. The decision rule: evaluate every monetization mechanic against this checklist and reject or redesign any that match, regardless of revenue projection, because predatory mechanics generate revenue by exploiting the player in ways that produce churn, regulatory risk, and reputational damage that exceed the revenue.

### Model the Lifetime Value Consequence, Not Just the Launch Spike

Aggressive monetization produces a launch revenue spike and a long-term revenue decline as trust erodes and the audience churns; ethical monetization produces a lower spike and a sustained long-term revenue floor as trust retains the audience. The decision rule: model revenue on a multi-year horizon, not a launch quarter, and prefer the model with the higher long-term floor even if its spike is lower. Teams that optimize the spike ship monetization that mines the audience and then must replace it, which is more expensive and less sustainable than retaining it.

### Treat Player Trust as the Actual Revenue Asset

The durable revenue asset is not the monetization scheme; it is the player trust that makes the audience willing to spend over years. The decision rule: evaluate every monetization decision by whether it builds or spends trust, and treat trust-spending decisions as drawing down a finite asset that must be replenished. Monetization that spends trust for revenue is liquidating the asset the business depends on, and the scheme that looked profitable in the quarter destroys the franchise in the years.

## Common Traps

### Monetizing the Friction Point

The team places the monetization offer at the moment the player is blocked — out of energy, out of lives, at a difficulty wall — so the offer reads as the solution to a problem the game created, and the player experiences it as extortion. The trap is that friction-point monetization converts well in the short term. The false signal is high conversion at the friction point. The harm is that the player correctly identifies the friction as manufactured to drive the purchase, trust collapses, the player who pays once resents it, the player who does not pay churns, and the conversion that looked like revenue is actually the liquidation of the trust that would have driven repeated purchases, because the team monetized the pain instead of the desire.

### The Accelerator That Drifts Into a Paywall

A monetized offering begins as a fair accelerator — pay to progress faster — and over successive tuning passes the free path is made slower or harder to drive conversion, until the offering is functionally a paywall and the free path is punitive. The trap is that each incremental tightening is small and defensible. The false signal is that conversion rises with each tightening. The harm is that the cumulative drift crosses the fair-to-predatory line without a single decision that crossed it, the audience perceives the change even if they cannot articulate it, sentiment sours, and the game that was fair at launch becomes predatory at maturity, because no one guarded the line against incremental erosion.

### The Truncated Free Experience That Pressures Conversion

The free experience is designed as a funnel that withholds satisfaction to pressure conversion, rather than a complete experience that earns the player's voluntary upgrade, and the free players — who are the community and the pipeline to paying — leave. The trap is that pressure converts a fraction immediately. The false signal is launch conversion numbers. The harm is that the free players who would have become the community and the word-of-mouth and the eventual payers never stay long enough to convert, the paying players find a dying community, and the game that optimized conversion in the funnel destroyed the ecosystem that conversion depends on, because the free experience was treated as a cost rather than an asset.

### Premium Currency Bundles Designed to Leave Leftovers

Premium currency is sold in bundles that never match item prices, so the player always has leftover currency and must buy more to use it, a deliberate obfuscation of real-world cost. The trap is that leftover currency drives repeated purchases. The false signal is higher revenue per paying user. The harm is that the player recognizes the manipulation, experiences every purchase as a small betrayal, trust erodes purchase by purchase, and the scheme that extracted more per transaction reduced the total transactions the player was willing to make, because obfuscation reads as dishonesty and dishonesty caps the relationship.

### Optimizing the Launch Spike and Ignoring the Long-Term Floor

The team evaluates monetization by launch-quarter revenue and ships the most aggressive scheme that maximizes the spike, ignoring that the same aggressiveness collapses the long-term floor as the audience churns. The trap is that the spike is the visible, celebrated metric. The false signal is a strong launch revenue report. The harm is that the scheme mines the launch audience and then must replace it at acquisition cost, the long-term revenue is far below what a less aggressive scheme would have sustained, and the franchise that could have generated revenue for years generates it for months, because the team optimized the quarter and sacrificed the years.

## Self-Check

- Have I mapped the core loop's friction points and rest beats, and placed monetization only at rest beats, never at friction?
- Is every monetized offering an accelerator with a reasonable free path, and have I confirmed none has drifted into a paywall?
- Does the free experience allow a non-paying player to engage the core loop, progress, and experience satisfaction completely?
- Did I evaluate each monetization mechanic against the predatory pattern checklist (obfuscation, pressure, vulnerability targeting, spend-value asymmetry) and reject matches?
- Am I modeling revenue on a multi-year horizon, preferring a higher long-term floor over a higher launch spike?
- Did I treat player trust as the revenue asset and evaluate each decision by whether it builds or spends that trust?
- Did I avoid premium currency bundles designed to leave leftovers, recognizing obfuscation as trust erosion?
