---
name: micro_interaction_and_feedback.md
description: Use when the agent is designing or implementing small interactive feedback (button presses, toggles, likes, drag affordances, swipe gestures, hover and press states, toasts, snackbar confirmations, success and error animations, optimistic UI feedback, empty and loading states, skeleton-to-content transitions), deciding how to acknowledge user input visually, choosing motion character that matches an action's semantic weight, building closed feedback loops for state changes, or reviewing an interface for over-animation, cognitive overload, or feedback that misleads. Also covers the tension between expressive and calm motion, avoiding celebratory animation on serious actions, honesty in loading and progress feedback, and the failure mode of feedback that looks impressive but communicates nothing or actively lies about state.
---

# Micro Interaction And Feedback

A micro-interaction is the small, self-contained animation tied to a single user action — a button press, a toggle flip, a like, a drag, a save confirmation. Its purpose is to close the loop between the user's intent and the system's response: the user did something, and the interface must show, immediately and legibly, what happened and what state they are now in. This is not decoration. When the loop is closed well, the user feels in control and confident; when it is broken — a button that gives no press feedback, a save that shows no confirmation, a toggle that animates but leaves the state ambiguous — the user is left guessing whether their action registered, repeating it, and losing trust. The judgment problem is deciding, for each interaction, what the user needs to perceive to know their action succeeded, failed, or is in progress, and expressing that through motion whose character matches the weight of the action.

Agents tend to fail in two opposite directions. The first is under-feedback: the interaction works mechanically but gives no perceptible response, so the user clicks again (double-submit), assumes it failed, or moves on uncertain. The second is over-animation: every press bounces, glows, ripples, and celebrates, turning routine feedback into noise that slows the user and trivializes serious actions. Both miss the point, which is that feedback is communication. A strong micro-interaction acknowledges the trigger within a frame, makes the resulting state unambiguous, settles into a stable end state, and uses motion whose character fits the action — confident for a confirmation, restrained for a destructive action, never celebratory for an error or grave for a success. The motion is the medium; the function is to tell the user what just happened.

## Core Rules

### Acknowledge Every Trigger Within A Frame

The moment of input — press, tap, hover, key — must produce a visible response within roughly a frame (about 16ms). Any perceptible delay between the user's action and the interface's response feels broken, because the user cannot tell whether their input registered. This is the foundation of all micro-interaction feedback: the system must say "I heard you" instantly, before any processing, animation, or network call completes.

- **Press and active states.** A button must show a pressed/active state the instant it is pressed (scale-down, background change, ripple). A delay here reads as lag or a missed click.
- **Hover and focus states** for interactive elements, so the user knows the element is a target before activating it. Hover must not be the only way to reach functionality (touch devices have no hover), but it provides confirmation on devices that support it.
- **Optimistic UI for actions with latency.** If the action triggers a network call, acknowledge it locally and immediately (disable the button, show a spinner or "Saving…") rather than waiting for the response to show anything. The user should never wonder whether their click was lost in transit.

The test: perform the action and watch for the first visible response. If there is a perceptible gap between input and response, the acknowledgment is too slow.

### Make The Resulting State Unambiguous

After the interaction, the user must know what state they are in. A toggle that animates but leaves it unclear whether it is now on or off has failed; a like button whose heart fills but whose count does not update has created doubt. The end state must be legible without re-reading.

- **The visual end state must clearly encode the new state.** A toggle in the "on" position, a filled heart, a checked checkbox, a highlighted selected tab. If the user has to study the result to know the state, the feedback is insufficient.
- **State changes must be consistent and complete.** If a like increments the count, both the heart fill and the count must update together; a partial update (heart fills, count lags) creates a moment of confusion.
- **Pair visual state with the semantic meaning.** "Saved" should look saved (a checkmark, a confirmation), not merely "different." The motion and the end state together must communicate the outcome, not just that a change occurred.

### Match The Motion Character To The Action's Semantic Weight

Motion has character — confident, playful, urgent, grave — and that character should match the action it represents. Mismatched character is jarring and sometimes offensive: a bouncy, celebratory animation on a delete or a payment failure trivializes a serious moment; a heavy, grave shake on a success feels alarming. The motion is part of the message, and the message must be consistent.

- **Routine confirmations: calm and quick.** A save, a toggle, a selection. The motion is short (100–200ms), eases out, and settles without flourish. It confirms without demanding attention.
- **Playful or celebratory moments: expressive motion is earned.** A game win, a goal achieved, a streak milestone. Here a bounce, a confetti, or an overshoot is appropriate because the moment warrants emphasis. Reserve expressive motion for these; do not deploy it on every routine action.
- **Destructive or error actions: restrained, never celebratory.** A delete, a failure, an error. The motion should be sober — a subtle fade, a clear red state, a shake only if it communicates the error without trivializing it. A celebratory bounce on an error is a tone-deaf failure.
- **Urgent actions: short and direct.** A confirmation needed now. Keep the motion minimal and fast; do not make the user wait through animation for an urgent decision.

The question for each interaction: does the motion's character match what the user just did and how they should feel about it? If a delete bounces like a celebration, the character is wrong.

### Build A Closed Loop: Trigger, Change, Result, Recovery

A complete micro-interaction is a closed loop, not a single animation. The loop has stages, and each must be handled:

- **Trigger acknowledged** (instant press/active feedback).
- **Change communicated** (the animation that shows the transition to the new state).
- **Result settled** (the stable end state, unambiguous).
- **Recovery defined** (what happens if the action fails or is reversed — the save that errors reverts to unsaved with a clear error; the toggle that the user toggles back animates in reverse from its current state).

The recovery stage is the most often missed. An interaction that can succeed but not show failure, or that cannot be reversed mid-animation, is half a loop. Define what the user sees if the action fails (revert the optimistic UI, show an error, re-enable the control) and ensure the animation can reverse from any point.

### Keep Feedback Calm For Routine Actions, Expressive Only For Emphasis

The most common over-animation failure is treating every interaction as an opportunity for expressive motion: buttons that ripple and bounce on every press, toggles that overshoot dramatically, icons that spin on hover. At the first use this looks polished; at the hundredth use it is exhausting noise that slows the user and dilutes the moments that genuinely warrant emphasis.

- **Default to calm, quick feedback for routine actions.** A button press is a short scale or color change, not a performance. The user performs it dozens of times; respect their time.
- **Reserve expressive motion for moments that earn it.** Achievements, completions, rare or high-stakes successes. When expressive motion is rare, it carries weight; when it is constant, it carries none.
- **Question any animation that would annoy a user performing the action fifty times a day.** If it would, it is too elaborate for a routine interaction.

### Make Loading And Progress Feedback Honest

Loading and progress states are feedback for actions that take time, and their honesty determines whether the user trusts the system. A progress bar that fills to 90% and sticks, a spinner that runs forever with no signal of progress, a "Saving…" that never resolves — these are broken promises that erode trust faster than a slow load.

- **Use skeleton screens for content loads** to show shape immediately and let the user anticipate the content; the skeleton should match the final layout to avoid a jarring reflow.
- **Use determinant progress when the work is measurable** (a real percentage, steps completed). Determinant progress is always preferable to indeterminate when you can compute it.
- **Use honest indeterminate states when progress is unknown** — a clearly labeled "Loading…" with motion that signals activity without implying a near end. Avoid the 99%-stuck progress bar, which implies a near end that never arrives and is worse than no progress bar.
- **Provide an escape when work fails.** A loading state that never resolves and offers no retry or error is a trap. Show an error and a recovery path when the work fails.

A loading animation that runs for a second is helpful; one that runs for ten seconds with no change is a broken promise. Tie the animation to the actual state of the work.

### Ensure Feedback Never Misleads About State

Feedback that looks like success when the action failed, or looks like progress when nothing is happening, is worse than no feedback because it creates false confidence. The cardinal rule: the feedback must reflect the true state.

- **Do not show success before the action completes** unless you are deliberately using optimistic UI with a defined revert-on-failure path. A "Saved!" toast shown before the save request returns, with no revert if it fails, leaves the user believing data was saved when it was not.
- **Do not show a progress bar advancing on a timer** when the underlying work is not progressing. A bar that creeps forward on a fixed interval regardless of real progress is a lie.
- **Distinguish "in progress" from "done" and "failed" visually.** A spinner means in progress; a checkmark means done; an error icon means failed. Conflating them (a spinner that means "done" in some context) confuses the user.

## Common Traps

### No Press Feedback, So The User Double-Submits

A button with no visible press state, so the user cannot tell their click registered and clicks again — submitting a form twice, duplicating an action. Acknowledge every trigger within a frame; disable the control after first press for actions that must not repeat.

### Celebratory Animation On A Serious Or Failed Action

A delete that bounces, or a payment failure that plays a success-like flourish. The motion's character trivializes or misrepresents the outcome. Match the motion character to the action's weight; keep errors and destructive actions restrained.

### Over-Animated Routine Interactions

Every button press ripples, glows, and overshoots. At the hundredth use it is noise that slows the user and dilutes emphasis. Default to calm, quick feedback; reserve expressive motion for moments that earn it.

### Toggle Or Like That Animates But Leaves State Ambiguous

A toggle that moves but does not clearly settle into on or off, or a like whose heart fills but whose count does not update. The user cannot tell what state they are in. Make the end state unambiguous and complete.

### The 99%-Stuck Progress Bar

A progress bar that fills to 90% quickly then creeps or stalls, implying a near end that never arrives. This is worse than an honest indeterminate spinner. Use real progress when measurable; use a clearly indeterminate indicator when not.

### Optimistic UI With No Revert On Failure

Showing "Saved!" before the request completes, then the request fails and the UI stays in the saved state. The user believes data was saved when it was not. Either wait for confirmation before showing success, or implement a revert-to-unsaved with a clear error on failure.

### Feedback That Cannot Reverse Mid-Animation

A toggle whose animation must complete before a reverse toggle registers, so a user who toggles back rapidly sees the animation play out fully or lands in the wrong state. Read the current state on each input and redirect the animation from wherever it is.

### Success Animation On An Error Path

A toast or animation that looks identical whether the action succeeded or failed, so the user cannot distinguish the outcome. Distinguish in-progress, done, and failed visually and textually.

### Hover-Only Feedback On Touch Devices

Feedback that appears only on `:hover`, which does not exist on touch devices, so mobile users get no acknowledgment. Provide tap/active-state feedback that works without hover.

## Self-Check

- [ ] Every user trigger (press, tap, key) produces a visible response within a frame — verified by performing the action and confirming no perceptible gap between input and the first visual response.
- [ ] The resulting state after each interaction is unambiguous: the end state clearly encodes the new state (toggle position, filled heart, checked box), and related elements (counts, labels) update together so the user knows what state they are in without re-reading.
- [ ] The motion character matches the action's semantic weight — routine actions are calm and quick, expressive motion is reserved for moments that earn it, destructive and error actions are restrained and never celebratory.
- [ ] Each micro-interaction is a closed loop: trigger acknowledged, change communicated, result settled, and recovery defined (what the user sees on failure, and the ability to reverse from any point mid-animation).
- [ ] Routine feedback is calm (no bounce/ripple/glow on every press), and expressive motion is rare enough to carry weight when it appears — verified by imagining the user performing the action fifty times.
- [ ] Loading and progress feedback is honest: skeletons match the final layout, determinant progress is used when measurable, indeterminate states are clearly labeled, no 99%-stuck progress bars, and failed work shows an error with a recovery path.
- [ ] Feedback never misleads about state: success is not shown before completion unless optimistic UI has a defined revert-on-failure path, progress bars do not advance on timers without real progress, and in-progress/done/failed are visually distinct.
- [ ] Feedback works across input modes: active/press states exist (not hover-only), so touch and keyboard users receive the same acknowledgment as mouse users.
- [ ] The highest-risk cases were verified — no-press-feedback double-submit, celebratory motion on errors, ambiguous end states, non-reversible mid-animation toggles, and dishonest loading states — not only the smooth single-use demo path.
