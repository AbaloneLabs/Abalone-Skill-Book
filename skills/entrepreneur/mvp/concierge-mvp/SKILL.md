---
name: concierge_mvp.md
description: Use when the agent is building, evaluating, or advising on a concierge MVP, where the product is delivered manually behind a simple interface to learn whether the value proposition holds before building automation, deciding what to learn versus what to fake, managing the cost of manual delivery, or knowing when to graduate from concierge to real software.
---

# Concierge MVP

A concierge MVP delivers the product's value to early customers manually, behind a simple interface, to learn whether the value proposition actually holds before investing in the automation that would make it scalable. It is a learning instrument, not a product, and its power is precisely that it lets a founder test the core hypothesis with real users and real transactions without building the system that would eventually deliver it. The judgment problem is choosing concierge when the risk is that no one wants the outcome, not when the risk is that the software is hard to build; designing the delivery so it isolates the value hypothesis; and knowing when the learning is sufficient to justify building the real thing. Agents tend to fail by overbuilding the concierge interface, by failing to isolate the hypothesis being tested, and by letting the manual delivery become a permanent crutch rather than a temporary probe.

Use this skill when building, evaluating, or advising on a concierge MVP. The goal is to learn whether the value proposition holds, cheaply and quickly, and to graduate to real software when the answer is clear.

## Core Rules

### Choose Concierge When The Risk Is Demand, Not Feasibility

A concierge MVP is the right tool when the central uncertainty is whether anyone wants the outcome, not whether the software can be built. If the risk is technical feasibility, a concierge MVP tests the wrong thing, because it removes the very difficulty that determines whether the product can exist.

Concierge is appropriate when:

- the core hypothesis is that customers value the delivered outcome, and the automation is secondary;
- the software to automate delivery is expensive or complex, and building it before validating demand is reckless;
- the manual delivery can be hidden behind a simple interface so the experience approximates the real product;
- the cost of manual delivery for a small number of early customers is far less than the cost of building the wrong product.

Concierge is wrong when the technical challenge is the product, when the value depends on scale that manual delivery cannot provide, or when the manual delivery cannot approximate the real experience closely enough to test the hypothesis.

### Isolate The Value Hypothesis From Everything Else

The concierge MVP exists to test one thing: whether the delivered outcome is valuable enough that customers will engage, pay, and return. Everything else, the interface polish, the breadth of features, the operational sophistication, should be minimized so the result isolates that hypothesis.

Isolate by:

- defining the single value hypothesis the concierge is testing, in one sentence;
- delivering that value manually with the simplest possible customer-facing interface;
- stripping away anything that does not test the core hypothesis;
- resisting the urge to build a polished product before the hypothesis is validated.

A concierge MVP that tests many things tests nothing cleanly. The discipline is to probe one hypothesis with maximum clarity.

### Hide The Manual Delivery Behind A Plausible Interface

The customer should experience something close to the real product, not a visibly manual process that biases their response. If customers know a human is doing the work, their feedback reflects the novelty rather than the value.

Hide the manual work by:

- using a simple but real interface, such as a form, email, or basic web app, that the customer interacts with;
- delivering the result through the same channel the real product would use;
- avoiding signals that reveal the manual labor unless transparency is itself part of the test;
- ensuring the latency and quality approximate what the real product would deliver.

The interface need not be sophisticated, but it must be plausible enough that the customer's engagement reflects genuine demand rather than curiosity about a manual service.

### Manage The Cost And Scale Of Manual Delivery

Concierge delivery is expensive in human time, and it does not scale. The point is to learn from a small number of customers, not to build a manual business, and the cost must be bounded so it remains a probe rather than a burden.

Manage cost by:

- limiting the number of concierge customers to what the team can serve well;
- charging for the service to test willingness to pay, not offering it free;
- tracking the time and cost of each delivery to understand the unit economics;
- setting a clear endpoint at which the concierge concludes and a decision is made.

A concierge MVP that grows without limit becomes a manual services business that consumes the team and never produces the learning needed to build software. Bound it deliberately.

### Measure The Right Signals: Value, Retention, And Willingness To Pay

The concierge MVP tests whether the delivered outcome is valuable, and the signals that prove value are specific. Vanity metrics like signups or engagement with the interface miss the point.

Measure:

- willingness to pay: do customers pay for the manually delivered outcome, and at what price;
- retention: do customers return for repeat delivery, indicating sustained value;
- outcome quality: does the delivered result actually solve the customer's problem;
- referral: would customers recommend the service, indicating strong perceived value.

Signups and one-time transactions do not prove the value proposition. Payment, retention, and referral do. Design the concierge to surface these signals.

### Know When To Graduate From Concierge To Real Software

The concierge MVP has a defined endpoint, and continuing it past that point wastes the learning. The decision to build real software should follow clear evidence, not indefinite manual delivery.

Graduate when:

- the value hypothesis is validated: customers pay, return, and refer;
- the unit economics of manual delivery are understood, informing the automated model;
- the features that matter are clear, learned from what customers actually used;
- the team is ready to invest in automation with confidence in the demand.

If the hypothesis is not validated, the concierge has done its job by preventing the team from building something no one wants. If it is validated, graduate. Do not linger in manual delivery.

### Avoid The Manual Services Trap

A concierge MVP can drift into a manual services business, where the team delivers the product manually to a growing customer base and never builds the software that would make it scalable. This is a trap, because the manual business consumes all available capacity and produces no path to scale.

Avoid the trap by:

- bounding the number of customers and the duration of the concierge;
- treating manual delivery as learning, not as revenue;
- resisting the temptation to grow a manual customer base because it pays;
- committing to a graduation decision at a defined point.

The concierge exists to answer a question, not to become a business. Keep it bounded.

### Use The Learning To Inform The Real Product

The concierge MVP's output is not just a go or no-go decision; it is detailed learning about what customers value, what features matter, and how the product should work. Capture that learning to build the right software.

Capture learning by:

- documenting what customers asked for, used, and ignored;
- recording the workflows that delivered value manually, to inform automation;
- noting the price points and packaging that resonated;
- translating the manual delivery into a specification for the automated product.

A concierge MVP that produces only a yes or no wastes the richest learning. Extract the design intelligence it generates.

## Common Traps

### Concierge When The Risk Is Feasibility, Not Demand

If the technical challenge is the product, concierge tests the wrong thing. Use it when demand is the uncertainty.

### Testing Many Hypotheses At Once

A concierge that probes many things tests nothing cleanly. Isolate one value hypothesis.

### Overbuilding The Interface

Polish and features distract from the hypothesis. Keep the customer-facing interface minimal.

### Visible Manual Delivery Biasing Feedback

If customers know a human is doing the work, their response reflects novelty, not value. Hide the labor.

### Unbounded Manual Delivery

A concierge that grows becomes a manual services business that never scales. Bound the customers and duration.

### Measuring Signups Instead Of Value

Signups and one-time use do not prove the proposition. Measure payment, retention, and referral.

### Lingering In Concierge Past The Decision Point

Once the hypothesis is validated or refuted, graduate. Do not extend manual delivery indefinitely.

### Ignoring The Learning Beyond Go Or No-Go

The concierge produces design intelligence, not just a verdict. Capture what to build, not just whether to build.

## Self-Check

- [ ] Concierge was chosen because the core uncertainty is demand for the outcome, not technical feasibility.
- [ ] A single value hypothesis is defined and isolated, not bundled with other tests.
- [ ] The manual delivery is hidden behind a plausible interface that approximates the real product.
- [ ] The number of concierge customers and the duration are bounded to keep delivery a probe, not a business.
- [ ] Customers are charged, to test willingness to pay, not offered the service free.
- [ ] The signals measured are value, retention, and referral, not vanity metrics like signups.
- [ ] A clear graduation decision point is defined, with criteria for building real software or stopping.
- [ ] The concierge has not drifted into a manual services business that consumes the team.
- [ ] Learning about features, workflows, pricing, and packaging is captured to inform the automated product.
- [ ] The decision to build software follows validated demand, not indefinite manual delivery.
