---
name: javascript_immutability_and_deep_copy.md
description: Use when the agent is reasoning about value vs reference semantics in JavaScript, avoiding accidental mutation of shared objects and arrays, choosing between shallow copy (spread, Object.assign, Array.from) and deep copy (structuredClone, JSON round-trip, libraries), designing immutable update patterns for state (React/Redux), comparing deep equality, freezing objects, or debugging bugs where changing one object affects another. Covers reference semantics, shallow vs deep copy tradeoffs, immutable state updates, structuredClone vs JSON pitfalls, and the tradeoff between immutability safety and performance/ergonomics.
---

# Immutability And Deep Copy

JavaScript has value semantics for primitives (string, number, boolean) and reference semantics for objects and arrays. Assigning an object or passing it to a function shares the reference, so mutating it through one alias affects every other alias. The judgment problem is knowing when shared mutable references will bite, choosing the right copy depth, and updating state immutably so that change detection and reasoning about correctness actually work.

Agents tend to mutate shared objects in place (causing bugs where one change affects unrelated code), use shallow copy where deep copy is needed, use JSON round-trip and silently drop functions/dates/undefined, or rebuild large state trees on every update for "immutability" without measuring. The harm appears as cross-component state corruption, React components that do not re-render because the reference did not change, deep copies that lose data or break on circular references, and performance problems from copying huge structures. The real work is recognizing reference sharing, copying at the right depth, and updating immutably with minimal structural change.

## Core Rules

### Understand Value vs Reference Semantics, And Where Sharing Bites

Primitives are copied on assignment: `let b = a` for numbers gives `b` an independent value. Objects and arrays are shared by reference: `let b = a` for an object makes `b` point at the same object; mutating `b.x` changes `a.x`.

- Function arguments share references too: `function f(obj) { obj.x = 1; }` mutates the caller's object.
- Nested sharing: `const a = { items: [1, 2] }; const b = { items: a.items };` — `b.items` is the same array as `a.items`; pushing to one appears in both.
- Arrays of objects share the element objects: copying the array gives a new array whose elements are the same object references.

The first question when handing an object to code that might mutate it is: "do I need to defend against mutation?" If yes, copy. If the data is private and trusted, sharing may be fine.

### Choose Shallow Copy When One Level Suffices

A shallow copy duplicates the top level but shares nested references. Use it when you only mutate top-level fields.

- **Spread**: `{ ...obj }` (object) or `[...arr]` (array) creates a one-level copy.
- **`Object.assign({}, obj)`**: equivalent to spread for objects.
- **`Array.from(arr)` / `arr.slice()`**: one-level array copies.
- After a shallow copy, mutating a top-level field of the copy does not affect the original, but mutating a nested object does (because it is shared).

Shallow copy is fast and idiomatic for immutable updates where you replace a nested value entirely rather than mutate it: `{ ...state, user: { ...state.user, name: "new" } }` — you replace `user` with a new object, not mutate the old one.

### Choose Deep Copy When Nested Mutation Must Not Leak

When the consumer may mutate nested structures, or you need a fully independent clone, use deep copy.

- **`structuredClone(obj)`** (modern, built-in): clones most objects including nested structures, `Date`, `RegExp`, `Map`, `Set`, `ArrayBuffer`, and cyclic references. It does not clone functions or DOM nodes, and throws on them.
- **`JSON.parse(JSON.stringify(obj))`**: a legacy deep copy that loses `undefined`, functions, `Date` (becomes string), `Map`/`Set`, symbols, and rejects circular references. It works only for plain JSON data.
- **Libraries** (lodash `_.cloneDeep`): handle edge cases and older environments.

Match the method to the data. `structuredClone` is the right default for plain data with dates and cycles; JSON round-trip is acceptable only for plain JSON-shaped data without dates or cycles.

### Update State Immutably: Replace, Do Not Mutate

In stateful UI (React, Redux, Vue) and in any code that compares old and new state to detect changes, you must produce a new top-level reference when state changes, not mutate in place. Mutation in place leaves the reference identical, so change detection sees no change.

- Return new objects/arrays: `{ ...state, count: state.count + 1 }`, `[...items, newItem]`.
- For nested updates, spread at each level: `{ ...state, user: { ...state.user, name: "x" } }`.
- Use immutable update helpers (`immer`) for deeply nested state to avoid spread pyramids.

The goal is structural sharing: the new state reuses unchanged branches and only allocates new nodes along the changed path. This is what makes immutable state efficient.

### Compare By Reference For Speed, By Value For Correctness

Reference equality (`===`, `Object.is`) is fast: two references are equal only if they point at the same object. This is why immutable updates work — change detection compares references, and a new reference signals a change.

- Use reference equality when immutability guarantees that unchanged state keeps the same reference (React memoization, Redux selectors).
- Use deep equality (`lodash.isEqual`, custom) only when you must compare by value, and understand it is O(n) and may be slow on large structures.
- Avoid deep equality in hot paths; design for reference equality via immutability instead.

### Freeze To Catch Mutation In Development

`Object.freeze(obj)` makes an object effectively immutable (mutation throws in strict mode, silently fails otherwise). Deep freeze recursively for development assertions. Use freezes to catch accidental mutation during development; they are usually too slow for production hot paths but invaluable for catching shared-mutation bugs early.

## Common Traps

### Mutating A Shared Object In Place

`function update(user) { user.name = "x"; return user; }` mutates the caller's object. If the caller did not expect it, this corrupts unrelated state. Copy before mutating, or return a new object.

### Shallow Copy Where Deep Was Needed

`const copy = { ...obj }; copy.nested.field = 1;` mutates `obj.nested.field` because `nested` is shared. Use deep copy or replace nested objects immutably.

### JSON Round-Trip Dropping Data

`JSON.parse(JSON.stringify(obj))` loses `undefined`, functions, `Date` (becomes string), `Map`/`Set`, and throws on cycles. Use `structuredClone` for data with these types.

### Mutation Breaking Change Detection

Mutating state in place leaves the reference unchanged, so React/Redux see no change and skip re-rendering. Produce a new reference via immutable update.

### Deep Copying Huge Structures Every Update

Deep-cloning a large state tree on every keystroke is expensive. Use structural sharing (immutable updates that copy only the changed path) or a library like immer.

### Assuming `===` Compares Contents

`{} === {}` is `false`; reference equality does not compare contents. Use deep equality only when needed, and prefer reference equality via immutability for performance.

### Deep Equality In Hot Paths

`_.isEqual` on every render or every item is O(n) and slow. Design for reference equality and memoize.

## Self-Check

- [ ] Reference vs value semantics is understood: shared object/array references are defended against when mutation could leak across aliases.
- [ ] Shallow copy (spread, `Object.assign`, `slice`) is used when only top-level mutation must be isolated; nested references are known to be shared.
- [ ] Deep copy uses `structuredClone` (or a library) for data with dates, maps, sets, or cycles; JSON round-trip is used only for plain JSON data without those.
- [ ] State updates are immutable: a new top-level reference is produced on change, with nested objects replaced rather than mutated, so change detection works.
- [ ] Structural sharing is used so that immutable updates copy only the changed path, not the entire state tree.
- [ ] Reference equality (`===`/`Object.is`) is the default comparison; deep equality is reserved for genuine value comparison and kept out of hot paths.
- [ ] `Object.freeze` (or deep freeze) is used in development to catch accidental mutation of shared state.
- [ ] No shared mutable object is passed to code that mutates it without the caller understanding and accepting the aliasing.
