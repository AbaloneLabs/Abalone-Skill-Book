---
name: pricing-tier-architecture.md
description: Use when the agent is designing pricing tier architecture, deciding how many tiers to offer, what features belong in each tier, setting price points across tiers, designing upgrade paths and good-better-best structures, or evaluating how tier structure affects conversion expansion and customer satisfaction.
---

# Pricing Tier Architecture

Tier architecture is the structure of choices you offer customers: how many tiers, what is in each, how they are priced relative to each other, and how users move between them. It is a deceptively complex design problem because every choice interacts with every other. The number of tiers affects decision load; the feature distribution affects perceived fairness; the price spacing affects upgrade motivation; the naming affects positioning. A well-designed tier architecture guides users to the tier that fits their needs and gives them a clear path to upgrade as those needs grow. A poorly designed one creates confusion, leaves money on the table, suppresses upgrades, and generates resentment that surfaces as complaints and churn.

This skill covers the judgment needed to design and evaluate pricing tier architecture: how many tiers, what goes where, how to price them, and how to keep the structure coherent as the product evolves.

## Core Rules

### Design tiers around distinct customer segments with distinct needs, not around feature bundling

The foundation of good tier architecture is that each tier serves a distinct customer segment with distinct needs and willingness to pay. Tiers that are bundled around feature inventory rather than around segment needs produce options that fit no one well and that confuse the buyer. Start from the segments.

- Identify the distinct customer segments: by size (individual, small team, enterprise), by use case (light, professional, mission-critical), by sophistication, by willingness to pay.
- Design each tier to serve one segment's needs coherently, with the features that segment values at a price that segment can bear.
- Avoid tiers that mix features from different segments' needs, which creates options that are partially attractive to several segments and fully attractive to none.

When the tiers map to real segments, the user recognizes their tier and chooses confidently. When they do not, the user puzzles over options that all seem partially wrong.

### Choose the number of tiers to balance choice against decision load

The number of tiers is a tradeoff between offering enough choice to fit different segments and avoiding so much choice that users are paralyzed. Both extremes fail: too few tiers force segments into options that do not fit; too many tiers overwhelm and delay or prevent the decision.

- Two or three tiers fit most products and most buyers. The "good-better-best" structure is durable because it offers a clear comparison and a natural upgrade path without overwhelming.
- More tiers are justified only when there are genuinely distinct segments whose needs cannot be served by collapsing into fewer options. Adding tiers to capture marginal revenue usually adds confusion that costs more than it gains.
- Fewer than two paid tiers leaves no upgrade path within the product and caps expansion revenue.

Resist the pressure to add tiers to capture every segment edge case. Each addition increases decision load for every buyer and maintenance load for the team.

### Distribute features across tiers following the value gradient

The distribution of features across tiers should follow the value gradient: the value a user receives should escalate clearly and coherently as they move up the tiers. This is what makes the upgrade motivating rather than arbitrary.

- Each tier should deliver a coherent, complete experience for its target segment. A tier that is missing a piece its segment needs feels broken and generates complaints.
- The step up to the next tier should add value that the next segment genuinely needs, not arbitrary limits removed. The upgrade should feel like gaining capability, not like escaping a restriction.
- Avoid splitting a coherent workflow across tiers. If a segment needs steps A, B, and C together, putting C in a higher tier creates frustration, not upgrade motivation.

The test: can a user in each segment use their tier to accomplish their goals completely, and is the next tier's value clearly relevant to a more demanding user?

### Price the tiers relative to each other to motivate the right choice

The relative pricing of tiers shapes which one users choose, and this is as important as the absolute prices. The spacing and anchoring between tiers guide the buyer toward the choice the business wants them to make.

- The price gap between tiers should be large enough to make the choice meaningful but not so large that the upgrade feels unattainable. A gap too small makes the higher tier feel like an obvious choice (devaluing the lower tier); a gap too large makes the higher tier feel out of reach (capping expansion).
- Anchoring matters. The presence of a high-priced top tier makes the middle tier feel reasonable, even if few buy the top. This is the basis of good-better-best, and it is effective when used honestly.
- The most popular or target tier should be positioned as the natural choice for the largest segment, with the others providing comparison that makes it look right.

Price the tiers as a coherent structure, not as independent points. The relationships between them drive behavior as much as the absolute numbers.

### Provide a clear and natural upgrade path

Tier architecture is not just about the initial choice; it is about the path from the entry tier to higher tiers as the user's needs grow. A well-designed architecture makes the upgrade natural and motivating; a poorly designed one caps users at the entry tier because the next step is unclear, unattractive, or blocked.

- The upgrade should be triggered by the user's own growth (more usage, more users, more capability needed), not by artificial limits that feel punitive.
- The value added at each step up should be clearly relevant to a user who has outgrown the current tier, so the upgrade feels like a response to need rather than a push.
- Make the upgrade process itself simple and reversible. Friction in upgrading suppresses expansion; irreversibility suppresses it further by making the choice feel risky.

### Keep the tier architecture coherent and stable, changing it deliberately

Tier architecture needs to evolve as the product and market do, but frequent or incoherent changes confuse customers, erode trust, and create support burden. Each change should be deliberate, coherent, and communicated, not reactive.

- Avoid adding tiers or features to tiers reactively to capture specific deals or respond to competitors, which produces an incoherent structure over time.
- When changing the architecture, consider the effect on existing customers. Changes that move features to higher tiers or raise prices feel like takeaways and generate resentment, even when justified.
- Communicate changes clearly and give existing customers a graceful path, especially when changes affect what they are paying for.

### Name tiers to communicate positioning, not to obscure

Tier names shape how buyers perceive the options and themselves within them. Names that communicate the intended segment and value ("Individual," "Team," "Enterprise") help buyers self-select. Names that obscure ("Basic," "Plus," "Pro," "Premium" with no clear differentiation) add confusion and can subtly shame buyers away from lower tiers.

- Name tiers to reflect the segment or the value, in language the buyer uses.
- Avoid names that imply the lower tiers are inferior, which shames buyers who legitimately belong there and pushes them to tiers that do not fit.
- Keep naming consistent across the structure, so the progression is intuitive.

## Common Traps

### Tiers bundled around features rather than segments

Options that mix features from different segments' needs fit no one well and confuse the buyer. Design each tier around a distinct segment with distinct needs.

### Too many tiers creating decision paralysis

Adding tiers to capture every segment edge case increases decision load and maintenance cost beyond the revenue gained. Use the minimum number of tiers that genuinely serve distinct segments.

### Splitting a coherent workflow across tiers

Putting part of a workflow in a higher tier creates frustration, not upgrade motivation. Keep each tier a complete experience for its segment.

### Price gaps that are too small or too large

Gaps too small devalue the lower tier; gaps too large cap expansion. Price the tiers as a coherent structure where the relationships drive the right choice.

### Upgrade path that is unclear or blocked by artificial limits

Users cap at the entry tier because the next step is unclear or gated punitively. Make the upgrade natural, motivated by the user's growth, and simple to execute.

### Reactive tier changes that make the structure incoherent

Adding tiers or features reactively to win deals produces a structure that confuses customers and is hard to maintain. Change the architecture deliberately and coherently.

### Changes that feel like takeaways to existing customers

Moving features up or raising prices generates resentment and churn, even when justified. Communicate changes and provide a graceful path for existing customers.

### Tier names that obscure or shame

Names that do not communicate segment or value add confusion; names that imply lower tiers are inferior shame buyers and push them to ill-fitting tiers. Name tiers to communicate positioning honestly.

## Self-Check

- Does each tier serve a distinct customer segment with distinct needs, or are tiers bundled around feature inventory?
- Is the number of tiers the minimum that genuinely serves distinct segments, or have I added tiers that increase confusion more than revenue?
- Does each tier deliver a coherent, complete experience for its segment, with the upgrade adding value the next segment genuinely needs?
- Are the price gaps between tiers calibrated to motivate the right choice, neither devaluing the lower tier nor capping expansion?
- Is the upgrade path natural and motivated by the user's growth, simple to execute, and reversible?
- Have I avoided reactive changes that make the structure incoherent, and changed the architecture deliberately when needed?
- When I last changed the architecture, did I consider the effect on existing customers and provide a graceful path?
- Do the tier names communicate segment and value honestly, without obscuring or shaming lower-tier buyers?
- If a new customer looked at my tiers today, would they recognize their tier and feel good about choosing it, or would they be confused and delayed?
- Does the architecture provide a clear path from entry to expansion, or does it cap users at the tier they start in?
