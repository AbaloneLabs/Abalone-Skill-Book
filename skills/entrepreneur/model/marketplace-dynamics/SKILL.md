---
name: marketplace_dynamics.md
description: Use when the agent is designing or analyzing a two-sided marketplace, balancing supply and demand liquidity, setting take rates and fees, managing marketplace density by geography or category, addressing quality and trust, or diagnosing why a marketplace is stalling between growth in listings versus growth in transactions.
---

# Marketplace Dynamics

A marketplace is a platform whose core transaction is matching supply with demand, and its health is governed by dynamics that do not exist in ordinary product businesses. A marketplace can have growing listings, growing users, and growing traffic and still be failing, if the match rate, liquidity, and transaction completion are degrading. The metrics that look like growth in a product business can hide decay in a marketplace, because the value is created not by the presence of participants but by successful matches between them. The judgment problem is understanding the levers that drive marketplace liquidity, balancing supply and demand so that both sides find value quickly, and diagnosing whether the marketplace is healthy or merely large. Agents tend to fail by chasing listing growth without transaction growth, by mispricing the take rate, and by ignoring density, the concentration of supply and demand that makes matching possible.

Use this skill when designing, operating, or diagnosing a two-sided marketplace: setting fees, managing density, addressing liquidity, or interpreting marketplace metrics. The goal is a marketplace where supply and demand find each other efficiently and transact repeatedly.

## Core Rules

### Optimize For Liquidity, Not For Size

Liquidity is the probability that a participant finds a match and transacts, and it is the true measure of marketplace health. Size without liquidity is a marketplace that looks busy but does not work.

Optimize for liquidity by:

- measuring match rate, time-to-match, and transaction completion, not just listings and users;
- ensuring supply and demand are balanced so neither side waits too long;
- concentrating density before spreading thin, so matches are actually possible;
- removing friction in discovery, trust, and transaction so matches convert.

A marketplace with a thousand listings and a ten percent match rate is healthier than one with a million listings and a one percent match rate. Liquidity, not volume, is the goal.

### Balance Supply And Demand Deliberately

Marketplaces fail when supply and demand are out of balance. Too much supply means providers cannot find enough demand and leave; too much demand means consumers cannot find supply and leave. Balance must be actively managed.

Balance by:

- measuring the ratio of supply to demand and its effect on match rate and time-to-match;
- subsidizing or recruiting the scarce side, often supply in early marketplaces;
- throttling the abundant side if it degrades the experience for the scarce side;
- recognizing that the right balance differs by category, geography, and time.

Imbalance is not self-correcting; it compounds, because the frustrated side leaves, worsening the imbalance for the other. Manage balance actively.

### Build Density Before Breadth

Marketplace density is the concentration of supply and demand within a geography, category, or network, and it is what makes matching efficient. Spreading thin across many regions or categories before achieving density in any one produces a marketplace that works nowhere.

Build density by:

- launching in a narrow geography or category where supply and demand can concentrate;
- achieving liquidity in that beachhead before expanding;
- expanding only when density and unit economics support it;
- resisting the temptation to be everywhere, which dilutes density everywhere.

Density is the prerequisite for liquidity, and liquidity is the prerequisite for a working marketplace. Win one dense market before attempting many thin ones.

### Set The Take Rate Against Value And Competition

The take rate, the platform's fee on each transaction, must reflect the value the platform adds, the competitive landscape, and the elasticity of both sides. Too high strangles transactions; too low leaves value uncaptured and may signal low value.

Set take rate by:

- pricing against the value the platform provides in discovery, trust, payments, and tools;
- considering what participants would pay to transact without the platform;
- recognizing that take rates often differ by side and by transaction type;
- being willing to start low to reach liquidity and raise as value is proven.

A take rate that ignores value and competition is either leaving money on the table or driving participants away. It is a strategic lever, not a fixed cost-plus margin.

### Manage Quality And Trust As Core Infrastructure

Marketplaces live or die on trust. Low-quality supply, fraud, and bad experiences drive away the demand that supply depends on, and the marketplace enters a death spiral.

Build trust by:

- verifying participants on both sides to reduce fraud and risk;
- using reviews and ratings to surface quality and penalize poor actors;
- providing dispute resolution and guarantees that protect participants;
- removing consistently poor participants to preserve marketplace quality.

Trust is not a feature; it is the infrastructure that makes participants willing to transact with strangers. Neglect it and the marketplace collapses.

### Distinguish Growth In Listings From Growth In Transactions

A common marketplace failure is celebrating listing or user growth while transaction growth lags. Listings without transactions indicate that supply is not finding demand, or that demand is not converting.

Diagnose by tracking:

- listing growth versus transaction growth, which should move together in a healthy marketplace;
- match rate and time-to-match, which reveal whether discovery is working;
- repeat transaction rate, which reveals whether the experience retains participants;
- the gap between active and total participants, which reveals churn.

A marketplace whose listings grow faster than transactions is accumulating supply that cannot find demand. This is a warning sign, not a success metric.

### Address The Supply And Demand Experience Separately

Supply and demand have different needs, different journeys, and different success metrics, and a marketplace must serve both deliberately. Optimizing for one side at the expense of the other unbalances the marketplace.

Serve both sides by:

- designing distinct experiences and tools for supply and demand;
- measuring success separately for each side, retention, satisfaction, earnings or value;
- recognizing when an improvement for one side harms the other;
- investing in the side that is the bottleneck, which is often supply early on.

A marketplace that serves only one side well will lose the other, and with it, its reason to exist.

### Plan For Marketplace Evolution Over Time

Marketplaces change as they mature: density grows, categories expand, and participant expectations shift. A strategy that worked at launch may fail at scale, and the marketplace must evolve.

Plan evolution by:

- anticipating how take rates, density, and quality demands change with scale;
- expanding categories or geographies only when density and economics support it;
- evolving trust and governance as the participant base grows and diversifies;
- watching for new forms of competition, including disintermediation at scale.

A static marketplace strategy decays. Design for the marketplace the business will become, not only the one it is today.

## Common Traps

### Chasing Size Over Liquidity

Listing and user growth without match rate and transaction growth is decay disguised as success.

### Supply-Demand Imbalance

Too much of either side frustrates the other and compounds into a death spiral. Balance actively.

### Spreading Thin Before Density

Expanding geographies or categories before achieving liquidity produces a marketplace that works nowhere.

### Mispricing The Take Rate

A take rate disconnected from value and competition either strangles transactions or leaves value uncaptured.

### Neglecting Trust And Quality

Without verification, reviews, and dispute resolution, bad actors drive away the participants the marketplace needs.

### Confusing Listings With Transactions

Listings growing faster than transactions is a warning sign, not a success metric.

### Optimizing One Side At The Other's Expense

Serving only supply or only demand unbalances the marketplace and loses the neglected side.

### Static Strategy As The Marketplace Matures

Strategies that worked at launch fail at scale. Evolve density, take rates, and governance over time.

## Self-Check

- [ ] Liquidity, match rate, time-to-match, and transaction completion are measured, not just listings and users.
- [ ] Supply and demand are balanced deliberately, with the scarce side subsidized or recruited.
- [ ] Density is achieved in a beachhead geography or category before expansion.
- [ ] The take rate reflects value added, competition, and the elasticity of both sides.
- [ ] Trust infrastructure, verification, reviews, dispute resolution, and quality control are core, not afterthoughts.
- [ ] Listing growth and transaction growth are tracked together, and divergence is treated as a warning.
- [ ] Supply and demand experiences are designed and measured separately, with the bottleneck side prioritized.
- [ ] Repeat transaction rate and participant retention are tracked, not just acquisition.
- [ ] The marketplace strategy anticipates evolution in density, take rates, and governance at scale.
- [ ] The marketplace is judged by whether matches happen and repeat, not by how large it appears.
