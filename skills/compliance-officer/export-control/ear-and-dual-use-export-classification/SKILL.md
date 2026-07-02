---
name: ear_and_dual_use_export_classification.md
description: Use when the agent is classifying an item, software, or technology under the EAR Commerce Control List, assigning an ECCN or EAR99, determining dual-use status, reading the country chart, or deciding whether an export license is required.
---

# EAR And Dual-Use Export Classification

Export control compliance begins with classification. Under the Export Administration Regulations (EAR), administered by the Commerce Department's Bureau of Industry and Security (BIS), every item, software, and technology subject to the EAR must be classified to determine whether a license is required for a given export, reexport, or transfer. The classification determines the control parameters: the reason for control, the country chart intersections, the license exceptions that may apply, and the recordkeeping that follows. A wrong classification cascades into wrong license determinations, illegal shipments, and enforcement exposure that is often strict liability.

Use this skill before classifying an item under the EAR, assigning an Export Control Classification Number (ECCN), concluding an item is EAR99, determining dual-use status, reading the country chart for license requirements, or selecting a license exception. The compliance officer must drive classification from the item's technical parameters and the ECCN text, not from a guess or a vendor's label.

## Core Rules

### Establish Jurisdiction Before Classification

Before classifying under the EAR, confirm the item is subject to the EAR and not subject to another agency's jurisdiction.

- Items subject to the EAR include all items in the United States, US-origin items wherever located, certain foreign-made items containing more than a de minimis level of controlled US content, and certain foreign-made direct products of US technology or software.
- Items subject to the ITAR (US Munitions List) are not EAR items; they are State Department jurisdiction. See the ITAR skill.
- Items subject to other agencies (nuclear, certain drug, certain Treasury) are excluded from the EAR.
- Foreign-made items may be subject to the EAR through the foreign direct product rule or de minimis rules.

If jurisdiction is wrong, the classification is wrong. Resolve jurisdiction first.

### Determine The Commerce Control List Classification

Items subject to the EAR are either specifically described in a Commerce Control List (CCL) ECCN or fall to EAR99.

- The CCL organizes items into categories (0 through 9) and product groups (A equipment, B test, C materials, D software, E technology).
- An ECCN specifies the controlled items by technical parameters, the reasons for control (national security, anti-terrorism, missile technology, crime control, etc.), and the country chart intersections.
- If an item is not specifically described in any ECCN, it is EAR99, meaning it is subject to the EAR but generally does not require a license for most destinations.
- EAR99 is a residual catch-all, not a conclusion that the item is uncontrolled; end-use, end-user, and embargo rules still apply.

Read the ECCN technical parameters carefully. Classification turns on whether the item meets the specific performance thresholds, not on a general description.

### Classify Technology And Software Separately

Technology and software are controlled distinctly from the hardware they relate to.

- Technology for the development, production, or use of a controlled item is itself controlled, often under the corresponding "E" or "D" group ECCN.
- Source code may be controlled differently from object code or executables.
- The release of technology to a foreign person, wherever located, can be a deemed export; see the deemed exports skill.
- Bundling hardware with software or technology does not collapse the classifications; each component needs its own.

### Read The Reasons For Control And Country Chart

The license requirement depends on the intersection of the ECCN's reasons for control and the destination.

- Each ECCN lists reasons for control (NS, AT, MT, CC, CB, RS, etc.) and the country chart columns that apply.
- A license is required if the reason for control applies to the destination on the country chart.
- Some controls are unilateral, others multilateral (Wassenaar, MTCR, Australia Group, Nuclear Suppliers Group).
- Anti-terrorism (AT) controls apply broadly to many destinations and often drive license requirements for EAR99 items to embargoed countries.

Do not assume a license is or is not required based on destination alone; read the specific reason-for-control and country chart intersection for the ECCN.

### Evaluate License Exceptions Before Applying For A License

License exceptions authorize exports that would otherwise require a license, subject to conditions.

- License Exception STA, LVS, GBS, TSR, and others each have eligibility criteria based on item, destination, and end-use.
- Exceptions have conditions and recordkeeping requirements; misuse voids the authorization.
- Not all ECCNs permit all exceptions; check the ECCN's exception eligibility.
- End-use and end-user restrictions can override an otherwise available exception.

Confirm the exception applies to the specific item, destination, end-use, and end-user, and document the basis.

### Apply End-Use, End-User, And Entity List Controls Independently

Even if an item is EAR99 or a license exception applies, separate controls can require a license or prohibit the export.

- Entity List, Unverified List, Military End-User List, and Denied Persons List impose license requirements or presumptions of denial independent of ECCN.
- End-use rules (military, intelligence, WMD, certain semiconductor, certain aircraft) can require a license regardless of classification.
- Comprehensive embargoes prohibit exports regardless of classification.

Classification is necessary but not sufficient. Layer end-use and end-user analysis on top.

### Document The Classification With Evidence

A classification must be defensible. Record the ECCN, the technical parameters relied upon, the reason for control, the country chart intersection, the license determination, any exception relied upon, and the date. Classifications must be revisited when the item, the regulations, or the destination changes.

## Common Traps

### Classifying From The Item Name Instead Of Technical Parameters

ECCNs turn on specific thresholds such as speed, accuracy, power, or composition. A name-based classification is unreliable.

### Defaulting To EAR99 Without Checking The CCL

EAR99 is the residual for items not specifically described. Concluding EAR99 without reviewing the CCL skips the analysis.

### Treating EAR99 As Uncontrolled

EAR99 items remain subject to the EAR, including end-use, end-user, entity list, and embargo controls. EAR99 is not a free pass.

### Ignoring Technology And Software Classification

Technology and software have their own ECCNs. Classifying only the hardware misses controlled technology releases.

### Reading Only The Destination, Not The Reason-For-Control Intersection

A license depends on the reason for control applied to the destination via the country chart. Destination alone does not determine the requirement.

### Assuming A License Exception Applies Universally

Exceptions are item-, destination-, and end-use-specific with conditions. Misuse voids the authorization and can create liability.

### Overlooking Entity List And End-Use Controls

A correct ECCN does not resolve Entity List, Military End-User, or end-use prohibitions. These operate independently.

### Failing To Revisit Classification On Change

Item modifications, regulatory updates, and new destinations can change the classification. A static classification becomes stale.

## Self-Check

- Has jurisdiction been confirmed as EAR rather than ITAR or another agency, including foreign direct product and de minimis analysis for foreign-made items?
- Is the ECCN assigned by reading the technical parameters of the item against the ECCN text, not by name or vendor label?
- If EAR99 is concluded, has the Commerce Control List been reviewed to confirm no ECCN specifically describes the item?
- Are technology and software classified separately from hardware, with source code distinguished from object code?
- Has the reason-for-control and country chart intersection been read to determine the license requirement for the specific destination?
- Have license exceptions been evaluated for the specific item, destination, end-use, and end-user, with eligibility and conditions documented?
- Are Entity List, Unverified List, Military End-User List, Denied Persons List, and end-use rules applied independently of the ECCN?
- Are comprehensive embargo prohibitions respected regardless of classification?
- Is the classification documented with ECCN, technical parameters, reason for control, country chart intersection, license determination, exception basis, and date?
- Is the classification revisited when the item, regulations, or destination changes?
