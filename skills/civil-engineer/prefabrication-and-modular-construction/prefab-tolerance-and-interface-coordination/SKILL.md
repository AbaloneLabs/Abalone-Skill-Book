---
name: prefab-tolerance-and-interface-coordination.md
description: Use when the agent is coordinating tolerances between factory-built modules or components and site-built elements, defining the manufacturing and erection tolerances, designing the interface details that absorb tolerance accumulation, reconciling the prefab grid with the site foundation survey, or resolving fit conflicts between modules and core structures. Applies during design coordination and before erection, while ensuring that every interface can absorb the expected variation.
---

# Prefab Tolerance And Interface Coordination

Tolerance is the controlled variation that allows a building to be assembled from parts made in different places by different methods, and in prefabricated construction it is the single most consequential coordination discipline. A factory-built module is precise, but a site-built foundation is not; the difference must be absorbed at the interface, and if the interface is not designed to absorb it, the modules do not fit, the connections do not close, and the finishes do not align. Tolerance accumulation, where small variations in each module compound up the building, is the failure mode that distinguishes prefab from site-built construction, and it is invisible until erection.

The harm this skill prevents is a foundation poured out of tolerance that no interface detail can absorb, a module-to-module joint that cannot close because the accumulated deviation exceeds the connection capacity, a prefab grid that does not reconcile with the site survey, or a fit conflict between the modules and a site-built core that forces field rework at the highest cost point of the project. Engineers tend to specify tight factory tolerances and assume the site will match, but the site-built elements (foundations, cores, podiums) are always less precise than the factory, and the interface must be designed for the realistic site tolerance, not the ideal.

## Core Rules

### Specify Separate Manufacturing And Erection Tolerances Explicitly

Prefabricated construction has two distinct tolerance regimes that must be specified separately and explicitly. The manufacturing tolerance is the variation allowed in the factory-built module (tight, controlled, typically millimeters). The erection tolerance is the variation allowed in placing the module on site (looser, affected by foundation accuracy, crane precision, and survey control). The interface tolerance is the variation the connection detail can absorb, and it must exceed the sum of the manufacturing and erection variation.

Specify each tolerance numerically, based on the relevant standard (PCI for precast, the modular building standard, the steel erection tolerance standard), and do not leave tolerances to assumption. A project with no specified tolerance defaults to whatever the manufacturer and the installer each assume, and the assumptions never match.

### Design The Interface Detail To Absorb The Realistic Site Tolerance

The interface detail, the connection between the module and the foundation or between adjacent modules, must absorb the realistic site tolerance, not the ideal. Site-built foundations vary in location, elevation, and levelness by amounts that exceed the module's factory precision, and the interface must close the gap. Shims, grout pads, oversize holes, adjustable brackets, and sealant joints are the standard absorption details.

Design the interface for the worst-case tolerance stack: the foundation at the extreme of its tolerance, the module at the extreme of its tolerance, and the erection at the extreme of its precision. If the interface cannot close the worst-case gap, redesign the interface (more shimming capacity, larger oversize holes, a wider sealant joint) or tighten the site tolerance (more rigorous survey and placement control). Do not assume the average case, because the average case does not occur at every interface.

### Reconcile The Prefab Grid With The Site Foundation Survey Before Erection

The prefab grid, the theoretical position of every module, must be reconciled with the as-built foundation survey before any module is erected. The foundation is poured to a site tolerance that may deviate from the prefab grid, and the deviation must be detected and corrected (or absorbed) before the modules arrive. Discovering a foundation deviation at erection, when the modules are already on site, forces field rework at maximum cost.

Survey the foundation after placement, compare the as-built positions to the prefab grid, and identify every deviation. Where the deviation exceeds the interface absorption capacity, correct the foundation (re-shim, re-grout, re-pour a pad) before erection. Establish the survey control points that will be used to position each module, and verify them against the prefab grid.

### Control Tolerance Accumulation Up The Building

Tolerance accumulates: each module placed with a small deviation adds to the deviation of the modules below, and over a multi-story building the accumulated deviation can exceed the interface capacity at the upper levels. A building where each story is placed 5 millimeters off accumulates 25 millimeters over five stories, which may exceed the connection tolerance at the top.

Control accumulation by establishing reference control at intervals up the building: survey every few stories, compare the as-built position to the theoretical, and correct the deviation (re-shim the next module to pull back toward grid) before it compounds. Do not assume that small per-module deviations are negligible, because they compound. Specify the accumulation control strategy in the erection procedure.

### Coordinate The Interface Between Prefab And Site-Built Elements

Most prefab buildings combine factory-built modules with site-built elements (a concrete core, a steel podium, a site-built lobby), and the interface between them is the highest-risk fit point. The site-built element is less precise than the module, and the connection must absorb the difference. A module designed to bear on a site-built core that is out of plumb, or to connect to a site-built beam that is out of position, does not fit.

Coordinate the interface between prefab and site-built elements explicitly: define the tolerance of each, design the connection to absorb the difference, and verify the as-built site-built element before the module is connected. Where the site-built element is critical (a load-bearing core), survey it and reconcile to the prefab grid before module erection.

### Mock Up And Test Critical Interfaces Before Production

The critical interfaces, the connections that occur most frequently or that carry the highest load, should be mocked up and tested before production. A mock-up reveals whether the interface detail actually absorbs the tolerance, whether the connection is buildable by the erection crew, and whether the finishes align. Discovering an interface failure in the field, after the modules are produced, is far more expensive than discovering it in a mock-up.

Build a mock-up of the most critical interface (the module-to-foundation connection, the module-to-module connection), test it at the worst-case tolerance, and revise the detail based on the result. Document the tested detail as the production standard.

### Establish The Survey And Verification Protocol For Erection

Erection requires a survey protocol that verifies the position of every module as it is set, detects deviation immediately, and corrects it before the next module is placed. Surveying after the fact, when the deviation has accumulated and the connections are closed, is too late. The survey must be real-time, keyed to the control points, and tied to the tolerance limits.

Establish the survey protocol: the control points, the measurement at each module (location, elevation, plumbness), the tolerance limits, and the correction procedure when a module is out of tolerance. Train the erection crew on the protocol and enforce it at every module.

## Common Traps

### Assuming The Site Will Match The Factory Precision

Site-built foundations and cores are always less precise than factory modules, and the interface must absorb the difference. The trap is designing the interface for factory precision and discovering the site deviation at erection.

### No Specified Tolerance, Defaulting To Conflicting Assumptions

A project with no specified manufacturing or erection tolerance defaults to the manufacturer's and the installer's assumptions, which never match. The trap is treating tolerance as implicit.

### Interface Designed For The Average Case, Not The Worst Case

An interface that absorbs the average tolerance but not the worst-case stack fails at the extreme interfaces. The trap is designing for the mean; the requirement is designing for the stack.

### Foundation Deviation Discovered At Erection

A foundation out of tolerance, discovered when the modules are on site, forces field rework at maximum cost. The trap is not surveying and reconciling the foundation before delivery.

### Uncontrolled Tolerance Accumulation

Small per-module deviations that compound up the building exceed the interface capacity at the upper levels. The trap is assuming small deviations are negligible; they accumulate.

### Prefab-To-Site-Built Interface Not Coordinated

A module that does not fit the site-built core or podium because the interface tolerance was not coordinated. The trap is treating the site-built elements as independently toleranced.

### Interface Failure Discovered In The Field

A critical interface that does not absorb the tolerance, discovered after production, forces field rework. The trap is not mocking up and testing before production.

## Self-Check

- Are manufacturing, erection, and interface tolerances specified separately and numerically, based on the relevant standard?
- Is the interface detail designed to absorb the worst-case tolerance stack, not the average case?
- Has the as-built foundation been surveyed and reconciled to the prefab grid before module delivery?
- Is tolerance accumulation controlled by reference survey and correction at intervals up the building?
- Is the interface between prefab modules and site-built elements (cores, podiums) explicitly coordinated and verified?
- Have the critical interfaces been mocked up and tested at worst-case tolerance before production?
- Is a real-time survey protocol established for erection, with control points, tolerance limits, and correction procedure?
- Does every interface have a documented absorption detail (shims, grout, oversize holes, sealant) that exceeds the expected variation?
