---
name: target_segment_and_persona_definition.md
description: Use when the agent is defining a target customer segment, building personas, choosing which users to serve first, deciding segment boundaries, or evaluating whether a segment is large and reachable enough to focus on.
---

# Target Segment And Persona Definition

Choosing a target segment is not describing everyone who might buy. It is naming the specific customer whose problem the product is built to solve first, and being honest about who is secondary or out of scope. A product built for everyone tends to serve no one well, because the needs, economics, buying behavior, and success criteria of different groups conflict. Segment definition decides who the team optimizes for when tradeoffs appear, and that decision quietly shapes every roadmap, pricing, and messaging choice that follows.

Agents often miss this because segmentation feels obvious. They reach for demographics, firmographics, or job titles because those are easy to label, then build personas that read like character profiles rather than decision tools. The harm is a segment that looks precise on a slide but does not predict behavior, willingness to pay, or which feature matters. Personas become decoration, segments become too broad, and the product drifts toward the average of unrelated needs.

Use this skill before answering broad questions such as "who is our target customer", "which segment should we serve first", "how many personas do we need", "is this segment big enough", or "should we expand to a new audience". The goal is to prevent the agent from producing personas and segments that are demographically neat but behaviorally meaningless, too numerous to guide decisions, or too broad to focus the team.

## Core Rules

### Segment By Differences That Change The Product

A useful segment is a group that differs from other groups in a way that changes what you should build, how you should price, how you should sell, or how you should support them. If two groups would receive the same product, the same message, and the same pricing, they are not separate segments for product purposes even if their demographics differ.

Segment on variables that actually drive behavior and economics:

- the job they are hiring the product to do;
- the workflow or context the product enters;
- the pain severity and frequency;
- willingness to pay and budget authority;
- buying process and decision criteria;
- cost to serve and support burden;
- switching cost from alternatives;
- success metrics they care about.

Demographics and firmographics can be useful proxies for reachability, but they rarely explain why a product fits. Prefer behavioral and economic segmentation, then layer demographics on top for targeting and channels.

### Distinguish Primary, Secondary, And Out-Of-Scope

A focused target definition names one primary segment the product is optimized for, a small number of secondary segments it can serve acceptably, and an explicit out-of-scope set. The primary segment is the customer whose tradeoffs win when features conflict. Without that hierarchy, every stakeholder can claim their customer is the target and the product becomes a compromise.

For the primary segment, be able to answer:

- Why this group first, in terms of pain, willingness to pay, and reachability?
- What do we optimize for them even at the cost of others?
- What evidence confirms they exist and have the problem?

For secondary segments, name what they get for free because it serves the primary, and what they do not get. For out-of-scope, name who you are choosing not to serve and why, so the team can say no to requests that pull focus.

### Define The Ideal Customer Profile For B2B

In business-to-business products, the ideal customer profile (ICP) is the account-level description of the best-fit customer, not a single buyer persona. It captures firmographic, technographic, and situational attributes that correlate with value realization, expansion, and retention. A strong ICP lets sales, marketing, and product align on which accounts deserve the most investment.

A useful ICP includes:

- industry and sub-industry;
- company size, structure, and geography;
- relevant technology stack or dependencies;
- organizational trigger or timing event;
- buying committee composition;
- current alternative and switching cost;
- expansion potential and lifetime economics.

Favor attributes that predict success and churn, validated against won, lost, and churned accounts, rather than attributes that merely describe your current customers. A segment defined only by who already buys can encode luck and survivorship bias.

### Treat Personas As Decision Tools, Not Decoration

A persona is useful only if it changes a decision. The point is not a name, a photo, and a quote. The point is a compact model of a real user's job, pains, gains, and context that the team consults when designing, prioritizing, and messaging. If removing the persona would not change any roadmap or copy choice, it is not earning its place.

A working persona captures:

- the job they are trying to get done and the outcome they measure;
- their pains, anxieties, and failure modes today;
- their gains, desired outcomes, and what success looks like;
- the context and constraints of their environment;
- their decision criteria and information sources;
- a representative quote drawn from real research.

Anchor every field in evidence from interviews, support tickets, sales calls, or behavioral data. A persona invented from internal assumption is worse than none, because it lends false confidence to guesses.

### Limit The Number Of Personas

Most products need one to three primary personas, not eight. Each additional persona multiplies the tradeoffs the team must reconcile and dilutes focus. When everything is a persona, nothing is a target. A long persona deck usually signals that segmentation has not actually happened.

If you find yourself with many personas, ask:

- Which differences actually change the product?
- Can several be merged into one persona with variants?
- Which persona is primary, and which are secondary?
- Are some of these actually user roles within one buyer journey rather than distinct segments?

Merge personas that share the same job, pains, and economics. Split personas only when the differences force different product, pricing, or messaging decisions.

### Size The Segment Honestly

A segment must be large enough to matter and reachable enough to acquire. Enthusiasm for a group does not make it viable. Estimate size and reachability with the same rigor you would apply to a financial commitment, because the target decision commits engineering, sales, and marketing capacity.

Assess:

- total addressable population for the segment;
- the portion with the actual problem and willingness to pay;
- the portion reachable through feasible channels;
- the cost to acquire a customer in this segment;
- the cost to serve and support them;
- the expected lifetime value and payback period;
- concentration risk if a few customers dominate.

Express size as a range with stated assumptions, not a single optimistic number. If the segment is viable only under the most favorable assumption, that is a discovery question, not a target decision.

### Match Segment To Willingness To Pay And Cost To Serve

A segment that has the problem but cannot or will not pay enough to cover the cost to serve is not a viable target at the current model. Willingness to pay and cost to serve are segment properties, not product properties, and they often vary more across segments than within them. Two segments can share a problem yet have wildly different economics.

Before committing, estimate for the segment:

- price they will pay relative to the value delivered;
- sales motion required and its cost;
- onboarding, support, and success burden;
- churn and contraction risk;
- expansion and referral potential.

If the economics do not work for the primary segment, either change the segment, change the model, or change the product. Do not paper over the gap with optimistic projections.

### Decide When To Narrow Or Expand

Segment focus is not permanent. The right target can narrow as you learn who actually values the product, or expand once a beachhead is won and the model is proven. The decision to narrow or expand should be driven by evidence about fit, economics, and reachability, not by pressure to show growth or by fear of missing a market.

Narrow when:

- you cannot serve the stated segment well with one product;
- one sub-segment shows clearly better retention and economics;
- sales and marketing are spread too thin to win any group;
- feedback reveals that different segments need conflicting features.

Expand when:

- the primary segment is saturated or the model is proven;
- adjacent segments share the same job and economics;
- a new capability genuinely serves the new group without diluting the core.

Make the change explicit and update the primary, secondary, and out-of-scope definitions so the whole company realigns.

## Common Traps

### Segmenting Only By Demographics

Age, role, industry, or company size are easy to collect but rarely explain why a product fits. Two people with the same title can have opposite needs. Segments built on demographics alone produce messaging and features that feel generic because they are not anchored in the job or pain. Use demographics for targeting, not for definition.

### Personas As Marketing Wallpaper

A persona with a name, photo, and invented hobbies does not guide decisions and often never gets consulted after the kickoff. It consumes effort and creates the illusion of customer understanding. The trap is that the team feels they know the customer while still designing from internal assumptions. A persona must be tied to jobs, pains, and evidence, and it must be referenced when tradeoffs arise.

### Defining Too Many Personas

When a team lists six or eight personas, focus disappears and every feature request can claim a persona as justification. The trap is mistaking coverage for clarity. More personas rarely means more insight; it usually means segmentation has not been completed. The result is a product that tries to please everyone and satisfies no group deeply.

### Treating Current Customers As The Segment

Defining the target as "who buys from us today" encodes whatever accidents produced those sales. You may be serving a segment that churns, that is expensive to support, or that is too small to sustain growth. The trap is survivorship bias presented as strategy. Validate the segment against fit, economics, and potential, not just against the existing customer list.

### Ignoring Willingness To Pay And Cost To Serve

A segment can have a real, painful problem and still be a bad target if it cannot pay enough to cover acquisition and support costs. The trap is falling in love with the pain while ignoring the economics. Products built for sympathetic but unprofitable segments drain capacity and never reach a sustainable model.

### Over-Broad "Everyone" Segments

Saying the product is for "anyone who needs X" feels safe because it excludes no one, but it gives the team no basis for tradeoffs. The trap is that broad statements avoid disagreement while quietly ensuring the product has no edge. A real target definition must be willing to exclude someone.

### Expanding Too Early

Expanding to a new segment before the primary beachhead is won splits focus and often loses both markets. The trap is chasing apparent growth before the core model is proven, which leaves the original segment under-served and the new segment under-resourced. Expand from strength, not from restlessness.

## Self-Check

- [ ] The segment is defined by differences in job, behavior, pain, or economics, not by demographics alone.
- [ ] A primary segment is named, with secondary and out-of-scope groups explicitly distinguished.
- [ ] For B2B, an ideal customer profile is defined with attributes that predict success and churn, validated against won, lost, and churned accounts.
- [ ] Each persona captures job, pains, gains, context, and decision criteria, anchored in real research rather than invention.
- [ ] The number of personas is small enough to guide decisions, and merged variants are justified rather than multiplied.
- [ ] Segment size is expressed as a range with stated assumptions for addressable population, reachable portion, and concentration risk.
- [ ] Willingness to pay, acquisition cost, and cost to serve are estimated for the segment and the economics are shown to work.
- [ ] Criteria for when to narrow or expand the target are stated, tied to evidence about fit, economics, and reachability.
- [ ] Out-of-scope customers are named so the team can decline requests that pull focus from the primary segment.
- [ ] No persona or segment definition relies on a single optimistic number or on the existing customer list as the sole source of truth.
