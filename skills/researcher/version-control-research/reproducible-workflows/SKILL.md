---
name: reproducible_workflows.md
description: Use when the agent is building a reproducible analysis workflow, automating a pipeline from raw data to results, choosing workflow tools, managing dependencies, or ensuring a study can be rerun end to end.
---

# Reproducible Workflows

A reproducible workflow is one where a defined process turns raw data into results, and that process can be rerun to produce the same outputs. This sounds simple, but most research analyses are not reproducible in practice. They depend on manual steps, undocumented choices, specific software versions, and files that live on one person's machine. When a workflow is not reproducible, three harms follow. The original team cannot regenerate its own figures after a small change. Reviewers and readers cannot verify how a result was produced. And the result silently drifts as the environment changes, so the published numbers no longer match what the code produces. Reproducibility is not a virtue; it is the condition under which a result can be trusted at all.

The agent should use this skill when designing an analysis pipeline, automating data-to-result steps, choosing workflow or notebook tools, managing computational environments, or preparing a study for others to rerun. The goal is to keep the agent from producing results that depend on undocumented manual steps, when the entire value of a computational result is that it can be regenerated from its inputs.

## Core Rules

### Make The Whole Path From Data To Result Explicit

A reproducible workflow has no hidden steps. Map the entire path.

- Where raw data enter the pipeline.
- How data are cleaned, validated, and transformed.
- How derived variables and features are constructed.
- How models are fit or analyses are run.
- How tables, figures, and numbers are generated.
- How the manuscript or report incorporates the outputs.

If any step is a manual click, a copy-paste, or an undocumented edit, the workflow is not reproducible at that point. Make every step explicit and scripted.

### Automate With A Single Entry Point

A reproducible workflow can be triggered by one command that runs the whole pipeline. The user should not need to know the order of scripts or run them by hand.

Approaches include the following.

- A makefile or task runner that encodes dependencies and order.
- A workflow manager that tracks inputs and outputs.
- A master script that calls each stage in sequence.
- A notebook or report generator that executes code and renders output.

The single entry point is what lets a reviewer or future team member regenerate everything without reverse-engineering the process.

### Track Dependencies Between Steps

Steps in a workflow depend on each other. Changing the cleaning step should trigger re-running the analysis, not silently leave stale results. Track these dependencies.

- Each output should know its inputs.
- When an input changes, dependent outputs should be rebuilt.
- Stale outputs should be detectable, not assumed correct.

Workflow tools handle this automatically. Without dependency tracking, the team must remember what to rerun, and they will forget, producing inconsistent results.

### Pin And Capture The Computational Environment

Results depend on the software environment as much as on the code. Capture it.

- Pin package versions in a lockfile.
- Specify language versions.
- Use containers or virtual environments that capture the full stack.
- Record the operating system where relevant.
- Prefer environment specifications over prose installation notes.

A workflow that runs today but not next year, because a library updated, is only temporarily reproducible. Environment capture extends reproducibility across time.

### Separate Raw Data From Derived Data

Raw data should be treated as immutable. All cleaning and derivation produce new files, never overwrite the raw source.

- Keep raw data read-only.
- Write all transformations to derived files.
- Make the transformation code the source of truth for derived data.
- Never edit raw data by hand to fix a problem; fix it in the pipeline.

Editing raw data silently destroys the ability to trace results back to their source. The pipeline must be the only path from raw to derived.

### Use Seeds And Control Randomness

Random processes, such as sampling, splitting, and stochastic model fitting, must be controlled for reproducibility.

- Set random seeds explicitly.
- Record the seed used for each result.
- Be aware that some operations, such as parallel processing or GPU computation, may not be fully deterministic even with a seed.
- Document the remaining sources of nondeterminism.

A result that depends on an unrecorded random seed cannot be reproduced exactly. Record seeds and acknowledge limits.

### Validate Inputs And Outputs At Each Stage

A pipeline can silently produce wrong results if inputs are malformed or outputs are implausible. Build in checks.

- Validate input schemas and ranges at the start.
- Check for unexpected missingness or duplicates.
- Assert that outputs fall within expected bounds.
- Log warnings for anomalies rather than failing silently.

Validation catches problems early, when they are cheap to fix, rather than after they propagate into published figures.

### Document The Workflow For Re-Use

A workflow that only its author can run is not reproducible in any meaningful sense. Document it.

- A readme explaining how to run the pipeline.
- The expected inputs and where to get them.
- The expected outputs and where they appear.
- The runtime and resource requirements.
- Known issues and limitations.

Documentation turns a working pipeline into a usable one. Without it, even a technically reproducible workflow dies when the author moves on.

### Test The Workflow By Rebuilding From Scratch

The real test of reproducibility is rebuilding from scratch on a clean environment. Do this before publication.

- Clone the repository fresh.
- Set up the environment from the specification.
- Obtain the data as a new user would.
- Run the single entry point.
- Compare outputs to the reported results.

If the rebuild fails or differs, the workflow is not yet reproducible, regardless of how it looks on paper. This test catches the hidden assumptions that undermine reproducibility.

## Common Traps

### Leaving Manual Steps In The Pipeline

A manual click or copy-paste breaks reproducibility at that point. Script every step.

### Running Scripts By Hand In Order

Relying on memory for script order invites mistakes. Use a single entry point with tracked dependencies.

### Overwriting Raw Data

Editing raw data destroys traceability. Treat raw data as immutable.

### Forgetting To Set Or Record Seeds

Unrecorded randomness makes results unreproducible. Set and document seeds.

### Assuming The Environment Will Stay Stable

Dependencies drift. Pin and capture the environment.

### Trusting Outputs Without Validation

Silently wrong outputs propagate into results. Validate at each stage.

### Documenting Only For The Author

A workflow only its author can run is not reproducible in practice. Document for a stranger.

### Never Testing A Clean Rebuild

Without a from-scratch test, hidden assumptions survive. Rebuild on a clean environment before publishing.

## Self-Check

- Is the entire path from raw data to result explicit, with no hidden manual steps?
- Can the whole pipeline be triggered by a single command?
- Are dependencies between steps tracked so stale outputs are detected?
- Is the computational environment pinned and captured, including versions and containers?
- Are raw data kept immutable, with all derivation happening in the pipeline?
- Are random seeds set and recorded, with nondeterminism acknowledged?
- Are inputs and outputs validated at each stage?
- Is the workflow documented well enough for a newcomer to run it?
- Has the workflow been rebuilt from scratch on a clean environment with outputs matching?
- For complex, published, or shared workflows, has a research software engineer or reproducibility advisor reviewed the pipeline before release?
