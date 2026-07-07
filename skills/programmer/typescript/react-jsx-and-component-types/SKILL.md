---
name: typescript_react_jsx_and_component-types.md
description: Use when the agent is writing React or JSX in TypeScript, typing component props and state, choosing between function components and type signatures (FC, JSX.Element, ReactNode), typing event handlers and refs, using generics in components, typing hooks (useState, useRef, useContext, useReducer) and custom hooks, handling children and render props, controlling vs uncontrolled inputs, typing context and providers, or debugging "Type X is not assignable to type ReactNode", ref callback typing, and generic component inference. Covers component prop typing, ReactNode vs JSX.Element, event and ref typing, generic components, hook typing, context typing, and the tradeoff between strict prop types and ergonomic component APIs.
---

# React, JSX, And Component Types

React and TypeScript combine well, but the type-level patterns for components, props, hooks, events, and context have many sharp edges. Props types define the component's contract, event handlers carry specific element-type dependencies, refs have mutable-vs-immutable flavors, and generic components require careful inference design. The judgment problem is typing components so the compiler catches real prop and state mistakes, without adding ceremony that makes components painful to author or use.

Agents tend to over-use `React.FC` (which historically added implicit children and obscured return types), type event handlers as `any`, mistype refs, fail to make context provide-undefined-checks, or write generic components whose type parameters do not infer. The harm appears as props that should be rejected but pass, event handlers with wrong targets, refs that the compiler cannot track, context that crashes at runtime when a provider is missing but type-checks as always-defined, and generic components that force callers to specify types manually. The real work is typing props precisely, choosing the right node/handler/ref types, designing context that reflects the possibility of a missing provider, and making generics infer.

## Core Rules

### Type Props With Explicit Interfaces, And Avoid `React.FC` Pitfalls

Define props as an explicit `interface` or `type`, and annotate the function parameters directly. Avoid `React.FC` (or `React.FunctionComponent`) as the default wrapper.

- `function Button({ label, onClick }: ButtonProps) { ... }` is clear and lets the return type infer.
- `React.FC` historically added an implicit `children` prop (confusing when you do not want children) and obscured the return type. Modern React removed the implicit children, but direct annotation remains clearer and more flexible.
- For return types, let them infer (`JSX.Element` or `ReactNode`); annotating `: JSX.Element` is usually unnecessary and can be too narrow if the component returns `null` or a string.

Type props explicitly, annotate parameters, and let inference handle the rest.

### Distinguish `ReactNode`, `ReactElement`, And `JSX.Element`

These types overlap but differ, and choosing the wrong one rejects valid content or accepts invalid content.

- **`ReactNode`**: the broadest — anything React can render: elements, strings, numbers, arrays, `null`, `undefined`, `boolean`, portals. Use this for `children` and for "anything renderable."
- **`ReactElement`**: a single React element object (the result of `<div />`), not a string or number. Use when you specifically need one element object.
- **`JSX.Element`**: essentially `ReactElement` in the default configuration; the type of a JSX expression. Use for return types where you want exactly an element.

For `children`, use `ReactNode` (children can be strings, arrays, etc.). For "render this one element", use `ReactElement`. Using `JSX.Element` for children rejects strings and numbers, which is usually wrong.

### Type Event Handlers Against The Specific Element

React event handlers carry the element type in their target. `React.ChangeEvent<HTMLInputElement>` for an input's change, `React.MouseEvent<HTMLButtonElement>` for a button click.

- `function onChange(e: React.ChangeEvent<HTMLInputElement>) { ... e.target.value ... }` — `e.target.value` is a string because the target is an input.
- Typing the handler as `any` loses the target type and the safety it provides.
- `React.FormEvent<HTMLFormElement>` for form submits, `React.KeyboardEvent<HTMLDivElement>` for key events, etc.

Match the event type to the element. This is where React+TS catches real bugs (accessing `value` on a non-input target).

### Choose `useRef` For Mutable Values And DOM References, Typed Correctly

`useRef` has two roles with different types: holding a mutable value, and referencing a DOM node.

- **Mutable value**: `const timer = useRef<number | null>(null);` — the ref holds a value you can mutate without re-rendering. Initialize with the starting value (or `null`), and type the ref accordingly. `useRef<number>(0)` for a value always present; `useRef<number | null>(null)` for one that may be unset.
- **DOM ref**: `const inputRef = useRef<HTMLInputElement>(null);` — references a DOM node, initially `null` until mounted. Access `inputRef.current` and narrow for null after mount.
- Do not confuse the two: a DOM ref's current is `null` before mount, so type it as `T | null` and check.

Mistyping a ref (e.g., `useRef<HTMLInputElement>(null)` when you meant a mutable counter) causes confusing errors. Decide the role and type accordingly.

### Make Context Reflect The Possibility Of A Missing Provider

`useContext` returns the context's default value if no provider is present. If the default is `undefined`, the type is `T | undefined`, and consumers must check — which is usually correct, because a missing provider is a real condition.

- Create context with `undefined` default: `const Ctx = createContext<MyContext | undefined>(undefined);`.
- In the hook, check and throw: `const ctx = useContext(Ctx); if (!ctx) throw new Error("Ctx.Provider missing"); return ctx;`. This narrows the type to `T` for consumers and gives a clear runtime error if the provider is absent.
- Avoid creating context with a "dummy" non-undefined default to satisfy the type, because then a missing provider silently uses the dummy instead of failing.

A common bug is context typed as always-defined but crashing at runtime when the provider is missing. Make the type honest (`T | undefined`) and check.

### Design Generic Components To Infer Their Type Parameters

A generic component (`function List<T>(props: ListProps<T>)`) lets each call site have a concrete `T`. The challenge is making `T` infer from the props rather than requiring callers to specify it.

- Place the type parameter on a prop the caller must supply (e.g., the `items` array), so inference picks it up: `function List<T>({ items, render }: { items: T[]; render: (item: T) => ReactNode })`.
- If `T` appears only in optional props or not at all in the call, inference fails and callers must specify `<T>` manually, which is ergonomic pain.
- For forwardRef generic components, the ref type adds complexity; use the standard patterns or `forwardRef` with a generic-aware signature.

Design generics so the caller's data drives inference. Test that callers do not need to annotate the type parameter.

### Type Custom Hooks By Their Return Contract

A custom hook is a function; type its arguments and return explicitly. If it returns a tuple (like `useState`), type it as a tuple `[T, (v: T) => void]`, not an array, so destructuring gets the right types.

- `function useCounter(initial: number): [number, () => void]` — callers destructure `[count, increment]` with correct types.
- Returning an object is often clearer than a tuple for hooks with many values.
- Annotate the return type to lock the contract and prevent inference drift.

### Decide Controlled vs Uncontrolled Input Typing

A controlled input takes `value` and `onChange`; an uncontrolled input takes `defaultValue` and a ref. Type the props to reflect which contract the component offers, and do not mix (an input that is sometimes controlled and sometimes not is a common source of bugs). Use discriminated unions if a component supports both modes with different required props.

## Common Traps

### `React.FC` With Implicit Children Or Obscured Return

`React.FC` historically added implicit `children` and obscures the return type. Annotate props directly on the function parameters instead.

### `JSX.Element` For `children`

`children` can be strings, numbers, arrays, `null`. Use `ReactNode`, not `JSX.Element`, for children.

### Event Handler Typed As `any`

Loses the target type and the safety it provides. Type handlers against the specific element (`ChangeEvent<HTMLInputElement>`).

### DOM Ref Typed As Non-Null

A DOM ref is `null` until mount. Type as `T | null` and check `current` before use.

### Context With A Dummy Default

A non-undefined default masks a missing provider. Use `undefined` default and check in the hook, throwing if absent.

### Generic Component That Does Not Infer

If the type parameter does not appear in a required prop, callers must specify it manually. Design generics so inference flows from the caller's data.

### Hook Return Typed As Array Instead Of Tuple

`[T, setter]` typed as `(T | setter)[]` loses positional types. Use a tuple type.

### Mixing Controlled And Uncontrolled Props

An input that takes both `value` and `defaultValue` is ambiguous and bug-prone. Type the component for one contract, or use a discriminated union for both.

## Self-Check

- [ ] Component props are typed with explicit interfaces and annotated on the function parameters; `React.FC` is avoided in favor of direct annotation.
- [ ] `children` is typed as `ReactNode` (not `JSX.Element`); single-element slots use `ReactElement`.
- [ ] Event handlers are typed against the specific element (`ChangeEvent<HTMLInputElement>`, `MouseEvent<HTMLButtonElement>`), not `any`.
- [ ] `useRef` is typed by role: mutable value refs hold the value type, DOM refs are `T | null` and checked after mount.
- [ ] Context is created with an `undefined` default, and the consumer hook checks and throws if the provider is missing, narrowing the type to `T`.
- [ ] Generic components infer their type parameters from required props so callers do not annotate them manually.
- [ ] Custom hooks return explicitly typed contracts (tuples typed as tuples, objects for many fields).
- [ ] Controlled vs uncontrolled input props are typed for one contract, or discriminated if both are supported.
