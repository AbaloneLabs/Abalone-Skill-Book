---
name: dependency_range_and_resolution.md
description: Use when the agent is specifying version ranges or constraints for dependencies, reasoning about resolution algorithms and version conflicts, managing lockfiles for reproducibility, updating dependencies safely, handling security advisories and vulnerable transitive dependencies, or choosing versioning strategies that balance freshness against stability and reproducibility.
---

# Dependency Range and Resolution

Declaring a dependency is not just naming a package; it is writing a constraint that a resolution algorithm will combine with hundreds of other constraints to pick a set of versions, and the result is often surprising. The developer who writes `"^1.2.0"` imagines they are pinning "version 1," but what they have actually done is authorize the resolver to pick any 1.x that satisfies all other constraints in the tree, which may be 1.2.0 today and 1.9.7 after the next install, with different behavior in each. Agents who treat version ranges as a convenience ("just use the caret, it's fine") encounter the consequences as mysterious bugs that appear or disappear between installs, conflicts that cannot be resolved without forcing a version, and security vulnerabilities lurking in transitive dependencies they never directly declared.

The judgment problem is that version ranges trade reproducibility for freshness, and the trade must be made with eyes open. A wide range (`^1`, `~1.2`, `>=1.0`) lets the resolver pick newer versions automatically, which delivers fixes and features but means two installs at different times may produce different dep sets (non-reproducible builds, different behavior). A pinned exact version (`1.2.3`) is reproducible but receives no updates until you bump it, so security fixes and bug fixes wait for explicit action. The lockfile is the reconciliation: ranges express intent (what versions are acceptable), the lockfile records the resolved reality (what was actually chosen), and reproducible installs use the lockfile while updates re-resolve the ranges. The agent must understand how resolution works, why conflicts arise, how to read and control the lockfile, and how to update safely without introducing breakage or vulnerabilities.

## Core Rules

### Understand the range syntax and what it actually permits

Range syntax varies by ecosystem (SemVer caret/tilde in npm, Cargo, Gems; requirements specifiers in Python; Maven's version ranges), and each permits a different set of versions. Know precisely what your chosen range allows: `^1.2.3` allows 1.x up to (not including) 2.0; `~1.2.3` allows 1.2.x up to 1.3; `>=1.2.3` allows anything 1.2.3 or higher including incompatible majors. A range that is wider than you intend authorizes the resolver to pull in versions you have not evaluated, including versions with breaking behavior if the dependency violates SemVer. Choose the narrowest range that meets your needs, and understand that wider ranges trade stability for automatic updates.

### Use the lockfile to reconcile reproducibility with range-based updates

The lockfile is the key to getting both reproducibility and freshness. Ranges in the manifest express which versions are acceptable; the lockfile records the exact versions resolved for the whole tree, including transitives. Reproducible installs and builds use the lockfile (install locked versions exactly), so two installs at different times produce identical dep sets. Updates re-resolve the ranges against the latest available versions and write a new lockfile, which you then review and test. Never bypass the lockfile in CI (e.g., a fresh resolve that ignores it), or CI installs different versions than developers and parity breaks. Commit the lockfile so the resolved tree is shared and reviewable.

### Reason about the whole dependency tree, not just direct dependencies

Most version conflicts and most vulnerabilities involve transitive dependencies you did not directly declare. Your direct dep on A depends on B@^2, your direct dep on C depends on B@^1, and the resolver must find a B that satisfies both or fail (or, in some ecosystems, install both). Read the resolved tree (the lockfile or a tree-inspection command) to understand what is actually included. When a conflict arises, the fix may be upgrading or downgrading a direct dep to shift the transitive constraints, adding an override/resolution, or patching the offending dependency. You cannot manage what you have not inspected; the direct manifest is not the full picture.

### Resolve version conflicts deliberately, not by forcing

When the resolver cannot satisfy all constraints, the temptation is to force a version (override the constraint to make the error go away). This can work, but it can also install a version that one of your dependencies is incompatible with, producing runtime failures the resolver was trying to prevent. Prefer resolving conflicts by adjusting direct dependencies (upgrade or downgrade to align transitive constraints), and use overrides/resolutions only when you understand why the conflict exists and have verified the forced version actually works with all consumers. Document the override and the reason, because it will need re-evaluation when dependencies update.

### Update dependencies on a cadence, not reactively or never

Dependencies accumulate security fixes, bug fixes, and compatibility updates, and a project that never updates falls further behind until updating becomes a massive, risky effort (the dependency update is then a project, not a routine). Establish a cadence for updates (weekly or monthly for routine updates), automate detection of available updates and security advisories (Dependabot, Renovate, npm audit, cargo audit), and apply updates in small, testable increments. Major version bumps (potential breaking changes) get their own evaluation and migration; patch and minor bumps (compatible, under SemVer) are lower-risk and can be applied more readily. The goal is continuous, low-risk updates rather than rare, high-risk catch-up.

### Handle security advisories with urgency and trace transitive paths

A security advisory in a dependency is urgent, and the affected dependency is often transitive (you depend on X which depends on the vulnerable Y). Trace the path from your code to the vulnerable package to understand whether you are actually exposed (you may depend on Y transitively but never call the vulnerable function). When exposed, update to a fixed version; if no fix is available, apply a mitigation (an override to a patched fork, a virtual patch, or removing the usage). Be aware that fixing a transitive vulnerability may require overriding a version your direct dependency does not yet support, which is a deliberate trade. Track advisories continuously, not at release time.

### Treat reproducibility and security as joint requirements, not opposites

Reproducibility (locked, stable dep sets) and security (up-to-date, patched dep sets) can feel opposed, but they are joint requirements managed by the same tooling. The lockfile gives reproducibility; the cadence of re-resolving and updating the lockfile gives security. The failure is to optimize one at the expense of the other: locking forever (secure-against-change but vulnerable to unpatched flaws) or resolving fresh every time (always-current but non-reproducible and prone to surprise breakage). The healthy state is a committed lockfile (reproducible) that is regularly, deliberately updated (secure), with each update reviewed and tested.

## Common Traps

### Using wide ranges without understanding what they permit

`^1` or `>=1.0` authorizes versions you have not evaluated, including behavior changes if the dependency violates SemVer. Choose the narrowest range that meets your needs.

### Bypassing the lockfile in CI

A fresh resolve in CI ignores the lockfile and may pick different versions than developers, breaking parity and reproducibility. Install from the lockfile in CI and commit it.

### Managing only direct dependencies and ignoring the tree

Conflicts and vulnerabilities usually involve transitives. Inspect the resolved tree, not just the manifest, to know what is actually included.

### Forcing a version to make a conflict error go away

This can install a version incompatible with a consumer, causing runtime failures the resolver was preventing. Resolve by adjusting direct deps, and override only with verification and documentation.

### Never updating, then facing a massive catch-up

Deferring updates turns routine maintenance into a high-risk project. Update on a cadence with automated advisory detection, in small testable increments.

### Assuming a security advisory does not affect you because it is transitive

Trace the path to the vulnerable package and determine actual exposure. Transitive vulnerabilities are the majority of real exposures.

### Optimizing reproducibility or security at the other's expense

Locking forever ignores patches; resolving fresh every time is non-reproducible. Use a committed lockfile that is regularly updated to satisfy both.

## Self-Check

- Do you understand precisely what version set your chosen range syntax permits, and have you chosen the narrowest range that meets your needs rather than defaulting to a wide caret?
- Is the lockfile committed, used for reproducible installs in CI (not bypassed by a fresh resolve), and re-resolved deliberately for updates?
- Do you inspect the full resolved dependency tree (including transitives), rather than reasoning only from the direct manifest, when diagnosing conflicts or vulnerabilities?
- When a version conflict arises, do you resolve it by adjusting direct dependencies or by a verified, documented override, rather than forcing a version to silence the error?
- Is there a cadence for routine dependency updates with automated detection of available updates and security advisories, so updates are continuous and low-risk rather than rare and high-risk?
- For security advisories, do you trace the transitive path to determine actual exposure and apply a fix or mitigation with urgency, rather than assuming transitive means unaffected?
- Are reproducibility and security managed jointly via a committed, regularly-updated lockfile, rather than one optimized at the expense of the other?
- Are major version bumps (potential breaking changes) evaluated and migrated separately from compatible patch/minor updates?
