---
name: package_trust_and_verification.md
description: Use when the agent is evaluating whether to trust and adopt a third-party package or dependency, defending against typosquatting and dependency confusion attacks, pinning dependency versions and verifying integrity, configuring registry and scope policies, or auditing an existing dependency set for trustworthiness. Also covers the failure mode of installing a package based on popularity or name without verifying reputation, account-takeover of a maintainer publishing malicious versions, dependency confusion pulling from public registries over private ones, and unpinned or floating versions that silently introduce untrusted code.
---

# Package Trust And Verification

Every dependency is code you did not write, executing with your application's privileges, and adopting one is an act of trust that is rarely examined as carefully as it should be. The judgment problem is that the signals developers use to choose a package — popularity, a familiar name, the fact that it works — are exactly the signals attackers exploit. Typosquatting registers a package with a name one character different from a popular one, relying on typos and familiarity; dependency confusion tricks a package manager into pulling a public package with the same name as your private one, executing attacker code inside your build; a compromised maintainer account publishes a malicious version of a legitimate package, riding its reputation. The discipline is to verify a package before adopting it (reputation, maintainer history, source, activity), to pin versions and verify integrity so a future change cannot silently introduce untrusted code, to configure registries and scopes so private names cannot be shadowed by public ones, and to treat the dependency set as an attack surface that is reviewed periodically rather than adopted once and forgotten.

Agents tend to install packages to get a function working and move on. The harm appears as malicious packages slipping into the dependency tree (via typosquatting, confusion, or compromised maintainers), as builds that silently pull new untrusted versions because nothing is pinned, and as a dependency set no one has reviewed since inception, accumulating trust debt. The judgment is to evaluate a package's trust before adoption, to pin and verify integrity so trust is stable, to defend against the named attacks (typosquatting, confusion, takeover) with configuration and process, and to audit the dependency set periodically so trust is maintained, not assumed. A dependency is a security decision made quickly; the discipline is to make it deliberately.

## Core Rules

### Verify A Package Before Adopting It

Before adding a dependency, evaluate whether it is trustworthy. The signals that matter are the maintainer's reputation and history, the package's provenance and activity, and the absence of red flags — not merely popularity or that it works.

- **Examine the maintainer and history.** Who maintains it, for how long, with what track record? A package maintained by a known, reputable team with a long clean history is more trustworthy than one created recently by an unknown account.
- **Examine the source and repository.** Is the published package built from a public, reviewable source? A package whose source is not inspectable is harder to trust.
- **Examine activity and responsiveness.** A maintained package (recent updates, responsive to issues) is less likely to be abandoned-and-adopted (a common attack vector) than a dormant one.
- **Check for red flags.** A name very similar to a popular package (typosquatting), a recent maintainer change, a sudden version jump with odd changes — these warrant extra scrutiny.
- **Prefer packages from reputable sources and organizations** over unknown individual accounts where the stakes are high, while recognizing that individual maintainers can be trustworthy.

### Pin Versions And Verify Integrity

Trust in a dependency is trust in a specific version; floating versions (a range, "latest") silently introduce new code on every install, code that has not been re-verified. Pin the exact version and verify its integrity so the dependency cannot change without an explicit, reviewed update.

- **Pin exact versions, not ranges.** A pinned version (`=1.2.3`, a lockfile with a hash) ensures every install resolves to the same code; a range (`^1.2.3`) resolves to whatever is latest, which may be untrusted.
- **Verify integrity (hashes, signatures).** A lockfile with hashes, or signed packages, ensures the resolved artifact is the one you verified; without integrity verification, a compromised registry could serve a different artifact for the same version.
- **Review and update dependencies deliberately.** Updates should be a reviewed change (what changed, is the new version still trusted), not an automatic drift; a periodic, reviewed update cadence balances security and stability.
- **Use lockfiles and commit them.** The lockfile records the exact resolved dependency tree; committing it ensures every build and every contributor uses the same verified set.

### Defend Against Typosquatting, Dependency Confusion, And Account Takeover

The named attacks against the package supply chain are well-understood, and each has a defense. Apply the defenses rather than hoping the attacks do not target you.

- **Typosquatting: verify the exact name and prefer copy-paste over typing.** A package whose name differs by one character from a popular one is a common attack; verify the name against the official source, and prefer copying the install command rather than typing it.
- **Dependency confusion: configure scopes and registries so private names resolve only to private registries.** A package manager that falls back to a public registry for a name that should be private can be tricked into pulling an attacker's public package; configure scoped registries so private scopes never fall back to public.
- **Account takeover: prefer packages with multi-maintainer or organizational control, and watch for maintainer changes.** A package controlled by a single account is one credential compromise away from a malicious publish; organizational control or 2FA on the maintainer account reduces the risk.
- **Monitor for the indicators of these attacks.** A new version from a new maintainer, a name change, a sudden scope expansion — these warrant pausing adoption or update.

### Configure Registry And Scope Policies

The package manager's configuration determines where packages can be pulled from and what is allowed. Configure it defensively so the attack paths (public fallback for private names, untrusted registries) are closed.

- **Configure scoped registries so private scopes resolve only to private registries.** This is the primary defense against dependency confusion; ensure the package manager will not fall back to a public registry for a private name.
- **Restrict which registries are allowed.** Allowing only trusted registries (and disallowing arbitrary public registries) reduces the attack surface.
- **Use proxy or internal registries where feasible.** An internal registry that proxies and vets public packages gives a control point for approval and scanning.
- **Apply policies consistently across environments.** A policy that applies in CI but not locally (or vice versa) leaves a gap; ensure the configuration is the same everywhere.

### Audit The Dependency Set Periodically

Trust in a dependency is not permanent; maintainers change, packages are abandoned and adopted, vulnerabilities are discovered. Audit the dependency set periodically so trust is maintained, not assumed at adoption and never revisited.

- **Periodically review the dependency set for continued trust.** Are the maintainers still reputable? Have packages been abandoned or transferred? Have licenses or behaviors changed?
- **Remove unused dependencies.** A dependency that is no longer used is still an attack surface; remove it.
- **Track and respond to advisories.** Subscribe to vulnerability advisories for the dependencies in use and respond when one affects you (see the vulnerability-triage skill).
- **Prefer fewer dependencies.** Every dependency is trust debt; the fewer you have, the smaller the surface and the less there is to audit.

## Common Traps

### Adopting On Popularity Or Name Without Verification

Installing a package because it is popular or has a familiar name, without examining the maintainer, history, or source, falling for typosquatting or a compromised package. Verify reputation, maintainer history, source, and activity before adopting.

### Unpinned Or Floating Versions

A dependency specified as a range or "latest," silently introducing new untrusted code on every install. Pin exact versions, verify integrity (hashes, signatures), and review updates deliberately.

### Dependency Confusion Via Public Fallback

A package manager that falls back to a public registry for a private name, pulling an attacker's public package into the build. Configure scoped registries so private scopes never fall back to public.

### Ignoring Account-Takeover Risk

A package controlled by a single account with no 2FA, one credential compromise away from a malicious publish. Prefer multi-maintainer or organizational control and 2FA; watch for maintainer changes.

### Trust Assumed Once And Never Revisited

A dependency adopted and never re-audited, accumulating trust debt as maintainers change, packages are abandoned, and vulnerabilities emerge. Audit the dependency set periodically; remove unused dependencies; track advisories.

### No Integrity Verification

A pinned version without integrity verification (no hash, no signature), so a compromised registry could serve a different artifact for the same version. Verify integrity with hashes or signatures.

### Configuration Gaps Across Environments

Registry and scope policies applied in CI but not locally (or vice versa), leaving a gap an attacker can exploit. Apply policies consistently everywhere.

## Self-Check

- [ ] Packages are verified before adoption: maintainer reputation and history, source and repository reviewability, activity and responsiveness, and red-flag checks (typosquatting-similar names, recent maintainer changes, sudden version jumps) are examined, with preference for reputable sources and organizations where stakes are high.
- [ ] Dependencies are pinned to exact versions (not ranges or "latest"), integrity is verified (lockfile hashes or signatures), lockfiles are committed, and updates are reviewed and deliberate rather than automatic drift.
- [ ] Defenses against named attacks are applied: typosquatting (verify exact name, copy-paste over typing), dependency confusion (scoped registries so private scopes never fall back to public), and account takeover (multi-maintainer/organizational control, 2FA, monitoring for maintainer changes).
- [ ] Registry and scope policies are configured defensively (private scopes resolve only to private registries, allowed registries restricted, internal proxy/vetting where feasible) and applied consistently across all environments.
- [ ] The dependency set is audited periodically (maintainers still reputable, packages not abandoned/transferred, licenses/behaviors unchanged), unused dependencies are removed, advisories are tracked and responded to, and fewer dependencies are preferred to reduce the surface.
- [ ] The highest-risk cases were verified — a typosquat caught by name verification, a dependency-confusion attempt blocked by scoped registry configuration, a pinned version with integrity verification preventing a swapped artifact, and a periodic audit catching an abandoned-and-transferred package — not only the clean install-works path.
