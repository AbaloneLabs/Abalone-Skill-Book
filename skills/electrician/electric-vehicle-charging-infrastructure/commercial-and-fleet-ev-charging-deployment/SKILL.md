---
name: commercial-and-fleet-ev-charging-deployment.md
description: Use when the agent is deploying commercial or fleet EV charging, installing networked stations with billing and backend systems, wiring multi-unit dwelling (MUD) shared charging, coordinating with the utility for high-power DC fast charging, or designing make-ready infrastructure for commercial EVSE sites.
---

# Commercial and Fleet EV Charging Deployment

A commercial EV charging site is not a row of residential chargers scaled up. It is a networked, metered, utility-coordinated power installation where the electrical work is inseparable from communications, billing, access control, and a utility relationship that may require a new transformer and a demand-charge tariff. The electrician who treats it as branch circuits and boxes will deliver power to the pedestals but leave the site non-functional, because the chargers cannot authenticate users, cannot bill, cannot report faults, and may not even be permitted to energize without the network backend online. The judgment problem is that commercial and fleet deployment spans electrical, networking, contractual, and utility domains, and the trades optimize each in isolation while the integration points — the make-ready boundary, the network drop, the metering scheme, the utility service upgrade — are where projects stall or fail. An electrician who understands only the conduit run will install infrastructure that the site host cannot operate and that the charger vendor cannot commission. This skill covers the decisions that determine whether a commercial EV charging deployment is buildable, operable, and coordinated with every party it depends on.

## Core Rules

### Define the Make-Ready Boundary Before Pouring Concrete or Pulling Wire

Commercial EV charging splits cleanly into "make-ready" infrastructure (the utility service, transformer, switchgear, feeders, conduit, and pull boxes up to the charger pedestal) and the charger itself (the EVSE unit, its internal electronics, cable, and connector). The boundary between them is a contractual and practical line: the site host or a developer usually funds and owns the make-ready, while the charger vendor supplies and maintains the EVSE. Confusion about who provides what — who terminates the feeder at the pedestal, who supplies the concrete foundation, who runs the low-voltage network drop — causes delays, change orders, and incompatible interfaces. The electrician must establish the make-ready scope from the site plan and the vendor's installation requirements before construction, confirming the conduit size, the stub-up location, the feeder termination point, and the low-voltage provisions at each pedestal.

The trap is building the make-ready to a generic assumption and discovering at charger delivery that the stub-ups are in the wrong place, the conduit is too small for the charger feeders, or the network drop was never planned. The defense is to obtain the charger vendor's installation drawings early, to coordinate the make-ready scope against them, and to oversize conduit deliberately to accommodate future charger models and additional circuits.

### Engage the Utility Early for Any DC Fast Charging or Large Multi-Charger Site

DC fast charging at 50 kW and above, and any site with enough Level 2 chargers to materially affect demand, requires utility coordination that begins long before construction. The utility must determine whether the existing distribution can support the new load, whether a new transformer and service are needed, and what the tariff structure (particularly demand charges) will be. For high-power DCFC (150–350 kW), a dedicated transformer and often medium-voltage service are required, and the utility's engineering and construction timeline can be six to twelve months. The site's electrical design depends on the utility's service offering: the transformer size and location, the available voltage and phases, and the fault current at the service point all flow from the utility and determine the switchgear, conductor, and charger selection.

The trap is designing the site electrical and then approaching the utility, only to learn the service must be upgraded, the transformer placed elsewhere, or the project delayed by a year. The defense is to submit the utility service request with the estimated load at the start of the project, to design around the utility's offered service, and to treat the utility timeline as the project's critical path.

### Plan the Network and Communications Infrastructure as Part of the Electrical Design

Networked commercial chargers require a reliable data connection to a backend for authentication, billing, session tracking, and remote diagnostics. This connection is typically Ethernet to each charger (or a gateway), backhauled via cellular or a building network, and it is part of the electrical installation. The electrician must run conduit and cable for the data network alongside the power feeders, terminate it at each pedestal, and ensure the gateway or cellular modem has power and adequate signal. A charger without a network connection cannot authenticate users, cannot process payment, and may default to a disabled or free-charging state that defeats the site's revenue model. OCPP (Open Charge Point Protocol) is the common communication standard, and the chargers, the network management system, and any local controller must be compatible.

The trap is treating the network as the IT vendor's problem and discovering at commissioning that no conduit was run to the pedestals, forcing a costly retrofit through finished site work. The defense is to include data conduit and drops in the make-ready scope, to confirm the backhaul method (cellular vs building network) during design, and to test connectivity before the site is paved over.

### Design the Metering and Billing Scheme to Match the Site's Ownership and Access Model

Commercial charging sites bill users for energy, and the metering scheme must match how the site is owned and accessed. Options include charger-internal submetering (the networked EVSE measures each session and bills via the backend), utility-owned separate meters (each charger or group on its own utility meter), and master-metered with submetering (the site host pays the utility and re-bills users via the charger network). Each has regulatory, accuracy, and cost implications: utility-owned meters are revenue-grade and legally sufficient for resale in most jurisdictions, while charger submeters may need to meet accuracy standards and may not be permitted for energy resale in some locales. For multi-unit dwellings, the scheme must also address how residents are assigned to chargers, how guests are handled, and how common-area charging is billed to the association.

The trap is assuming the charger's internal metering is sufficient for billing without checking local rules on energy resale and meter accuracy. The defense is to determine the billing and ownership model early, to verify whether charger submetering meets the jurisdiction's resale requirements, and to specify utility-owned meters where revenue-grade metering is required.

### Address Multi-Unit Dwelling Constraints on Access, Assignment, and Common Infrastructure

Multi-unit dwelling (MUD) charging is the hardest residential-adjacent deployment because it combines shared infrastructure, individual ownership, deeded parking, and association governance. The electrical challenges — limited service capacity, long feeder runs to distant parking, load management across many units — are compounded by access and equity questions: which parking spaces get chargers, how are they assigned, who pays for the common infrastructure, and how are non-resident or guest vehicles handled. The electrical design must support a fair and expandable scheme, typically starting with a shared feeder and an ALMS to a common panel, with branch circuits or dedicated circuits to assigned spaces. The service capacity and load management must be planned for full build-out from the beginning, even if chargers are installed in phases, because retrofitting conduit and feeders in a finished garage is far more expensive than oversizing during initial construction.

The trap is installing chargers for the first few residents who request them without planning the common infrastructure for full build-out, then facing an expensive retrofit when the next wave of residents wants charging. The defense is to design the make-ready for the full parking count from the start, to use a shared feeder with load management, and to phase the charger installation on top of conduit and capacity that is already in place.

### Manage Utility Demand Charges Through Staged Charging and Load Control

Commercial electricity tariffs include demand charges based on the peak power drawn in a billing period, and for a charging site this peak is set by the maximum simultaneous charger output. A fleet that fast-charges ten trucks at once can set a demand peak that dominates the entire bill, even if the energy consumed is modest. Demand charges can exceed the energy charges on a commercial EV site, making load management an economic necessity, not just a capacity tool. Staging charging (charging vehicles sequentially rather than simultaneously), capping the site's total power via an ALMS, and shifting charging to off-peak hours all reduce the demand peak and the bill. The electrician should understand the site's tariff and design the load management to limit demand, because the site host's operating cost depends on it.

The trap is designing the site for maximum simultaneous charging and handing the owner a demand-charge bill that makes the site uneconomic to operate. The defense is to understand the tariff structure, to design the ALMS to cap site demand, and to discuss staging and off-peak charging with the fleet operator or site host.

## Common Traps

### Building Make-Ready Without the Charger Vendor's Installation Drawings

Conduit and pull boxes are installed based on a generic site plan, and when the chargers arrive, the stub-ups do not match the pedestal footprint, the conduit is too small for the feeder, and the network drop is missing. The mechanism of the trap is that make-ready must interface to a specific charger model, and generic assumptions about pedestal dimensions, conduit entry, and termination points are usually wrong. The false signal is that the site plan "looks right," which reflects the layout but not the hardware interface. The harm is demolished concrete, cut and re-pulled feeders, and a delayed commissioning. The defense is to obtain and build to the charger vendor's installation drawings before any site work.

### Approaching the Utility After Designing the Site

A DCFC site is designed for four 150 kW chargers, and the utility is contacted to connect the service, only to report that the existing distribution cannot support 600 kW and a new transformer and possibly a line extension are required, with a year-long lead time. The mechanism of the trap is that high-power charging loads are large enough to require dedicated utility infrastructure, and the utility's capacity and timeline govern what is buildable. The false signal is that the site has a service, which is true but insufficient for the load. The harm is a redesigned and delayed project. The defense is to submit the utility service request at project initiation and to design around the utility's offering.

### Omitting the Network Drop and Discovering Chargers Cannot Authenticate or Bill

The power feeders are run to every pedestal, but no data conduit was included, and at commissioning the chargers cannot reach the backend, cannot authenticate users, and cannot process payment. The mechanism of the trap is that networked chargers are useless without connectivity, and the data infrastructure is easy to overlook because it is not a power circuit. The false signal is that the chargers have power and light up, which proves the electrical work but not the operational readiness. The harm is a site that cannot generate revenue until a network retrofit is completed through finished hardscape. The defense is to include data conduit and drops in the make-ready scope and to test backhaul connectivity before site completion.

### Assuming Charger Submetering Is Legally Sufficient for Energy Resale

A commercial site relies on the chargers' internal meters to bill users for energy, but the jurisdiction requires revenue-grade utility metering for energy resale, and the charger submeters do not meet the accuracy or certification standard. The mechanism of the trap is that energy resale is regulated, and not all submetering is permitted to be the basis for billing. The false signal is that the charger reports kilowatt-hours, which proves measurement but not legal sufficiency for resale. The harm is a billing scheme that cannot be enforced and potential regulatory exposure. The defense is to verify the local rules on energy resale and to specify utility-owned meters where required.

### Phasing MUD Charging Without Planning the Common Infrastructure for Full Build-Out

An apartment association installs chargers for the first six residents who request them, each on a dedicated home-run circuit, with no shared feeder or load management. When twenty more residents request charging, the service cannot support them and there is no conduit infrastructure to extend. The mechanism of the trap is that incremental installation without a master plan forces a full retrofit for each expansion phase, and the cost compounds. The false signal is that the first phase works, which proves the concept but not the scalability. The harm is an expensive and disruptive retrofit and residents denied charging. The defense is to design the make-ready for full parking-count build-out at the start, with shared feeders and an ALMS, and to phase only the charger units on top of the existing infrastructure.

### Ignoring Demand Charges and Designing for Maximum Simultaneous Charging

A fleet depot installs DC fast chargers sized for simultaneous full-rate charging of every truck, and the first utility bill includes a demand charge that dwarfs the energy charge, making the depot's operating cost unsustainable. The mechanism of the trap is that commercial tariffs price peak demand, and simultaneous fast charging sets a high peak regardless of total energy. The false signal is that the chargers are correctly sized for the fleet, which is true for power but ignores the tariff consequence. The harm is an operating cost that threatens the fleet's economics. The defense is to understand the tariff, to design the ALMS to cap site demand, and to stage charging to minimize peak draw.

### Overlooking the Edge Case or Exception

The standard networked Level 2 commercial site is handled well, but the case of a mixed DCFC and Level 2 site on a constrained service, or a MUD with a master-metered tariff and submetering restrictions, is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I define the make-ready boundary against the charger vendor's installation drawings before construction, confirming stub-up locations, conduit size, feeder termination points, and low-voltage provisions at each pedestal?
- For any DCFC or large multi-charger site, did I submit the utility service request at project initiation and design around the utility's offered service, transformer, and timeline?
- Did I include data conduit and network drops in the make-ready scope, confirm the backhaul method (cellular vs building network), and verify OCPP compatibility between the chargers and the network management system?
- Did I determine the metering and billing model early, and verify whether charger submetering meets the jurisdiction's energy-resale requirements or whether utility-owned meters are required?
- For multi-unit dwellings, did I design the make-ready for full parking-count build-out with shared feeders and an ALMS, so that phasing adds chargers without retrofitting infrastructure?
- Did I understand the site's commercial tariff and design the load management to cap demand and stage charging, rather than sizing for maximum simultaneous output that triggers unmanageable demand charges?
- Did I oversize conduit during initial construction to accommodate future charger models, additional circuits, and capacity expansion?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
