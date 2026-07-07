---
name: motor-accessibility.md
description: Use when the agent is designing for users with motor and dexterity impairments, choosing target sizes, tuning timing and motion thresholds, designing keyboard and switch access, handling drag-and-drop and gesture alternatives, or evaluating pointer and input accessibility risks in a product interface.
---

# Motor Accessibility

Motor accessibility is the practice of making interfaces usable by people whose physical control of pointers, keyboards, touch surfaces, or switches is limited, slow, fatiguing, or atypical. It is not only about making buttons bigger. It concerns the whole input model: how much precision a target demands, how much time a motion allows, whether every action has a non-pointing alternative, whether drag and gesture interactions trap users, and whether the interface punishes mistakes that arise from tremor, weakness, spasticity, or one-handed operation.

Agents tend to under-weight motor accessibility because most design and testing happens with a fast, precise mouse or a healthy thumb on a flagship phone. The result is interfaces that feel fine to the designer but are exhausting, impossible, or error-prone for users with cerebral palsy, Parkinson's, repetitive strain injury, amputation, arthritis, tremor, or temporary injuries. The harm is concrete: failed purchases, locked-out accounts, abandoned forms, and exclusion from essential services.

This skill gives the agent judgment for input and interaction design where motor ability varies widely.

## Core Rules

### Treat target size as a floor, not a goal

Minimum touch target dimensions exist for a reason, but they are a lower bound, not an aspiration. A target that barely meets the minimum still fails when it sits near a screen edge, adjacent to another target, or on a device held one-handed. Treat the published minimum as the smallest acceptable size and enlarge targets wherever density allows. Pay special attention to destructive actions, small icon-only buttons, and closely spaced controls, where a near-miss causes real damage.

### Scrutinize every interaction that requires precision or sustained motion

Drag-and-drop, sliders, drawing, pinch-to-zoom, multi-finger gestures, and small handles all assume fine, stable, sustained control. Each of these needs an equivalent path that uses discrete, forgiving input: type a number instead of dragging a slider, click arrows instead of dragging a handle, select-then-apply instead of drag-and-drop, tap buttons instead of pinch. If a precision interaction has no discrete alternative, it is an accessibility barrier, full stop.

### Audit timing thresholds against real human variation

Autocomplete delays, hover-reveal menus, time-limited toasts, session timeouts, and hold-to-confirm interactions all encode assumptions about how fast a user can act. Many default timings are tuned for able-bodied speed and fail users who move slowly, use switch devices, or read slowly. Identify every time-based interaction, question who it excludes, and either remove the time pressure, make it adjustable, or provide a generous default. Never let a timeout destroy unsaved work without warning and extension.

### Guarantee full keyboard operability and visible focus

Keyboard access is the backbone of motor accessibility, because keyboards, switch devices, sip-and-puff controllers, and voice control all ultimately emit keyboard-like commands. Every actionable element must be reachable, operable, and dismissible by keyboard alone, in a logical order, with a visible focus indicator. Custom widgets must implement the expected keyboard patterns; a custom dropdown that only opens on click is broken for keyboard users. Test tab order, focus trapping, focus restoration after dialogs, and skip links.

### Design for one-handed, one-input, and switch access

Many users operate a device with one hand, one input device, or a single switch. Interfaces that assume simultaneous inputs (pressing modifier keys while clicking, two-finger gestures, chorded shortcuts) exclude these users. Provide single-step alternatives, avoid requiring simultaneous actions, and ensure scanning and switch-accessible patterns exist for complex interactions. A command that can only be performed with a coordinated two-handed gesture is inaccessible.

### Make error tolerance proportional to consequence

Motor impairment produces more accidental activations, missed targets, and partial inputs. The interface must absorb these gracefully: generous hit areas, undo for destructive actions, confirmation for irreversible steps, and forgiving input parsing (ignoring stray characters, accepting multiple date formats). The more severe the consequence of an error, the more recovery and confirmation the design must provide.

### Consider fatigue and cumulative load

Motor accessibility is not only about whether a single action is possible, but about whether a task composed of hundreds of actions is sustainable. Repetitive clicking, long tab chains, and frequent small motions accumulate into fatigue and pain for users with motor conditions. Prefer fewer, larger, more forgiving interactions over many small precise ones. Reduce the number of discrete inputs required to complete a frequent task.

## Common Traps

### Assuming large targets alone solve motor accessibility

Big buttons help, but they do nothing for a drag interaction that has no discrete alternative, a session timeout that logs a slow user out, or a custom widget that traps keyboard focus. Target size is one factor among many; agents who stop at "make it bigger" miss the input-model problems that actually block users.

### Over-relying on hover and mouse-only patterns

Hover-reveal menus, hover-to-expand panels, and drag-only interactions are invisible or impossible to keyboard, touch, and switch users. Any information or action available only on hover is inaccessible. Provide focus and tap equivalents, and never gate critical actions behind hover.

### Tuning timing to the designer's own speed

Defaults inherited from frameworks are often calibrated to fast, able-bodied interaction. A 300ms hover delay, a 5-minute session timeout, or a hold-to-confirm of 800ms may feel instant to the designer and be impossible for a motor-impaired user. Always question inherited timings rather than accepting them.

### Forgetting destructive actions and small adjacent targets

The most damaging motor-accessibility failures cluster around small, dense, or destructive controls: a tiny delete icon next to a save icon, a close button in a corner, a row of icon-only buttons. A near-miss here causes data loss or navigation away. Give destructive and high-stakes targets the most generous sizing and separation.

### Assuming "it works with a mouse" means it is accessible

Mouse operability is necessary but nowhere near sufficient. A control that works with a mouse but has no keyboard path, no visible focus, and no touch equivalent excludes most motor-impaired users. Test with keyboard, switch, and touch, not just the pointer you happen to use.

### Ignoring custom widgets and rich interactions

Native HTML controls carry built-in keyboard and accessibility behavior. Custom-built dropdowns, menus, tabs, and sliders usually do not, unless deliberately implemented. Agents frequently build custom widgets for visual reasons and silently strip away the accessibility the native control provided. Default to native controls; when custom is unavoidable, implement the full expected keyboard and focus behavior.

## Self-Check

- Does every actionable element meet or exceed minimum target size, with extra margin for destructive and edge-adjacent controls?
- Does every precision interaction (drag, slider, gesture, pinch, draw) have a discrete, forgiving alternative?
- Have you identified every time-based interaction and confirmed it does not exclude slow or switch-based users, with no timeout destroying unsaved work?
- Is the entire interface operable by keyboard alone, with logical order, visible focus, focus restoration, and no traps?
- Are there any interactions requiring simultaneous inputs, and do they have single-step alternatives?
- Do destructive actions have undo or confirmation proportional to their consequence?
- Have you tested with keyboard, touch, and ideally switch input, not only a mouse?
- Have you reduced cumulative input load for frequent tasks rather than only checking single actions?
