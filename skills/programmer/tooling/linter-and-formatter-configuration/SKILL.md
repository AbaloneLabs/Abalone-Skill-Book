---
name: linter_and_formatter_configuration.md
description: Use when the agent is selecting or configuring a linter or formatter (ESLint, Ruff, golangci-lint, Clang-Tidy, Prettier, Black), choosing lint rules and severity levels, enabling auto-fix, managing exceptions and inline disables, writing custom rules, resolving tool conflicts, or rolling out linting incrementally without blocking existing code.
---

# Linter and Formatter Configuration

Linters and formatters are cheap quality multipliers, but they are also one of the most common sources of pointless friction in a codebase. The failure mode is not usually "we have no linter" but "we have a linter configured badly": rules that fire on stylistic preferences and start religious wars, auto-fix that silently rewrites code in review, exceptions accumulated until the rules mean nothing, or a formatter and a linter that disagree and fight each other on every save. Agents often enable every available rule (because more rules means more quality, the reasoning goes) and discover that the team spends more time arguing about rule config than the rules save.

The judgment problem is that linting and formatting serve different purposes and must be configured with different philosophies. A formatter enforces consistency (one canonical layout) and should be non-negotiable and automatic; a linter catches defects and smells (bugs, security issues, unreachable code) and should be calibrated to the team's tolerance for noise. The agent must decide which rules are worth the friction they impose, separate "this is wrong" (a real defect) from "I prefer this" (a style opinion), and roll out tooling in a way that improves the codebase rather than creating a wall of errors that gets disabled wholesale. A linter that is too strict gets turned off; a linter that is too lenient catches nothing. The skill is finding the calibration that the team will actually live with.

## Core Rules

### Separate formatting (consistency) from linting (defect detection)

These are different problems with different correct approaches. Formatting (Prettier, Black, gofmt, rustfmt) enforces one canonical style and should be automatic, non-negotiable, and run on save and in CI without discussion; it eliminates style debate by removing choice. Linting (ESLint, Ruff, Clang-Tidy) detects potential defects and should be calibrated to the codebase's needs. Do not use a linter to enforce style that a formatter already handles; that duplicates enforcement and creates conflict. Let the formatter own style, and let the linter own correctness.

### Choose rules by value, not by completeness

The temptation is to enable every rule the linter offers. This is a mistake. Each rule imposes a cost (cognitive load, false positives, exceptions to manage) and the value varies enormously. Prioritize rules that catch real defects (unused variables, unreachable code, null dereference patterns, security issues, common bug patterns) over rules that encode style preferences. For each rule, ask: what bug does this prevent, how often does that bug actually occur, and how many false positives does it produce? Disable or lower the severity of rules that fire often but prevent rare or minor problems. A linter with 20 high-value rules that the team respects is more effective than 200 rules that everyone disables.

### Make auto-fix safe and predictable

Auto-fix is powerful but dangerous. A formatter's auto-fix is safe because formatting is semantically neutral. A linter's auto-fix is not always safe; some fixes change behavior, and a fix applied automatically across the codebase can introduce subtle bugs. Enable auto-fix only for rules whose fixes are provably safe (purely syntactic, no semantic change), and require manual review for rules whose fixes could change behavior. Never auto-fix in a way that mixes mechanical changes with logic changes in the same commit, because the mechanical noise hides the logic change in review. Run auto-fix as a separate, reviewable step.

### Manage exceptions explicitly and review them periodically

Some code legitimately violates a rule, and inline disables (`// eslint-disable-next-line`, `# noqa`) are the escape hatch. But exceptions accumulate and erode the rule's value. Require every exception to carry a reason (why is this disabled, what was the alternative considered), make exceptions as narrow as possible (single line, not whole file), and periodically audit the exception list to remove ones that are no longer needed or to fix the underlying issue. A codebase with hundreds of unexplained disables has effectively disabled the linter while keeping the illusion of enforcement.

### Resolve formatter and linter conflicts before they reach developers

A formatter and a linter that disagree (the formatter wants one layout, the linter flags it) create an unwinnable loop: developers fix one, the other reverts it. This is a tooling failure, not a developer failure. Resolve it at the configuration level: either align the rules so they agree, or clearly partition responsibilities (formatter owns layout, linter does not flag what the formatter owns). Test the combination in CI so a conflict is caught before it reaches developers' editors. Never ship a config where the tools fight.

### Roll out linting incrementally on existing codebases

Enabling a strict linter on a large existing codebase produces thousands of errors and the team will disable the rules wholesale to make the build pass. The correct rollout is incremental: start with rules that can be auto-fixed and apply them in a dedicated commit, then enable remaining rules only on new or changed code (via a baseline, a "new rules apply to new code" mode, or per-directory opt-in), and gradually pay down the existing violations over time. This trades a burst of unpayable debt for a sustainable gradient. Do not enable strict rules globally on day one of an existing codebase.

### Write custom rules only when the cost is justified

Most teams do not need custom rules, and custom rules are expensive to write, maintain, and keep correct as the language evolves. Before writing a custom rule, check whether an existing rule or rule set covers the case. Write a custom rule only when it encodes a project-specific invariant that prevents a recurring real defect (e.g., a domain-specific API misuse pattern). Document the rule's purpose, test it against positive and negative cases, and assign it an owner. A custom rule with no owner rots and eventually flags code that is actually correct.

### Enforce in CI, but give fast feedback locally

The linter and formatter must run in CI as a gate, but CI feedback is slow. Configure editors and a pre-commit or pre-push hook to run the same checks locally so developers get feedback immediately, before they push. Keep the local check fast (run only on changed files if the tool supports it) so it does not discourage use. The CI gate is the source of truth; the local check is the fast feedback loop that makes compliance cheap.

## Common Traps

### Enabling every available rule

More rules is not more quality; it is more noise and more exceptions. Each rule should earn its place by preventing a real defect with acceptable false-positive cost.

### Using a linter to enforce style a formatter already handles

This duplicates enforcement and creates conflicts. Let the formatter own style and the linter own correctness.

### Auto-fix that silently changes behavior

A linter auto-fix is not always semantically safe. Enable auto-fix only for provably safe fixes, and never mix mechanical auto-fixes with logic changes in one commit.

### Accumulating unexplained inline disables

Every disable should carry a reason and be as narrow as possible. Periodically audit and remove exceptions that are no longer needed; otherwise the linter is effectively disabled.

### Shipping a formatter and linter that fight

If the tools disagree, developers are caught in a loop. Resolve the conflict in configuration and verify the combination in CI.

### Enabling strict rules globally on an existing codebase

This produces an unpayable wall of errors and the team disables the rules. Roll out incrementally with a baseline and new-code-only enforcement.

### Custom rules with no owner and no tests

Custom rules rot and eventually flag correct code. Write them only for high-value project-specific invariants, test them, and assign ownership.

## Self-Check

- Is formatting handled by a dedicated formatter (automatic, non-negotiable) and separated from linting (defect detection), with no overlap or conflict between the two?
- Was each enabled lint rule chosen for the real defect it prevents, with acceptable false-positive cost, rather than enabling every available rule?
- Is auto-fix enabled only for provably safe (semantically neutral) fixes, with behavior-changing fixes requiring manual review, and never mixed with logic changes in one commit?
- Does every inline disable carry a reason, cover the narrowest possible scope, and get periodically audited so exceptions do not accumulate unexplained?
- Do the formatter and linter agree (no loops where one reverts the other), and is their combination verified in CI?
- For an existing codebase, was strict linting rolled out incrementally (auto-fixable rules first, remaining rules on new code via a baseline) rather than enabled globally on day one?
- Are custom rules limited to high-value project-specific invariants, tested against positive and negative cases, and owned by a named person or team?
- Do the same checks run in CI (as the gate) and locally (as fast feedback, ideally on changed files only), so compliance is enforced but not painful?
