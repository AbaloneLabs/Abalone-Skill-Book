---
name: version_and_changelog_consistency.md
description: Use when the agent is editing API or reference documentation and must verify that version numbers, deprecation notices, and changelog entries are consistent across all documentation, checking that the current version is stated correctly, deprecated features are marked, and changelog entries accurately describe what changed in each version.
---

# Version And Changelog Consistency

API and reference documentation evolves across versions. Each version may add features, change behavior, deprecate functionality, or remove elements. Documentation must reflect the current version accurately, mark deprecated features, and maintain a changelog that records what changed. Inconsistencies between the version stated in the docs, the features described, the deprecation notices, and the changelog confuse developers about what is current, what is safe to use, and what has changed. Version and changelog consistency is the discipline of keeping these elements aligned. Editors who update one part of the documentation without checking the others create the inconsistencies that mislead developers.

Use this skill when editing API or reference documentation to verify version and changelog consistency. It covers version number accuracy, deprecation handling, and changelog completeness. The goal is documentation where a developer can determine the current version, what changed, and what is deprecated without encountering contradictions.

## Core Rules

### Verify The Current Version Is Stated Correctly

Documentation should state the current API or library version clearly and accurately. A wrong version number, or different version numbers in different places, creates confusion about what the documentation describes. Verify the version.

Check that the documentation states the current version and that the stated version is correct. Verify the version appears consistently wherever it is mentioned, such as in headers, introductions, and metadata. If the documentation covers multiple versions, verify the version selection mechanism is clear and that the displayed content matches the selected version. A developer who reads the version and builds against it relies on that version being accurate. Inconsistent or wrong version statements undermine trust and can cause developers to build against features or behaviors that do not match their version.

### Mark Deprecated Features Clearly

When features are deprecated, the documentation must mark them clearly so developers know to avoid them or migrate. Undocumented deprecation leads developers to build on features that will be removed. Mark deprecations.

For each deprecated feature, verify the documentation includes a clear deprecation notice. The notice should state that the feature is deprecated, when it was deprecated, and what the replacement or migration path is, if any. Use consistent visual or textual markers for deprecation so developers can spot deprecated items at a glance. Verify deprecated features are not presented as recommended or current without the deprecation context. Clear deprecation marking helps developers make informed decisions about what to use and plan for migration before removal.

### Verify Changelog Entries Accurately Describe Changes

The changelog records what changed in each version. Each entry should accurately describe the changes: additions, modifications, deprecations, removals, and fixes. Inaccurate or incomplete changelog entries mislead developers about version differences. Verify accuracy.

For each changelog entry, verify it accurately describes the changes in that version. Check that additions are listed, modifications are described with what changed, deprecations are noted, removals are identified, and fixes are summarized. Verify the entry matches the actual changes by checking against the version's release notes, commit history, or implementation. Flag entries that are vague, incomplete, or inaccurate. A changelog is a historical record developers use to understand version differences and plan upgrades; its accuracy is essential.

### Ensure Cross-References Between Docs And Changelog Are Consistent

Documentation and changelog should cross-reference consistently. If the docs mention a feature was added in version 2.0, the changelog for 2.0 should list that addition. Inconsistencies between docs and changelog create confusion. Verify cross-references.

Check that version-specific information in the documentation matches the changelog. If a feature is noted as new in a version, the changelog should record it. If a feature is marked deprecated as of a version, the changelog should note the deprecation. If a behavior changed in a version, the changelog should describe the change. Cross-reference consistency ensures a developer can trace any version-specific claim in the docs to the changelog and find agreement. Inconsistency suggests one or the other is wrong.

### Document Breaking Changes Prominently

Breaking changes, which can break existing integrations, must be documented prominently. Developers upgrading need to know what will break and how to adapt. Document breaking changes.

For each breaking change, verify it is documented prominently, not buried. Breaking changes should be called out in the changangelog with clear marking, such as a "Breaking Change" label. They should be summarized in migration guides or upgrade notes for the relevant version. The documentation for affected features should note the breaking change and the required adaptation. Prominent breaking change documentation helps developers upgrade safely, without discovering breaks through failure. Buried or undocumented breaking changes cause production failures and erode trust.

### Maintain A Migration Path For Deprecated And Removed Features

When features are deprecated or removed, developers need a migration path: what to use instead and how to transition. Documentation should provide this path, not just announce the deprecation or removal. Provide migration guidance.

For each deprecated or removed feature, verify the documentation provides a migration path. This includes what the replacement is, if any, how to transition, and any deadlines or timelines for removal. Migration guidance should be specific enough that a developer can adapt their integration, not just a statement that the feature is deprecated. Where migration is complex, provide a migration guide. Providing a path respects the investment developers have made in the deprecated feature and helps them transition without disruption.

### Keep Versioned Documentation Aligned

If the documentation maintains separate versions, each version's docs must be internally consistent and aligned with that version's actual behavior. Versioned docs that drift from their version's behavior mislead developers targeting that version. Maintain alignment.

For each maintained documentation version, verify the content matches that version's actual API behavior. Check that features described exist in that version, that deprecated features are marked as of the right version, and that version-specific behaviors are correctly described. Versioned documentation requires maintenance: as new versions are released, older versions' docs must be preserved accurately or archived. Drift between versioned docs and actual behavior causes failures for developers targeting specific versions. Maintain alignment or clearly mark older versions as unsupported.

## Common Traps

### Wrong Or Inconsistent Version Numbers

State the current version correctly and consistently throughout.

### Undocumented Or Unmarked Deprecations

Mark deprecated features clearly with replacement and timeline.

### Inaccurate Or Incomplete Changelog Entries

Verify changelog entries match actual changes. Accuracy is essential.

### Docs And Changelog Contradicting Each Other

Cross-reference version-specific claims between docs and changelog for consistency.

### Buried Breaking Changes

Document breaking changes prominently with migration guidance.

### Deprecation Without Migration Path

Provide what to use instead and how to transition, not just the deprecation notice.

### Versioned Docs Drifting From Actual Behavior

Maintain each version's docs to match that version's API behavior.

## Self-Check

Before treating version and changelog consistency as complete, verify:

- The current version is stated correctly and consistently wherever it appears.
- Version selection, if multiple versions are documented, is clear and displays matching content.
- Deprecated features are marked clearly with deprecation version and replacement.
- Deprecated features are not presented as recommended without deprecation context.
- Each changelog entry accurately describes the additions, changes, deprecations, removals, and fixes in that version.
- Version-specific claims in the documentation match the corresponding changelog entries.
- Breaking changes are documented prominently with clear marking and migration guidance.
- Migration paths are provided for deprecated and removed features.
- Each maintained documentation version aligns with that version's actual API behavior.
- A developer could determine the current version, what changed, and what is deprecated without encountering contradictions.
