---
name: fake_quality_and_illusion_engineering.md
description: Use when the agent is deciding what to build for real versus fake behind the scenes in an MVP, choosing between prototypes mockups concierge and Wizard of Oz approaches, managing the operational cost and ethical limits of manual delivery, deciding when manual is sustainable and when it must be automated, or diagnosing why an MVP that works in demo fails under real customer load.
---

# Fake Quality And Illusion Engineering

An MVP's purpose is learning, not production. To learn cheaply and quickly, a startup often needs to deliver an experience that feels complete to the customer while being operated manually or partially faked behind the scenes. This is illusion engineering: the deliberate choice to build the appearance of a finished product using humans, spreadsheets, manual triggers, or hardcoded outputs, so that the team can learn whether the experience is valuable before investing in the automation that would make it real. The judgment problem is deciding what must be genuinely built because it is core to the value or because faking it is infeasible, and what can be faked because faking it is cheaper and still produces valid learning. The failure modes cut both ways: building too much real infrastructure before learning wastes money, while faking something that is actually the core value produces false confidence in a product that cannot scale.

Use this skill when designing an MVP, when deciding what to build versus fake, when running a concierge or Wizard of Oz MVP, when the operational cost of manual delivery is growing, or when diagnosing why an MVP validated but the real product failed. The goal is to prevent the agent from over-building real systems before learning, and to keep it from faking the very thing the product is supposed to deliver.

## Core Rules

### Decide What Is Core Value Versus Supporting Plumbing

The first decision is which parts of the experience the customer is actually paying for and which are infrastructure that enables the value. Core value must eventually be real, but it can sometimes be faked for learning. Supporting plumbing should almost always be faked or bought, never built, in an MVP.

Classify each component:

- core value: the outcome or experience the customer came for, such as the analysis, the match, the content, the transaction;
- supporting plumbing: authentication, billing, dashboards, notifications, integrations, infrastructure;
- table stakes: features the customer expects to exist but that do not differentiate, such as password reset or email receipts.

Build the core value to a level that produces valid learning. Fake or defer the plumbing. Never build table stakes beyond the minimum that lets the core value be experienced.

### Fake The Plumbing, Buy The Plumbing, Or Defer The Plumbing

Authentication, payments, email, hosting, and dashboards are solved problems. Building them for an MVP is almost always a misuse of scarce engineering time. The exception is when the plumbing itself is the core value, such as a security product where authentication is the point.

Handle plumbing by:

- using off-the-shelf services for auth, payments, email, and hosting;
- deferring dashboards and admin tools until they are needed to operate;
- faking settings and configuration screens with hardcoded values during learning;
- using no-code or low-code tools to assemble the experience without custom code.

The test is whether building the plumbing produces learning. If not, it is not worth building in an MVP.

### Decide Whether The Core Value Can Be Faked Validly

Some core value can be faked without invalidating the learning, because the customer experiences the outcome without needing to know how it was produced. Other core value cannot be faked, because the production method is part of what the customer is evaluating, or because faking it would not scale to the test the team needs.

Ask, for each core component:

- does the customer care how the result is produced, or only that it is produced;
- can a human produce the result manually within the response time the customer expects;
- is the manual production feasible for the number of customers in the test;
- would faking it produce false confidence about scalability, accuracy, or cost.

A recommendation engine can be faked by a human curating results. A real-time fraud detection system cannot be faked, because latency and scale are part of the value. Match the faking strategy to whether the production method is load-bearing in the value proposition.

### Manage The Operational Cost Of Manual Delivery

Concierge and Wizard of Oz MVPs shift cost from engineering to operations. A human doing the work behind the scenes is sustainable for a handful of customers and unsustainable for hundreds. The operational cost must be tracked, because an MVP that requires ten engineer-hours per customer cannot teach anything about a scalable business.

Track and bound manual cost by:

- measuring the time and effort required per customer per transaction;
- setting a ceiling on the number of customers the manual operation can serve;
- defining the learning threshold at which the manual approach has served its purpose;
- planning the graduation to automation before the manual cost becomes a crisis.

A concierge MVP that grows beyond its operational capacity without graduating to software has stopped being an experiment and become an unsustainable service business.

### Be Honest About The Ethics And Transparency Of Faking

A Wizard of Oz MVP presents an automated experience that is actually manual. This is a legitimate learning technique, but it has ethical limits. Customers who believe they are using a finished product and make decisions based on that belief, such as committing data or money, may be harmed if the manual operation fails or disappears.

Manage the ethics by:

- being transparent where the customer's reliance is high, such as with financial or health data;
- avoiding faking in regulated domains where the production method is legally material;
- setting expectations about the product's maturity where appropriate;
- having a clear plan and timeline for making the faked component real or shutting it down.

Faking is acceptable for learning; deceiving customers into material reliance on a product that cannot persist is not.

### Define The Graduation Criteria From Fake To Real

An MVP that fakes components must have explicit criteria for when to invest in making them real. Without graduation criteria, the team either over-invests prematurely or runs the manual operation indefinitely past its useful life. Graduation criteria tie the build decision to the learning outcome.

Define graduation by:

- the learning outcome that would justify building the real version, such as confirmed willingness to pay at a target price;
- the operational threshold that forces building, such as customer count exceeding manual capacity;
- the quality or accuracy threshold that manual delivery can no longer meet;
- the timeline beyond which continuing to fake is no longer acceptable.

A team without graduation criteria is either building blindly or faking indefinitely.

### Ensure The MVP Tests The Scalable Claim, Not Just The Manual Claim

A subtle failure is an MVP that validates the manual experience but not the scalable one. If the core value depends on speed, cost, or scale that only automation can provide, a manual MVP may produce positive learning that does not transfer. The team must be clear about which claims the MVP can and cannot test.

Distinguish by:

- identifying which aspects of the value depend on automation, such as latency, cost per unit, or volume;
- noting that a manual MVP cannot validate those aspects and must be supplemented by feasibility analysis;
- avoiding the inference that because the manual experience was valued, the automated one will be viable.

A concierge MVP validates desirability. It does not validate the unit economics or scalability that automation would determine.

## Common Traps

### Building Real Plumbing Before Learning

Authentication, billing, and dashboards built from scratch consume engineering time without producing learning. Buy, fake, or defer them.

### Faking The Core Value

If the production method is part of what the customer evaluates, faking it produces false confidence. Match the faking strategy to whether the method is load-bearing.

### Letting Manual Cost Grow Unbounded

A concierge MVP that scales beyond manual capacity becomes an unsustainable service. Track per-customer operational cost and set a ceiling.

### Deceiving Customers Into Material Reliance

Faking is acceptable for learning; deceiving customers into relying on a product that cannot persist, especially with sensitive data, is not.

### No Graduation Criteria

Without explicit criteria for when to build the real version, the team either builds prematurely or fakes indefinitely.

### Inferring Scalability From A Manual MVP

A manual MVP validates desirability, not the cost, speed, or scale that automation would determine. Do not over-generalize.

### Over-Building Table Stakes

Password resets, settings screens, and admin tools built beyond the minimum distract from the core learning without adding value.

## Self-Check

- [ ] Each component is classified as core value, supporting plumbing, or table stakes, and treated accordingly.
- [ ] Plumbing is bought, faked, or deferred rather than built custom unless it is itself the core value.
- [ ] The decision to fake each core component considers whether the production method is load-bearing in the value proposition.
- [ ] The operational cost per customer is measured and bounded by a ceiling the manual approach can sustain.
- [ ] The ethical limits of faking are respected, with transparency where customer reliance is high or data is sensitive.
- [ ] Graduation criteria define when to invest in making faked components real, tied to learning outcomes and operational thresholds.
- [ ] The MVP's claims are scoped to what the approach can test; scalability claims are not inferred from a manual MVP.
- [ ] No table-stakes feature has been built beyond the minimum needed to experience the core value.
- [ ] The team can state, for each faked component, what learning it enables and when it must become real.
- [ ] The MVP has not consumed engineering on infrastructure that produces no learning about the core hypothesis.