---
name: open_source_contribution_and_cla.md
description: Use when the agent is establishing contribution policies for an open source project, drafting or reviewing Contributor License Agreements CLAs and Developer Certificate of Origin DCO, managing copyright assignment versus license grant, reviewing inbound equals outbound license terms, evaluating corporate contributor approval, or designing governance for projects accepting external contributions.
---

# Open Source Contribution and CLA

When a project accepts contributions from outside its core team, it receives code carrying the contributor's own copyright and any underlying rights. Without a contribution framework, the project accumulates code whose licensing status is unclear, whose origin is unverified, and whose future relicensing is blocked by the need to locate and obtain consent from every contributor. The judgment problem is that contribution management is invisible when things go well and catastrophic when they go wrong. Agents often treat contribution agreements as administrative overhead, when in fact they govern the project's ability to distribute, defend, and relicense its own software.

The second judgment problem is the tension between contributor friction and project protection. Heavy contribution agreements deter casual contributors and slow community growth; no agreement at all leaves the project legally exposed and unable to relicense. The skill is choosing the right instrument (CLA, DCO, or inbound-equals-outbound), calibrating its burden to the project's scale and risk, and applying it consistently so that every line of code in the project has a clear provenance and license. Agents must also navigate the corporate contributor problem: an individual's contribution may belong to their employer, and a project that accepts it without employer consent inherits an unresolved rights issue that can surface years later.

This skill addresses contribution frameworks under US copyright and contract law with reference to major project practices (Apache CLA, GNU Fiduciary License Agreement, Linux DCO). CLA drafting involves legal judgment about scope and enforceability; a licensed attorney should review project contribution policies.

## Core Rules

### Choose the contribution instrument deliberately

Three principal instruments exist, each with different burdens and protections. A Contributor License Agreement (CLA) is a contract under which the contributor grants the project a license (and sometimes assigns copyright) to the contribution; it can be comprehensive or lightweight. A Developer Certificate of Origin (DCO) is a lightweight attestation, signed via commit tag, that the contributor has the right to submit the code under the project license; it imposes minimal friction. An inbound-equals-outbound clause (as in Apache 2.0) provides that contributions are licensed under the same terms as the project license, requiring no separate agreement. Choose based on project needs: large projects needing relicensing flexibility and patent grants typically use a CLA; community projects prioritizing low friction often use DCO or inbound-equals-outbound.

### Distinguish license grant from copyright assignment

A CLA may grant a license or require assignment of copyright. A license grant leaves copyright with the contributor but gives the project rights to use, modify, and redistribute; this is the common, contributor-friendly model. Copyright assignment transfers ownership to the project or its steward, which gives maximum flexibility (including relicensing under different terms) but is more intrusive and, in some jurisdictions, must meet formal requirements to be effective. Assignment is typically used only where the steward needs full control, such as projects that dual-license or may change license in the future. Match the instrument to the project's relicensing needs.

### Secure an explicit patent grant from contributors

Copyright alone does not cover patent rights a contributor may hold that the contribution practices. A robust CLA includes a patent grant: the contributor grants a license to any patents necessary to practice the contribution. Without it, a contributor could contribute code that practices their patent and later assert infringement against the project or its users. The Apache 2.0 license and the Apache CLA include patent grants; projects accepting contributions without a patent grant carry this risk. Evaluate whether the project's risk profile warrants a patent grant.

### Address the employer-ownership problem explicitly

An individual contributor's code may belong to their employer under work-for-hire or employment IP-assignment rules. A contribution from an individual who lacks authority to license their employer's rights creates an unresolved chain of title. Robust contribution frameworks address this: some CLAs require employer consent for contributors whose work is owned by an employer; the DCO's attestation that the contributor is authorized implicitly shifts responsibility to the contributor. For corporate contributors, require that contributions be made under the employer's authority, often through a corporate CLA. Do not assume an individual's sign-off clears employer rights.

### Define the scope of the license grant clearly

The CLA's grant should specify what rights the project receives: a perpetual, irrevocable, worldwide, royalty-free license to use, modify, distribute, and sublicense the contribution, and to prepare derivative works. Address whether the grant is exclusive or non-exclusive (almost always non-exclusive, so the contributor retains rights). Define whether the grant covers only the contribution or extends to related patents. Ambiguous scope invites disputes; precise scope protects both parties. Ensure the grant permits the project to distribute under its chosen license, including future versions.

### Preserve relicensing flexibility where needed

If the project may need to change its license in the future (for example, to a newer version of a license or to a more permissive or protective terms), the contribution framework must permit it. A CLA can grant broad sublicensing rights that enable relicensing; copyright assignment enables it fully; a DCO or inbound-equals-outbound tied to a specific license version may constrain relicensing to compatible licenses only. Assess the project's likely future needs and choose an instrument that preserves the necessary flexibility. Projects that have needed to relicense without broad grants have faced years of contributor outreach.

### Maintain provenance records for every contribution

The project must be able to show, for any line of code, that it was contributed under the project's terms. Require sign-off (DCO tag), signed CLA, or equivalent for every contribution, and retain the records. Use tooling that blocks merges without the required attestation or agreement. Provenance records are the project's defense against claims of unauthorized inclusion and the foundation for any future relicensing or license defense. A contribution accepted without a record is a contribution the project may not be able to prove it has the right to use.

### Establish governance for contribution acceptance

Define who may accept contributions, what review is required, and how the contribution framework is enforced. For corporate-sponsored projects, clarify the relationship between corporate contributors and the project steward. For foundation-hosted projects, follow the foundation's contribution policies. Governance that is unclear or inconsistent undermines the legal framework, because inconsistent enforcement suggests the framework is not actually binding.

## Common Traps

### No contribution framework at all

Accepting contributions without any agreement, CLA, or DCO leaves the project with unclear rights to its own code, unable to relicense, and vulnerable to claims of unauthorized inclusion.

### Conflating CLA and DCO

A CLA is a contract with broad grants; a DCO is an attestation of authority. They serve different purposes and are not interchangeable. Choose the instrument that matches the project's needs.

### Ignoring employer ownership of individual contributions

An individual's contribution may belong to their employer. Accepting it without employer authority leaves an unresolved rights issue that can surface years later, often after the contributor has left.

### No patent grant

Without a patent grant from contributors, the project carries the risk that a contributed practice is later asserted as patent infringement. Evaluate whether the risk warrants requiring a grant.

### Overly burdensome CLA that deters contributors

A heavy CLA with assignment and broad grants may protect the project but deter casual contributors and slow community growth. Calibrate the burden to the project's scale.

### Inconsistent enforcement

If the contribution framework is enforced for some contributors and not others, its binding effect is undermined. Enforce consistently through tooling.

### Ambiguous relicensing rights

A framework tied to a specific license version may block future relicensing. If relicensing flexibility is needed, secure broad sublicensing or assignment rights from the start.

## Self-Check

- Has the project chosen a contribution instrument (CLA, DCO, or inbound-equals-outbound) matched to its scale, risk, and relicensing needs?
- Does the framework distinguish license grant from copyright assignment, and is the choice consistent with the project's relicensing and control needs?
- Does the framework include a patent grant from contributors if the project's risk profile warrants it?
- Does the framework address the employer-ownership problem, requiring employer authority for contributions owned by an employer?
- Is the scope of the license grant clear, covering use, modification, distribution, sublicensing, and derivative works, on a non-exclusive basis?
- Does the framework preserve relicensing flexibility if the project may need to change licenses, through broad sublicensing or assignment?
- Are provenance records maintained for every contribution, with tooling that blocks merges without the required attestation or agreement?
- Is governance for contribution acceptance defined, including who may accept contributions and how the framework is enforced consistently?
- Have I avoided over-burdensome agreements that deter contributors while still protecting the project?
- Have I flagged that CLA and contribution policy drafting involves legal judgment and jurisdictional variation, and that a licensed attorney should review the project's framework?
