---
name: open_source_license_compliance.md
description: Use when the agent is auditing a codebase for open source license obligations, analyzing copyleft viral effects of GPL AGPL LGPL and MPL licenses, evaluating permissive MIT BSD Apache licenses, tracking transitive dependencies, determining disclosure source publication obligations, checking license compatibilityibility for combined works, or building an open source compliance program for software distribution.
---

# Open Source License Compliance

Open source compliance is the discipline of satisfying the obligations that open source licenses impose when you distribute or convey software that includes open source components. The judgment problem is that open source licenses are real, enforceable copyright licenses, not casual permissions, and their obligations attach automatically when triggered. The most consequential obligations, particularly copyleft disclosure and source-publication requirements, can force a company to publish its own proprietary source code, and they are triggered by distribution behavior that developers rarely recognize as legally significant.

The second judgment problem is transitive depth. A modern application depends on hundreds or thousands of packages, each with its own license, and obligations propagate through the dependency tree. An agent who inspects only direct dependencies misses the license buried three levels deep that imposes copyleft on the whole product. The skill is maintaining an accurate, continuously updated inventory of all open source components and their licenses, understanding the obligation profile of each license family, and engineering the build and distribution process to satisfy obligations without surrendering proprietary rights. Agents must resist the twin errors of treating all open source as interchangeable and risk-free, and of treating any copyleft as an absolute prohibition; both reflect a failure to read the actual license terms.

This skill addresses open source compliance primarily under US copyright interpretation of major licenses (GPL family, AGPL, LGPL, MPL, Apache, MIT, BSD) with reference to EU and other jurisdictions. License interpretation involves judgment and some unsettled questions; specific distribution models and combined-work questions should be reviewed by a licensed attorney familiar with open source law.

## Core Rules

### Maintain a complete and current software bill of materials

Compliance begins with knowing what is in the product. Maintain a software bill of materials (SBOM) that lists every open source component, its version, its license, and its origin, including transitive dependencies. Use automated scanning tools that identify licenses and detect undeclared components, and update the SBOM at each release. A compliance program built on a partial inventory is a program that will fail at the component it never saw. Treat the SBOM as a living artifact, not a one-time exercise.

### Classify each license by obligation profile

Licenses fall into broad families with distinct obligation profiles. Permissive licenses (MIT, BSD, Apache 2.0) typically require attribution, retention of license notices, and sometimes a statement of changes, but do not require source publication of your own code. Weak copyleft licenses (LGPL, MPL) require source for modifications to the licensed component itself and impose conditions on combining, but generally do not propagate to separate works that merely link or use the component. Strong copyleft licenses (GPL) require that derivative works be licensed under compatible terms and source be provided. Network copyleft (AGPL) extends the trigger to providing the software as a service over a network, not just distribution. Know the profile of every license in the stack before distribution.

### Understand what triggers obligations, distribution and conveyance

Most obligations trigger on distribution or conveyance of the software. Internal use without distribution generally triggers few obligations, with the critical exception of AGPL, which triggers on network interaction. Clarify your distribution model: are you distributing binaries, providing a hosted service, offering an API, or using internally only? The trigger analysis determines which obligations apply. SaaS use of GPL code without distribution generally does not trigger GPL source obligations, but AGPL does, and the line between a modified service and a derivative work is contested.

### Analyze copyleft propagation carefully

The hardest question is whether copyleft propagates to your own code. For GPL, the key distinction is whether your code and the GPL code form a single derivative work or separate and independent works. Static linking generally creates a single work triggering copyleft; dynamic linking is contested; communication via stable APIs or separate processes generally does not. Plugins, templates, and configuration that interoperate tightly may or may not propagate. Err on the side of caution where the analysis is uncertain, and document the architectural separation that supports non-propagation. Do not assume linking is safe because the GPL code is a small fraction of the product.

### Evaluate license compatibility before combining

Not all open source licenses can be combined. Licenses are compatible if the obligations of one can be satisfied simultaneously with the other. GPL-licensed code can generally be combined with permissively licensed code, but combining GPL with code under an incompatible copyleft license (e.g., certain Apache 2.0 with GPL version 2 combinations) can be impermissible. When combining, identify the most restrictive license in the combined work and confirm the others are compatible. Incompatible combinations cannot be distributed at all, regardless of attribution or source publication.

### Satisfy attribution and notice obligations precisely

Even permissive licenses impose obligations. MIT and BSD require retaining the copyright notice and license text. Apache 2.0 requires retaining notices, stating changes, and including a copy of the license, plus handling of the NOTICE file. These obligations are cheap to satisfy but are routinely violated by stripping notices during minification or build. Implement automated notice generation in the build process so attribution is correct by construction. Failure to attribute is a copyright infringement even though the license is permissive.

### Engineer the architecture to manage copyleft exposure

Where copyleft exposure is unacceptable, engineer around it. Replace a GPL dependency with a permissively licensed alternative. Isolate copyleft components behind a process boundary or network API so they remain separate works. Use LGPL components in ways that respect the linking distinction. Document the architectural decisions and the license rationale so future developers do not inadvertently collapse the separation. Architecture is a compliance tool, not just an engineering choice.

### Establish a review and approval process

No compliance program works without a gate. Require open source review before any new dependency is introduced and before any release. Define an approval policy: which licenses are pre-approved, which require legal review, and which are prohibited. Track exceptions and their rationale. Make the process lightweight enough that developers comply rather than bypass, using automated tooling integrated into the build and pull-request workflow.

## Common Traps

### Inspecting only direct dependencies

The copyleft license that forces source publication is often a transitive dependency three levels deep. Scan the full dependency tree, not just direct imports.

### Assuming copyleft is an absolute prohibition

Copyleft licenses are usable in commercial products with proper architecture and compliance. Treating any GPL component as forbidden forfeits valuable technology and reflects a failure to analyze the actual integration.

### Treating AGPL like GPL

AGPL's network-interaction trigger means SaaS and cloud use can trigger source obligations that GPL would not. Applying GPL analysis to AGPL code is a critical error for any service provider.

### Stripping notices during build

Minification, bundling, and build processes routinely strip copyright and license notices, creating infringement out of permissively licensed code. Automate notice preservation.

### Assuming dynamic linking is always safe

The treatment of dynamic linking under GPL is contested, and some maintainers and commentators take a broad view. Relying on dynamic linking to avoid copyleft without architectural separation is risky.

### Ignoring license compatibility

Combining GPL with an incompatible license produces a work that cannot be distributed at all. Compatibility must be checked before combining, not discovered at release.

### No continuous SBOM maintenance

A one-time scan becomes stale immediately as dependencies update. Compliance built on a snapshot inventory misses new components introduced after the scan.

### Conflating internal use with distribution

Internal use of GPL code generally does not trigger source obligations, but AGPL does, and providing the software to customers as a service may trigger obligations depending on the license and architecture. Analyze the actual trigger.

## Self-Check

- Is there a current, complete software bill of materials covering direct and transitive dependencies, maintained at each release?
- Has each license in the stack been classified by obligation profile (permissive, weak copyleft, strong copyleft, network copyleft)?
- Has the distribution or conveyance model been analyzed to determine which obligations actually trigger, including AGPL network-interaction triggers?
- For each copyleft component, has propagation to proprietary code been analyzed, with architectural separation documented where non-propagation is claimed?
- Has license compatibility been confirmed for all combined works, identifying the most restrictive license and verifying others are compatible?
- Are attribution and notice obligations satisfied automatically in the build process, preserving notices through minification and bundling?
- Where copyleft exposure is unacceptable, has the architecture been engineered to isolate or replace the component, with decisions documented?
- Is there a review and approval gate before new dependencies and releases, with a defined policy on pre-approved, review-required, and prohibited licenses?
- Have I confirmed that automated scanning is integrated into the build and pull-request workflow so compliance is continuous, not snapshot?
- Have I flagged that license interpretation involves unsettled questions and jurisdictional variation, and that a licensed attorney familiar with open source law should review material compliance questions?
