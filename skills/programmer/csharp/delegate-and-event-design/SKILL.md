---
name: delegate_and_event_design.md
description: Use when the agent is designing delegate types, events, multicast invocations, callback APIs, pub/sub systems, async event handlers, weak event patterns, or diagnosing event subscription leaks, handler ordering issues, and reentrancy bugs in C# and .NET applications.
---

# Delegate and Event Design

Delegates and events are the primary extensibility mechanism in C#, and they are deceptively easy to use incorrectly. The language makes `event SomeHandler += handler` look like harmless syntax, but each subscription is a strong reference with ordering semantics, lifetime implications, and reentrancy risks that compound as a codebase grows. The judgment problem is not "how do I declare an event" but how to design callback contracts that do not leak memory, that tolerate handler exceptions and ordering, and that remain safe when handlers are slow, async, or reentrant.

Most delegate/event bugs share a root cause: the designer assumed handlers are fast, well-behaved, and short-lived, then reality delivered slow handlers, throwing handlers, and subscribers that outlive the publisher. A multicast delegate invocation that throws in the middle leaves later handlers uninvoked. An event on a long-lived object held by a short-lived subscriber leaks the subscriber until the publisher dies. An async void event handler that throws crashes the process. None of these are caught by happy-path testing.

## Core Rules

### Decide delegate type vs event based on who may subscribe and unsubscribe

A public `delegate` field or property lets any caller invoke it and replace it. A public `event` restricts external access to `+=` and `-=` only; only the declaring type can raise it. Use `event` whenever external code should listen but not raise or clear the handler list. Expose a raw delegate only when you genuinely want callers to invoke or reassign it, which is rare.

Prefer the framework-provided `EventHandler<TArgs>` and `Action`/`Func` families over custom delegate types unless the custom name documents intent that the generic cannot. Custom delegate types add a type that callers must learn and that cannot be assigned from a lambda without explicit construction.

### Treat handler ordering as undefined unless you enforce it

Multicast delegate invocation order is the order of subscription in the current implementation, but it is not contractual. If your design requires a specific order (e.g., a logging handler must run before a mutating handler), do not rely on subscription order. Either expose ordered phases as separate events, or maintain an explicit ordered list of handlers and invoke them yourself with documented semantics.

### Decide exception policy before raising

When a multicast invocation throws, handlers after the throwing one are skipped. Decide explicitly:

- Swallow and continue: iterate `GetInvocationList()` yourself, wrap each call in try/catch, aggregate exceptions, and continue invoking the rest. This is usually the right choice for events where one bad handler should not break others.
- Fail fast: let the exception propagate, accepting that later handlers are skipped. Appropriate for critical state-change events where partial invocation leaves inconsistent state.

Never leave this decision implicit. The default (propagate) silently violates the expectation that all subscribers are notified.

### Separate the notification contract from the work

An event handler that does heavy I/O on the raising thread blocks the publisher and every subsequent handler. Decide whether handlers run synchronously on the raising thread or are dispatched asynchronously. For UI or performance-sensitive publishers, raise the event on a lightweight path and let handlers queue their own work. Document which thread/context the event is raised on, because handlers assume.

### Manage subscription lifetime explicitly

Every `+=` needs a planned `-=` or an owning scope. The cleanest patterns:

- The subscriber implements `IDisposable` and detaches in `Dispose`.
- The subscription is scoped to a unit of work that clears handlers on completion.
- Use `WeakReference`-based weak event patterns when the publisher genuinely outlives many short-lived subscribers (e.g., a global message bus).

Do not rely on the subscriber being collected; if the publisher holds it, it will not be.

### Choose weak events deliberately, not by default

Weak event patterns (e.g., `WeakReference<TSubscriber>` event managers, or the WPF `WeakEventManager`) break strong-reference leaks but add complexity and subtle bugs: a subscriber can be collected *while it still wants events* if nothing else roots it, and debugging "why did my handler stop firing" is painful. Use weak events only for genuinely long-lived publishers with many anonymous short-lived subscribers. For owned object graphs, prefer explicit detach.

### Design the event payload for forward compatibility

Once an event is public, its argument shape is a contract. Favored approaches:

- Use a dedicated `XxxEventArgs` class with a set of properties, so you can add optional properties later without breaking subscribers.
- Mark the class `sealed` only if you are certain no one needs to extend it; otherwise leave it unsealed but document.
- Never reuse an existing event argument type for a semantically different event.

Avoid passing mutable shared state through event args unless you intend handlers to mutate it (e.g., cancel flags). Shared mutable args create ordering-dependent bugs.

### Async event handlers need a return type or a dispatcher

`async void` event handlers cannot be awaited by the raiser, propagate exceptions to the SynchronizationContext (often crashing the process), and make it impossible to know when all handlers completed. For async notification, either:

- Use a custom async pattern where handlers return `Task` and the raiser awaits all of them, or
- Fire-and-forget with explicit error logging inside the handler, accepting that the raiser does not track completion.

Do not expose a standard `event` with the expectation that `async void` handlers will be safe.

## Common Traps

### Forgetting to detach and leaking the subscriber

`publisher.Changed += OnChanged;` with no `-=` roots the subscriber for the publisher's lifetime. This is the single most common event bug. Audit every subscription for its detach path.

### Assuming null-conditional raise is enough

`Event?.Invoke(this, args)` avoids a NullReferenceException when there are no subscribers, but it does nothing about handler exceptions, ordering, or slow handlers. It is a necessary guard, not a complete strategy.

### Mutating state in a handler that the raiser then uses

If the raiser reads its own state after raising an event, and a handler mutated that state (directly or indirectly), the raiser's logic becomes dependent on subscriber behavior. Either raise events after the raiser has finished using its state, or pass immutable snapshots in the args.

### Reentrancy during raise

A handler that synchronously calls back into the publisher (e.g., to read more state or unsubscribe) can cause the publisher to raise again or to mutate the invocation list mid-iteration. Guard against reentrant raise, and copy the invocation list before iterating if handlers may unsubscribe.

### Subscribing the same handler twice

`+= handler` with the same delegate instance twice invokes it twice. If your design intends at-most-once subscription, check or use a set. This is easy to miss when subscribing from a constructor that runs more than once or from a factory.

### Using `event` for request/response

Events are for notification, not for requesting a value from a handler. If you need a handler to return data, use a delegate that returns a value, a callback with a result parameter, or a query interface. Events-that-return-values conflate two contracts and break when there are zero or multiple subscribers.

### Static events

A `static event` is a global publisher that roots every subscriber forever unless they detach. Treat static events as a serious lifetime hazard; prefer scoped instances or explicit weak patterns.

## Self-Check

- For every `event` and delegate field, is external access restricted to subscribe/unsubscribe where appropriate, with raising limited to the declaring type?
- Is there an explicit exception policy for multicast invocation? If a handler throws, are the remaining handlers still invoked (or is the skip documented and intentional)?
- Does every `+=` have a matching `-=` or an owning scope that clears the subscription? Trace the lifetime of each subscriber against its publisher.
- Have you documented which thread or context each event is raised on, and are slow handlers expected to dispatch their own work?
- If you use a weak event pattern, have you confirmed subscribers remain rooted by something other than the event, so they are not collected prematurely?
- Are async event handlers avoiding `async void`, or is fire-and-forget with internal error handling an explicit, documented choice?
- Is the event argument type designed to allow adding properties later without breaking existing subscribers?
- Are there any static events, and if so, is the lifetime risk documented and mitigated?
- Have you guarded against reentrancy (handler calling back into the publisher during raise) and duplicate subscription of the same delegate?
