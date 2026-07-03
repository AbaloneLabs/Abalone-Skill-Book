---
name: lifecycle-intervention-targeting.md
description: Use when the agent is targeting interventions at specific lifecycle stages, deciding what message or nudge to send to which users when, designing stage-appropriate communications, avoiding over-messaging and notification fatigue, or measuring whether lifecycle interventions actually move users forward without causing backlash.
---

# Lifecycle Intervention Targeting

Lifecycle interventions — the messages, nudges, feature exposures, and offers directed at users based on where they are in their journey — are where the lifecycle model meets the user. Done well, they deliver the right help at the right moment and move users forward: a struggling new user gets the guidance they need; an active user learns about a capability they are ready for; a declining user is re-engaged before they are lost. Done poorly, they produce the experience users complain about most: too many messages, irrelevant pushes, notifications that feel surveillant, and the gradual training of users to ignore or disable everything. The line between helpful and harmful is thin, and the product manager who targets interventions carefully will move users forward; the one who targets broadly will alienate them.

This skill covers the judgment needed to target lifecycle interventions: who to reach, with what, when, and how to know whether the intervention helps or harms.

## Core Rules

### Target interventions to the user's actual state and need, not to a campaign schedule

The most common intervention failure is targeting by campaign calendar ("send the week-three email to all new users") rather than by the user's actual state and need. Calendar-based targeting reaches users at moments determined by the company's convenience, not by the user's situation, and so it is frequently irrelevant — reaching users who do not need the message, while missing those who do. State-based targeting reaches users when the intervention is actually useful.

- Trigger interventions by user behavior and state: an action taken (or not taken), a milestone reached (or missed), a pattern that indicates a need. The trigger should be the event that makes the intervention relevant.
- Make the intervention's content match the trigger. A user who just hit a ceiling of their current usage needs to learn about the next capability; a user who has not returned needs a reason to come back, not a feature announcement.
- Avoid blanket sends to broad segments defined only by time or stage. The relevance is in the specificity of the trigger and the match of the content.

The test of targeting: would this specific user, in this specific state, find this specific message helpful right now? If the answer is "not necessarily," the targeting is too broad.

### Respect the user's attention as a finite, depletable resource

Every intervention spends a finite budget of user attention, and the budget is smaller and more fragile than teams assume. Each message, notification, and nudge makes the next one less effective, and at some point the cumulative volume trains the user to ignore or disable all of them — including the ones that would have genuinely helped. Treating attention as free produces intervention programs that degrade their own channel.

- Budget interventions across the lifecycle, not per campaign. If onboarding, feature adoption, re-engagement, and monetization each send independently, the user receives the sum, which is usually too much.
- Prioritize ruthlessly within the budget. Not every intervention that could help is worth the attention it costs; choose the few with the highest net value.
- Monitor the health of the channel: notification opt-out rates, open rates, dismiss rates. Declining channel health is a signal that the volume exceeds what users will bear.

A smaller number of well-targeted interventions outperforms a larger volume of loosely targeted ones, both in effect and in channel health.

### Measure the net effect of each intervention, including its cost to the relationship

Interventions are typically measured on the immediate metric they target (click-through, conversion, the action they promote) and this is where the most dangerous mis-optimization occurs. An intervention that moves the immediate metric but damages the broader relationship — through annoyance, surveillance feeling, or trust erosion — is a net loss that the immediate metric hides.

- Measure the effect on downstream retention and lifetime value, not just on the immediate response. An intervention that drives a click but harms retention is destructive.
- Measure the effect on the channel and on overall notification perception. An intervention that works for its target but trains broader dismissal is a tax on every future intervention.
- Measure the effect on users who received the intervention but did not respond. The non-responders are the majority, and their experience of the intervention (helpful, ignorable, or annoying) determines whether the channel stays healthy.

The net effect is the gain in the target behavior minus the cost in attention, trust, and channel health. Interventions whose net effect is negative should be stopped, even if their immediate metric looks good.

### Match the channel and tone to the intervention and the relationship

The channel (email, push, in-app, SMS) and the tone of an intervention shape how it is received as much as its content. A message that is helpful in one channel is intrusive in another, and a tone that fits an engaged user alienates a new one.

- Match channel to urgency and intimacy. Push and SMS are high-urgency, high-intrusion; use them sparingly for things that genuinely warrant interrupting the user. Email is lower-urgency and tolerates more volume. In-app is contextual and non-interruptive, ideal for guidance tied to what the user is doing.
- Match tone to the relationship stage. A new user needs helpful, low-pressure guidance; an engaged user can be introduced to new capabilities; a declining user needs a compelling reason to return, not a generic nudge.
- Consider the cumulative experience across channels. A user receiving email, push, and in-app messages from different teams experiences them as one relationship with the product, and incoherence across channels reads as disorganization.

### Personalize based on behavior, but stay on the right side of the surveillance line

Behavioral personalization makes interventions more relevant and more effective, but it also risks crossing into feeling surveilled, where the user perceives that the product knows too much and is using it to manipulate. The line is real and user-perception-dependent, and crossing it damages trust disproportionately.

- Personalize based on behavior the user understands the product can see (actions they took in the product), and avoid personalization based on data whose source is opaque or surprising to the user.
- Make personalization feel helpful, not uncanny. "Since you used X, you might find Y useful" feels helpful; "we noticed you haven't logged in for 11 days" feels surveilled.
- When in doubt, err toward less personalization and more transparency. A slightly less targeted intervention that does not feel creepy beats a highly targeted one that does.

### Time interventions to the moment of maximum relevance and minimum disruption

When an intervention arrives matters as much as what it says. An intervention at the moment of maximum relevance (right after the triggering behavior, at the point of need) is welcomed; the same intervention at a random or disruptive moment is ignored or resented.

- Trigger in-product interventions contextually, at the moment the user is in a position to act on them.
- Time out-of-product interventions (email, push) to when the user is likely to be receptive and able to act, not to when the company's send job runs.
- Avoid interrupting the user during flow or during moments of frustration. An intervention that interrupts a struggling user compounds the frustration; one that waits for a natural pause is received far better.

### Design for the users who do not respond, who are the majority

Most users who receive an intervention will not respond to it, by design or by circumstance. The intervention's effect on these non-responders — whether it was a harmless ignorable nudge or an annoying interruption — determines whether the channel stays healthy for future use. Design for the non-responders as deliberately as for the responders.

- Ensure that an unresponded intervention is easy to dismiss and does not recur aggressively. A user who dismissed a nudge once should not be nagged repeatedly.
- Ensure that the intervention, even unresponded, does not convey something false or damaging (that the user is behind, that they are missing out, that the product is desperate).
- Treat the non-responder experience as a first-class design concern, because non-responders are the majority and their perception shapes the channel's future effectiveness.

## Common Traps

### Calendar-based targeting instead of state-based

Sending by schedule rather than by user state reaches users at irrelevant moments and misses those who need the intervention. Target by behavior and state.

### Attention treated as free

Each team sends independently, and the user receives an unsustainable volume that trains dismissal of all interventions. Budget attention across the lifecycle and prioritize within it.

### Measuring only the immediate metric

An intervention that moves click-through or conversion but harms retention and channel health is a net loss hidden by the immediate metric. Measure net effect including downstream retention and channel health.

### Channel and tone mismatched to the intervention and relationship

A helpful message in the wrong channel or tone is intrusive. Match channel to urgency and intimacy, and tone to relationship stage.

### Personalization that crosses into surveillance

Highly targeted interventions based on data whose source surprises the user feel creepy and damage trust. Personalize on data the user understands, and err toward transparency.

### Interruption at moments of frustration or flow

An intervention that interrupts a struggling user compounds the frustration. Time interventions to moments of relevance and receptivity.

### Designing only for responders

The non-responders are the majority, and their experience determines channel health. Design the non-responder experience as deliberately as the responder experience.

### Aggressive recurrence to non-responders

Nagging users who dismissed an intervention trains them to dismiss all future interventions. Respect dismissal and avoid aggressive recurrence.

## Self-Check

- Is each intervention triggered by the user's actual state and need, or by a campaign schedule convenient to the company?
- Am I budgeting user attention across the lifecycle, prioritizing the highest-net-value interventions, rather than each team sending independently?
- For each intervention, am I measuring the net effect on downstream retention and channel health, not just the immediate response metric?
- Are the channel and tone matched to the intervention's urgency and the user's relationship stage?
- Does my personalization feel helpful and transparent, or does any of it risk feeling surveilled?
- Is each intervention timed to a moment of relevance and receptivity, avoiding interruption during frustration or flow?
- Have I designed the experience for non-responders, who are the majority, ensuring the intervention is harmlessly dismissible and does not recur aggressively?
- Am I monitoring channel health (opt-outs, open rates, dismissals) and reducing volume when health declines?
- Across all teams, would the user experience the cumulative interventions as a coherent, helpful relationship or as a barrage of unrelated pushes?
- If I received my own product's full set of lifecycle interventions as a user, would I find them helpful or would I look for the mute button?
