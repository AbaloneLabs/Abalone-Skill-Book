---
name: rust_lifetime_and_borrow_checker.md
description: Use when the agent is writing or reviewing Rust code that involves named lifetimes, lifetime elision, lifetime bounds, higher-ranked trait bounds (HRTBs), variance, self-referential structs, iterator borrowing, returning references, struct fields that borrow constructor arguments, fighting borrow-checker errors, or designing APIs where references must outlive a scope. Covers the mechanics of how the borrow checker proves references are valid, when to name lifetimes, variance and subtyping, and how to read lifetime errors instead of silencing them with clones.
---

# Lifetime And Borrow Checker

Rust's borrow checker proves, at compile time, that no reference outlives the data it points to. This is the core of Rust's memory safety, but the proof is mechanical and literal: it reasons about scopes, regions, and variance, not about your intent. The judgment problem is translating a design into lifetime annotations the checker can verify, and reading its errors as information rather than fighting them with clones and restructuring.

Agents tend to add lifetime parameters reactively, sprinkle `'a` on everything until it compiles, clone values to silence errors they do not understand, or design self-referential types that the borrow checker fundamentally cannot express. The harm appears as APIs with incomprehensible lifetime signatures, functions that force callers to clone, types that cannot be constructed, and silent soundness holes patched over with `unsafe`. The real work is knowing when lifetimes are needed, when elision already does the right thing, how variance constrains what is assignable, and how to restructure code so the ownership story is clear rather than annotated into submission.

## Core Rules

### Know When The Compiler Infers Lifetimes (Elision)

You do not need to annotate most lifetimes. Elision rules let the compiler fill them in for common patterns. Understand the three rules so you annotate only when elision is insufficient:

- Each elided lifetime in an input position becomes its own distinct lifetime parameter.
- If there is exactly one input lifetime, it is assigned to all elided output lifetimes.
- If there are multiple input lifetimes but one is `&self`/`&mut self`, that lifetime is assigned to all elided output lifetimes.

When elision applies, adding explicit lifetimes is noise. When elision does not apply (multiple input references with an output reference, or no `&self`), the compiler forces you to name the relationship. That forcing is the signal: the compiler cannot decide which input the output borrows, so you must state it. Read the error as a question, not an obstacle.

### Name Lifetimes To Express A Borrowing Relationship

When you write a named lifetime, you are asserting a relationship: "this output reference is valid for at least as long as this input reference." Name lifetimes to express which input a returned reference borrows from.

- `fn first<'a>(s: &'a str) -> &'a str` says the returned slice shares the lifetime of `s`.
- `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str` says the result is valid for the intersection of the two inputs' lifetimes — the shorter of the two.
- Multiple distinct lifetimes (`'a`, `'b`) express that one output borrows from one specific input, not the others.

Do not collapse distinct relationships into one lifetime parameter to reduce annotation count. If an output borrows only from `a`, give it `'a` and leave `b` unconnected; binding them together (`'a` on both) over-constrains callers by forcing the unrelated input to live as long as the borrowed one.

### Understand Variance And Subtyping

Lifetimes have variance, which determines when one type is a subtype of another. This is what makes `&'static str` usable where `&'a str` is expected, and what makes some refactorings type-check or fail.

- `&'a T` is covariant in `'a`: a longer lifetime is a subtype of a shorter one. `&'static str` can be used where `&'short str` is required.
- `&'a mut T` is covariant in `'a` but the relationship to `T` depends on `T`'s variance.
- `fn(T) -> U` is contravariant in `T` and covariant in `U`.
- `Cell<T>`, `UnsafeCell<T>`, and `&mut T` make `T` invariant, which forbids subtyping and is what prevents writing a wrong-typed value through a mutable reference.

When the compiler rejects an assignment that "should" work, consider variance. Invariance is the common culprit: a type containing `&mut T` or a `Cell` forces exact type matching to preserve soundness. Do not try to "fix" this with transmute or unsafe; it is the checker protecting you from a real bug.

### Avoid Self-Referential Structs

A struct that holds both an owned value and a reference to a part of itself (a self-referential struct) cannot be expressed in safe Rust. The borrow checker cannot prove the reference stays valid after the struct is moved, because moving the struct would invalidate the internal pointer. The compiler will reject it.

- Do not attempt self-referential structs with lifetime hacks; they are fundamentally unsound to move.
- If you need the pattern (caches, parsers, arena-backed trees), use an arena (`typed-arena`, `bumpalo`), an index-based representation (store indices, not references), `Pin` (for self-referential futures, carefully), or `owning_ref`/`ouroboros` crates (which use unsafe internally and have subtle constraints).
- Recognize the smell: a struct field of type `&'a T` where `'a` is tied to another field of the same struct. Redesign to avoid the borrow.

### Keep Iterator And Callback Borrows Short

Borrow-checker friction is highest where borrows span many statements or cross abstraction boundaries. Two common cases:

- **Iterator borrowing**: collecting references while mutating the underlying collection fails because the mutable borrow conflicts with the iterator's shared borrow. Restructure by collecting owned values first, then mutating, or by indexing.
- **Callbacks capturing references**: a callback that captures a reference to local data and outlives the local's scope fails. Bound the callback's lifetime to the data, or clone the data into the closure.

The general principle: prefer borrows that are short and local. When a borrow must be long-lived, make the lifetime relationship explicit and verify the data truly lives that long. Long borrows across function boundaries are where the checker earns its keep and where most real bugs live.

### Design APIs So Callers Do Not Need To Understand Your Lifetimes

A well-designed API hides lifetime complexity. If every caller of your function must write a five-lifetime type signature, the abstraction is leaking its internals.

- Return owned values (`String`, `Vec<T>`) when callers will store the result long-term; return references only for transient, scoped use.
- Use `Cow<'a, str>` when a function sometimes borrows and sometimes owns, so callers get a uniform type.
- Provide convenience constructors that take owned input when the alternative is a lifetime-laden API.
- Put complex lifetimes behind a `struct` with a clear name, so callers refer to the struct, not its lifetime parameters.

## Common Traps

### Cloning To Silence A Borrow Error

A clone makes the error go away but hides the real issue: usually unclear ownership or a borrow that genuinely cannot be valid. Before cloning, understand why the borrow failed. Clone small values deliberately; do not reflexively clone to escape the checker.

### One Lifetime Parameter For Everything

Writing `'a` on every reference in a signature to make it compile binds unrelated lifetimes together and over-constrains callers. Use distinct lifetimes for unrelated references; let elision handle the simple cases.

### Treating `'static` As A Fix-All

Annotating a reference as `'static` to satisfy a bound usually means the value does not actually live forever. `'static` is appropriate for string literals, global constants, and leaked data, not for borrowed local data. Forcing `'static` on a borrowed value is unsound and the compiler will catch the attempt.

### Misreading Variance As A Compiler Bug

When `&'static T` works but `&'short T` does not (or vice versa), or when a mutable reference rejects a subtype, variance is at play. Do not assume the checker is wrong; trace the variance of each type parameter.

### Self-Referential Lifetimes In Structs

A struct like `struct S<'a> { data: String, ref_to_data: &'a str }` where `ref_to_data` points into `data` cannot be constructed soundly. The compiler rejects it. Redesign with indices, an arena, or ownership restructuring.

### Holding A Mutable Borrow Too Long

`let m = &mut vec; ... use vec ...` fails because the mutable borrow lives until its last use. Restrure by narrowing the borrow's scope (introduce a block, or rebind) so the borrow ends before the conflicting access.

### Expecting NLL To Save You Across Function Calls

Non-lexical lifetimes (NLL) shortened borrows within a function, but they do not change how lifetimes cross function boundaries or how they interact with struct fields. A borrow returned from a function still lives as long as the signature says.

## Self-Check

- [ ] Lifetimes are named only when elision is insufficient; the annotation expresses a real borrowing relationship, not a guess.
- [ ] Distinct borrowing relationships use distinct lifetime parameters; unrelated references are not bound to the same lifetime just to compile.
- [ ] The variance of each type parameter is understood; invariant positions (`&mut T`, `Cell`) are not "fixed" with unsafe or transmute.
- [ ] No self-referential structs are attempted; the pattern is replaced with arenas, indices, or owning structures.
- [ ] Returned references borrow from a specific, named input, and the lifetime relationship is correct (the output does not outlive what it borrows).
- [ ] `'static` is used only for genuinely static data (literals, globals, leaks), not as a way to satisfy a bound on borrowed data.
- [ ] Iterator and callback borrows are short and local; long-lived borrows are explicit and verified.
- [ ] The API hides lifetime complexity from callers; they do not need to write complex lifetime signatures to use it.
- [ ] Every clone that silenced a borrow error was investigated; the ownership story is clear, not papered over.
- [ ] Lifetime errors were read as information about which reference outlives which, not treated as obstacles to bypass.
