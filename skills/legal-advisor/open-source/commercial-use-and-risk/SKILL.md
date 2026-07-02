---
name: open_source_commercial_use_and_risk.md
description: Use when the agent is commercializing software that includes open source components, structuring dual licensing models, avoiding copyleft contamination of proprietary code, evaluating warranty disclaimer and liability limitation effects, assessing indemnification gaps, conducting open source due diligence for M&A or product launch, or reviewing business risk of using GPL AGPL copyleft code in commercial offerings.
---

# Open Source Commercial Use and Risk

Commercial use of open source software is lawful and widespread, but the commercial context amplifies the legal consequences of compliance failures. When software is sold, licensed to enterprises, embedded in products, or offered as a service, the stakes of a license violation rise from embarrassment to injunction, damages, and forced disclosure of proprietary code. The judgment problem is that open source licenses are drafted to enable free use and redistribution, which can lull commercial users into treating them as risk-free, when in fact their conditions, once breached, terminate the license automatically and expose the user to copyright infringement claims with none of the protections a commercial licensee would negotiate.

The second judgment problem is the warranty and liability vacuum. Open source licenses universally disclaim warranties and limit liability, which is favorable to the licensor but means the commercial user bears all risk of defects, security vulnerabilities, and IP infringement claims. There is no vendor to indemnify. An agent advising on commercial use must therefore evaluate not only compliance but the allocation of risk that open source imposes, and structure the commercial offering (indemnities, insurance, escrow, support) to fill the gaps that the open source license creates. The skill is using open source aggressively where it creates value, while engineering the commercial and legal structure to contain the risks that its license terms and provenance create.

This skill addresses commercial use of open source under US law with reference to international practice. The interaction of open source licenses with commercial contracts, insurance, and M&A due diligence involves judgment and unsettled questions; a licensed attorney should review material commercial arrangements.

## Core Rules

### Map the commercial model to the compliance trigger

Compliance obligations depend on how the software reaches the customer. Selling or distributing binaries triggers distribution obligations; providing a hosted service may trigger AGPL network obligations but generally not GPL; embedding in a physical product triggers distribution; internal tooling triggers little. Map each commercial model to the licenses in the stack and confirm obligations are satisfied for that model. A compliance program that is correct for one distribution model may fail for another. Re-evaluate when the commercial model changes, for example moving from on-premise distribution to SaaS.

### Engineer to avoid unwanted copyleft propagation

Where copyleft would contaminate proprietary code, engineer the architecture to preserve separation. Options include replacing the copyleft component with a permissive alternative, isolating the copyleft code behind a process or network boundary so it remains a separate work, or confining use to internal operations that do not trigger distribution. Document the architectural rationale and enforce it through code review so future changes do not collapse the separation. Treat copyleft management as an ongoing engineering discipline, not a one-time decision.

### Evaluate dual licensing as a business model

Dual licensing distributes the same software under both an open source license (typically a strong copyleft like GPL) and a separate commercial license. Users who want to avoid copyleft obligations buy the commercial license. This model, used by projects like MySQL historically, monetizes users who cannot accept copyleft. It requires that the steward own or license all rights to the code, which depends on a robust contribution framework (see the contribution and CLA skill). Dual licensing is viable only where copyright aggregation is achievable and where the copyleft license creates genuine pressure to purchase the commercial license.

### Account for the warranty and liability vacuum

Open source licenses disclaim all warranties and limit liability to the maximum extent permitted by law. The commercial user receives no warranty of merchantability, fitness for purpose, or non-infringement, and no recourse for defects or damages. This shifts all risk to the commercial user and, through the commercial offering, to its customers. Structure the commercial offering to address the gaps: provide your own warranties and support to customers, obtain intellectual property infringement insurance, maintain code escrow where continuity matters, and build internal testing and security processes that substitute for vendor warranties. Do not assume the open source license provides any protection for your customers.

### Assess indemnification gaps and fill them contractually

Commercial customers increasingly demand IP infringement indemnities. Open source licenses provide none; the licensor disclaims liability. A company commercializing open source-based software must decide whether to offer indemnities to its customers, which exposes it to infringement claims it cannot pass through to the open source licensor. Options include offering indemnities backed by insurance, offering limited indemnities with caps, or declining to indemnify for components the customer knows are open source. Match the indemnity offering to the company's risk tolerance and insurance coverage, and disclose the open source provenance so customers understand the basis.

### Conduct open source due diligence before M&A and product launch

Open source issues are a frequent finding in M&A due diligence and a frequent cause of deal delays or price adjustments. Before a product launch or transaction, conduct a full open source audit: generate an SBOM, verify license compliance, confirm contribution provenance, identify any copyleft contamination, and remediate issues. Buyers will require representations about open source compliance and may require indemnities or escrows for unresolved issues. Discovering a copyleft problem late in a transaction can derail it; early audit and remediation preserve deal value.

### Manage the reputational and community dimension

Commercial use of open source carries a community dimension. Failing to satisfy attribution or contribution norms, or attempting to rebrand community code without honoring its license, can generate public criticism and loss of goodwill that harms the business more than the legal risk. Engage with upstream projects, contribute back where appropriate, and respect community norms. For projects with contributor communities, coordinate commercial strategy with community expectations to avoid forks and conflict.

### Address security and maintenance responsibility

Open source components require ongoing maintenance for security vulnerabilities. The commercial user bears this responsibility because there is no vendor warranty. Establish processes to track vulnerabilities in open source dependencies (using databases and scanners), patch promptly, and backport fixes where necessary. The absence of a vendor means the company itself becomes the maintainer of last resort for the components it ships. Build this cost into the commercial model.

## Common Traps

### Treating open source as risk-free because it is free

Open source licenses are enforceable and their conditions, once breached, terminate the license automatically. Free of cost is not free of obligation or risk.

### Assuming SaaS avoids all copyleft

SaaS generally avoids GPL distribution obligations, but AGPL triggers on network interaction. Applying GPL analysis to AGPL code in a service is a critical error.

### Offering customer indemnities without backing

Promising IP infringement indemnities on open source-based software, without insurance or a cap, exposes the company to claims it cannot pass through. Match indemnities to coverage.

### Ignoring the warranty vacuum

Customers may assume the commercial vendor warrants the software. Open source disclaimers leave the vendor bearing all risk. Build substitute processes and contract terms.

### Discovering copyleft contamination during M&A

Late discovery of copyleft contamination can derail a transaction or reduce its value. Audit and remediate before the transaction, not during diligence.

### Collapsing architectural separation over time

An architecture designed to isolate copyleft code can be inadvertently collapsed by refactoring. Enforce separation through ongoing code review.

### Dual licensing without copyright aggregation

Dual licensing requires ownership or licensing of all rights. Attempting it with community contributions not covered by a CLA fails because the steward lacks rights to offer the commercial license.

### Neglecting security maintenance

Open source dependencies carry vulnerabilities that the commercial user must patch. Failing to maintain them creates security risk for customers and liability for the vendor.

## Self-Check

- Has each commercial model been mapped to the licenses in the stack, confirming obligations are satisfied for that specific distribution or service model?
- Where copyleft would contaminate proprietary code, is the architecture engineered to preserve separation, with the rationale documented and enforced through review?
- If pursuing dual licensing, is copyright aggregation achievable through the contribution framework, and does the copyleft license create genuine commercial pressure?
- Has the warranty and liability vacuum been addressed through substitute processes, customer warranties, insurance, and escrow where continuity matters?
- Are customer indemnity offerings matched to insurance coverage and risk tolerance, with open source provenance disclosed?
- Is open source due diligence (SBOM, compliance, provenance, copyleft) conducted before product launch and M&A transactions, with issues remediated early?
- Are security vulnerabilities in open source dependencies tracked and patched, with maintenance responsibility built into the commercial model?
- Does the commercial strategy respect community norms and upstream relationships to avoid reputational and community harm?
- Is the architecture for copyleft separation enforced over time, not just at initial design, through ongoing review?
- Have I flagged that commercial use interacts with open source licenses, commercial contracts, insurance, and M&A in ways that involve judgment and unsettled questions, and that a licensed attorney should review material commercial arrangements?
