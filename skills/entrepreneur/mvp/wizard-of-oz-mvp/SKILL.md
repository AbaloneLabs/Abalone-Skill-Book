---
name: wizard_of_oz_mvp.md
description: Use when the agent is building, evaluating, or advising on a Wizard of Oz MVP, where the product appears automated to users but is operated manually behind the scenes to learn whether the experience is valuable before building the automation, deciding what to fake versus what to build, managing the deception ethically, or knowing when the learning justifies the real investment.
---

# Wizard of Oz MVP

A Wizard of Oz MVP presents users with an experience that appears automated and complete, while the actual work is done manually behind the scenes. Like the concierge MVP, it tests whether an experience is valuable before investing in the automation that would deliver it at scale. The distinction is that the Wizard of Oz hides the manual labor entirely, presenting a fully automated front, where the concierge may be transparent about human involvement. The judgment problem is choosing Wizard of Oz when the value depends on the experience of automation, designing the illusion so it tests the right hypothesis, managing the ethical and operational cost of the deception, and graduating to real software once the learning is clear. Agents tend to fail by faking the wrong layer, by letting the manual operation become unsustainable, and by treating the illusion as a product rather than a probe.

Use this skill when building, evaluating, or advising on a Wizard of Oz MVP. The goal is to learn whether the automated experience is valuable, cheaply, and to build the real thing only when the answer justifies it.

## Core Rules

### Choose Wizard Of Oz When The Value Depends On The Automated Experience

The Wizard of Oz is the right tool when the value proposition depends on the experience of an automated, instant, or seamless product, and building that automation before validating demand would be reckless. If the value does not depend on the automation feeling real, a concierge or simpler test may suffice.

Wizard of Oz is appropriate when:

- the core hypothesis is that users value the automated experience itself, such as instant results or seamless delivery;
- building the real automation is expensive or technically risky, and validating demand first reduces risk;
- the manual operation can approximate the automated experience closely enough that users respond to the real thing;
- the cost of operating manually for a small user base is far less than building the wrong automation.

Wizard of Oz is wrong when the manual operation cannot approximate the automated experience, when the value depends on scale the manual operation cannot provide, or when the deception is unethical for the context.

### Fake The Layer That Tests The Hypothesis, Build The Rest

The Wizard of Oz fakes the specific layer whose automation is expensive and whose value is uncertain, while building the surrounding experience well enough that the illusion holds. Faking the wrong layer, or faking everything, wastes effort and muddies the learning.

Fake selectively by:

- identifying the one layer whose automation is the expensive uncertainty, such as the intelligence, matching, or generation;
- building the surrounding interface, flow, and delivery well enough to be plausible;
- operating the faked layer manually so the user experiences the intended outcome;
- ensuring the user cannot distinguish the manual operation from the eventual automation.

The art is faking exactly the layer that matters and building the rest to support the illusion. Faking too much produces a brittle, unsustainable operation; faking too little exposes the manual labor.

### Ensure The Illusion Approximates The Real Experience Closely

The user's response is valid only if the experience they have approximates the real product closely enough that their engagement reflects genuine demand. If the manual operation introduces latency, errors, or quality gaps, the learning is contaminated.

Approximate the real experience by:

- matching the latency the real automation would produce, not slower;
- matching the quality and consistency the real product would deliver;
- ensuring the interface and flow match the intended product;
- avoiding signals that reveal the manual operation, unless transparency is part of the test.

A Wizard of Oz that delivers a worse experience than the real product would underestimates demand. One that delivers a better experience, because humans do the work carefully, overestimates it. Calibrate the illusion to the real product's likely performance.

### Manage The Ethical Dimension Of The Deception

The Wizard of Oz involves deception: users believe they are using an automated product when humans are doing the work. For a short, bounded learning probe with early users, this is usually acceptable, but the ethical line depends on context and must be considered deliberately.

Manage ethics by:

- bounding the deception to a short learning period with a small user base;
- being transparent in research contexts where users expect to be studied;
- avoiding contexts where the deception could cause harm, such as health, finance, or safety;
- being prepared to disclose the nature of the MVP if asked, rather than lying.

The deception of a bounded learning probe is generally defensible; sustained deception of a paying customer base is not. Keep the Wizard of Oz short and bounded.

### Bound The Manual Operation Before It Becomes Unsustainable

The manual operation behind a Wizard of Oz is labor-intensive and does not scale, and operating it for too long or for too many users burns out the team and produces no path to the real product. The operation must be bounded from the start.

Bound the operation by:

- limiting the number of users to what the team can serve at real-product quality;
- setting a defined duration for the Wizard of Oz, after which a decision is made;
- tracking the labor cost per user to understand the unit economics of the real automation;
- resisting growth in users, because each new user adds manual load without advancing the learning.

A Wizard of Oz that grows users without limit becomes an unsustainable manual operation that consumes the team. Bound it to preserve its purpose as a probe.

### Measure The Signals That Prove The Automated Experience Is Valuable

The Wizard of Oz tests whether the automated experience is valuable, and the signals that prove it are specific to that hypothesis. Vanity metrics miss the point.

Measure:

- engagement: do users use the experience as intended, at the frequency the real product would expect;
- retention: do users return, indicating sustained value in the experience;
- willingness to pay: will users pay for the automated experience, at the price the real product would charge;
- outcome quality: does the experience actually deliver the value it promises.

Signups and curiosity-driven one-time use do not prove the automated experience is valuable. Retention, payment, and outcome quality do. Design the Wizard of Oz to surface these signals.

### Know When To Graduate To Real Automation

The Wizard of Oz has a defined endpoint, and continuing it past that point wastes the learning and strains the team. The decision to build real automation should follow clear evidence.

Graduate when:

- the experience is validated: users engage, return, and pay;
- the unit economics of the manual operation inform what the automation must achieve;
- the features and flows that matter are clear from what users actually used;
- the team is ready to invest in automation with confidence in the demand.

If the experience is not validated, the Wizard of Oz has prevented building automation no one wants. If it is validated, build the real thing. Do not linger in the illusion.

### Use The Learning To Specify The Real Automation

The Wizard of Oz's output is not just a go or no-go; it is detailed learning about what the automation must do, how fast it must be, and what quality it must deliver. Capture that learning to build the right system.

Capture learning by:

- documenting the workflows the manual operation performed, to specify the automation;
- recording the latency and quality users accepted or rejected, to set performance requirements;
- noting the features users used and ignored, to scope the build;
- translating the manual operation into a technical specification.

A Wizard of Oz that produces only a verdict wastes the richest learning. Extract the specification it generates.

## Common Traps

### Wizard Of Oz When The Value Does Not Depend On Automation

If the value does not depend on the automated experience, a simpler test suffices. Use Wizard of Oz when the automation is the point.

### Faking The Wrong Layer

Faking everything is unsustainable; faking the wrong layer muddies the learning. Fake exactly the expensive, uncertain layer.

### An Illusion That Does Not Approximate The Real Product

Latency, errors, or quality gaps contaminate the learning. Calibrate the illusion to the real product's likely performance.

### Unethical Or Unbounded Deception

Sustained deception of paying customers is indefensible. Keep the Wizard of Oz short, bounded, and context-appropriate.

### Unsustainable Manual Operation

Growth in users adds load without advancing learning. Bound the operation from the start.

### Measuring Curiosity Instead Of Value

One-time curiosity-driven use does not prove the experience is valuable. Measure retention, payment, and outcome.

### Lingering In The Illusion

Once the hypothesis is validated or refuted, graduate. Do not extend the manual operation indefinitely.

### Ignoring The Specification The Operation Generates

The manual workflows specify the automation. Capture the performance, quality, and feature requirements, not just a verdict.

## Self-Check

- [ ] Wizard of Oz was chosen because the value depends on the automated experience, not merely on the outcome.
- [ ] The faked layer is the expensive, uncertain one, and the surrounding experience is built to support the illusion.
- [ ] The illusion approximates the real product's latency, quality, and flow closely enough to produce valid learning.
- [ ] The deception is bounded to a short period with a small user base, and is context-appropriate.
- [ ] The manual operation is bounded in users and duration to remain a probe, not a business.
- [ ] The signals measured are engagement, retention, payment, and outcome quality, not vanity metrics.
- [ ] A clear graduation decision point is defined, with criteria for building real automation or stopping.
- [ ] The operation has not become an unsustainable manual business that consumes the team.
- [ ] Learning about workflows, performance requirements, and feature scope is captured to specify the automation.
- [ ] The decision to build automation follows validated demand, not indefinite manual operation.
