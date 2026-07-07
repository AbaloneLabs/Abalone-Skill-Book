---
name: design_tooling_and_pipelines.md
description: Use when the agent is selecting, configuring, or reviewing design tooling and pipelines, including design systems tooling, token pipelines, component libraries, design-to-code sync, plugin and automation choices, asset export pipelines, Figma or Sketch workflows, CI checks for design, and version control for design artifacts.
---

# Design Tooling And Pipelines

Design tooling is infrastructure, not preference. The choice of tools, plugins, token pipelines, and export workflows determines whether a design system stays consistent as it scales or drifts into fragmentation. Most teams choose tools based on familiarity and then discover, months later, that the pipeline cannot express the constraints the product actually needs: conditional tokens, platform-specific outputs, accessible component variants, or automated checks against spacing and color.

Use this skill when setting up, migrating, or auditing the tooling that carries design intent into production. The goal is to prevent the agent from recommending a stack that looks modern but cannot enforce the system's rules, or from over-automating a pipeline that the team cannot maintain.

## Core Rules

### Choose Tools Against Required Capabilities, Not Popularity

A tool decision should start from the constraints the product and team impose, not from what is trending. Before selecting or migrating tools, enumerate the capabilities that are non-negotiable.

Capability questions to answer first:

- Does the tool support the component and variant model the system needs, including nested variants and property controls?
- Can it express design tokens at the granularity required (color, spacing, typography, radii, elevation, motion)?
- Does it support the platforms the product ships to, including web, iOS, Android, and any embedded or kiosk contexts?
- Can it integrate with the engineering component library and version control workflow?
- Does it support the accessibility review needs, such as contrast checking and semantic annotation?
- Can multiple designers work concurrently without destructive conflicts?

A tool that scores well on demos but cannot meet these requirements will force workarounds that erode the system.

### Treat Tokens As The Contract Between Design And Code

Tokens are the most leveraged part of a design pipeline. When tokens are authoritative and shared, design and code stay aligned automatically. When tokens are decorative, drift is inevitable.

A robust token pipeline should:

- define tokens in a single source of truth, not duplicated across files and tools;
- separate primitive tokens (raw values), semantic tokens (purpose-bound, such as `color.text.error`), and component tokens where needed;
- transform tokens into platform-specific outputs, accounting for the different formats each platform expects;
- version tokens so that changes are intentional and reviewable;
- be consumable by both the design tool and the codebase, ideally through automation rather than manual sync.

Decide early whether the source of truth lives in code, in the design tool, or in a dedicated tokens package. Each choice has tradeoffs in ownership, review, and tooling support.

### Decide The Design-To-Code Boundary Deliberately

Pipelines range from fully manual handoff to automated code generation. Each point on this spectrum has different failure modes.

- **Manual handoff**: maximum designer control, maximum drift risk, depends on documentation discipline.
- **Linked component libraries**: design tool components mirror code components, reducing drift but requiring maintenance of both.
- **Token sync only**: automates values but not structure; flexible but leaves component behavior to engineers.
- **Code generation**: highest automation, but generated code rarely matches production quality, accessibility, or framework conventions without heavy customization.

Automated generation sounds efficient but often produces code that engineers must rewrite, which destroys trust in the pipeline. Match the automation level to the team's capacity to maintain it.

### Build CI And Automated Checks For Design Constraints

A design system that relies on designers remembering every rule will drift. Pipelines should encode enforceable constraints as automated checks wherever possible.

Checks worth automating:

- token usage: components reference tokens rather than raw values;
- contrast: text and meaningful UI meet required ratios;
- spacing and grid: values come from the defined scale;
- icon and asset hygiene: naming, formats, and sizes conform;
- component usage: deprecated components are flagged;
- naming conventions for layers, variants, and styles.

Automated checks shift quality left, catching issues before review. But checks must be maintainable: a brittle lint rule that flags false positives will be disabled, and once disabled it will not come back.

### Plan For Asset Export And Format Requirements

Asset pipelines are easy to underestimate. Icons, illustrations, logos, and raster images each have format, naming, density, and optimization requirements that differ by platform.

A sound asset pipeline defines:

- which formats each asset type uses (SVG for vector, PNG or WebP or AVIF for raster, with platform-specific needs);
- naming conventions that survive export and match code expectations;
- density or resolution variants for each target platform;
- optimization steps that reduce size without visible quality loss;
- a single source file that regenerates all exports, so manual edits do not diverge.

Manual export from a design tool, renamed by hand, is a pipeline that will break the moment someone is absent.

### Version Control Design Artifacts Meaningfully

Design files need version control, but the model differs from code. Binary files, branching, and merge conflicts behave differently, and many design tools have their own history model.

Decide:

- what is committed and how often;
- how branching maps to feature work;
- how conflicts are resolved when two designers edit the same file;
- how released or approved states are marked;
- how the design tool's internal history relates to the external version control.

A version control strategy that ignores how designers actually work will be bypassed, leaving the team with no reliable history.

### Keep Pipelines Maintainable By The People Who Use Them

A pipeline that only one person understands is a liability. Tooling choices should account for who will maintain them after the original author leaves.

Favor pipelines that:

- use widely understood formats and standards, such as JSON or YAML for tokens;
- have documentation and examples, not just configuration;
- can be debugged by designers and engineers, not only by a specialist;
- fail loudly and explain the failure, rather than silently producing wrong output.

## Common Traps

### Over-Automating A Pipeline The Team Cannot Maintain

Sophisticated code generation or sync tooling that no one fully owns becomes a black box. When it breaks, work stops, and the team reverts to manual processes.

### Treating Tokens As Optional Decoration

If tokens exist in the design tool but are not consumed by code, or exist in code but are ignored in design, they provide no alignment benefit and add maintenance cost.

### Duplicating The Source Of Truth

When token values live in the design tool, in a tokens file, and in code, three sources drift apart silently. There must be one authoritative source.

### Choosing A Tool For Its Plugins Then Losing The Plugins

Plugin ecosystems change. Building a pipeline that depends on a single third-party plugin with no maintainer creates fragile infrastructure.

### Ignoring Platform Differences In Output

A pipeline that assumes one output format will fail for multi-platform products. iOS, Android, and web have different token formats, asset densities, and component expectations.

### Letting Automated Checks Rot

Lint rules and CI checks that produce false positives get disabled and never restored. Checks need ownership and periodic review.

## Self-Check

- [ ] Tool choices are justified against required capabilities, including variants, tokens, platforms, accessibility, and concurrency.
- [ ] Tokens have a single source of truth and flow to both design and code through a defined transform.
- [ ] The design-to-code automation level matches the team's capacity to maintain it.
- [ ] Enforceable design constraints are encoded as automated checks that fail loudly and are owned.
- [ ] The asset export pipeline defines formats, naming, densities, optimization, and a regenerable source.
- [ ] Design artifacts have a version control strategy that matches how designers actually work.
- [ ] The pipeline is documented and maintainable by more than one person.
- [ ] Platform-specific output differences are handled, not assumed away.
