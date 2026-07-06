---
name: dependency_security_and_trust.md
description: Use when the agent is adding, upgrading, pinning, or removing a library, package, framework, SDK, CLI, or module; evaluating whether to trust a dependency; scanning or triaging CVEs and vulnerability advisories; managing lockfiles, transitive dependencies, or reproducible builds; responding to a supply-chain incident (compromised package, typosquat, dependency confusion, malicious update); producing or consuming an SBOM; choosing an update cadence or dependabot/renovate policy; reviewing license compatibility; or deciding whether to vendor, fork, or rewrite instead of depending. Also covers the attack surface of dependencies, trust signals versus real security, bus factor and maintenance health, and the cost of both too many and too few dependencies.
---

# Dependency Security And Trust

Every dependency is code that runs with your privileges, accesses your data, and ships inside your product — but that you did not write, cannot fully inspect, and will not always understand. A modern application pulls in hundreds or thousands of packages, most of them transitive, and each is a trust decision: you are betting that the package's authors are competent and honest, that its future updates will not turn malicious, and that no attacker will compromise it upstream of you. The recurring supply-chain failures are not exotic — they are a package whose maintainer abandoned it, a transitive dependency with a known CVE that no one triaged, a typosquat that looked close enough to the real name, or a popular library that pushed a malicious update to every project that auto-updated. The judgment problem is deciding, for each dependency, whether to trust it, how tightly to constrain it, and how to know when it has become a liability.

Agents tend to under-invest here because adding a dependency is trivially easy and immediately rewarding: one command, a feature works, the task is done. The cost is deferred and distributed: the attack surface grows, the upgrade burden accumulates, a license incompatibility surfaces during due diligence, and a vulnerability advisory lands in the inbox months later with no owner. The mirror failure is over-caution — refusing all dependencies and reimplementing everything, which trades supply-chain risk for correctness and maintenance risk. The judgment is not "dependencies bad" or "dependencies good"; it is treating each dependency as a deliberate trust and maintenance decision with a lifecycle, not a one-time install.

This skill is about the security and trust of third-party code. It complements the cryptography skill (which covers protecting your own data) and is distinct from it. Here the question is whether to let someone else's code into your system at all, and how to govern it once it is in.

## Core Rules

### Treat Every Dependency As Code That Runs With Your Privileges

The starting frame is that a dependency is not a passive artifact; it executes. A library imported into your application runs in your process, with your filesystem, network, and secret access; an SDK in your build pipeline can read your source and tokens; a CLI in your CI can exfiltrate your artifacts. Treat the decision to add a dependency with the same seriousness as the decision to let a new contributor commit code — because that is effectively what it is.

Consequences of this frame:

- **Prefer fewer dependencies, especially for security-sensitive paths.** Auth, crypto, payment, and secrets-handling code should depend on the smallest, most audited set possible. A transitive dependency in the auth path is a transitive dependency in your trust boundary.
- **Scope and isolate where possible.** Run untrusted or low-trust tooling in sandboxed, least-privilege environments (separate containers, ephemeral CI runners, no access to production secrets). Do not give a build tool credentials it does not need.
- **Review what a dependency actually does at install and build time, not only at runtime.** Many supply-chain attacks execute in install scripts (`postinstall`, setup hooks) or build steps, before the code ever runs in production.

### Evaluate Trust On Substance, Not Popularity

The instinct to trust a popular package is reasonable but unreliable. Downloads, stars, and "everyone uses it" are weak security signals: a widely-used package can be abandoned, can have a CVE, or can be compromised precisely because it is a high-value target. Evaluate trust on substantive criteria:

- **Maintenance health.** Is it actively maintained? Recent commits, responsive maintainer, open issues being triaged, security advisories published promptly. An abandoned popular package is a liability, not an asset — it will accumulate unpatched vulnerabilities.
- **Bus factor and ownership.** Is it maintained by one person, a small group, or a foundation/company? A single-maintainer package is one life event away from abandonment or compromise (this has happened repeatedly). Check who has publish access and how many people that is.
- **Security posture.** Does it have a security policy, a way to report vulnerabilities, signed releases, 2FA on publishing, and a history of responding to advisories? These are real signals; download count is not.
- **Provenance and signatures.** Does the package support provenance (SLSA, Sigstore, signed artifacts) so you can verify it was built from the claimed source? Increasingly, ecosystems support this; prefer packages that do.
- **Code quality and test coverage.** A package with tests, CI, and readable code is more likely to be correct and more likely to be maintained. A package with none is higher risk in every dimension.

A package that is popular but abandoned, single-maintainer, with no security policy and no signed releases is high-risk regardless of its download count. A less-popular package maintained by a reputable team with provenance may be lower-risk. Judge the substance.

### Pin, Lock, And Make Builds Reproducible

An unpinned or floating dependency is a supply-chain incident waiting to happen: the same `package.json`/`requirements`/`Cargo.toml` can resolve to different code tomorrow than it does today, and a compromised upstream update will flow silently into your build. Reproducibility is the baseline defense.

- **Use lockfiles.** Lock exact versions (including transitive dependencies) so that a fresh install produces the same dependency tree as the last verified build. Commit the lockfile.
- **Pin by content hash where the ecosystem supports it** (Subresource Integrity for browser assets, hash-pinned dependencies, immutable references). Pinning by version number is weaker than pinning by content, because a version can be republished or a registry can be compromised.
- **Avoid floating ranges in production.** `^`, `~`, `*`, or `latest` mean the code you run is not the code you reviewed. Use ranges only where you have a deliberate, automated update process that tests before adopting.
- **Make installs reproducible.** A build that resolves "latest" each time cannot be audited, rolled back, or forensically compared. Reproducibility is what makes every other control (scanning, review, SBOM) meaningful.

A lockfile turns "what code are we running?" from an unanswerable question into an inspectable fact.

### Account For Transitive Dependencies

Most of your dependency risk is not in the packages you directly chose — it is in the packages those packages pull in, often transitively several layers deep. A decision to add one library can add dozens of packages you never reviewed, by authors you never evaluated. Supply-chain attacks frequently target a transitive dependency precisely because it is less scrutinized.

- **Inspect the full dependency tree, not just direct dependencies.** Know what you are actually installing. A small direct dependency that pulls in a huge tree can be higher-risk than a larger self-contained one.
- **Apply the same trust standard transitively.** A transitive dependency is still code running in your process. If your direct dependency pulls in something untrusted or unmaintained, that is your risk.
- **Watch for dependency sprawl.** When a tree balloons, question whether the direct dependency is worth what it brings. Tools that visualize or cap the dependency tree help surface this.
- **Be wary of dependencies that execute at install time.** Install scripts in transitive packages are a common attack vector, because they run with your privileges before you have reviewed anything.

### Scan For Known Vulnerabilities And Triage By Real Risk

Known vulnerabilities (CVEs, GHSA advisories, ecosystem-specific databases) are the most discoverable supply-chain risk, and scanning for them is table stakes. But scanning produces alerts; triage is where the judgment lives.

- **Scan continuously, in CI and on a schedule.** Catch advisories on introduction (in the PR that adds the dependency) and on disclosure (when a new CVE is published against a dependency you already have).
- **Triage by actual exposure, not by CVSS alone.** A critical CVE in a function you never call, in an environment it cannot reach, is lower risk than a high-severity one on your hot path. Ask: Is the vulnerable code path reachable? Is the vulnerable configuration in use? Is the attack precondition realistic in your deployment?
- **Distinguish "vulnerable version installed" from "vulnerable behavior exploitable."** The first is what the scanner reports; the second is what matters. A scanner cannot know your usage; you must.
- **Have a remediation path for each severity.** Patch, upgrade, pin to a fixed version, apply a mitigation, or accept with documented justification. An advisory with no owner and no decision is how vulnerabilities persist for years.

Scanning without triage produces alert fatigue, where real risks drown in noise. Triage is the work; scanning is the input.

### Defend Against Name Confusion Attacks

Attackers routinely publish packages whose names are close to popular ones, hoping a typo or a misconfigured resolver installs the malicious version instead. The variants:

- **Typosquatting.** `lodhash` instead of `lodash`, `reqeusts` instead of `requests`. A mistyped install command pulls the attacker's package.
- **Dependency confusion.** An internal package name that also exists (or can be registered) on a public registry. A resolver that checks the public registry first pulls the attacker's package into a build that expected the internal one.
- **Brand/account impersonation.** A package named to suggest it is official (`company-sdk`) or maintained by an account named to resemble the real maintainer.

Defenses:

- **Install from trusted, scoped registries.** Use private registries or scoping so internal names resolve internally and cannot be shadowed by public packages.
- **Verify names exactly.** Treat the package name as security-relevant; a near-miss is the attack, not a coincidence.
- **Pin and lock.** A locked dependency cannot be silently swapped for a same-named different package in a fresh resolve.
- **Configure resolver precedence.** Ensure internal/scoped packages resolve from internal sources first, closing the dependency-confusion window.

### Maintain An SBOM And Know What Is In Your Product

A Software Bill of Materials (SBOM) is the inventory of what ships in your product — direct and transitive dependencies, versions, and (increasingly) their provenance. It is the answer to "what code are we running, and where did it come from?" and it is essential for incident response: when a new CVE drops, the first question is "are we exposed?" and an SBOM is how you answer in minutes instead of days.

- **Generate an SBOM as part of the build.** Automate it so it stays current; a hand-maintained inventory is always stale.
- **Store it with the release.** Tie each shipped artifact to the SBOM that describes its dependency tree, so you can answer exposure questions for any past version.
- **Use it for triage and audits.** When an advisory lands, query the SBOM rather than rebuilding the tree from memory. Customers and regulators increasingly require it.

### Check License Compatibility, Not Only Security

A dependency that is secure and well-maintained can still be unusable because of its license. License obligations are legal constraints that attach to your product, and discovering an incompatibility late — during due diligence, an audit, or a customer's legal review — is expensive.

- **Identify the license of every dependency, including transitive.** Tools can enumerate this; do not assume.
- **Flag copyleft and source-disclosure licenses.** GPL-family licenses can impose obligations on your product (source availability, licensing of combined work) that may be incompatible with your distribution model. AGPL is especially consequential for networked services.
- **Watch for "source-available" or non-commercial licenses.** These are not open source and may restrict commercial use or redistribution. A license that forbids commercial use cannot be used in a commercial product, however good the code is.
- **Resolve incompatibilities deliberately.** Replace, obtain a commercial license, or isolate — but do not ship and hope no one notices.

### Decide An Update Strategy Deliberately and decide Whether To Depend, Vendor, Fork, Or Rewrite

Dependencies need updates: for security patches, for compatibility, and to avoid the cliff where an ancient version can no longer be upgraded at all. But updates also carry risk (regressions, breaking changes, new transitive dependencies, occasionally malicious updates). The strategy is a deliberate balance, not a default.

- **Patch security updates quickly, with targeted testing.** A security patch on your hot path warrants fast adoption; a minor feature release does not.
- **Group and schedule non-security updates.** Regular, bounded update windows (weekly/monthly) with test gates avoid both stale dependencies and update-driven churn.
- **Beware auto-apply without gates.** Tools that auto-merge dependency updates are convenient and are also how malicious upstream updates flow straight into production. Auto-update is acceptable for low-risk paths with strong tests and pinning; it is dangerous for security-sensitive or lightly-tested paths.
- **Avoid the "never update" cliff.** A dependency left un-updated for years eventually cannot be upgraded at all (breaking changes accumulate, transitive conflicts), forcing a rewrite. Gradual, continuous updates prevent this.

Not every need should be met with a dependency, and not every dependency should stay external. The options, in rough order of commitment:

- **Depend (the default).** Lowest effort, ongoing trust and upgrade burden, shared maintenance with the ecosystem.
- **Vendor (copy the code in, pinned).** Removes upstream-change risk and registry dependency, at the cost of owning all maintenance and security fixes yourself. Good for stability-critical, rarely-changing code.
- **Fork.** Take over maintenance explicitly. Justified when the upstream is abandoned or diverging, but you now own it fully.
- **Rewrite or implement yourself.** Removes the trust surface entirely but trades supply-chain risk for correctness and maintenance risk. Only worth it for small, well-understood, security-critical pieces where the cost of self-implementation is lower than the cost of trust.

The decision turns on size, criticality, maintenance health, and how unique the functionality is. A tiny utility you could write in an afternoon is often better self-implemented than pulled in with its dependency tree; a complex, actively-maintained library is almost always better depended on than reimplemented.

## Common Traps

### Trusting On Popularity Alone

Choosing a package because it has many downloads or stars, without checking maintenance health, ownership, or security posture. Popular packages get compromised and abandoned too; popularity is reach, not trustworthiness. Evaluate the substance.

### Floating Versions And Unlocked Installs

Using `latest`, wide ranges, or installing without a committed lockfile, so the code that runs is not the code that was reviewed. A compromised or buggy upstream update flows silently into production. Pin and lock; adopt changes through a tested update process.

### Ignoring Transitive Dependencies

Reviewing only the direct dependencies, while the real risk and bulk of the code lives in transitive packages pulled in several layers deep. Inspect the full tree and apply trust standards transitively — especially for packages with install-time scripts.

### Triage By CVSS Alone

Treating every critical CVE as an emergency, or dismissing every low-severity one, based only on the score. Reachability, configuration, and environment determine real risk. A critical CVE in an unreachable path is noise; a lower-score one on your hot path may be the real emergency.

### Typosquat Or Dependency Confusion By Mistake

Installing a near-miss package name, or letting a resolver pull an internal name from a public registry. Treat names as security-relevant, scope internal packages to private registries, and configure resolver precedence to close the confusion window.

### Auto-Merging Updates Without Gates

Enabling auto-merge on dependency PRs for convenience, so a malicious or breaking upstream update lands in production without review. Auto-update is acceptable only with strong tests, pinning, and on low-risk paths; security-sensitive paths need human review.

### The Never-Update Cliff and shipping A License Incompatibility

Leaving dependencies un-updated for years to "avoid breakage," until the gap is so large that upgrading is a major project or impossible. Continuous, bounded updates prevent the cliff; deferred updates create it.

Including a GPL/AGPL or non-commercial-licensed dependency without realizing it imposes obligations on your product, then discovering it during an audit or customer review. Enumerate licenses (including transitive) and resolve incompatibilities before shipping.

### Giving Build Tools Unneeded Credentials and abandoned Dependency Left In Place

Running build tools, install scripts, or CI actions with broad access to secrets, source, or production, so a compromised dependency can exfiltrate everything. Scope and isolate tooling to least privilege; a build step should not hold production credentials.

A dependency whose maintainer has gone silent, with open security issues and no releases, kept because "it still works." It is accumulating unpatched risk. Plan a replacement, fork, or vendor before a CVE forces it under deadline.

## Self-Check

- [ ] Each dependency was evaluated on substance (maintenance health, ownership/bus factor, security policy, provenance/signatures, code quality) — not chosen on popularity, downloads, or stars alone.
- [ ] Dependencies are pinned and locked (content-hash where supported), the lockfile is committed, and builds are reproducible — no floating ranges or `latest` reach production without a tested update process.
- [ ] The full transitive dependency tree was inspected and held to the same trust standard as direct dependencies, with attention to packages that execute at install or build time.
- [ ] Known vulnerabilities are scanned continuously (on introduction and on disclosure) and triaged by real exposure (reachability, configuration, environment) rather than by CVSS score alone, with a defined remediation path per advisory.
- [ ] Name-confusion attacks are defended against: package names are verified exactly, internal packages are scoped to private registries, and resolver precedence prevents dependency confusion.
- [ ] An SBOM is generated automatically per build, stored with the release, and usable to answer "are we exposed?" for any past version when an advisory lands.
- [ ] Licenses of all dependencies (including transitive) are enumerated; copyleft, source-available, and non-commercial licenses are flagged and resolved before shipping.
- [ ] The update strategy is deliberate: security patches adopted quickly with testing, non-security updates grouped and gated, auto-merge used only on low-risk paths with strong tests, and no dependency is left to the never-update cliff.
- [ ] Build tools, install scripts, and CI actions run with least privilege and no unneeded access to secrets, source, or production — a compromised dependency cannot exfiltrate everything.
- [ ] For each dependency the depend/vendor/fork/rewrite choice was considered against size, criticality, and maintenance health — security-sensitive paths carry the smallest, most audited set, and abandoned dependencies have a replacement plan rather than indefinite retention.
