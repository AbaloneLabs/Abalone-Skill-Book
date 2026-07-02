---
name: ruby_blocks_procs_and_lambdas.md
description: Use when the agent is writing or reviewing Ruby code involving blocks, yield, Proc objects, lambda, the ampersand operator, method-to-proc conversion, closure capture, return and break semantics inside blocks, argument arity checking, defining methods that accept or return callable objects, or choosing between a block, a proc, and a lambda for a given abstraction.
---

# Blocks, Procs, And Lambdas In Ruby

Ruby has three related but distinct ways to package executable code — blocks, Procs, and lambdas — and they differ in two load-bearing ways: how `return` and `break` behave inside them, and how strictly they check argument count. These differences are not academic. A `return` inside a block or Proc returns from the enclosing method (and can produce a surprise or, in some contexts, a `LocalJumpError`), while a `return` inside a lambda returns only from the lambda. A Proc silently accepts the wrong number of arguments; a lambda raises `ArgumentError`. Choosing the wrong callable type turns straightforward code into a debugging puzzle, and the puzzle is worse because the failure is often delayed until a specific call site is hit.

Agents tend to treat blocks, Procs, and lambdas as interchangeable "closures" and pick based on syntax convenience. The harm is that the choice changes control flow and error behavior in ways that are invisible at the call site. A method that yields to a block and expects the block to return a value behaves differently when passed a Proc whose `return` escapes the method. A refactored iterator that captures a Proc with a `break` can raise where the original block did not. The judgment problem is to know, for each piece of deferred code, what its return and argument semantics must be, and to pick the construct whose semantics match — and to recognize when the Rails-idiomatic block style is the right call and when an explicit lambda makes the contract clearer.

## Core Rules

### Know The Three Constructs And Their Two Distinguishing Axes

Ruby packages code in three forms, and the differences matter at every call site.

- **Block:** an anonymous chunk of code passed to a method (after the arguments, between `do...end` or `{...}`). A method yields to it with `yield`. A block is not an object and cannot be stored in a variable directly; it is converted to a Proc via `&`. Every method has an implicit block.
- **Proc:** an object (instance of `Proc`) wrapping a block, created with `Proc.new { }` or `proc { }`. It can be stored, passed around, and called multiple times.
- **Lambda:** a stricter Proc created with `lambda { }` or the `->` stabby literal. It is still a Proc (`lambda?.class == Proc`) but with different return and arity semantics.

The two axes that distinguish them:

- **Return/break behavior:** `return` inside a Proc or block returns from the enclosing method (the method that the block was defined in lexically). `return` inside a lambda returns only from the lambda. `break` inside a block returns from the method that yields to the block; inside a Proc called directly it can raise `LocalJumpError`.
- **Arity strictness:** a Proc is lenient — it ignores extra arguments and pads missing ones with `nil`. A lambda is strict — it raises `ArgumentError` on the wrong number of arguments.

### Match The Callable Type To The Required Return Semantics

The return behavior is the most consequential difference and the most common source of bugs.

- Use a **block** when the code is a transformation or callback handed to a single method (`map`, `each`, `select`, `tap`). The block returns a value to the method via the last expression; `return` inside it escapes the enclosing method, which is rarely what you want and usually a bug.
- Use a **lambda** when you want a self-contained callable whose `return` stays inside it — callbacks stored for later, strategies passed to a method, handlers in a dispatch table. A lambda behaves like an anonymous method.
- Use a **Proc** only when you specifically want lenient arity and you understand the return-escapes-method semantics. Most code that reaches for `Proc.new` should use a lambda instead.

Strong choice: `submit_btn.on_click -> { save_form }` (lambda, self-contained). Weak choice: `handler = Proc.new { return save_form }` whose `return` escapes whatever method later calls `handler.call`.

### Use yield For The Common Case, & For When You Need The Block As An Object

Most methods that take a block should use `yield` directly — it is faster (no Proc allocation) and idiomatic. Convert the block to a Proc with `&` only when you need to store it, pass it to another method, or introspect it.

- `def map_each; yield x; yield y; end` — yield is the idiomatic way to invoke the implicit block.
- `def store_callback(&block); @callbacks << block; end` — the `&` captures the block as a Proc you can store.
- `array.map(&method(:to_s))` — the `&` converts a method to a Proc to pass where a block is expected. `method(:name)` returns a Method object; `&` converts it.
- `&` also converts a symbol to a proc via `&:to_s`, which is shorthand for `{ |x| x.to_s }`. Useful for short transformations, but avoid stacking `&:method` calls where a block reads better.

### Check Arity When It Matters, Trust Lambda Strictness

Because Procs accept any argument count silently, a Proc with the wrong arity fails subtly (nil arguments, dropped arguments) rather than loudly. Lambdas fail loudly, which is usually what you want for stored callbacks.

- For stored callbacks and strategy objects, prefer lambdas so an incorrect call site raises immediately.
- If you accept a Proc and its arity matters, check `proc.arity` before calling, or document the expected arity and trust callers.
- Be aware that `proc { |a, b| }` and `lambda { |a, b| }` report different arity for optional/default args; `arity` returns a negative number to signal optional args.

### Be Deliberate About Closure Capture

Blocks, Procs, and lambdas all close over the variables visible at their definition site (lexical scope), and they capture the variable, not its value at definition time. Mutating the captured variable after defining the closure affects what the closure sees.

- A closure created in a loop that captures the loop variable sees the variable's final value unless you bind it per-iteration (Ruby 1.9+ creates a new binding per block invocation for block parameters, but variables from the enclosing scope are shared).
- Closures that capture large or mutable objects extend those objects' lifetimes and can cause surprising mutations; if a closure outlives its intended scope, the captured state persists.
- For callbacks that must snapshot a value at definition time, capture it as a block parameter (`proc { |captured_x| ... }.call(current_x)`) or freeze/dup it.

### Return Values And The Last Expression

Ruby methods and blocks return the value of their last expression. This is powerful but easy to misuse: a block intended for side effects that accidentally returns a value can change behavior when the method uses the block's return value (e.g., `map` uses the return value; `each` ignores it).

- Be explicit about whether a block's return value is consumed. `each` ignores it; `map`, `select`, `reduce`, `max_by` consume it. A block written for `each` that ends with a `puts` returns `nil`, which is fine for `each` but wrong for `map`.
- Prefer `tap` (returns the receiver) for side-effect chains and `then`/`yield_self` for transformation chains, to make the return intent explicit.

## Common Traps

### return Inside A Block Or Proc Escaping The Method

`def foo; [1,2].each { |x| return x if x == 2 }; :normal; end` returns `2`, not `:normal`, because `return` inside the block returns from `foo`. This is occasionally useful and frequently a bug, especially when the block is later refactored into a stored Proc and the `return` raises `LocalJumpError` because there is no enclosing method to return from. Use a lambda, or restructure to avoid `return` in blocks.

### Proc Lenient Arity Hiding Bugs

`proc { |a, b| a + b }.call(1)` returns `1 + nil` (TypeError) rather than an argument error, because the Proc silently padded `b` with nil. A lambda would raise `ArgumentError` immediately, pointing at the real bug. Prefer lambdas for stored callables.

### Storing A Block And Calling It After Its Method Returned

A Proc captures the binding of its definition scope. If you store a Proc that references locals of a method and call it after that method returned, the captured locals persist (the binding is kept alive). This is usually fine but can leak memory or hold references longer than intended; be aware when storing callbacks long-term.

### break Inside A Block Raising LocalJumpError

`break` inside a block returns from the method that yields to the block. If the block is converted to a Proc and called outside a yielding method, `break` raises `LocalJumpError`. Avoid `break` inside Procs you store or pass around; use it only in blocks yielded to by iterators.

### Confusing method(:x) With a Symbol to_proc

`&:to_s` is shorthand for a proc that sends `to_s` to its argument. `&method(:to_s)` converts the actual method object `to_s` (of `self`) to a proc. They behave differently when `self` matters: `&method(:puts)` calls `puts` on the original `self`; `&:puts` would call `.puts` on each element. Know which you mean.

### Using A Block For Side Effects Inside map

`items.map { |x| puts x }` returns an array of nils (the return value of `puts`), which is almost never intended. Use `each` for side effects, `map` for transformations. Mixing them silently produces `[nil, nil, nil]` where a transformed array was expected.

### Overusing &method And Symbol-to-proc For Readability

`[1,2,3].map(&:to_s).map(&:upcase)` chains symbol-to-proc calls that read as a stack of method sends. For more than one step, a block (`map { |n| n.to_s.upcase }`) is usually clearer. Symbol-to-proc shines for single, obvious transformations.

## Self-Check

- [ ] The callable type (block, Proc, lambda) was chosen for its return and arity semantics, not for syntax convenience; stored callbacks and strategy objects are lambdas.
- [ ] No `return` inside a block or Proc escapes an enclosing method unintentionally; where self-contained return is needed, a lambda is used.
- [ ] No `break` appears inside a Proc that is stored or passed around outside a yielding method, where it would raise `LocalJumpError`.
- [ ] Arity-sensitive callables are lambdas (strict) or have their arity checked, so wrong argument counts fail loudly rather than padding with nil.
- [ ] Methods use `yield` for the common block case and `&` only when the block must be stored, forwarded, or introspected.
- [ ] Closure capture is deliberate: loop-captured variables are bound per-iteration where needed, and long-lived callbacks do not unintentionally extend large or mutable captured state.
- [ ] Block return values are consumed only where intended (`map`, `select`, `reduce`) and ignored only where appropriate (`each`); side-effect blocks are not used inside value-consuming operators.
- [ ] `tap` and `then`/`yield_self` are used to make return intent explicit in chains, rather than relying on the last-expression convention where it is ambiguous.
- [ ] Symbol-to-proc and `&method(:x)` are used for single clear transformations, not stacked into unreadable chains where a block would be clearer.
- [ ] The chosen construct's behavior under refactoring (block extracted to stored Proc, method extracted to lambda) has been considered and remains correct.
