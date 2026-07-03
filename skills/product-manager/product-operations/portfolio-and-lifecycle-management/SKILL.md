---
name: portfolio_and_lifecycle_management.md
description: Use when the agent is managing a portfolio of products or features across their lifecycle, deciding what to maintain, mature, sunset, or invest in, or coordinating product decisions across multiple teams and product lines.
---

# Portfolio And Lifecycle Management

A product portfolio is a living system, not a static catalog. Every product and feature moves through a lifecycle from introduction through growth, maturity, harvest, and eventual sunset, and each stage demands different investment, metrics, and attention. The failure mode of most portfolios is asymmetry: teams are excellent at launching new things and terrible at retiring old ones, so maintenance cost accumulates, attention fragments, and the portfolio slowly drowns the strategy it was meant to execute.

Use this skill before making decisions about what to invest in, maintain, mature, or sunset across a portfolio, or before coordinating priorities across multiple teams and product lines. The goal is to prevent the agent from treating every product as perpetually in growth, from underestimating the carrying cost of legacy, from sunsetting in a way that destroys customer trust, or from allocating resources as if each product existed in isolation. Use it when the team is reviewing portfolio health, planning a sunset, managing cross-product dependencies, or setting objectives that match each item's true lifecycle stage.

## Core Rules

### Treat Each Item By Its Lifecycle Stage

A feature in introduction needs different investment, metrics, and tolerance than one in maturity or harvest. Applying a single management style across the portfolio is the most common portfolio error, because it either starves new bets or over-invests in declining ones. Lifecycle stage should drive how you set objectives, how you staff, and what you measure.

Stage-appropriate focus:

- introduce: validate problem-solution fit, learn fast, tolerate ambiguity;
- grow: scale acquisition and activation, harden reliability, build moat;
- mature: optimize efficiency, defend share, harvest margin, control cost;
- harvest: extract value with minimal investment, prepare migration paths;
- sunset: communicate, migrate, and retire with discipline.

Name the stage of every significant product and feature explicitly, and revisit the label on a regular cadence so items do not silently drift.

### Make Retirement A First-Class Decision

Most portfolios only grow because retiring is harder than launching. Sunsetting requires admitting decline, managing migration, and absorbing short-term complaint, so it gets postponed until legacy cost becomes unbearable. Treating retirement as a normal, scheduled decision is what keeps a portfolio healthy.

For each mature or declining item, periodically ask:

- what is the true maintenance and support cost;
- what strategic value does it still create;
- what would happen to dependent customers if it were retired;
- is there a viable migration path;
- what is the cost of keeping it another year.

If an item is carried only by inertia or fear, that is the signal to plan a disciplined sunset rather than another year of quiet decay.

### Account For The Real Cost Of Legacy

Legacy products and features are not free once shipped. They consume engineering attention, support load, documentation upkeep, security and compliance work, and cognitive overhead, all of which is invisible in a roadmap that only counts new launches. The carrying cost of legacy is the hidden tax that starves new investment.

Make carrying cost visible by tracking, per legacy item:

- engineering hours spent on maintenance and bugs;
- support ticket volume and resolution time;
- security, compliance, and infra cost;
- opportunity cost of the team tied to it.

When the carrying cost is quantified, the decision to retire or reinvest becomes concrete instead of emotional.

### Sunset With Discipline And Trust

A badly handled sunset damages trust far beyond the product being retired. Customers who built workflows on a feature, integrated against an API, or committed budget to a product feel betrayed when retirement is abrupt, poorly communicated, or lacks a migration path. The way you retire something is remembered longer than the way you launched it.

A disciplined sunset includes:

- a clear, early announcement with dates and rationale;
- a realistic migration path or replacement;
- a transition window long enough for real customers;
- data export and continuity guarantees;
- direct outreach to high-impact accounts;
- support enablement for the transition.

Never sunset by simply going dark. The communication and migration plan is the product work of the sunset stage.

### Manage Cross-Product Dependencies And Shared Platforms

In a multi-product portfolio, dependencies are where surprise delays and broken promises live. Shared platforms, common identity, shared data models, and cross-product experiences all create coupling that a single product team cannot see alone. Portfolio management exists largely to make these dependencies legible and to govern the shared layers deliberately.

Identify and govern:

- shared platforms and their roadmap dependencies;
- common infrastructure that multiple products rely on;
- cross-product experiences that require coordinated releases;
- shared data, identity, and permission models;
- teams whose roadmap blocks or enables others.

Make dependency commitments explicit and owned, because implicit dependencies become silent bottlenecks that surface as missed dates.

### Allocate Resources By Portfolio Strategy, Not By Inertia

Resource allocation across a portfolio should follow strategy, but in practice it follows history. The team that has always been large stays large, the product that always got attention keeps getting it, and new bets are starved because the budget was already spent. Deliberate reallocation is the core lever of portfolio management.

Reallocate against:

- the lifecycle stage and strategic priority of each item;
- the expected return and risk of each bet;
- the carrying cost being freed by sunsets;
- the dependencies and platform investments that unlock multiple products.

Review allocation on a fixed cadence so that yesterday's priorities do not automatically become tomorrow's budget.

### Set Stage-Appropriate Objectives And Metrics

The objectives that fit a growth-stage product will mislead a mature one, and the efficiency metrics that fit a harvesting product will starve an introducing one. Each item's goals must match its stage, or the portfolio will be managed against the wrong definition of success.

Match metrics to stage:

- introduce: learning, validation, early engagement signals;
- grow: activation, retention, market share, velocity;
- mature: margin, efficiency, churn defense, share retention;
- harvest: cost-to-serve, margin extracted, migration progress;
- sunset: migration completion, customer continuity, trust preserved.

A single north-star metric applied across all stages produces distorted incentives and hides decline behind vanity growth.

## Common Traps

### Only Adding, Never Retiring

Portfolios naturally accumulate because launching is celebrated and retiring is painful. The trap is a portfolio that grows forever in surface area while the team's capacity stays fixed, until maintenance consumes all new investment.

### Applying Growth Metrics To Mature Products

Forcing a mature product to hit growth targets leads to gimmicks, forced feature sprawl, and frustrated teams. The trap is refusing to accept that a product's stage has changed and managing it against goals it can no longer honestly meet.

### Underestimating Carrying Cost

Legacy looks cheap because its cost is spread across many teams and rarely aggregated. The trap is keeping a declining product because no one has added up the real engineering, support, and opportunity cost it absorbs.

### Sunsetting Abruptly And Destroying Trust

A sudden deprecation with no migration path saves the retiring team effort but costs the company trust across every product. The trap is optimizing the sunset for internal convenience rather than for customer continuity.

### Ignoring Hidden Cross-Product Dependencies

When each team plans in isolation, shared platform and integration dependencies surface only as missed launches. The trap is treating portfolio management as a roll-up of independent plans rather than as governance of the coupled layers between them.

### Allocating By History Instead Of Strategy

Budget and headcount follow last year's distribution because reallocation is politically easier to avoid. The trap is a portfolio where new strategic bets are perpetually underfunded while declining products retain their historic share.

### Labeling Everything As Growth

Calling every product a growth bet avoids the uncomfortable work of classifying decline. The trap is a portfolio with no harvest or sunset stage, which means nothing ever gets the disciplined retirement it needs.

## Self-Check

- [ ] Every significant product and feature has an explicitly named lifecycle stage that drives its objectives.
- [ ] Retirement is treated as a scheduled, normal decision, not an admission of failure to be avoided.
- [ ] The carrying cost of legacy items is quantified, including engineering, support, security, and opportunity cost.
- [ ] Each planned sunset has a communication plan, migration path, transition window, and data continuity guarantee.
- [ ] Cross-product dependencies and shared platforms are identified, owned, and made explicit in roadmaps.
- [ ] Resource allocation is reviewed on a fixed cadence and reallocated against strategy rather than history.
- [ ] Objectives and metrics are matched to each item's lifecycle stage, not applied uniformly.
- [ ] Mature and declining items are not being forced to meet growth-stage targets.
- [ ] No product is being carried solely by inertia or fear of complaint without a reviewed retirement plan.
- [ ] The portfolio is actively balanced across introduce, grow, mature, harvest, and sunset rather than only growing.
