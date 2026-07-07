---
name: dependency_and_supply_chain_security.md
description: Use when the agent is adding, upgrading, or auditing third-party dependencies (libraries, packages, modules, container base images, build tools, or plugins); choosing whether to depend on a package or vendor it; responding to a vulnerability advisory (CVE, GHSA, npm advisory) in a dependency; designing dependency pinning, lockfiles, provenance, or SBOM practices; configuring Dependabot/Renovate/OWASP Dependency-Check; reviewing a pull request that adds dependencies; or hardening a build pipeline against dependency confusion, typosquatting, or malicious package injection. Covers transitive dependency risk, license and supply-chain obligations, vendoring versus depending, and the blast radius of a compromised dependency.
---

# Dependency And Supply Chain Security

Modern software is mostly other people's code. A typical application depends on hundreds or thousands of packages, most of them transitive — pulled in by a direct dependency, often invisible to the developer who added it. This is an enormous productivity gain and an enormous attack surface. Every dependency is code that executes with your application's privileges, reads your configuration, touches your secrets, and ships to your users. A single compromised package, a malicious maintainer, a typosquatted name, or a dependency-confusion attack can turn a routine `install` or `build` into a full compromise of your source, your CI, and your production environment. The supply chain is now one of the most common and most damaging vectors for real breaches.

Agents tend to treat dependencies as frictionless: add the package, import the function, move on. The cost is deferred and invisible until an advisory lands, a license conflict blocks a release, a transitive package turns out to be malware, or a build pulls a different version in CI than locally. The judgment problem is recognizing that every dependency is a security, licensing, maintenance, and availability decision — not merely a convenience — and that the practices around selection, pinning, scanning, and response determine whether a dependency incident is a non-event or a breach. This skill is about the discipline of depending on external code safely: choosing what to depend on, locking what you use, knowing what you have, and responding when something you depend on is compromised.

## Core Rules

### Treat Every Dependency As A Security And Maintenance Decision

Adding a dependency is not free. Before introducing one, evaluate it as deliberately as you would evaluate hiring a contractor who will have access to your codebase:

- **Is it necessary?** Could a small amount of your own code replace it? Dependencies that exist to avoid ten lines of code are usually a net negative once maintenance, security, and bundle-size costs are counted. For substantial functionality (crypto, parsing, protocols), prefer a vetted library over hand-rolled code.
- **Is it trustworthy?** Who maintains it? Is it widely used and scrutinized, or is it a single unknown author? Does it have a history of timely security responses? A popular, well-maintained package is lower risk than an obscure one with no track record.
- **Is it actively maintained?** An unmaintained dependency accumulates unpatched vulnerabilities. Check the last release, the issue activity, and whether the maintainer is responsive. A dependency with no recent activity is a future incident.
- **What is its own dependency footprint?** A package that pulls in fifty transitive dependencies multiplies your attack surface by fifty. Inspect the dependency tree, not just the direct package.
- **What is the license, and is it compatible?** GPL, AGPL, and other copyleft licenses can impose obligations incompatible with your product. Check before depending, not at release time.

### Pin And Lock Dependencies Reproducibly

The version that runs in production must be the version you reviewed and tested, byte for byte. Floating version ranges (`^`, `~`, `>=`) mean that two builds separated in time may resolve to different code, and that an attacker who publishes a new version of a dependency you depend on can inject code into your next build.

- **Use lockfiles and commit them.** `package-lock.json`, `yarn.lock`, `Cargo.lock`, `go.sum`, `poetry.lock`, `Gemfile.lock`, `pip-tools` output — these pin the exact resolved versions including transitives. Commit them so every environment resolves identically.
- **Pin base images by digest, not just tag.** A container tag like `node:18` moves over time; the image you pull today differs from the one you pulled last month. Pin by digest (`node:18@sha256:...`) for reproducibility, or accept that you must regularly re-scan.
- **Pin build tools and CI actions by commit SHA, not tag.** GitHub Actions and similar systems allow tags to be moved; a compromised or renamed tag can change what your CI runs. Pin to the immutable commit SHA.
- **Decide whether to auto-upgrade or gate upgrades.** Auto-upgrading dependencies (Renovate, Dependabot) keeps you current and patched, but each upgrade is a change that can break behavior or introduce a malicious version. Gate auto-upgrades behind tests and review for critical dependencies.

### Maintain Visibility: Know Exactly What You Depend On

You cannot respond to a vulnerability in a package you do not know you use. Continuous visibility into the full dependency graph, including transitives, is the prerequisite for any supply-chain response.

- **Generate and maintain a Software Bill of Materials (SBOM).** An SBOM (SPDX, CycloneDX) is a machine-readable inventory of every component and its version. It is increasingly required by regulation and is the basis for "are we affected?" when an advisory lands.
- **Run dependency scanning continuously, not just once.** New advisories are published daily. Integrate a scanner (OWASP Dependency-Check, Snyk, GitHub Dependabot, Trivy, `cargo audit`, `npm audit`, `pip-audit`) into CI and on a schedule. Treat new findings as triaged work, not ignored noise.
- **Scan container images, not just application manifests.** A base image brings OS packages, libraries, and tools whose vulnerabilities do not appear in your language's dependency file. Scan the final image.
- **Distinguish direct from transitive dependencies in triage.** A vulnerability in a transitive package you never call directly may be lower risk than one in a package on your hot path, but only if you can confirm the code path is unreachable — which requires knowing the dependency exists.

### Respond To Vulnerabilities With Triage, Not Panic

When an advisory lands, the wrong response is either to ignore it (because everything is always vulnerable to something) or to drop everything and upgrade immediately (because some upgrades break production). The right response is triage based on exploitability and impact.

- **Assess exploitability in your context, not just the CVSS score.** A critical CVE in a library you depend on but never invoke in a reachable path may be a low practical risk. A high-severity issue in code that processes untrusted input on your hot path is urgent. Read what the vulnerability is and whether your usage is affected.
- **Check for a fixed version, and whether the fix is safe to adopt.** Sometimes the patched version is a major upgrade with breaking changes. Weigh the risk of the vulnerability against the risk of the upgrade; for urgent issues, a backported patch or a temporary mitigation may be preferable.
- **Have a path to mitigate without upgrading when necessary.** Compensating controls (input validation, network isolation, disabling the affected feature behind a flag, WAF rules) can reduce risk while a proper fix is prepared. Know which mitigations apply to which classes of vulnerability.
- **Track the finding to closure.** An acknowledged vulnerability that is never fixed is a latent incident. Use the scanner's tracking or a ticket so advisories do not silently age.

### Defend Against Supply-Chain Attack Patterns

Beyond vulnerable-but-honest dependencies, the supply chain includes active adversaries. Recognize and defend against the common attack patterns:

- **Dependency confusion.** An attacker publishes a public package with the same name as one of your private internal packages, and a build that resolves from both public and private registries installs the malicious public one. Use scoped namespaces, private registry configuration that fails closed, and explicit registry pinning.
- **Typosquatting.** An attacker publishes a package whose name is a misspelling of a popular one (`lodahs` for `lodash`). Review dependency names carefully; prefer copy-paste from official docs over typing.
- **Malicious updates to legitimate packages.** A maintainer's account is compromised or coerced, and a new version contains malware. Pinning and lockfiles limit this to when you choose to upgrade; auto-upgrade without review does not.
- **Compromosed build tools and CI actions.** A build plugin, a linter, or a CI action runs with access to your secrets and source. Pin these by SHA and review them as you would any dependency.

## Common Traps

### Adding A Dependency To Avoid A Few Lines Of Code

Pulling in a package for a trivial function (left-pad, is-odd, a single utility), multiplying the attack surface and bundle size for negligible convenience. Write the few lines yourself for trivial functionality.

### Floating Version Ranges In Production

Using `^` or `>=` so that production resolves to whatever is latest at build time, meaning a malicious or buggy new version can be pulled without review. Use lockfiles and pin critical dependencies.

### Ignoring Transitive Dependencies

Reviewing only direct dependencies while the attack surface is dominated by transitives you never explicitly chose. Inspect and scan the full tree.

### Auto-Upgrading Without Review On Critical Paths

Configuring Dependabot/Renovate to auto-merge upgrades to save effort, so a malicious or breaking version ships without a human looking at it. Auto-merge is acceptable for low-risk, well-tested dependencies; it is dangerous for security-sensitive ones.

### Pinning Container Tags Instead Of Digests

Pulling `node:18` and getting a different image over time, so production runs code that was never reviewed or scanned. Pin by digest for reproducibility, or accept and manage the drift.

### Treating CVSS Score As Priority Without Exploitability

Panic-upgrading every critical-CVSS finding regardless of whether your code path is affected, while ignoring a high-severity issue in reachable code. Triage by exploitability in your context, not by the headline number.

### No SBOM Or Inventory When An Advisory Lands

Being unable to answer "do we use the vulnerable package?" quickly, forcing a manual search across every service while customers wait. Maintain an SBOM and continuous scanning.

### CI Actions Pinned By Tag

Pinning GitHub Actions or build plugins by tag, so a moved or compromised tag changes what CI executes. Pin by commit SHA.

## Self-Check

- [ ] Each dependency was evaluated before addition for necessity (could it be a few lines of code?), trustworthiness (maintainer reputation, usage), maintenance activity (recent releases, responsive maintainer), transitive footprint, and license compatibility — not added reflexively for convenience.
- [ ] Dependencies are pinned via committed lockfiles (language lockfiles) and by digest (container base images) or commit SHA (CI actions/build tools), so the version that runs in production is the version that was reviewed and tested.
- [ ] A full dependency inventory (SBOM) is generated and maintained, covering direct and transitive dependencies plus container image contents, and is the basis for answering "are we affected?" when an advisory lands.
- [ ] Dependency and container-image scanning runs continuously in CI and on a schedule, findings are triaged (not ignored as noise), and each finding is tracked to closure with a decision (fix, mitigate, or accept with documented rationale).
- [ ] Vulnerability response is triaged by exploitability in the actual code path and impact, not by CVSS score alone, and compensating controls (input validation, feature flags, isolation) are used to mitigate when a direct upgrade is not immediately safe.
- [ ] Supply-chain attack patterns are defended against: dependency confusion (scoped namespaces, private-registry-fails-closed), typosquatting (names reviewed against official sources), malicious updates (pinning and gated review), and compromised build tools (SHA-pinned CI actions).
- [ ] Auto-upgrade tooling, where used, is configured to gate or review upgrades to security-sensitive dependencies rather than auto-merge blindly.
- [ ] The decision to depend versus vendor is made deliberately, with vendoring considered for small, stable, or security-critical code where supply-chain risk outweighs the maintenance cost of carrying a copy.
