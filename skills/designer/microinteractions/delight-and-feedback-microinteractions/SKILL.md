---
name: delight_and_feedback_microinteractions.md
description: Use when the agent is designing small-scale feedback and delight moments such as button press responses, toggle animations, like and favorite reactions, hover hints, selection confirmations, celebratory confirmations, and the moment-to-moment responses that tell users their input was received.
---

# Delight And Feedback Microinteractions

Microinteractions are the small, contained moments where the interface responds to a single action: a button depressing, a toggle flipping, a heart filling, a checkmark drawing, a badge incrementing. They are easy to dismiss as trivial because each one is tiny, but together they form the texture of the product and the user's sense that the interface is alive and trustworthy. The judgment problem is that delight and feedback are different goals that are constantly confused: feedback exists so the user knows something happened, while delight exists to make a moment feel good. Over-investing in delight produces a giddy, distracting interface; under-investing in feedback produces a dead, untrustworthy one.

Use this skill before designing the response to taps, clicks, toggles, selections, likes, favorites, submissions, completions, and any contained moment of user input. The goal is to prevent the agent from either skipping feedback entirely or burying functional moments under unnecessary spectacle.

## Core Rules

### Separate Feedback From Delight And Prioritize Feedback

Feedback is the foundation: the user must know their input was received and what resulted. Delight is layered on top of feedback and is optional. A button that gives no press response feels broken regardless of how charming its idle state is. A button that animates beautifully but does not confirm the action actually occurred has failed its primary job.

Design the feedback first: did the system register the input, is work happening, did it succeed, did it fail? Only then consider whether a touch of delight improves the moment. If resources are limited, feedback always wins.

### Match Response Intensity To Action Significance

Not every action deserves a celebration. A routine toggle does not need confetti. A like does not need a full-screen burst. Matching the energy of the microinteraction to the significance of the action keeps the interface calm and makes the genuinely important moments stand out.

A useful scale:

- routine input, such as a tap or hover, gets a fast, subtle response that confirms receipt;
- a state change, such as a toggle or selection, gets a clear but contained transition;
- a completion, such as a task finished or a goal reached, can earn a slightly richer moment;
- a rare milestone, such as an achievement or an empty inbox, can justify celebration.

When every action is celebrated, nothing feels special, and the interface becomes exhausting. Reserve delight for moments the user actually values.

### Make Feedback Immediate And Direct

The response to a direct action must begin almost instantly, within roughly 100 milliseconds, to feel connected to the user's input. Any perceptible delay breaks the sense that the user caused the response. This immediacy matters more than the total duration of the animation: the start must be instant, even if the settle takes longer.

Direct manipulation responses, such as a button scaling down on press or a card lifting on grab, should track the user's input in real time. Lagging feedback makes the interface feel detached and lowers the user's confidence that their actions register.

### Connect The Feedback To The Action Visually

Feedback should appear where the user's attention already is: on or near the element they acted on. A like that fills the heart the user tapped confirms the action. A confirmation that appears far away, or a toast that slides in from a distant corner, can be missed and can feel disconnected from the cause.

When feedback must appear elsewhere, such as a count updating elsewhere on the screen, connect it visually to the source, through motion, color, or position, so the user understands the relationship. Disconnected feedback forces the user to puzzle out cause and effect.

### Use Delight To Reward Genuine Accomplishment

Delight works best when it rewards something the user actually achieved or values. Completing a task, clearing a list, reaching a streak, finishing onboarding, or sending an important message are moments where a small celebratory touch feels earned and human. The delight should feel like recognition, not noise.

Avoid delight that rewards trivial or automatic actions, because it trains the user to ignore the interface's signals. A confetti burst for opening a menu devalues the confetti burst for finishing a project.

### Keep Delight Brief And Non-Blocking

Delightful moments should be short and should never make the user wait. A microinteraction that delays the next action, forces the user to watch an animation, or blocks input for more than a moment stops being delightful and becomes an obstacle. Delight that the user cannot skip, especially on repeated encounters, turns into annoyance.

Keep celebratory moments to a second or two, allow the user to continue interacting during or immediately after, and ensure the delight does not interrupt the flow of work. Delight is garnish, not the meal.

### Ensure Delight Does Not Undermine Clarity

A microinteraction that is charming but unclear is a failure. If a toggle animates playfully but the user cannot tell whether it is on or off, the delight has damaged the function. If a success animation is so elaborate that it obscures the result, it has defeated its purpose.

Test each microinteraction for legibility: after the animation, is the resulting state unambiguous? If not, simplify the motion or strengthen the resting state. Clarity must survive the delight.

### Respect Frequency And Repetition

A microinteraction encountered once may delight; encountered a hundred times a day, it may irritate. Consider how often the user will trigger the moment. High-frequency actions need fast, unobtrusive responses that do not draw attention to themselves. Low-frequency, high-value moments can afford more character.

Design for the hundredth encounter, not just the first. A delight that becomes tiresome on repetition is a design error, not a success.

### Honor Reduced-Motion And Accessibility

Even small motion must respect the reduced-motion preference and remain perceivable for users who cannot rely on motion. A toggle that communicates state only through a flip animation fails users who reduce motion; it must also show state through position, color, or label. A press response that relies on a scale change must have a non-motion alternative.

Feedback conveyed by motion should always have a redundant cue, so the microinteraction remains clear when motion is reduced or removed.

## Common Traps

### Skipping Feedback For The Sake Of Minimalism

A clean, understated interface that gives no response to input feels broken and untrustworthy, no matter how elegant it looks at rest.

### Celebrating Routine Actions

Confetti, bursts, and elaborate animations on frequent, trivial actions train users to ignore signals and make the interface feel frantic.

### Delayed Or Disconnected Feedback

Responses that begin too late or appear far from the action break the user's sense of cause and effect.

### Delight That Obscures Result

Animations so elaborate that the user cannot tell the resulting state trade function for charm and fail at both.

### Blocking The User With Delight

Microinteractions that force the user to wait or watch, especially on repeat, turn delight into friction.

### Motion-Only Feedback

Responses that communicate state only through movement fail users who reduce motion or cannot perceive the change.

### Designing Only For The First Encounter

Delight that charms once but irritates on the hundredth use is poorly calibrated for real product use.

## Self-Check

- [ ] Feedback is designed before delight, and every input confirms receipt, progress, success, or failure.
- [ ] The intensity of each microinteraction matches the significance of the action, with celebration reserved for genuinely valued moments.
- [ ] Direct manipulation responses begin within roughly 100 milliseconds to feel connected to the user's input.
- [ ] Feedback appears on or near the acted-on element, or is visually connected to it when it appears elsewhere.
- [ ] Delight rewards genuine accomplishment rather than trivial or automatic actions.
- [ ] Delightful moments are brief, non-blocking, and skippable on repeated encounters.
- [ ] The resulting state remains unambiguous after the animation; clarity survives the delight.
- [ ] High-frequency actions use fast, unobtrusive responses calibrated for the hundredth encounter, not just the first.
- [ ] Motion-based feedback has redundant cues, and a reduced-motion path keeps the microinteraction clear.
