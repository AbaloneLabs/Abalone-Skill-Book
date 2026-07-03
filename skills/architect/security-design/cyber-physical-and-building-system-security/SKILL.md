---
name: cyber_physical_and_building_system_security.md
description: Use when the agent is designing the convergence of physical and cyber security, integrating building automation and OT networks with IT, planning secure control rooms and server rooms, coordinating access control and video with the network, or addressing the cyber-physical risks of smart and connected building systems.
---

# Cyber Physical And Building System Security

Cyber-physical security is the discipline of protecting building systems — building automation, access control, video surveillance, life safety, elevators, and energy management — from cyber threats that could compromise the physical building and its occupants. As buildings become more connected, the operational technology networks that run them converge with the information technology networks that carry data, and the attack surface expands from the physical perimeter to every connected device. Agents often treat cyber security as an IT problem and miss that the building's physical design — the location of control rooms, the segregation of networks, the physical protection of network infrastructure, the design of the building automation architecture — determines the cyber-physical risk, and that an architect who does not understand this convergence designs vulnerable buildings. The architect owns the physical and architectural dimension of cyber-physical security, in coordination with IT, OT, and security consultants. The goal is a building whose connected systems are protected at the network, the device, and the physical levels, and whose cyber-physical risks are designed out rather than patched in.

## Core Rules

### Understand The Building's Cyber-Physical Attack Surface

Begin by mapping the building's connected systems — building automation, lighting, HVAC, access control, video, life safety, elevators, energy management, and any smart building platforms — and the networks that connect them, because each connected device is a potential entry point for a cyber attack that could affect the physical building. Understand the convergence of operational technology, which runs the building, with information technology, which carries data, and the risks that convergence creates. The attack surface map is the basis for the cyber-physical design, and it must be developed with the IT, OT, and security consultants. The architect must understand the attack surface well enough to design the physical and architectural protections, even though the cyber measures are consultant scope.

### Segregate OT And IT Networks Architecturally

The segregation of operational technology networks from information technology networks is the foundational cyber-physical control, because it limits the ability of an attacker who breaches one network to reach the other, and it has physical and architectural implications. Plan the network architecture with the IT and OT consultants, with firewalls, demilitarized zones, and physical separation where required, and locate the network infrastructure — server rooms, telecom rooms, distribution — to support the segregation. Provide the physical pathways, the secure rooms, and the access control that the network segregation requires, because a logical segregation that is not supported physically is easily defeated. The architect must design the physical infrastructure that the network security depends on.

### Design Secure Control Rooms, Server Rooms, And Network Infrastructure

The control room, the server room, and the network distribution rooms are the physical heart of the building's cyber-physical systems, and their location, construction, and access control determine the building's vulnerability to physical compromise. Locate these rooms away from public access, with hardened construction, dedicated access control, environmental controls, and redundant power, and design them for the operations and security staff who use them. Coordinate the rooms with the network architecture, the building automation, and the security operations, because they house the systems that run the building. A control room located in a public area, with standard construction and uncontrolled access, is a cyber-physical vulnerability that no network measure can compensate for. The architect must design these rooms as secure physical assets.

### Coordinate Access Control And Video With The Network Architecture

Access control and video surveillance are cyber-physical systems: they are physical security measures that depend on networked devices, and their compromise could disable physical security or expose sensitive data. Coordinate the access control and video systems with the network architecture, with the devices on appropriately segregated networks, the data stored securely, and the systems protected from tampering and spoofing. Design the physical infrastructure — pathways, power, network — that these systems require, and locate the devices and the headend equipment in secure locations. The architect must treat access control and video as cyber-physical systems, not as standalone hardware, and coordinate them with the network and security consultants.

### Address The Cyber Risks Of Smart Building And IoT Systems

Smart building platforms, IoT devices, and connected equipment expand the building's capabilities but also its attack surface, because each connected device is a potential vulnerability, and many devices have weak default security. Specify devices and systems with cyber security appropriate to their risk, require vendors to provide security documentation, and coordinate the integration of smart systems with the network architecture and the cyber security program. Avoid the temptation to specify connected devices for convenience without considering their cyber risk, because a building full of poorly secured IoT devices is a soft target. The architect must specify connected systems with cyber security in mind, in coordination with the IT and OT consultants, and must challenge devices that introduce unmanaged risk.

### Plan For Cyber Incident Response And Building Resilience

A cyber attack on building systems could disable HVAC, lighting, access control, or life safety, and the building must be designed to remain safe and operable under such an attack. Plan for manual overrides and local controls that allow critical systems to operate without the network, design the life safety systems to fail safe and to operate independently of compromised networks, and coordinate the incident response plan with the building operations and the cyber security team. The architect must design for the failure mode in which the building's connected systems are compromised, because resilience under cyber attack is a design responsibility, not only an operational one. A building that becomes unsafe when its network is attacked has failed its occupants.

### Integrate Physical And Cyber Security Governance

Cyber-physical security requires coordination across the physical security, IT, OT, and facilities teams, and the building design must support that governance. Provide the spaces, the systems, and the documentation that allow these teams to operate together, with a security operations center or control room that integrates physical and cyber monitoring, with clear ownership of each system, and with the building information modeled and handed over in a form that supports ongoing security management. The architect must design the physical and organizational infrastructure that supports integrated security governance, because the building's cyber-physical security depends on the people and processes that operate it, supported by the design.

## Common Traps

### Treating Cyber Security As Purely An IT Problem

The team delegates cyber security to the IT consultant, who secures the data network, while the building automation, access control, and life safety networks are designed and installed without cyber coordination, leaving the building's physical systems vulnerable. The mechanism is that cyber security feels like an IT scope and the building systems feel like mechanical scope, and the false signal is that the IT network is secured. The harm is that an attacker reaches the building systems through the unsecured OT network, disables HVAC or access control, and the building is compromised despite IT security. Cyber-physical security must be coordinated across IT, OT, and physical security, with the architect integrating the physical and architectural protections.

### Converging Networks Without Segregation

The OT and IT networks are converged onto a single network for simplicity and cost, eliminating the segregation that would contain a breach, so an attacker who reaches one network reaches all the building's systems. The mechanism is that convergence is cheaper and simpler and segregation feels like extra infrastructure, and the false signal is that the network is unified and therefore efficient. The harm is that a single breach compromises the building's physical systems, and the containment that segregation would have provided is lost. OT and IT networks must be segregated, with firewalls and physical separation, supported by the architectural design of the network infrastructure.

### Locating Control And Server Rooms In Insecure Locations

The building automation server, the access control headend, or the security operations center is located in a closet off a public corridor, with standard construction and uncontrolled access, so the systems that run the building are physically vulnerable. The mechanism is that these rooms are treated as utility space and located wherever convenient, and the false signal is that the equipment is installed and so it is secure. The harm is that an attacker with physical access compromises the systems directly, bypassing the network security entirely. Control rooms, server rooms, and network infrastructure must be located in secure, hardened, access-controlled spaces designed as critical assets.

### Specifying Connected Devices Without Cyber Risk Consideration

The team specifies smart lighting, connected thermostats, IoT sensors, and integrated platforms for convenience and sustainability, without evaluating the cyber risk each device introduces, so the building fills with poorly secured devices that expand the attack surface. The mechanism is that connected devices offer attractive features and their cyber risk is invisible, and the false signal is that the devices are specified and will work. The harm is that the devices are compromised, used as entry points to the network, or manipulated to affect the building's operation. Connected devices must be specified with cyber security in mind, with vendor documentation, network segregation, and risk assessment for each device category.

### Neglecting Manual Overrides And Resilience Under Cyber Attack

The building's systems are fully networked, with no manual overrides or local controls, so when the network is compromised the building cannot operate its HVAC, lighting, or access control, and the occupants are endangered. The mechanism is that networked operation is the design and manual fallback feels like redundancy, and the false signal is that the systems work under normal conditions. The harm is that a cyber attack disables the building's systems, the occupants face unsafe conditions, and the building cannot be operated until the network is restored. The design must provide manual overrides and local controls for critical systems, with life safety designed to fail safe and operate independently of compromised networks.

### Fragmenting Security Governance Across Teams

Physical security, IT, OT, and facilities each manage their own systems with no integration, so no one sees the building's security holistically and the gaps between the teams become vulnerabilities. The mechanism is that each team owns its scope and integration feels like organizational overhead, and the false signal is that each system is managed. The harm is that the gaps between the teams — an access control system the IT team does not monitor, a building automation network the facilities team does not secure — become the attack paths. The design must support integrated security governance, with spaces, systems, and documentation that allow the teams to operate together.

## Self-Check

- [ ] Has the building's cyber-physical attack surface been mapped with IT, OT, and security consultants, identifying every connected system and network?
- [ ] Are OT and IT networks segregated, with firewalls and physical separation, supported by the architectural design of the network infrastructure?
- [ ] Are control rooms, server rooms, and network distribution rooms located in secure, hardened, access-controlled spaces designed as critical assets?
- [ ] Are access control and video systems coordinated with the network architecture, on appropriately segregated networks, with data stored securely?
- [ ] Are connected devices and smart systems specified with cyber security in mind, with vendor documentation and risk assessment for each device category?
- [ ] Does the design provide manual overrides and local controls for critical systems, with life safety designed to fail safe and operate independently of compromised networks?
- [ ] Does the design support integrated security governance, with spaces, systems, and documentation that allow physical security, IT, OT, and facilities to operate together?
- [ ] Has the cyber-physical risk been designed out at the architectural level, rather than left to be patched in by consultants after construction?
