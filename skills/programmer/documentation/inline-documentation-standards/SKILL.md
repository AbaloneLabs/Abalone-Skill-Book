---
name: inline_documentation_standards.md
description: Use when the agent is writing code comments, doc comments, API documentation strings, inline explanations, deciding what to comment and what to leave implicit, generating reference docs from source, writing code examples in documentation, or reviewing whether existing comments add value or have gone stale.
---

# Inline Documentation Standards

Inline documentation is the layer of explanation that lives with the code, and it is consistently done badly in two opposite directions. The first failure is no comments at all, on the theory that "good code is self-documenting," which is true for what the code does but false for why it does it, what invariants it assumes, and what it deliberately does not handle. The second failure is over-commenting: restating the code in English ("increment i by one"), documenting abandoned approaches, or leaving comments that drift from the code until they actively mislead. Both failures leave the next reader worse off, and both come from not having a clear theory of what a comment is for.

The judgment problem is that a comment's value is determined by what the code alone cannot convey. Code expresses mechanism perfectly; it expresses intent, constraints, history, and non-obvious decisions poorly. The comments worth writing are the ones that carry information the reader cannot recover from the code: why this approach over the obvious alternative, what invariant must hold for this to be safe, what external contract is being satisfied, what edge case was considered and deliberately not handled. Comments that restate mechanism are noise; comments that explain intent and constraints are load-bearing. The agent must develop the judgment to tell them apart and must treat comment maintenance as part of code maintenance, because a stale comment is worse than no comment.

## Core Rules

### Comment the why, not the what

The single most important distinction. Code already tells the reader what is happening line by line. A comment that restates the mechanism ("set timeout to 30 seconds") adds nothing and will rot. A comment that explains the reason ("30s, not the default 10s, because upstream batches can take up to 25s under load; lowering this caused spurious retries in incident #1234") carries information no reading of the code could recover. Before writing a comment, ask whether the reader could derive it from the code; if yes, delete it. Write only what the code cannot say.

### Use doc comments to define contracts, not to narrate implementation

Doc comments (the structured comments that generate API reference docs) serve a different purpose than inline comments: they define the contract a caller can rely on. A good doc comment specifies parameters (meaning, units, valid ranges, what null/empty means), return values (meaning, failure modes), preconditions (what state must hold), postconditions (what state is guaranteed after), side effects (what else changes), and errors (what exceptions or error values mean and when). It does not describe the implementation, because implementation is not contract. Keep doc comments honest about what is guaranteed versus what is incidental; callers will depend on anything you document, so documenting an incidental behavior makes it a permanent obligation.

### Document invariants, assumptions, and deliberate non-handling

The most valuable inline comments mark non-obvious safety conditions and intentional decisions. Examples: an invariant the code relies on ("caller must hold the lock; this function does not acquire it"), an assumption about input ("we assume ids are monotonic; if that changes, the cache invalidation breaks"), a workaround for an external bug ("upstream returns 200 on failure until v3; check the body, not the status"), or a deliberate non-decision ("we intentionally do not retry here; retries are the caller's responsibility"). These comments prevent future maintainers from "fixing" a safety condition or removing a workaround that exists for a reason. Without them, the code looks simpler than it is.

### Keep comments and code in lockstep; treat stale comments as bugs

A comment that contradicts the code is worse than no comment because it actively misleads. When you change code, update or delete the affected comments in the same change. Reviewers should reject comments that no longer match the code, just as they would reject broken code. The reason comments rot is that teams treat them as optional to maintain; make comment maintenance a reviewable, required part of every change. If you cannot keep a comment accurate, prefer deleting it over leaving it to mislead.

### Write examples that are runnable and correct

Examples in doc comments and READMEs are read as authoritative. An example that does not compile, uses a deprecated API, or shows a wrong pattern propagates the error into every caller's code. Examples must be runnable, tested (ideally as part of CI via doctests or extracted snippets), and kept current with the API. A stale example is a bug shipped to every reader. Prefer one correct, tested example over several untested ones.

### Match comment density to surprise, not to a quota

There is no correct comments-per-line ratio. Dense, obvious code needs few comments; tricky code needs many. Allocate comments where the code is surprising, where a bug was fixed and the fix is non-obvious, where performance depends on a subtle choice, or where a reader will predictably ask "why?" Code that reads as a straightforward expression of intent needs no narration. The goal is that every comment earns its place by adding information the reader would otherwise lack.

### Prefer clarifying the code over commenting the confusion

When you find yourself writing a comment to explain confusing code, first ask whether the code can be made clearer. A rename, an extracted helper, a simpler structure, or a well-named variable often eliminates the need for the comment entirely. Code that needs a long comment to explain is often code that can be restructured to explain itself. Comment only what restructuring cannot fix. The best comment is often the one you did not need to write because you improved the code instead.

### Make documentation generation part of the build

If your language supports doc comments (Rust rustdoc, Javadoc, Python docstrings, JSDoc, Go doc comments), wire generation into CI so the reference docs are never stale relative to the code. Treat broken doc builds (malformed comments, broken cross-references, failing doctests) as build failures. This forces doc comments to stay syntactically correct and cross-referenced, even if it cannot enforce that they stay semantically accurate.

## Common Traps

### Restating the code in prose

Comments that translate each line into English add maintenance burden without insight. If the comment says exactly what the code does, delete it. Comment only what the code does not say.

### Letting comments drift until they contradict the code

A comment updated in 2021 describing code changed in 2023 actively misleads. Stale comments are worse than no comments. Update or delete comments with every code change, and review them as rigorously as code.

### Documenting incidental behavior as if it were contract

Once you document a behavior in a doc comment, callers depend on it and it becomes a constraint. Document only what you intend to guarantee. Incidental details (current implementation limits, internal caching) should not appear in public docs.

### Writing examples that do not compile or use deprecated APIs

Stale examples propagate errors into callers' code. Test every example (doctests, snippet extraction) so a breaking change fails the build rather than shipping a broken example.

### Over-commenting obvious code to hit a quota

Comment-for-comment's-sake produces noise that readers learn to skip, which means they miss the important comments too. Comment by surprise, not by quota.

### Commenting confusion instead of fixing the code

A long comment explaining tangled code is a signal the code should be refactored. Prefer renaming, extraction, and simplification; comment only what remains non-obvious after the code is as clear as you can make it.

### Leaving "TODO" comments that never get resolved

TODOs accumulate forever and become invisible noise. Either attach a ticket and a date, or delete them. A TODO with no owner and no plan is litter, not documentation.

## Self-Check

- Does each comment explain why (intent, constraint, history, deliberate non-handling) rather than restating what the code already says?
- Do doc comments define the caller's contract (parameters, returns, preconditions, postconditions, side effects, errors) rather than narrating implementation details?
- Are invariants, assumptions, workarounds for external bugs, and deliberate non-decisions documented so future maintainers do not remove safety conditions?
- Are comments updated or deleted in the same change that modifies the code, and do reviewers reject comments that contradict the code?
- Are examples in doc comments and READMEs runnable, tested (doctests or extracted snippets), and current with the API, rather than stale and misleading?
- Is comment density matched to where the code is surprising, with no quota-driven noise on obvious code?
- Before adding a comment to explain confusing code, did you first try to clarify the code (rename, extract, simplify) so the comment became unnecessary?
- Is doc generation wired into CI so broken or stale doc comments fail the build, and are cross-references kept valid?
- Are TODO comments either ticketed with an owner and date, or removed, rather than left to accumulate as permanent litter?
