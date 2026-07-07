---
name: microinteractions.md
description: Use when the agent is designing the anatomy and choreography of microinteractions, defining the trigger, rules, feedback, and loops of a contained interaction, deciding when a microinteraction is warranted, sequencing its states, tuning its timing and easing, or ensuring small-scale interactions form a coherent system rather than ad-hoc animations.
---

# Microinteractions

A microinteraction is a single, contained moment of interaction: a trigger causes the system to follow rules, produce feedback, and possibly loop. It looks like a small animation, but it is really a tiny state machine with its own logic, and getting it right is what makes an interface feel responsive, trustworthy, and alive. Agents tend to treat microinteractions as decoration, add animations without defining the underlying rules, or tune motion by taste until the moments feel inconsistent across the product. The harm is an interface where small interactions feel random, where feedback is missing or misleading, and where the texture of the product erodes trust one tiny moment at a time.

Use this skill before designing the response to a contained action, defining its states, or tuning its motion. The goal is to prevent the agent from animating without logic, from skipping the rules that govern the interaction, or from creating microinteractions that feel disconnected from the rest of the system.

## Core Rules

### Model Every Microinteraction As Trigger, Rules, Feedback, And Loops

A microinteraction is not an animation; it is a small system. The durable model, drawn from interaction design, has four parts: a trigger (what starts it), rules (what determines what happens), feedback (what communicates the state to the user), and loops or modes (how it repeats or changes over time). Skipping any part produces a moment that looks right but behaves wrong.

Define all four parts:

- trigger: the user action or system event that starts it (a tap, a threshold crossed, a time elapsed);
- rules: the logic that determines what the system does in response (what is valid, what is rejected, what sequence follows);
- feedback: how the system communicates the rules and state to the user (visual, audio, haptic);
- loops and modes: how the interaction repeats, accelerates, or changes with continued use.

An animation with no defined rules is a guess at behavior. Defining the four parts turns a vague "make it feel nice" into a designed interaction whose behavior can be reasoned about and tested.

### Separate The Rules From The Feedback

A common confusion is conflating what the system does with how it shows it. The rules determine whether a toggle turns on; the feedback shows that it did. When these are tangled, changing the feedback (a different animation) becomes hard, and the rules become implicit in the visuals rather than explicit in logic. Separating them makes both clearer and more maintainable.

Keep rules and feedback distinct:

- define the rules first: what input is valid, what state results, what constraints apply;
- then design the feedback that communicates that state and those rules to the user;
- ensure the feedback faithfully represents the rules, so the user is never misled;
- allow the feedback to change (rebrand, motion refresh) without rewriting the rules.

Feedback that does not match the rules misleads users about what happened. A spinner that shows "loading" when the request already failed is a rules-feedback mismatch that erodes trust.

### Let Feedback Match The Significance Of The Action

Not every action deserves the same feedback. A toggle changing state needs a clear, immediate response. A like button might warrant a touch of personality. A destructive confirmation needs unmistakable, serious feedback. Matching feedback to significance keeps the interface calm where calm is right and emphatic where emphasis matters.

Scale feedback to significance:

- routine state changes need clear, unobtrusive feedback (a toggle flip, a color change);
- consequential actions warrant more emphatic feedback (a confirmation animation, a status change);
- destructive or irreversible actions need feedback that signals gravity, not playfulness;
- avoid celebratory feedback for mundane actions, which makes everything feel noisy.

A microinteraction that celebrates every trivial action trains users to ignore feedback. A microinteraction that gives no feedback on an important action leaves users uncertain. Match the moment to the meaning.

### Define All States, Including Edge And Error Cases

A microinteraction designed only for the happy path breaks the moment something goes wrong. What happens when the input is invalid, the network fails, the action is interrupted, or the user reverses mid-interaction? Undefined edge and error states produce dead-ends, stuck animations, and confusing silence precisely when the user most needs clarity.

Define every state:

- the initial, active, completed, and idle states;
- the error and failure states, with feedback that explains what went wrong;
- the interrupted state, when the user backs out or the context changes;
- the disabled and loading states, when the interaction is unavailable or pending.

A microinteraction whose error state is "nothing happens" is incomplete. The hard moments are where trust is built or lost, and they must be designed, not left to chance.

### Tune Timing And Easing To Communicate, Not To Show Off

Motion has meaning. The duration and easing of a microinteraction communicate whether something is light or heavy, instant or deliberate, playful or serious. Motion tuned by taste, without principle, produces moments that feel arbitrary and inconsistent. Motion tuned to communicate produces a coherent texture across the product.

Tune motion deliberately:

- use shorter durations for small, frequent interactions (a toggle, a hover) so they feel instant;
- use longer durations for larger or more significant changes so they feel intentional;
- choose easing that matches the physical metaphor (deceleration for things arriving, acceleration for things leaving);
- avoid linear motion for organic interactions, which feels mechanical and dead;
- keep timing consistent across similar interactions so the product feels coherent.

Motion that draws attention to itself, rather than to the state change, is usually wrong. The best microinteraction motion is felt more than noticed.

### Make Microinteractions Reversible Where Possible

A microinteraction that cannot be undone forces the user to be certain before acting, which adds friction and fear. Where the action is low-stakes, making it instantly reversible (toggle back, undo a like, dismiss a swipe) lets users explore and act with confidence. Reversibility is a property of the rules, and it should be designed in, not wished for.

Design reversibility:

- for low-stakes actions, allow instant reversal with no penalty;
- make the reverse action as easy as the original, so undoing is not harder than doing;
- for consequential actions, confirm before committing rather than relying on undo;
- communicate that an action is reversible, so users feel safe exploring.

A product where every microinteraction is final feels rigid and punishing. A product where routine actions are effortlessly reversible feels confident and forgiving.

### Ensure Microinteractions Form A Coherent System

Individual microinteractions designed in isolation produce a product that feels like a collection of unrelated moments. A toggle that flips, a button that fades, and a card that slides, each with different timing and easing, create texture noise. Microinteractions must form a system with shared motion principles, so the product feels like one coherent thing.

Build a coherent system:

- define shared motion principles (durations, easings, directions) that all microinteractions follow;
- reuse motion patterns across similar interactions so users learn the visual language;
- document the motion system so new microinteractions inherit it rather than inventing their own;
- review microinteractions together, not just individually, to catch inconsistency.

A microinteraction that contradicts the system's motion principles feels foreign, even if it is well-designed in isolation. Coherence is a system property that emerges from shared discipline.

### Test Microinteractions Under Real Conditions

A microinteraction that feels perfect in a design tool can fail in reality: too slow on a low-end device, too subtle in bright light, too easy to trigger by accident on a touch screen, or too disruptive when repeated rapidly. Testing under real conditions reveals failures that idealized review hides.

Test realistically:

- test on the target devices, including low-end hardware where motion may lag;
- test in real lighting and contexts where subtlety may be lost;
- test rapid or repeated triggering to catch interactions that break under fast use;
- test with users who rely on reduced-motion settings, which should disable non-essential animation.

A microinteraction that only works in the design tool is not finished. Real conditions are where the interaction is actually experienced.

## Common Traps

### Animating Without Defining The Rules

A microinteraction with motion but no defined trigger, rules, or loops is a guess at behavior; model all four parts.

### Conflating Rules With Feedback

Tangling what the system does with how it shows it makes both hard to change and risks misleading the user; keep them separate.

### Uniform Feedback For All Actions

Celebrating trivial actions or under-responding to consequential ones distorts the interface; scale feedback to significance.

### Designing Only The Happy Path

Undefined error, interrupt, and disabled states produce dead-ends and stuck animations at the moments users need clarity most.

### Motion Tuned By Taste Alone

Durations and easings chosen without principle produce inconsistent texture; tune motion to communicate.

### Making Every Interaction Final

Microinteractions that cannot be undone add friction and fear; make low-stakes actions reversible and easy to reverse.

### Isolated Microinteractions Without A System

Individually fine moments with different timing and easing create texture noise; build shared motion principles.

### Testing Only In The Design Tool

Microinteractions that feel perfect in idealized review can fail on slow devices, in bright light, or under rapid use; test real conditions.

## Self-Check

- [ ] Every microinteraction is modeled as trigger, rules, feedback, and loops or modes, with all four defined.
- [ ] Rules are defined separately from feedback, so the logic is explicit and the visuals can change without rewriting behavior.
- [ ] Feedback matches the significance of the action, emphatic for consequential moments and calm for routine ones.
- [ ] All states are designed, including error, failure, interrupt, disabled, and loading, not only the happy path.
- [ ] Timing and easing are tuned to communicate (light, heavy, instant, deliberate), with consistent principles across similar interactions.
- [ ] Low-stakes actions are reversible, with the reverse action as easy as the original.
- [ ] Microinteractions follow shared motion principles so the product feels coherent, not like unrelated moments.
- [ ] Microinteractions were tested on target devices, in real conditions, under rapid use, and with reduced-motion settings.
