---
name: deemed_exports_and_technology_release.md
description: Use when the agent is evaluating deemed export rules, release of controlled technology or source code to foreign nationals, visual inspection access, the fundamental research exemption, or technology controls within the United States workforce and research settings.
---

# Deemed Exports And Technology Release

An export does not require a border crossing. Under the EAR and ITAR, the release of controlled technology or technical data to a foreign person, wherever located, is "deemed" to be an export to that person's country of nationality. This means a US employer can commit an export control violation by sharing engineering data with a foreign-national employee inside the United States, by granting server access to a foreign contractor, or by allowing a visitor to visually inspect controlled equipment. Deemed export risk is concentrated in research institutions, engineering teams, manufacturing sites, and any environment where controlled technology is accessible to a multinational workforce.

Use this skill before advising on deemed export exposure, technology release controls, visual inspection safeguards, source code sharing, the fundamental research exemption, or workforce technology access governance. The compliance officer must identify where controlled technology exists, who can access it, and whether each release is authorized.

## Core Rules

### Understand The Deemed Export Principle

The deemed export rule treats the release of technology to a foreign person as an export to that person's country or countries of nationality.

- Under the EAR, a release of technology or source code subject to the EAR to a foreign person in the United States is deemed an export to the foreign person's most recent country or countries of nationality.
- Under the ITAR, releasing technical data to a foreign person is an export requiring authorization, including inside the United States.
- The release can be oral, visual, written, or electronic.
- Multiple nationalities matter: a person with two nationalities triggers deemed export analysis for each.

The location of the foreign person does not eliminate the rule; it applies inside the United States as well as abroad.

### Identify What Constitutes A Release

A release is broader than handing over a document. It includes any making available of technology.

- application or use of technology by a foreign person;
- visual inspection by a foreign person of controlled equipment, facilities, or blueprints;
- oral exchange of technology, including at meetings or conferences;
- access to source code, design files, or technical specifications on a server;
- practical application of technology in a lab or production line;
- internship, fellowship, or employment access to controlled technology.

Granting a foreign national network access to a directory containing controlled technology is a release, even if they never open the file. Access equals potential release.

### Map Where Controlled Technology Resides

Deemed export controls require knowing where controlled technology exists in the organization.

- engineering design files and CAD models;
- manufacturing process documentation;
- source code repositories;
- lab notebooks and research data;
- production floor equipment and tooling;
- technical specifications on shared drives or wikis;
- proprietary algorithms and models.

If the location of controlled technology is unmapped, access controls cannot be designed. Inventory technology holdings by classification level.

### Control Access By Nationality And Authorization

Access to controlled technology must be gated by the recipient's nationality and the applicable authorization.

- Identify the nationality of each person with potential access, including employees, contractors, visitors, interns, and collaborators.
- Determine the ECCN or USML category of the technology they could access.
- Determine whether a license, license exception, or exemption authorizes the deemed export to that nationality.
- Implement technical access controls (role-based access, segmentation, need-to-know) to prevent unauthorized access.
- For ITAR technical data, authorization is typically more restrictive than for EAR technology.

Do not rely on the absence of a request for access; design controls to prevent access by default.

### Handle Source Code And Software Specially

Source code is a frequent source of deemed export violations because it is easily shared.

- Source code for controlled items is itself controlled technology.
- Access to a repository by a foreign national is a release.
- Object code and executables may be controlled differently, but source code is the higher-risk asset.
- Open-source releases must be evaluated for control status before publication.

Gate repository access by nationality and authorization, and audit access logs.

### Apply The Fundamental Research Exemption Carefully

The fundamental research exemption excludes basic and applied research from EAR controls where the results are ordinarily published and shared broadly.

- The exemption applies to research at accredited institutions of higher learning where there is no publication restriction.
- It does not apply to proprietary research, research with publication approval rights granted to a sponsor, or research with access or dissemination controls.
- It does not cover the underlying controlled technology used to conduct the research; only the research results.
- ITAR has a narrower fundamental research exclusion; do not assume parity.

Confirm each element of the exemption before relying on it. A publication restriction imposed by a sponsor can destroy the exemption.

### Manage Visual Inspection And Tours

Visual inspection is a release. Tours of production floors, labs, or facilities can constitute a deemed export.

- Restrict foreign national access to areas with controlled technology.
- Use non-disclosure and visitor controls, recognizing they do not substitute for export authorization.
- Brief tour guides and hosts on deemed export risk.
- Control photography and recording in sensitive areas.

### Document Deemed Export Authorizations

Record each deemed export authorization: the technology classification, the recipient nationality, the authorization relied upon (license, exception, exemption), and the basis. Maintain access lists and reconcile them to authorizations periodically.

## Common Traps

### Assuming No Export Occurred Because Nothing Left The Country

A release to a foreign person in the United States is a deemed export. Domestic workforce sharing is within scope.

### Granting Broad Network Access Without Nationality Gating

Access to a shared drive containing controlled technology is a release. Default-open access creates unauthorized deemed exports.

### Treating Visual Inspection As Harmless

Visual inspection of controlled equipment is a release. Tours and facility access can constitute exports.

### Overlooking Multiple Nationalities

A person with two or more nationalities triggers deemed export analysis for each. Capturing only one nationality is incomplete.

### Misapplying The Fundamental Research Exemption

The exemption requires no publication restriction and applies to research results, not underlying technology. Sponsor controls can void it.

### Assuming ITAR And EAR Deemed Export Rules Are Identical

ITAR technical data releases are more restrictive. EAR license exceptions do not automatically apply to ITAR data.

### Ignoring Source Code Repository Access

Repository access by a foreign national is a release of source code. Audit logs and access controls are essential.

### Relying On Non-Disclosure Agreements In Place Of Authorization

NDAs do not authorize a deemed export. They are not a substitute for a license, exception, or exemption.

## Self-Check

- Is the deemed export principle applied so that releases of technology to foreign persons, wherever located, are treated as exports to their country or countries of nationality?
- Is "release" understood broadly to include oral, visual, written, electronic, and practical application, including server and repository access?
- Is controlled technology inventoried across engineering files, source code, process documentation, labs, and shared systems so access controls can be designed?
- Is access to controlled technology gated by recipient nationality and applicable authorization, with default-deny controls?
- Are multiple nationalities captured for each person with potential access, not only a single nationality?
- Is source code treated as controlled technology, with repository access gated and audited by nationality?
- Is the fundamental research exemption applied only where there is no publication restriction, recognizing it covers research results and not underlying technology?
- Is the ITAR fundamental research exclusion treated as narrower than the EAR exemption, without assuming parity?
- Are visual inspection and facility tour risks controlled, with foreign national access restricted in sensitive areas?
- Are deemed export authorizations documented with classification, recipient nationality, authorization relied upon, and basis, with access lists reconciled periodically?
