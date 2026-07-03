---
name: retention-play-design.md
description: Use when the agent is designing retention plays, building interventions to keep at-risk users, deciding between habit-building and re-engagement approaches, matching the retention play to the churn cause, measuring whether retention efforts work without being misled by survivorship bias, or balancing retention investment against acquisition.
---

# Retention Play Design

Retention plays are the deliberate interventions designed to keep users who are at risk of leaving, and to build the habits that keep users from becoming at risk in the first place. They are where churn prediction meets action, and where most of the effort and most of the waste in retention work occurs. A well-designed retention play, matched to the cause of risk and measured honestly, saves users who would otherwise have left. A poorly designed one — generic, mistimed, measured on the wrong metric — produces activity that looks like retention work while changing nothing, or worse, annoys the users it was meant to save. The product manager who designs retention plays with rigor will retain meaningfully more users; the one who applies generic best practices will not.

This skill covers the judgment needed to design, target, and measure retention plays: matching the play to the cause, building habits versus re-engaging, and measuring honestly.

## Core Rules

### Match the retention play to the specific cause of churn risk

The most common retention failure is applying a generic play ("send a re-engagement email") to all at-risk users regardless of why they are at risk. The cause of risk determines what intervention could work, and a mismatched play cannot succeed no matter how well executed.

- For value-based risk (the user is not getting enough value), the play must address the value gap: guidance to underused capabilities, a clearer path to the core value, or help overcoming an obstacle. A generic nudge to return does not fix a value problem.
- For experience-based risk (the user hit friction or problems), the play must address the experience: resolution of the problem, acknowledgment and recovery, or a smoother path. A promotional offer does not fix an experience problem.
- For competitive risk (the user is considering an alternative), the play must reinforce the unique value and address the comparison: highlighting capabilities the alternative lacks, addressing the specific gap that prompted the comparison.
- For fit-based and need-based risk, acknowledge that retention plays are largely ineffective and avoid wasting effort; these users will leave regardless.

A play that does not match the cause is motion without progress. Diagnose the cause, then design the play.

### Distinguish habit-building from re-engagement, and invest in both

Retention work splits into two fundamentally different categories, and conflating them produces misallocated effort. Habit-building prevents users from becoming at risk; re-engagement tries to recover users who already are. Both matter, but they are different plays with different timing, different success rates, and different leverage.

- **Habit-building** works with users who are active but not yet deeply engaged, establishing the patterns that make the product part of their routine. It is high-leverage because it prevents risk from developing, and its success rate is high because the user is still engaged.
- **Re-engagement** works with users whose engagement has already declined, trying to reverse the trend. It is lower-leverage because the user has often already decided to leave, and its success rate is lower because the window has narrowed.

Invest disproportionately in habit-building, which prevents the risk, while maintaining a re-engagement capability for users who slip. A team that invests only in re-engagement is forever catching users who have already partially left.

### Time the play to the intervention window, not to the company's convenience

The effectiveness of a retention play depends sharply on timing. A play delivered while the user is still engaged and the risk is emerging has a real chance of working; the same play delivered after the user has disengaged has almost no chance. The intervention window opens when the leading signals appear and closes when the user's decision is made.

- Trigger retention plays by leading signals of risk, not by lagging metrics or by calendar. The window is between the signal and the decision.
- Deliver the play at the moment the user can still act on it and is still receptive. A play that arrives after the user has stopped paying attention is wasted.
- Recognize that the window is narrow and that speed of intervention matters. A play triggered immediately by a signal outperforms the same play delivered days later.

### Make the play genuinely valuable to the user, not just beneficial to the company

Retention plays that are transparently self-interested — "we noticed you haven't logged in, please come back so we keep you as a user" — are received as the company's problem, not the user's, and they fail. Plays that deliver genuine value to the user — a capability they will find useful, help with a problem they have, an outcome they want — succeed because they give the user a reason to engage that serves the user.

- Frame the play around what the user gets, not what the company wants. The retention is a byproduct of the user receiving value.
- Avoid plays that are purely promotional or guilt-inducing, which signal that the company cares about the metric, not the user.
- The best retention play is one the user would thank the company for, were they aware it was a retention effort. If the user would find the play manipulative or self-serving, redesign it.

### Measure retention plays on net retention, with attention to survivorship and timing bias

Retention measurement is full of traps that make ineffective plays look effective. The most common is survivorship bias: users who respond to a retention play may be the ones who would have stayed anyway, and the play gets credit for retention it did not cause.

- Measure with a control group: users flagged as at-risk who did not receive the play, compared to those who did. Only the difference is the play's effect.
- Account for timing. A play that "retains" users for an extra week before they churn anyway is not retention; it is delay. Measure retention over a meaningful horizon, not just the immediate response.
- Watch for cannibalization: a play that retains users in one period by offering a discount or concession may increase churn in the next period, when the concession ends or the underlying cause reasserts. Measure the net effect over time, not just the immediate period.

Without honest measurement, retention work becomes a portfolio of plays that each look good on their own metric while aggregate retention does not improve.

### Calibrate retention investment against acquisition and against the value of the user retained

Not every at-risk user is worth the same retention effort, and a retention program that treats all users equally misallocates resources. Calibrate the investment to the value of the user retained and to the cost of acquiring a replacement.

- Weigh the cost of the retention play (including the concession or incentive, if any) against the lifetime value of the user if retained. A play that costs more than the user's remaining value is not worth it, however effective.
- Compare retention cost to acquisition cost. If retaining a user costs more than acquiring a comparable new one, the play may not be economical, however emotionally appealing retention feels.
- Segment the investment: higher-touch, higher-cost plays for higher-value users; scalable, lower-cost plays for the broader base. A one-size-fits-all retention program either under-invests in high-value users or over-invests in low-value ones.

### Design retention plays that do not train users to churn

A subtle failure of retention plays, especially those involving concessions (discounts, free months, feature unlocks), is that they can train users that churning (or threatening to) is rewarded. Users who learn that disengaging produces a better offer have an incentive to disengage periodically, and the retention play becomes a cause of the behavior it was meant to prevent.

- Be cautious with concession-based plays, and avoid making them predictable or automatic. If users learn the pattern, the play creates the churn.
- Prefer value-based plays (help, capability, outcome) over concession-based plays, because value-based plays do not create the perverse incentive.
- When concessions are necessary, make them genuine one-time recovery efforts, not a recurring pattern the user can game.

## Common Traps

### Generic plays applied regardless of churn cause

A re-engagement email sent to all at-risk users cannot address a value gap or an experience problem. Match the play to the specific cause of risk.

### Conflating habit-building with re-engagement

Investing only in re-engagement (catching users who have partially left) while neglecting habit-building (preventing risk) is lower-leverage and runs on a treadmill. Invest in both, disproportionately in habit-building.

### Plays timed to the company's schedule, not the intervention window

A play delivered after the user has disengaged has little chance. Trigger by leading signals and deliver within the window before the decision is made.

### Self-interested plays that the user experiences as the company's problem

Plays framed around what the company wants ("please come back") fail; plays that deliver value the user wants succeed. Frame around user value.

### Survivorship bias making ineffective plays look effective

Users who respond may be those who would have stayed anyway. Measure with a control group and over a meaningful horizon, accounting for delay and cannibalization.

### Retention at any cost, ignoring economics

A play that costs more than the user's remaining value, or more than acquiring a replacement, is not economical. Calibrate investment to user value and acquisition cost.

### Concession-based plays that train users to churn

Predictable concessions teach users that disengaging is rewarded, creating the churn the play was meant to prevent. Prefer value-based plays and make concessions genuine one-time efforts.

### Measuring immediate response rather than net retention

A play that drives a click or a short-term return but does not change long-term retention is not working. Measure net retention over a meaningful horizon.

## Self-Check

- Does each retention play match the specific cause of churn risk it is meant to address, or am I applying generic plays regardless of cause?
- Am I investing in both habit-building (preventing risk) and re-engagement (recovering risk), with disproportionate investment in the higher-leverage habit-building?
- Are my plays triggered by leading signals and delivered within the intervention window, or timed to the company's convenience?
- Would the user experience this play as genuinely valuable to them, or as the company's self-interest dressed up?
- Am I measuring retention with a control group, over a meaningful horizon, accounting for delay and cannibalization, rather than trusting immediate response metrics?
- Is the cost of each play calibrated against the lifetime value of the user retained and the cost of acquiring a replacement?
- Am I avoiding concession-based plays that could train users to churn, or making concessions genuinely one-time?
- Am I segmenting retention investment by user value, with higher-touch plays for higher-value users and scalable plays for the broader base?
- Across my portfolio of retention plays, is aggregate retention actually improving, or do the plays each look good while the aggregate does not move?
- If a user received one of my retention plays, would they feel helped by the company or manipulated by it?
