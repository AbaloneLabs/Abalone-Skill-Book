---
name: javascript_prototype_chain_and_this_binding.md
description: Use when the agent is designing or debugging JavaScript prototypal inheritance, the prototype chain, property lookup and shadowing, __proto__ vs prototype, Object.create vs class syntax, super in subclasses, static vs instance members, mixins, hasOwnProperty vs in checks, or reasoning about how this resolves inside inherited, static, getter, or arrow-function methods. Covers the mechanics of prototype-based delegation, classes as syntactic sugar over prototypes, and the interaction between the prototype chain and this binding.
---

# Prototype Chain And `this` Binding

JavaScript inheritance is prototypal: objects delegate to other objects through a chain of prototypes, and property lookups walk that chain until they find a match or reach `null`. The `class` keyword is syntactic sugar over this delegation, not a separate system. The judgment problem is reasoning about *which object owns which property*, how lookups traverse the chain, how `this` resolves when a method is inherited or shared, and where the sugar hides behavior that breaks in real code.

Agents tend to confuse `__proto__` with `prototype`, assume `class` works like classes in other languages (it does not — it is delegation), put data on prototypes where it gets shared unexpectedly, lose `this` when calling inherited methods detached from their receiver, or reach for deep hierarchies when composition would be simpler. The harm appears as shared mutable state across instances (a property on the prototype mutated by one instance affecting all), methods that silently fail because `this` is wrong, broken `instanceof` checks across realms, and fragile inheritance towers. The real work is understanding delegation, placing data and methods correctly, binding `this` deliberately, and preferring composition over deep inheritance.

## Core Rules

### Understand The Two Different Prototype References

The most common confusion is between `__proto__` (or `Object.getPrototypeOf`) and `prototype`. They are different references on different objects.

- **`__proto__`** (the actual prototype link): every *object* has a `[[Prototype]]` internal slot, accessed via `Object.getPrototypeOf(obj)` or (legacy) `obj.__proto__`. This is the link the runtime walks during property lookup.
- **`prototype`**: only *functions* have a `.prototype` property, and it is the object that becomes the `[[Prototype]]` of instances created with `new`. It exists so that `new Foo()` instances delegate to `Foo.prototype`.

So `Foo.prototype` is *not* the prototype of `Foo`; it is the prototype of *instances of `Foo`*. The prototype of `Foo` itself (a function object) is `Function.prototype`. Conflating these two is the root of most prototype misunderstandings. When you write `Object.getPrototypeOf(instance) === Foo.prototype`, that is `true`; `Foo.__proto__ === Function.prototype` is also `true` and is a different fact.

### Trace Property Lookup Along The Chain

When you read `obj.prop`, the runtime looks for `prop` on `obj`, then on `obj.__proto__`, then that object's prototype, and so on until found or until `null`. Writes (`obj.prop = x`) usually create an *own* property on `obj`, shadowing the inherited one rather than mutating the prototype — which is why shared methods on the prototype are safe but shared *data* on the prototype is dangerous.

- Methods on the prototype are shared by reference and called with `this` set to the receiver, so they behave correctly per-instance.
- Data (objects, arrays) placed on the prototype is shared across all instances. Mutating it (`this.sharedArray.push(x)`) affects every instance because the lookup finds the same array on the prototype and mutates it. Put per-instance data in the constructor (`this.x = ...`), never on the prototype.
- Use `hasOwnProperty` (or `Object.hasOwn`) to check for own properties vs inherited ones; the `in` operator walks the whole chain. This matters for serialization, copying, and detecting inherited defaults.

### Know That Class Is Sugar Over Prototypes

A `class` does not introduce a new inheritance mechanism; it compiles to constructor functions and prototype links with stricter ergonomics.

- `class Foo { method() {} }` puts `method` on `Foo.prototype`, exactly as `Foo.prototype.method = function() {}` would. Instances delegate to it identically.
- `class Foo { x = 1 }` (field syntax) puts `x` as an *own* property on each instance (like writing `this.x = 1` in the constructor), which is the correct place for per-instance data.
- `static` members go on the constructor function itself (`Foo.staticMethod`), not on the prototype. `this` inside a static method is the class/constructor.
- `extends` wires up the prototype chain: `Sub.prototype.__proto__ === Super.prototype`, and `Sub.__proto__ === Super` (so statics are inherited too).

Treat `class` as convenient syntax, not as a fundamentally different model. The delegation rules still apply, and reasoning about them still matters when something breaks.

### Resolve `this` By Receiver, Including Inherited Methods

`this` in a method is determined by the *receiver* (the object before the dot at the call site), not by where the method is defined. An inherited method called as `instance.method()` has `this === instance`, which is why prototype methods work per-instance. The trap is *detaching* a method from its receiver.

- `const fn = instance.method; fn()` loses the receiver; `this` falls back to default binding (`undefined` in strict mode, global otherwise). This is why `setTimeout(obj.method, 0)` or passing `obj.method` as a callback breaks.
- Inherited methods are not immune: `const fn = instance.toString; fn()` has the same problem.
- Fix with `.bind(instance)`, an arrow wrapper (`() => instance.method()`), or by making the method a bound field (`method = () => {...}` which captures `this` lexically per instance).

### Use `super` Correctly In Subclasses

`super.method()` calls the parent's method with the current `this` (the subclass instance), enabling override-and-extend. `super` is lexically bound to the class declaration and resolved through the prototype chain, but its `this` is the call's receiver.

- Always call `super()` in a subclass constructor before using `this`; the engine creates `this` via the super call. Forgetting it throws.
- `super.method()` in a method delegates to `Super.prototype.method` with `this` set to the instance — so state set by the parent constructor is visible.
- `super` in a *static* method refers to the parent class's statics, not instance prototype methods. `super` cannot be used outside class methods (it is not a general feature).

### Prefer Composition Over Deep Inheritance

Prototype-based inheritance is single-inheritance (one `[[Prototype]]` link per object). Deep hierarchies couple subclasses to parent internals and are hard to refactor. Composition (giving an object references to other objects that provide behavior) is usually more flexible.

- Use mixins (`Object.assign(Target.prototype, Mixin)`) to share behavior across unrelated classes, but be aware they flatten methods onto the target and can cause name collisions.
- Favor "has-a" relationships (an object *containing* collaborators) over "is-a" hierarchies for behaviors that vary independently.
- Keep hierarchies shallow; if a subclass exists only to override one method, consider composition or a strategy parameter instead.

### Mind Cross-Realm And `instanceof` Limitations

`instanceof` checks the prototype chain of the left operand against `constructor.prototype`. It works within a realm but breaks across realms (iframes, different Node `vm` contexts) because each realm has its own copy of globals like `Array` and `Object`.

- `iframe.contentWindow.Array !== Array`, so `[] instanceof Array` is `false` for arrays from another realm. Use `Array.isArray(x)` (realm-independent) for arrays.
- For duck-typing across realms, check for the presence of methods/properties rather than `instanceof`.
- `Symbol.hasInstance` lets a type customize `instanceof`, so `instanceof` is not always a simple chain walk.

## Common Traps

### Putting Shared Mutable Data On The Prototype

`Foo.prototype.items = []` means every instance shares one array; `instance.items.push(x)` mutates it for all. Put per-instance data in the constructor or as a class field.

### Confusing `__proto__` And `prototype`

`Foo.prototype` is the prototype of *instances*, not of `Foo`. `Foo.__proto__` is `Function.prototype`. Mixing them up leads to wrong lookups and broken delegation.

### Losing `this` By Detaching An Inherited Method

`const fn = obj.method; fn()` loses the receiver. Bind explicitly or wrap in an arrow when passing methods as callbacks.

### Forgetting `super()` In A Subclass Constructor

`this` is not available until `super()` runs in a subclass constructor. Using `this` before `super()` throws; omitting `super()` entirely throws.

### `in` Finding Inherited Properties

`"toString" in obj` is `true` for every object (inherited). Use `Object.hasOwn(obj, key)` to check own properties when detecting fields you actually set.

### Treating `class` Like Classical Inheritance From Another Language

JS classes are delegation. `instanceof`, `super`, and static inheritance follow prototype rules, not a separate class system. Deep hierarchies and multiple inheritance expectations break.

### `instanceof` Failing Across Realms

Arrays/objects from another iframe or `vm` context fail `instanceof Array`. Use `Array.isArray` or structural checks for cross-realm code.

### Arrow Methods Breaking `super` Or Prototype Override

A class field arrow (`method = () => {}`) is an own property per instance, so subclasses cannot override it via the prototype chain and `super.method` will not find it on the prototype. Use regular methods when subclass override or `super` is intended.

## Self-Check

- [ ] `__proto__` (instance prototype link) and `prototype` (function's instance-prototype object) are not conflated; lookups are traced through the correct chain.
- [ ] Per-instance data lives in the constructor or as class fields, never as mutable data on the prototype; shared prototype data is read-only or avoided.
- [ ] `class` is understood as sugar over prototypes; methods land on `.prototype`, fields as own properties, statics on the constructor.
- [ ] `this` is resolved by receiver; methods passed as callbacks are bound (`.bind`, arrow wrapper, bound field) where the receiver would be lost.
- [ ] `super()` is called before `this` in subclass constructors; `super.method()` is used to override-and-extend with the correct `this`.
- [ ] `hasOwnProperty`/`Object.hasOwn` is used to distinguish own from inherited properties; `in` is not relied on for "did I set this."
- [ ] Inheritance is kept shallow; composition and mixins are preferred where behaviors vary independently.
- [ ] `instanceof` is not relied on across realms; `Array.isArray` or structural duck-typing is used for cross-realm values.
- [ ] Methods intended for subclass override are regular prototype methods, not arrow fields, so `super` and the prototype chain work.
- [ ] No deep inheritance tower exists where a flatter composition or strategy would be clearer.
