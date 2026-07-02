---
name: deemed_export_and_reexport.md
description: Use when the agent is advising on deemed exports and intangible technology transfers to foreign nationals, re-export of controlled items between third countries, US-origin item re-export reach and de minimis rules, release of controlled technology to foreign-person employees and visitors, mergers and acquisitions affecting export-control status, and global movement of controlled goods, software, and technology.
---

# Deemed Export and Re-export

Two concepts cause the most enforcement exposure in export control: the deemed export (a release of controlled technology to a foreign national inside the country of origin is treated as an export to that person's country of nationality) and the re-export (a subsequent movement of a controlled item from one foreign country to another, which remains subject to the origin country's controls). Both are counter-intuitive because they regulate conduct that does not look like a cross-border shipment.

The judgment problem is that deemed exports happen invisibly inside a company's own offices, labs, and code repositories whenever a foreign-national employee, contractor, or visitor accesses controlled technology, and re-exports happen whenever a controlled item moves again after its first export, even between two non-origin countries. Agents frequently miss these because there is no physical border crossing and no customs filing, yet the legal exposure equals or exceeds that of a direct export. Mergers and acquisitions add a further layer, as a change in ownership can change the export-control status of items and trigger re-licensing.

This skill is advisory only. Deemed-export and re-export rules are jurisdiction-specific and technically intricate. Confirm with qualified trade-compliance counsel before acting.

## Core Rules

### Treat any release of controlled technology to a foreign national as a deemed export

A deemed export occurs when controlled technology or source code is released to a foreign national (non-citizen, in many regimes also non-permanent-resident) inside the country of origin. Release can be visual inspection, oral exchange, application for the technology, or practice with it, and includes access via shared repositories, production systems, design reviews, and technical support.

- Map every location and system where controlled technology resides, and identify who has or could obtain access.
- The relevant nationality for a deemed export is generally the person's most recent citizenships or nationalities, not their place of birth or residence.

### Apply license requirements to deemed exports as to physical exports

If a license would be required to export the technology to the foreign national's country of nationality, a license is required to release it to that person domestically. License exceptions may apply, but only within their conditions.

- Do not assume that hiring a foreign national is purely an immigration matter; it is also an export-control matter if the role accesses controlled technology.
- Permanent residents are generally treated as nationals of the country of permanent residence in some regimes (notably the US), but verify the rule.

### Control intangible release channels systematically

Deemed exports occur through channels that trade-compliance teams may not see: shared code repositories, engineering wikis, cloud production environments, design-review meetings, visitor tours of facilities, and technical-support calls. Access controls and role-based permissions are the primary control.

- Implement nationality-based access restrictions on controlled technology, reviewed against the license status for each nationality.
- Manage visitors, contractors, and joint-venture partners with the same rigor as employees when they may access controlled technology.

### Understand re-export reach of origin-country controls

A re-export is a movement of an item from one foreign country to another. Items that originated in a controlling jurisdiction (notably US-origin items under the EAR) generally remain subject to that jurisdiction's controls wherever they are located, including when re-exported between third countries.

- Trace the origin of items, software, and technology in the supply chain and product to identify whose re-export controls apply.
- Re-export of a controlled item may require a license from the origin country's authority, even when neither the exporter nor the importer is in that country.

### Apply de minimis and foreign-direct-product rules carefully

Origin-country re-export controls apply to items with more than a de minimis proportion of controlled origin-country content, and (under some regimes, notably US rules) to items that are the direct product of controlled technology or of a plant that is the direct product of controlled technology. These rules extend reach to items manufactured abroad.

- Calculate de minimis content correctly; the threshold and the included content differ by destination and item type.
- Foreign-direct-product rules can capture items with no physical US content if they are produced using controlled US technology.

### Manage mergers, acquisitions, and corporate changes

A change in the ownership, control, or nationality of a company can change the export-control status of its items, employees, and transactions. An acquisition by a foreign person can convert previously domestic technology access into deemed exports, and can trigger investment-screening and export-license requirements.

- Conduct export-control due diligence in M&A, mapping controlled technology, foreign-national access, and existing licenses.
- Post-closing integration must not proceed in a way that creates unauthorized deemed exports or re-exports before new licenses are obtained.

### Coordinate deemed-export, immigration, and security-clearance processes

Deemed-export analysis, immigration/work-authorization, and any facility security clearances interact. A person may be work-authorized but not export-authorized for a given role.

- Align HR, trade-compliance, and security functions so that hiring, onboarding, and role changes trigger export review.
- Document deemed-export license applications and approvals tied to specific individuals and roles, and review on role change.

## Common Traps

### Treating deemed exports as an HR-only matter

Deemed-export compliance requires coordination between trade compliance, HR, IT (access controls), and security. Leaving it to HR results in foreign-national hires with uncontrolled access to controlled technology. Make it a cross-functional process.

### Using place of birth or residence instead of nationality

The relevant attribute for deemed-export analysis is generally the person's citizenship(s)/nationality(ies). Using residence or birthplace misidentifies the destination country and the license requirement. Capture all nationalities.

### Assuming a domestic release is not an export

The defining feature of a deemed export is that it occurs domestically. Teams that think "export means shipping abroad" will miss every deemed export. Train engineering, research, and support teams specifically.

### Ignoring re-export reach of US-origin items

Items with US-origin controlled content above de minimis, or that are foreign-direct products of US technology, remain subject to US EAR worldwide. Reselling such items between third countries without re-export analysis is a frequent violation. Trace origin.

### Foreign-direct-product blind spots

Even items with no physical US content can be caught if produced using controlled US technology or tooling. Assuming "made abroad means not subject to US rules" ignores the foreign-direct-product rules.

### M&A integration creating unauthorized exports

Post-acquisition integration that combines controlled technology with foreign-national access before export review can create immediate deemed-export violations. Sequence integration after export-control due diligence and licensing.

### Visitor and joint-venture access overlooked

Tours of facilities, joint-development projects, and contractor access can release controlled technology to foreign nationals. Visitors and partners need the same access controls as employees.

### Stale deemed-export licenses

A deemed-export license tied to an individual does not transfer to a new role, project, or employer. Re-assess on role change, project change, or corporate change.

## Self-Check

- Have I mapped all locations and systems where controlled technology resides, and identified foreign-national access for deemed-export analysis?
- For each foreign national with access to controlled technology, have I determined the license requirement based on their nationality(ies) and the destination-country controls, and applied for licenses where required?
- Are access controls on repositories, production systems, design reviews, and support channels implemented by nationality and license status, covering employees, contractors, visitors, and partners?
- For re-exports, have I traced the origin of items, software, and technology, and identified whose re-export controls apply, including de minimis and foreign-direct-product reach?
- For any merger, acquisition, or corporate change, has export-control due diligence been performed, and is integration sequenced to avoid unauthorized deemed exports or re-exports?
- Are deemed-export, immigration, and security processes coordinated so that hiring, onboarding, and role changes trigger export review?
- Are deemed-export licenses documented per individual and role, and re-assessed on role, project, or corporate change?
- Have I flagged that deemed-export and re-export rules are jurisdiction-specific and must be confirmed with qualified trade-compliance counsel?
