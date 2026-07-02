---
name: reproducible_research_and_computing.md
description: Use when the agent is making computational research reproducible, pinning software environments and random seeds, versioning data and code together, containerizing computational pipelines, ensuring notebook or script results can be regenerated, diagnosing results that differ across machines or runs, or preparing computational work for publication, peer review, or archival.
---

# Reproducible Research And Computing

Reproducibility is the property that a computational result can be regenerated — by the original author months later, by a peer reviewer, or by an independent researcher — from the recorded inputs. It is the foundation that distinguishes a finding from an artifact, because every computational result is the product of a specific combination of code, data, environment, parameters, and randomness, and if that combination is not captured, the result cannot be checked, extended, or trusted. The difficulty is that the combination is invisible at the time the result is produced: the code ran, the environment was whatever was installed, the random seed was whatever the library defaulted to, and the result appeared. It is only later — when the result must be regenerated, or when a slightly different environment produces a different answer — that the missing pieces are discovered, by which point the environment has changed and the original state is unrecoverable.

Agents tend to treat reproducibility as a documentation concern to be handled at publication time, after the result is done. By then the environment has drifted, the intermediate data is gone, the random seed was not recorded, and the notebook that produced the figure has been edited past the state that generated it. The judgment problem is to capture the full computational provenance at the time the work is done — the exact code, the exact data, the exact environment, the exact parameters and seeds — in a form that lets the result be regenerated without relying on memory or on a machine that no longer exists in the same state. Getting this wrong produces results that cannot be reproduced by the author, cannot be verified by reviewers, and quietly diverge when anyone tries to build on them.

## Core Rules

### Pin The Entire Software Environment, Not Just The Code

A computational result depends not only on the source code but on the entire stack that executes it: the language version, the installed libraries and their exact versions, the system libraries, the compiler, and the operating system. Two runs of identical source code on different environments can produce different results because a library version changed an algorithm, the compiler reordered floating-point operations, or a system dependency updated. Pinning only the code (in version control) leaves the environment unpinned, and the environment is where most irreproducibility lives.

Capture the environment as code that can recreate it:

- **Dependency lockfiles** pin exact library versions (the lockfile is the source of truth, not the manifest with ranges).
- **Environment managers** (conda, pip-tools, poetry, renv, Packrat) record the full resolved dependency set.
- **Containers** (Docker, Singularity) or **declarative environments** (Nix) capture the entire stack including system libraries and the OS, so the environment is a single reproducible artifact rather than a set of instructions that may resolve differently.
- **Record the toolchain** (compiler version, flags, runtime version) that affects numerical behavior, because the same source can produce different floating-point results under different compilers or optimization flags.

The standard is that a clean machine, given the recorded environment definition, recreates the exact environment that produced the result. If it cannot, the environment was not fully captured.

### Version Data And Code Together, And Link Them Explicitly

A result is a function of both code and data, and reproducibility requires both to be versioned and linked. Code in version control with unpinned data (a path to a file that may change, a database that may be updated, a URL that may serve different content over time) is not reproducible, because the data the code processed is not the data a future run will process. The data must be versioned and the specific version linked to the specific code version that produced the result.

Practices for data-code linkage:

- **Version the data** with a content hash (a checksum, a DVC pointer, a git-LFS commit) so the exact bytes are identified, not just a filename.
- **Pin remote data sources** by their version or snapshot, not by a live URL that may change. Download data at a recorded point in time and record the source and the retrieval date.
- **Link code and data versions explicitly** in the record of a result: "this figure was produced by commit X of the code, processing data version Y." A result without this linkage cannot be regenerated because the correspondence is lost.
- **Handle derived data** by recording the pipeline that produced it from raw data, so derived data can be regenerated if the raw data or the pipeline is updated. Storing only the derived data, without the pipeline, makes it irreproducible if the derivation needs to change.

### Fix Randomness Explicitly Through Seeding

Stochastic computations (Monte Carlo, random initialization, sampling, stochastic optimization) produce different results on each run unless the random number generator is seeded. An unseeded stochastic computation is irreproducible by definition: the result depends on a random sequence that was never recorded. Worse, the randomness may be hidden inside a library that seeds from the system clock by default, so the computation appears deterministic to the author but is not.

Control randomness explicitly:

- **Seed every random number generator** the computation uses, including those inside libraries (numpy, PyTorch, TensorFlow each have their own RNG state that must be seeded separately).
- **Record the seed** with the result, so the exact random sequence can be reproduced.
- **Be aware that seeding alone does not guarantee bitwise reproducibility** across hardware, library versions, or parallel configurations (see the parallel-scientific-computing and numerical-stability skills). Seeding makes the run reproducible within the same environment; cross-environment bitwise reproducibility requires additionally fixing the evaluation order and the library versions.
- **Control nondeterminism in the compute platform** where it affects results: GPU kernels with atomics, multithreaded reductions, and data loaders with parallel workers can introduce nondeterminism that a seed does not control. Set the platform's deterministic mode where available, and accept that some operations cannot be made bitwise-reproducible without a performance cost.

### Capture Computational Provenance For Every Published Result

A figure, a table, or a number in a publication is the output of a specific computational path: a specific script, run on specific data, in a specific environment, with specific parameters. Reproducibility requires that this path be captured and linked to the result, so the result can be traced back to its inputs and regenerated. Provenance that lives only in the author's memory is lost the moment the project moves on.

For each published result, record:

- **The exact script or notebook** (at a specific commit) that produced it.
- **The data version** it processed.
- **The environment** (from the lockfile or container) it ran in.
- **The parameters and random seed** it used.
- **The computational resources** (if they affect the result, e.g., parallel configuration, GPU model).

Tools that capture provenance automatically (workflow managers like Snakemake, Nextflow, DVC; notebook execution recorders) reduce the manual burden and are more reliable than recording by hand. The standard is that a reader, given the published result and its provenance record, can regenerate the result.

### Make Notebooks Reproducible, Not Just Runnable

Computational notebooks (Jupyter, R Markdown, Pluto) are the medium for much research, and they are also a leading source of irreproducibility. A notebook's state — the values in memory after a sequence of cell executions — depends on the order and the number of times each cell was run, and this state is not captured by the notebook file (which stores only the code and the last outputs). A notebook that produces a figure when run top-to-bottom on a clean kernel may produce a different figure when cells are run in a different order, or when a cell is re-run after editing an earlier cell, because the in-memory state diverges from what a clean run would produce.

Make notebooks reproducible:

- **Restart-and-run-all before trusting a result.** The only reliable test that a notebook reproduces its output is to restart the kernel and run every cell in order. A notebook that has not been restart-and-run-all'd may contain hidden state that a clean run would not reproduce.
- **Clear outputs before committing** so the notebook file does not carry stale outputs that misrepresent the current code, and so the committed file is the code, not a snapshot of one execution.
- **Use notebooks as the record, not just the scratchpad.** If a notebook produces a published result, its clean-run reproducibility must be verified, and it should be versioned with its environment and data like any other script.
- **Prefer parametrized, script-executable notebooks** (papermill, Jupyter Book, R Markdown render) for pipeline stages, so the notebook is executed as a reproducible unit rather than interactively.

### Test Reproducibility Before Relying On It

Reproducibility is a property that must be verified, not assumed. A pipeline that the author believes is reproducible may not be: a dependency resolves differently on a second machine, a data source has updated, a random seed was not actually fixed. The only way to know a result is reproducible is to regenerate it from the recorded inputs on a clean environment, and to confirm the output matches.

Build a reproducibility check into the workflow: a clean-environment regeneration of key results, run before publication or submission, that confirms the recorded inputs produce the recorded outputs. A result that cannot be regenerated from its own recorded provenance is not reproducible, regardless of how thoroughly the provenance was documented — the documentation may be incomplete or wrong, and only regeneration catches the gap.

## Common Traps

### Pinning Code But Not The Environment

Version-controlling the source code while leaving the library versions, system dependencies, and toolchain unpinned, so the same code produces different results in different environments. Capture the full environment as a reproducible artifact.

### Unpinned Or Live Data Sources

Pointing the code at a data file path, a database, or a URL that may change, so a future run processes different data. Version the data by content hash and link the specific version to the code version.

### Hidden Unseeded Randomness

Forgetting to seed a library's internal RNG (so it defaults to the clock), or seeding one RNG but not another, producing results that differ across runs. Seed every RNG and record the seed.

### The Notebook With Hidden State

A notebook whose output depends on cells run in a non-linear order or re-run after edits, so a clean restart-and-run-all produces a different result. Verify notebooks with restart-and-run-all before trusting their output.

### Assuming Seeding Guarantees Cross-Environment Bitwise Reproducibility

Fixing the seed and expecting identical results across hardware, library versions, or parallel configs, when floating-point evaluation order and platform nondeterminism break bitwise reproducibility. Seeding fixes the random sequence; it does not fix the arithmetic.

### Provenance Recorded By Hand After The Fact

Documenting the computational path from memory after the result is done, by which point the environment has drifted and the exact inputs are uncertain. Capture provenance automatically at execution time.

### Derived Data Without The Pipeline

Storing only the processed data, without the pipeline that produced it from raw data, so the result cannot be regenerated if the raw data or the processing changes. Record the derivation pipeline, not just its output.

### Believing Reproducible Without Regenerating

Assuming a result is reproducible because the inputs were recorded, without ever regenerating it on a clean environment to confirm. Verify reproducibility by regeneration; documentation can be incomplete or wrong.

## Self-Check

- [ ] The entire software environment (language, libraries, system dependencies, compiler, OS) is captured as a reproducible artifact (lockfile, container, or Nix derivation), and a clean machine can recreate it — not just the source code.
- [ ] Data is versioned by content hash or snapshot, remote sources are pinned to a retrieval point, and the specific data version is linked to the code version that produced each result.
- [ ] Every random number generator (including those inside libraries) is seeded, the seed is recorded with the result, and platform nondeterminism (GPU atomics, parallel reductions) is controlled or documented as a reproducibility limit.
- [ ] For each published result, the computational provenance (script commit, data version, environment, parameters, seed, compute resources) is recorded and linked to the result, ideally captured automatically by a workflow manager.
- [ ] Notebooks that produce results were verified with restart-and-run-all on a clean kernel, outputs are cleared before commit, and parametrized execution is used for pipeline stages.
- [ ] The distinction between within-environment reproducibility (seeding) and cross-environment bitwise reproducibility (which requires fixing evaluation order and library versions) is understood and documented where relevant.
- [ ] Derived data is accompanied by the pipeline that produced it from raw data, so it can be regenerated, not just the processed output.
- [ ] Reproducibility was verified by regenerating key results from the recorded inputs on a clean environment before publication or submission — not assumed from the documentation.
