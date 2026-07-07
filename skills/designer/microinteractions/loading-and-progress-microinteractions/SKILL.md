---
name: loading_and_progress_microinteractions.md
description: Use when the agent is designing loading indicators, spinners, progress bars, skeleton screens, optimistic UI, incremental loading, pull-to-refresh, upload and download progress, and the small-scale feedback that tells users the system is working during short waits.
---

# Loading And Progress Microinteractions

Waiting is where trust is won or lost. When the user acts and nothing visible happens, they assume the system is broken, the action failed, or they need to try again, often leading to duplicate submissions and compounded problems. Loading and progress microinteractions exist to bridge the gap between the user's action and the system's response, but they are frequently designed as generic spinners that communicate nothing useful. The judgment problem is that the right loading treatment depends entirely on how long the wait is, whether the duration is known, and what the user needs to feel confident that the system is working.

Use this skill before choosing between spinners, progress bars, skeleton screens, optimistic updates, and other waiting feedback, and before deciding what the user should see during uploads, downloads, refreshes, and data fetches. The goal is to prevent the agent from defaulting to a single spinner everywhere and ignoring the duration, predictability, and psychological needs of the wait.

## Core Rules

### Match The Indicator To The Duration And Predictability

Different waits need different treatments, and the choice should be driven by how long the wait lasts and whether the duration is knowable.

- very short waits, under roughly 300 milliseconds, usually need no indicator at all, because showing one adds a flash that feels broken;
- short, indeterminate waits of a second or two can use a spinner or small inline indicator, because the user just needs to know work is happening;
- longer, determinate waits where progress can be measured should use a progress bar or percentage, because users tolerate waits far better when they can see how far along they are;
- content-heavy waits, where the eventual layout is known, should use skeleton screens, because they reduce perceived wait time by showing structure immediately.

The most common error is using an indeterminate spinner for a long, measurable process, which leaves the user with no idea whether they are 10 percent or 90 percent done.

### Prefer Skeleton Screens For Known Layouts

A skeleton screen shows the shape of the content that is about to appear, using gray placeholders where text, images, and controls will go. It reduces perceived waiting time because the user sees structure instantly rather than staring at a blank area or a spinner, and it prevents layout shift when the real content arrives.

Skeletons work best when the eventual layout is predictable, such as a list, a card grid, an article, or a dashboard. They are less useful when the content shape varies wildly or when the wait is short enough that a skeleton would flash and disappear. Match the skeleton's structure to the real content as closely as possible, including approximate proportions, so the transition to real content is smooth rather than jarring.

### Use Determinate Progress Whenever Possible

Humans tolerate known waits far better than unknown ones. A progress bar that moves from 0 to 100 percent, even if slow, gives the user a sense of trajectory and an estimate of remaining time. An indeterminate spinner offers no such reassurance and becomes anxiety-inducing the longer it runs.

Whenever the system can measure progress, such as during uploads, downloads, batch processing, or multi-step operations, show determinate progress. When progress cannot be measured, be honest with an indeterminate indicator rather than faking a determinate one, because a bar that stalls or jumps backward destroys trust faster than a spinner.

### Avoid Fake And Stalled Progress

A common, damaging pattern is the progress bar that races to 90 percent and then hangs, or that fills based on time rather than real progress to create an illusion of momentum. Users recognize this trick quickly, and once they do, they distrust every progress indicator in the product.

Show real progress when possible. If real progress is unavailable, use an indeterminate indicator rather than a dishonest determinate one. If you must approximate, slow the approach to completion and finish decisively when the work is actually done, rather than parking near the end.

### Preserve Context And Layout During Loading

Loading should not destroy the user's place in the interface. If a user taps a filter and the results reload, the filter controls, header, and surrounding context should remain stable while only the results area shows loading. Replacing the entire screen with a spinner forces the user to rebuild their context when the content returns.

Reserve the space that content will occupy so that the arrival of data does not cause layout shift. A skeleton screen or a fixed-height loading region prevents the page from jumping, which is both disorienting and a source of misclicks.

### Provide Optimistic UI Where Safe

Optimistic UI shows the expected result immediately, before the server confirms it, on the assumption that the action will succeed. It makes the interface feel instant for common, low-risk actions such as liking, toggling, or adding to a list. The user sees their change reflected at once, and the system reconciles in the background.

Optimistic UI is appropriate only when failure is unlikely and the cost of rolling back is low. It is dangerous for high-stakes or irreversible actions such as payments, submissions, or deletions, where showing success before confirmation can mislead the user into believing something happened that later fails. Match the optimism to the stakes.

### Handle Long Waits With Staged Feedback

For genuinely long operations, a single indicator is not enough. Stage the feedback as the wait extends: start with a simple indicator, add an estimated time or progress when available, offer the option to continue elsewhere or be notified when done, and never leave the user staring at an unchanged spinner for minutes.

Long waits should respect the user's time by allowing them to navigate away, receive a completion notification, or cancel. A loading state that traps the user in place for an extended duration is a usability failure regardless of how nice the spinner looks.

### Make Refresh And Pull-To-Refresh Meaningful

Pull-to-refresh and manual refresh actions should give clear feedback that the refresh occurred and whether anything changed. A spinner that appears and disappears without indicating whether new content arrived leaves the user unsure if the refresh worked. Confirm the outcome: show new content, indicate nothing changed, or surface a count of updates.

Avoid refresh patterns that silently do nothing or that reload the entire view destructively, discarding the user's scroll position or selections.

## Common Traps

### One Spinner Everywhere

Using an indeterminate spinner for every wait, including long measurable ones, leaves users without trajectory and increases abandonment.

### Spinner For Sub-300ms Waits

Showing a loading indicator for waits so short it flashes and vanishes makes the interface feel broken rather than fast.

### Fake Progress Bars

Bars that fill on a timer rather than real progress, or that stall near completion, destroy trust once users notice the pattern.

### Full-Screen Loading That Destroys Context

Replacing the entire screen with a spinner discards the user's place and forces reorientation when content returns.

### Optimistic UI On High-Stakes Actions

Showing success before confirmation for payments, submissions, or deletions misleads users when the action later fails.

### Layout Shift On Arrival

Loading regions without reserved space cause the page to jump when content loads, leading to disorientation and misclicks.

### Silent Refresh

Refresh actions that give no outcome feedback leave users unsure whether the refresh worked or changed anything.

### Trapping Users In Long Waits

Loading states that block navigation for extended durations disrespect the user's time and offer no escape.

## Self-Check

- [ ] The loading indicator matches the duration and predictability of the wait: none for sub-300ms, spinner for short indeterminate, progress bar for determinate, skeleton for known layouts.
- [ ] Skeleton screens are used for content-heavy waits with predictable layouts and match the real content's approximate structure.
- [ ] Determinate progress is shown whenever the system can measure it, with honest movement rather than time-based fakery.
- [ ] No fake or stalled progress bars are used to create an illusion of momentum.
- [ ] Loading preserves the user's context, with reserved space preventing layout shift when content arrives.
- [ ] Optimistic UI is used only for low-risk, easily reversible actions, never for high-stakes or irreversible ones.
- [ ] Long waits provide staged feedback, estimated time or progress, and an option to navigate away or be notified.
- [ ] Refresh and pull-to-refresh confirm the outcome, including whether anything changed.
- [ ] No loading state traps the user in place for an extended duration without an escape path.
