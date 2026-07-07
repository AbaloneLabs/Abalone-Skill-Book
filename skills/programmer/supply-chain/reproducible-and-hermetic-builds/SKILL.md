---
name: reproducible_and_hermetic_builds.md
description: Use when the agent is making builds reproducible (bit-for-bit identical output from the same source) or hermetic (no dependence on the ambient environment), designing build isolation, pinning toolchains and build inputs, reasoning about non-determinism in builds (timestamps, paths, randomness, parallelism order), or diagnosing unreproducible builds (two builds from the same source producing different artifacts). Also covers the failure mode of builds depending on ambient state (the developer's machine, network fetches during build, system libraries) producing works-on-my-machine artifacts, non-determinism making it impossible to verify a binary matches its source (a provenance and supply-chain security gap), and the recurring mistake of treating reproducibility as an aesthetic concern when it is a precondition for trustworthy provenance, deterministic CI, and reliable caching.
---

# Reproducible And Hermetic Builds

A reproducible build produces bit-for-bit identical output from the same source, and a hermetic build depends only on declared inputs (not the ambient environment). The judgment problem is that reproducibility and hermeticity are preconditions for trustworthy software, not aesthetic concerns. A non-hermetic build depends on the developer's machine, the network, or system libraries, producing "works on my machine" artifacts that differ across environments. A non-reproducible build produces different output each time, making it impossible to verify that a binary was built from its claimed source — a gap in supply-chain provenance that an attacker can exploit (ship a binary, claim it came from clean source, and the unverifiability hides the difference). Non-determinism (timestamps embedded in binaries, absolute paths, randomness, parallelism-dependent ordering) breaks reproducibility silently. The discipline is to make builds hermetic (all inputs declared and pinned, no ambient or network dependence) and reproducible (eliminate non-determinism), so that a build can be independently verified against its source and CI caching is reliable.

Agents tend to treat reproducibility as a nice-to-have, because the build produces a working artifact either way. The harm appears as "works on my machine" failures (a build that depends on an ambient library or tool fails in CI or production), as unverifiable binaries (a released binary cannot be confirmed to match its source, undermining provenance), as flaky CI (non-deterministic builds produce different artifacts and cache misses), and as supply-chain attack surface (an attacker can ship a tampered binary that cannot be distinguished from a clean build). The judgment is to declare and pin all build inputs, eliminate non-determinism, and verify reproducibility by building twice and comparing. A build that is not reproducible cannot be proven to come from its source, and a build that is not hermetic cannot be proven to be independent of the environment.

This skill covers build hermeticity, non-determinism elimination, toolchain and input pinning, and reproducibility verification. It complements the sbom-and-provenance skill (provenance attestation), the container-config-and-secret-injection skill (build environment isolation), and the dependency-update-and-renovation skill (pinned dependencies). Here the focus is the build process's determinism and isolation.

## Core Rules

### Make Builds Hermetic: Declare And Pin All Inputs

A hermetic build depends only on declared inputs — source, toolchain, dependencies, and environment — and nothing from the ambient environment:

- **No network fetches during the build.** A build that fetches dependencies or tools from the network at build time depends on the network's state (a dependency can change, a server can go down). Fetch and pin inputs before the build (vendored deps, a pinned mirror, a locked toolchain), and build offline.
- **Pin the toolchain and build tools.** The compiler, linker, and build tools are inputs; their versions affect the output. Pin them (a specific compiler version, a pinned toolchain image) and do not rely on whatever is installed on the build machine.
- **Pin system libraries and the build environment.** System libraries, environment variables, and the OS are inputs; a build that links a system library depends on that library's version. Use a pinned build environment (a container image, a Nix derivation) that declares the full environment.
- **Declare all inputs, including implicit ones.** Time, locale, environment variables, and the build directory path are implicit inputs; declare or fix them so the build does not depend on the ambient values.

### Eliminate Non-Determinism For Bit-For-Bit Reproducibility

Non-determinism — sources of variation unrelated to the source code — breaks reproducibility and must be eliminated:

- **Eliminate embedded timestamps.** Binaries that embed the build time differ each build; use a fixed timestamp (the source commit time, or SOURCE_DATE_EPOCH) rather than the current time.
- **Eliminate absolute paths.** Binaries that embed the build directory's absolute path differ across machines; use relative paths or a fixed build path.
- **Eliminate randomness and unordered iteration.** Random seeds, unordered map iteration, and parallelism-dependent ordering produce different output across runs; use fixed seeds, sorted iteration, and deterministic ordering.
- **Eliminate user and host metadata.** Usernames, hostnames, and process IDs embedded in output differ across environments; strip or fix them.
- **Control parallelism determinism.** Parallel builds can produce different output if order matters (e.g., archive entry order); ensure parallel steps produce deterministic results (sorted output, fixed order).

### Pin The Toolchain And Build Inputs

Pinning makes inputs explicit and immutable, which is necessary for both hermeticity and reproducibility:

- **Pin dependency versions (lockfiles).** A lockfile pins direct and transitive dependency versions so the same versions are used every build; without it, dependency versions float and the output varies (see the dependency-update skill).
- **Pin the toolchain version.** The compiler and tools' versions affect the output; pin them to specific versions and verify the pinned toolchain is used (not a system tool that shadows it).
- **Pin the build environment.** The OS, system libraries, and environment are inputs; pin them via a container image, a Nix derivation, or an equivalent declared environment, and build in that environment consistently.
- **Pin inputs durably.** A pinned input that disappears (a deleted mirror, a yanked package) breaks future builds; pin inputs to durable sources (a controlled mirror, vendored copies) for long-term reproducibility.

### Verify Reproducibility By Building Twice And Comparing

Reproducibility must be verified, not assumed — non-determinism is invisible until two builds are compared:

- **Build the same source twice in different environments and compare.** If the outputs are bit-for-bit identical, the build is reproducible; differences reveal non-determinism to fix. Build in different directories, on different machines, or at different times to surface environment-dependent variation.
- **Use reproducibility verification in CI.** A CI step that builds twice and compares catches non-determinism regressions before release; make it a gate for releases.
- **Use diffoscope or equivalent to diagnose differences.** When builds differ, tools that diff the artifacts (showing which bytes differ and why) localize the non-determinism source (a timestamp, a path, an ordering).
- **Publish reproducibility verification.** For released artifacts, publish that independent rebuilds match (a reproducibility attestation), strengthening provenance (see the sbom-and-provenance skill).

### Connect Reproducibility To Provenance And Trust

Reproducibility is a precondition for trustworthy provenance: only a reproducible build can be independently verified to match its source:

- **A reproducible build lets a verifier confirm a binary came from its source.** An independent party can rebuild from the claimed source and compare; a match proves the binary corresponds to the source, closing a supply-chain trust gap.
- **A non-reproducible build cannot be verified.** If the build is non-deterministic, a verifier cannot distinguish a clean binary from a tampered one by rebuilding; provenance attestation is weakened because it cannot be independently confirmed.
- **Reproducibility supports deterministic CI and caching.** A reproducible build enables reliable caching (the same input yields the same output, so cache hits are correct) and deterministic CI (the same source yields the same artifact, so CI results are comparable).

## Common Traps

### Builds Depending On Ambient State (Network, System Libraries, Machine)

A build that fetches from the network or links system libraries producing environment-dependent artifacts ("works on my machine"). Make builds hermetic: declare and pin all inputs, build offline.

### Embedded Timestamps, Paths, Or Randomness Breaking Reproducibility

Binaries embedding the build time, absolute paths, or random seeds differing each build. Eliminate non-determinism: fixed timestamps (SOURCE_DATE_EPOCH), relative paths, fixed seeds, sorted iteration.

### Assuming Reproducibility Without Verifying

Treating a build as reproducible because it "looks deterministic," missing non-determinism that only a two-build comparison reveals. Verify by building twice in different environments and comparing.

### Unpinned Toolchain Or Floating Dependencies

A compiler version or dependency range that floats, producing different output across builds or time. Pin the toolchain, dependencies (lockfile), and build environment.

### Treating Reproducibility As Aesthetic

Viewing reproducibility as a nice-to-have rather than a precondition for trustworthy provenance, deterministic CI, and reliable caching. Treat it as a supply-chain security requirement.

### Parallelism Producing Non-Deterministic Output

Parallel builds producing different output (e.g., archive entry order depending on thread timing) because order matters. Ensure parallel steps produce deterministic, sorted output.

## Self-Check

- [ ] The build is hermetic: all inputs (source, toolchain, dependencies, system libraries, environment) are declared and pinned, there are no network fetches during the build (inputs fetched and pinned beforehand), and the build runs offline in a pinned environment.
- [ ] Non-determinism is eliminated: no embedded timestamps (fixed via SOURCE_DATE_EPOCH or commit time), no absolute paths (relative or fixed), no randomness or unordered iteration (fixed seeds, sorted), no user/host metadata, and parallelism produces deterministic output.
- [ ] The toolchain, dependencies (lockfile), and build environment are pinned to specific versions, the pinned toolchain is verified to be used (not shadowed by a system tool), and inputs are pinned to durable sources for long-term reproducibility.
- [ ] Reproducibility is verified by building the same source twice in different environments and comparing bit-for-bit, this verification runs in CI as a release gate, and differences are diagnosed with diffoscope or equivalent.
- [ ] The reproducibility-provenance connection is understood: a reproducible build enables independent verification that a binary matches its source (strengthening provenance), and a non-reproducible build cannot be verified (weakening provenance).
- [ ] The highest-risk cases were verified — a network-dependent build failing offline, a timestamp/path breaking reproducibility, a floating toolchain producing different output, and an unverifiable binary undermining provenance — not only the "it builds" state.
