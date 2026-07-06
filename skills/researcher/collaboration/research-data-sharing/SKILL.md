---
name: research_data_sharing.md
description: Use when the agent is sharing research data with collaborators or the public, writing a data sharing plan, choosing a repository, setting access controls, or balancing openness with privacy and contractual limits.
---

# Research Data Sharing

Sharing data lets others verify, reuse, and build on a study, and it is increasingly required by funders and journals. But sharing is not simply uploading a file. Research data often contain identifying information about participants, carry contractual or legal constraints, require documentation to be usable, and embed choices about who gets access and when. When data sharing is handled carelessly, three harms follow. Participants are re-identified or exposed despite assurances of confidentiality. Collaborators or lower-resourced partners are excluded from the data they helped produce. And the shared dataset is so poorly documented that no one can actually use it, defeating the purpose.

The agent should use this skill when writing a data management or sharing plan, depositing data in a repository, responding to a data request, setting terms for collaborator access, or deciding what can and cannot be shared. The goal is to keep the agent from treating sharing as a box to check, when it is a decision about privacy, equity, and the long-term usability of the data.

## Core Rules

### Decide What Can Be Shared And What Cannot

Not all data can or should be shared openly. Classify data before planning sharing.

- Open data, can be shared publicly with no risk, such as aggregate statistics or fully synthetic data.
- Controlled-access data, can be shared under conditions, such as genomic or clinical data with re-identification risk.
- Restricted data, cannot be shared outside the original consent and agreement, such as data with contractual limits or sensitive qualitative transcripts.
- Embargoed data, can be shared after a defined period or publication.

For each dataset, determine the highest level of sharing that is ethical and legal, and document why. Defaulting to no sharing when controlled sharing is possible withholds public benefit. Defaulting to open sharing when re-identification is possible harms participants.

### Protect Participant Privacy Before Sharing

Re-identification risk is often underestimated. Seemingly anonymous data can be re-identified by combining fields, and qualitative transcripts can identify participants through distinctive details.

Before sharing, apply the following.

- Remove or encode direct identifiers, names, addresses, contact details.
- Assess indirect identifiers, rare combinations of demographics, occupations, locations, dates.
- Consider aggregation or suppression for small cells.
- Apply formal privacy methods where appropriate, such as differential privacy for aggregate releases.
- Review qualitative data for identifying detail and redact or pseudonymize.
- Obtain participant consent for the level of sharing planned, or confirm the original consent permits it.

If the original consent did not anticipate sharing, open sharing may be impossible even if technically de-identified. Consent is a constraint on sharing, not just a privacy technicality.

### Choose The Right Repository And License

A repository gives data a persistent home and identifier. Choose one that fits the data type and discipline.

Consider the following.

- Domain relevance, a repository used and trusted by the field.
- Access control capabilities, if controlled access is needed.
- Persistent identifiers, such as DOIs for citation.
- Long-term preservation and sustainability.
- Metadata standards supported.
- Cost and institutional support.

Pair the repository choice with an appropriate license. Open data should carry a clear license such as CC0 or CC-BY so others know what reuse is allowed. Vague or missing licenses deter reuse because users cannot be sure of their rights. For sensitive data, use a data use agreement rather than an open license.

### Document Data So Others Can Actually Use It

Unusable shared data fails the purpose of sharing. Documentation should let a competent stranger understand and reuse the data.

Provide the following.

- A data dictionary defining each variable, unit, and code.
- Codebook for categorical values and missing data codes.
- Collection protocol and any deviations.
- Processing and transformation steps from raw to shared form.
- Software and versions needed to read or analyze the data.
- Known limitations, errors, or anomalies.
- Link to related code, protocols, and publications.

Documentation is part of the dataset, not an optional extra. A dataset released without a data dictionary is closer to a puzzle than a resource.

### Set Access Terms For Controlled-Access Data

For data that cannot be open, define a transparent access process.

- What applicants must provide, such as research question and ethics approval.
- Who reviews requests and by what criteria.
- What users may and may not do with the data.
- Reporting and publication expectations.
- Duration and revocation conditions.
- Security requirements for storing and handling the data.

A clear, fair access process protects participants and enables legitimate reuse. An opaque or arbitrary process frustrates researchers and can hide inequitable gatekeeping.

### Plan Timing Relative To Publication And Partners

Decide when data are shared relative to publication and to the interests of partners who produced them.

Common approaches include the following.

- Sharing at publication, the most common default.
- Sharing after an embargo that lets the producing team publish first.
- Sharing on registration for prospective studies, before results exist.
- Immediate open sharing for community or public-good projects.

Funders increasingly require sharing at or near publication. Plan for this from the start so it is not a scramble at submission. Respect embargo terms agreed with partners, especially lower-resourced or community partners who need time to publish from their own data.

### Ensure Equitable Sharing Across Partners

Data sharing can reproduce inequity when better-resourced teams extract data from partners and publish first. Plan sharing to be fair.

- Include all partners in decisions about what is shared and when.
- Give producing partners time and support to analyze their own data.
- Credit data contributors in citations and authorship.
- Build local capacity for analysis where partners lack it.
- Avoid terms that let only well-resourced teams exploit the data.

Equitable sharing is not only an ethics question; it determines whether partners will collaborate again.

### Track And Report Data Sharing

Keep records of what was shared, where, under what terms, and when. Report sharing in publications through a data availability statement that is accurate and specific. Vague statements that promise availability on request without a mechanism are a transparency failure.

Track reuse where possible, because data citations and downstream uses demonstrate impact and justify the effort of sharing.

## Common Traps

### Assuming De-Identification Removes All Risk

Indirect identifiers and record linkage can re-identify data thought to be anonymous. Assess re-identification risk realistically.

### Sharing Beyond The Original Consent

If participants did not consent to the planned sharing, even de-identified sharing may breach the agreement. Check consent before sharing.

### Depositing Data Without Documentation

A dataset without a data dictionary or protocol is nearly useless to others and wastes the sharing effort.

### Choosing An Open License By Default For Sensitive Data

Open licenses assume open sharing is appropriate. For sensitive data, use controlled access and data use agreements instead.

### Promising Availability On Request With No Mechanism

Without a process, request-based sharing often fails. Use a repository or documented process.

### Letting Better-Resourced Teams Publish First From Partner Data

Failing to give producing partners time and credit reproduces inequity and damages future collaboration.

### Ignoring Funder And Journal Sharing Requirements

Late discovery of mandatory sharing policies causes rushed, poor-quality deposits. Plan sharing from the proposal stage.

### Treating Qualitative Data As Easy To Share

Qualitative transcripts carry high re-identification risk and may require redaction, pseudonymization, or restricted access.

## Self-Check

- Has each dataset been classified as open, controlled-access, restricted, or embargoed, with reasons documented?
- Has re-identification risk been assessed and mitigated through de-identification, aggregation, or formal privacy methods?
- Does the planned sharing comply with the original participant consent and any contractual limits?
- Has an appropriate repository and license or data use agreement been chosen for the data type?
- Does the shared dataset include a data dictionary, codebook, protocol, processing steps, and known limitations?
- For controlled-access data, is there a transparent, fair access process with clear criteria and security requirements?
- Is the timing of sharing planned relative to publication and partner interests, respecting embargoes?
- Are all partners included in sharing decisions, with credit and capacity considerations addressed?
- Is data sharing accurately reported in publications with specific availability statements?
- For data involving sensitive populations, genomic data, or cross-border transfer, has a data governance advisor, privacy officer, or legal counsel reviewed the sharing plan?
