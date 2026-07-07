---
name: facilities-management-handover-and-asset-information.md
description: Use when the architect is preparing for building handover to operations, assembling asset information models (AIM), defining COBie deliverables, commissioning closeout data, O&M manual requirements, or coordinating as-built information transfer to facilities management teams.
---

# Facilities Management Handover and Asset Information

The handover from construction to operations is where a building's design intent either becomes a maintainable reality or a chronic source of failure. Architects often treat handover as a contractual closing step rather than an information transfer that determines decades of operational performance. The facilities management (FM) team inherits the consequences of incomplete, inaccurate, or poorly structured asset data, and the gaps surface only when maintenance fails, warranties cannot be enforced, or systems cannot be serviced.

This skill covers the judgment the architect needs when defining what information must be handed over, in what structure, at what level of detail, and verified against what evidence. The risk is not only missing data but data that is technically present yet unusable: unverified COBie exports, O&M manuals that reference uninstalled equipment, warranty start dates that do not align with substantial completion, or BIM models whose asset IDs do not match the FM system.

## Core Rules

### Define handover deliverables at design stage, not at closeout

Asset information requirements must be defined in the employer's information requirements (EIR) and contract documents before construction begins, not negotiated at substantial completion. Decide early:

- Which assets are "in scope" for FM data (typically maintainable assets above a cost or criticality threshold, not every fixture).
- What attributes each asset requires (manufacturer, model, serial number, install date, warranty, maintenance frequency, criticality, replacement cost).
- The required data exchange format (COBie, proprietary FM schema, or native BIM).
- The level of information need (LOIN) per asset class, not a single blanket LOD.

A common failure is to request "all asset data" with no threshold, which produces an unmanageable dataset that no FM team will ever use. Define the asset register scope by criticality and maintainability.

### Align the BIM model, the asset register, and the FM system

The asset identifier is the join key between the physical asset, the BIM model, the COBie export, the O&M manual, and the CMMS (computerized maintenance management system). If these identifiers diverge, the handover data is unusable. Enforce a single asset naming convention across all deliverables and require that every maintainable asset in the BIM model carries the same ID that will appear in the FM system.

### Verify, do not accept, handover data

Contractor-supplied COBie and O&M data are frequently copy-pasted from submittals, unverified against installed conditions. The architect's role is to require evidence of verification: installed equipment photographs matching tagged assets, serial numbers captured at installation, and manufacturer documentation that matches the as-installed model (not the specified model). Require a data quality sign-off, not merely a data delivery.

### Coordinate warranty and maintenance data with contractual milestones

Warranty start dates, maintenance requirements, and service contracts must align with the certificate of substantial completion and the commissioning report. A warranty that starts at contract execution rather than substantial completion can expire months before the owner expects. A preventive maintenance schedule that assumes a different operating profile than the commissioned system will invalidate warranties.

### Structure O&M information for retrieval, not for completeness

An O&M manual that contains every document but cannot answer "how do I service asset X-117 in under five minutes" has failed its purpose. Require indexing by asset ID, separation of safety-critical procedures, and quick-reference one-pagers for emergency isolation and shutdown. The FM team operates under time pressure; the structure of information matters as much as its presence.

## Common Traps

### Treating COBie delivery as a checkbox

Exporting COBie from a BIM authoring tool at handover produces a file, not verified asset data. COBie generated without per-asset verification typically contains placeholder values, missing serial numbers, and maintenance tasks copied from templates. The trap is believing that a delivered COBie file satisfies the handover obligation; it satisfies the deliverable but not the intent.

### Confusing specified data with installed data

Submittals and specifications describe what was proposed. Installed equipment may differ due to substitutions, field changes, or value engineering. Handover data built from submittals rather than verified installed conditions will mismatch the physical building and undermine the FM system on day one.

### Ignoring asset criticality in scoping

Capturing every door, light fixture, and diffuser creates a dataset so large that critical assets are buried. The trap is optimizing for completeness over usability. Scope the asset register to maintainable, critical, and high-value assets, and document the exclusion logic so the FM team understands what is and is not tracked.

### Misaligning the BIM-to-FM data mapping

The BIM model is organized by building elements; the FM system is organized by maintainable assets. These are not the same. A wall is a building element; the access panel in it is a maintainable asset. A direct dump of BIM elements into an FM system creates noise. Define the mapping explicitly and test it on a sample before full handover.

### Forgetting soft-landings and post-handover support

Handover is not the end of the architect's responsibility where data quality is concerned. Soft landings and a defined post-occupancy support period allow the FM team to surface data errors while the contractor is still engaged. Omitting this leaves the owner to discover and absorb data defects alone.

## Self-Check

- Did I define the asset information requirements and exchange format in the contract documents before construction, not at closeout?
- Is there a single, enforced asset naming convention that links the BIM model, COBie export, O&M manual, and FM system?
- Did I scope the asset register by criticality and maintainability, and document the exclusion logic for non-tracked elements?
- Is the handover data verified against installed conditions (serial numbers, photographs, as-installed documentation), not merely copied from submittals?
- Do warranty start dates, maintenance schedules, and service contracts align with the certificate of substantial completion and the commissioning report?
- Is the O&M information structured for rapid retrieval by asset ID, including emergency isolation procedures?
- Have I included a soft-landings or post-handover support period to allow the FM team to surface data defects while the contractor remains engaged?
- Have I tested the BIM-to-FM data mapping on a representative sample before accepting the full dataset?
