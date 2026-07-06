---
name: endpoint_and_parameter_accuracy.md
description: Use when the agent is editing API or reference documentation and must verify that endpoints, parameters, methods, and data types are described accurately, checking that the documentation matches the actual API behavior, that required and optional parameters are distinguished, and that errors in endpoint or parameter description are caught before publication.
---

# Endpoint And Parameter Accuracy

API documentation is a contract. Developers build against it, integrating systems and writing code that depends on the documented behavior being correct. An endpoint described with the wrong method, a parameter listed as optional when it is required, or a data type misstated causes integration failures that cost hours of debugging and erode trust in the documentation. Endpoint and parameter accuracy is the discipline of verifying that API documentation describes the actual API. Editors who edit API docs for prose quality without verifying against the implementation miss the technical errors that cause real failures.

Use this skill when editing API or reference documentation to verify endpoint and parameter accuracy. It covers verifying against the implementation, distinguishing required from optional, and checking data types and constraints. The goal is documentation developers can rely on as an accurate description of the API.

## Core Rules

### Verify Documentation Against The Implementation

The foundational rule is that documentation must match the implementation. Every endpoint, parameter, method, and response described in the docs should correspond to the actual API behavior. Verify against source.

Where possible, check the documentation against the API implementation or a specification, such as an OpenAPI or Swagger file. Verify each documented endpoint exists, with the correct path and HTTP method. Verify parameters match what the implementation accepts. Verify response structures match what the implementation returns. Discrepancies between docs and implementation are the most damaging API documentation errors, because they cause integration failures. If you cannot access the implementation, flag that verification is needed and recommend it be done by someone who can. Never assume the documentation is accurate because it was written by an engineer; engineers make documentation errors too.

### Distinguish Required From Optional Parameters

Parameters must be correctly marked as required or optional. A developer who omits a parameter marked optional that is actually required gets errors; one who includes a parameter marked required that is actually optional wastes effort. Verify the distinction.

For each parameter, verify whether it is required or optional, and mark it accordingly in the documentation. Required parameters are those the API demands; optional parameters have defaults or are genuinely optional. Check that the marking matches the implementation. Where a parameter is conditionally required, required only in some cases, document the conditions clearly. The required or optional distinction guides developers in constructing valid requests; getting it wrong causes predictable failures. Use consistent formatting to mark required parameters, such as a visual indicator or explicit label.

### Verify Data Types And Formats

Each parameter and response field has a data type and often a format. Wrong types cause integration errors: a developer sending a string where an integer is expected, or formatting a date incorrectly. Verify types and formats.

For each parameter and response field, verify the documented data type matches the implementation. Check whether it is a string, integer, boolean, array, object, or a specific format like date-time, UUID, or email. Verify format constraints, such as maximum string length, numeric range, or allowed values. Document the type and format clearly, using consistent conventions. Where a parameter accepts specific values, such as an enumeration, list the allowed values. Type and format accuracy prevents the errors that arise when developers send data the API cannot process.

### Document Default Values And Behavior

Optional parameters often have default values, and the API has default behaviors when parameters are omitted. These defaults should be documented, because they affect what the API does without explicit instruction. Document defaults.

For each optional parameter, verify and document the default value that applies when the parameter is omitted. For behaviors that have defaults, such as pagination defaults or sorting defaults, document what those defaults are. Defaults matter because they determine the API's behavior in common cases where developers rely on the default rather than specifying explicitly. Undocumented defaults lead to confusion when the API behaves differently than expected. Documented defaults let developers understand and predict behavior without testing each omission.

### Check Parameter Placement And Encoding

Parameters appear in different locations: path, query string, headers, or request body. The placement affects how developers construct requests. Wrong placement causes requests to fail. Verify placement and encoding.

For each parameter, verify the documented placement matches the implementation. Is it a path parameter, query parameter, header, or body field? Document the placement clearly. Verify encoding requirements, such as whether values must be URL-encoded, how arrays are passed, and how complex objects are serialized. Parameter placement and encoding errors cause requests that look right but fail because the API cannot find or parse the parameters. Clear documentation of placement and encoding helps developers construct valid requests the first time.

### Verify Response Structures And Status Codes

API documentation must accurately describe responses, including status codes and response structures. A developer integrating needs to know what to expect, including success and error responses. Verify responses.

For each endpoint, verify the documented success status code and response structure match the implementation. Check that response fields, their types, and their meanings are correct. Verify error responses are documented, including status codes and error formats. Common errors, such as validation failures or authentication errors, should be documented with their response structures. Incomplete or wrong response documentation causes integration failures when developers encounter responses they did not expect. Document both the happy path and the error paths.

### Flag Anything That Cannot Be Verified

You may not always be able to verify every detail against the implementation. In those cases, flag what could not be verified rather than asserting accuracy. Flag uncertainty.

Where you cannot verify a detail, such as an endpoint behavior or a parameter requirement, note that it needs verification by someone with implementation access. Do not let the documentation imply certainty you could not confirm. Flagging uncertainty ensures the gap is known and can be addressed, rather than hiding a potential error. It also maintains the documentation's credibility, because flagged uncertainty is honest; unverified assertion that proves wrong destroys trust.

## Common Traps

### Editing Prose Without Verifying Technical Accuracy

API docs are a contract. Verify against the implementation.

### Wrong Required Or Optional Marking

Verify and clearly mark which parameters are required versus optional.

### Incorrect Or Undocumented Data Types

Verify types and formats. Type errors cause integration failures.

### Undocumented Defaults

Document default values and behaviors. They determine common-case behavior.

### Wrong Parameter Placement Or Encoding

Verify whether parameters are path, query, header, or body. Document encoding.

### Incomplete Or Wrong Response Documentation

Document success and error responses with correct structures and status codes.

### Asserting Accuracy Without Verification

Flag what cannot be verified. Do not imply certainty you cannot confirm.

## Self-Check

Before treating endpoint and parameter documentation as accurate, verify:

- Each documented endpoint has been checked against the implementation for path and method.
- Parameters are correctly marked as required or optional, matching the implementation.
- Data types and formats for parameters and response fields are verified and documented.
- Format constraints, such as length, range, and allowed values, are documented.
- Default values for optional parameters and default behaviors are documented.
- Parameter placement, path, query, header, or body, is correct and clearly documented.
- Encoding requirements, including URL encoding and serialization, are specified.
- Success status codes and response structures are verified and documented.
- Error responses, including common errors, are documented with their structures.
- Anything that could not be verified is flagged for confirmation by someone with implementation access; a developer building against this documentation would encounter no surprises from doc-implementation mismatches
