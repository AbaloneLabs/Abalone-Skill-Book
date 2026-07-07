---
name: elixir_mix_releases_and_deployment.md
description: Use when the agent is building or deploying an Elixir/Erlang release (mix release, Elixir 1.9+ releases, Mix.Release config, runtime configuration with config/runtime.exs and RELEEX/config providers), configuring the BEAM for production (cookie, EPMD, node name, system limits), running in Docker/Kubernetes, hot code upgrades (appups/relups, mix release --upgrade), clustering nodes (libcluster, distributed Erlang), handling OTP application startup and supervision, or is diagnosing "release cannot find config", "runtime.exs vs config.exs confusion", "BEAM cookie mismatch / node cannot connect", "port already in use / EPMD", "release too big / missing assets", or deployment/restart-strategy issues. Covers mix releases and runtime config, BEAM distribution (cookie/EPMD/node naming), clustering, Docker/K8s packaging, hot upgrades vs rolling restarts, and supervision/startup correctness.
---

# Mix Releases And Deployment In Elixir

Elixir's deployment story centers on *releases* (self-contained BEAM bundles with the app, its dependencies, the Erlang runtime, and a start script), built by `mix release` (built-in since Elixir 1.9). Releases differ fundamentally from source/Mix-based dev runs: they are precompiled, use runtime configuration (`config/runtime.exs`) evaluated at boot rather than compile-time `config/config.exs`, and package the BEAM. Agents confuse compile-time and runtime config (a secret baked at compile time leaks across environments; `runtime.exs` exists to defer to boot), mismatch the distribution cookie (nodes cannot connect), misconfigure node naming for clustering, ship oversized images (the release includes the full BEAM), or attempt hot code upgrades without understanding `appup`/`relup`. The judgment problem is to build a release with correct runtime config, to configure BEAM distribution deliberately (cookie, node name, EPMD), to package for Docker/K8s appropriately, and to choose rolling restarts vs hot upgrades by operational maturity.

Agents bake secrets at compile time, mismatch cookies, ship bloated images, or attempt hot upgrades naively. The remedy is `runtime.exs` for boot-time config, deliberate distribution setup, multi-stage Docker builds, and rolling restarts unless hot upgrades are justified.

## Core Rules

### Use runtime.exs For Boot-Time Configuration; Do Not Bake Secrets At Compile Time

`config/config.exs` and `config/prod.exs` are evaluated at *compile* time — values are baked into the release. This is appropriate for compile-time concerns (the HTTP port in a URL, a compile-time feature flag) but wrong for secrets and environment-specific values, which would be frozen into the artifact and leak across environments. `config/runtime.exs` (Elixir 1.11+) is evaluated at *boot* time, reading env vars/config providers at startup — use it for database URLs, secret keys, pool sizes, and anything environment-specific. Structure: keep `config/config.exs` for compile-time defaults; put all runtime/env-specific config in `runtime.exs` reading `System.get_env("VAR")` with clear error messages when a required var is missing. A release without `runtime.exs` bakes the wrong config; a release with secrets in `config/prod.exs` leaks them.

- `config/config.exs`/`prod.exs` are compile-time (baked in); use only for compile-time concerns.
- `config/runtime.exs` is boot-time; use for secrets, DB URLs, env-specific values, reading `System.get_env`.
- Fail loudly in `runtime.exs` if a required env var is missing (clear error at boot, not a silent nil later).

### Configure BEAM Distribution Deliberately (Cookie, Node Name, EPMD)

For a clustered/distributed BEAM, each node needs a shared *magic cookie* (the authentication secret between nodes — mismatched cookies prevent connections), a *node name* (`name` for fully-qualified/FQDN, `sname` for short names — pick one and be consistent; you cannot mix), and EPMD (the Erlang Port Mapper Daemon, TCP 4369, that resolves node names to ports). Set the cookie via `RELEASE_COOKIE` env var (release) or `--cookie` flag; never commit it. Node naming: `--name node@fqdn.example.com` (long names, routable) vs `--sname node@host` (short names, same host/subnet). Open the distribution port range (EPMD 4369 + the ephemeral node ports) in firewalls/security groups. `libcluster` automates cluster formation (gossip, Kubernetes, DNS strategies); configure its strategy deliberately.

- Shared `RELEASE_COOKIE` for all nodes; never commit it; mismatched cookies block connections.
- `--name` (FQDN, long) or `--sname` (short, same subnet) — pick one consistently; do not mix.
- EPMD (TCP 4369) + node distribution ports must be open; `libcluster` for automated formation.

### Package Releases For Docker/Kubernetes With Multi-Stage Builds

A release should run in a minimal image. Use a multi-stage Dockerfile: build stage compiles the release (full Elixir/Erlang image, `mix deps.get`, `mix compile`, `mix assets.deploy`, `mix release`), runtime stage copies only the release (`_build/prod/rel/<app>`) into a slim image (alpine or debian-slim with the needed libc). This keeps images small (the BEAM is ~50MB; a source image is far larger). Run as a non-root user. Set `MIX_ENV=prod` at build. For Kubernetes, use a Deployment/StatefulSet with proper liveness/readiness probes (a `/health` endpoint), resource limits (the BEAM needs bounded memory to avoid OOM-kills; set `ERL_AFLAGS` for scheduler limits), and a rolling update strategy.

- Multi-stage Dockerfile: build stage compiles the release; runtime stage copies only the release into a slim image.
- Run as non-root; set `MIX_ENV=prod` at build; bundle assets (`mix assets.deploy`) before release.
- K8s: liveness/readiness probes, resource limits (bound BEAM memory), rolling updates.

### Prefer Rolling Restarts Over Hot Code Upgrades Unless Operationally Mature

The BEAM supports *hot code upgrades* (replacing code without stopping the VM, via `appup`/`relup` files), which is a celebrated feature but operationally demanding: you must author correct `appup` instructions (how to transform each process's state across the version), handle supervision tree changes, and test the upgrade path. For most teams, *rolling restarts* (replace nodes one at a time via the orchestrator) are simpler, safer, and sufficient — zero-downtime without hot-upgrade complexity. Reserve hot upgrades for true in-place, stateful, cannot-restart scenarios (long-lived processes that cannot lose state), and only with a tested `appup`/`relup` pipeline (often via `distillery`-style tooling or hand-authored appups). Do not attempt hot upgrades ad hoc.

- Rolling restarts (orchestrator replaces nodes) for zero-downtime — simpler and safer for most teams.
- Hot code upgrades only for stateful cannot-restart cases, with tested `appup`/`relup` instructions.
- Do not attempt hot upgrades ad hoc; the state-transformation logic is error-prone.

### Ensure OTP Application Startup And Supervision Are Correct

A release starts the OTP application per its `mod` callback (`Application.start/2`), which starts the top supervisor. Ensure: (1) the supervision tree is correct (children in dependency order, `rest_for_one`/`one_for_one` strategies chosen by coupling); (2) `init/1` of long-running GenServers is fast (slow init delays startup and trips K8s readiness timeouts — push slow work to `handle_continue`); (3) `Application.start` returns `{:ok, pid}` promptly; (4) graceful shutdown via `Application.stop`/`init.stop` closes sockets/DB connections (the BEAM traps SIGTERM if configured; set `ERL_AFLAGS` or the release's `vm.args` for signal handling). A misbehaving supervisor (a child that crashes on init in a `one_for_all` tree) prevents the app from starting.

- Supervision tree correct (child order, restart strategy by coupling); `Application.start` returns promptly.
- GenServer `init` is fast (slow work in `handle_continue`); readiness probes account for startup time.
- Graceful shutdown closes resources; SIGTERM handling configured.

## Common Traps

### Baking Secrets In Compile-Time Config

`config/prod.exs` with a secret key freezes it into the release. Use `runtime.exs` and env vars.

### Cookie Mismatch Between Nodes

Nodes cannot connect. Share `RELEASE_COOKIE`; never commit it.

### Mixing Long And Short Node Names

`--name` and `--sname` cannot interconnect. Pick one consistently.

### EPMD / Distribution Ports Blocked

TCP 4369 + node ports closed in the firewall. Open them; or use an alternative discovery.

### Oversized Docker Image

A source image or a single-stage build ships gigabytes. Multi-stage build copying only the release.

### Attempting Hot Upgrades Ad Hoc

Untested `appup` state transformations corrupt process state. Use rolling restarts unless mature.

### Slow GenServer init Tripping Readiness Probes

A slow `init` delays app start; K8s kills the pod. Push slow work to `handle_continue`.

### Missing Graceful Shutdown

SIGTERM kills the BEAM without closing connections. Configure signal handling and `Application.stop`.

## Self-Check

- [ ] All environment-specific values and secrets are in `config/runtime.exs` (boot-time, reading `System.get_env`), not baked into compile-time `config/prod.exs`; missing required vars fail loudly at boot.
- [ ] BEAM distribution is configured deliberately: shared `RELEASE_COOKIE` (not committed), consistent node naming (`--name` or `--sname`, not mixed), EPMD (4369) and distribution ports open, `libcluster` strategy chosen if clustering.
- [ ] The release is packaged in a minimal image via a multi-stage Dockerfile (build stage compiles; runtime stage copies only the release), run as non-root with `MIX_ENV=prod` and bundled assets.
- [ ] Kubernetes/Docker deployment uses liveness/readiness probes, resource limits (bounded BEAM memory), and a rolling update strategy.
- [ ] Hot code upgrades are used only for justified stateful cannot-restart cases with tested `appup`/`relup` instructions; rolling restarts are the default for zero-downtime.
- [ ] OTP application startup is correct: supervision tree (child order, restart strategy), fast GenServer `init` (slow work in `handle_continue`), prompt `Application.start`, and graceful shutdown on SIGTERM.
- [ ] The release has been considered under config-bake, cookie mismatch, port/firewall, image size, and startup/shutdown scenarios, and remains correct.
- [ ] Release overlays/config providers (if used) are documented and tested across environments.
