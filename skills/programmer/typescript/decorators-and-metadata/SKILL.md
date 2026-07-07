---
name: typescript_decorators_and_metadata.md
description: Use when the agent is writing or reviewing TypeScript decorators (class, method, property, accessor, parameter), legacy experimental decorators vs the TC39 stage-3 decorator proposal, reflect-metadata and emitDecoratorMetadata, dependency injection containers (TypeDI, InversifyJS, NestJS), decorator factories and execution order, metadata design keys, or is diagnosing decorator ordering surprises, metadata not emitted, "decorator does not apply", inheritance and override issues, or migration from experimental to standard decorators. Covers decorator semantics, metadata reflection, DI container integration, and the pitfalls of the experimental-vs-standard split.
---

# Decorators And Metadata In TypeScript

Decorators are a syntax for adding annotations and meta-behavior to classes and their members, and they sit on a fault line that produces most of the confusion around them: there are two incompatible decorator specifications. The legacy "experimental decorators" (`experimentalDecorators`) shipped in TypeScript for a decade and underpin most existing ecosystem code — NestJS controllers, TypeORM entities, class-validator decorators, InversifyJS/TypeDI injection, and MobX/mobx-react. The TC39 stage-3 standard decorators are a different, cleaner design that is now native to modern TypeScript and runtimes, with different semantics (decorators receive a context object, cannot easily mutate the descriptor the old way, and have a well-defined initializer model). Code written for one does not run under the other, metadata keys differ, and libraries assume one or the other. The judgment problem is to know which decorator world a codebase is in, to use decorators as readable annotations backed by a runtime that consumes the metadata (rather than as invisible magic), and to handle the ordering, inheritance, and metadata-emission rules that quietly change behavior.

Agents often sprinkle decorators (`@Injectable()`, `@Get()`, `@Column()`, `@IsString()`) because a framework requires them, without understanding that a decorator is a function called at class-definition time, that ordering of multiple decorators is bottom-up for methods and top-down for parameters, that `emitDecoratorMetadata` is what makes DI-by-type possible and is a compile-time emit that depends on a `reflect-metadata` polyfill at runtime, and that inheritance does not always re-run decorators on overridden members. The remedy is to treat decorators as a contract between the annotation and the framework that reads it, to verify metadata is actually emitted and consumed, and to be deliberate during any migration between the experimental and standard worlds.

## Core Rules

### Know Which Decorator World The Codebase Is In, And Stay Consistent

Determine whether the project uses experimental decorators (`experimentalDecorators: true`, usually with `emitDecoratorMetadata: true` and `reflect-metadata` imported) or standard TC39 decorators (no flag, modern TS/runtime). They are not interchangeable: an experimental decorator factory has signature `(target, key, descriptor) => ...`, a standard decorator receives `(value, context)` and returns an initializer/extra class element. Mixing them in one file produces confusing errors. Pick one per project, set the tsconfig accordingly, and use libraries compatible with that choice.

- Experimental: required by most current framework-heavy code (NestJS, TypeORM, class-validator, InversifyJS). Needs `experimentalDecorators` and, for DI-by-type, `emitDecoratorMetadata` plus `import 'reflect-metadata'`.
- Standard (stage-3): native, no polyfill, cleaner semantics, but the ecosystem migration is ongoing; verify each library supports it before adopting.

### Understand That A Decorator Is A Function Called At Definition Time

A decorator is not a runtime wrapper added per call; it is a function invoked once when the class is defined, receiving the member (or a descriptor/context) and returning a replacement. Side effects in a decorator factory run at decoration time, not per method invocation. This means: a decorator that closes over mutable state shares that state across all instances (it is per-class-member, not per-instance); a decorator that performs expensive setup does so once, not per call; and the decorated member is replaced by whatever the decorator returns (or left unchanged if it returns nothing).

- For per-instance behavior, the decorator must read/write `this` at call time, not capture state in the factory.
- A method decorator that returns a new function replaces the method for all instances; design the replacement to forward to the original via the descriptor.

### Use emitDecoratorMetadata And reflect-metadata Together, Or Not At All

DI-by-type (`@Inject()` resolving the constructor parameter's type) depends on `emitDecoratorMetadata`, which emits `design:paramtypes`, `design:type`, and `design:returntype` metadata keys at compile time, and on `reflect-metadata` being imported at runtime to read them. Without `emitDecoratorMetadata`, there is no type metadata to read; without `reflect-metadata`, the metadata keys do not exist at runtime. Both must be present, and `reflect-metadata` must be imported once, before any decoration runs (usually in the entry point).

- The emitted design metadata uses the TypeScript type at the declaration site; interfaces and type aliases are erased, so `design:paramtypes` records `Object` for interface-typed parameters — DI containers need a token (a class or a string symbol) for interface-typed injections.
- Metadata is a compile-time emit; changing a type without recompiling leaves stale metadata.

### Get Decorator Ordering Right

Multiple decorators on a member apply in a defined order, and that order changes behavior (e.g., a validation decorator wrapping a logging decorator behaves differently from the reverse). For experimental decorators: method/property decorators apply bottom-up (the decorator closest to the member runs first), while parameter decorators apply in source order. For factories (`@Dec()` with parentheses), the factory is called top-down to produce the decorator functions, then the decorators apply bottom-up. Standard decorators apply top-down. When order matters (and it usually does for wrappers), verify the actual application order against the spec the codebase uses.

### Handle Inheritance And Overrides Deliberately

Decorators on a base-class method do not automatically re-apply to an override in a subclass; the subclass method is undecorated unless it carries its own decorators. Metadata set on the base is inherited via the prototype chain for reads but writes target the class where the decorator ran. When designing a decorator-based framework, decide and document whether subclassing inherits decoration, and test the override case.

### Prefer Decorators As Annotations Backed By A Runtime, Not Standalone Magic

The cleanest use of decorators is as declarative annotations (`@Get('/users')`, `@Column()`, `@IsEmail()`) that a framework reads (via metadata or registration) to wire behavior. Decorators that invisibly transform logic (`@Memoize`, `@Deprecate`) are convenient but hide control flow from readers; prefer them only where the annotation is the point (validation, routing, ORM mapping) and keep transformative decorators small and well-named. Always ensure a runtime consumer exists; a decorator with no reader is dead annotation.

## Common Traps

### Mixing Experimental And Standard Decorators

A file with `experimentalDecorators` off but using `@Injectable()` (an experimental-shape factory) fails or silently no-ops. Confirm the tsconfig flag and the library's expected decorator flavor match.

### Missing reflect-metadata Import

DI-by-type works in tests (where the import happens early) and fails in a deployed bundle that tree-shook the side-effect import. Import `reflect-metadata` once in the entry point and ensure the bundler does not drop it.

### emitDecoratorMetadata Off But Code Assumes It

`Reflect.getMetadata('design:paramtypes', ...)` returns `undefined` when `emitDecoratorMetadata` is off, breaking DI. Enable the flag and recompile, or pass explicit tokens.

### Decorator State Shared Across Instances

A factory-captured variable is shared by all instances of the class. For per-instance state, read/write `this` inside the wrapped method, or attach per-instance data via a `WeakMap` keyed by `this`.

### Wrong Decorator Ordering

A `@Log` above `@Auth` wraps differently than the reverse, and the bottom-up application of experimental decorators is counter-intuitive. Test the composed behavior, especially when one decorator short-circuits (auth failing before logging).

### Inheritance Drops Decoration

Overriding a decorated method in a subclass loses the decorator unless reapplied. Document and test the inheritance contract for decorator-based frameworks.

### Interface-Typed Parameter Resolves To Object

`design:paramtypes` records `Object` for an interface-typed parameter, so DI cannot resolve it by type. Use a class token or an explicit `@Inject(TOKEN)`.

## Self-Check

- [ ] The codebase uses one decorator flavor consistently (experimental with `experimentalDecorators`/`emitDecoratorMetadata`/`reflect-metadata`, or standard stage-3), and the tsconfig and library choices match that flavor.
- [ ] Decorators are understood as definition-time functions: per-class-member state is not mistaken for per-instance state, and per-instance behavior reads/writes `this` at call time.
- [ ] DI-by-type has `emitDecoratorMetadata` enabled, `reflect-metadata` imported once in the entry point (and not tree-shaken), and interface-typed parameters use explicit tokens rather than relying on erased types.
- [ ] Multiple decorators are applied in the intended order for the chosen flavor, and composed behavior (especially short-circuiting decorators) has been tested.
- [ ] Inheritance and overrides behave as documented: subclass overrides carry their own decorators where required, and metadata reads/writes target the intended class.
- [ ] Decorators are used as annotations backed by a runtime consumer (validation, routing, ORM, DI), and transformative decorators are small, well-named, and not hiding control flow.
- [ ] No decorator is dead annotation: every decorator has a reader, and metadata keys are emitted and consumed consistently after recompilation.
- [ ] Any migration between experimental and standard decorators has been planned per-file with the signature/semantics differences accounted for, and not assumed to be a flag flip.
