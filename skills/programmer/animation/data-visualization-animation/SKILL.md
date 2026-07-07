---
name: data_visualization_animation.md
description: Use when the agent is animating charts, graphs, or data-driven visuals — transitions between data states, morphing bars or lines, animated sorting and filtering, animated axis or scale changes, enter/update/exit transitions for data points, object constancy (matching elements across states so they animate rather than flash), or reviewing a chart for motion that obscures values, implies false trends, or misleads. Also covers the tension between engaging motion and accurate data perception, animating aggregation or rescaling, the risk of motion that exaggerates or invents trends, easing that distorts the shape of the data, and the failure mode of animation that looks impressive but makes the chart harder to read or actively deceptive.
---

# Data Visualization Animation

Animating data differs from animating UI chrome: the motion itself becomes part of the message the chart conveys. When a bar grows, a line draws, or a point moves, the user reads that motion as a statement about the data. Done well, animation reveals the structure of a change; done poorly, it obscures values, implies trends that do not exist, or distracts from precise comparison. The cardinal risk is that the motion lies — or appears to lie — about the data. Agents tend to treat chart animation like any other UI animation: add a nice entrance, transition everything smoothly, default to the easing used for panels and buttons. This is the wrong frame. A chart's first job is to let the user read values and compare them accurately; motion is justified only when it helps the user understand a *change* in the data, and must never cost legibility or honesty.

## Core Rules

### Decide Whether The Transition Communicates A Change The User Needs To Understand

Not every data state change warrants animation. Animation is justified when it helps the user perceive *how* the data changed — valuable for sorting, filtering, morphing, and switching series. It is not justified when the change is incidental, when the user needs precise values, or when the motion would obscure more than it reveals.

- **Animate when the change itself is the message.** A resort shows items trading rank; a filter shows items entering and exiting.
- **Do not animate when the user needs precise values.** A dashboard read for exact numbers does not benefit from a grow-in that delays legibility. Prefer instant rendering with a subtle highlight of what changed.
- **Keep entrance animations short.** A bar chart that draws itself in over a second delays the moment the user can read it.

### Preserve Object Constancy So Elements Animate Rather Than Flash

Object constancy is the core technique of honest data animation: matching each graphical element across data states by a stable key, so that when the data changes, the element animates from its old value to its new one rather than disappearing and reappearing. Without it, a data update causes a flash — old elements vanish, new ones pop in — and the user cannot track what became what.

- **Assign each data point a stable key** (an id, a category name) that persists across states, and bind the element to it. On update, the element transitions to its new value; on exit, it animates out; on enter, it animates in.
- **Handle enter, update, and exit as distinct transitions.** Conflating these produces visual chaos.
- **Beware key collisions and unstable keys.** If two points share a key, or the key changes between renders for the same entity, constancy breaks and elements animate to the wrong targets or flash.

### Keep Axis And Scale Changes Honest, Because They Reshape Perception

Animating an axis or scale is high-risk: the scale defines how the user perceives every value. When the y-axis rescales during a transition, every bar or line appears to move even if its underlying value did not change — and the user may read the rescale as a data change.

- **Animate axis changes visibly** so the user perceives the scale is changing, not the data. If ticks and gridlines transition in sync with the data morph, the user attributes the motion correctly. If the axis snaps while the data animates, the motion is ambiguous.
- **Avoid rescaling that exaggerates differences.** An axis that auto-scales to the data range can make a trivial change look dramatic. Decide the scale range deliberately; do not let an animation-driven auto-scale invent visual drama the data does not support.
- **Prefer fixed scales across compared states.** Animating the axis during comparison undermines the comparison.

### Use Easing And Duration That Do Not Distort The Data's Shape

The easing and duration of a data transition affect how the change reads. A slow ease makes a small change feel ponderous; overshoot makes a value appear to exceed its real target; a fast snap makes a large change feel trivial.

- **Prefer ease-in-out or linear for value morphs** where the rate of change should feel even, rather than expressive easing that overshoots or bounces. Overshoot on a bar temporarily shows a value higher than the real data — a misrepresentation, however brief.
- **Keep duration short enough to read the final state quickly** (often 300–600ms, longer than a UI micro-interaction because the change carries information).
- **Match duration to the complexity of the change.** A simple value update is quick; a full resort may need slightly longer so the motion is trackable, but cap it so it does not drag.

### Never Let Motion Imply A Trend Or Relationship The Data Does Not Contain

The most serious failure in data animation is motion that fabricates meaning. A line that draws left-to-right implies a sequential progression; if the data is not sequential, the draw-in implies an order that does not exist. Every animated transition carries an implied narrative that must match the data's actual structure.

- **Do not animate a non-sequential chart as if sequential.** A categorical bar chart whose bars grow left-to-right implies an ordering; if the categories are unordered, animate them together or reveal without implying sequence.
- **Do not dramatize a statistically trivial change.** A 2% change animated with a sweeping morph looks significant; reserve dramatic motion for genuinely significant changes.
- **Be cautious with interpolation.** Interpolating between two data states shows intermediate values that may not be real data points.

### Make Data Animation Accessible And Robust To Rapid Updates

Data visualizations are often updated frequently (live dashboards, streaming data) and must remain legible. Under `prefers-reduced-motion`, replace data transitions with instant updates or a brief, non-motion highlight of what changed. Throttle or debounce rapid updates so a streaming source does not produce constant, unreadable motion; batch updates and transition between stable states. Ensure labels, tooltips, and axis values stay legible, and a tabular alternative exists for users who need exact values.

## Common Traps

### Overshoot Or Bounce On Data Elements Shows False Values

Using spring overshoot or bounce on bars or points makes the element temporarily exceed its real data value before settling — a brief but real misrepresentation: the user sees values the data does not contain.

### Flash Instead Of Morph Due To Missing Object Constancy

Re-rendering the chart on each update without stable keys makes old elements vanish and new ones pop in (a flash) rather than animating from old to new state, so the user cannot track what became what.

### Animating A Non-Sequential Chart As If Sequential

Drawing a categorical bar chart left-to-right or growing bars in sequence implies an ordering the data does not contain — the motion fabricates a narrative the user reads as a progression or ranking.

### Axis Rescale That Reads As A Data Change

Auto-scaling the axis during a transition makes every element appear to move, which the user misreads as the data changing rather than the scale — a perceptual lie about the values.

### Entrance Animation That Delays Reading Values

A chart that draws itself in over a second on every load delays the moment the user can read values — optimizing for polish at the cost of legibility.

### Motion That Dramatizes A Trivial Change

A small percentage change animated with a sweeping morph makes it look significant — the motion amplifies a change the data does not warrant.

### Constant Re-Animation On Streaming Data

A live dashboard that re-animates on every data tick produces constant unreadable motion — frequent updates defeat the purpose of animation, and the chart never settles into a readable state.

## Self-Check

- [ ] Each data transition was judged for whether it communicates a change the user needs to understand — animation is used for sorts, filters, morphs, and series switches, and avoided when the user needs precise values or the change is incidental.
- [ ] Object constancy is preserved: each data point is bound to a stable key, and enter/update/exit are handled as distinct transitions, so updates animate rather than flash — verified by confirming no elements pop in/out on a data change and no key collisions cause mis-targeted animation.
- [ ] Axis and scale changes are honest: transitions are visible (not snapping while data animates), scales are not auto-rescaled to exaggerate differences, and compared states share a fixed scale.
- [ ] Easing and duration do not distort the data's shape — no overshoot or bounce on data elements (which would show false values), duration is short enough to read the final state quickly, and easing conveys the change without dramatizing or trivializing it.
- [ ] Motion does not imply a trend the data lacks: non-sequential charts are not animated as sequential, trivial changes are not dramatized, and interpolation does not show meaningless intermediate values.
- [ ] Under `prefers-reduced-motion`, data transitions are replaced with instant updates or a non-motion highlight, and rapid/streaming updates are throttled or batched.
- [ ] The chart stays legible during and after animation: labels, tooltips, and axis values stay readable, motion does not blur or overlap text, and a tabular alternative exists for users who need exact values.
- [ ] The highest-risk cases were verified — overshoot showing false values, flash from missing object constancy, axis rescale misread as data change, sequential animation of unordered categories, and constant re-animation on streaming data — not only the smooth single-transition demo path.
