---
name: component_architecture_and_state.md
description: Use when the agent is building or refactoring a UI, splitting a screen into components, deciding where state lives (local vs global, client vs server), choosing a state management library, dealing with prop drilling or lifting state up, deciding controlled vs uncontrolled inputs, placing data-fetching logic, aligning components with a design system, diagnosing unnecessary re-renders, or reviewing whether a component boundary is well-placed. Also covers single-responsibility components, presentational vs container split, server state vs client state, immutability and render performance, composition over configuration, and the cost of premature or over-broad global state. Use when designing a new feature's component tree, when a component has grown unmanageable, when state is scattered or duplicated, or when a UI change touches too many files.
---

# Component Architecture And State

A component tree and its state are two halves of the same decision: *what does each piece of UI own, and where does the data it shows come from?* These look like separate questions but they constrain each other. Where you put state determines which components must re-render and which stay stable; where you draw component boundaries determines how far data must travel and how much each piece can be reused. Get either wrong and the cost is concrete: a component that re-renders the whole tree on every keystroke, a form whose state is split across three stores that disagree, a "reusable" button that takes twenty props because it knows about the page around it, or a feature that cannot be changed without editing fifteen files because the boundaries followed the wrong axis.

Agents tend to under-invest in these decisions because the harm is delayed and the immediate reward is high. Lifting all state to a global store feels clean and makes the first feature fast; the cost shows up months later when every component depends on a giant store, a change to one slice re-renders unrelated screens, and the store has become a coupling point nothing can be tested in isolation. Splitting a component "for reuse" before the reuse is real produces props-threaded abstractions that are harder to change than the original. The judgment problem is not "how do I build this screen" but two questions held together: *what is the right boundary for each piece of UI, and what is the right home for each piece of state — and which answers will still be right when the feature grows?*

## Core Rules

### Split Components By Responsibility And Change-Axis, Not By Visual Size

A component is a unit of "things that change together and can be reasoned about together." The common failure is splitting by visual appearance (a header, a card, a row) or by line count, which produces components that look clean but couple unrelated concerns. The better axis is responsibility: what does this piece do, what does it know, and what would change independently?

- **Single responsibility per component.** A component should do one thing — render a thing, manage a form, fetch some data, orchestrate a layout. A component that both fetches data and renders detailed UI and manages interaction state is doing three jobs; when any one changes, the others are dragged along.
- **Split where the rate or reason for change differs.** If the data-fetching logic changes on a different cadence than the visual treatment, they want to be separate. If two parts of a screen change for the same business reason, splitting them apart just spreads one change across files.
- **Extract when reuse is real, not anticipated.** Duplicate a component twice before extracting a shared one; extract when a third use confirms the shared shape, and accept that the first extraction may need revision. Premature extraction bakes in an abstraction that fits only the first two cases and constrains the third.

Signs a boundary is wrong: a component takes a large props bag where most props are passed straight through (it is a pass-through, not an owner); a component is reused in only one place; a change to one feature forces edits across many small components that all participate in it.

### Put State At The Lowest Level That Can Own It, And No Lower

The default home for state is the component that uses it, because local state is the cheapest: it re-renders only that component, it is trivially testable, and it dies when the component unmounts. State should leave its local home only when there is a real reason — never "for consistency" or "in case we need it elsewhere." For each piece of state, ask why it cannot stay local:

- **Shared between siblings.** If two sibling components need the same state, lift it to their nearest common ancestor — not to a global store. The common ancestor is the smallest scope that satisfies the sharing requirement.
- **Needs to persist beyond the component's life.** If the state must survive unmount/remount (a draft, a scroll position, a wizard step), it may need to live higher or in a store/router — but only if the persistence is actually required.
- **Truly global.** Auth/user identity, theme, feature flags, and locale are genuinely app-wide and belong in a global context. Most "global" state is not; it is feature-scoped state that someone lifted out of convenience.

The strong default is local state; lifting is a deliberate act with a stated reason. A codebase where every piece of state lives in a global store has paid the cost of global state (coupling, re-render scope, test setup) for state that needed none of it.

### Separate Server State From Client State — They Are Not The Same Thing

The most common conceptual error is treating data fetched from the server like any other client state and stuffing it into the same store. Server state has fundamentally different properties: it is asynchronous, it can be stale, it is owned by the server (not the client), it needs caching and revalidation, and multiple components often need the same query. A general-purpose state store is the wrong tool for it.

- **Use a data-fetching/server-state library** (React Query, SWR, Apollo, RTK Query, or equivalent) for remote data. These handle caching, deduplication of concurrent requests, background revalidation, loading/error states, and invalidation — concerns a plain store forces you to rebuild by hand.
- **Reserve the general store for true client state** — UI state, drafts, toggles, locally-computed derived values — that the server does not own and that does not have a staleness lifecycle.
- **Do not duplicate server data into client state.** Copying a fetched user into a global store "so other components can read it" creates two sources of truth that drift. Let the server-state cache be the single source; components read from it.

Conflating the two produces the classic bugs: stale data shown after an update because the client copy was not refreshed, redundant refetches because nothing deduplicates, and "the data changed but the UI didn't" because the mutation did not invalidate the right cache.

### Choose A State Library By The Problem, Not By Hype Or Default

The choice of state management should follow from the shape of the state and the rendering needs, not from what is popular. Different tools optimize for different things:

- **`useState` / component-local.** Best for ephemeral, component-scoped state. No library needed; reach for nothing else until this is insufficient.
- **Context / scoped providers.** Best for state shared across a subtree that changes rarely relative to reads (theme, auth, locale). Beware: a context value change re-renders every consumer, so context is poor for high-frequency state.
- **Atomic / fine-grained stores (Zustand, Jotai, signals).** Best when many components subscribe to independent slices and you want updates scoped to only the subscribers of the changed slice. Avoids the "change one field, re-render the world" problem of coarse stores.
- **Redux-style stores.** Best for complex, interrelated client state with a strong need for time-travel debugging, middleware, or a single serializable event log. Heavyweight; justified only when the complexity warrants the ceremony.

The decision criterion: how many components read this state, how often does it change, and how much of the tree must re-render on a change. Match the tool to those answers. Reaching for Redux on a form, or for a global store on a local toggle, is overhead with no payoff.

### Treat Props As A Contract And Resist Prop Drilling By Fixing The Boundary

Prop drilling — passing data through many intermediate components that do not use it — is a symptom, not a disease. It signals that the state lives too high relative to where it is used, or that intermediate components are doing layout when they should be passing children. Two valid responses, chosen by the cause:

- **Move the state down** to the component that actually owns the interaction, so the data stops needing to pass through unrelated layers. Often the real fix.
- **Use composition (children / render props / slots)** so intermediate layout components never see the data — they render the children they are given. This eliminates drilling without introducing a global dependency.

The wrong fix is reflexively reaching for a global store or context to "avoid prop drilling." That trades a local, traceable data flow for an invisible global dependency that is harder to test and reason about. Use context when the data is genuinely needed across a wide subtree and changes infrequently; do not use it as a plumbing shortcut. A component whose only job is to thread props to a deep child is a sign the boundary is wrong.

### Prefer Immutability And Memoize Only When Measured

Most modern UI frameworks use reference equality to decide what changed, so mutating state in place breaks change detection (the framework sees the same reference and skips the update), and unnecessary new references cause re-renders that are wasted. The discipline:

- **Always update state immutably** (return a new object/array, never mutate). Mutation in place is not just a style issue — it produces UI that does not update because the framework cannot detect the change.
- **Do not memoize prematurely.** Memoizing every component and every value adds overhead and complexity for re-renders that may be cheap. Profile first; memoize the specific components whose re-render is measurably expensive (large lists, expensive computations), and keep the memoization honest (stable references for the memoized inputs).
- **Keep derived state computed, not stored.** If a value can be computed from existing state, compute it during render rather than storing a synchronized copy. Stored derived state is a second source of truth that drifts when the source changes and the sync is missed.

### Decide Controlled Vs Uncontrolled By Where The Source Of Truth Must Live

An input (form field, toggle, slider) is either controlled (its value is driven by React/state, and changes flow up) or uncontrolled (the DOM holds the value, read via ref). The choice is about where truth lives:

- **Controlled** when the value must drive other UI (validation messages, conditional fields, submit-button enablement), when it must be transformed on input, or when it is part of submitted state. The component is the source of truth.
- **Uncontrolled** when the field is write-only until submit, when you want to avoid re-rendering on every keystroke, or when integrating non-React code. The DOM is the source of truth; you read it when needed.

The common error is mixing the two on the same input (providing a `value` but also reading from a ref), which produces inputs that fight their own state. Pick one model per input and be consistent. For complex forms, a form library usually resolves this for you — but understand which model it uses, because it determines how validation, default values, and resets behave.

### Keep Data-Fetching At The Boundary, Not Deep In The Tree

Where data is fetched shapes the whole component tree. Fetching deep inside leaf components produces waterfalls (component renders, then fetches, then a child renders, then fetches again — serial round trips) and duplicated requests (two components fetching the same data independently). The strong patterns:

- **Fetch at route or page boundaries**, so the data for a view is requested together and can be parallelized, and so the loading/error states belong to the view rather than scattered across leaves.
- **Co-locate the query with the component that owns the data's lifecycle**, but let a server-state cache deduplicate so multiple consumers share one fetch.
- **Avoid fetch-on-render waterfalls.** If a parent must know a child's data to render, that data belongs to the parent. If two pieces of data are independent, fetch them in parallel, not sequentially.

### Align Components With The Design System, And Let Tokens Own Consistency

A component library or design system is the shared vocabulary of UI. Aligning application components to it (using its primitives, its spacing/color tokens, its variants) is what keeps a product visually consistent and lets design changes propagate. The judgment:

- **Compose design-system primitives rather than restyling them per page.** If every screen re-implements a button, the design system provides no leverage and consistency drifts.
- **Let design tokens (spacing, color, typography scales) be the source of consistency**, not hardcoded values. A component that hardcodes `#3B82F6` breaks theming and dark mode.
- **Distinguish application components from design-system components.** Design-system components are generic and stable (a Button, a Modal); application components are specific and volatile (a CheckoutForm). Do not push volatile application logic into the design system, and do not fork the design system for one-off needs.

## Common Traps

### Lifting Everything To A Global Store On Day One

Storing all state globally because it "might be shared later." Every component depends on the store, every change re-renders broadly, and the store becomes a coupling point that is hard to test or refactor. Start local; lift only with a stated reason.

### Stuffing Server Data Into The Client Store

Copying fetched data into Redux/Zustand "so components can read it." The data goes stale, mutations don't invalidate it, and refetches aren't deduplicated. Use a server-state library; keep the client store for true client state.

### Storing Derived State And Letting It Drift

Keeping a `fullName` in state that is computed from `firstName`/`lastName`, then forgetting to update it when the source changes. Compute derived values during render; do not store a synchronized copy.

### Prop Drilling "Fixed" With A Global Context

Threaded props through five layers, so the engineer wraps the app in a Context to skip the threading. The context now re-renders every consumer on every change, and the data flow is invisible. Fix the boundary (move state down, use composition) before reaching for context.

### Splitting Components For Reuse That Never Comes

Extracting a `UserCard` shared component after one use, then discovering the second use needs different fields, actions, and layout — so the component grows a forest of conditional props. Duplicate first; extract when reuse is confirmed.

### Memoizing Everything To "Optimize"

Wrapping every component in `memo` and every value in `useMemo` without measuring. The overhead of memoization can exceed the cost of the re-renders it prevents, and it adds a maintenance burden (every input must be a stable reference or the memo is useless). Profile, then memoize the expensive cases.

### Mutating State In Place Because It "Looks Simpler" and a Giant Component That Fetches, Validates, And Renders

Doing `state.items.push(newItem)` instead of returning a new array. The framework sees the same reference and does not re-render, producing a UI that silently fails to update. Always update immutably.

One component handles the API call, the form state, the validation, and the rendering. Every change — visual, data, or logic — touches it, and it cannot be tested in pieces. Split by responsibility: a data/container layer, a form/state layer, and presentational children.

### Fetching In Leaf Components Causing Waterfalls and mixing Controlled And Uncontrolled On The Same Input

A parent renders, a child renders and fetches, then a grandchild renders and fetches — three serial round trips for data that could have been fetched in parallel at the page level. Fetch at the boundary; parallelize independent queries.

Providing a `value` prop and also reading the input via ref, so the input's displayed value and its tracked state disagree. Pick one model per input and commit to it.

## Self-Check

- [ ] Each component has a single responsibility and was split along a change/reuse axis, not by visual size or line count — no component exists as a pure pass-through threading props it does not use, and no shared component was extracted before reuse was confirmed.
- [ ] State lives at the lowest level that can own it: ephemeral state is local, shared sibling state is lifted only to the nearest common ancestor, and only genuinely app-wide state (auth, theme, locale, flags) is global — every lift to a store or context has a stated reason rather than being done "for consistency."
- [ ] Server state is separated from client state: remote data is managed by a data-fetching/server-state library that handles caching, deduplication, revalidation, and invalidation, and server data is not duplicated into a client store that would drift.
- [ ] The state library was chosen by the shape of the problem (number of readers, change frequency, re-render scope), not by default or hype — local state where local suffices, fine-grained stores for independent slices, context only for infrequently-changing subtree-shared data.
- [ ] Prop drilling was resolved by fixing the boundary (moving state down or using composition/children) rather than by reflexively introducing a global store or context; data flow remains traceable rather than invisibly global.
- [ ] State updates are always immutable (new references, never in-place mutation), derived values are computed during render rather than stored as synchronized copies, and memoization is applied only to components/values proven expensive by measurement — not blanket-applied.
- [ ] Each input is consistently controlled or uncontrolled based on where the source of truth must live, with no input mixing both models, and the choice reflects whether the value must drive other UI or is write-only until submit.
- [ ] Data fetching lives at route/page boundaries (parallelized, with loading/error states owned by the view) rather than in leaf components causing serial waterfalls or duplicated requests.
- [ ] Application components compose design-system primitives and use design tokens (spacing, color, typography) as the source of consistency rather than hardcoding values, and volatile application logic is not pushed into the stable design-system layer.
