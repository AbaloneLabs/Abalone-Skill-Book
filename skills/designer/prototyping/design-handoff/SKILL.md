---
name: design_handoff.md
description: Use when the agent is translating interactive prototypes into build-ready specifications, annotating prototype interactions and states for engineers, documenting component behavior and edge cases, defining responsive and interaction specs from a prototype, or ensuring the prototype's intent survives implementation without ambiguity or guesswork.
---

# Design Handoff

Design handoff is the translation of a prototype's intent into specifications that engineers can build without guessing. It looks like handing over a file, but it is really a communication problem: the prototype embodies decisions about structure, interaction, states, and edge cases that must be made explicit, or the build diverges from the intent. Agents tend to treat handoff as delivering the prototype, assume the interactions are self-evident, or leave edge cases undefined. The harm is a build that looks right in the happy case but behaves wrong in every other state, forcing weeks of clarification that the handoff should have prevented.

Use this skill before handing off a prototype or its specifications. The goal is to prevent the agent from shipping a prototype whose intent is ambiguous, from leaving states and edge cases undefined, or from treating handoff as file delivery rather than intent transfer.

## Core Rules

### Treat Handoff As Intent Transfer, Not File Delivery

The prototype is a representation of intent, not the intent itself. Engineers cannot read a prototype the way designers can; they need the decisions behind it made explicit. Treating handoff as delivering the prototype file leaves engineers to infer intent, and inference produces divergence. Handoff is the structured transfer of every decision the prototype embodies.

Transfer intent explicitly:

- document not just what the prototype shows but why each decision was made;
- provide the context (user tasks, constraints, research) that shaped the design, so engineers understand the purpose;
- make the prototype accessible alongside the specs, so engineers can experience the intent;
- be available to clarify, because no spec anticipates every question.

A prototype delivered without its intent is a riddle. The handoff's job is to make the intent unambiguous so the build matches the design.

### Specify Every State, Not Just The Happy Path

Prototypes usually demonstrate the happy path: the populated, successful, default state. But real interfaces have many states: empty, loading, error, partial, disabled, and edge cases with extreme content. If handoff specifies only the happy path, engineers build only the happy path, and every other state is improvised, usually wrong.

Specify all states:

- document the empty state (no data, first use) with its content and layout;
- document the loading state with its skeleton, spinner, or progressive reveal;
- document the error state with its messages, recovery actions, and layout;
- document edge cases: very long content, zero results, maximum density, unusual inputs.

A handoff that covers only the default state ships an interface that breaks the moment reality differs from the ideal. States are where most build divergence occurs, and they must be specified.

### Document Interaction Behavior, Not Just Appearance

A prototype shows what interactions look like, but engineers need to know how they behave: what triggers them, what rules govern them, what feedback results, and what happens on interruption. Appearance without behavior leaves engineers to guess the logic, and the guess is often wrong. Interaction specs must define the behavior, not just the visual.

Document interaction behavior:

- define the trigger for each interaction (click, hover, focus, gesture, threshold);
- define the rules: what is valid, what is rejected, what sequence follows;
- define the feedback: what communicates the state change to the user;
- define interruption and reversal: what happens if the user backs out or the context changes.

A button that looks right but whose disabled, loading, and error behaviors are unspecified will be built wrong. Behavior, not appearance, is what makes an interaction work.

### Define Responsive Behavior Across Breakpoints

A prototype often shows one viewport, but the build must work across all of them. If handoff does not define how the layout adapts across breakpoints, engineers improvise the adaptation, producing inconsistent responsive behavior. The responsive rules must be part of the handoff.

Specify responsive behavior:

- define how the layout reflows at each breakpoint (mobile, tablet, desktop);
- specify what changes: column counts, component arrangements, navigation patterns, spacing;
- address ultrawide and narrow edge cases, not just common widths;
- define which behaviors are consistent across breakpoints and which adapt.

A handoff with one viewport specified leaves the responsive implementation to chance. Responsive rules are part of the design and must be handed off.

### Annotate The Prototype Directly

Specifications separated from the prototype force engineers to cross-reference two sources, and they drift apart. Annotations placed directly on the prototype, linked to the elements they describe, keep the spec and the design connected and reduce ambiguity. Direct annotation is more usable than a separate spec document.

Annotate in context:

- place annotations on the specific elements and states they describe;
- link related annotations (a button's states, a form's validation) so they are found together;
- use the prototype's own frames to show each state with its spec;
- keep annotations concise and scannable, with detail available on demand.

A spec document that describes the prototype in prose, separate from the visuals, is harder to use and more likely to be ignored. Annotation in context is more accurate and more used.

### Include Accessibility Specifications

Accessibility is often an afterthought in handoff, left for engineers to infer, which means it is usually incomplete. The prototype's accessibility requirements (roles, names, focus order, contrast, keyboard behavior) must be specified explicitly, or the build will not meet them. Accessibility specs are part of the handoff, not optional.

Specify accessibility:

- define the semantic structure: landmarks, headings, and roles for each element;
- specify accessible names for icon-only controls and complex widgets;
- define focus order and focus management for dynamic interactions;
- document keyboard behavior and any established widget patterns to follow.

An accessible prototype handed off without accessibility specs becomes an inaccessible build. The requirements must travel with the handoff.

### Provide Component And Token References

Designs are built from components and tokens that already exist in the system, and the handoff should reference them rather than re-specify them. If the handoff treats every element as custom, engineers miss opportunities to reuse, and the build diverges from the system. Mapping the design to existing components and tokens keeps the build consistent and efficient.

Reference the system:

- map design elements to existing components and variants in the design system;
- reference design tokens (color, spacing, type) by name, not raw values;
- flag where the design requires new components or token additions, with rationale;
- ensure the references are current to the system version in use.

A handoff that re-specifies a standard button as a custom element causes engineers to build a duplicate. Mapping to the system preserves consistency and reduces effort.

### Define What Is Final Versus Open

Not everything in a prototype is decided at handoff. Some elements are final; others are exploratory or open questions. If the handoff does not distinguish these, engineers may build open elements as final, or treat final elements as changeable. The handoff must communicate what is firm and what remains to be resolved.

Mark status clearly:

- label which elements and decisions are final and ready to build;
- flag open questions and edge cases that need resolution before or during build;
- communicate dependencies (waiting on content, research, or a system update);
- set expectations for what may change, so engineers do not over-invest in unstable areas.

A handoff that presents everything as final when some is open causes wasted engineering effort on elements that change. Clarity about status keeps the build focused on what is stable.

### Plan For Collaboration, Not One-Way Delivery

Handoff is not a one-way handoff of a finished artifact; it is the start of a collaboration between design and engineering. Questions will arise, edge cases will surface, and the build will reveal problems the prototype hid. Treating handoff as delivery-and-done leaves these to be resolved poorly; treating it as ongoing collaboration resolves them well.

Collaborate through the build:

- be available to answer questions as engineers implement;
- review the build against the intent early and often, not only at the end;
- treat build-revealed problems as design problems to solve together, not engineering failures;
- document decisions made during the build so the spec stays current.

A handoff treated as final delivery breaks down when reality diverges from the prototype. Collaboration is what keeps the built product faithful to the design intent.

## Common Traps

### Handoff As File Delivery

A prototype without its documented intent leaves engineers to infer decisions; transfer intent explicitly, not just files.

### Specifying Only The Happy Path

Interfaces have empty, loading, error, and edge-case states; specify all of them or the build improvises them wrong.

### Appearance Without Behavior

Interactions that look right but lack trigger, rule, and feedback specs get built with wrong logic; document behavior.

### Undefined Responsive Behavior

A single-viewport handoff leaves responsive adaptation to chance; define how the layout adapts across breakpoints.

### Specs Separated From The Prototype

A separate spec document drifts from the design and is ignored; annotate the prototype directly in context.

### Accessibility As Afterthought

Accessibility requirements left for engineers to infer are usually incomplete; specify roles, names, focus, and keyboard behavior.

### Re-Specifying Existing Components

Treating standard elements as custom causes duplicate builds; map the design to existing components and tokens.

### One-Way Delivery Without Collaboration

Build-revealed problems go unresolved when handoff is delivery-and-done; collaborate through the build and keep the spec current.

## Self-Check

- [ ] Handoff transfers the design's intent and context, not just the prototype file, with the designer available to clarify.
- [ ] Every state (empty, loading, error, partial, disabled, edge cases) is specified, not only the happy path.
- [ ] Interaction behavior (trigger, rules, feedback, interruption, reversal) is documented, not just appearance.
- [ ] Responsive behavior across breakpoints (reflow, changes, edge widths) is defined, not left to engineer improvisation.
- [ ] Specifications are annotated directly on the prototype in context, not separated into a drifting document.
- [ ] Accessibility specifications (roles, names, focus order, keyboard behavior, widget patterns) are included in the handoff.
- [ ] Design elements are mapped to existing components and tokens, with new additions flagged with rationale.
- [ ] Final versus open elements are clearly marked, and the handoff is treated as ongoing collaboration through the build.
