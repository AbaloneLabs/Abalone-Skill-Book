---
name: dockerfile_best_practices.md
description: Use when the agent is authoring or reviewing a Dockerfile at the instruction level, ordering layers for cache efficiency, using BuildKit features such as --mount=type=cache, structuring multi-stage builds with named stages and per-stage build args, choosing between alpine, distroless, and debian-slim bases for a specific runtime, setting up a non-root user with a fixed UID/GID and correct volume ownership, writing a .dockerignore, combining RUN steps and cleaning package caches to shrink layers, wiring ENTRYPOINT vs CMD with signal forwarding, or adding a HEALTHCHECK. Use when a Dockerfile is slow to build, produces a large image, rebuilds dependencies on every code change, runs as root, ignores signals, or fails to shut down cleanly. The image-design philosophy, base-image security, tagging, and scanning are covered by container-image-design; this skill is the concrete instruction-level craft of writing the Dockerfile itself.
---

# Dockerfile Best Practices

A Dockerfile is a build program, not a script. Each instruction produces a cached layer, each layer becomes part of an immutable artifact, and the order and shape of those instructions decide whether a one-line code change rebuilds in two seconds or two minutes, whether the final image is twenty megabytes or two gigabytes, and whether the process inside responds to `SIGTERM` or gets force-killed mid-request. The Dockerfile is also the place where the gap between "it runs on my machine" and "it runs in production" is closed or widened: a sloppy Dockerfile bakes in host-specific assumptions, floating versions, build tools, and root, and ships all of that to every environment identically.

Agents tend to write the first Dockerfile that produces a working image and stop, because the image runs and the costs are invisible until much later. The costs are real and they compound: a build that invalidates the dependency layer on every code change makes CI slow and discourages the frequent rebuilds that keep images current; an Alpine base chosen for size that breaks a glibc-built native extension; a `CMD sh -c` that swallows signals and turns graceful shutdown into a ten-second forced kill; a `.dockerignore` that ships `.git` and `node_modules` into the build context and into a layer. The judgment problem is treating the Dockerfile as an engineering artifact whose instructions have mechanical consequences, and making those consequences work for the image rather than against it.

This skill is the instruction-level craft of writing a Dockerfile. It overlaps with `container-image-design` on base images, multi-stage builds, and non-root users, because those decisions are made *in* the Dockerfile. Where they overlap, this skill covers the *how* — the concrete instruction patterns and the failure modes of each instruction — while `container-image-design` covers the *why*: the security philosophy, the reproducibility and supply-chain posture, and the tagging/scanning program. Use both together.

## Core Rules

### Order Instructions So Volatile Change Invalidates The Least Cache

Docker caches layers by instruction, and a cache miss on one instruction invalidates the cache for *every instruction after it*. This single rule explains almost all build-speed problems. The fix is mechanical: put the things that change rarely at the top, and the things that change often at the bottom.

- **Copy the dependency manifest and install deps before copying application source.** Copy `package.json` + `package-lock.json`, `Cargo.toml` + `Cargo.lock`, `requirements.txt` + a constraints file, `go.mod` + `go.sum` first, run the install, *then* `COPY` the source. When you edit a source file, only the source layer rebuilds; the expensive dependency layer is reused. If you `COPY . .` first and then install, every code change re-runs the full install.
- **Put stable OS setup before anything build-specific.** `apt-get` installs, locale setup, and directory creation belong above the copy of source or config that changes per build.
- **Be aware of what invalidates cache beyond content.** Cache keys include the instruction text, so editing a `RUN` string (even a comment inside it) busts that layer. For `COPY`, the cache key is a checksum of the source files, so touching a file's timestamp without changing content does not bust it under BuildKit, but changing content does. `ADD` from a URL is never cached reliably and should be avoided.

Strong order: `FROM` → system packages → create user/dirs → copy lockfile/manifest → install deps → copy source → build → final stage. Weak order: `COPY . .` near the top, which makes every build a cold build.

### Use BuildKit Cache Mounts For Build Caches That Should Not Live In A Layer

Package managers and build tools keep caches (`~/.cargo`, `~/.m2`, `~/.npm`, pip's wheel cache, Go's build cache) that you want populated across builds but absolutely do not want in the final image. Historically these were either lost (slow rebuilds) or accidentally baked into a layer (bloat). BuildKit's `--mount=type=cache` solves both:

```
RUN --mount=type=cache,target=/root/.cargo/registry cargo build --release
RUN --mount=type=cache,target=/var/cache/apt \
    --mount=type=cache,target=/var/lib/apt \
    apt-get update && apt-get install -y --no-install-recommends curl
```

The cache is persisted by the builder across builds and never written into a layer. Rules for using it well:

- **Give each cache a distinct target and scope.** Use `id=...` and `sharing=locked` (default) or `sharing=shared` when concurrent builds need it. Mixing unrelated caches into one target causes corruption.
- **Pair cache mounts with `--mount=type=bind` for read-only inputs** where you want to avoid copying, but prefer plain `COPY` when the layer should be reproducible from the context.
- **Require BuildKit.** Set `DOCKER_BUILDKIT=1` or use `docker buildx build`; on older daemons `--mount` is a syntax error. Make the build pipeline require BuildKit explicitly.

Cache mounts are the single biggest win for compiled-language build times, and they keep the caches out of the image at the same time.

### Structure Multi-Stage Builds With Named Stages And Selective Copies

A multi-stage build is only as good as the discipline of what you copy out of the builder. Name your stages (`FROM ... AS builder`), reference them by name in later `FROM` and `COPY --from=builder`, and copy *only* the artifact you need, not a directory tree.

- **Copy the smallest thing that runs.** A single compiled binary, a directory of bundled assets, or a virtualenv — not `/src`, not the whole build output. `COPY --from=builder /app/target/release/api /usr/local/bin/api`.
- **Use a separate stage for test/fixtures if needed**, so test data never reaches the final image even transitively.
- **Drive per-stage variation with `ARG` declared in the stage that uses it.** `ARG` scope is the stage it is declared in; declare it inside the builder for build-time values (target arch, feature flags) and inside the final stage for runtime values. A common mistake is declaring an `ARG` at the top and expecting it in a later stage — it is empty there unless re-declared.
- **Prefer distroless or scratch for the final stage** when the runtime allows it. `gcr.io/distroless/nodejs20-debian12`, `scratch` for a static binary. This is where the size and attack-surface win actually lands.

A well-structured multi-stage Dockerfile reads top-to-bottom as builder → final, with the final stage containing only `FROM`, a `COPY --from=builder` of the artifact, a `USER`, and an `ENTRYPOINT`. If the final stage accumulates `apt-get`, source copies, or build tools, the multi-stage split is not doing its job.

### Choose The Base Image By What The Runtime Actually Needs

Base image choice is covered as philosophy in `container-image-design`; here is the instruction-level decision. The question is always "what does the process need at runtime," answered minimally:

- **Static binary (Go, statically-linked Rust) → `scratch` or distroless static.** Nothing else. Smallest, no CVE surface from a base. You must supply CA certificates and timezone data yourself if the binary needs TLS or timezones.
- **Interpreted or dynamically-linked (Node, Python, JVM, Ruby) → distroless runtime image.** Has the runtime and its core deps, no shell, no package manager. Strong default.
- **Need a shell or a specific system package → minimal Debian (`debian:bookworm-slim`) or Alpine.** Step up only when justified, and justify it. A debug shell is not a production justification; ship distroless in prod and keep a separate debug image if needed.

The Alpine/musl trap is real and frequent: a Python wheel or Node native extension built against glibc segfaults or fails to load under musl; some crates and C extensions assume glibc behaviors. If you pick Alpine for size, verify the application and all native dependencies actually run under musl, or accept that the "savings" cost you a week of debugging. When in doubt, `debian-slim` is the boring correct choice; Alpine is the optimization you earn after confirming compatibility.

Pin the base by digest (`node:20.11.1@sha256:...`) for reproducibility, or at minimum by exact version tag (`node:20.11.1`, never `node` or `node:20`).

### Create A Non-Root User With A Fixed UID/GID And Correct Ownership

Running as root is the default and is a defect in any production image. The instruction pattern is to create a user with a *fixed* numeric UID/GID (not a name that resolves differently per environment), own the files it needs, and `USER` into it:

```dockerfile
RUN groupadd --system --gid 10001 app \
 && useradd  --system --uid 10001 --gid app --home-dir /app --shell /usr/sbin/nologin app
RUN mkdir -p /app/data && chown -R app:app /app
USER 10001:10001
```

The subtle part is volumes and write paths:

- **A fixed UID matters because mounted volumes are owned by numeric UID on the host.** If the image uses UID 1000 and the host volume is owned by 1000, writes work; if the image UID is whatever `useradd` picked this build, it is unpredictable. Choose a UID that does not collide with common host users (avoid 1000; 10001+ is conventional).
- **Mark every directory the app writes to as owned by the user.** A `/tmp` or `/data` the process cannot write to is the most common "works locally, permission denied in prod" failure.
- **For distroless images there is no `useradd`.** Use the distroless-provided non-root user (`USER nonroot:nonroot`, UID 65532) or copy a `/etc/passwd` from a builder stage that created it.
- **Do not `chown -R /`** — it is slow and bakes a new layer. Chown only what the runtime needs.

Pair this with dropping capabilities and setting `no-new-privileges` at runtime (the orchestration/security skills cover the runtime side); the Dockerfile's job is to not *require* root.

### Write A .dockerignore That Excludes Everything Not Needed To Build

The build context is everything under the build directory, sent to the daemon before the build starts. Without a `.dockerignore`, that includes `.git` (often the largest thing in a repo), `node_modules`, virtualenvs, local `.env` files with secrets, build outputs, IDE config, and test artifacts. This bloats the context (slow sends), can leak secrets into the context, and — if any of it is `COPY`ed — into a layer.

A strong `.dockerignore` is allow-list-flavored: start by excluding the obvious heavy and sensitive paths, then exclude anything that is not a build input:

```
.git
.gitignore
node_modules
**/__pycache__
*.pyc
.venv
venv
.env
.env.*
docker-compose*.yml
Dockerfile*
target/
dist/
build/
.vscode
.idea
coverage/
*.log
tests/
```

Review it whenever the repo gains a new generated directory. A `.dockerignore` that was correct a year ago is wrong now if a new `coverage/` or `next/` directory appeared. Also note: `.dockerignore` is evaluated against the *context root*, and `COPY` paths must still exist in the context after ignoring — if you ignore something a later `COPY` needs, the build breaks.

### Combine RUN Steps To Reduce Layers And Clean Up In The Same Layer

Each `RUN` is a layer, and files created and deleted in different layers still exist in the earlier layer's history. Two consequences: too many tiny `RUN` instructions produce an image with many layers (historically a limit, still a clarity and size cost), and a `rm` in a later layer does not shrink the image.

- **Combine an install and its cleanup in one `RUN`.** `apt-get update && apt-get install -y ... && rm -rf /var/lib/apt/lists/*` in one layer keeps the package lists out of the image. Splitting them leaves the lists in the install layer forever.
- **Use `--no-install-recommends` with apt** to skip recommended-but-not-required packages, often halving install size.
- **Remove build-only artifacts in the builder stage, not the final stage** — they never get copied out anyway. The final stage should have nothing to clean up because it never received anything extra.
- **Do not over-combine.** A single thousand-line `RUN` is hard to read and a cache miss rebuilds all of it. Combine what changes together; separate what should cache independently.

Squashing (`docker build --squash`, experimental) collapses layers into one, but it is rarely worth the loss of cache granularity. Prefer correct layer ordering and cleanup over squashing.

### Make Builds Reproducible By Pinning And Removing Non-Determinism

A Dockerfile that builds "the same" image today and next month is one where the inputs are pinned and the steps are deterministic. The instruction-level discipline:

- **Pin the base image by digest**, not floating tag. `FROM python:3.12` drifts; `FROM python:3.12.4@sha256:...` does not.
- **Pin dependencies via lockfiles** (`package-lock.json`, `Cargo.lock`, `poetry.lock`, `requirements.txt` with hashed pins). `pip install flask` with no version is non-reproducible.
- **Avoid `apt-get install` without versions where it matters**, and run `apt-get update` and `install` in the same `RUN` so the package index matches the installed versions.
- **Avoid time- and randomness-dependent steps** where possible (timestamps embedded in builds, `latest` fetches). Some toolchains (`SOURCE_DATE_EPOCH` for reproducible builds) let you control this.

Perfect byte-reproducibility is not always achievable, but you should know how reproducible your build is and pin what you can. `container-image-design` covers the reproducibility philosophy; the instruction-level work is the pinning above.

### Wire ENTRYPOINT And CMD For Signal Forwarding And Composability

`ENTRYPOINT` and `CMD` interact, and the form you choose decides whether the process is PID 1, whether it receives signals, and whether the image is composable with extra args.

- **Prefer the exec form (`["binary", "arg"]`) over the shell form (`binary arg`).** The shell form runs under `/bin/sh -c`, which becomes PID 1 and does not forward `SIGTERM` to your process — so the container ignores shutdown and is force-killed after the grace period. The exec form runs your process as PID 1 directly.
- **Use ENTRYPOINT for the fixed command and CMD for default arguments.** This lets operators override args (`docker run image --port 9090`) without retyping the binary, and lets the image serve as a composable unit.
- **If your process is not PID-1-aware, use a minimal init** (`tini`, `dumb-init`, or distroless's `/sbin/tini`) as the entrypoint to reap zombies and forward signals. Languages whose runtimes handle signals correctly (Go, Rust with a signal handler, Node with a handler) can be PID 1 directly; many cannot.
- **A shell-script entrypoint that ends in `exec "$@"`** forwards signals only if the final `exec` replaces the shell with the real process. A script that runs the process as a child does not.

Bad signal handling is invisible until a deploy, when every replica takes the full grace period to die and drops in-flight requests. Test shutdown: `docker stop` a container and confirm it exits in well under the grace period, not after it.

### Add A HEALTHCHECK Only When The Orchestrator Needs It And The App Can Answer

`HEALTHCHECK` lets Docker (and some orchestrators) mark a container unhealthy based on a command's exit code. It is useful for `docker run`/Compose and for platforms that read it, but it is *not* what Kubernetes uses (k8s uses liveness/readiness probes; see the orchestration skill). Add it when:

- **The deployment platform reads `HEALTHCHECK`.** If you target k8s, a `HEALTHCHECK` in the image is ignored; put the probe in the manifest instead.
- **You have a cheap, real liveness signal.** A curl to `/healthz` that hits the app, not a check that a port is open. Avoid checks that pass when the app is wedged (e.g., a static file) or that are expensive enough to load the app.

Use `--start-period` to give the app time to boot before failures count, and `--interval`/`--timeout`/`--retries` tuned to the app's real response time. A `HEALTHCHECK` that is too aggressive restarts a slow-but-healthy app; one that is too loose never fires. Prefer the orchestrator's probes where available, and treat the in-image `HEALTHCHECK` as a fallback for plain Docker.

## Common Traps

### `COPY . .` Before Dependency Install

Copying everything first, then running `npm install` / `pip install` / `cargo build`, invalidates the dependency layer on every source edit and makes every build cold. Copy the manifest and lockfile, install, then copy source.

### A Shell-Form CMD That Swallows SIGTERM

`CMD api --port 8080` (shell form) runs under `sh -c`, so `sh` is PID 1 and your process never sees `SIGTERM`. The container is killed after the grace period, dropping in-flight requests. Use the exec form `CMD ["api", "--port", "8080"]` so the process is PID 1.

### Choosing Alpine And Breaking On musl libc

Switching to `alpine` for size, then discovering a Python C extension, a Node native module, or a Rust crate built against glibc fails or segfaults under musl. Verify musl compatibility for every native dependency before committing to Alpine; otherwise use `debian-slim`.

### Baking A Secret In Via A Build Arg Or A Deleted File

`ARG GITHUB_TOKEN` followed by `RUN git clone ... $GITHUB_TOKEN` leaves the token in the image history even if nothing copies it onward. BuildKit secret mounts (`--mount=type=secret`) keep secrets out of layers; never pass secrets through `ARG` or `ENV`, and never `COPY` a secret in and `rm` it later — the intermediate layer still has it.

### No .dockerignore, So .git And node_modules Enter The Context

The build context balloons, the send is slow, and if a `COPY . .` exists, `.git` history and local `node_modules` land in a layer. Maintain a real `.dockerignore` and review it when the repo gains generated directories.

### Installing And Cleaning Up In Separate RUN Steps

`RUN apt-get install -y curl` then `RUN rm -rf /var/lib/apt/lists/*` — the lists are already baked into the first layer; the `rm` does not shrink the image. Combine update, install, and cleanup in one `RUN`.

### A Fixed Tag That Is Not Actually Fixed

`FROM node:20` is not pinned; upstream moves it across patch releases, so the same Dockerfile builds a different image over time and occasionally breaks. Pin by exact version (`node:20.11.1`) or, for reproducibility, by digest.

### Running As Root Because useradd Was "Complicated"

Skipping the non-root user because the app "just works" as root ships a container where a compromised process is root over the container filesystem. Create a fixed-UID user, own the write paths, and `USER` into it. The two extra lines are not optional for production.

### An ENTRYPOINT Script That Does Not exec The Real Process

A `entrypoint.sh` that runs `node server.js` as a child (without `exec`) keeps the shell as PID 1; signals go to the shell, not the node process, and shutdown hangs. End entrypoint scripts with `exec "$@"`.

## Self-Check

- [ ] Instructions are ordered so dependency manifests and stable setup come first and volatile application source comes last, so a one-line code change does not invalidate the dependency layer.
- [ ] Build caches (cargo, maven, npm, pip, go build) use `--mount=type=cache` so they persist across builds without entering any layer, and the build pipeline requires BuildKit.
- [ ] Multi-stage builds use named stages, copy only the runtime artifact (binary, bundled assets, or virtualenv) into the final stage via `COPY --from=builder`, and the final stage contains no build tools or source.
- [ ] The base image is the smallest the runtime needs (scratch/distroless preferred, debian-slim or alpine only when justified), Alpine is used only after confirming musl compatibility for all native deps, and the base is pinned by digest or exact version.
- [ ] A non-root user with a fixed numeric UID/GID is created, all write paths (data, tmp, cache) are owned by it, and `USER <uid>:<gid>` is set; the image does not require root to run.
- [ ] A `.dockerignore` excludes `.git`, dependency directories, virtualenvs, local env files, build outputs, IDE config, and test artifacts, and was reviewed against the current repo contents.
- [ ] `RUN` steps that install packages combine update, install (`--no-install-recommends`), and cleanup of package lists in one layer, so transient files are not left in earlier layers.
- [ ] Dependencies and base image are pinned (lockfiles, exact versions, digests) and build steps avoid `latest` and time-dependent behavior, so the same source produces a verifiable image.
- [ ] `ENTRYPOINT`/`CMD` use the exec form so the application process is PID 1 (or a minimal init like `tini` is the entrypoint), and `docker stop` exits well within the grace period rather than being force-killed.
- [ ] A `HEALTHCHECK` is present only when the target platform reads it (not for k8s, which uses manifest probes), points at a real liveness endpoint, and uses `--start-period` so boot time is not counted as failure; secrets are injected via BuildKit secret mounts, never via `ARG`/`ENV` or copied-and-deleted files.
