---
name: research_version_control.md
description: Use when the agent is versioning research code, data, or documents, setting up a repository for a study, writing commit messages, tagging analysis versions, or deciding what should and should not be tracked.
---

# Research Version Control

Version control is how a research project keeps a faithful history of what was done, when, by whom, and why. Without it, the project has only its current state and the fading memory of the team. When version control is absent or misused, three harms follow. A result cannot be traced back to the exact code and data that produced it, so reproducibility fails. Errors introduced weeks ago cannot be found because there is no history to search. And collaborative edits overwrite each other silently, so no one knows which version is authoritative. Version control is not a developer convenience; it is the audit trail that lets a study defend its own results.

The agent should use this skill when setting up a research repository, deciding what to track, writing commits and tags, handling data and outputs under version control, or rescuing a project that has lost its history. The goal is to keep the agent from treating version control as optional infrastructure, when it is the mechanism that connects a published result to the precise conditions that produced it.

## Core Rules

### Put Code, Configuration, And Documentation Under Version Control

Anything that affects the results should be versioned. This includes the following.

- Analysis scripts and notebooks.
- Configuration files and parameters.
- Documentation, protocols, and data dictionaries.
- Manuscript drafts and supplementary materials.
- Environment definitions, such as dependency files or container specifications.

Versioning these artifacts means any result can be tied to a specific state of the project. A result produced by an unversioned script floating on a desktop cannot be reproduced or trusted once the script changes.

### Decide Deliberately What To Track About Data

Data require a different strategy from code, because data can be large, sensitive, or changing. Decide explicitly what to track.

- Small, non-sensitive, derived data can live in the repository.
- Large raw data belong in dedicated storage or a data repository, referenced by location and identifier.
- Sensitive or participant data must never go in a public repository, even if it seems de-identified.
- Changing data need either a versioned store or a record of which version was used in each analysis.

A common pattern is to track code in version control, store data externally, and record in the code the exact data version or hash used for each analysis. Mixing large data into a code repository bloats history and makes the repository unusable.

### Write Commits That Explain Why, Not Just What

A commit message that says fixed bug or updated analysis tells future readers nothing. The diff already shows what changed; the message should explain why.

A useful commit message includes the following.

- A short summary of the change.
- The reason for the change, what problem it solves or what decision it reflects.
- Reference to an issue, decision, or analysis step where relevant.

Treat commit messages as part of the research record. Months later, the reason for a change is exactly what the team will need and will have forgotten.

### Tag And Release Analysis Versions

A result reported in a paper corresponds to a specific state of the code and data. Tag that state with a meaningful label, such as a version number or the manuscript stage, and record it.

- Tag the exact commit used for each figure or table.
- Record the data version or hash used alongside the tag.
- Note the software environment used for the run.
- Preserve the tag so the result remains reproducible even as the project continues.

A paper that cannot point to a tagged version of its own analysis is a paper whose results may drift as the code evolves. Tags freeze the result in time.

### Use Branches To Isolate Experimental Changes

Experimenting directly on the main line of history mixes tried and abandoned approaches with the authoritative analysis. Use branches for the following.

- Exploratory analyses that may not survive.
- Alternative model specifications.
- Refactors that might break results.
- Collaborative work before it is ready to merge.

Keep the main branch in a known-good state, so that the current authoritative analysis is always clear. Merging experimental work back should be a deliberate decision, not an accident.

### Track The Environment, Not Just The Code

Code that ran today may not run next year because dependencies change. Version the environment too.

- Pin dependency versions in a lockfile or requirements file.
- Use containers or environment specifications that capture the full stack.
- Record the operating system, language versions, and key library versions.
- Prefer reproducible environment tools over ad hoc installation notes.

A result tied to code but not to its environment is only partially reproducible. Dependency drift is a leading cause of replication failure.

### Connect Version Control To Reproducible Workflow

Version control is most powerful when paired with a reproducible workflow, where a single command rebuilds results from data and code. Structure the repository so that the following is clear.

- Where the raw data enter.
- How they are cleaned and transformed.
- How analyses and figures are generated.
- How the manuscript pulls in results.

A repository where a newcomer can run one command and regenerate the paper's figures is far more trustworthy than one where results depend on undocumented manual steps.

### Manage Collaboration Through Version Control

Version control is also a collaboration tool. Use it to coordinate rather than to overwrite.

- Use pull requests or merge requests for reviewed changes.
- Require review for changes to the main analysis.
- Resolve conflicts explicitly rather than forcing pushes.
- Keep the history readable so the team can follow what happened.

Emailing scripts back and forth, or editing a shared drive with no history, destroys the audit trail that version control exists to provide.

### Protect Sensitive Information From Accidental Commits

Repositories leak secrets and sensitive data through careless commits. Protect against the following.

- Credentials, keys, and tokens committed by accident.
- Participant data or identifiers pushed to a public repository.
- Internal documents not meant for public release.

Use ignore files, pre-commit checks, and careful review of what is staged. Once sensitive data are pushed to a public repository, removing them is difficult because the history retains them. Prevention is far easier than cure.

## Common Traps

### Versioning Only The Final Script

The history of how the analysis evolved is part of reproducibility. Committing only the final version loses the reasoning behind changes.

### Committing Large Or Sensitive Data

Large data bloat the repository, and sensitive data can expose participants. Store data externally and reference it.

### Vague Commit Messages

Messages that say what instead of why leave the team unable to understand past decisions. Explain the reason.

### Forgetting To Tag Results

Without a tag, a reported result cannot be tied to a specific code state. Tag every reported analysis.

### Editing The Main Branch Directly For Experiments

Mixing experiments into the main line makes the authoritative analysis unclear. Use branches.

### Ignoring The Environment

Code without pinned dependencies may not run later. Version the full environment.

### Relying On Manual Steps

Undocumented manual steps break reproducibility. Automate the workflow from data to results.

### Pushing Secrets Or Participant Data

Accidental commits of credentials or identifiers are hard to undo. Prevent them before staging.

## Self-Check

- Are code, configuration, documentation, and environment definitions under version control?
- Is there a deliberate strategy for what data are tracked versus referenced externally?
- Do commit messages explain why changes were made, not just what changed?
- Are reported results tagged to specific commits with data versions recorded?
- Are experimental changes isolated in branches, keeping the main branch authoritative?
- Is the software environment versioned and reproducible, including dependencies?
- Can a single command rebuild the results from data and code?
- Is collaboration handled through reviewed merges rather than overwriting shared files?
- Are sensitive data and credentials protected from accidental commits?
- For shared, long-term, or published repositories, has an experienced reproducibility advisor or research software engineer reviewed the version control setup?
