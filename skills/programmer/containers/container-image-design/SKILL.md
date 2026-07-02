---
name: container_image_design.md
description: Use when the agent is writing a Dockerfile, designing a container image, choosing a base image, optimizing image size or build speed, setting up multi-stage builds, running containers as non-root, scanning images for vulnerabilities, deciding an image tagging strategy, separating configuration from the image, ensuring build reproducibility, or signing and promoting images through a registry. Also covers layer caching, distroless and scratch bases, immutable tags, secrets injection, SBOM, image registries, and supply-chain hardening of container artifacts. Use when building any container that will run in production, when an image is too large or slow to build, when a vulnerability scan fails, or when auditing container security and reproducibility.
---

# Container Image Design

A container image is the immutable unit that carries your code, its runtime, its dependencies, and its tooling into production. Every choice baked into the image — the base OS, the installed packages, the user it runs as, the tags it carries, the layers it is built from — becomes a property of every deployment that uses it, repeated across every replica in every environment, and very hard to change once it is in production traffic. An image is not "the app plus whatever the base image happened to have." It is a deliberate, reproducible, security-relevant artifact, and treating it casually produces images that are bloated, unreproducible, full of known vulnerabilities, and impossible to roll back to with confidence.

Agents tend to produce a working image quickly (copy the default template, use a fat base, build as root, tag as `latest`) and stop there, because the image runs and the harm is invisible. The harm shows up later: a multi-gigabyte image that slows every deploy and every scale-up; a build that cannot be reproduced because a floating tag drifted; a critical CVE sitting in production for months because the base bundled hundreds of packages the app never uses; a tag named `latest` that points to different code on Monday and Tuesday, making "roll back to the previous version" meaningless. The judgment problem is deciding, for each image, how to make it small, secure, reproducible, and operationally identifiable — and recognizing that these are not optimizations but baseline correctness for anything that runs in production.

## Core Rules

### Choose The Smallest Base Image That Still Runs The Application

The base image determines the attack surface, the size, and the vulnerability exposure of everything built on top of it. A general-purpose distribution (a full Debian or Ubuntu) bundles hundreds of packages — shells, utilities, libraries — that the application never invokes but which still carry CVEs, still get scanned, and still must be patched. Smaller is not cosmetic; smaller is fewer things that can be exploited and fewer things to maintain.

Order of preference, from most to least minimal:

- **`scratch`** — empty filesystem; you supply a static binary and nothing else. Smallest possible, zero CVE surface from a base, but no shell, no package manager, no libc unless you statically link. Ideal for statically-linked Go/Rust services; impractical for anything needing a dynamic libc or a runtime.
- **Distroless** — only the runtime and the application's runtime dependencies, no shell, no package manager, no utilities. Nearly as small as scratch but with a maintained, scanned base. Strong default for interpreted (Python/Node/JVM) and dynamically-linked compiled services.
- **Minimal distribution** (`alpine`, minimal Debian) — a real OS but stripped down. Useful when you genuinely need a shell or a specific package, but it still carries more than distroless and (for Alpine) introduces musl libc, which can break binaries built against glibc.

The decision is driven by what the application actually needs at runtime, not by what is convenient to build. If the production container does not need a shell, do not ship one — a shell inside a container is an attacker's foothold. Pick the smallest base that runs the app, and justify each step up the size ladder.

### Use Multi-Stage Builds To Keep Build Tools Out Of The Final Image

The tools needed to *build* an artifact (compilers, SDKs, build dependencies, test runners, source code) are not the tools needed to *run* it, and shipping the former in the production image is pure cost and risk: a larger image, a larger attack surface, and source code or secrets from the build leaking into production. Multi-stage builds solve this by separating the build environment from the runtime environment:

- A **builder stage** contains the full toolchain, compiles the binary or produces the artifact, and installs build-only dependencies.
- A **final stage** starts from the minimal runtime base and copies *only* the artifact the builder produced — the binary, the bundled assets, the installed runtime deps — nothing else.

The result is a final image that contains the application and its runtime dependencies but not the compiler, the source tree, the test fixtures, or the build caches. This single technique often shrinks images by an order of magnitude and removes entire categories of vulnerability. Apply it to every image where build-time and runtime environments differ, which is most of them.

### Run As A Non-Root User With The Minimum Capabilities Needed

By default, containers run as root, and a process running as root inside a container is root over the container's filesystem and, depending on the runtime and configuration, can escalate toward the host. Running as root is a default, not a requirement, and it should be reversed for any production image:

- **Create and switch to a non-root user** in the Dockerfile, with a fixed UID/GID so filesystem permissions are predictable across environments. Ensure the application files are owned appropriately and writable paths are explicit.
- **Drop capabilities** the container does not need and, where possible, run with a restricted profile (no-new-privileges, read-only root filesystem). A web service rarely needs `CAP_NET_RAW` or `CAP_SYS_ADMIN`; remove them.
- **Do not run a shell as PID 1.** Use the application process or a minimal init; a shell as PID 1 does not handle signals correctly, which breaks graceful shutdown and leaves zombies.

The goal is that even if an attacker compromises the application process, the blast radius is bounded: a non-root user, no shell, no package manager to install tooling, no capabilities to escalate. Defense in depth means no single misconfiguration is catastrophic.

### Order Layers For Cache Efficiency And Put Volatile Layers Last

A container image is a stack of layers, and each layer is cached and reused across builds. The order of instructions determines how much can be reused when the source changes, which determines how fast builds run and how much bandwidth and storage each push consumes. The principle: **things that change rarely go first; things that change often go last.**

- **Dependency manifests before application code.** Copy `package.json` / `Cargo.lock` / `requirements.txt` / `go.mod` and install dependencies in an early layer, then copy the application source in a later layer. When you change a line of application code, the dependency layer is cached and only the code layer rebuilds. If you copy everything at once, every code change invalidates the dependency install, making every build slow.
- **Stable environment before volatile config.** Set up the OS packages, the runtime, and the directory structure before copying anything that changes per build.
- **Combine related operations** where it reduces layers and avoids leaving transient files in intermediate layers (e.g., `apt-get install` and cleanup in one `RUN` so the package lists are not baked into an earlier layer).

Cache efficiency is not just speed; it is also reproducibility and cost. A build that pulls and rebuilds everything every time is slow, consumes registry bandwidth, and discourages the frequent rebuilds that keep dependencies current. A well-ordered Dockerfile makes small changes cheap.

### Use Immutable, Meaningful Tags — Never `latest` For Production

An image tag is how deployments identify what to run, and a tag that can change meaning is an operational hazard. `latest` is the worst offender: it is a floating pointer that resolves to different image digests over time, so "deploy the latest image" means something different today than yesterday, and "roll back to the previous version" is undefined because there is no record of what `latest` pointed to before.

Use tags that are immutable and meaningful:

- **Immutable identifiers** — the git commit SHA, a build ID, or the image digest (content-addressed, cannot change). These uniquely and permanently identify a specific build. This is what production deployments should reference, so that "what is running" and "roll back to the prior build" are unambiguous.
- **Semantic version tags** for released artifacts, treated as immutable once published (never re-tag a different image onto an existing version).
- **Environment or channel tags (`staging`, `prod`)** only as moving pointers *in addition to* the immutable identifier, never as the sole reference, and never re-pointed silently.

The discipline: every production deployment references a specific, immutable identifier; moving pointers are layered on top for convenience but the source of truth is the digest or commit. A registry should enforce immutability (reject re-tagging of existing tags) so that a human error cannot silently change what a tag means.

### Separate Configuration And Secrets From The Image

An image should be the same artifact across all environments; what differs between dev, staging, and production is configuration, and that configuration must not be baked into the image. The image carries code and immutable dependencies; the environment supplies:

- **Configuration** (database hosts, feature flags, log levels, tuning parameters) via environment variables, config files mounted at runtime, or a config service — not hardcoded and not built into a layer.
- **Secrets** (credentials, API keys, tokens, private keys) via a secrets manager, mounted files, or runtime-injected environment variables — *never* baked into the image, never in a build argument that ends up in image metadata, and never in an intermediate layer even if later removed (intermediate layers persist).

Baking a secret into an image is a security incident waiting to happen: the image is replicated to registries, pulled by developers and CI, and often retained long after the secret rotates. Anyone with read access to the image has the secret, and removing it later requires rebuilding from before it was added. Keep the image environment-agnostic; inject everything that varies at runtime.

### Make Builds Reproducible

A reproducible build is one where the same source produces a byte-identical (or at least behaviorally-identical) image, so that "this is the image built from commit X" is a verifiable claim rather than a hope. Reproducibility matters because it underpins trust in the deployment: if you cannot reproduce the artifact, you cannot confirm what is actually running, and supply-chain verification (signing, attestation) becomes meaningless.

Threats to reproducibility:

- **Floating tags and unpinned versions** in the base image (`FROM node` instead of `FROM node:20.11.1@sha256:...`) — the same Dockerfile builds a different image as the upstream tag moves. Pin base images by digest for reproducibility, or at least by exact version.
- **Non-deterministic build steps** — fetching "latest" dependencies, network installs that vary by time, build tools that embed timestamps or random values. Pin dependency versions (lockfiles) and control sources of randomness.
- **Build context that varies by machine** — depending on the builder's local state, timezone, or environment. Make the build self-contained.

You do not need perfect byte-reproducibility in every case, but you should know how reproducible your build is, pin what you can, and treat "we cannot reproduce what is in production" as a problem to fix, not a curiosity.

### Scan For Vulnerabilities And Act On Findings With A Triage Policy

Every image carries the vulnerabilities of its base and its dependencies, and an unscanned image in production is an unknown risk. Scan images in the pipeline (on build, on push to the registry, and periodically for images already deployed, since new CVEs are discovered against unchanged images). But scanning produces signal only if there is a triage policy:

- **Fix or block on critical, exploitable vulnerabilities** in components the application actually uses. A CVE in an installed-but-unused package is lower priority than one on the request path.
- **Distinguish exploitable from theoretical.** A CVE in a build tool that never runs in production, or in a library loaded only in a path the app does not exercise, is a different risk from one reachable by an attacker. Triage by reachability and impact, not by raw count.
- **Define what blocks a release versus what is tracked.** A policy that blocks on every CVE, including unexploitable ones, paralyzes releases; a policy that ignores all findings leaves real risk unaddressed.
- **Rebuild regularly against patched bases.** The durable fix for base-image CVEs is keeping the base current, not chasing individual patches.

The goal is not zero findings (unrealistic) but a known, triaged, declining risk surface, with the most exploitable issues fixed fastest.

### Sign Images And Verify Provenance Through The Registry

An image in a registry is trusted only if you can verify it came from your build system and was not tampered with in transit or storage. Signing and provenance attestation close the supply-chain gap between "the build produced this" and "production is running this":

- **Sign images** with a key or identity (Sigstore/cosign, Notation, or your registry's built-in signing) so deployments can verify the image is authentic.
- **Attest provenance** — record where the image was built from (commit, build parameters, base images) so you can confirm the running artifact corresponds to reviewed source.
- **Verify at admission** — configure the deployment runtime to reject unsigned or unattested images, so a substituted or malicious image cannot run even if it reaches the registry.

This is the container equivalent of the dependency-trust discipline applied to your own artifacts. An unsigned image running in production is an assumption that the registry and the build pipeline were not tampered with — an assumption worth verifying rather than making silently.

## Common Traps

### Using `latest` As A Production Tag

`latest` floats, so two deploys of "latest" can run different code, and "roll back" has no meaning. Production must reference immutable identifiers (digest or commit SHA); `latest` is acceptable only for local dev convenience, never as the deployment target.

### Shipping The Build Toolchain In The Production Image

A single-stage build from a full SDK image leaves the compiler, source, and build deps in the final image — bloated, vulnerable, and leaking build details. Use multi-stage builds so the final image contains only the runtime artifact.

### Running As Root "Because It Works"

The container runs as root by default and the app happens to work, so it ships that way. A compromised app process then has root over the container and a path toward host escalation. Create a non-root user, drop unneeded capabilities, and treat root-in-container as a defect to fix, not a default to accept.

### Baking Secrets Into A Layer (Even A Deleted One)

A secret passed as a build argument or copied in and `rm`'d in a later layer is still present in the intermediate layer's history — anyone who pulls the image can recover it. Secrets must be injected at runtime, never embedded in the image build. If a secret was baked in, treat it as compromised and rotate it.

### A Floating Base Tag That Drifts

`FROM python:3` resolves to different patch versions over time, so the same Dockerfile builds a different image next month, breaking reproducibility and occasionally introducing a regression. Pin the base by digest (or at least exact version) so a build is what you think it is.

### Prioritizing Image Size Over Correctness Or Compatibility

Chasing the smallest possible image by switching to Alpine, then discovering the app breaks under musl libc, or stripping a CA bundle so TLS verification fails, or removing a library the runtime needs. Minimize within the constraint that the image must actually run correctly in production; size is a goal, not the goal.

### Scanning But Never Triaging

The scanner reports hundreds of CVEs, the team ignores them all because the list is overwhelming, and a critical exploitable one is buried in the noise. Scan with a triage policy that distinguishes exploitable from theoretical and blocks on the former; an unread scan report is worse than no scan because it creates false confidence.

### Re-Tagging An Existing Immutable Tag

Pushing a new image to an existing version tag (`v1.4.2` now points to different content). This silently changes what deployments pull and destroys the ability to know what ran. Enforce immutability in the registry so tags cannot be overwritten.

### A Shell As PID 1 That Breaks Shutdown

Using `CMD ["sh", "-c", "..."]` or an entrypoint shell script as PID 1, which does not forward signals, so `SIGTERM` is ignored and the container is killed forcefully after a timeout — dropping in-flight requests. Run the application process directly as PID 1, or use an init that forwards signals, so graceful shutdown works.

## Self-Check

- [ ] The base image is the smallest that runs the application (scratch/distroless preferred, minimal distribution only when justified), chosen to minimize attack surface and CVE exposure rather than for build convenience.
- [ ] Multi-stage builds keep the build toolchain, source code, and build-only dependencies out of the final image, which contains only the runtime artifact and its runtime dependencies.
- [ ] The container runs as a non-root user with a fixed UID/GID, unneeded Linux capabilities are dropped, no-new-privileges / read-only root filesystem is set where feasible, and the application process (not a shell) is PID 1 so signals and graceful shutdown work.
- [ ] Dockerfile layers are ordered for cache efficiency: dependency manifests and stable setup first, volatile application code last, and transient files are not left in intermediate layers.
- [ ] Production deployments reference immutable identifiers (image digest or git commit SHA); `latest` and other floating tags are never the deployment target, and the registry enforces tag immutability so existing tags cannot be silently overwritten.
- [ ] Configuration and secrets are injected at runtime (env vars, mounted files, secrets manager) and are not baked into the image; no secret was ever passed as a build argument or left in an intermediate layer.
- [ ] Base images and dependencies are pinned (by digest or exact version) and builds are reproducible — the same source produces a verifiable image, with sources of non-determinism controlled.
- [ ] Images are scanned in the pipeline and periodically once deployed, with a triage policy that fixes or blocks on critical exploitable vulnerabilities (by reachability and impact), distinguishes theoretical from exploitable, and rebuilds against patched bases to keep the risk surface declining.
- [ ] Images are signed and their provenance attested, and the deployment runtime verifies signatures/attestations at admission so unsigned or tampered images cannot run.
