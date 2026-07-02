---
name: swift_arc_and_memory_management.md
description: Use when the agent is writing or reviewing Swift code involving reference cycles, strong/weak/unowned references, closure capture lists, escaping and non-escaping closures, value types vs reference types and their memory implications, delegates, NotificationCenter observers, Combine subscriptions, or any design where Automatic Reference Counting retain cycles can leak objects or where value-vs-reference semantics determine copying and identity behavior.
---

# ARC And Memory Management In Swift

Swift uses Automatic Reference Counting (ARC): every strong reference to a class instance increments a retain count, and the instance is deallocated when the count reaches zero. ARC is deterministic and predictable, but it cannot break cycles. If two objects hold strong references to each other, or a closure captures a strong reference to the object that owns it, the retain count never reaches zero and the objects leak for the lifetime of the process. Unlike garbage collection, ARC will not find and collect cycles; the programmer must break them with `weak` or `unowned`. This is the central memory-management judgment in Swift, and it is invisible until a leak accumulates or a delegate crashes.

Agents often write Swift as if ARC handled everything, because for simple ownership graphs it does. The harm appears in the patterns that create cycles: delegates held strongly, closures passed to escaping contexts that capture `self`, NotificationCenter observers never removed, Combine cancellables stored on the publishing object. These leak silently — no crash, no error, just growing memory and stale objects that keep responding to events after they should be gone. The judgment problem is to recognize the cycle-creating patterns by sight, to choose `weak` vs `unowned` vs strong deliberately based on lifetime relationships, to use capture lists in escaping closures as a matter of course, and to decide at the type level whether a value type (struct/enum) or a reference type (class) is correct, because that choice determines whether ARC even applies.

## Core Rules

### Recognize The Cycle-Creating Patterns By Sight

Reference cycles have a small number of recurring shapes. Learn to spot them before writing them.

- **Parent-child with back-pointer:** a parent owns a child strongly, and the child holds a reference back to the parent (a delegate, a view referencing its controller). If both are strong, the cycle leaks. The back-pointer must be `weak` (or `unowned`).
- **Closures capturing self:** a closure stored on, or passed to an escaping context owned by, an object that captures that object strongly forms a cycle. `class Controller { var onComplete: (() -> Void)?; func setup() { onComplete = { self.doThing() } } }` — `onComplete` holds `self`, `self` holds `onComplete`, leak.
- **Observers and subscriptions:** NotificationCenter observers, KVO, Combine `sink` subscriptions, Timer targets — if the observer/subscription is stored on or retains the observed object, a cycle forms. Combine cancellables stored as instance properties while the closure captures `self` is a classic.
- **Mutual references in data models:** two model objects referencing each other (order references customer, customer caches recent orders) leak if both directions are strong.

The discipline: whenever you write a property or closure that references another object that might reference back, ask "is this a cycle?" and reach for `weak` if it is.

### Choose weak vs unowned By The Lifetime Relationship

Both `weak` and `unowned` break cycles by not incrementing the retain count, but they differ in what they assume about the referenced object's lifetime.

- **`weak`** is for references that may outlive the object they point to. A `weak var` is always optional (`weak var delegate: Delegate?`), and reading it may yield nil after the object is deallocated. Use `weak` when the referenced object can be freed before the referrer — the safe default for delegates, observers, and any back-pointer where the lifetime is independent.
- **`unowned`** is for references guaranteed to have the same or longer lifetime than the referrer. An `unowned` reference is non-optional and crashes if the referenced object is freed before the referrer accesses it. Use `unowned` only when you can prove the referenced object outlives the referrer (e.g., a child referencing a parent that owns it and will always be freed first).
- Prefer `weak` unless you have a specific, proven reason for `unowned`. The crash from a dangling `unowned` is worse than the optional handling `weak` requires, and the performance difference is negligible in almost all cases.

Strong choice: `weak var delegate: Delegate?` for any callback target whose lifetime you do not control. Weak choice: `unowned let delegate: Delegate` "for performance" when the lifetime is not actually guaranteed.

### Use Capture Lists In Escaping Closures As A Matter Of Course

A closure that escapes the scope it was defined in (stored, passed to async code, used as a callback) captures its captured variables strongly by default. Inside a method, `self` is captured strongly, and if the closure is stored on `self` or owned by `self`, a cycle forms. Capture lists override this.

- `[weak self] in` captures self weakly; inside the closure, `self` is optional, so bind it (`guard let self = self else { return }`). This is the standard pattern for escaping closures that may outlive their owner.
- `[unowned self] in` captures self unowned; use only when the closure's lifetime is bounded by self's (e.g., a closure that self stores and that will be freed with self).
- `[self] in` captures self strongly and explicitly; use only when you have confirmed no cycle (the closure is not stored on or owned by self).
- Capture specific values by value with `[localCopy = self.property] in` to snapshot them and avoid capturing all of self.

Non-escaping closures (passed to a function and called before it returns) do not need capture lists because they cannot form cycles — they are freed when the call returns. The compiler marks closures `@noescape` by default for ordinary parameters.

### Mark Closures escaping Deliberately

A closure parameter marked `@escaping` can be stored or called after the function returns; a non-escaping closure cannot. The distinction determines whether capture-list concerns apply.

- `@escaping` closures are the ones that can form cycles, because they outlive the call. When you write a function that stores a closure parameter (in a property, in a dispatch call, in an async context), mark it `@escaping` and document the lifetime expectation.
- Non-escaping closures (the default for ordinary parameters) are cycle-safe and need no capture list, which is why APIs like `map` and `forEach` do not require `[weak self]`.
- When you see `@escaping`, think "capture list" — that is the signal that the closure may outlive self and needs weak/unowned capture.

### Prefer Value Types Where Identity And Sharing Are Not Needed

Structs and enums are value types: they are copied on assignment, not reference-counted, and cannot form cycles. Classes are reference types: they are shared, reference-counted, and can form cycles. The choice determines whether ARC applies at all.

- Use a struct for data models, DTOs, configurations, and any type where copying is the right semantics and shared identity is not needed. Structs eliminate an entire class of memory bugs because they cannot cycle.
- Use a class when you need shared mutable identity (an object that multiple owners see and mutate), Objective-C interop, or polymorphism via inheritance (though protocols often serve better).
- Mixing value and reference types: a struct containing a class property still references the class by reference; the struct's copy shares the class instance. Be deliberate about which nested types are value vs reference, because a "struct" that holds class properties is not fully value-semantic.

The Swift-idiomatic bias is toward value types; reach for classes only with a reason.

### Clean Up Observers And Subscriptions Explicitly

Some cycle-prone APIs do not use weak references automatically and require explicit teardown.

- NotificationCenter observers registered with a block capture the observer strongly; store the returned token and call `remove(_:)` in `deinit`, or use the block-based API with `[weak self]`.
- Combine `sink` subscriptions return a `AnyCancellable` that must be stored (often in a `Set<AnyCancellable>`); if the cancellable is stored on the publishing object and the closure captures self, use `[weak self]` in the closure.
- Timer (`Timer.scheduledTimer`) retains its target; a timer targeting self keeps self alive until invalidated. Invalidate timers in `deinit` or on the appropriate lifecycle event.
- KVO and other observer patterns similarly require explicit removal; forgetting it leaks the observer.

## Common Traps

### Delegate Held Strongly, Leaking Both Objects

`class View { var delegate: Controller }` where the controller owns the view creates a cycle. The delegate must be `weak var delegate: Controller?`. This is the single most common Swift leak.

### Escaping Closure Capturing self Strongly

`onComplete = { self.finish() }` stored on self leaks. Use `[weak self] in` and bind. The compiler does not warn about this; the leak is silent.

### unowned Reference That Outlives Its Target

`unowned let parent: Parent` in a child that can survive the parent (e.g., the child is passed to async work) crashes when the parent is freed. Use `weak` unless the lifetime is provably bounded.

### Combine sink Storing A Cancellable On self With Strong Capture

`cancellables.insert(publisher.sink { self.handle($0) })` stored on self leaks if the publisher is also tied to self. Use `[weak self] in` in the sink closure.

### Timer Retaining Its Target

`Timer.scheduledTimer(timeInterval:target:...)` retains the target until invalidated. A timer targeting a view controller keeps the controller alive after dismissal. Invalidate in `deinit` or use the block-based API with `[weak self]`.

### Struct With Class Properties Thinking It Is Value-Semantic

`struct Counter { var object: NSMutableString }` — copying the struct copies the reference to the same NSMutableString; mutations through one copy affect the other. If you need value semantics, the nested type must also be a value type, or you must implement copy-on-write.

### Assuming ARC Will Collect Cycles

ARC does not detect or collect cycles; it only counts strong references. A cycle is permanent until the process ends. There is no GC safety net. Every potential cycle must be broken explicitly with `weak`/`unowned`.

### NotificationCenter Observer Never Removed

Block-based observers registered without `[weak self]` and never removed leak the observer and keep it responding to notifications after it should be gone. Store the token and remove in `deinit`, or use `[weak self]` in the block.

## Self-Check

- [ ] Every back-pointer, delegate, and observer property is `weak` (or `unowned` with a proven lifetime guarantee), and no two objects hold strong references to each other.
- [ ] Every escaping closure (`@escaping`, stored closures, async/dispatch callbacks) that captures `self` uses a capture list (`[weak self]` or `[unowned self]` with justification), and `weak self` closures bind self before use.
- [ ] `unowned` is used only where the referenced object's lifetime is provably at least as long as the referrer's; the default for uncertain lifetimes is `weak`.
- [ ] Combine subscriptions, NotificationCenter observers, KVO, and Timers are torn down explicitly (cancellables stored, tokens removed in `deinit`, timers invalidated), and their closures use `[weak self]`.
- [ ] Value types (struct/enum) are the default for data, DTOs, and configuration; classes are used only where shared identity, interop, or inheritance is genuinely needed.
- [ ] Structs that contain class properties are recognized as not fully value-semantic, and copy-on-write or value-typed nesting is used where value semantics are required.
- [ ] No cycle is left for "ARC to collect" — every potential cycle has been identified and broken with weak/unowned, because ARC does not collect cycles.
- [ ] Closure parameters are marked `@escaping` only when they actually escape, and `@escaping` is treated as the signal to add a capture list.
- [ ] The code has been checked for leaks (Xcode Memory Graph, Instruments Allocations/Leaks) for the patterns above, especially delegates, escaping closures, and subscriptions.
- [ ] The choice of `weak` vs `unowned` vs strong at each reference reflects the actual lifetime relationship, not a copy-paste habit.
