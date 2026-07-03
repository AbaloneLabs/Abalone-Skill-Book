---
name: churn-prediction-and-signals.md
description: Use when the agent is building or evaluating churn prediction, identifying leading indicators of churn risk, distinguishing predictive signals from noise, designing churn risk scoring, deciding which churn is preventable versus inevitable, or interpreting churn data without being misled by lagging metrics and confounders.
---

# Churn Prediction And Signals

Churn is the outcome every product team wants to prevent, and almost always detects too late. By the time a user cancels or goes dormant, the decision was made days or weeks earlier, and the moment to intervene has passed. The value of churn prediction is to detect the leading signals that precede the decision, creating a window in which intervention might still work. But prediction is hard, and the failure modes are severe: false signals waste intervention effort and annoy users, lagging metrics create the illusion of prediction while reporting the past, and confounders mislead about which users are actually at risk. The product manager who understands these failure modes can build prediction that genuinely creates an intervention window; the one who does not builds a churn dashboard that looks predictive and isn't.

This skill covers the judgment needed to build, evaluate, and act on churn prediction: what signals matter, what signals mislead, and how to distinguish preventable churn from the inevitable.

## Core Rules

### Distinguish leading signals from lagging metrics, and prioritize the leading ones

The foundational error in churn work is treating lagging metrics as if they were predictive. A drop in login frequency, a decline in usage, a cancellation request — these are lagging. They indicate that churn is underway or has already happened, not that it is approaching. Acting on lagging metrics means intervening after the decision is made, when intervention rarely works.

- Identify leading signals: changes in behavior that precede the churn decision. These often appear as shifts in the pattern of engagement (breadth narrowing, depth decreasing, key actions dropping) before overall usage declines.
- Leading signals are harder to find and less obvious than lagging ones, which is why they are underused. They require analysis of what changed before users churned, not just what was true when they churned.
- Prioritize intervention triggered by leading signals, where the window is still open, over intervention triggered by lagging metrics, where the window has closed.

The predictive value is in the leading signals. A prediction system built on lagging metrics reports the past, not the future.

### Validate that signals actually predict churn, with attention to base rates and confounders

A signal that "churned users tended to show" is not necessarily predictive. The validation must account for base rates and confounders, or the signal will mislead.

- Check the false positive rate: how many users who showed the signal did not churn? A signal that appears in many users who do not churn is low-value for targeting, because most interventions will be wasted on users who would have stayed anyway.
- Check the base rate of churn. In products with low churn, even a signal that strongly correlates with churn may identify mostly false positives, because the non-churners vastly outnumber the churners.
- Investigate confounders. A signal may correlate with churn because both are caused by a third factor (a segment characteristic, a cohort effect, a seasonality), and the signal is not itself actionable. The classic example: a signal that identifies a specific segment as high-churn may reflect that the segment was always higher-churn, not that something changed to put them at risk.

Validate prediction against held-out data and over time, not just on the dataset it was built on, which will always look more predictive than it is.

### Separate preventable churn from inevitable churn, and focus prediction on the preventable

Not all churn is preventable, and a prediction system that flags all churn risk wastes effort on users who will leave regardless. Some churn is structural: the user's need ended, their circumstances changed, they were never a good fit, or they left for a reason no product change would address. Predicting this churn accurately is useless, because there is no intervention that would change the outcome.

- Distinguish the reasons for churn: fit-based (wrong product for the user), need-based (need ended), value-based (did not get enough value), experience-based (friction or problems), and competitive (left for an alternative).
- Focus prediction and intervention on the churn that is preventable: primarily value-based and experience-based, where an intervention could change the outcome.
- Accept that fit-based and need-based churn are largely inevitable, and that spending intervention effort on them wastes resources that would be better spent on the preventable cases.

The highest-value prediction identifies users who would have stayed if the right intervention reached them, not all users who will leave.

### Segment churn risk and prediction by the dimensions that matter

Aggregate churn prediction hides the variation that determines what intervention would work. A user at risk because they never activated needs a different intervention than a user at risk because of a recent regression, and a single risk score cannot distinguish them.

- Segment churn risk by cause (what is driving the risk), by segment (who the user is), and by stage (where they are in the lifecycle).
- Tailor the prediction to identify not just that a user is at risk, but why, so the intervention can match the cause.
- Avoid a single monolithic churn score that flags risk without informing action. A risk score that does not tell you what to do is half a tool.

### Treat prediction as a hypothesis to be tested through intervention, not as truth

Churn prediction produces probabilities, not certainties, and the only way to know whether a signal is actionable is to test whether intervening on it changes the outcome. A signal that predicts churn but on which intervention has no effect is not actionable, however predictive it is.

- Test interventions on predicted-at-risk users with controlled comparison, so the effect of the intervention can be distinguished from the baseline churn rate of that group.
- A signal is actionable only if intervention on users flagged by that signal reduces churn relative to not intervening. Predictive without actionable is interesting but not useful.
- Refine the prediction based on the intervention results. If an intervention works for some flagged users but not others, the signal is over-broad and can be refined.

### Watch for the prediction system's effect on the user experience

A churn prediction system that drives interventions has a footprint in the user experience: the interventions it triggers are felt by users, and the targeting itself can feel surveillant if users sense that their behavior is being monitored for risk. The system's net effect includes both the churn it prevents and the experience it shapes.

- Monitor whether interventions driven by prediction feel helpful or intrusive to the users who receive them.
- Be cautious with prediction that uses data whose collection would surprise users, which can feel like surveillance.
- Balance the precision of targeting against the user experience cost of being targeted. A slightly less precise intervention that feels respectful may net better than a highly precise one that feels creepy.

### Recognize the limits of prediction and the value of root-cause work

Prediction identifies who is at risk; it does not address why users churn in the first place. A team that invests only in prediction and intervention treats the symptoms, catching some at-risk users, while the underlying causes continue to generate new at-risk users. The higher-leverage long-term work is root-cause: understanding and addressing why users churn, so fewer become at-risk.

- Use prediction to create the intervention window for users at risk now.
- Use root-cause analysis of churned users to reduce the flow of new at-risk users over time.
- Balance the two. A team that only predicts and intervenes runs on a treadmill; a team that only does root-cause work loses the users it could have saved in the meantime.

## Common Traps

### Lagging metrics mistaken for leading signals

Acting on drops in usage or cancellations means intervening after the decision is made. Find and act on the leading signals that precede the decision.

### Signals that correlate but do not predict, due to base rates and confounders

A signal that churned users showed may also be shown by many who did not churn, and may reflect a confounding factor rather than actionable risk. Validate with attention to false positives, base rates, and confounders.

### Predicting inevitable churn and wasting intervention effort

Fit-based and need-based churn are largely unpreventable. Focus prediction on the preventable causes (value-based, experience-based) where intervention could change the outcome.

### A single risk score that flags without informing action

A monolithic churn score tells you who is at risk but not what to do. Segment risk by cause and stage so the intervention can match.

### Prediction treated as truth rather than hypothesis

A signal is actionable only if intervention on it changes the outcome. Test interventions with controlled comparison; refine based on results.

### Prediction-driven interventions that feel surveillant

Targeting based on behavior users did not know was monitored, or that feels like surveillance, damages trust. Balance precision against user experience.

### Only predicting and intervening, never addressing root causes

A team that only catches at-risk users runs on a treadmill while underlying causes generate new risk. Balance prediction with root-cause work that reduces the flow of at-risk users.

### Over-fitting prediction to the dataset it was built on

A model that looks predictive on its training data overstates its real-world performance. Validate on held-out data and over time.

## Self-Check

- Am I acting on leading signals that precede the churn decision, or on lagging metrics that report it after the fact?
- Have I validated that my signals predict churn with attention to false positives, base rates, and confounders, on held-out data?
- Am I focusing prediction on preventable churn (value-based, experience-based) rather than inevitable churn (fit-based, need-based)?
- Does my prediction identify why a user is at risk, not just that they are, so the intervention can match the cause?
- Have I tested whether intervening on predicted-at-risk users actually reduces churn, treating prediction as a hypothesis?
- Are my prediction-driven interventions felt by users as helpful, or do any feel surveillant or intrusive?
- Am I balancing prediction and intervention (saving users at risk now) with root-cause work (reducing the flow of new at-risk users)?
- Have I validated my prediction over time and on data it was not built on, rather than trusting in-sample performance?
- For the users my system flags as at-risk, what fraction would have stayed without intervention, and is that acceptable given the cost of intervention?
- If my prediction system were turned off tomorrow, how would I know — what would change in the churn rate, and would that reveal the system's real value?
