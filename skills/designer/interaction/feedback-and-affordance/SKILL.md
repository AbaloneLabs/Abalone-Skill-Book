---
name: feedback_and_affordance.md
description: Use when the agent is designing how an interface communicates the result of user actions, planning system feedback for success, failure, progress, and ambiguity, deciding what response time and modality each action deserves, balancing immediate feedback against deferred processing, designing for perceived performance, or ensuring that every interactive element signals its availability and consequence without relying on color, motion, or sound alone.
---

# Feedback And Affordance

Every action a user takes is a question, and the interface must answer it. Feedback is the answer the system gives: did the action register, is it still processing, did it succeed, did it fail, and what happens next. Affordance is the question the element asks before the action: can this be pressed, dragged, edited, or ignored. Together they form the loop that lets users act with confidence. When the loop is closed, users feel in control; when it is broken, they click again, doubt whether anything happened, and eventually mistrust the product. Most interaction failures are not logic errors but feedback failures: the system did the right thing and never told the user.

Agents tend to fail feedback design in predictable ways. They treat feedback as a single "show a toast" decision rather than a system tuned to action type, consequence, and latency. They make every action feel equally important, so trivial confirmations shout and critical failures whisper. They rely on a single modality, usually color or motion, that vanishes for users who cannot perceive it. Or they design affordance for the default state and forget that the same element must continue to communicate under hover, focus, loading, error, and disabled conditions.

Use this skill before finalizing any interactive behavior, when planning the feedback an action deserves, and when auditing whether elements communicate their availability. The goal is a feedback system whose modality, timing, and prominence match each action's consequence, and an affordance model that survives every state.

## Core Rules

### Match Feedback Prominence To Action Consequence

Not every action deserves the same feedback. A confirmation that shouts for a trivial action trains users to dismiss confirmations, and a whisper for a destructive action lets users walk away believing nothing happened. Feedback prominence should scale with consequence.

Scale feedback by consequence:

- low-consequence, reversible actions, such as liking a post or toggling a filter, need only a subtle, immediate state change;
- moderate actions, such as saving a draft or submitting a form, need clear confirmation that the action succeeded and the result is persisted;
- destructive or irreversible actions, such as deleting data or sending a payment, need prominent, persistent confirmation and a clear path to verify or undo;
- ambiguous outcomes, where success is not yet known, need ongoing feedback that the action is still in flight.

A product where every action triggers a modal confirmation is a product where users stop reading confirmations. Reserve prominence for what matters.

### Close The Loop For Every Action Type

Feedback is not only about success. Users need answers for at least four outcomes: the action registered, it is still processing, it succeeded, and it failed. Designing only the success case leaves the other three to chance.

Design feedback for each outcome:

- immediate acknowledgment, so the user knows the input was received, even before processing completes;
- progress indication for actions that take time, so the user is not left staring at a static screen;
- success confirmation that is proportional and, where useful, reversible;
- failure feedback that explains what went wrong and what to do next, never just a generic error.

An action that produces no immediate response is indistinguishable from a broken action, and users will retry it, often causing duplicate submissions or compounded errors.

### Match Feedback Timing To Perceived And Actual Latency

The timing of feedback shapes perceived performance more than the actual speed of the operation. Users tolerate waiting when they are told they are waiting, and they resent short delays that appear to be freezes. The discipline is to align feedback timing with the latency the user actually experiences.

Tune timing to latency:

- under roughly 100 milliseconds, the response feels instantaneous and needs no explicit progress indicator;
- between 100 milliseconds and 1 second, a subtle state change, such as a button press, is enough, but the user starts to notice the wait;
- between 1 and 10 seconds, show an explicit progress indicator and, where possible, an estimate, because unindicated waits feel broken;
- beyond 10 seconds, the user will likely context-switch, so offer a way to leave and return, and notify on completion.

Hiding latency behind silence is the most common perceived-performance failure. A spinner that appears at 50 milliseconds is mildly annoying; a spinner that never appears at 3 seconds is a bug report.

### Choose Modality Deliberately And Never Rely On One Alone

Feedback can be visual, auditory, or haptic, and each modality has users who cannot perceive it. Choosing a single modality makes the feedback invisible to those users and fragile in environments where that modality is unavailable, such as a muted screen or a screen reader.

Combine modalities with intent:

- pair visual feedback with a secondary cue, such as icon, text, or position, never relying on color or motion alone;
- use sound for events that may occur while the user is not looking at the screen, but never as the only signal;
- use haptic feedback on touch devices for confirmation of physical gestures, but treat it as supplementary;
- ensure every feedback state is comprehensible with a single modality, so that losing one does not lose the message.

### Make Affordance Survive Every State

Affordance is often designed only for the default state, but elements must continue to communicate their availability under hover, focus, loading, error, and disabled conditions. An element whose affordance vanishes under load or error becomes indistinguishable from a broken one.

Design affordance across states:

- every interactive element should remain visibly interactive in its default, hover, focus, and active states;
- disabled and read-only states must be visually distinct from each other and from enabled, with the reason made clear where it is not obvious;
- loading states must not strip the element's identity, so the user knows what is loading and can still locate it;
- error states must preserve the element's affordance so the user can retry or correct.

### Provide Undo And Reversibility Where Actions Are Destructive

The strongest feedback for a consequential action is the ability to take it back. Undo converts a destructive, anxiety-inducing action into a reversible, confident one, and it reduces the need for intrusive confirmation dialogs that interrupt flow.

Prefer reversibility over confirmation:

- where an action can be safely undone, offer undo rather than a blocking confirmation, which users learn to dismiss;
- where an action is truly irreversible, such as permanent deletion or payment, require explicit confirmation and make the consequence unmistakable;
- make undo discoverable and time-bounded clearly, so users know how long they have to reverse;
- log reversible actions so users can verify what they did and return to a prior state.

### Design Feedback For Errors As Recovery, Not Blame

Error feedback is where feedback design most often fails users. A generic "something went wrong" tells the user nothing and implies the failure is theirs to resolve unaided. Effective error feedback explains what happened, why, and what to do next.

Design errors as recovery paths:

- explain the error in plain language, avoiding codes or internal terminology;
- attribute the cause honestly, distinguishing user input errors from system failures;
- provide a clear next action, such as retry, correct a field, or contact support;
- preserve the user's input and context so recovery does not mean starting over.

### Test Feedback Under Real Conditions

Feedback that works in a quiet demo fails in the real world: on slow networks, under interrupted attention, on assistive technology, and in environments where sound is off. Feedback must be validated under the conditions users actually experience.

Test realistically:

- test under throttled networks and slow processing to confirm progress feedback appears when it should;
- test with screen readers and keyboard navigation to confirm feedback is perceivable without sight or mouse;
- test with sound off to confirm no critical feedback depends on audio;
- test interrupted and abandoned actions to confirm the system recovers and the user can resume.

## Common Traps

### Uniform Feedback Prominence

Making every action equally loud trains users to ignore confirmations and miss the critical ones. Scale prominence to consequence.

### Designing Only The Success Case

Actions also register, process, and fail. Feedback that covers only success leaves the other outcomes to chance.

### Hiding Latency Behind Silence

Unindicated waits feel like freezes. Align feedback timing with the latency users actually experience.

### Single-Modality Feedback

Color-only, motion-only, or sound-only feedback is invisible to users who cannot perceive that modality. Combine cues.

### Affordance That Vanishes Under Load Or Error

Elements must remain identifiable and interactive across loading, error, and disabled states, not only in the default.

### Confirmation Dialogs For Reversible Actions

Blocking confirmations for actions that could simply be undone interrupt flow and train users to dismiss them.

### Generic Error Messages

"Something went wrong" offers no recovery path. Errors should explain, attribute honestly, and direct the next action.

### Untested Real-World Conditions

Feedback that works in a demo fails on slow networks, with sound off, or on assistive technology. Test under real conditions.

## Self-Check

- [ ] Feedback prominence scales with action consequence: subtle for trivial, clear for moderate, prominent and reversible for destructive.
- [ ] Every action has feedback for acknowledgment, progress, success, and failure, not only the success case.
- [ ] Feedback timing matches perceived latency, with progress indicators appearing where waits exceed roughly one second.
- [ ] No feedback relies on a single modality; visual cues are paired with secondary signals, and sound and haptics are supplementary.
- [ ] Affordance survives every state: default, hover, focus, active, loading, error, and disabled elements remain identifiable and appropriately interactive.
- [ ] Disabled and read-only states are visually distinct, and the reason is clear where not obvious.
- [ ] Reversible actions offer undo rather than blocking confirmation, and truly irreversible actions require explicit, unmistakable confirmation.
- [ ] Error feedback explains the cause in plain language, attributes honestly, and provides a clear recovery action that preserves user input.
- [ ] Feedback was tested under realistic conditions: throttled networks, sound off, screen readers, and keyboard navigation.
- [ ] No critical feedback depends on a modality that can be unavailable to real users.
