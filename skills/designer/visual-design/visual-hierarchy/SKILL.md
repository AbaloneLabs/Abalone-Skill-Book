---
name: visual_hierarchy.md
description: Use when the agent is establishing what leads on a screen, deciding which elements draw attention first, organizing content by importance, differentiating primary from secondary and tertiary information, balancing competing calls to action, or ensuring users can scan a screen and instantly perceive what matters most.
---

# Visual Hierarchy

Visual hierarchy is the system of cues that tells the eye what to look at first, second, and last. It looks like making some things bigger, but it is really the combined effect of size, weight, color, spacing, position, and contrast that ranks every element on a screen by importance. Agents tend to treat hierarchy as a heading ladder, make everything somewhat prominent so nothing is missed, or apply visual emphasis inconsistently until the screen competes with itself. The harm is a layout where users cannot tell what matters, where every call to action shouts equally, and where the primary task is buried among secondary options.

Use this skill before finalizing the prominence of elements on a screen. The goal is to prevent the agent from flattening hierarchy through over-emphasis, from applying emphasis without a clear ranking, or from creating screens where the eye does not know where to settle.

## Core Rules

### Rank Every Element By Importance Before Styling

Hierarchy begins with a decision about what matters, not with visual styling. Before choosing sizes or colors, rank the elements on the screen by their importance to the user's task. Without an explicit ranking, styling becomes arbitrary and the result reflects habit rather than intent.

Establish the ranking first:

- identify the primary element: the one thing the user most needs to see or do;
- identify secondary elements that support the primary;
- identify tertiary elements that are available but not essential;
- identify structural elements (navigation, chrome) that should recede.

Only after this ranking exists should visual treatment be applied. Styling without ranking produces screens where emphasis is distributed by feel rather than by purpose.

### Build Hierarchy From Multiple Cues, Not Size Alone

The most common hierarchy mistake is relying only on size. Bigger headings and smaller body create a ladder, but a ladder where every rung is the same weight and color is flat. Strong hierarchy combines several cues so that importance is unmistakable.

Combine hierarchy cues:

- size: larger elements read as more important;
- weight: bolder elements read as more important;
- color: darker or more saturated elements read as more important;
- position: elements higher or more central read as more important;
- spacing: more whitespace around an element signals its importance;
- contrast: elements that contrast more with their surroundings stand out.

A heading that is slightly larger, bolder, darker, and more isolated reads as far more important than one that is only larger. Multiple cues compound.

### Limit The Number Of Prominent Elements

Hierarchy works through contrast, and contrast requires restraint. If many elements are prominent, none are, because prominence is relative. A screen with six equally emphasized calls to action has no hierarchy. The discipline is deciding what does not get emphasis.

Restrict prominence:

- aim for one clearly dominant element per screen or section;
- allow a small number of secondary elements;
- relegate everything else to a calm baseline treatment;
- resist the pressure to make everything noticeable, which makes nothing noticeable.

Every request to "make this stand out more" should prompt the question: what should stand out less as a result? Emphasis is a zero-sum system.

### Differentiate Primary, Secondary, And Tertiary Actions

Calls to action are where hierarchy most often fails. When primary, secondary, and tertiary actions look similar, users do not know which path to take, and the most important action gets lost among options. A clear action hierarchy guides users to the right choice.

Style actions by rank:

- primary actions use the strongest treatment: filled, brand color, highest contrast;
- secondary actions use a medium treatment: outlined or lower-saturation fill;
- tertiary actions use the weakest treatment: text links or ghost buttons;
- destructive actions are visually distinct from primary actions, often using the error semantic color.

A screen with three filled buttons of equal weight has no primary action. The visual treatment should make the recommended path obvious.

### Use Spacing To Reinforce Importance

Spacing is a quiet but powerful hierarchy cue. More space around an element signals its importance; less space signals that it is part of a group. Agents often set spacing for alignment and forget its role in ranking, missing an opportunity to reinforce hierarchy without adding visual noise.

Use spacing for hierarchy:

- give the primary element generous surrounding whitespace;
- group secondary elements closer together to show they are peers;
- separate sections with larger gaps to indicate transitions;
- avoid equal spacing everywhere, which flattens perceived importance.

Spacing is especially valuable because it reinforces hierarchy without adding color or weight, keeping the layout calm.

### Let Content Density Match Importance

Not everything deserves the same visual real estate. Important content should occupy more space; minor content should be compact. A layout that gives equal space to critical and trivial information flattens hierarchy spatially.

Match density to importance:

- give primary content room to breathe and dominate;
- compress secondary and tertiary content so it is available but not competing;
- consider progressive disclosure: hide detail until needed so the initial view is clean;
- avoid giving minor options large controls or prominent placement.

A settings panel that gives the same prominence to a critical security toggle and a cosmetic preference has failed to rank by importance.

### Ensure Hierarchy Survives Real Content

Hierarchy designed with ideal placeholder content often collapses with real data. Headings that are clear at two words wrap awkwardly at ten. Lists that look scannable with three items become walls with twenty. A hierarchy must be tested against the content it will actually hold.

Test with real content:

- use realistic, variable-length content in designs, not lorem ipsum;
- test headings, names, and labels at their longest plausible length;
- check lists and tables at their densest;
- verify that truncation and wrapping do not destroy the hierarchy.

A hierarchy that only works with perfect content is fragile. Real content is messy, long, and unpredictable, and the hierarchy must hold.

### Maintain Consistent Hierarchy Across Screens

Hierarchy is a system property, not a per-screen decision. If headings are styled one way on one screen and another way elsewhere, users lose the ability to scan quickly because the cues are inconsistent. A consistent hierarchy system lets users learn the visual language once and apply it everywhere.

Enforce cross-screen consistency:

- define heading levels, action styles, and emphasis treatments as reusable tokens and components;
- apply the same hierarchy rules across all screens, with documented exceptions;
- review hierarchy consistency as part of design critique;
- update the system centrally so changes propagate rather than fragmenting.

Inconsistent hierarchy forces users to relearn the visual language on every screen, increasing cognitive load.

## Common Traps

### Styling Before Ranking

Applying visual treatment without first ranking elements by importance produces emphasis distributed by habit rather than intent.

### Relying On Size Alone

A heading ladder where every rung has the same weight and color is flat; combine size, weight, color, spacing, and contrast.

### Making Everything Prominent

When many elements are emphasized, none stand out; prominence is relative and requires restraint.

### Equal-Weight Calls To Action

Primary, secondary, and tertiary actions that look similar leave users unsure which path to take; style actions by rank.

### Ignoring Spacing As A Hierarchy Cue

Equal spacing everywhere flattens perceived importance; use generous whitespace around primary elements.

### Equal Space For Critical And Trivial Content

Giving minor options the same prominence as critical ones flattens hierarchy spatially; match density to importance.

### Hierarchy That Fails With Real Content

Designs tested only with ideal placeholders collapse with long, variable, real-world data; test with realistic content.

### Inconsistent Hierarchy Across Screens

When heading and emphasis treatments vary per screen, users cannot learn the visual language; enforce a consistent system.

## Self-Check

- [ ] Every element on the screen has been ranked by importance before visual treatment was applied.
- [ ] Hierarchy is built from multiple cues (size, weight, color, position, spacing, contrast), not size alone.
- [ ] The number of prominent elements is limited, with one clearly dominant element per screen or section.
- [ ] Primary, secondary, and tertiary actions are visually distinct, with destructive actions separated from primary ones.
- [ ] Spacing reinforces importance, with generous whitespace around primary elements and tighter grouping of peers.
- [ ] Content density matches importance, with primary content given room and minor content compressed or progressively disclosed.
- [ ] The hierarchy was tested against realistic, variable-length content, including longest plausible headings and densest lists.
- [ ] Hierarchy treatments are consistent across screens, defined as reusable tokens and components.
