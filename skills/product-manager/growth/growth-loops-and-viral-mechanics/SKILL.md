---
name: growth_loops_and_viral_mechanics.md
description: Use when the agent is designing growth loops, building viral or referral mechanics, engineering sustainable acquisition channels, modeling network effects, or deciding between paid and organic growth strategies for a product.
---

# Growth Loops And Viral Mechanics

Sustainable growth is not the result of spending more on acquisition; it is the result of building engines where the product's own users create the next users. A growth loop is a self-reinforcing cycle where an output of the product, content, invitations, shared work, network value, becomes the input to new acquisition, and each new user compounds the loop rather than merely adding to a total. The judgment problem is that viral mechanics and growth loops are easy to describe and hard to engineer honestly: most products are not naturally viral, forcing virality onto a product that does not warrant it produces manipulative friction that damages retention, and a loop that works on paper often fails because the conversion steps were never measured. The skill is distinguishing where a genuine loop exists from where one is being willed into existence, and engineering the loop with honest math rather than hopeful metaphors.

Use this skill before designing referral or viral features, before modeling a growth loop, before deciding between paid and organic growth, or before claiming a product has network effects. The goal is to prevent the agent from building manipulative virality that harms retention, from claiming a loop exists without the math to support it, or from defaulting to paid acquisition when an organic loop is viable.

## Core Rules

### Distinguish Loops From Funnels

A funnel is a linear path from acquisition to conversion, where each stage leaks and the only way to get more output is to pour more input at the top. A loop is a cycle where the output feeds the input, so growth compounds rather than merely adds. Most products are built as funnels and mistakenly described as loops.

Distinguish:

- funnel: spend to acquire, convert, retain; growth requires more spend;
- loop: users create something that acquires new users; growth compounds;
- hybrid: paid acquisition seeds a loop that then compounds.

If removing paid acquisition stops growth entirely, you have a funnel, not a loop. Naming this honestly is the first step.

### Identify The Genuine Loop Type For The Product

Loops take different forms, and the right one depends on the product's natural behavior. Forcing the wrong loop type produces friction and churn.

Common loop types:

- viral referral: users invite others for a benefit, works when the product is better with more users;
- content: users create content that attracts new users through search or social, UGC and SEO;
- embedded: the product is shared through use, invoices, documents, collaborations that expose new users;
- network: each user increases value for all, marketplaces and communication tools;
- word-of-mouth: satisfaction drives organic recommendation, no engineered mechanic.

A loop must fit the product's natural value. A referral loop in a single-player utility adds friction without compounding.

### Model The Loop With Honest Math

A loop is only real if its math compounds. Model each step with realistic, not optimistic, conversion rates to determine whether the loop grows, plateaus, or decays.

Model:

- the number of new users each existing user generates, the viral coefficient k;
- the conversion at each step: invitation sent, opened, signed up, activated;
- the time it takes for one loop cycle to complete;
- the retention that determines whether acquired users stay long enough to feed the loop;
- the saturation point where the loop slows as the reachable population is consumed.

If k is below 1 and paid acquisition is not seeding the loop, the loop decays. Honest modeling reveals this before build.

### Ensure The Loop Enhances Rather Than Damages The Core Experience

Forced virality, mandatory invitations, gated sharing, social prompts that interrupt the user's job, trades short-term acquisition for long-term retention. A loop that harms the core experience produces users who arrive and churn, defeating the compounding.

Evaluate:

- does the loop arise from genuine product value or from manufactured friction;
- does sharing serve the sharer's goal or only the company's acquisition;
- does the loop add steps to the core workflow or remove them;
- would users share organically without the mechanic.

The strongest loops are those the user initiates because it serves them, not because the product demands it.

### Connect Acquisition To Activation And Retention

A loop that acquires users who never activate or retain is a leak, not growth. The loop's value depends on the full lifecycle, not just the invitation.

Ensure:

- acquired users reach the activation moment that delivers value;
- retention is high enough that users remain to feed the loop;
- the acquired users are the right segment, not just any users;
- the loop does not acquire low-intent users who churn and poison metrics;
- the unit economics of loop-acquired users are positive.

A viral coefficient built on churn-prone users is a treadmill, not compounding growth.

### Measure Each Step Of The Loop, Not Just The Outcome

A loop that is measured only by total growth cannot be diagnosed or improved. Each step must be instrumented so the team can find where the loop leaks.

Measure:

- how many users initiate the loop action;
- how many invitations, shares, or exposures result;
- the open and click rates on shared artifacts;
- the signup and activation conversion of loop-acquired users;
- the retention and value of loop-acquired versus other users.

Step-level measurement turns the loop from a black box into an optimizable system.

### Recognize When Paid Acquisition Is The Honest Answer

Not every product has a viable organic loop, and pretending otherwise wastes effort. For many products, especially single-player utilities and enterprise tools, paid acquisition with strong retention and expansion is the honest and effective strategy.

Choose paid when:

- the product's value is not inherently social or shareable;
- the target segment is narrow and reachable through paid channels;
- the unit economics support profitable paid acquisition;
- retention and expansion make the lifetime value exceed CAC;
- an organic loop has been tested and does not compound.

### Watch For Saturation And Diminishing Returns

Loops slow as they saturate the reachable population. A loop that compounds early decelerates as the easy acquisitions are exhausted. Planning for perpetual compounding ignores this reality.

Plan for:

- the saturation point where loop-driven acquisition slows;
- the shift from compounding to maintenance as the market matures;
- the need to open new loops or segments as the primary loop saturates;
- the diminishing returns on optimizing each loop step.

## Common Traps

### Calling A Funnel A Loop

Describing paid acquisition plus retention as a loop when removing spend stops growth.

### Forcing Virality Onto Non-Viral Products

Engineering mandatory sharing into products whose value is not social, damaging retention.

### Optimistic Loop Math

Modeling the viral coefficient with hopeful conversion rates that do not survive contact with reality.

### Loop Without Retention

Acquiring users through the loop who never activate or stay, producing a leak rather than growth.

### Black-Box Measurement

Tracking only total growth without instrumenting each loop step, making leaks invisible.

### Manipulative Mechanics

Loops that serve acquisition at the expense of the user's experience and trust.

### Perpetual Compounding Assumptions

Planning for unending exponential growth that ignores saturation and diminishing returns.

### Rejecting Paid Acquisition Dogmatically

Avoiding paid channels on principle when honest unit economics make them the right strategy.

## Self-Check

- [ ] The growth engine is honestly classified as a loop, funnel, or hybrid, not mislabeled.
- [ ] The loop type matches the product's natural value rather than being forced onto it.
- [ ] The loop is modeled with realistic conversion rates, viral coefficient, cycle time, retention, and saturation.
- [ ] The loop enhances the core experience rather than adding friction that damages retention.
- [ ] Loop-acquired users activate and retain at rates that make the loop compound rather than leak.
- [ ] Each step of the loop is instrumented so leaks can be found and optimized.
- [ ] Paid acquisition is chosen honestly where no viable organic loop exists, with positive unit economics.
- [ ] Saturation and diminishing returns are planned for as the loop matures.
- [ ] No manipulative mechanic is used that trades user trust for short-term acquisition.
- [ ] The loop is measured by full lifecycle value, not just acquisition count.
