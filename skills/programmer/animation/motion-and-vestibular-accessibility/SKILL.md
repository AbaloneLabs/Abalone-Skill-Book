---
name: motion_and_vestibular_accessibility.md
description: Use when the agent is adding scroll-tied or background motion (parallax, sticky hero zooms, autoplay carousels, full-screen transitions, screen-flipping onboarding), deciding whether motion should autoplay or be user-initiated, handling prefers-reduced-motion, evaluating flashing or high-contrast animation against seizure thresholds, building motion that may affect users with vestibular disorders, motion sensitivity, migraines, or photo-sensitivity, or reviewing a UI for motion that could trigger nausea, dizziness, or seizures. Also covers WCAG 2.3 animation thresholds, the difference between essential and non-essential motion, large-scale movement risk, and the failure mode of treating reduced-motion support as an optional polish rather than a safety requirement.
---

# Motion And Vestibular Accessibility

Motion in an interface is not experienced identically by everyone. For most users, a parallax scroll, a full-screen transition, or an autoplaying carousel is a minor aesthetic choice. For a meaningful minority — users with vestibular disorders, motion sensitivity, migraines, or photo-induced seizure conditions — the same motion is physically harmful: it triggers dizziness, nausea, disorientation, migraine episodes, or, at the extreme, seizures. This is not a preference about taste; it is a safety issue. An interface that ships motion known to harm these users, without a way to avoid it, is an interface that excludes and injures them, and in several jurisdictions it is an accessibility violation. The judgment problem is deciding, for each piece of motion, whether it carries risk, how to reduce or remove that risk, and how to provide a safe path for users who cannot perceive the motion without harm.

Agents tend to treat motion accessibility as optional polish because the harm is invisible: the developer does not experience it, the demo looks impressive, and the affected users simply leave the site rather than complain. The result is interfaces with autoplay parallax, full-screen zoom transitions, and rapid flashing that are genuinely unsafe for the population they harm. The mirror failure is treating *all* motion as dangerous and removing it entirely, which discards the genuine value motion provides (guidance, feedback, perceived-latency reduction) for the users who benefit. The judgment is to distinguish essential from non-essential motion, to remove or reduce the motion that carries vestibular or seizure risk, and to make the reduced-motion path a deliberate, tested part of the feature rather than an afterthought.

## Core Rules

### Honor prefers-reduced-motion As A First-Class Requirement

`prefers-reduced-motion` is the user's OS-level signal that they want less motion. A meaningful fraction of users have it enabled — not only those with diagnosed conditions, but anyone who finds motion distracting or uncomfortable. Honoring it is not optional polish; it is the baseline accessibility contract for motion.

- **Provide a reduced-motion path for every non-essential animation.** When the media query matches, remove or replace decorative motion: a fade becomes an instant cut, a slide becomes an appearance, a parallax becomes static, an autoplay carousel becomes static or user-advanced.
- **Build the reduced-motion path as part of the feature, not as a bug fix.** Wire the media query when the animation is added; do not defer it. A feature shipped without reduced-motion support is a feature shipped broken for those users.
- **Test that the media query actually engages.** Enable the OS reduced-motion setting (macOS, iOS, Windows, Android all expose it) and confirm the experience changes. An unwired or incorrectly-targeted media query is a silent failure.

The reduced-motion path is the correct experience for those users, not a degraded version of the "real" experience. Build it deliberately.

### Distinguish Essential From Non-Essential Motion, And Remove The Non-Essential Under Reduced Motion

Not all motion can be removed — some motion communicates essential information (a progress bar showing real work, a loading indicator with no alternative). The discipline is to classify each animation and handle it accordingly:

- **Non-essential motion: remove or replace entirely under reduced motion.** Decorative parallax, entrance flourishes, ambient floating elements, autoplay carousels, background zoom, scroll-tied movement. These carry no information; removing them loses nothing for the reduced-motion user.
- **Essential motion: minimize but do not necessarily remove.** A progress bar tied to real work, a loading spinner when no text alternative exists. These may remain, but should be minimized (slower, smaller, less screen-covering) and should never be the sole carrier of information (pair with text or a static indicator).
- **Never make essential motion the only way to perceive something.** A change conveyed only by animation is invisible to reduced-motion users and to screen-reader users. Always pair motion with a non-motion channel (text, icon, announcement).

When in doubt, remove rather than reduce. A reduced-motion user prefers a static, functional interface over a "minimized" animation that still triggers discomfort.

### Never Autoplay Large-Scale, Parallax, Or Full-Screen Motion

The highest-risk motion for vestibular harm is motion the user did not initiate and cannot avoid: full-screen background movement, scroll-tied parallax that moves large areas of the screen, hero images that zoom on scroll, autoplaying carousels that slide the whole viewport. These create the visual-vestibular mismatch that triggers motion sickness, because the user's eyes perceive large-scale movement while their body perceives stillness.

- **Do not autoplay large-scale motion.** If motion must be present, make it user-initiated (the user clicks to advance, scrolls deliberately) rather than imposed on load or on every scroll.
- **Parallax and scroll-tied movement are the most common culprits.** Background layers moving at different speeds, sticky elements that transform as you scroll, full-bleed images that scale — these are the patterns most reported as nausea-inducing. Under reduced motion, replace them with static equivalents; consider whether they are needed at all.
- **Full-screen view transitions** (onboarding screens that flip, route changes that sweep the whole viewport) carry similar risk at scale. Keep them short, user-initiated, and provide a static alternative.

The question for any large-scale motion: is this worth the risk to users for whom it is harmful? Usually the answer is no, and a static or smaller-scale alternative achieves the same goal safely.

### Keep All Flashing Well Below The Seizure Threshold

Flashing or high-contrast alternating motion can trigger photo-induced seizures in susceptible individuals, and this is a hard safety line, not a guideline. WCAG 2.3.1 (Seizures and Physical Reactions) sets the threshold: content must not flash more than three times per second, or the flash must be below the general flash and red flash thresholds (small enough and low-contrast enough to be safe). Large-area, high-contrast flashing is the most dangerous.

- **Never flash more than three times per second.** This is the hard limit; design any flashing animation well below it.
- **Avoid large-area, high-contrast flashing entirely.** A full-screen alternation between bright colors, or a large flashing overlay, is the highest-risk pattern and should not be used regardless of rate.
- **Scrutinize loading animations, error indicators, and video overlays.** These are common sources of unintended rapid flashing. A strobing error state or a flickering loader can cross the threshold.
- **Test with a photosensitive epilepsy analysis tool** if any flashing is present. Tools exist (PEAT, browser-based analyzers) to evaluate whether content crosses the threshold.

This is the one area of motion accessibility where the consequence is medical harm. Treat the threshold as inviolable.

### Avoid Surprising Or Disorienting Motion The User Did Not Initiate

Even motion below the seizure threshold can disorient if it is unexpected: an element that suddenly flies across the screen, a layout that reflows dramatically on scroll, a background that shifts without user action. The vestibular system is sensitive to unexpected visual movement, especially when the user is not braced for it.

- **Prefer user-initiated motion over imposed motion.** Motion that responds to a click, a drag, or a deliberate scroll is expected; motion that starts on its own (autoplay, ambient loops, scroll-triggered reveals that sweep large areas) is not.
- **Keep motion local rather than full-screen when possible.** A small element animating in place is less disorienting than the whole viewport transforming.
- **Avoid motion that creates a false sense of physical movement.** Simulated camera moves, zooming that implies forward/backward motion, and rapid panning are the patterns most likely to induce vestibular discomfort.

### Provide Controls For Motion Where It Is Central To The Experience

For experiences where motion is genuinely central (a game, an immersive storytelling site, a data visualization with essential animation), provide a way for users to reduce or disable it:

- **An in-app motion or animation toggle**, distinct from the OS setting, for users who want less motion on this specific site.
- **A static or reduced-motion alternative path** through the content, so users who cannot tolerate the motion still get the information or experience.
- **Clear labeling** of motion-heavy sections so users can choose whether to engage.

This is especially important for content where the OS reduced-motion setting may not be respected (canvas, WebGL, video) — provide an explicit control in those cases.

## Common Traps

### Autoplay Parallax With No Reduced-Motion Path

A hero with scroll-tied parallax and ambient floating elements, shipped without a `prefers-reduced-motion` alternative. For motion-sensitive users this is not a minor annoyance — it can trigger vestibular discomfort or nausea. Build the reduced-motion path as part of the feature.

### Treating Reduced Motion As Optional Polish

Deferring reduced-motion support because "we'll add it later" or "it's a nice-to-have." Motion that harms users is a shipped defect. Wire the media query when the animation is added and test that it engages.

### Flashing Near Or Above The Seizure Threshold

A loading animation, error indicator, or video overlay that flashes rapidly or alternates high-contrast colors more than three times per second. This is a hard accessibility failure (WCAG 2.3.1) and a real seizure trigger. Keep all flashing well below the threshold and avoid large-area high-contrast flashing entirely.

### Large-Scale Imposed Motion

Full-screen view transitions, background zooms on scroll, or sweeping reveals that move large areas of the screen without user initiation. These create the visual-vestibular mismatch that induces nausea. Prefer user-initiated, local motion.

### Motion As The Only Signal

A change conveyed only by animation (a shake for error, a glow for success) is invisible to reduced-motion users and screen-reader users. Pair every motion-conveyed state with text, an icon, or an announcement.

### Reduced-Motion Path That Only Slows Motion

Reducing the speed or size of an animation rather than removing it, which can still trigger discomfort for sensitive users. For non-essential motion, remove entirely under reduced motion; do not merely minimize.

### Assuming "No One Complained" Means It's Safe

The absence of complaints does not mean the motion is safe — affected users typically leave rather than report. Ship motion accessibility proactively based on the known risk, not reactively based on complaints.

### Ignoring Canvas, WebGL, Or Video Motion

Motion in canvas, WebGL, or video may not respect the CSS `prefers-reduced-motion` media query automatically. Provide explicit controls or reduced-motion alternatives for these contexts.

## Self-Check

- [ ] `prefers-reduced-motion` is honored with a deliberate reduced-motion path that removes non-essential motion entirely (not merely slows it) and minimizes essential motion — and the path was tested by enabling the OS setting and confirming the experience changes.
- [ ] Each animation is classified as essential or non-essential: non-essential motion is removed under reduced motion, essential motion is minimized and paired with a non-motion channel (text, icon, announcement) so it is never the sole carrier of information.
- [ ] No autoplay large-scale, parallax, or full-screen motion is present; motion that exists is user-initiated and local rather than imposed on the whole viewport.
- [ ] No content flashes more than three times per second, and no large-area high-contrast flashing exists — verified against the WCAG 2.3.1 seizure threshold, with flashing animations scrutinized (loading states, error indicators, video overlays).
- [ ] Motion does not create a false sense of physical movement (simulated camera moves, rapid panning, zoom implying forward/backward motion) that would induce vestibular discomfort.
- [ ] For experiences where motion is central (games, immersive content, canvas/WebGL/video), an explicit in-app motion toggle or static alternative path is provided and labeled, since the OS setting may not be respected automatically.
- [ ] Motion is never the only signal for a state change — every motion-conveyed state is paired with text, an icon, or a live-region announcement.
- [ ] The highest-risk cases were verified — autoplay parallax engaging under reduced motion, flashing against the seizure threshold, large-scale imposed motion, and motion-only signals — not only the smooth default path.
- [ ] Motion accessibility was treated as a safety requirement shipped with the feature, not as optional polish deferred until complaints, recognizing that affected users leave rather than report.
