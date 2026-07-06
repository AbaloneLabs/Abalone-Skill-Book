---
name: deposit_workflows_and_policies.md
description: Use when the agent is designing or revising repository deposit workflows, writing deposit policies and submission agreements, setting up mediated or self-deposit processes, defining embargo and access policies, handling theses and research data deposit, or establishing content selection and appraisal criteria for an institutional repository.
---

# Deposit Workflows And Policies

A repository's value is realized only when content actually enters it under controlled, consistent, and policy-grounded conditions. Deposit is where good repository intentions meet messy reality: researchers who do not want to deposit, theses with unclear rights, datasets too large or too messy to ingest, embargoes that conflict with funder mandates, and metadata that varies wildly between depositors. The workflow and policy that govern deposit determine whether the repository becomes a trusted, well-described, rights-clear collection or an inconsistent pile of whatever was easiest to accept. Agents tend to focus on the technical act of upload and under-design the policy, rights review, metadata quality control, and the human workflow that makes deposit reliable. A deposit policy that accepts everything on the depositor's terms creates a repository no one can trust for completeness, description, or access.

Use this skill when designing deposit workflows, writing deposit and access policies, setting up mediated or self-deposit processes, or handling complex deposit types such as theses and research data. The goal is to prevent the agent from building a workflow that ignores rights clearance, accepts inadequate metadata, fails to enforce embargoes consistently, or creates a deposit process so burdensome that researchers avoid it.

## Core Rules

### Define What The Repository Accepts And Why

A deposit policy must state clearly what content the repository will and will not accept, and the criteria behind those boundaries. A repository that accepts everything becomes unmanageable; one that accepts too little fails to serve its mission. The scope should be explicit and defensible.

A deposit scope policy should specify:

- the content types accepted: scholarly publications, theses, datasets, learning objects, grey literature, administrative records, cultural heritage materials;
- the community eligible to deposit, by affiliation, role, or relationship to the institution;
- the exclusions and the reasoning, such as formats the repository cannot preserve or content outside the institutional mission;
- the criteria for research data, including minimum documentation and format expectations;
- the relationship to other repositories, when deposit should be redirected to a disciplinary or generalist repository;
- the process for requesting exceptions or negotiating non-standard deposits.

Vague scope ("scholarly outputs of the institution") invites inconsistent decisions and disputes. Specific scope enables consistent intake.

### Resolve Rights Before Accepting Content

The most damaging repository failures involve content deposited without clear rights, leading to takedowns, legal exposure, or embargoes that cannot be enforced. Rights clearance must be a precondition of deposit, not an afterthought, and the policy must make the depositor's responsibility explicit while providing institutional support for complex cases.

Rights review should address:

- whether the depositor has the right to deposit, for published works often limited by the publisher's policy, discoverable via Sherpa Romeo or direct publisher terms;
- the version to be deposited, accepted manuscript versus published version, and how the version is identified;
- any embargo period required by the publisher or funder, and how the embargo is recorded and enforced;
- third-party content within the deposit, such as images, figures, or data the depositor did not create;
- the license under which the deposit is made available, such as a Creative Commons license, and whether the depositor can grant it;
- the non-exclusive license granted to the institution to preserve and distribute the work.

Document the rights basis for every deposit. A repository that cannot explain why it has the right to hold and distribute a work cannot defend a takedown request or a challenge.

### Design Metadata Quality Into The Workflow

Metadata is what makes deposited content discoverable, citable, and preservable. Deposit workflows that accept whatever metadata the depositor provides produce a repository that is neither. Metadata quality must be engineered into the workflow through required fields, controlled vocabularies, validation, and a review step for mediated deposit.

Build metadata quality control by:

- defining a mandatory metadata profile with required fields such as title, creator, date, resource type, abstract, and rights;
- using controlled vocabularies and authority records for names, subjects, and identifiers, including ORCID, ROR, and DOI;
- validating identifiers and formats at submission, rejecting deposits with malformed or missing required metadata;
- providing depositor-facing guidance and templates that make good metadata the path of least resistance;
- building a mediated review step for high-value or complex deposits, such as theses and datasets;
- recording provenance metadata for who deposited what and when, using PREMIS or equivalent.

Metadata review is not optional polish; it is the difference between a citable, discoverable repository and an opaque one.

### Choose Between Self-Deposit And Mediated Deposit Deliberately

The choice between depositor self-service and staff-mediated deposit shapes the entire workflow, the staffing model, and the quality of the collection. Self-deposit scales but depends on depositor motivation and produces variable metadata; mediated deposit produces quality but creates a bottleneck tied to staff capacity. Most repositories use a hybrid, with self-deposit for routine content and mediation for complex or high-stakes items.

Decide based on:

- the volume and regularity of deposits, which determines whether mediation is feasible;
- the metadata and rights complexity of the content, which determines whether mediation is necessary;
- the depositor community's willingness and capacity to self-deposit;
- the staffing available for review and remediation;
- the strategic value of the content, which may justify mediation even when self-deposit is possible.

Neither pure model is universally right. The decision should be explicit and revisited as volume and staffing change.

### Handle Theses And Dissertations As A Special Workflow

ETDs are among the most common and most complex repository content types because they combine mandatory deposit, rights complexity, embargoes, and high metadata expectations. A dedicated ETD workflow should address the specific risks.

ETD workflow considerations:

- integration with the graduate school's degree-award process so deposit is a condition of graduation;
- embargo handling for patent-sensitive or publishable material, with clear maximum embargo periods and automatic release;
- verification of the deposited version as the final approved version;
- supplementary material and large files, including datasets and multimedia;
- author agreements covering preservation, distribution, and the non-exclusive license;
- persistent identification and citation, including DOI assignment where appropriate;
- discovery integration so ETDs appear in catalog and discovery layers.

ETDs fail most often when the embargo is recorded inconsistently or when the workflow depends on a single staff member's institutional knowledge.

### Manage Embargoes As Enforced, Expiring Obligations

Embargoes are not passive metadata; they are active obligations that must be enforced until they expire and then reliably lifted. A repository that records an embargo but fails to release the content on time, or releases it early, has failed both the author and the reader. Embargo management must be operational, not documentary.

Operational embargo management requires:

- recording the embargo end date as machine-actionable data, not free text;
- automated review of embargoed items approaching expiry and at expiry;
- a workflow to transition items from restricted to open access on the correct date;
- clear access conditions during embargo, including who can see what;
- audit logging of embargo changes and releases;
- handling of embargo extensions requested by authors, with documented criteria.

Treat embargo expiry as a scheduled event with an owner, not as something that will be noticed when a patron complains.

### Make Deposit Low-Friction For Depositors

Even the best policy fails if the deposit process is so painful that researchers avoid it. Deposit friction is the single greatest predictor of repository underuse. The workflow should minimize depositor effort while preserving quality, leveraging integrations and automation.

Reduce friction by:

- pre-filling metadata from external sources such as Crossref, ORCID, and the institutional identity system;
- integrating deposit into existing researcher workflows, such as grant reporting or publication submission;
- providing clear, short guidance rather than long policy documents at the point of deposit;
- accepting content in the formats researchers actually produce, then transforming for preservation;
- offering personal assistance for first-time or complex deposits;
- providing depositors with immediate confirmation, a persistent link, and compliance evidence for funders.

The goal is for the path of least resistance for the researcher to be the path that results in a well-described, rights-clear deposit.

### Document The Workflow So It Survives Staff Turnover

Repository workflows often live in the heads of one or two staff members. When they leave, the workflow breaks. Every deposit workflow, including its exceptions and judgment calls, must be documented well enough that a competent successor can operate it.

Documentation should cover the end-to-end process, decision points and their criteria, tools and accounts used, contact points for escalations, and the handling of common exceptions. Review the documentation against actual practice periodically, because undocumented drift is how workflows silently degrade.

## Common Traps

### Accepting Content Without Rights Clearance

Depositing content because the depositor provided it, without verifying the right to deposit and distribute, creates legal and reputational risk. This is a trap because the deposit feels like success but stores up takedown risk that surfaces later and publicly.

### Letting Metadata Quality Drift

Accepting depositor-provided metadata without validation or review produces a repository that is technically full but practically undiscoverable. This is a trap because the degradation is invisible until someone searches and fails to find what is there.

### Recording Embargoes But Not Enforcing Them

Treating embargo as a metadata field rather than an operational obligation leads to premature or delayed release. This is a trap because embargo failures damage author trust and can violate publisher or funder terms.

### Building A Workflow Only One Person Understands

Concentrating workflow knowledge in a single staff member means turnover breaks the repository. This is a trap because the risk is invisible until the person leaves, at which point recovery is expensive.

### Making Deposit So Hard Researchers Avoid It

A workflow optimized for internal quality at the expense of depositor experience produces an empty repository. This is a trap because the policy looks rigorous but the collection never materializes.

### Accepting Everything To Drive Volume

Lowering scope and quality standards to inflate deposit counts produces a repository that cannot be trusted for completeness or description. This is a trap because volume metrics feel like success while undermining the repository's value proposition.

### Ignoring Research Data Specificity

Treating datasets like publications, with the same workflow and metadata, fails because datasets require documentation, format consideration, and size handling that publications do not. This is a trap because data deposits then arrive under-described and unpreservable.

## Self-Check

- Does the deposit policy state explicitly what content is accepted, by whom, and on what criteria, with a process for exceptions?
- Is rights clearance a precondition of every deposit, with the version, embargo, third-party content, and license documented?
- Does the workflow enforce a mandatory metadata profile with controlled vocabularies, validation, and review for complex deposits?
- Is the self-deposit versus mediated-deposit decision explicit, justified by volume and complexity, and reviewed periodically?
- For ETDs, is deposit integrated with degree award, are embargoes bounded and enforced, and is the final version verified?
- Are embargoes recorded as machine-actionable dates with automated expiry handling and audit logging?
- Have you reduced depositor friction through pre-filling, integration, guidance, and assistance?
- Is the full workflow, including exceptions and judgment calls, documented well enough to survive staff turnover?
- Is there a mechanism to redirect out-of-scope content to an appropriate disciplinary or generalist repository?
