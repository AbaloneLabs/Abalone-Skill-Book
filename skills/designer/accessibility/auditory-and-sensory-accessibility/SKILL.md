---
name: auditory-and-sensory-accessibility.md
description: Use when the agent is designing for users who are deaf or hard of hearing, designing for sensory sensitivity, handling captions and transcripts, managing audio and motion that may trigger seizures or discomfort, or evaluating auditory, vestibular, and sensory accessibility risks in media-rich interfaces.
---

# Auditory and Sensory Accessibility

Auditory and sensory accessibility covers the needs of users who are deaf or hard of hearing, users with sensory processing differences, users sensitive to motion and animation, and users at risk of seizures or vestibular discomfort from visual stimuli. It is a distinct concern from motor or cognitive accessibility, though they overlap. The core judgment problem is that media-rich interfaces increasingly rely on sound, animation, parallax, autoplay video, and dynamic motion to convey information and emotion, and each of these can exclude, distress, or harm a meaningful subset of users.

Agents tend to treat captions and "reduce motion" as the entirety of this domain. In reality, the risks are broader: information locked inside audio with no text equivalent, sounds used as the only status signal, animations that trigger vestibular responses, flashing that risks seizures, autoplay that ambushes users, and visual complexity that overwhelms sensory processing. The harm ranges from exclusion (a deaf user cannot get critical information) to physical harm (a seizure) to cumulative distress (an autistic user abandoning a product).

This skill gives the agent judgment for the auditory and sensory dimensions of an interface.

## Core Rules

### Provide text equivalents for all audio, and audio equivalents where sound carries unique information

Anything conveyed by sound must also be available as text: captions for speech, transcripts for audio, and visible status indicators for audio alerts, errors, and notifications. Conversely, where information is conveyed only visually and could benefit auditory access (for blind or low-vision users), provide audio alternatives. The principle is that no critical information depends on a single sensory channel. Audit every place sound carries meaning and ensure a non-auditory equivalent exists.

### Make captions accurate, complete, and meaningful, not present

The existence of captions is not the same as usable captions. Captions must include all meaningful speech, identify speakers, convey relevant non-speech audio (a doorbell, an alarm, music mood), and be synchronized with the media. Auto-generated captions are a starting point, not a finish line; they routinely mangle names, numbers, and technical terms. For critical or public content, human-reviewed captions are the standard. Position captions so they do not obscure essential visuals.

### Give users control over autoplay, volume, and media playback

Autoplaying audio or video ambushes users: it can startle, interfere with screen readers, blast in quiet environments, and consume data. Media with audio should not autoplay; if it must autoplay, it should start muted with visible, accessible controls to play sound. Always provide accessible controls for play, pause, stop, volume, and captions, and ensure these controls are keyboard operable and labeled.

### Respect motion sensitivity and provide a real reduce-motion path

A meaningful minority of users experience nausea, dizziness, headaches, or distress from parallax, auto-moving carousels, large-scale animations, and smooth scrolling. Provide a genuine reduced-motion alternative that substantially removes or simplifies motion, not merely a token toggle that leaves the triggering effects in place. Honor the operating-system reduce-motion preference automatically. Large-scale movement, especially of background elements, is the highest-risk category and should be conservative by default.

### Eliminate seizure-triggering flashing and enforce safe limits

Content that flashes more than a few times per second, or with high contrast and large area, can trigger photosensitive seizures. Apply the established flashing limits rigorously, and treat any flashing, strobing, or rapid transitions as a potential hazard requiring review. This is one of the few accessibility issues that can cause direct physical harm; treat it as non-negotiable. When in doubt, remove the effect.

### Design for sensory load and processing differences

Interfaces with multiple simultaneous animations, auto-rotating content, dense motion, unpredictable audio, and high visual complexity can overwhelm users with sensory processing differences, autism, or attention conditions. Prefer calm, predictable, user-initiated motion over ambient movement. Allow users to pause, simplify, or disable decorative motion and background activity. Reduce unnecessary simultaneous stimuli, especially in flows requiring focus or decision-making.

### Do not rely on color or a single sensory channel for meaning

Information conveyed by color alone, sound alone, or shape alone excludes users who cannot perceive that channel. Status indicated only by a red border fails colorblind and screen-reader users; an error indicated only by a beep fails deaf users. Always pair sensory cues: color plus text, sound plus a visible indicator, shape plus a label.

### Ensure audio quality and clarity for users who rely on residual hearing

For users who are hard of hearing rather than fully deaf, audio quality matters: clear speech, minimal background noise, adjustable playback speed, and the ability to boost or isolate speech. Avoid layering important speech over music or noise. Provide controls for playback speed and, where feasible, audio enhancement.

## Common Traps

### Treating captions as a checkbox rather than a quality bar

Adding any captions is treated as completion, but inaccurate, incomplete, or poorly synchronized captions can be worse than none, because they mislead. Auto-captions full of errors communicate that deaf users are an afterthought. Define a quality standard for captions and transcripts, especially for public and critical content.

### Providing a reduce-motion toggle that does not actually reduce motion

A common pattern is a "reduce motion" setting that disables one decorative animation while leaving parallax, auto-scroll, and large-scale movement in place. This is a trap: it signals accommodation while delivering none. Audit the reduce-motion path against the actual high-risk effects and ensure it substantively removes them.

### Autoplaying media and assuming muted is enough

Muted autoplay still consumes resources, distracts, and can interfere with assistive technology, and unmuted autoplay is a severe barrier. Treat autoplay as a last resort, and never let media with sound start without explicit user action. Autoplay that cannot be disabled is an accessibility failure.

### Forgetting non-speech audio as information

Captions often cover speech but omit meaningful non-speech audio: a knock, an alarm, a tone indicating success or failure. For deaf users, these sounds carry information. Captions and transcripts must convey relevant non-speech audio, not only spoken words.

### Ignoring the cumulative sensory environment

Each animation or sound may be acceptable alone, but together they create an overwhelming environment. Agents evaluate effects in isolation and miss the cumulative load. Audit the full sensory environment of a screen or flow, especially dashboards, landing pages, and multimedia experiences.

### Assuming "no one complained" means it is safe

Seizure risk and vestibular discomfort are underreported; users who are harmed often simply leave. The absence of complaints is not evidence of safety. Evaluate motion and flashing against established limits proactively, rather than waiting for harm to be reported.

### Over-relying on color and shape for status

Status badges, error states, and validation frequently rely on color or icon shape alone. This fails colorblind users, users with low vision, and screen-reader users. Always pair the sensory cue with text or another redundant channel.

## Self-Check

- Does every piece of audio content have an accurate, complete, synchronized text equivalent, including relevant non-speech audio?
- Are captions human-reviewed for critical and public content, with speakers identified and positioned to avoid obscuring visuals?
- Does media avoid autoplaying with sound, and are play, pause, volume, and caption controls accessible and keyboard operable?
- Is there a genuine reduced-motion path that removes high-risk effects and honors the OS reduce-motion preference?
- Have you verified that no content exceeds safe flashing limits, treating any strobing or rapid transition as a hazard?
- Have you audited the cumulative sensory load of screens and flows, reducing unnecessary simultaneous motion and sound?
- Is any meaning conveyed by color, sound, or shape alone paired with a redundant text or alternative channel?
- For hard-of-hearing users, is important speech clear of background noise, with adjustable playback speed?
