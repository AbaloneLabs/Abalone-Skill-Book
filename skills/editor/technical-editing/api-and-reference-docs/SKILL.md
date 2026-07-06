---
name: api-and-reference-docs.md
description: Use when the agent is editing API documentation, SDK guides, reference material, developer portals, endpoint references, schema definitions, or any technical reference content where readers look up specific facts, signatures, or behaviors and where incompleteness or inaccuracy blocks integration.
---

# API And Reference Docs

Reference documentation is read differently from prose. Readers of reference docs are not reading sequentially; they are looking up a specific fact, signature, behavior, or example to solve an immediate problem. They arrive with a question, scan for the answer, and leave. This reading pattern demands a different editorial standard. Reference docs must be complete, accurate, scannable, consistent, and example-rich, because a missing parameter, a wrong type, or an absent example forces the reader to read source code or abandon the integration. The editor of reference docs is verifying that the documentation describes the system as it actually behaves, not as the author assumed it behaves.

## Core Rules

### Verify Every Signature, Type, And Parameter Against The Source

Reference docs describe contracts. Each endpoint, function, method, class, or schema documented must match the actual implementation. The editor verifies names, parameter lists, types, return types, optionality, default values, and enumerated values against the source code, schema, or interface definition. A parameter documented as optional that is actually required, a type documented as string that is actually an enum, a default value that has changed, all break integrations. Verification is not optional; it is the core of reference editing. Where source access is limited, flag unverified claims and request confirmation.

### Document Every Parameter, Field, And Return Value

Completeness is non-negotiable in reference docs. Every parameter must be documented: its name, type, whether it is required or optional, its default, allowed values, and its meaning. Every field in a request or response body must be documented. Every possible return value, status code, or error must be listed. Omitting a field because it seems obvious or internal leaves readers guessing. The editor's job is to find gaps and demand they be filled. A reference doc with missing fields is worse than one that admits incompleteness, because it implies completeness it does not have.

### Provide Concrete, Copy-Pasteable Examples

Reference readers want examples they can run or adapt. Each endpoint, function, or component should have at least one complete, working example showing a typical use. Examples must be correct, current, and tested against the documented behavior. A request example must show the actual request format, including headers, body, and parameters. A response example must show an actual response. Examples that are simplified to the point of being non-functional, or that use placeholder values that do not work, mislead. Provide examples in the languages or formats the audience uses, and keep them synchronized with the reference text.

### Structure For Lookup, Not Sequential Reading

Reference docs are scanned, not read. Structure them so readers find answers fast. Use consistent section ordering across all endpoints or components: summary, parameters, request, response, errors, examples. Use headings, tables, and code blocks to make facts visible at a glance. Put the most-needed information, the summary and the example, near the top. Use tables for parameters and fields, because tables let readers compare and find. Avoid burying critical facts in prose paragraphs that require reading to extract.

### Document Errors, Edge Cases, And Failure Modes

Happy-path documentation is incomplete. Reference docs must document every status code, every error response, every exception, and the conditions that trigger them. Readers integrating against an API need to know what can go wrong, not only what goes right. Document rate limits, pagination behavior, versioning, deprecation, and breaking changes. Document edge cases: empty results, maximum values, concurrent access, idempotency. The editor asks, for each endpoint or function, what happens when inputs are invalid, when limits are hit, when dependencies fail, and ensures the docs answer.

### Maintain Versioning And Changelog Discipline

Reference docs describe a versioned system. Each doc must state which version it describes, and the docs must be updated when versions change. Breaking changes must be called out prominently, with migration guidance. Maintain a changelog that records what changed, when, and why, so readers upgrading can assess impact. When multiple versions are supported, keep version-specific docs separate and clearly labeled, so readers do not mix guidance from different versions. Stale version references are a common and serious reference doc failure.

### Ensure Consistency Across The Reference Set

Reference docs are read as a set. Readers move between endpoints, schemas, and guides, and inconsistency between them causes confusion. Terminology, naming conventions, formatting, example style, and section structure must be consistent across the entire reference. Authentication, base URLs, common headers, and shared parameters should be documented once and referenced, not re-explained inconsistently in each endpoint. The editor reads across the set, not only within individual pages, to catch cross-document inconsistency.

### Match The Documentation To The Actual Behavior, Not The Intended Behavior

Authors sometimes document what the system should do rather than what it does. The editor must distinguish intended from actual behavior. Where they diverge, the documentation must describe actual behavior, and the divergence should be reported as a bug or a known issue. Documenting intended behavior that differs from actual behavior guarantees that every reader who follows the docs will fail. When actual behavior is surprising or undesirable, note it honestly rather than glossing it.

## Common Traps

### Documenting From Memory Or Assumption

Authors who wrote the system may document from memory, omitting details they consider obvious or recalling behavior inaccurately. The editor must treat all reference content as unverified until checked against source. Memory is unreliable for parameter order, types, defaults, and edge cases. Verify, verify, verify.

### Incomplete Parameter And Field Documentation

A common failure is documenting the main parameters but omitting optional ones, nested fields, or metadata fields. Readers hit the undocumented field in a response and do not know what it means. The editor cross-checks the documented fields against the actual schema or response, demanding documentation for every field, including those that seem internal or self-explanatory.

### Examples That Do Not Work

Examples that contain placeholder values, wrong endpoints, outdated authentication, or simplified payloads that do not match real behavior waste reader time. Every example must be runnable or, at minimum, structurally accurate to a real request or response. Test examples against the live system or a representative test environment. An example that fails undercuts the credibility of the entire reference.

### Stale Documentation After System Changes

APIs and systems change, and docs that are not updated become wrong. Parameter names change, endpoints are deprecated, behaviors shift. The editor must check the last-updated date, confirm the docs reflect the current system, and flag content likely to be stale. Couple documentation updates to release processes so that changes ship with doc updates. Stale docs erode trust faster than missing docs.

### Burying Critical Information In Prose

Reference readers scan. Critical facts like required authentication, rate limits, breaking changes, or deprecation buried in paragraphs are missed. Surface critical information in callouts, prominent labels, or consistent locations. A breaking change hidden in a sentence on line 40 will be overlooked by readers scanning for the endpoint signature.

### Inconsistent Error Documentation

Errors documented for some endpoints but not others, error codes listed in one place and omitted in another, and inconsistent formatting of error information all reduce usefulness. Standardize error documentation across the reference set. Every endpoint documents its errors in the same place, in the same format, with the same level of detail.

### Ignoring Pagination, Filtering, And Common Behaviors

Many endpoints share behaviors: pagination, filtering, sorting, rate limiting, authentication. Documenting these once, centrally, and referencing them is more maintainable than re-documenting per endpoint, but the editor must ensure each endpoint that exhibits these behaviors links to or notes the shared behavior. A reader who does not realize an endpoint paginates will think it returns incomplete data.

### Assuming The Reader Knows The Conventions

Reference docs often assume shared conventions: how authentication works, what the base URL is, how errors are shaped. New readers do not know these conventions. Document the conventions once, prominently, early in the reference, and ensure each endpoint at least references them. Do not make readers reverse-engineer conventions from examples.

## Self-Check

- Has every signature, parameter, type, default, and return value been verified against the actual source code, schema, or interface definition?
- Are all parameters, fields, request bodies, response bodies, status codes, and errors documented, with none omitted as obvious or internal?
- Does each endpoint or component include at least one complete, correct, tested example showing realistic request and response?
- Is the documentation structured for scanning, with consistent section ordering, tables for parameters and fields, and code blocks for examples?
- Are errors, edge cases, rate limits, pagination, versioning, deprecations, and breaking changes documented for each relevant endpoint?
- Is the version described by the documentation clearly stated, current, and accompanied by a maintained changelog?
- Is terminology, formatting, naming, example style, and section structure consistent across the entire reference set?
- Does the documentation describe actual system behavior, with any divergence from intended behavior noted honestly and reported?
- Are shared conventions like authentication, base URLs, and error formats documented once and referenced consistently?; are examples free of placeholder values that would prevent them from working, and have they been tested against the live or test system?
- Is critical information like breaking changes, required authentication, and deprecation surfaced prominently rather than buried in prose?; has stale content been identified, with last-updated dates noted and content flagged for re-verification after system changes?
