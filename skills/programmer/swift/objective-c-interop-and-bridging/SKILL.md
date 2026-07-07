---
name: swift_objective_c_interop_and_bridging.md
description: Use when the agent is writing Swift that interoperates with Objective-C, using @objc and @objcMembers, NSObject subclasses, bridging headers and module maps, nullable and nullability annotations, NSError pointer APIs and try, selectors and #selector, KVO and key paths, blocks vs closures, NS_SWIFT_NAME and NS_REFINED_FOR_SWIFT, importing C APIs, or is diagnosing "this Objective-C API is awkward in Swift", nullability lost to ImplicitlyUnwrappedOptional, NSError not bridged to throws, selector ambiguity, name collisions, or "cannot find type in scope" from a bridging header. Covers the Swift/Objective-C/C bridging boundary, nullability and error bridging, API naming refinement, and the pitfalls of unsafe interop.
---

# Objective-C Interop And Bridging In Swift

Most real Swift code does not live in a pure-Swift world; it interoperates with Objective-C and C — the Apple frameworks, legacy codebases, third-party SDKs, and C libraries. That interop is impressively seamless (you call Objective-C methods as if they were Swift methods), and the seamlessness is the trap, because the two languages have fundamentally different semantics that the bridge papers over imperfectly. Objective-C has no generics on most APIs, no real nullability unless annotated, error handling via out-parameters (`NSError**`) rather than throws, reference semantics everywhere, and dynamic dispatch through selectors. When such an API crosses into Swift, the compiler makes guesses: unannotated references become implicitly-unwrapped optionals (the crash-prone `String!`), generic collections lose their element type (`NSArray` not `[Any]`), an `NSError**` parameter becomes `throws` only if it follows the convention, and selector-based APIs lose type safety. The judgment problem is to understand what the bridge does and does not translate, to annotate the Objective-C side where you control it (nullability, generics, Swift names), to handle the unsafe remainder deliberately on the Swift side, and to avoid the patterns that compile but crash or misbehave.

Agents call into Objective-C APIs as if they were Swift, ship `ImplicitlyUnwrappedOptional` crashes from unannotated APIs, write selectors that the compiler cannot check, or wrap C APIs unsafely. The remedy is to treat the interop boundary as a design surface: annotate and refine where you can, wrap the unsafe remainder in a safe Swift facade, and verify nullability, error bridging, and memory ownership at the boundary.

## Core Rules

### Annotate Nullability On The Objective-C Side You Control

Unannotated Objective-C references import to Swift as implicitly-unwrapped optionals (`NSString!`), which the compiler treats as non-optional in most expressions and crashes if nil. This is the single largest source of interop crashes. Annotate the Objective-C APIs you own with `nullable`, `nonnull`, and `null_resettable`, or wrap a header region in `NS_ASSUME_NONNULL_BEGIN`/`END` (defaulting to non-null) and mark the genuinely-nullable ones. Annotated APIs import as proper optionals (`NSString?`) or non-optionals (`NSString`), restoring Swift's null-safety across the boundary.

- Use `NS_ASSUME_NONNULL_BEGIN`/`END` around headers, marking only the nullable parameters/returns explicitly.
- `null_resettable` for a property that returns a non-nil default (e.g., a UIKit property) imports as non-optional with a default.
- For APIs you do not own, treat imported `!` types as potentially nil and check or wrap them.

### Bridge NSError Out-Parameters To throws Correctly

Objective-C signals failure via an `NSError**` out-parameter and a `BOOL`/`nil` return; Swift bridges this to `throws` only when the convention is followed (the error parameter is the last, `NSError**` type, and the return indicates success). When the convention is followed, `try method()` works; when it is not (wrong parameter order, or the API sets the error without returning failure), the error is not bridged and must be read manually. On the Swift side, call such APIs with `try` and catch the typed error; do not ignore the error parameter.

- Ensure your Objective-C APIs follow the `NSError**`-last convention so they bridge to `throws`.
- Catch the specific `NSError` domain/code you expect; do not catch generically and lose the information.
- Convert `NSError` to a Swift `Error`-conforming type at the boundary for a cleaner Swift API.

### Use @objc And @objcMembers Deliberately, Not Pervasively

`@objc` exposes a Swift declaration to Objective-C (and to Objective-C runtime features: selectors, KVO, NSCoding). It has costs: the declaration must be representable in Objective-C (no Swift-only features like generics on some declarations, tuples, `struct` beyond simple cases), and `@objc` methods use dynamic dispatch. `@objcMembers` on a class exposes all its members. Expose to Objective-C only what needs to be exposed (a method called via `#selector`, a class instantiated from a storyboard/`NSClassFromString`, a property observed via KVO); do not annotate everything, which bloats the binary and disables optimizations.

- `@objc` for methods called by selectors (`#selector`), `@IBAction`/`@IBOutlet`, `NSCoding`/`NSKeyedArchiver`, KVO-observed properties, and classes referenced by name from Objective-C or storyboards.
- `dynamic` (implies `@objc`) when you need Objective-C runtime dispatch (KVO, method swizzling) — use sparingly.
- Pure Swift types that do not cross to Objective-C should not be `@objc`.

### Handle Selectors And Key Paths Safely

`#selector(method)` is compile-checked (the method must exist and be `@objc`), which is far safer than the string-based Objective-C `NSSelectorFromString`. Use `#selector` and resolve ambiguity with `#selector(method(_:))` specifying the parameter signature. For key paths, prefer Swift key paths (`\Type.property`) over string-based KVC; they are type-checked and refactoring-safe. Where you must use runtime introspection (responds(to:), perform(_:)), isolate it behind a typed Swift wrapper rather than spreading it through the code.

- Use `#selector`, not `NSSelectorFromString`, for target/action.
- Prefer Swift key paths (`\Type.key`) over string KVC.
- Wrap runtime dispatch (`perform(_:)`) in a typed facade; do not let untyped selectors propagate.

### Refine Imported APIs With NS_SWIFT_NAME And NS_REFINED_FOR_SWIFT

Objective-C APIs often import with awkward Swift names (abbreviations, wrong argument labels, non-idiomatic types). `NS_SWIFT_NAME` renames the imported API to idiomatic Swift (including argument labels). `NS_REFINED_FOR_SWIFT` imports the API with a leading underscore so you can wrap it in a clean Swift extension that presents a better interface (hiding the `_`-prefixed original). Use these to present a Swift-native API over an Objective-C implementation, rather than forcing Swift callers to use awkward imported names.

- `NS_SWIFT_NAME` to rename methods/types/properties to idiomatic Swift.
- `NS_REFINED_FOR_SWIFT` plus a Swift extension to wrap an awkward API in a clean one.
- Bridge value types: convert `NSArray`/`NSDictionary` to typed Swift collections, `NSString` to `String`, in the wrapper.

### Manage Memory Ownership At The C Boundary

C APIs that return "retained" or "unretained" objects, or that work with raw pointers, require you to follow the ownership rule or leak/crash. Swift's `Unmanaged` lets you take an unretained or retained reference across the C boundary (`takeUnretainedValue()`, `takeRetainedValue()`); choose the one matching the C API's contract. For CoreFoundation toll-free-bridged types, the bridged cast rules apply. Get the ownership wrong and you either leak (took retained when unretained) or crash later (took unretained when retained, then it was released). Document the C API's ownership contract and wrap it once, correctly, behind a safe Swift facade.

## Common Traps

### ImplicitlyUnwrappedOptional Crashes From Unannotated APIs

An unannotated Objective-C API imports as `NSString!` and crashes when nil. Annotate the header or check/wrap on the Swift side.

### NSError Not Bridged To throws

An `NSError**` API that does not follow the convention does not bridge; the error is lost if you call it as a non-throwing method. Follow the convention or read the error manually.

### @objc On Everything

Pervasive `@objc`/`@objcMembers` bloats the binary and disables Swift optimizations; expose only what crosses to Objective-C.

### String Selectors Instead Of #selector

`NSSelectorFromString("foo")` is unchecked and crashes at runtime if wrong. Use `#selector`.

### Awkward Imported Names Left Unrefined

`NSArray`, abbreviated names, and missing argument labels make the Swift API ugly. Use `NS_SWIFT_NAME`/`NS_REFINED_FOR_SWIFT` and a typed wrapper.

### Wrong Ownership At A C Boundary

`takeUnretainedValue()` on a retained return leaks or crashes. Match the C API's ownership contract.

### KVO/KVC Without Key Path Safety

String-based `value(forKey:)` is unchecked; prefer Swift key paths and typed observation.

### Generic Swift Type Exposed To Objective-C

A generic Swift class or a struct with unsupported features cannot be `@objc`; redesign the interop surface to be Objective-C-representable.

## Self-Check

- [ ] Objective-C APIs you control are annotated for nullability (`NS_ASSUME_NONNULL_BEGIN`/`END` plus explicit `nullable`), so they import as proper optionals, not implicitly-unwrapped optionals.
- [ ] `NSError**` APIs follow the bridging convention (error last, success return) and are called with `try`/`catch`, with errors converted to a Swift `Error` type at the boundary.
- [ ] `@objc`/`@objcMembers`/`dynamic` are applied only to declarations that must cross to Objective-C or use runtime features, not pervasively.
- [ ] Selectors use `#selector` (compile-checked), key paths use Swift key paths (`\Type.property`), and runtime dispatch is wrapped in typed facades.
- [ ] Imported APIs are refined with `NS_SWIFT_NAME`/`NS_REFINED_FOR_SWIFT` and wrapped in clean Swift extensions that bridge to typed collections and value types.
- [ ] C-boundary memory ownership is handled correctly with `Unmanaged` (`takeRetainedValue`/`takeUnretainedValue`) matching the C API's contract, wrapped once behind a safe facade.
- [ ] No implicitly-unwrapped optional from an unannotated API is used as if non-nil without a check, and no untyped selector/key-path string propagates unchecked.
- [ ] The interop boundary has been considered under nullability, error bridging, naming, and ownership, and presents a safe, idiomatic Swift surface.
