---
name: dependency_update_and_renovation.md
description: Use when the agent is managing dependency updates (automated dependency bots, Renovate/Dependabot), deciding update cadence and grouping strategy, reasoning about breaking-change risk and semantic versioning trust, handling transitive dependency updates, or diagnosing update-related breakage (a patch update breaking the build, a transitive bump changing behavior, automated updates landing unreviewed). Also covers the failure mode of pinning everything and falling far behind (accumulating technical debt and security exposure), auto-merging updates without review (a compromised or buggy dependency reaching production), trusting semver blindly (a "patch" update with a behavior change or a transitive bump with a breaking change), and the recurring mistake of treating dependency updates as a chore rather than a continuous, risk-managed process that balances freshness against stability.
---

# Dependency Update And Renovation

Dependencies must be updated — for security, for bug fixes, for new features — and the update process is a continuous risk-management problem, not a chore. Automated tools (Renovate, Dependabot) open update pull requests, but the judgment is in how updates are configured, reviewed, and merged. The judgment problem is balancing freshness against stability. Pinning everything and never updating feels safe but accumulates debt: the gap to current grows, security fixes are delayed, and the eventual update is large and risky. Auto-merging everything feels efficient but lets a compromised or buggy dependency reach production unreviewed. Trusting semantic versioning blindly fails because a "patch" update can carry a behavior change, and a transitive dependency bump (not in the manifest) can break behavior invisibly. The discipline is to set update cadence and grouping for risk control, to review updates by risk tier (security/critical vs routine), to handle transitive updates explicitly, and to verify updates in CI before merge. Dependency updates are a supply-chain security and stability process, not a janitorial task.

Agents tend to either ignore updates (pin and forget) or auto-merge blindly, because the demo of "dependencies are now current" is satisfying and the risk is invisible. The harm appears as a security vulnerability unpatched for months (pinned and forgotten), as a production outage from an auto-merged update (a behavior change in a "patch" release), as a transitive dependency bump breaking the build invisibly (not in the manifest, not reviewed), and as a large, risky update after long neglect (the gap made it expensive). The judgment is to treat updates as continuous and risk-tiered, to configure automation for the project's risk tolerance, to make transitive updates visible, and to verify in CI. A dependency update process that is either frozen or uncontrolled is a liability — frozen accumulates exposure, uncontrolled imports breakage.

This skill covers update cadence and grouping, risk-tiered review, transitive dependency handling, and CI verification. It complements the dependency-security-and-trust skill (trust decisions for new dependencies), the vulnerability-triage-and-response skill (responding to known vulnerabilities), and the sbom-and-provenance skill (knowing what is in a build). Here the focus is the ongoing update process and its risk management.

## Core Rules

### Balance Freshness Against Stability With Cadence And Grouping

The update cadence and grouping strategy control how much change lands how often, and they should be configured for the project's risk tolerance, not left at defaults:

- **Update regularly to stay close to current.** Regular, small updates are lower-risk than rare, large ones; staying close to current keeps the gap small, security fixes prompt, and each update reviewable. Pinning and neglecting accumulates a large, risky gap.
- **Group updates to control change volume.** Grouping related updates (e.g., all patch updates together, or all dependencies in a monorepo package together) batches change into reviewable units rather than a flood of individual PRs; grouping reduces review fatigue and lets CI validate the combined change.
- **Separate update streams by risk tier.** Security and critical updates should be fast-tracked (separate stream, prompt review); routine updates can be batched and slower. Do not let routine updates block or delay security updates.
- **Set the cadence for the project.** A library consumed by many downstream projects may update conservatively (to avoid churning consumers); an application may update aggressively. Match cadence to the project's role and the team's capacity to review.

### Review Updates By Risk Tier, Not All The Same

Not all updates carry the same risk, and review effort should match the risk:

- **Security and critical updates need prompt, careful review.** A security update is urgent but must still be reviewed (a malicious "security fix" is a known attack vector); verify the update addresses the vulnerability, check the diff, and confirm CI passes before merge.
- **Major updates (breaking changes) need thorough review.** A major version bump signals breaking changes; review the changelog/migration guide, update calling code, and test thoroughly. Schedule major updates rather than auto-merging.
- **Minor and patch updates can be lighter-reviewed but not unreviewed.** Lower risk does not mean no risk; a "patch" can carry a behavior change. Review the changelog and verify in CI; consider auto-merge only for well-tested, low-impact updates with strong CI.
- **Do not auto-merge without verification.** Auto-merge is appropriate only when CI is comprehensive and the update is low-risk; a compromised or buggy dependency auto-merged to production is a known failure mode. Require CI to pass and, for higher-risk updates, human review.

### Handle Transitive Dependencies Explicitly

Transitive dependencies (dependencies of dependencies) update invisibly when a direct dependency's version range permits, and they can break behavior without a manifest change:

- **Make transitive updates visible.** A lockfile pinning transitive versions makes updates visible (a lockfile change shows the transitive bump); without a lockfile, transitive versions float and can change between installs. Use a lockfile and review its changes.
- **Review transitive bumps in the lockfile.** When a direct dependency update bumps a transitive dependency, review the transitive change too — it can carry a behavior change or vulnerability even if the direct update is benign.
- **Pin or constrain critical transitive dependencies.** If a transitive dependency is critical (a security-sensitive library), pin or constrain it explicitly so it does not float; rely on the lockfile for routine pinning and explicit constraints for critical cases.
- **Audit transitive dependencies regularly.** Transitive dependencies are a common source of vulnerabilities (you depend on them but did not choose them); audit them (with the SBOM and vulnerability scanning) regularly, not only direct dependencies.

### Trust Semver Conditionally, Not Blindly

Semantic versioning communicates intent, but it is a convention, not a guarantee, and it is sometimes violated:

- **Semver signals risk tier, not certainty.** A patch update is intended to be backward-compatible, but maintainers make mistakes (a "patch" with a behavior change) or disagree on what is breaking. Use semver to prioritize review, not to skip it.
- **Transitive bumps may not follow semver at all.** A transitive dependency's version is chosen by the direct dependency's range, not by your semver expectations; a transitive bump can be a major change that the direct dependency permits.
- **Verify behavior, not just version.** For critical dependencies, verify the update's behavior in CI (tests, integration tests) rather than trusting the version number; a test that exercises the dependency's behavior catches a semver violation.
- **Watch for yanked and compromised versions.** A version can be yanked (removed by the maintainer) or compromised (a malicious publish); pinning and review protect against pulling a bad version, and provenance verification (see the sbom-and-provenance skill) protects against compromise.

### Verify Updates In CI Before Merge

CI is the gate that catches update-related breakage, and it must be comprehensive for the update process to be safe:

- **Run the full test suite on update PRs.** Unit, integration, and (where feasible) end-to-end tests catch behavior changes from updates; a minimal CI suite misses regressions.
- **Run security and license scans on updates.** An update can introduce a vulnerability or an incompatible license; scan updates (SCA tools) before merge, not only at release.
- **Verify the build is reproducible.** An update that changes the build output (different binary, different lockfile) signals a behavior or environment change; reproducible builds (see the reproducible-and-hermetic-builds skill) catch this.
- **Require CI to pass before auto-merge.** If auto-merging, require CI to pass comprehensively; a weak CI gate makes auto-merge dangerous.

## Common Traps

### Pinning Everything And Falling Far Behind

Pinning all dependencies and neglecting updates, accumulating a large gap to current that makes the eventual update large, risky, and security-exposed. Update regularly to stay close to current.

### Auto-Merging Without Review Or Strong CI

Auto-merging updates without comprehensive CI or human review for higher-risk updates, letting a compromised or buggy dependency reach production. Require CI to pass and review by risk tier.

### Trusting Semver Blindly

Assuming a "patch" update is always safe because semver says so, missing a behavior change or semver violation. Use semver to prioritize review, not skip it; verify behavior in CI.

### Ignoring Transitive Dependency Updates

Focusing on direct dependencies and missing transitive bumps (in the lockfile) that carry behavior changes or vulnerabilities. Use a lockfile and review transitive changes.

### Treating Updates As A Chore, Not Risk Management

Treating dependency updates as janitorial work to be minimized or automated away, rather than a continuous supply-chain security and stability process. Manage updates as risk-tiered and continuous.

### Security Updates Blocked By Routine Update Backlog

Mixing security and routine updates in one stream so a routine backlog delays a security fix. Separate streams so security updates are fast-tracked.

## Self-Check

- [ ] Update cadence and grouping are configured for the project's risk tolerance (regular small updates to stay close to current, grouped for reviewable units, security/critical updates fast-tracked separately from routine).
- [ ] Updates are reviewed by risk tier (security/critical prompt and careful, major thorough with migration, minor/patch lighter but not unreviewed), and auto-merge is used only with comprehensive CI and for low-risk updates.
- [ ] Transitive dependencies are handled explicitly: a lockfile pins and makes transitive updates visible, transitive bumps are reviewed in the lockfile, critical transitives are explicitly constrained, and transitives are audited regularly.
- [ ] Semver is trusted conditionally (used to prioritize review, not skip it), transitive bumps are not assumed to follow semver, behavior is verified in CI for critical dependencies, and yanked/compromised versions are guarded against by pinning and provenance.
- [ ] CI verifies updates comprehensively (full test suite, security/license scans, reproducible build check) and is required to pass before auto-merge.
- [ ] The highest-risk cases were verified — a pinned-and-forgotten dependency with an unpatched vulnerability, an auto-merged update causing an outage, a transitive bump breaking behavior invisibly, a semver violation in a "patch" — not only the "dependencies are current" state.
