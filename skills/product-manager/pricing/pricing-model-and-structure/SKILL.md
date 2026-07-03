---
name: pricing_model_and_structure.md
description: Use when the agent is choosing a pricing model, deciding between subscription and usage and tiered and freemium structures, defining what is charged for and how, or evaluating how pricing structure shapes user behavior and revenue.
---

# Pricing Model And Structure

A pricing model is the structural decision about how a product charges: what unit value is measured, whether payment is recurring or one-time, how usage maps to cost, and which capabilities are paid. It is distinct from price-setting, which decides the actual numbers. The model is the architecture of the commercial relationship, and it shapes user behavior, revenue shape, and product direction far more than most teams realize.

Agents miss this because a pricing model feels like a finance decision made once. They pick subscription because it is the default, or usage because it sounds modern, without examining what behavior the model rewards and punishes. The harm is a model that penalizes the usage that creates value, that misaligns incentives between the company and the customer, or that locks the product into a revenue shape it cannot grow out of. Pricing structure is hard to change later, so a careless early choice compounds.

Use this skill before answering broad questions such as "how should we price this", "subscription or usage-based", "what should we charge per", "should we have a free tier", or "is our pricing model still right". The goal is to prevent the agent from choosing a model by default and to make the link between structure, behavior, and value explicit.

## Core Rules

### Understand What Each Model Incentivizes

Every pricing model rewards some behavior and discourages other behavior. The choice should be deliberate, based on what usage you want to encourage and what value you deliver.

Common models and their incentives:

- one-time purchase rewards shipping new versions to drive re-purchase, but disconnects ongoing revenue from ongoing value;
- subscription rewards retention and habit, but can charge the same whether the customer uses the product lightly or heavily;
- usage or metered pricing aligns cost to value consumed, but can make costs unpredictable and discourage the heavy use that signals success;
- per-seat pricing scales with adoption and team size, but can penalize inviting colleagues;
- tiered pricing lets customers self-select by need, but requires thoughtful gating;
- freemium uses a free tier for distribution, but must convert enough to fund the free base;
- hybrid models combine several, often a base subscription plus usage overages.

Ask of any model: what does this make the customer want to do more of, and less of, and is that what we want?

### Choose A Value Metric That Aligns To Value Received

The unit you charge per, the value metric, should grow as the value the customer receives grows. A misaligned metric charges customers more without giving them more value, or delivers more value without capturing it.

A good value metric:

- increases as the customer gets more value from the product;
- is understandable and predictable to the buyer;
- is measurable reliably by the product;
- aligns with how the customer thinks about their own growth.

Charging per seat for a collaboration tool aligns value and cost. Charging per project for a tool where users create hundreds of trivial projects punishes normal use. The metric is the hinge between customer success and revenue; get it wrong and the two pull apart.

### Use Good-Better-Best And Feature Gating Deliberately

Tiered packaging, good-better-best, lets customers self-select and creates natural upgrade paths, but only if the gating is principled. Arbitrary gating confuses buyers and creates support friction.

Principles for gating:

- gate by segment and value, so each tier serves a distinct buyer;
- put the features that define each tier's value in that tier, not scattered across;
- make the upgrade trigger natural, so growth in usage or need leads to the next tier;
- avoid gating features that are table stakes, which feels punitive;
- keep the number of tiers disciplined, since each adds sales and support complexity.

The structure of tiers is a product decision. It should be designed and tested, not assembled from leftover features.

### Connect Pricing Structure To Retention And Usage

Pricing structure affects whether customers stay and how deeply they use the product. A model that punishes usage discourages the engagement that drives retention; a model that decouples cost from value invites churn when the customer reassesses.

Examine:

- does the model reward the customer for getting more value, or penalize them?
- does the bill surprise the customer, and do surprises drive churn?
- does the structure create natural expansion as the customer grows?
- does the model make downgrade or churn feel fair or adversarial?

The pricing model is part of the retention loop. A model that feels fair and grows with value compounds; a model that feels extractive erodes the relationship.

### Recognize How Pricing Constrains Or Enables Product Direction

A pricing model is not neutral toward roadmap. It shapes what the company is incentivized to build, because features that increase the value metric increase revenue, and features that do not are harder to justify.

Ask:

- does the model reward building the features that create the most customer value?
- does it create perverse incentives, such as building friction that increases the metric?
- will the model still fit as the product expands into new use cases or segments?
- is the model flexible enough to evolve, or does it lock in a single dimension of value?

A model chosen for today's product can become a ceiling tomorrow. Build in the ability to evolve the structure as the product and market mature.

### Avoid Incentive Misalignment

The most damaging pricing models are those where the company's interest conflicts with the customer's. If revenue rises when customers fail, when they waste resources, or when they cannot use the product efficiently, the model is misaligned and will corrode trust.

Watch for:

- pricing that profits from customer mistakes or waste;
- pricing that discourages the usage that signals a healthy customer;
- pricing that makes the customer ration a feature that should be used freely;
- pricing where the company benefits from customer confusion.

Aligned pricing makes the company win when the customer wins. Misaligned pricing extracts value rather than creating it.

### Know When To Change The Model

A pricing model is not permanent, but changing it is costly and disruptive, so the trigger should be clear. Change the model when the value metric no longer reflects value, when the model blocks a strategic direction, when retention suffers because of structure, or when the market has moved to a different expectation.

Plan a model change as a project, not a tweak:

- assess migration impact on existing customers;
- decide grandfathering and transition terms;
- prepare sales, support, and billing systems;
- communicate the change in terms of value, not just cost.

Do not change the model on a hunch. Change it when the evidence shows the structure itself is the problem.

## Common Traps

### Defaulting To Subscription Without Examining Fit

The team adopts subscription because it is familiar, then discovers the product delivers value in bursts and customers churn between them, or that heavy users cost more to serve than the flat price covers.

### Choosing A Value Metric That Punishes Value

The metric charges for the very usage that indicates a successful customer, so the pricing discourages the behavior the product exists to enable.

### Arbitrary Feature Gating

Features are distributed across tiers by convenience rather than logic, so buyers cannot tell which tier fits them and sales spends its time explaining the matrix.

### Ignoring How Structure Shapes Retention

The model looks fine at the point of sale but creates surprise bills or rationing behavior that drives churn months later.

### Letting Pricing Incentivize The Wrong Roadmap

Because revenue tracks a particular metric, the team builds features that move the metric rather than features that create value, and the product drifts.

### Profiting From Customer Failure

The model makes more money when customers waste resources or fail to use the product well, aligning the company against the customer and eroding trust.

### Treating The Model As Permanent

The team lives with a broken model for years because changing pricing feels too hard, until the misalignment becomes a strategic bottleneck.

## Self-Check

- [ ] The pricing model was chosen by examining what behavior it rewards and punishes, not by default or fashion.
- [ ] The value metric grows with the value the customer receives and is understandable, predictable, and measurable.
- [ ] The model does not penalize the usage that signals a healthy, successful customer.
- [ ] Tiered packaging and feature gating are principled by segment and value, with natural upgrade triggers.
- [ ] The structure supports retention and natural expansion rather than creating surprise or rationing that drives churn.
- [ ] The model's incentives align with the roadmap, rewarding features that create customer value rather than features that merely move the metric.
- [ ] The company does not profit from customer failure, waste, or confusion.
- [ ] The model is flexible enough to evolve as the product, market, and segments mature.
- [ ] If a model change is under consideration, migration, grandfathering, and enablement are planned, not improvised.
- [ ] The chosen structure was reviewed against how the target customer actually buys and consumes value.
