---
name: defaults_and_smart_settings.md
description: Use when the agent is choosing default values for settings, designing smart or adaptive product behavior, deciding what to infer versus what to ask, building personalization that adapts over time, or reviewing whether defaults protect or harm users.
---

# Defaults And Smart Settings

Defaults are the most important settings because most users never change them. A default is a decision made on behalf of everyone, applied silently, and lived with indefinitely. Smart settings, adaptive behavior, and personalization extend this idea: the product observes context and adjusts itself. Both carry the same risk. When the inferred or default behavior is right, it feels invisible and helpful. When it is wrong, it feels like the product made a choice for the user without permission, and trust erodes fast.

Use this skill before choosing defaults for new settings, designing adaptive or personalized behavior, deciding whether to ask the user or infer the answer, building recommendation or automation features that change behavior over time, or reviewing whether existing defaults serve users or only serve engagement metrics. The goal is to prevent the agent from picking convenient defaults, over-personalizing without transparency, or building "smart" features that override user intent in ways the user cannot see or correct.

## Core Rules

### Treat Every Default As A Decision With A Beneficiary

A default is never neutral. Ask who benefits from it. A default that maximizes engagement may harm the user's time, attention, privacy, or safety. A default that maximizes privacy may reduce convenience or feature value. A default that maximizes revenue may erode trust.

State the tradeoff explicitly before choosing. Prefer defaults that serve the user's stated goal over defaults that serve a metric the user did not ask to optimize. When business and user interests conflict, make the conflict visible to the team rather than hiding it inside a toggle.

### Choose Defaults For The Median User, Then Protect The Edges

Defaults should serve the typical user in the typical context. But the users most harmed by a wrong default are usually the edge cases: people with accessibility needs, people in low-bandwidth or high-cost environments, people in regulated contexts, people whose data is more sensitive, people for whom a mistake is costly.

After choosing a median default, audit the edges. Ask who is hurt if this default is wrong for them, and how badly. For high-harm edges, either adjust the default, make the setting more discoverable, or add a safeguard. A default that is fine for most and catastrophic for a few is not a good default.

### Prefer Inference Only When It Is Reliable And Reversible

Smart settings infer behavior from signals: location, time, history, device, contacts, usage patterns. Inference is powerful but probabilistic. It will be wrong some of the time.

Infer when:

- the signal is strong and the inference is usually correct;
- the cost of a wrong inference is low;
- the user can see, understand, and override the result;
- the behavior is reversible.

Do not infer when:

- the signal is weak or ambiguous;
- a wrong inference causes embarrassment, cost, safety risk, or data exposure;
- the user cannot easily detect or correct the mistake;
- the behavior is hard to reverse, such as sending a message or making a payment.

When in doubt, ask. A one-time question in context is often better than a persistent guess.

### Make Smart Behavior Observable And Correctable

Adaptive features that change behavior silently break the user's model of the product. If the product starts behaving differently and the user does not know why, they cannot tell whether it is a feature, a bug, or a manipulation.

For smart settings:

- surface that a behavior is adaptive, not fixed;
- show the signals or reasons behind an inference where feasible;
- let the user correct the inference and remember the correction;
- let the user turn adaptive behavior off entirely;
- avoid changing behavior in ways that feel like the product is reading the user's mind without consent.

Transparency is not a footnote. It is part of the feature.

### Separate What To Ask, Infer, And Default

For every product decision, choose one of three strategies:

- Ask the user, when the choice is consequential, personal, or hard to infer.
- Infer, when the signal is strong and the cost of error is low.
- Default, when most users want the same thing and the cost of changing is low.

Misusing these creates harm. Asking too much fatigues users and produces low-quality answers. Inferring too much feels invasive and produces errors. Defaulting on a deeply personal choice removes agency. Match the strategy to the stakes.

### Design Defaults That Age Well

A default chosen at launch may be wrong a year later as the product, user base, regulations, and norms change. Treat defaults as living decisions.

- document the reasoning behind each default;
- revisit defaults when the user base, feature set, or regulatory context shifts;
- watch for defaults that quietly became harmful, such as a sharing default that made sense before a feature went social;
- prefer conservative defaults for anything involving privacy, visibility, data retention, or cost, because the harm of over-exposure compounds.

### Respect User Corrections Over Inferred Patterns

When a user explicitly corrects a smart behavior, that correction should win over the inferred pattern. A product that keeps reverting a user's manual choice because "the algorithm thinks otherwise" teaches the user that their input does not matter.

Persist corrections, apply them to similar cases where reasonable, and avoid re-prompting for the same decision unless the context genuinely changed.

### Avoid Dark Patterns In Default Design

Defaults can be weaponized. A default that opts users into marketing, tracking, sharing, or paid features, especially when the opt-out is buried, is a manipulation dressed as a setting.

Apply the reversal test: if you would be uncomfortable explaining this default to the user in plain language, it is probably wrong. Ethical defaults are explainable, reversible, and aligned with what the user would choose if asked.

## Common Traps

### Defaults Optimized For Engagement Over User Good

A default that increases time-on-product, notifications, or data sharing may look successful in metrics while quietly harming users. Always ask whose goal the default serves.

### Inferring High-Stakes Behavior

Guessing who someone wants to message, what they want to share, what they want to pay for, or what language they want to use can cause embarrassment or cost. Reserve inference for low-stakes, reversible behavior.

### Silent Personalization

When the product changes behavior without telling the user why, the user cannot tell helpful from harmful. Adaptive features need visible reasoning and an off switch.

### Asking Too Much To Avoid Deciding

Forcing the user to configure everything, under the guise of respect for choice, often just offloads the team's unresolved decisions. Good defaults reduce the number of questions, not increase them.

### Reverting User Corrections

A smart feature that overrides the user's explicit choice because the inferred pattern disagrees trains users to stop engaging. Corrections are signal, not noise.

### Conservative Defaults That Quietly Reduce Value

The opposite extreme is also a trap: defaults so restrictive that the product feels broken or useless. Balance protection with usefulness, and measure whether conservative defaults are pushing users away from value they actually want.

### Treating Defaults As Set-And-Forget

Defaults chosen once and never revisited drift out of alignment with reality. Schedule re-evaluation, especially for privacy, sharing, and retention defaults.

## Self-Check

- [ ] Each default has an explicit rationale and a named beneficiary, and the team-facing tradeoff is documented.
- [ ] Edge cases that would be harmed by a wrong default are identified, and safeguards or discoverability are added for them.
- [ ] Inference is used only where signals are strong, errors are low-cost, and the behavior is reversible and correctable.
- [ ] Adaptive behavior is observable, explains its reasoning where feasible, and can be turned off.
- [ ] Each decision is matched to the right strategy: ask, infer, or default, based on stakes and reliability.
- [ ] User corrections persist and override inferred patterns rather than being silently reverted.
- [ ] Privacy, visibility, retention, and cost defaults are conservative and revisit-ready, not set-and-forget.
- [ ] No default relies on buried opt-outs, pre-checked boxes, or manipulation dressed as user choice.
- [ ] The default would be defensible if explained to the user in plain language.
- [ ] Defaults are scheduled for re-evaluation as the product, user base, and regulatory context evolve.
