---
name: api-and-data-acquisition.md
description: Use when the agent is acquiring research data through APIs or web scraping, choosing between official APIs and scraping, handling rate limits and pagination, managing authentication and credentials, capturing provenance and timestamps, or designing a robust and reproducible data acquisition pipeline that documents exactly what was collected and when.
---

# API And Data Acquisition

Acquiring data through APIs or web scraping is now a common research method, and its apparent ease hides serious pitfalls. An API returns structured data, but that data is shaped by the platform's design decisions: what it exposes, what it hides, and how it ranks and filters results. Scraping extracts data from pages, but pages change without notice, breaking pipelines and silently altering what is collected. Agents often write a script, run it once, treat the output as a dataset, and never document the conditions under which it was gathered. The judgment problem is to choose between APIs and scraping deliberately, to respect rate limits and access rules, to capture the provenance and timing of every record, and to build a pipeline that is reproducible and resilient. A dataset whose collection conditions are unknown, or that silently changed mid-collection, cannot support reliable inference about anything except itself.

Use this skill when acquiring data through APIs or scraping. The goal is to prevent the agent from treating acquisition as a one-off technical task, from ignoring the platform's shaping of the data, and from producing datasets whose provenance and timing are undocumented. The agent has freedom in tooling, but the acquisition must be deliberate, respectful, and fully documented.

## Core Rules

### Choose Between APIs And Scraping Deliberately

Official APIs and scraping differ in reliability, legality, and the data they yield. The choice must be justified.

Choose by:

- preferring official APIs where they provide the needed data, as they are more stable and sanctioned;
- scraping only where no adequate API exists or where the API omits needed fields;
- the terms of service and access rules each route entails;
- the reliability and maintenance burden of each approach.

Scraping what an API provides adds legal risk and fragility for no gain. Conversely, relying on a limited API that omits key data produces an incomplete dataset. The choice should follow the data needs and the rules.

### Understand How The Platform Shapes The Data

API and search results are not neutral samples. Platforms rank, filter, and personalize what they return.

Understand by:

- whether results are ranked, filtered, or personalized, and how;
- whether pagination returns the full set or a capped subset;
- whether the API exposes deleted or historical content;
- how the platform's design decisions become sampling decisions in the dataset.

An API that returns "top" results by an opaque algorithm yields a ranked sample, not a census. Treating it as complete misrepresents the dataset and the findings drawn from it.

### Respect Rate Limits And Be A Good Citizen

Rate limits and access rules exist for technical and ethical reasons. Violating them risks bans and harm.

Respect by:

- reading and following documented rate limits and access policies;
- implementing throttling, backoff, and caching to minimize requests;
- identifying the client honestly where identification is expected;
- avoiding aggressive concurrency that stresses the host.

Hammering a server risks being blocked and degrades service for others. Responsible acquisition is both ethical and more sustainable for long-running collection.

### Handle Authentication And Credentials Securely

API access often requires keys or tokens that must be protected, never embedded in shared code.

Handle by:

- storing credentials in environment variables or secret managers, not in code;
- never committing credentials to version control;
- rotating or revoking keys when exposed;
- documenting the authentication method without exposing the secrets.

A leaked API key can be abused, can violate terms, and can contaminate the dataset. Credential security is part of responsible acquisition.

### Capture Provenance, Timestamps, And Raw Responses

Every record must carry evidence of when and how it was acquired. Without provenance, the dataset cannot be audited.

Capture by:

- a collection timestamp for every record or batch;
- the API endpoint, parameters, and version used;
- the raw response where feasible, before transformation;
- a manifest describing the collection run and its conditions.

Platforms change, content disappears, and algorithms shift. Provenance and raw capture let the dataset be reinterpreted and trusted even as the source evolves.

### Design For Reproducibility And Resilience

Acquisition pipelines break when sites or APIs change. They must be designed to fail gracefully and to be re-run.

Design by:

- checkpointing and resuming so partial failures do not lose progress;
- logging errors, retries, and anomalies for review;
- versioning the pipeline and the source schema it expects;
- documenting how to re-run the collection end to end.

A pipeline that must be re-run from scratch after any error, or that cannot be re-run at all, is not reproducible. Resilience protects the investment and the credibility of the data.

### Document The Sampling Frame And Its Limits

Acquired data is a sample shaped by access, timing, and platform rules. These limits must be stated.

Document by:

- what population of content or users the dataset represents;
- what is excluded by API limits, pagination caps, deletion, or access rules;
- the time window of collection and how the source may have changed;
- the implications for generalization from the dataset.

A dataset of public posts collected over a week is not a dataset of all posts by all users. The sampling frame and its gaps determine what the data can say.

### Plan For Data Volatility And Long-Term Access

Web data is volatile: pages change, APIs deprecate, platforms shut down. Long-term access requires planning.

Plan for:

- archiving raw responses and snapshots, not just processed data;
- recording the schema and version at collection time;
- anticipating deprecation and migration paths;
- a data management plan for preservation and sharing within legal limits.

A dataset that exists only as live API queries cannot be re-examined when the API changes. Preservation of raw captures protects the research over time.

## Common Traps

### Acquisition As One-Off Task

Running a script once and treating the output as a dataset, without documenting conditions, undermines reproducibility.

### Ignoring Platform Shaping

Treating ranked or filtered API results as a neutral census misrepresents the dataset.

### Rate Limit Violations

Ignoring access rules risks bans, legal issues, and harm to the host service.

### Embedded Credentials

Hardcoding API keys in shared code exposes them to abuse and contamination.

### Missing Provenance

Records without timestamps, endpoints, or raw captures cannot be audited or reinterpreted.

### Fragile Non-Resumable Pipeline

A pipeline that loses progress on error or cannot be re-run is not reproducible.

### Unstated Sampling Limits

Generalizing from a capped or time-bound dataset as if it were complete overstates the findings.

## Self-Check

- Is the choice between API and scraping justified by data needs and access rules?
- Is the platform's ranking, filtering, and pagination analyzed as a sampling decision?
- Are rate limits and access policies respected through throttling, backoff, and caching?
- Are credentials stored securely, never embedded in code or version control?
- Does every record carry provenance: timestamp, endpoint, parameters, and raw response where feasible?
- Is the pipeline designed to checkpoint, resume, log errors, and be re-run end to end?
- Is the sampling frame and its exclusions, caps, and time window documented?
- Are raw captures archived for long-term access despite source volatility?
- Is the acquisition method fully documented for reproducibility?
- Are the dataset's limits on generalization stated alongside the data?
