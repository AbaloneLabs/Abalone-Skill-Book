---
name: exception_safety_and_raii.md
description: Use when the agent is writing exception-safe code, reasoning about basic versus strong versus nothrow guarantees, handling stack unwinding and resource cleanup, managing partial-failure state during multi-step operations, or designing constructors and destructors that may throw in RAII style.
---

# Exception Safety and RAII

Exception safety is the discipline of ensuring that when an operation fails partway through, the program is left in a valid state with no leaked resources and no corrupted invariants. It is easy to write code that works when nothing throws; the hard part, and the part that matters in production, is what happens when an allocation fails, a copy throws, or a dependency errors on the third step of a five-step operation. In languages with deterministic destruction (C++, Rust's panic unwinding), RAII ties resource lifetimes to scopes so cleanup happens automatically, but RAII is not magic: a type whose constructor throws after acquiring some resources, or a destructor that throws during stack unwinding, can still leak or terminate.

The judgment problem is knowing which exception-safety guarantee each operation provides, structuring multi-step operations so a failure rolls back partial work, and writing constructors and destructors that do not undermine RAII. The agent should not treat "it compiles and the happy path works" as sufficient; exception safety is about the unhappy paths.

This skill applies whenever you are writing or reviewing resource-managing code, multi-step mutating operations, or types with non-trivial construction and destruction in languages with exceptions or panic unwinding.

## Core Rules

### Know the three exception-safety guarantees and design to a chosen one

Exception safety is not binary. The standard taxonomy (from the C++ community, but applicable broadly) defines three levels:

- **Basic guarantee**: if an operation fails, the program remains in a valid (but possibly changed) state, and no resources are leaked. Invariants hold; the object is usable, though its exact state may differ from before.
- **Strong guarantee**: if an operation fails, the program state is rolled back to exactly what it was before the operation began (commit-or-rollback semantics). The operation either fully succeeds or has no observable effect.
- **Nothrow / nofail guarantee**: the operation cannot fail (cannot throw, cannot report an error). Required for destructors and certain operations invoked during unwinding.

Decide which guarantee each operation provides, and document it. Not every operation needs the strong guarantee (it can be expensive), but every operation must provide at least the basic guarantee. Weak code provides neither, leaking resources and breaking invariants on failure.

### Tie resource lifetimes to scope with RAII

Resource Acquisition Is Initialization means a resource is acquired in a constructor and released in a destructor, so that exiting a scope (normally or via an exception) always releases the resource. This is the primary defense against leaks:

- Wrap every owned resource (memory, file handles, sockets, locks, database connections) in an RAII type whose destructor releases it.
- Never hold a raw resource across code that can throw; acquire it directly into its managing object.
- Prefer standard/library RAII types (`unique_ptr`, `Box`, `MutexGuard`, `defer`, `using`, `try-with-resources`) over hand-rolled cleanup.

The benefit is that cleanup is automatic and cannot be forgotten when an early return or exception is added later.

### Use the copy-and-swap idiom for the strong guarantee

To provide the strong guarantee for a mutating operation, do all the work that can fail on a copy or temporary, then swap the result into place with an operation that cannot fail:

1. Make a copy of the data (or build the new state in a temporary).
2. Perform the mutating work on the copy/temporary. If any step throws, the original is untouched.
3. Swap the finished result into the target using a nothrow swap.

This gives commit-or-rollback semantics. The cost is the extra copy, which is why the strong guarantee is not free and is reserved for operations where rollback matters.

### Ensure constructors either fully construct or clean up

A constructor that acquires multiple resources must not leak if a later acquisition fails:

- Acquire each resource into its own RAII member immediately, so that if the Nth acquisition throws, the already-constructed members' destructors run and release their resources.
- Avoid acquiring resources into raw pointers/handles in the constructor body and assigning them to members later; a throw in between leaks.
- Prefer member initializers and composition of RAII types over manual acquisition ordering.

The rule of thumb: a constructor should either complete (producing a fully-formed object) or throw, with all partially-acquired resources already released by the RAII members constructed so far.

### Never let destructors throw

Destructors that throw are catastrophic, because they may run during stack unwinding (when another exception is already in flight), and a second exception during unwinding calls `std::terminate` (or the language equivalent). Rules:

- Destructors must be nothrow. If cleanup can fail, log/swallow the error in the destructor and surface it through a separate `close()`/`shutdown()` method if the caller needs to know.
- Destructors that release resources (close a file, release a lock) must not fail in a way that escapes; a failed close should not prevent other destructors from running.
- In languages with panic-on-drop or similar, the same principle applies: dropping must not introduce a second failure.

### Handle partial failure in multi-step operations

When an operation performs several steps that each mutate state, a failure midway leaves the system in a partial state. Strategies:

- **Defer mutations until all work is done**: build the result in temporaries, then commit in one nothrow step (copy-and-swap). This gives the strong guarantee.
- **Compensating actions**: if you cannot build in a temporary, record each step and apply inverse operations on failure (a saga). This gives only the basic guarantee (state is valid but changed).
- **Transactions**: wrap the steps in a database or in-memory transaction that rolls back on failure.

Weak choice: mutating state directly in each step with no rollback, so a failure on step 3 leaves steps 1 and 2 applied. Strong choice: build-then-commit or transactional rollback.

### Distinguish recoverable errors from programming errors

Not all failures should be exceptions:

- **Recoverable errors** (file not found, network timeout, invalid user input) are expected outcomes the caller may handle. Use return values, Result/Option types, or checked exceptions.
- **Programming errors** (null dereference, index out of bounds, violated invariant) represent bugs. Use assertions or unrecoverable panics; catching them encourages continuing in a corrupt state.

Reserve exceptions for truly exceptional, recoverable conditions. Using exceptions for normal control flow obscures real errors and complicates exception-safety reasoning.

## Common Traps

### Holding raw resources across throwing code

Acquiring a resource into a raw pointer, then calling code that can throw before assigning it to a managing object, leaks the resource on throw. Acquire directly into the RAII type.

### Destructors that throw

A throwing destructor during stack unwinding terminates the program. Any cleanup that can fail must be swallowed or moved out of the destructor.

### Mutating state step-by-step with no rollback

A multi-step operation that applies each mutation directly, with no build-then-commit or compensating actions, leaves a partial state on failure. This breaks invariants silently.

### Confusing the basic and strong guarantees

Documenting or assuming the strong guarantee when only the basic guarantee is provided leads callers to rely on rollback that does not happen. State which guarantee each operation actually provides.

### Constructors that acquire into raw members

A constructor that opens three files into raw handles and throws on the fourth leaks the first three. Acquire into RAII members so partial construction cleans up.

### Using exceptions for normal control flow

Throwing to signal an expected condition (end of iteration, not-found) obscures real exceptions and forces every caller to be exception-safe against non-exceptional paths. Use return values for expected outcomes.

### Assuming RAII makes cleanup free

RAII makes cleanup automatic, but destructors still run and still cost. Tight loops that create and destroy many RAII objects can be slower than reusing them. RAII is about correctness, not zero cost.

### Forgetting that move/swap must be nothrow

The strong guarantee relies on the final commit step being nothrow. If the swap or move used for commit can throw, the guarantee is lost. Ensure commit operations are nothrow.

## Self-Check

- Does each operation document which exception-safety guarantee it provides (basic, strong, nothrow), and is at least the basic guarantee always met?
- Are all owned resources held in RAII types whose destructors release them, with no raw resources held across throwing code?
- For operations requiring the strong guarantee, is the copy-and-swap or build-then-commit pattern used so a failure rolls back all changes?
- Do constructors acquire resources into RAII members so that partial construction cleans up on throw?
- Are all destructors nothrow, with any failing cleanup swallowed or moved to a separate close/shutdown method?
- For multi-step mutating operations, is there a defined rollback strategy (build-then-commit, transaction, or compensating actions) rather than direct mutation with no recovery?
- Is the final commit/swap step guaranteed nothrow so the strong guarantee actually holds?
- Are exceptions reserved for exceptional recoverable conditions, with expected outcomes signaled via return values?
- Have you considered what happens when each individual step of a multi-step operation fails, not just the happy path?
- For resource-managing types, have you verified that copy, move, and swap operations preserve the intended guarantee?
