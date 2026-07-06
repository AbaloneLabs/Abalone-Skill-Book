---
name: example_validity.md
description: Use when the agent is editing API or reference documentation and must verify that code examples are valid, runnable, and accurate, checking that examples match the documented behavior, use current syntax and library versions, and actually produce the results claimed.
---

# Example Validity

Code examples in documentation are what developers copy, adapt, and trust. An example that does not run, uses deprecated syntax, or produces different results than claimed causes frustration and erodes credibility in the entire documentation. Developers who encounter one broken example begin to distrust all examples, testing each rather than relying on them. Example validity is the discipline of ensuring code examples are correct, runnable, and accurate. Editors who treat examples as illustrations rather than tested code miss the errors that cause real failures when developers use them.

Use this skill when editing API or reference documentation that contains code examples. It covers verifying examples run, match documented behavior, and use current syntax. The goal is examples developers can copy, run, and trust.

## Core Rules

### Verify Examples Actually Run

The foundational requirement is that code examples run without error. An example that fails when copied and executed is worse than no example, because it wastes developer time and damages trust. Verify execution.

Where possible, run each code example to verify it executes without errors. This catches syntax errors, missing imports, incorrect method names, and other issues that prevent execution. If you cannot run the example yourself, flag it for testing by someone who can. Examples that have never been run are a common source of documentation errors, because writers often write examples from memory or inference rather than testing them. The standard is that a developer who copies the example into the appropriate environment, with the documented prerequisites, can run it successfully.

### Check Examples Match Documented Behavior

An example should demonstrate the behavior the surrounding documentation describes. If the docs say an endpoint returns a list and the example shows a single object, or the docs describe pagination and the example ignores it, the example contradicts the documentation. Verify alignment.

For each example, verify it demonstrates what the surrounding text describes. Check that the example's request matches the documented endpoint, parameters, and method. Check that the example's response matches the documented response structure. Check that any claims about what the example shows are accurate. An example that contradicts its documentation confuses developers about which to believe. Alignment between example and documentation reinforces both; contradiction undermines both.

### Verify Syntax And Library Versions Are Current

Code examples use specific languages, libraries, and versions. Examples with outdated syntax or deprecated libraries fail or mislead developers using current versions. Verify currency.

Check that each example uses current, non-deprecated syntax for its language and libraries. Verify that library or SDK versions referenced are current and compatible. Flag examples using deprecated methods, outdated patterns, or old versions that may not work with current environments. Documentation with outdated examples forces developers to translate to current syntax, which introduces errors and defeats the example's purpose. Where the API or library has version-specific behavior, ensure the example matches the version the documentation targets. Currency matters because developers use examples in current environments.

### Ensure Examples Include All Necessary Context

An example should include everything needed to run, or clearly reference where to find it. Examples that omit imports, setup, or prerequisites fail when copied because the context is missing. Ensure completeness.

Review each example for completeness. Does it include necessary imports, configuration, or setup? If the example assumes prior context, such as an authenticated client or a configured environment, is that context stated or provided? Examples that show only the core call without the surrounding setup may illustrate the concept but fail when copied because the setup is missing. Where including full setup makes the example too long, provide a link to a complete runnable example and clearly state what the snippet assumes. Completeness ensures examples are not just illustrative but usable.

### Test Example Inputs And Outputs

For examples that show specific inputs and outputs, verify the inputs produce the claimed outputs. An example that shows a request and a response should have a response that the request would actually produce. Verify input-output alignment.

For examples with shown inputs and outputs, verify the input produces the shown output. If the example sends a specific request and displays a response, confirm that response is what the API returns for that request. Check that values in the response, such as IDs or timestamps, are consistent with the request and documented behavior. Inconsistent input-output pairs mislead developers about what to expect. Where the response varies, such as containing generated IDs, note that the shown values are illustrative. Alignment between shown inputs and outputs makes examples reliable models of API behavior.

### Use Realistic, Meaningful Examples

Examples should use realistic inputs and scenarios that developers will actually encounter. Trivial examples with placeholder values like "foo" and "bar" or unrealistic scenarios fail to teach how to use the API in real situations. Use realistic examples.

Review examples for realism. Do they use plausible values, such as real-looking names, addresses, or IDs, rather than meaningless placeholders? Do they demonstrate scenarios developers will face, such as creating a resource, handling an error, or paginating results, rather than trivial calls? Realistic examples teach better because developers can map them to their own use cases. Placeholder-heavy examples force developers to infer what real values look like, which introduces errors. Where placeholders are necessary, use realistic ones and label them as examples.

### Cover Common And Important Cases

Examples should cover the cases developers most need: the basic happy path, common variations, and important edge cases or errors. Examples that cover only the simplest case leave developers unprepared for real usage. Cover meaningfully.

Assess example coverage. Is the basic use case demonstrated? Are common variations, such as optional parameters or filtering, shown? Are important edge cases, such as error handling or rate limiting, covered? Are authentication and pagination, common pain points, illustrated? Identify gaps in coverage and recommend examples to fill them. Comprehensive example coverage helps developers integrate successfully without resorting to trial and error or support requests. Coverage should be guided by what developers actually need, not by what is easy to write.

## Common Traps

### Examples That Do Not Run

Verify execution. Untested examples fail when developers use them.

### Examples Contradicting The Documentation

Verify examples demonstrate the documented behavior and response.

### Outdated Syntax Or Deprecated Libraries

Use current syntax and versions. Outdated examples fail in current environments.

### Missing Imports Or Setup Context

Include necessary context or link to complete runnable examples.

### Input-Output Pairs That Do Not Match

Verify shown inputs produce shown outputs. Note illustrative values.

### Placeholder-Heavy Trivial Examples

Use realistic values and scenarios developers will actually encounter.

### Covering Only The Simplest Case

Cover common variations, errors, and important edge cases, not just the happy path.

## Self-Check

Before treating code examples as valid and reliable, verify:

- Each example has been run or flagged for testing and executes without errors.
- Examples demonstrate the behavior and responses the surrounding documentation describes.
- Syntax and library versions are current and non-deprecated.
- Examples include necessary imports, setup, and context, or link to complete versions.
- Shown inputs produce the shown outputs, with illustrative values labeled.
- Examples use realistic values and scenarios, not meaningless placeholders.
- Coverage includes the basic case, common variations, errors, and important edge cases.
- Authentication, pagination, and other common pain points are illustrated.
- Examples are in the languages and styles the target developers use.
- A developer copying an example could run it and get the claimed result without modification.
