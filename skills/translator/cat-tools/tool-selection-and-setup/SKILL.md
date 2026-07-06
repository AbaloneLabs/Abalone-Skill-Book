---
name: tool_selection_and_setup.md
description: Use when the agent is selecting a CAT tool or TMS for a translation project or program, configuring project settings, evaluating tool capabilities against requirements, or deciding between desktop enterprise and cloud-based translation environments.
---

# Tool Selection And Setup

The choice of CAT tool or translation management system shapes every aspect of a translation program: what file formats are supported, how teams collaborate, how quality is tracked, what leverage is possible, and how much engineering overhead each project carries. Yet tool selection is often made casually, driven by familiarity, price, or a single feature, without mapping the tool's capabilities to the program's real requirements. The result is friction: formats that require manual conversion, collaboration that breaks across locations, quality workflows the tool cannot express, and assets locked in proprietary structures. Tool setup is equally consequential: a powerful tool misconfigured produces poor segmentation, weak matching, and lost metadata. Selecting and setting up a tool is an engineering decision that should be driven by requirements, tested against real content, and revisited as the program evolves. The right tool, well configured, disappears into the workflow; the wrong tool, or the right tool poorly configured, becomes a constant obstacle.

Use this skill when selecting a CAT tool or TMS, configuring project settings, or evaluating tool capabilities against requirements. The goal is a tool and configuration that fit the program's needs and that support, rather than hinder, quality and efficiency.

## Core Rules

### Define Requirements Before Evaluating Tools

Tool selection must be driven by requirements, not by features. Define requirements first.

Gather the program's needs: source file formats, target languages and scripts, team size and locations, collaboration model, quality workflow steps, integration with content or code repositories, reporting needs, security and data residency constraints, and budget. Prioritize the requirements into must-have, important, and nice-to-have. A tool that excels on nice-to-have features but misses a must-have, such as a critical file format or a security requirement, is the wrong tool. Requirements prevent selection from being swayed by demos and feature lists.

Document the requirements and use them as the evaluation scorecard.

### Match Tool Capabilities To File Format Needs

File format support is a frequent point of failure. Match the tool to the formats the program actually uses.

Verify that the tool handles the program's file formats natively or with reliable filters: software resource files such as XML, JSON, properties, and RESX; documentation formats such as Markdown, DITA, and structured XML; office formats; and design or e-learning formats. Test with real files, not just samples, because filters vary in how they handle inline tags, entities, and complex structures. A tool that requires manual pre- and post-processing for every file adds engineering overhead and error risk to every project.

If the program uses niche or proprietary formats, confirm support before committing.

### Evaluate Collaboration And Workflow Support

Team collaboration needs differ, and tools support it differently. Evaluate collaboration against the team model.

For distributed teams, cloud-based tools enable real-time collaboration and shared assets; desktop tools require file exchange and merge workflows. For large projects, consider simultaneous multi-translator access, assignment and workload management, and review workflows built into the tool. Evaluate whether the tool supports the program's quality workflow, such as bilingual review, revision, and sign-off, or whether those must be managed outside. A tool that cannot express the workflow forces workarounds that lose tracking and metadata.

Match the collaboration model to how the team actually works, not to an idealized process.

### Consider Asset Portability And Lock-In

Translation assets, TM, termbases, and project files, must be portable. Consider lock-in.

Proprietary asset formats tie the program to one tool; if the tool changes pricing, discontinues features, or fails, migrating assets is costly. Prefer tools that support industry-standard exchange formats such as TMX for memory, TBX for terminology, and XLIFF for bilingual files. Evaluate export completeness: some tools export assets with loss of metadata or custom fields. Asset portability is an insurance policy; it costs little to maintain and saves enormous pain if a migration is needed.

A program locked into a proprietary format has lost control of its own assets.

### Configure Segmentation Matching And Quality Settings

Tool setup determines whether the tool performs well. Configure the key settings deliberately.

Configure segmentation rules for the content types, as covered in segmentation guidance. Set fuzzy match thresholds appropriate to the content. Configure quality assurance checks such as terminology consistency, number and tag verification, and length limits. Set up TM and termbase connections with correct precedence. Define project templates that capture the configuration so it is applied consistently. A tool with default settings often produces poor segmentation, weak matching, and no automated quality checks, undermining the investment in the tool.

Setup is not a one-time task; refine configurations as content and requirements evolve.

### Evaluate Security And Data Handling

Translation content can be confidential, and tools handle data differently. Evaluate security against requirements.

Cloud-based tools store data on vendor servers; evaluate the vendor's security certifications, data residency options, and access controls. Desktop and on-premise tools keep data within the organization's control but require local security management. For sensitive content such as legal, medical, or proprietary material, confirm the tool meets the program's security and compliance requirements, including encryption, access logging, and data retention. A breach of translation data can be as harmful as a breach of the source.

Do not assume security; verify it against the content's sensitivity.

### Test With Real Projects Before Committing

Demos and trials with sample content do not reveal real-world performance. Test with real projects.

Run a pilot project through the candidate tool using actual files, actual team members, and actual workflow steps. Evaluate segmentation quality, match rates, collaboration smoothness, filter reliability, and reporting usefulness. Gather feedback from translators and project managers. A pilot reveals problems that a feature checklist cannot, such as sluggish performance on large files or confusing review interfaces. Use the pilot results to finalize the decision and configuration.

A tool that looks good in a demo can fail on the real work; testing prevents costly mistakes.

### Plan For Training And Adoption

A tool delivers value only if the team uses it well. Plan for training and adoption.

Budget time and resources for training translators, reviewers, and project managers on the tool. Provide documentation and support for the configured workflows. Monitor adoption and address resistance or confusion. A tool imposed without training produces inconsistent use, workarounds, and lost asset quality. Adoption is ongoing: new team members need onboarding, and configuration changes need communication.

## Common Traps

### Selecting On Features Rather Than Requirements

A tool rich in features can miss a must-have requirement; drive selection by requirements.

### Assuming File Format Support Without Testing

Filters vary; untested formats lead to manual conversion overhead and error risk on every project.

### Ignoring Collaboration Model Fit

A tool that cannot express the team's workflow forces workarounds that lose tracking and metadata.

### Proprietary Asset Lock-In

Without standard exchange format support, the program loses control of its TM and termbases.

### Accepting Default Settings

Default segmentation, matching, and QA settings often produce poor results; configure deliberately.

### Overlooking Security For Cloud Tools

Cloud storage introduces data handling risks; verify security against content sensitivity.

### Committing Without A Pilot

Demos do not reveal real performance; test with real projects before adopting.

### No Training Or Adoption Plan

A tool without training is used inconsistently, producing workarounds and lost asset quality.

## Self-Check

Before finalizing a tool selection or setup, verify:

- Requirements are defined and prioritized into must-have, important, and nice-to-have, and drive the evaluation.
- The tool supports the program's actual file formats, verified with real files, not just samples.
- Collaboration and workflow support match the team's model, including review and sign-off steps.
- Assets are portable via industry-standard exchange formats such as TMX, TBX, and XLIFF, minimizing lock-in.
- Segmentation, matching, and quality assurance settings are configured deliberately, not left at defaults.
- Security and data handling meet the content's confidentiality and compliance requirements.
- A pilot project tested the tool with real files, team, and workflow before commitment.
- Training and adoption are planned, with documentation and ongoing support for the configured workflows.
- No must-have requirement is unmet in favor of nice-to-have features.
- The tool and configuration fit the program's needs and support, rather than hinder, quality and efficiency.
