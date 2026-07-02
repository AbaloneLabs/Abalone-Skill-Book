---
name: python_packaging_and_dependency_management.md
description: Use when the agent is setting up or refactoring a Python project's packaging, choosing between pyproject.toml and setup.py, configuring build backends (setuptools/hatchling/flit/poetry), managing dependencies and lockfiles (pip-tools, poetry, uv, pipenv), designing virtual environment strategy, building wheels and sdist distributions, writing PEP 440 version specifiers and classifiers, publishing to PyPI, or structuring a monorepo with multiple installable packages. Covers dependency resolution, reproducibility, optional/extras dependencies, and supply-chain pinning.
---

# Packaging And Dependency Management

Python packaging is a set of overlapping standards (PEP 517, 518, 621, 660, 703) layered on decades of tooling. "It works on my machine" is the default failure mode: the same `requirements.txt` resolves differently across machines and times, a transitive dependency updates and breaks production, or a build that worked locally fails on a clean CI because an implicit system dependency was never declared. The judgment problem is making the project reproducible, portable, and honest about its dependencies.

Agents tend to copy a `requirements.txt` from memory, pin nothing or everything, mix the abstract dependency list with the locked environment, and assume `pip install` is deterministic (it is not — resolution depends on the index state at install time). The harm is delayed: builds that pass today and fail next month, security vulnerabilities from unpinned transitive deps, and libraries that refuse to install because their constraints conflict. The real work is separating "what we depend on" from "exactly what we installed," choosing a build backend deliberately, and versioning dependencies with intent.

## Core Rules

### Use pyproject.toml As The Single Source Of Project Metadata

Modern Python packaging centers on `pyproject.toml` (PEP 518/621). It declares the build backend, project metadata, dependencies, optional extras, entry points, and tool configuration (mypy, ruff, pytest) in one file. Prefer it over `setup.py`/`setup.cfg` for new projects.

- `setup.py` is now only needed for genuinely dynamic build logic that a declarative backend cannot express. Most projects need none.
- Pick a build backend deliberately: `setuptools` (mature, familiar, verbose), `hatchling` (fast, modern, flexible), `flit` (simple, for pure-Python single-module packages), `poetry-core`/`pdm-backend` (paired with their CLIs). The choice affects build behavior, dynamic versioning, and editable installs.
- Put metadata under `[project]` (PEP 621) so it is backend-agnostic and tool-readable. Use `[build-system]` to pin the backend and its build requirements.

### Separate Abstract Dependencies From The Locked Environment

This is the most important distinction and the most commonly blurred one.

- **Abstract dependencies** (`[project.dependencies]`): the set of packages and version *ranges* your project needs to run, e.g., `requests >=2.28, <3`. These are what you publish.
- **Locked environment** (`requirements.lock`, `uv.lock`, `poetry.lock`): the *exact* resolved versions, including transitives, for reproducible installs. This is what you deploy.

A library publishes abstract dependencies (so downstream can resolve) and usually does not commit a lock. An application commits a lock (so deploys are reproducible) and may keep abstract deps minimal. Mixing the two — pinning exact versions in `[project.dependencies]` — over-constrains downstream and causes resolution conflicts. Use a tool (`pip-tools`, `uv`, `poetry`, `pdm`) to compile the lock from the abstract spec.

### Pin With Intent, Following PEP 440 Semantics

Version specifiers are a contract about compatibility, not a habit. PEP 440 defines the operators; use the one that matches your compatibility promise.

- `>=X` — "at least X, anything newer." Loose; risks breakage from upstream changes.
- `~=X.Y` (compatible release) — "X.Y.*", allows patch/minor updates within Y. Good default for libraries that follow semver-ish releases.
- `==X.Y.Z` — exact. Appropriate in lockfiles, over-constraining in abstract deps.
- `===X.Y.Z` — arbitrary equality (literal match). Rare; for non-PEP-440 versions.

For libraries, specify the minimum version you actually tested and an upper bound only where you know a future version breaks you (and document why). Unbounded `>=` invites future breakage; a blanket `<4` on a 3.x library is often cargo-cult. For applications/lockfiles, pin exact versions including hashes when supply-chain integrity matters (`pip install --require-hashes`).

### Manage Virtual Environments Consistently

A virtual environment isolates a project's dependencies from the system Python and from other projects. Decide a strategy and apply it uniformly so CI, local dev, and Docker all match.

- One environment per project; never install project deps into the system interpreter.
- Pin the Python version itself (`requires-python = ">=3.11"`) so resolution and syntax features are consistent.
- Use a tool that makes environment creation reproducible (`uv`, `poetry`, `venv` + `pip-tools`, `hatch env`). Document the exact commands in your README or `Makefile`.
- In containers, create the venv as a non-root user, install from the lockfile, and avoid layering dev dependencies into the runtime image.

### Handle Optional Dependencies And Extras Explicitly

Not every consumer needs every feature. Use `[project.optional-dependencies]` (extras) to group optional sets: `dev`, `test`, `docs`, feature-gated deps like `ml = ["torch", "transformers"]`. Consumers install with `pip install yourpkg[dev,ml]`.

Decisions:

- Keep the core install lean; move heavy or niche deps into extras so a minimal install stays fast and secure.
- Document which extras exist and what they enable.
- For test/dev tooling that is never shipped, a separate `requirements-dev.txt` or a `dev` extra is fine; do not bloat the runtime dependency set.

### Choose Distribution Artifacts By Audience

A wheel (`.whl`) is a pre-built binary archive; a sdist (`.tar.gz`) is the source distribution. They serve different audiences.

- **Wheels** install fast and reliably because they skip the build step. Always build wheels when possible, including platform wheels for C extensions (using `cibuildwheel` for cross-platform binaries).
- **sdists** are the fallback when no compatible wheel exists or when a downstream needs to vendor/patch source. Always publish an sdist alongside wheels so your package remains buildable from source.
- Pure-Python packages produce a single `py3-none-any` wheel; extension modules produce per-platform/per-Python wheels.

### Structure Monorepos Around Installable Packages

In a monorepo, treat each deployable component as its own installable package with its own `pyproject.toml`, even if they live in one repo. This keeps boundaries explicit (see the module-boundary-design skill) and lets CI build, test, and version each independently.

- Use a workspace-aware tool (`uv workspace`, `hatch` monorepo, `pants`, `pdm`) or a shared lock plus per-package metadata.
- Decide whether internal packages depend on each other by path/editable install or by published version; the former is faster for development, the latter enforces real release boundaries.
- Avoid a single giant `requirements.txt` at the repo root that mixes all packages' deps — it destroys per-package reproducibility.

## Common Traps

### Pinning Everything In `[project.dependencies]`

Copying exact versions (`requests==2.31.0`) into the abstract dependency list over-constrains downstream and creates resolution conflicts when another package needs a different version. Reserve exact pins for the lockfile; use ranges in published metadata.

### Committing Only `requirements.txt` For An Application

A plain `requirements.txt` without a lock and without hashes is resolved fresh on each install and can change as the index changes. Use a lockfile (`uv.lock`, `pip-compile` output with hashes) so deploys are bit-for-bit reproducible and tamper-evident.

### Assuming `pip install` Is Deterministic

Without a lock, `pip install -r requirements.txt` resolves against the current index state and may pick newer transitive versions each time. Reproducibility requires a lock, a fixed index, and ideally hashes.

### Forgetting `requires-python`

Omitting `requires-python` lets the package install on interpreters where it cannot run (wrong syntax features, missing stdlib modules), producing confusing runtime errors. Always declare the minimum Python version.

### Shipping Dev Dependencies In The Runtime Set

Putting `pytest`, `mypy`, `black` in `[project.dependencies]` forces every consumer to install them. Move them to a `dev`/`test` extra or a separate dev requirements file.

### Dynamic Versioning Without A Source Of Truth

If the version is read from `__version__`, a git tag, or a separate file at build time, make sure that source is the single truth and that the build backend is configured to read it (`dynamic = ["version"]`). Drift between the published version and the in-code version causes release confusion.

### Mixing Build Backends Mid-Project

Switching from `setuptools` to `hatchling` (or vice versa) without migrating all metadata and build hooks breaks editable installs, dynamic versioning, and CI. Plan the migration as a deliberate change, not a casual edit.

### Not Publishing sdists

Wheels-only publishing means a consumer on an unsupported platform/Python cannot build from source. Always publish an sdist so the package remains buildable when no wheel matches.

## Self-Check

- [ ] Project metadata lives in `pyproject.toml` under `[project]` and `[build-system]`; `setup.py` exists only for genuine dynamic build logic.
- [ ] Abstract dependencies (published ranges) are separate from the locked environment (exact versions + hashes) used for deployment.
- [ ] Version specifiers follow PEP 440 and express a real compatibility promise; exact pins live in the lockfile, not in published metadata.
- [ ] `requires-python` declares the real minimum, and the Python version is consistent across local dev, CI, and containers.
- [ ] Optional/dev dependencies are in extras or separate dev files, not in the runtime dependency set.
- [ ] Both wheels and an sdist are produced and publishable; C extensions have per-platform wheels via `cibuildwheel` or equivalent.
- [ ] Virtual environment strategy is documented and identical across local, CI, and Docker.
- [ ] In a monorepo, each deployable package has its own metadata and the internal dependency model (path vs published version) is explicit.
- [ ] The version source of truth is single and correctly wired to the build backend.
- [ ] Installing from the lockfile on a clean machine reproduces the exact dependency set with matching hashes.
