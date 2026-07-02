---
name: ruby_metaprogramming_and_dsl.md
description: Use when the agent is writing or reviewing Ruby code using method_missing, define_method, eval, send, public_send, const_get, class_eval, module_eval, instance_variable_get/set, respond_to_missing, Rails-style dynamic finders and callbacks, or designing a DSL where methods are synthesized, delegated, or evaluated at runtime, and where the readability, debuggability, and hidden-complexity costs must be weighed against the convenience.
---

# Metaprogramming And DSL Design In Ruby

Ruby's metaprogramming — defining methods at runtime, intercepting unknown messages with `method_missing`, evaluating strings with `eval`, reflecting over classes and instance variables — is what makes Rails feel magical and what makes Rails-era codebases hard to debug. The magic is real: a few lines of `method_missing` and `define_method` can generate dozens of methods, implement a fluent DSL, or wire up a plugin system. The cost is also real: those methods do not show up in `grep`, they do not appear in documentation, their definitions are scattered and conditional, and when one breaks the stack trace points at `method_missing` with no indication of which dynamic method was intended. Metaprogramming trades explicit, greppable, debuggable code for concise, flexible, opaque code.

Agents reach for metaprogramming because it feels powerful and idiomatic in Ruby, especially under Rails influence. The judgment problem is to recognize that metaprogramming is a tool with steep costs, to prefer explicit definitions when they can do the job, to implement `method_missing` correctly (with `respond_to_missing?`, visibility, and a clear failure mode) when it is genuinely needed, to avoid `eval` for any input that is not fully trusted, and to weigh whether a DSL's readability win is worth the loss of navigability. The harm of excessive metaprogramming is a codebase where no one can find where a method is defined, where IDEs and documentation tools are blind, and where a typo in a dynamic method name fails at runtime in production instead of at compile/load time.

## Core Rules

### Prefer Explicit Definitions Over Generated Ones

If a method can be defined explicitly with `def`, define it explicitly. Generated methods (`define_method`, `method_missing`-synthesized methods) are invisible to `grep`, to documentation tools, to IDE jump-to-definition, and to readers scanning a class body. They are appropriate when the number or shape of methods is genuinely dynamic (config-driven, plugin-driven, reflection-driven), not as a way to avoid typing out a handful of methods.

- A class with five related methods should define them with `def`, not generate them in a loop "to stay DRY." The duplication is cheaper than the indirection.
- Generated methods are justified when the set is data-driven and large (e.g., generating accessors from a schema, wrapping every column of a database table), where explicit definitions would be unmaintainable.
- Even when generating, prefer `define_method` (which creates real methods visible to introspection) over `method_missing` (which only intercepts calls), so the methods appear in `instance_methods`, `respond_to?`, and documentation.

### Implement method_missing Correctly, Or Not At All

`method_missing` is the most overused and most often misimplemented metaprogramming hook. When you do use it, three things must accompany it.

- **Override `respond_to_missing?`** to match. If `method_missing` handles `find_by_name`, then `respond_to?(:find_by_name)` must return true; otherwise duck-typing checks, mocks, and serialization break. Forgetting `respond_to_missing?` is the canonical `method_missing` bug.
- **Call `super` for unhandled methods.** A `method_missing` that does not call `super` for methods it does not handle swallows `NoMethodError` for genuinely missing methods, turning typos into silent `nil` returns or confusing failures. Always fall through to `super` for anything you do not recognize.
- **Keep the pattern narrow and documented.** `method_missing` should match a specific, documented pattern (e.g., methods starting with `find_by_`), extract the dynamic part, and dispatch. A catch-all `method_missing` that tries to "do something smart" for any method is a debugging nightmare.

Strong choice: `define_method(:find_by_#{attr})` for a known finite set, generating real methods. Weak choice: `method_missing` that pattern-matches `find_by_*` at call time, invisible to introspection.

### Avoid eval For Untrusted Or Composed Input

`eval` executes a string as Ruby code. It is powerful, dangerous, and almost always the wrong tool.

- Never `eval` input that comes from a user, a file, a config, a network source, or any string composed from external data. It is a code-injection vulnerability identical in kind to SQL injection.
- Even for trusted input, `eval` defeats static analysis, hides the evaluated code from `grep`, and produces stack traces that point at the `eval` line rather than the evaluated code. Prefer block-based alternatives (`class_eval { ... }` with a block, not a string; `instance_eval`, `define_method` with a block).
- The legitimate uses of `eval` are narrow: loading trusted code from a trusted source, implementing a REPL, or rare DSL cases where the input is genuinely source code. Document why `eval` is necessary and why no alternative works.

Use the block form of `class_eval`/`module_eval`/`instance_eval` instead of the string form wherever possible; the block is real code that the analyzer sees.

### Use send And public_send With Visibility In Mind

`send` calls any method including private ones; `public_send` enforces visibility by raising on private/protected methods. Choosing between them is a visibility decision.

- Use `public_send` when you want to respect encapsulation — calling a method whose name comes from external data, where private methods should not be reachable.
- Use `send` deliberately when you need to reach a private method (testing internals, framework integration), and document why.
- Be cautious with `send` driven by external strings; it is a smaller injection surface than `eval` but still lets a caller reach private or dangerous methods.

### Weigh The Hidden Complexity Of Rails-Style Callbacks And DSLs

Rails `before_save`, `has_many`, `validates`, scopes, and concern modules are metaprogramming under the hood, and they hide substantial complexity behind readable declarations.

- Callbacks (`before_save`, `after_create`) make execution order implicit and scattered; a model with many callbacks has control flow that no single method reveals. Prefer explicit service objects for complex lifecycles, and keep callbacks short and side-effect-light.
- `has_many :through`, polymorphic associations, and default scopes change query semantics invisibly; a `default_scope` on a model silently alters every query of that model, including ones far from the definition. Use default scopes rarely and document them loudly.
- Concerns that mix in callbacks, validations, and scopes spread a model's behavior across files; the model reads as small but actually has behavior defined in five concerns. Audit concerns for hidden coupling.

The readability win of these DSLs is real for standard cases; the cost is that the implicit behavior must be learned, documented, and audited, especially when it stops being standard.

### Make DSLs Greppable And Documented

When you design a DSL (configuration blocks, fluent builders, declarative APIs), the DSL methods should be discoverable: defined with `define_method` so they appear in introspection, listed in documentation, and named consistently so a reader can find them.

- A DSL whose methods exist only via `method_missing` is invisible; readers cannot find what the DSL accepts. Generate real methods for the known vocabulary, and use `method_missing` only for genuinely open-ended extension.
- Document the DSL's vocabulary and its evaluation context (what `self` is inside the block, what methods are available). A DSL whose context is undocumented forces readers to read the implementation to use it.
- Keep DSL evaluation predictable: avoid `instance_eval` that changes `self` unless the DSL genuinely needs it, because it breaks access to the caller's instance variables and methods, surprising users.

## Common Traps

### method_missing Without respond_to_missing?

The most common `method_missing` bug. `obj.respond_to?(:dynamic_method)` returns false even though `obj.dynamic_method` works, breaking duck-typing, mocks (`allow(obj).to receive(:dynamic_method)`), and serialization. Always pair them.

### method_missing That Does Not Call super

A `method_missing` that handles only its intended pattern but forgets `super` for everything else turns genuine `NoMethodError`s (typos, wrong receiver) into silent nils or confusing behavior. Always fall through to `super`.

### eval On Composed Or External Strings

`eval(params[:expr])`, `eval(user_input)`, or `eval("...#{var}...")` is a code-injection vulnerability. Even `eval` of trusted-looking config is fragile and hides code from analysis. Use block forms or data-driven approaches instead.

### send With External Input Reaching Private Methods

`obj.send(params[:action])` lets a caller invoke any method, private or dangerous (`send(:system, ...)`). Use `public_send` and an allowlist of permitted method names when the method comes from external data.

### default_scope Silently Altering Every Query

A `default_scope` on a Rails model changes every query of that model, including `find`, associations, and joins far from the definition. It makes deleted records vanish, changes counts, and breaks queries that expect unscoped access. Use it rarely and unscope explicitly where needed.

### Callbacks Spreading Side Effects Across A Lifecycle

A model with `before_save`, `after_create`, `after_commit` callbacks that send mail, enqueue jobs, and update associated records has control flow that no method body reveals. Bugs in callbacks are hard to trace and easy to introduce by adding a callback that runs at the wrong time. Keep callbacks minimal; move orchestration to service objects.

### Concerns Hiding Coupling

A concern that adds callbacks, validations, and associations to a model makes the model look small while spreading its behavior across files. The model's actual behavior is the union of itself and all included concerns; readers must open every concern to understand it. Document included concerns' effects.

### instance_eval In A DSL Surprising The Caller

A configuration DSL that uses `instance_eval` to change `self` inside the block breaks the caller's access to its own instance variables and methods, which is surprising. Prefer `yield self` (explicit receiver) or `instance_exec` unless changing `self` is the point of the DSL.

## Self-Check

- [ ] Methods are defined explicitly with `def` or `define_method` wherever the set is finite; metaprogramming is reserved for genuinely dynamic, data-driven, or plugin-driven method sets.
- [ ] Every `method_missing` is paired with a matching `respond_to_missing?`, calls `super` for unhandled methods, and matches a narrow documented pattern.
- [ ] No `eval` operates on untrusted, external, or composed strings; block forms of `class_eval`/`instance_eval` are used wherever possible, and any `eval` is justified and documented.
- [ ] `send` is used deliberately to reach private methods only with a stated reason; `public_send` plus an allowlist is used when the method name comes from external data.
- [ ] Rails callbacks are short, side-effect-light, and ordered deliberately; complex lifecycles are moved to explicit service objects rather than hidden in callback chains.
- [ ] `default_scope` is used rarely and documented loudly; queries that need unscoped access use `unscoped` explicitly.
- [ ] DSL vocabulary is generated as real methods (visible to introspection and documentation) where finite, with `method_missing` reserved for open-ended extension; the DSL's evaluation context and available methods are documented.
- [ ] `instance_eval` is used in a DSL only when changing `self` is genuinely the point; otherwise `yield self` or `instance_exec` preserves the caller's context.
- [ ] No dynamic method or callback introduces behavior that fails only at runtime in production; load-time generation is preferred so errors surface early.
- [ ] The readability and conciseness win of each metaprogramming use has been weighed against the loss of greppability, debuggability, and IDE/documentation support, and the tradeoff is justified.
