---
name: component_composition_and_reuse.md
description: Use when the agent is designing a reusable component or hook API, deciding between composition and inheritance, choosing controlled versus uncontrolled component interfaces, extracting a custom hook, building a headless component or separating logic from presentation, designing a props API including naming, defaults, variants, and compound components, weighing a polymorphic as prop, deciding whether to abstract shared code or leave duplication, or applying the open-closed principle to a component so it can be extended without editing it. Also covers avoiding boolean-prop explosion, the prop-bag anti-pattern, the three-use rule for abstraction, design-system token consumption, and when duplication beats a premature abstraction. This skill is the API and composition craft layer and complements the state-placement and boundary concerns covered in component-architecture-and-state.
---

# Component Composition And Reuse

Once a component's boundaries and state placement are settled (the subject of component-architecture-and-state), a second set of questions remains: *what is this component's API, and how does it compose with the things around it?* This is the craft layer of frontend work. A well-designed component is a small, stable contract — a clear set of props, predictable defaults, and an obvious way to extend — that other engineers can use correctly without reading its source. A poorly-designed one is a tax that grows over time: a Button that sprouted fourteen boolean flags, a Modal whose every new use case required a new prop, a "reusable" form field so abstract that no two call sites use it the same way, a hook extracted so early that its signature changed three times and broke every consumer.

Agents tend to err in two directions here. The first is over-abstraction: extracting a shared component or hook after one or two uses, locking in an API that fits only the first cases, then watching every later requirement arrive as another conditional prop until the abstraction is harder to use than the duplication it replaced. The second is under-craft: shipping a component whose API is accidental — props named inconsistently, no sensible defaults, behavior toggled by booleans that interact in confusing ways, no path to extend without forking. The judgment problem is to design APIs that are small and honest about what they do, to prefer composition over configuration, and to time abstraction so that it lands when the shared shape is real rather than imagined.

This skill assumes component boundaries and state placement are already sound. It focuses on the API surface and composition mechanics: composition over inheritance, controlled versus uncontrolled API design, headless components and custom hooks, props and variant API design, the discipline of when to abstract, and how to keep components open for extension.

## Core Rules

### Prefer Composition Over Inheritance For UI

Inheritance ("a PrimaryButton extends Button, a IconButton extends PrimaryButton") fails for UI because real components vary along several independent axes — visual variant, size, element type, content, behavior — and a class hierarchy can only specialize along one axis at a time. The result is a deep tree where the combination you need (a small, primary, icon-leading button that renders as a link) does not exist and cannot be added without disturbing the hierarchy. Composition solves this by combining small, single-purpose pieces.

- **Compose with children, slots, and render props.** A `Card` takes a header slot, a body, and a footer; a `Dialog` takes its content as children. The component orchestrates layout without knowing what is inside it.
- **Compose behavior with hooks rather than inheriting it.** Shared logic lives in a hook (`useDialog`, `useCombobox`) that any component can call, rather than in a base class a component must extend.
- **Specialize by configuration and composition, not by subclassing.** A `PrimaryButton` is a `Button` with `variant="primary"`, not a subclass. This keeps the variation explicit and composable with other variations.

The test: if you are about to write `extends`, ask whether the variation is better expressed as a prop value, a composed child, or a hook. Almost always, in UI, it is.

### Design The Controlled Versus Uncontrolled API Deliberately

A reusable input-style component (field, select, toggle, date picker) must declare whether it is controlled (the caller owns the value and drives it via props) or uncontrolled (the component owns the value and notifies via callbacks, with an optional initial value), and the choice shapes the whole API. See component-architecture-and-state for the per-input decision of which model to use; here the concern is the *API contract* a reusable component exposes.

- **A controlled component takes `value` and `onChange` and is fully driven.** It has no internal state for its value; it reflects exactly what it is given. Predictable and testable, but the caller must manage every change.
- **An uncontrolled component takes `defaultValue` and emits `onChange` for observers, but owns the value internally.** Simpler for callers who do not need to drive every keystroke, and it can be read via a ref on submit.
- **Support both when the component is broadly reusable, with clear rules.** Many design-system components accept `value` (controlled) or `defaultValue` (uncontrolled) and throw if both are passed. Document the rule and enforce it; do not silently pick one.
- **Do not mix the models within one component's public API.** A component that is "controlled for the value but uncontrolled for the open/closed state" surprises callers. Be consistent, or split the concerns into separate components.

The API contract must be unambiguous about which model applies, what the callback signatures are, and what happens on edge cases (reset, programmatic value change, controlled-to-uncontrolled switches). Vague contracts produce integration bugs that are hard to attribute.

### Separate Logic From Presentation With Headless Components And Hooks

When behavior is reusable but presentation is not, the right unit is a headless component or custom hook that encapsulates the logic and exposes state and handlers, leaving rendering to the consumer. This is the cleanest separation of concerns in modern UI: the logic is tested once and reused, while each consumer renders it however it needs.

- **A headless component or hook owns behavior, not markup.** `useCombobox` returns the open state, the highlighted index, the handlers, and the ARIA props; the consumer wires them to whatever DOM it wants. The logic is reused; the look is not constrained.
- **Headless patterns are strongest for accessibility-heavy behavior.** Comboboxes, menus, dialogs, tabs, and date pickers have intricate keyboard and ARIA behavior that is hard to get right; a headless hook that handles it correctly, once, is high-leverage.
- **Keep the hook's return shape stable and documented.** The return value is the API. Whether it returns a tuple or an object, name it consistently, document each field, and avoid changing the shape — every consumer depends on it.

The boundary: extract a headless hook when the *logic* is genuinely shared and non-trivial. Do not extract a hook for logic used in one place, or for logic so simple that the hook adds indirection without reuse.

### Design The Props API Small, Named, And With Sensible Defaults

A component's props are its contract, and a good contract is small, consistently named, and usable without reading the source. Most prop-API pain comes from accidental design — props added one at a time under deadline until the API is incoherent.

- **Name props consistently with the platform and the ecosystem.** `value`/`onChange`, `disabled`, `aria-*`, `children` — match the conventions callers already know. A component that invents `onValueUpdated` where the ecosystem expects `onChange` forces callers to remember a special case.
- **Provide sensible defaults so the common case needs no props.** A `Button` with no props should render a reasonable default button. Every required prop is friction; require only what is genuinely required.
- **Spread through to the underlying element.** Accepting and forwarding `...rest` to the root element lets callers set `aria-*`, `data-*`, `id`, and event handlers without the component anticipating each one. This is how a component stays open without growing a prop per attribute.
- **Avoid boolean-prop explosion.** When a component grows `primary`, `secondary`, `outline`, `ghost`, `large`, `small`, `compact` as separate booleans, the combinations become untestable and contradictory (`primary` + `ghost`?). Prefer a single enumerated `variant` and a single `size` prop whose values are mutually exclusive.

The discipline is to treat the props list as a designed surface, not an accumulation. Audit it; consolidate contradictory booleans into enumerations; forward the long tail through `...rest` instead of enumerating it.

### Use Compound Components And Slots For Composable Structure

When a component has a fixed internal structure whose parts callers must configure independently (a `Dialog` with `Header`, `Body`, `Footer`, or a `Tabs` with `TabList` and `TabPanel`), compound components are usually the cleanest API: the parent provides context, and the children declare their role.

- **Compound components share context implicitly.** `Dialog.Header` does not take a `dialogId` prop; it reads it from context. The caller composes the parts without wiring them together.
- **Slots make the structure explicit and enforceable.** Where compound components are too loose, named slots (or a `slots` prop) let the parent declare which children fill which roles and reject misplaced content.
- **Resist the prop-bag anti-pattern.** The alternative — one giant component taking `headerText`, `headerIcon`, `headerAction`, `bodyText`, `footerPrimaryLabel`, `footerPrimaryOnClick`, and so on — scales badly and forces the component to understand every region. Prefer composition of parts over a bag of region-specific props.

Compound components and slots keep the parent's API small (it takes children) while giving callers full control of each region. They are the standard answer to "this component needs to be flexible about its content."

### Extract Custom Hooks By The Rules Of Reuse And Complexity

Custom hooks are the primary unit of reusable logic, but extracting them carelessly creates a forest of tiny hooks that obscure more than they clarify. Apply explicit rules.

- **Extract when logic is reused AND non-trivial.** A hook for logic used once, or for logic that is two lines, adds indirection without benefit. Extract when the same non-trivial logic is needed in a second place.
- **Name with the `use*` convention and return a stable shape.** `useFetch`, `useDebounce`, `useForm`. Return a tuple when there are few well-known values (like `useState`), an object when there are many named fields. Pick one and document it.
- **Honor the rules of hooks in the extracted API.** The hook's call order must be stable; its dependency arrays must be correct and minimal; it must not hide conditional hook calls behind its own conditionals. A hook that breaks the rules of hooks corrupts every consumer.
- **Keep dependencies honest.** A hook that captures stale closures or omits a dependency produces subtle bugs that surface far from the hook. Treat the dependency array as part of the contract.

The three-use guidance from abstraction discipline (below) applies to hooks too: tolerate duplication through the second use, extract confidently at the third, and expect to revise the API.

### Time Abstraction By The Three-Use Rule, And Prefer Duplication When Premature

The single most important judgment in reuse is *when to abstract.* Abstract too early and you lock in an API that fits only the first cases; abstract too late and you live with duplication. The three-use rule is a robust heuristic.

- **First use: write the code inline.** Do not extract. You do not yet know the shape.
- **Second use: tolerate the duplication.** Copy the code rather than extracting. The second case often reveals that the "shared" shape is actually different in an important way; extracting now bakes in the first case's assumptions.
- **Third use: extract, and expect to revise.** Three uses confirm a real shared shape. Extract it, and accept that the first extraction may need a second pass as later uses stress it.

The reason duplication is sometimes better than the wrong abstraction is that duplication is easy to refactor later, while a wrong abstraction is hard to remove once consumers depend on it. When in doubt, leave the duplication and a note; extract when the pattern is confirmed.

### Keep Components Open For Extension Via Composition, Slots, And Children

A component is well-designed when a new requirement can be met by composing, slotting, or passing children — without editing the component. This is the open-closed principle applied to UI: open for extension, closed for modification.

- **Accept `children` and render it.** The most extensible component is one that renders what it is given. A `Card` that renders its children is extensible forever; a `Card` that hardcodes its body is not.
- **Expose extension points as slots or render props.** When a region needs customizable rendering, accept a render prop or a slot rather than adding a boolean to toggle a hardcoded alternative.
- **Resist configuration flags that encode specific use cases.** A `showAdminBadge` prop means the component now knows about admin badges; the next use case wants a different badge and adds another flag. Prefer a slot the caller fills.
- **Forward attributes through `...rest`.** Callers can add `aria-*`, `data-*`, classes, and handlers without the component anticipating them.

The test of openness: can the next reasonable requirement be met by the caller, without a pull request to the component? If every new requirement needs a new prop on the component, the API is closed where it should be open.

### Consume Design-System Tokens And Expose A Coherent Variant API

When building on a design system, the application component's job is to consume tokens and expose a variant API that matches the system's vocabulary, not to re-implement styling.

- **Consume design tokens (color, spacing, typography) rather than hardcoding values.** A component that hardcodes a hex code breaks theming, dark mode, and brand swaps.
- **Expose variants that match the design system's vocabulary.** If the system defines `variant` as `primary | secondary | ghost`, the application component should use the same names, not invent `type="main"`.
- **Be cautious with the polymorphic `as` prop.** Letting a component render as any element (`as="a"`, `as="button"`) is powerful but trades type safety and predictability for flexibility; the prop's typing and the forwarded attributes become complex. Use it when the polymorphism is genuinely needed, and constrain it where possible.

## Common Traps

### Inheritance Hierarchy For UI Variations

Building `PrimaryButton extends Button extends BaseButton` and discovering the combination of variations you need does not exist. Compose with props and hooks instead of subclassing.

### Boolean-Prop Explosion

Adding `primary`, `outline`, `ghost`, `large`, `small` as separate booleans until contradictory combinations are possible and untestable. Consolidate into enumerated `variant` and `size` props.

### The Prop-Bag Component

One component taking `headerText`, `headerIcon`, `bodyContent`, `footerLabel`, `footerOnClick`, and a dozen more region-specific props, so it must understand every region. Split into compound components or slots.

### Extracting A Hook Or Component After One Use

Abstracting shared code before the shared shape is confirmed, then revising the API for every later use and breaking early consumers. Apply the three-use rule; tolerate duplication through the second use.

### Mixing Controlled And Uncontrolled In One Public API

A component that is controlled for one piece of state and uncontrolled for another, with no documented rule, surprising callers. Be consistent or split the concerns.

### A Headless Hook For Trivial Or Single-Use Logic

Extracting `useToggle` for a one-line boolean, adding indirection without reuse. Extract hooks only when the logic is reused and non-trivial.

### Configuration Flags Encoding Specific Use Cases

Adding `showAdminBadge` or `isCheckoutFlow` props so the component knows about specific features, guaranteeing the next feature needs another flag. Use slots or children so callers extend without editing the component.

### Hardcoding Values Instead Of Consuming Tokens and unconstrained Polymorphic `as` Prop

Hardcoding colors, spacing, or font sizes in an application component, breaking theming and dark mode. Consume design tokens.

Exposing `as={any}` for flexibility and losing type safety and predictable attribute forwarding. Constrain the allowed elements and type the forwarded attributes.

## Self-Check

- [ ] The component varies by composition (children, slots, render props, hooks) rather than by inheritance, and no class hierarchy exists where a prop value or composed child would express the variation better.
- [ ] Each reusable input-style component declares a controlled and/or uncontrolled API unambiguously — `value`/`onChange` for controlled, `defaultValue` for uncontrolled, with a documented and enforced rule when both are accepted, and no component mixes the two models within one public API.
- [ ] Reusable non-trivial logic is extracted as a headless component or custom hook that owns behavior and exposes a stable, documented return shape, while trivial or single-use logic is left inline rather than extracted into indirection.
- [ ] The props API is small and intentionally designed — props are named consistently with the platform and ecosystem, sensible defaults mean the common case needs no props, and `...rest` is forwarded to the root element so callers can set attributes without the component anticipating each one.
- [ ] Contradictory or combinatorial booleans were consolidated into enumerated `variant` and `size` props, and no component has a forest of boolean flags whose combinations are untestable.
- [ ] Structured components use compound components or named slots rather than a prop-bag of region-specific props, so the parent takes children/parts and does not need to understand every region.
- [ ] Abstraction was timed by the three-use rule — code was left inline or duplicated through the first two uses and extracted only at the third confirmed use — and no shared abstraction was created prematurely and then forced to absorb every later requirement.
- [ ] The component is open for extension without editing it: new requirements are met by children, slots, render props, or forwarded attributes, not by adding configuration flags that encode specific use cases.
- [ ] The component consumes design tokens (color, spacing, typography) rather than hardcoding values, exposes a variant API matching the design system's vocabulary, and any polymorphic `as` prop is constrained and correctly typed rather than unconstrained.
