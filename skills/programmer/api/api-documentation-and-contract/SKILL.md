---
name: api_documentation_and_contract.md
description: Use when the agent is writing or reviewing API documentation, defining an API contract (OpenAPI, gRPC schema, GraphQL schema, JSON Schema), generating client SDKs, designing developer-facing reference docs and guides, managing documentation drift from the implementation, or establishing the documentation workflow. Covers the API contract as the source of truth, contract-first vs code-first development, reference documentation vs developer guides, examples and edge cases, SDK generation, documentation testing and drift detection, versioning documentation, and the principle that an undocumented or misdocumented API behavior is a liability.
---

# API Documentation And Contract

An API's documentation is its contract with its consumers, and a contract that drifts from the implementation is worse than no contract, because it actively misleads. A consumer who reads that an endpoint returns a field, builds against that expectation, and discovers in production that the field is absent or differently typed, has been betrayed by the documentation; the API cannot be trusted, and the consumer's integration breaks. The discipline of API documentation is treating the documentation as a contract — a precise, versioned, machine-readable description of the API's behavior (endpoints, request/response schemas, error formats, authentication) that is the source of truth for both the implementation and the consumers — and keeping it synchronized with the implementation as both evolve. A machine-readable contract (OpenAPI, gRPC schema, GraphQL schema) enables tooling: client SDK generation, automated testing, mock servers, and drift detection that flags when the implementation diverges from the contract. Documentation that is "just prose" lacks these guarantees and drifts inevitably.

Agents tend to write documentation as an afterthought (prose added after the implementation, not updated as the API changes), to omit edge cases and error behavior, and to treat the contract and the documentation as separate concerns. The judgment problem is recognizing that the API contract is the source of truth that both implementation and consumers build against, that machine-readable contracts enable tooling that keeps documentation accurate, and that documentation quality is measured by whether a consumer can integrate correctly using it alone. This skill covers the discipline of API documentation and contracts: the contract as source of truth, contract-first development, reference docs vs guides, examples and edge cases, SDK generation, drift detection, and the documentation workflow.

## Core Rules

### Treat The API Contract As The Source Of Truth

The contract — a precise description of the API's behavior — is what the implementation must conform to and what consumers build against. It is the source of truth, not a derivative of the implementation.

- **Define the API as a machine-readable contract (OpenAPI, gRPC schema, GraphQL schema, JSON Schema).** A machine-readable contract precisely specifies endpoints, methods, request/response schemas, parameters, error formats, authentication, and version. It is the single source of truth that implementation, consumers, and tooling reference.
- **Contract-first development: define the contract, then implement to it.** Defining the contract before the implementation forces clear design (the API is specified, not emergent), enables parallel development (consumers build against the contract/mock while the implementation is built), and ensures the implementation conforms to the contract (not the reverse).
- **Code-first with contract generation is acceptable if the generation is the source of truth and is verified.** Generating the contract from code annotations (Swagger annotations, gRPC from proto) is viable, but the generated contract must be treated as authoritative, reviewed, and version-controlled — not a disposable artifact. Drift between code and contract must be detected.
- **Do not let the implementation silently diverge from the contract.** A code change that alters the API (a new field, a changed type) without updating the contract creates drift; consumers building against the contract are misled. Detect drift (contract-vs-implementation testing) and fail the build on it.

### Document Reference Behavior Precisely, Including Errors And Edge Cases

Reference documentation describes the API's behavior exhaustively: every endpoint, parameter, response, status code, and error. Imprecision here causes integration failures.

- **Document every endpoint's request and response schemas, including all fields, types, and whether each is required or optional.** A consumer needs to know exactly what to send and what to expect. Undocumented fields (present in the response but not in the docs) are a liability; consumers may depend on them, then break when they change.
- **Document all possible status codes and error response formats.** An endpoint that can return 200, 201, 400, 401, 403, 404, 409, 422, 429, 500 must document each, with the error response format and the conditions that trigger it. Consumers must handle errors correctly; undocumented errors cause unhandled cases.
- **Document edge cases and constraints: pagination, filtering, sorting, rate limits, idempotency, maximum sizes.** The happy path is obvious; the edge cases (what happens with an empty result, a too-large request, a conflicting state) are where consumers struggle. Document them explicitly.
- **Provide accurate, runnable examples for each endpoint.** Examples showing a real request and response (including headers, auth, and a representative body) let consumers verify their understanding. Examples that are out of date or simplified to the point of uselessness undermine the docs.

### Separate Reference Documentation From Developer Guides

Reference documentation (the contract, exhaustively precise) and developer guides (how to accomplish tasks with the API) serve different purposes and have different structures.

- **Reference docs: exhaustive, precise, generated from the contract.** Every endpoint, field, status code, generated from the machine-readable contract so it cannot drift. The "dictionary" of the API.
- **Developer guides: task-oriented, narrative, explaining how to use the API to accomplish goals.** "How to authenticate," "How to handle pagination," "How to create and update a resource" — guides that walk a consumer through real tasks, referencing the reference docs for details. The "tutorial" of the API.
- **Provide both; neither substitutes for the other.** Reference without guides is impenetrable (the consumer cannot see how to use the API); guides without reference are incomplete (the consumer lacks the precise behavior). Most consumers need both.
- **Include getting-started, authentication, and common workflows prominently.** A new consumer's first tasks (authenticate, make a first call, handle a common workflow) should be easy to find and follow. Friction here loses consumers before they integrate.

### Generate Client SDKs From The Contract

A machine-readable contract enables generating client SDKs in multiple languages, which dramatically reduces consumer integration effort and ensures consistency.

- **Generate SDKs from the contract (OpenAPI to TypeScript/Python/Go/etc., gRPC to language stubs).** Generated SDKs handle request construction, authentication, error parsing, and type safety, so consumers integrate in minutes rather than days. They also ensure the consumer's types match the contract (reducing type-related bugs).
- **Version the SDKs alongside the contract.** SDKs generated from contract version N target API version N; consumers pin to a SDK version matching the API version they use. SDK and contract versioning must be aligned.
- **Keep generated SDKs up to date as the contract evolves.** A stale SDK (generated from an old contract) does not reflect the current API. Regenerate and publish SDKs on contract changes; communicate updates to consumers.
- **Do not hand-write SDKs where generation suffices.** Hand-written SDKs drift from the contract, are inconsistent across languages, and are expensive to maintain. Generate; customize only where generation is insufficient.

### Detect And Prevent Documentation Drift

Documentation that does not match the implementation is a liability. Detect drift and prevent it mechanically.

- **Test the implementation against the contract.** Contract testing (generating tests from the contract, or validating implementation responses against the contract schema) detects drift: if the implementation returns a response that does not match the contract, the test fails. Run contract tests in CI.
- **Validate examples in the documentation against the contract.** Examples that show a response not matching the contract schema are drift; validate them. Outdated examples mislead consumers.
- **Fail the build on drift.** Drift detected in CI should fail the build, forcing the contract or the implementation to be reconciled before merge. Drift that is warned but not blocked accumulates.
- **Review contract changes with the same rigor as code changes.** A contract change affects consumers; review it (does it break compatibility? is it documented? are consumers notified?) as a first-class change, not a side effect of a code change.

### Version The Documentation With The API

API documentation evolves with the API, and consumers on older API versions need the documentation for their version.

- **Version the documentation alongside the API.** API v1's documentation describes v1's behavior; v2's describes v2's. Consumers on v1 must access v1's documentation, not be forced to read v2's (which may describe behavior v1 does not have).
- **Maintain documentation for supported versions.** As long as an API version is supported (not deprecated/retired), its documentation must be maintained and accurate. Retiring the documentation while the version is still supported strands consumers.
- **Provide a changelog between versions.** Consumers upgrading from v1 to v2 need to know what changed (new endpoints, changed behavior, breaking changes). A changelog documents the delta; reference docs document the absolute state.
- **Communicate documentation updates to consumers.** Documentation that corrects an error or clarifies behavior should be communicated (changelog, notification) so consumers who built against the old understanding can adjust.

## Common Traps

### Documentation As Prose Afterthought

Documentation written as prose after the implementation, not updated as the API changes, drifting from reality. Use a machine-readable contract as the source of truth; generate reference docs.

### Implementation Diverging From The Contract

A code change altering the API without updating the contract, creating drift that misleads consumers. Detect drift in CI; fail the build on it.

### Undocumented Errors And Edge Cases

Documentation covering only the happy path, leaving consumers to discover errors and edge cases in production. Document all status codes, error formats, and edge cases.

### Outdated Or Simplified Examples

Examples that do not match the current API or are too simplified to be useful, misleading consumers. Validate examples against the contract; keep them current and realistic.

### Reference Without Guides (Or Vice Versa)

Only exhaustive reference docs (impenetrable) or only narrative guides (incomplete), when consumers need both. Provide reference and guides.

### Hand-Written SDKs Drifting From The Contract

SDKs written by hand that diverge from the contract as the API evolves, inconsistent across languages. Generate SDKs from the contract.

### Drift Detected But Not Blocked

Contract-vs-implementation drift that is warned in CI but does not fail the build, accumulating until documentation is untrustworthy. Fail the build on drift.

### No Versioned Documentation

Documentation for only the latest API version, stranding consumers on older supported versions. Version the documentation; maintain it for supported versions.

## Self-Check

- [ ] The API is defined as a machine-readable contract (OpenAPI, gRPC schema, GraphQL schema, JSON Schema) that is the source of truth — implementation conforms to it, consumers build against it — and the workflow is contract-first or code-first-with-generation-verified.
- [ ] Reference documentation is exhaustive and precise: every endpoint, field, type, required/optional status, status code, error format, edge case (pagination, filtering, rate limits, idempotency, max sizes), and constraint is documented, generated from the contract so it cannot drift.
- [ ] Accurate, runnable examples (real requests and responses with headers, auth, and representative bodies) are provided for each endpoint and validated against the contract schema so they do not mislead.
- [ ] Both reference documentation (exhaustive, generated) and developer guides (task-oriented, narrative) are provided — neither substitutes for the other — with getting-started, authentication, and common workflows prominently featured.
- [ ] Client SDKs are generated from the contract in multiple languages, versioned alongside the contract, kept up to date as the contract evolves, and used in preference to hand-written SDKs that drift.
- [ ] Documentation drift is detected (contract-vs-implementation testing, example validation in CI) and blocked (drift fails the build), and contract changes are reviewed with the same rigor as code changes (compatibility, documentation, consumer notification).
- [ ] Documentation is versioned with the API, maintained for all supported versions, a changelog documents the delta between versions, and documentation updates are communicated to consumers.
- [ ] The documentation is tested from the consumer's perspective: a developer using only the documentation (and generated SDK) can authenticate, make their first call, handle errors, and accomplish a common workflow without needing to ask the API team — the measure of documentation quality.
