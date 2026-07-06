---
name: repository_platform_selection.md
description: Use when the agent is selecting or migrating a digital repository platform (DSpace, Fedora, Invenio, Samvera, Hyrax, Dataverse, Islandora), evaluating repository software for an institution, planning a platform migration, comparing open-source versus hosted repository solutions, or scoping repository infrastructure for theses, research data, cultural heritage, or institutional assets.
---

# Repository Platform Selection

Choosing a digital repository platform is one of the most consequential and least reversible decisions a library or archive makes. The platform becomes the substrate for years of deposits, metadata, identifiers, preservation actions, and public access, and a wrong choice locks the institution into workflows that are expensive to escape. The decision is rarely about which product is objectively best; it is about which product fits the institution's content types, staffing model, technical capacity, budget horizon, integration landscape, and preservation obligations. Agents tend to over-weight feature checklists and under-weight total cost of ownership, community health, migration risk, and the long tail of customization debt. A repository that demos well but has no local expertise to maintain it becomes a liability within two years.

Use this skill when evaluating, selecting, or migrating a digital repository platform. The goal is to prevent the agent from recommending on surface features, ignoring the staffing and sustainability realities, underestimating migration cost, or choosing a platform whose roadmap and community do not match the institution's preservation horizon.

## Core Rules

### Start From Content And Use Cases, Not From Software

Platform selection must begin with a precise description of what will be deposited and who will use it. A repository for electronic theses and dissertations (ETDs), one for research data, one for digitized cultural heritage objects, and one for institutional publications have genuinely different requirements, and no single platform is best at all of them.

Before comparing products, document:

- the content types: text, images, audio, video, datasets, software, compound objects, 3D models;
- the volume and growth rate of deposits;
- the depositor community: faculty self-deposit, mediated deposit, batch ingest from workflows;
- the primary user audiences and their access needs;
- the metadata complexity required, from simple Dublin Core to rich descriptive, administrative, and rights metadata;
- the preservation requirements, bit-level versus functional, and any certification ambitions such as CoreTrustSeal or TRAC;
- the integration requirements with discovery layers, authentication, ORCID, DOI minting, and preservation systems.

A feature matrix built before the use cases are defined produces a platform that scores well on paper but fails on the content the institution actually holds.

### Evaluate Total Cost Of Ownership, Not License Cost

Open-source repository software has no license fee, which creates the dangerous impression that it is free. The real costs are staffing, hosting, customization, upgrades, and the opportunity cost of staff time spent on infrastructure rather than services. A hosted or vendor-supported solution that appears expensive may be cheaper than a self-hosted open-source stack that requires a developer the institution does not have and cannot hire.

Total cost of ownership includes:

- systems administration and developer time for installation, configuration, and maintenance;
- hosting infrastructure, storage, and bandwidth, which grow with the collection;
- customization and integration development, almost always larger than estimated;
- ongoing upgrade effort, which compounds with customizations;
- training for repository managers and depositors;
- support contracts, consortium fees, or vendor maintenance;
- migration cost at the end of life, which should be planned from day one.

Build a multi-year cost model. A platform that is affordable in year one but unsustainable in year five is the wrong choice.

### Assess Community Health And Roadmap Realism

Repository platforms live or die by their communities. A platform with an active developer community, regular releases, documented migrations, and a governance model the institution can participate in is a safer long-term bet than a technically superior product with a shrinking community. DSpace, Fedora, Invenio, Samvera, Dataverse, and Islandora each have distinct community structures, release cadences, and sustainability models, and the institution should understand which it is joining.

Investigate:

- release frequency and whether releases are stable and documented;
- the size and diversity of the active developer base, not just the number of adopters;
- governance and funding model, and whether it is durable;
- the migration path between major versions and how painful past migrations were;
- the health of the user community mailing lists, conferences, and shared solutions;
- whether the roadmap addresses the institution's likely future needs or will require forks.

Be skeptical of roadmaps. What matters is delivered releases and a track record of non-catastrophic upgrades.

### Match Platform Architecture To Staffing Reality

The most common repository failure is not technical; it is a mismatch between the platform's operational demands and the institution's staffing. Fedora is powerful and flexible but assumes systems and developer capacity. DSpace is more turnkey but harder to customize deeply. Invenio and Samvera ecosystems reward institutions with developer capacity and punish those without it. A small library with no developer should not select a platform that requires one, however attractive its features.

Ask honestly:

- Does the institution have, or can it sustainably obtain, the systems administration capacity to run this platform?
- Does it have developer capacity for customization and integration, or will every change be a vendor bill?
- Is there a hosted or managed option that trades cost for staffing relief, and is that trade acceptable?
- Can the institution participate in a consortium or shared service to pool capacity?

The honest answer to staffing capacity should eliminate platforms before feature comparison begins.

### Plan Migration Before Committing

Migration is where repository projects bleed time and money, and where content is most at risk of loss or metadata degradation. The institution should have a credible migration plan before selecting a platform, not after, because a platform from which content cannot be exported cleanly is a trap.

Migration planning requires:

- confirming that the source system supports clean export of content and metadata in standard formats, such as METS, BagIt, or the platform's native export;
- assessing metadata quality in the source, because migration exposes every legacy shortcut;
- estimating the effort to map and transform metadata to the new platform's schema;
- planning for identifiers, persistent identifiers, and link persistence so existing citations do not break;
- accounting for fixity verification and checksum validation during transfer;
- budgeting time for a parallel-run or staged cutover rather than a big-bang migration.

If the candidate platform makes export difficult or proprietary, treat that as a serious red flag regardless of its other merits.

### Treat Preservation As A First-Class Requirement

A repository is not just an access platform; for many institutions it is the preservation system of record. The platform's preservation capabilities should be evaluated explicitly, not assumed. Bit-level preservation (fixity, redundancy, geographic replication) is the baseline. Functional preservation (format migration, emulation readiness, characterization) is a higher bar that few platforms meet alone and that usually requires integration with tools like Archivematica.

Evaluate:

- fixity checking mechanisms and how failures are surfaced;
- support for redundancy and replication, including geographically distributed copies;
- format characterization and risk reporting, such as via DROID, JHOVE, or Siegfried integration;
- the ability to record and act on preservation events using PREMIS;
- integration paths with dedicated preservation systems if the repository alone is insufficient;
- the institution's own capacity to act on preservation intelligence the platform produces.

A repository that stores content but cannot report on its preservation state is storage, not preservation.

### Consider The Integration Landscape

Repositories do not operate in isolation. They integrate with authentication and identity systems, discovery and catalog systems, citation and identifier services, ORCID and researcher profile systems, and downstream preservation and publishing systems. The integration cost and reliability often dominate the total effort of running a repository.

Map the required integrations and assess each platform's support for them: SAML or OIDC authentication, OAI-PMH harvesting, DOI and Handle minting, ORCID integration, IIIF for image and AV delivery, SWORD for deposit, and APIs for custom integrations. A platform strong in isolation but weak in integration will generate constant custom work.

## Common Traps

### Choosing On Feature Checklist Score

Feature matrices reward breadth over fit and ignore whether features are usable by the institution's actual staff. A platform can win a checklist and fail in production because no one can operate its features. This is a trap because checklists feel objective but measure the wrong thing.

### Treating Open Source As Free

The absence of a license fee hides the staffing, hosting, and customization costs that dominate repository TCO. This is a trap because budget conversations that start from "it's free" set unrealistic expectations and lead to under-resourced implementations that decay.

### Underestimating Customization Debt

Every customization made to adapt a platform to local needs becomes a liability at upgrade time. This is a trap because customizations feel like progress when made but compound into migration-blocking technical debt that can strand an institution on an obsolete version.

### Ignoring Migration Risk Until It Is Too Late

Institutions often select a platform and only later discover that exporting content is painful or that metadata does not map cleanly. This is a trap because migration cost, discovered late, can exceed the cost of the original selection and put content at risk.

### Assuming The Roadmap Will Deliver

Vendor or community roadmaps describing future features can sway selection toward a product that cannot yet meet needs. This is a trap because roadmaps slip, and an institution that selects on promised features may wait years for capabilities that arrive late or differently.

### Overbuilding For A Small Collection

Selecting an enterprise-grade platform for a modest collection imposes operational overhead disproportionate to the content. This is a trap because the institution pays platform complexity tax forever for capacity it does not use, and staff burn out maintaining infrastructure instead of services.

### Forgetting That Someone Must Deposit

Repositories with no deposits are empty shells. This is a trap because selection focused on the platform ignores the deposit workflow and depositor experience, and a platform nobody can or will deposit into fails regardless of its technical merits.

## Self-Check

- Did you define the content types, volume, depositors, audiences, and preservation requirements before comparing platforms?
- Does your cost model span at least five years and include staffing, hosting, customization, upgrades, and eventual migration?
- Did you assess the community health, release cadence, and governance of each candidate, rather than relying on feature lists?
- Did you honestly match each platform's operational demands to the institution's actual systems and developer capacity?
- Is there a credible, costed migration plan from the current system, with confirmed export and metadata mapping paths?
- Did you evaluate preservation capabilities explicitly, including fixity, redundancy, format characterization, and PREMIS event recording?
- Did you map required integrations (authentication, discovery, identifiers, IIIF, SWORD, ORCID) and confirm each platform supports them?
- Did you avoid selecting on roadmap promises, relying instead on delivered, documented releases?
- Is the deposit workflow and depositor experience part of the evaluation, not just the platform backend?
