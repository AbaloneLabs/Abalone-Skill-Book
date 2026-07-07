---
name: container_image_and_build.md
description: Use when the agent is building or optimizing container images (Docker, OCI), writing Dockerfiles, designing multi-stage builds, choosing base images, minimizing image size and layer count, managing image security (distroless, non-root user, vulnerability scanning), optimizing build cache and layer ordering, or designing the image build and publish pipeline. Covers base image selection, multi-stage builds, layer caching and ordering, image minimization, build reproducibility, supply-chain integrity (signing, provenance), and the security posture of the image itself (least privilege, scan, reduce attack surface).
---

# Container Image And Build

A container image is the deployable unit of a service, and its construction determines the service's security posture, startup speed, build performance, and supply-chain integrity. The image is not just "the app packaged up" — it is the entire environment the app runs in, including every library, tool, and shell that accompanies it. A naive image built from a full OS base, with the build toolchain included, running as root, accumulating layers in a poorly-ordered Dockerfile, is larger than necessary (slow to pull and start), carries a larger attack surface (the build tools and shell an attacker can use), and builds slowly (cache invalidated on every code change). The disciplined image is built from a minimal base, excludes everything the runtime does not need, runs as a non-root user, is reproducible from a version-controlled definition, and is scanned and signed so its integrity is verifiable. Container image construction is a security and performance decision, not just a packaging step.

Agents tend to write Dockerfiles that work (the image builds and runs) without considering the base image's attack surface, the layer ordering's effect on cache, or the runtime's privilege level, and they treat the image as trusted without scanning or signing. The judgment problem is recognizing that the image is the service's runtime environment and attack surface, that layer ordering and multi-stage builds determine build performance and image size, that the base image and privilege level determine the security posture, and that supply-chain integrity (scanning, signing, provenance) is what makes the image trustworthy. This skill covers the discipline of container image construction: base image selection, multi-stage builds, layer caching, minimization, reproducibility, and supply-chain security.

## Core Rules

### Choose A Minimal, Appropriate Base Image

The base image determines the image's starting size, the available tools (and attack surface), and the maintenance burden (security updates).

- **Prefer minimal bases: distroless, scratch, or slim variants.** A distroless or scratch base contains only the runtime and the application, with no shell, package manager, or extraneous tools — minimizing size and attack surface. A slim variant (debian-slim, alpine) is a middle ground. Avoid full OS bases (ubuntu, debian full) for production images unless a specific tool is required.
- **Match the base to the runtime's needs.** A statically-linked binary (Go, Rust) can run from scratch (just the binary). A runtime requiring a libc or shared libraries (Python, Node, JVM) needs a base with those libraries. Choose the minimal base that provides what the runtime needs.
- **Use official, maintained base images.** Official images (from the language runtime's maintainers, or the distro's maintainers) receive timely security updates and are widely scrutinized. Avoid unknown or unmaintained bases that may have unpatched vulnerabilities or malicious content.
- **Pin base image versions for reproducibility.** A base image tagged `latest` changes over time, producing non-reproducible builds. Pin to a specific version or digest (sha256) so the same Dockerfile produces the same base across builds, and updates are deliberate.

### Use Multi-Stage Builds To Exclude Build-Only Artifacts

A single-stage build includes the build toolchain (compilers, SDKs, dev dependencies) in the final image, inflating size and attack surface. Multi-stage builds separate building from running.

- **Build in one stage, copy the artifact to a minimal runtime stage.** The first stage has the full build tools (compiles the binary, bundles dependencies); the final stage is a minimal base with only the built artifact copied in. The build tools do not reach the final image.
- **This dramatically reduces size and attack surface.** A Java image with the JDK (build) is hundreds of MB; with only the JRE or a jlinked runtime (final stage), it is far smaller. A Go image with the toolchain vs. a scratch stage with the binary is the difference between hundreds of MB and a few MB.
- **Name and target the final stage explicitly.** Use named stages (FROM ... AS builder) and target the final stage (docker build --target final) so the build produces the minimal image, not an intermediate.
- **Copy only what the runtime needs.** In the copy step, include only the binary, the runtime dependencies, and the required config — not the source, the tests, or the build caches.

### Order Dockerfile Layers For Cache Efficiency

Docker builds images in layers, and each layer is cached. A layer that changes invalidates the cache for it and all subsequent layers, so the order determines how much is rebuilt on a code change.

- **Order from least-frequently-changing to most-frequently-changing.** Put stable steps first (base image, system package installs, dependency manifests), volatile steps last (copying application code). A code change then only rebuilds the final layers, not the dependency installation.
- **Copy dependency manifests and install dependencies before copying code.** Copy package.json/requirements.txt/Cargo.toml and install dependencies in an early layer; copy the application code in a later layer. A code change does not invalidate the dependency layer, so dependencies are not reinstalled on every code change — a major build-speed win.
- **Combine related commands to reduce layer count.** Each RUN/COPY/ADD creates a layer. Combine related commands (a RUN with && chained commands) to reduce layers and image size. Fewer layers mean smaller images and faster pulls.
- **Clean up in the same layer that creates waste.** A RUN that installs packages and removes the cache in separate layers leaves the cache in an intermediate layer (the image still contains it). Clean up (apt cache, pip cache) in the same RUN command so the waste does not reach the image.

### Run As Non-Root With Least Privilege

A container running as root can do anything the root user can inside its namespace; if an attacker escapes the container or exploits the application, root privilege amplifies the damage. Running as a non-root user limits the blast radius.

- **Create and switch to a non-root user.** Add a non-root user (USER directive, or a numeric user) and run the application as that user. The application then cannot modify system files or perform root-only operations inside the container.
- **Drop all Linux capabilities not needed.** Containers run with a default set of capabilities; drop all that the application does not need (CAP_NET_RAW, CAP_SYS_ADMIN, etc.) via docker run --cap-drop. Least privilege limits what a compromised container can do.
- **Make files read-only where possible.** A read-only filesystem (--read-only) prevents an attacker from writing payloads or modifying the application. Use it where the application does not need to write to the filesystem (or mount specific writable volumes).
- **Avoid privileged mode.** --privileged grants the container nearly all host capabilities, defeating the isolation. Never use it in production; if a specific capability is needed, grant just that capability.

### Ensure Reproducibility And Supply-Chain Integrity

An image should be reproducible from its definition (the same inputs produce the same image) and its integrity should be verifiable (the image deployed is the image built, unmodified).

- **Pin all versions for reproducibility.** Base images (by digest, not just tag), dependencies (lockfiles), and build tools (pinned versions) ensure the same Dockerfile produces the same image. Unpinned inputs produce drift across builds.
- **Scan images for known vulnerabilities.** A vulnerability scanner (Trivy, Grype, Snyk) checks the image's packages against vulnerability databases, flagging known CVEs. Scan on every build; block or warn on high-severity findings; update affected packages.
- **Sign images to verify integrity.** Image signing (cosign, Notation) cryptographically signs the image, so deployment can verify the image is the one built by the trusted pipeline, unmodified. This prevents deployment of a tampered or substituted image.
- **Attach provenance (SLSA) for traceability.** Provenance metadata records how the image was built (the source commit, the build commands, the builder identity), providing an auditable chain from source to image. This supports supply-chain security (SLSA levels) and incident investigation.

### Optimize The Build Pipeline

The image build runs frequently (on every commit or release); its performance affects developer productivity and CI cost.

- **Use BuildKit for parallel and cache-efficient builds.** BuildKit (the modern Docker build backend) builds stages in parallel, caches more efficiently, and supports advanced features (mount caches for package managers). Enable it (DOCKER_BUILDKIT=1 or the default in modern Docker).
- **Use a registry cache for CI builds.** CI builds that start cold rebuild everything; a registry cache (or a local cache shared across CI runs) reuses layers across builds, dramatically speeding up CI.
- **Build once, promote across environments.** Build the image once (the same artifact), promote it through environments (dev, staging, production) with environment-specific config applied at deploy. Do not rebuild per environment — that defeats artifact consistency (see deployment-parity).
- **Keep builds fast to maintain developer flow.** A slow build (many minutes) discourages iteration and delays feedback. Optimize layer ordering, use caches, and minimize the build scope to keep builds fast.

## Common Traps

### Full OS Base With Build Tools In The Final Image

A base like ubuntu with the compiler and dev dependencies included, inflating size and attack surface. Use multi-stage builds; copy only the artifact to a minimal base.

### Poor Layer Ordering (Rebuilding Dependencies On Every Code Change)

Copying code before installing dependencies, so every code change rebuilds the dependency layer. Copy manifests first, install dependencies, then copy code.

### Running As Root

A container running as root, amplifying the damage if the application is compromised. Create and use a non-root user; drop unneeded capabilities.

### Unpinned Base Image (latest)

A base tagged `latest` that changes between builds, producing non-reproducible images. Pin to a specific version or digest.

### No Vulnerability Scanning

An image with known vulnerabilities in its packages, deployed unscanned. Scan on every build; address high-severity findings.

### Unsigned Image (No Integrity Verification)

An image deployed without signature verification, allowing a tampered or substituted image to run. Sign images; verify signatures at deployment.

### Per-Environment Rebuilds

Building a different image per environment, defeating artifact consistency. Build once; promote the same image with environment-specific config.

### Waste Left In Intermediate Layers

Package caches or build artifacts left in intermediate layers (cleaned in a later layer), inflating image size. Clean up in the same layer that creates the waste.

## Self-Check

- [ ] The base image is minimal (distroless, scratch, or slim), matched to the runtime's needs, official and maintained, and pinned to a specific version or digest for reproducibility.
- [ ] Multi-stage builds separate the build environment (with toolchain) from the final runtime image (minimal, artifact-only), dramatically reducing size and attack surface, and the final stage is explicitly targeted.
- [ ] Dockerfile layers are ordered from least- to most-frequently-changing (base, packages, dependency manifests, dependency install, code copy), so code changes rebuild only the final layers, and related commands are combined with cleanup in the same layer to reduce size.
- [ ] The container runs as a non-root user with least privilege (capabilities dropped, read-only filesystem where feasible, no privileged mode), limiting the blast radius of a compromised application.
- [ ] The build is reproducible (all versions pinned) and the image is scanned for known vulnerabilities on every build (with high-severity findings addressed), signed for integrity verification (cosign, Notation), and has provenance metadata for traceability.
- [ ] The build pipeline is optimized: BuildKit for parallel and cache-efficient builds, registry or shared caching for CI, build-once-promote-across-environments (the same image deployed with environment-specific config), and builds kept fast for developer productivity.
- [ ] The image's startup behavior is considered: the application starts quickly (no heavy initialization blocking readiness), handles SIGTERM for graceful shutdown (see deployment skills), and the image includes health check metadata where appropriate.
- [ ] Supply-chain integrity covers the full path: the base image is trusted and scanned, dependencies are pinned and scanned, the build is reproducible and signed, and deployment verifies the signature — so the running image is verifiably the one built from the reviewed source.
