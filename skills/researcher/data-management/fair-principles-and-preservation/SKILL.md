---
name: fair_principles_and_preservation.md
description: Use when the agent is applying FAIR principles to research data, choosing identifiers and repositories, planning long-term preservation, or deciding how to make data findable, accessible, interoperable, and reusable over time.
---

# FAIR Principles And Preservation

The FAIR principles state that data should be findable, accessible, interoperable, and reusable, by both people and machines. FAIR is not the same as open; data can be FAIR and controlled-access. The principles exist because most research data are effectively lost, locked on laptops, in undocumented formats, or behind requests that no one honors. When FAIR is ignored, three harms follow. Data cannot be found or cited, so they contribute nothing after the paper. Data decay as formats and software change. And the scientific record becomes a set of claims without the underlying evidence to check or extend them. Preservation is what keeps FAIR data alive over time.

The agent should use this skill when preparing data for deposit, choosing a repository, assigning identifiers, writing a data availability statement, or planning what will happen to data after a project ends. The goal is to keep the agent from confusing FAIR with merely uploading a file, when FAIR is a set of design choices about identifiers, metadata, formats, licenses, and stewardship that determine whether data remain usable.

## Core Rules

### Make Data Findable Through Persistent Identifiers And Rich Metadata

Findable means a user, human or machine, can discover the data. This requires more than a filename.

- Assign a persistent identifier, such as a DOI, that resolves permanently.
- Include rich metadata describing the data in a standard schema.
- Index the data in a discoverable repository or catalog.
- Use a clear, citable title and description.
- Link the data to related publications, code, and protocols.

A dataset on a personal website with no identifier is not findable in any durable sense. The identifier is what lets the data be cited and located years later, even if the host changes.

### Make Data Accessible With A Clear, Honored Protocol

Accessible means the data can be retrieved, possibly under conditions. Accessibility does not require openness, but it does require a real, documented path.

- State exactly how to access the data.
- Use a repository or documented process, not a vague promise of availability on request.
- For open data, provide a direct download.
- For controlled data, describe the application process, criteria, and timeframe.
- For restricted data, state the constraint clearly and what alternatives exist.

A promise of availability on reasonable request without a mechanism is not accessibility in practice. Most such requests go unanswered, and the data are effectively lost.

### Make Data Interoperable Through Open Formats And Standards

Interoperable means the data can be combined with other data and tools. This depends on formats and vocabularies.

- Use open, documented file formats for preservation.
- Use community vocabularies and ontologies where they exist.
- Use consistent units, codes, and identifiers.
- Provide machine-readable metadata.
- Avoid formats that require proprietary software to read.

A dataset in a proprietary format with private codes is not interoperable. Even if findable and accessible, it resists integration and reuse.

### Make Data Reusable Through Documentation, Provenance, And License

Reusable means a user can understand and legitimately use the data. This is where most deposits fail.

- Provide a data dictionary, codebook, and protocol.
- Record provenance from raw to analyzed data.
- State a clear license for open data, such as CC0 or CC-BY.
- For controlled data, provide a data use agreement specifying allowed uses.
- Document known limitations and quality issues.
- Include attribution information so users can cite the data.

A dataset with no license deters reuse because users cannot be sure of their rights. A dataset with no documentation cannot be understood. Reusability is the hardest FAIR dimension and the most valuable.

### Separate FAIR From Open

FAIR data can be open or controlled. The principles are about being usable under defined conditions, not about being public. Sensitive data can be FAIR through controlled access with rich metadata, clear application processes, and documented terms.

Avoid the error of treating FAIR as a mandate to publish everything openly. Equally, avoid treating sensitivity as a reason to make data neither open nor FAIR. Controlled-access FAIR data serve both participants and science.

### Choose A Trustworthy Repository

The repository determines whether FAIR data stay FAIR. Choose one with the following properties.

- Issues persistent identifiers.
- Supports the needed access model, open or controlled.
- Uses recognized metadata standards.
- Commits to long-term preservation and fixity checks.
- Is sustainable and institutionally backed.
- Is recognized in the discipline.

Avoid personal websites, ephemeral cloud shares, or lab pages that vanish when the lab moves. A dataset is only as durable as its host.

### Plan Preservation Beyond The Project

Preservation is what keeps data FAIR over years. Plan it explicitly.

- Decide which data will be preserved long-term.
- Choose preservation formats that remain readable.
- Define the retention period required by funder or regulation.
- Assign a steward responsible after the project ends.
- Arrange for fixity checks and migration as formats age.
- Ensure the identifier and metadata remain resolvable.

Data without a preservation plan decay. The grant ends, the laptop is wiped, and the data, however FAIR at deposit, become unreachable.

### Apply FAIR To Software And Protocols Too

FAIR principles extend beyond datasets to the software, workflows, and protocols that produced the results. A dataset is only fully reusable when the code and methods that generated it are also findable, accessible, interoperable, and reusable.

- Deposit code in a persistent repository with a DOI.
- Document dependencies and versions.
- License code clearly, such as MIT or Apache.
- Archive protocols in a resource such as a protocols repository.
- Link data, code, and protocols bidirectionally.

A dataset without its code is a result without its method. FAIR applies to the whole evidence chain.

### Measure And Improve FAIRness

FAIRness can be assessed. Use available tools and rubrics to evaluate a dataset against the principles, and improve weak dimensions before deposit.

Common weak points include the following.

- Missing or vague license.
- Incomplete metadata.
- Proprietary formats.
- No persistent identifier.
- No provenance or data dictionary.

Treat FAIR assessment as a pre-deposit checklist, not a post-hoc audit. Improving FAIRness before deposit is far easier than after.

## Common Traps

### Confusing FAIR With Open

FAIR data can be controlled-access. Treating FAIR as open-only either over-shares sensitive data or under-shares shareable data.

### Promising Availability On Request

Without a repository or process, request-based sharing usually fails. Use a real mechanism.

### Depositing In A Proprietary Format

Proprietary formats undermine interoperability and decay over time. Use open formats for preservation.

### Omitting A License

A dataset with no license is legally ambiguous and deters reuse. Choose a clear license.

### Choosing An Unstable Host

Personal sites and lab pages vanish. Use a trustworthy, persistent repository.

### Skipping Provenance And Documentation

Findable and accessible data without documentation are not reusable. FAIR fails at the most valuable dimension.

### Forgetting Long-Term Stewardship

Data need a steward and preservation plan after the grant ends. Without one, FAIR data become unreachable.

### Applying FAIR Only To Datasets

Code and protocols are part of the evidence chain. Make them FAIR too.

## Self-Check

- Does the dataset have a persistent identifier and rich, standard metadata so it is findable?
- Is there a clear, honored access protocol, with a repository or documented process rather than a vague promise?
- Are open formats and community vocabularies used so the data are interoperable?
- Are documentation, provenance, license, and limitations provided so the data are reusable?
- Is FAIR distinguished from open, with controlled access used where sensitivity requires it?
- Is the repository trustworthy, persistent, and recognized in the discipline?
- Is long-term preservation planned, with formats, retention, stewardship, and fixity checks?
- Are software and protocols also made FAIR and linked to the data?
- Has FAIRness been assessed before deposit and weak dimensions improved?
- For sensitive, large-scale, or legally constrained data, has a data librarian, repository curator, or preservation specialist reviewed the deposit plan?
