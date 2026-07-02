---
name: dependency_management_and_locking.md
description: Use when the agent is adding, upgrading, pinning, or removing a dependency, resolving version conflicts in a dependency tree, responding to a vulnerability advisory, choosing a lockfile strategy, evaluating supply-chain risk of a new package, or deciding whether to vendor, fork, or reimplement third-party code.
---

# Dependency Management And Locking

Every dependency is a piece of someone else's code that runs with your privileges, ships in your artifacts, and breaks on your schedule. Adding one is cheap and feels free; removing one is expensive and rarely happens. The aggregate effect is that most projects carry a dependency tree far larger than anyone intends, full of packages no one audited, versions no one chose deliberately, and transitive code that can change behavior, introduce vulnerabilities, or disappear at any moment. Dependency management is the discipline of keeping that tree small, deliberate, and reproducible — because the moment you stop paying attention, the tree pays attention to you, usually during an incident.

Agents tend to treat dependencies as a solution to every problem: a package exists, so install it. The immediate cost is low and the deferred cost is invisible, so the tree grows. The judgment problem is to decide when a dependency is worth its total cost — not just its function, but its security surface, its maintenance burden, its lockfile discipline, its transitive footprint, and its supply-chain risk — and to manage the ones you do accept so they do not drift into unreviewed, un-reproducible, or vulnerable states.

## Core Rules

### Evaluate The Total Cost Before Adding A Dependency

The question is not "does a package exist for this" but "is this package worth carrying for the life of the project." A dependency's cost extends far beyond its API. Before adding one, weigh:

- **Maintenance and health** — is it actively maintained, or abandoned? A dependency on an unmaintained package is a dependency on a future fork-or-replace project you did not plan for.
- **Security surface** — does it handle untrusted input, crypto, network, or privileged operations? Every such dependency is an attack surface you are responsible for but cannot fully control.
- **Transitive footprint** — how many packages does it pull in, and are those trustworthy? A small direct dependency can drag in a large tree of code no one vetted.
- **License compatibility** — does its license permit your use, including distribution? Discovering a license conflict at release time is expensive.
- **Alternative** — could the needed function be written in a few dozen lines of code you fully own? For narrow needs, a small amount of owned code often beats a dependency you must track forever.

Favor dependencies that are well-maintained, widely used, narrowly scoped, and low in transitive weight. Favor reimplementing or vendoring when the need is small, the stakes are high, or the package is unstable. The default should not be "install"; it should be "is this worth the ongoing cost."

### Lock Dependencies For Reproducibility

A lockfile pins the exact versions (including transitive dependencies) that the project uses, so that every install — on every machine, in CI, in production — resolves to the same code. Without a lockfile, two builds separated by a day can resolve to different transitive versions, and a failure that appears "only in CI" or "only on my machine" becomes a debugging nightmare.

Commit the lockfile and treat it as authoritative for application and deployment projects. The lockfile is what makes builds reproducible and dependency changes reviewable: an upgrade shows up as a diff in the lockfile, which a reviewer can read, rather than as a silent resolution at install time. For libraries that others consume, decide deliberately whether to commit a lockfile (some ecosystems discourage it for libraries); either way, the library's declared version ranges must be narrow enough to be safe across the range you permit.

Locking is not optional hygiene; it is the foundation that makes every other dependency practice — upgrade cadence, vulnerability response, reproducible builds — possible.

### Upgrade Deliberately, Not By Accident

Dependencies change. New versions fix bugs, patch vulnerabilities, add features, and occasionally break behavior. Two failure modes surround upgrades: never upgrading, which leaves you on vulnerable and stale versions until forced; and upgrading implicitly, where a fresh install or a casual `update` pulls new versions no one reviewed.

A healthy cadence sits between these. Upgrade on a deliberate schedule, in small reviewable steps, with the lockfile diff as the artifact of what changed. Separate routine patch upgrades (bug and security fixes, low risk) from minor and major upgrades (new features, possible breaking changes, higher risk). For higher-risk upgrades, read the changelog, run the test suite, and stage the change before it reaches production. Avoid blanket `update everything` commands that produce an unreadable diff and entangle many independent upgrades into one risky change.

The goal is that every version in your tree is there because someone chose it, not because the resolver happened to pick it on a Tuesday.

### Respond To Vulnerabilities With Triage, Not Panic

Vulnerability advisories arrive constantly, and most do not affect your project. Responding to every advisory as urgent burns out the team, while ignoring them all leaves real exposure. Triage each advisory against actual exposure before acting.

For each advisory, determine:

- **Is the vulnerable code reachable?** Many advisories affect code paths your project never calls. A vulnerability in an optional feature you do not use may be a non-issue.
- **Is the vulnerable version actually in your tree?** Check the lockfile, not the manifest; the resolved version is what matters.
- **What is the severity and exploitability in your context?** A remote code execution in a network-facing path is urgent; a denial-of-service in a batch job is important but not 2am-urgent.
- **What is the fix path?** Upgrade the direct dependency, upgrade a transitive that carries the vulnerable code, apply a patch overlay, or mitigate by removing the reachable code path.

Automate the detection (dependency scanning in CI) but keep the triage human. A scanner that fires on every advisory without reachability produces alert fatigue, which leads to ignoring the scanner — and the one advisory that mattered.

### Manage Transitive Dependencies And Conflicts

The dependency you add directly is a fraction of the code that enters your tree. Transitive dependencies — the dependencies of your dependencies — are where most of the footprint, the vulnerabilities, and the version conflicts live. You are responsible for them even though you did not choose them.

Practices that keep transitive dependencies manageable:

- **Prefer dependencies with small transitive trees.** A package that pulls in fifty transitive dependencies for a small function is rarely worth it.
- **Resolve version conflicts deliberately.** When two dependencies require incompatible versions of a shared transitive, the resolver picks one (possibly breaking the other) or fails. Understand which version wins and why, and whether deduplication is safe for that package.
- **Audit the tree periodically.** Tools that list the full resolved tree let you see what is actually present. Surprises — unexpected packages, duplicate versions, abandoned transitive dependencies — are common and worth surfacing.
- **Pin or override vulnerable transitives** when the direct dependency cannot be upgraded yet, using the override mechanism your ecosystem provides. Document why the override exists and when it can be removed.

### Treat The Dependency Tree As A Supply-Chain Risk Surface

Dependencies are third-party code that executes in your context. A compromised or malicious dependency — whether a typosquat, a hijacked maintainer account, or a build-time payload — can exfiltrate secrets, introduce backdoors, or corrupt builds. The risk is real and has materialized repeatedly in the ecosystem.

Reduce supply-chain risk through layered controls:

- **Prefer packages with a trustworthy provenance** — maintained by known individuals or organizations, with a history of responsible updates.
- **Pin to specific versions and verify integrity** through checksums or signed manifests where the ecosystem supports it, so a replaced artifact cannot silently enter your build.
- **Scope build-time and dev dependencies tightly.** Build tooling has access to source and secrets during the build; a malicious dev dependency is as dangerous as a malicious runtime one.
- **Review the dependency's own dependencies** for obvious red flags before adopting, especially for packages handling sensitive data or operations.
- **Have a plan for removal.** Any dependency can be compromised or abandoned; knowing how you would replace or reimplement critical ones reduces panic when it happens.

## Common Traps

### Installing A Package For Every Small Need

Reaching for a dependency for every trivial function grows the tree fast and accumulates packages no one audits. For narrow, stable needs, a small amount of owned code is often cheaper in total than a dependency you must track, upgrade, and secure forever.

### The Uncommitted Or Ignored Lockfile

A lockfile that is not committed, or that CI regenerates fresh on every run, provides no reproducibility. Two builds resolve differently, failures appear intermittent, and upgrades happen silently. Commit the lockfile and use it everywhere.

### Blanket Upgrades With Unreadable Diffs

Running a blanket update that touches dozens of packages at once produces a diff no one can review and entangles independent risks. Upgrade in small, deliberate steps with readable lockfile diffs, separating routine patches from riskier minor and major bumps.

### Treating Every Vulnerability Advisory As Equally Urgent

Responding to every advisory as a fire drill causes alert fatigue and leads to ignoring advisories altogether. Triage by reachability and exploitability in your context; most advisories do not affect reachable code.

### Ignoring Transitive Dependencies

The direct dependencies are the visible minority; the transitives are where most of the code, conflicts, and vulnerabilities live. Audit the full tree periodically and resolve conflicts deliberately rather than trusting the resolver's default.

### Assuming A Dependency Is Immutable

A version pinned today can be re-published, yanked, or compromised tomorrow if integrity is not verified. Pinning without integrity verification protects against resolver drift but not against artifact replacement. Use checksums or signed manifests where available.

### Removing A Dependency Without Removing Its Configuration

Deleting a package from the manifest but leaving its configuration, lockfile entries, type declarations, or runtime references creates ghost dependencies that confuse future readers and can re-pull the package transitively. Remove a dependency completely, including all its traces.

## Self-Check

- [ ] Each dependency in the tree was added deliberately, with consideration of maintenance health, security surface, transitive footprint, license, and whether owned code was a better fit.
- [ ] The lockfile is committed and authoritative across local, CI, and production installs, so every install resolves to the same versions.
- [ ] Upgrades happen on a deliberate cadence in small reviewable steps with readable lockfile diffs, not via blanket updates or silent fresh resolution.
- [ ] Vulnerability advisories are triaged by reachability and exploitability in the project's actual context before action, and detection is automated while triage stays human.
- [ ] The full transitive tree is audited periodically, version conflicts are resolved deliberately with the winning version understood, and vulnerable transitives are overridden with documented reasons.
- [ ] Supply-chain risk is addressed through provenance checks, integrity verification (checksums or signatures), tight scoping of build-time dependencies, and a removal plan for critical dependencies.
- [ ] Removed dependencies were fully removed, including configuration, lockfile entries, and references — no ghost dependencies remain.
- [ ] The dependency tree is no larger than the project genuinely needs, and each entry could be justified if challenged.
