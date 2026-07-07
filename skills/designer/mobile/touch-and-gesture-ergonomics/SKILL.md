---
name: touch_and_gesture_ergonomics.md
description: Use when the agent is designing for touch ergonomics, thumb zones, one-handed reach, grip and posture, tap target sizing and spacing, gesture comfort, repetitive strain, accessibility of touch input, fat-finger errors, or any interface where the physical reality of hands, fingers, grips, and device holding determines whether interactions succeed, and must decide target placement, size, separation, and gesture selection based on the body rather than the pixel grid.
---

# Touch And Gesture Ergonomics

Touch interface design is a physical design problem before it is a visual one. A finger is blunt, imprecise, and attached to a hand that grips a device in a posture that changes what is reachable. The judgment problem is not making buttons big enough, though that matters. It is understanding how the hand holds the device, where the thumb can comfortably reach, how grip changes with device size and context, how fingers occlude the screen, and how repeated or awkward motions cause strain and error over time. Agents tend to fail by designing for the pixel grid and the designer's own comfortable two-handed grip, by placing critical actions in zones that are unreachable one-handed, by sizing targets to visual elegance rather than physical contact, and by ignoring that the same gesture is comfortable or painful depending on where and how often it is performed.

Use this skill when designing or reviewing touch interfaces for phones, tablets, kiosks, or any surface where fingers are the input, and when deciding target placement, sizing, spacing, gesture selection, or reachability. The goal is interactions that succeed because they respect the body, not despite it.

## Core Rules

### Design For The Grip And Posture, Not The Pixel

How a device is held determines what is reachable, and grip varies by device size, context, and user. A phone is often held one-handed with the thumb doing the work; a tablet may be held with two hands with thumbs reaching the lower corners, or cradled in one arm with the other hand pointing; a kiosk is operated standing, arm extended. Each posture creates a different zone of comfortable reach and a different zone of strain. Design target placement to the likely grip, not to visual symmetry.

For the target device, determine:

- the likely grip or grips, one-handed, two-handed, cradled, or extended;
- where the thumb or finger naturally rests and comfortably reaches;
- which zones are easy, which require stretch, and which require regripping;
- how grip shifts when walking, lying down, or sharing the screen.

A layout that looks balanced but places primary actions in a strain zone fails the body it must serve.

### Respect The Thumb Zone On Handheld Devices

On a phone held one-handed, the thumb sweeps an arc across the lower and middle of the screen, and the upper corners and top edge are hard or impossible to reach without regripping. This arc is the thumb zone, and it is where frequent and primary actions belong. Placing a primary control in the top-right of a large phone forces the user to adjust their grip or use a second hand for a task that should be effortless.

Apply thumb-zone thinking:

- place frequent, primary actions in the lower and middle reachable zone;
- reserve the top of the screen for less frequent actions, navigation, and status;
- consider how the zone differs for left- and right-handed users;
- account for growing device sizes that shrink the reachable zone relative to the screen.

The thumb zone shrinks as phones grow; a layout that was reachable on a small phone may be painful on a large one.

### Size Targets To Physical Contact, Not Visual Elegance

A fingertip is roughly 10mm wide, and the contact patch is larger and less precise than the visual target the user believes they are hitting. Targets sized for visual neatness, small icons, tight rows, are often too small to hit reliably in motion. Size interactive targets to tolerate imprecise contact, and remember that visual size and touch size are separate decisions: a small visual control can have a larger invisible hit area.

For each target, ensure:

- the hit area meets the platform's minimum and is comfortable for the audience;
- adjacent targets are spaced far enough that a miss does not trigger a neighbor;
- destructive neighbors of safe actions have extra separation;
- small visual controls have expanded hit areas, not shrunk ones.

Visual density must not be bought at the cost of physical reliability.

### Separate Destructive And Safe Targets Generously

The cost of a mis-tap is not uniform. Missing a "like" button and hitting "share" is minor; missing "edit" and hitting "delete" is serious. The spacing and separation between targets should reflect the consequence of an error. Destructive, irreversible, or hard-to-notice actions deserve more separation from their neighbors than safe, reversible ones, and often deserve confirmation or undo as well.

Scale separation to consequence:

- safe, reversible neighbors: standard spacing;
- actions with moderate consequence: increased spacing;
- destructive or irreversible neighbors: generous separation, confirmation, or relocation.

Treating all spacing as equal ignores that some errors are trivial and others are costly.

### Account For Occlusion By The Hand And Finger

The hand and finger obscure part of the screen during interaction. Feedback that appears directly under the contact point, critical labels placed beneath where the thumb rests, and confirmation text hidden by the gripping hand are all invisible to the user at the moment they matter most. Design so that important information and feedback are visible away from the contact and grip areas.

Check for occlusion:

- feedback for an action appears where the hand is not, not only under the finger;
- labels and counters are not permanently hidden by the resting grip;
- confirmation and error messages are visible above or beside the contact area;
- left- and right-handed occlusion are both considered.

A confirmation the user cannot see because their hand covers it is no confirmation.

### Match Gestures To Comfort And Frequency

Gestures have different physical costs. A tap is cheap; a long press held in an awkward position is tiring; a swipe across the full screen is more effort than a short one; multi-touch gestures require two hands or dexterity that not all users have. Match the gesture to how often it is performed and how comfortable it is in the likely grip. A gesture that is elegant in a demo may be painful when performed fifty times in a session.

Evaluate gestures by:

- how often the user performs them per session;
- whether they are comfortable in the likely grip and posture;
- whether they require reach, stretch, or dexterity that excludes some users;
- whether a lower-effort alternative exists for frequent actions.

Frequent actions deserve low-effort gestures; rare actions can afford more effort, but never at the cost of excluding users who cannot perform them.

### Consider Repetitive Strain And Fatigue Over A Session

A single interaction that is slightly awkward is tolerable; the same interaction repeated dozens of times becomes painful. Designs that require repeated reaching to the top of a large screen, repeated long presses, or repeated full-screen swipes can cause fatigue and strain over a session, even when each instance seems acceptable in isolation. Evaluate interactions cumulatively, not just once.

Ask:

- how many times will the user perform this motion in a typical session;
- does the motion require stretch, reach, or an awkward hold;
- could a more proximate or lower-effort alternative reduce cumulative strain;
- are there users for whom the repetition is not just tiring but impossible.

Cumulative comfort determines whether a design is usable for real, sustained use.

### Provide Alternatives For Users Who Cannot Perform A Gesture

Gestures assume dexterity, reach, and motor control that not all users have. A swipe, pinch, or long press that is the only path to an action excludes users with tremor, limited mobility, or one-handed constraints, and conflicts with assistive technology. For any gesture-based interaction, provide a visible, accessible alternative path that does not depend on the gesture.

## Common Traps

### Designing For The Designer's Comfortable Grip

A layout that is easy in the designer's two-handed, seated grip may be unreachable one-handed in the real world. Design for the likely grip and posture.

### Primary Actions In The Top Corners

The upper corners of a large phone are unreachable one-handed. Reserve the top for infrequent actions and place primary actions in the thumb zone.

### Visual Size Treated As Touch Size

A small, elegant icon that is also a small target fails in motion. Decouple visual size from hit area and size targets to physical contact.

### Equal Spacing For Unequal Consequences

Standard spacing between a safe action and a destructive neighbor invites costly errors. Scale separation to consequence.

### Feedback Hidden Under The Hand

Confirmation or error text that appears where the hand grips or the finger contacts is invisible when it matters. Place feedback away from occlusion.

### Elegant Gestures That Fatigue Over Repetition

A gesture that is fine once may be painful after fifty repetitions. Evaluate cumulative strain, not single-instance comfort.

### Gestures As The Only Path

Swipe, pinch, and long-press-only paths exclude users who cannot perform them and conflict with assistive technology. Provide visible alternatives.

### Ignoring Device Size Growth

A layout reachable on a small phone may be painful on a large one. Re-evaluate reach and thumb zones as device sizes change.

## Self-Check

- [ ] Target placement reflects the likely grip and posture, not visual symmetry or the designer's own grip.
- [ ] Frequent and primary actions sit within the thumb zone for one-handed use on handheld devices.
- [ ] Interactive targets are sized to physical contact with adequate hit areas, and small visual controls have expanded hit areas.
- [ ] Destructive or irreversible targets have generous separation from safe neighbors, scaled to consequence.
- [ ] Important feedback and labels are visible away from the hand and finger contact areas, for both left- and right-handed use.
- [ ] Gestures are matched to frequency and comfort, with lower-effort alternatives for frequent actions.
- [ ] Cumulative strain over a session was considered, not just single-interaction comfort.
- [ ] Every gesture-based interaction has a visible, accessible alternative that does not depend on the gesture.
- [ ] Reach and thumb-zone assumptions were re-checked against the actual target device sizes, including large phones.
- [ ] The design was evaluated under realistic conditions: one-handed, in motion, interrupted, and repeated.
