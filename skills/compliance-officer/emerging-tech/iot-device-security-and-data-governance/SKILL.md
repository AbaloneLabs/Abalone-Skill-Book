---
name: iot-device-security-and-data-governance.md
description: Use when the agent is developing, deploying, or procuring connected IoT devices, designing device security and update lifecycle, managing the data collected by sensors and connected products, or assessing privacy and safety risk across an IoT product ecosystem.
---

# IoT Device Security and Data Governance

Internet-of-things products combine physical safety, cybersecurity, privacy, and data-governance risk in a single device that may operate for years in homes, factories, vehicles, or public spaces. Agents often treat IoT as a conventional software product and overlook that the device collects real-world physical data, may control physical systems, often cannot be easily updated, and frequently outlives the company's attention to it. The judgment problem is designing security and privacy into the device from inception, governing the data it collects across its lifecycle, and planning for the device's long operational life including the end of vendor support.

This skill applies to product, engineering, compliance, privacy, and security functions developing or procuring connected devices for consumer, industrial, healthcare, automotive, or smart-city use. IoT security, safety, and privacy requirements differ by jurisdiction, sector, and device class, and are tightening through dedicated IoT cybersecurity regimes. Verify the applicable security, safety, and data-protection standards and consult specialist counsel for safety-critical or regulated device classes.

## Core Rules

### Design Security for the Device Lifecycle, Not Just for Launch

An IoT device's risk profile is defined by its entire operational life, which can span many years, and security designed only for launch degrades as vulnerabilities are discovered and adversaries evolve. Build in a vulnerability management and disclosure process, a secure update mechanism that can patch vulnerabilities over the support life, cryptographic protection of firmware and communications, and secure boot to prevent malicious firmware replacement. Define the support life explicitly, because devices that stop receiving security updates become liabilities for as long as they remain in use. Plan for the end of support, including how users will be notified and what happens to devices that can no longer be secured, because unsupported connected devices in the field are a persistent attack surface and safety risk.

### Conduct the Physical-Safety Risk Assessment for Connected Control

When an IoT device controls a physical system, a thermostat, a vehicle component, an industrial actuator, a medical delivery pump, a security lock, a cybersecurity failure becomes a physical-safety failure. Conduct a safety-security risk assessment that traces how a compromise, malfunction, or loss of connectivity could cause physical harm, and apply the safety hierarchy of controls to the connected functionality. This includes failsafe behavior when connectivity is lost, redundancy for safety-critical functions, and isolation between safety-critical control and internet-facing interfaces. Treating the device as a conventional software product misses the class of harm that arises when code controls the physical world.

### Govern the Data the Device Collects Across Its Lifecycle

IoT devices frequently collect continuous, granular data about the physical environment and the people in it, including location, presence, activity, biometric, audio, and video data. This data is often more sensitive than the user realizes and is collected in spaces where expectation of privacy is high, such as the home. Apply data minimization at the device level, collecting only what is needed for the function; identify when the collected data constitutes sensitive or special-category data triggering heightened obligations; provide meaningful notice and choice given the device's often limited interface; and govern retention, secondary use, and sharing of the device data. The data-governance design must account for the fact that the device may collect data about non-users, such as visitors or household members, who have not consented to anything.

### Manage the Supply Chain and Component Risk

An IoT device's security depends on its components, firmware libraries, and manufacturing process, not only on the company's own code. Compromised components, vulnerable third-party libraries, and tampered manufacturing can introduce backdoors or vulnerabilities that the company cannot detect through its own testing. Apply supply-chain security practices including component provenance, software-bill-of-materials tracking for firmware dependencies, vetting of contract manufacturers, and controls over who can load firmware during manufacturing. The supply chain is a recognized attack vector for IoT, and a company that secures its own code but not its components remains exposed.

### Address Interoperability, Platform Dependency, and Lock-In Risk

IoT devices often depend on cloud services, companion apps, and platform ecosystems that the device manufacturer may not control. If the cloud service is discontinued, the companion app is withdrawn from an app store, or the platform changes its terms, the device can lose functionality or become inoperable. This creates consumer-protection, warranty, and e-waste risk. Design for resilience where feasible, communicate the dependencies and their risks to purchasers, and plan for the scenario in which a dependency is withdrawn. The device that bricks when a server shuts down is a product-liability and reputation risk.

### Plan for Decommissioning and Data Deletion

When an IoT device reaches end of life, is returned, resold, or discarded, the data it holds and its access to cloud accounts must be managed. Build a secure factory-reset and data-deletion capability that a user can invoke, ensure that returned devices are wiped before resale, and revoke the device's cloud credentials so that a discarded device cannot continue to access the user's account or data. Devices that retain user data and credentials after disposal create privacy and security risk for the original owner and for any subsequent owner.

## Common Traps

### Security Designed Only for Launch

Devices that stop receiving updates become liabilities for their entire field life. Define the support life, build a secure update mechanism, and plan for end of support.

### Ignoring Physical-Safety Consequences of Cyber Failures

When a device controls a physical system, a cybersecurity failure can cause physical harm. Conduct a safety-security assessment and apply failsafe and isolation controls.

### Over-Collecting Sensitive Data Without Minimization

Continuous granular data, especially in private spaces, is often sensitive and may capture non-users. Minimize at the device level and govern retention and secondary use.

### Unmanaged Supply-Chain and Component Risk

Compromised components and vulnerable libraries introduce backdoors that internal testing cannot detect. Track the software bill of materials and vet the manufacturing chain.

### Device Bricking When a Dependency Is Withdrawn

Dependence on cloud services and platforms that can be discontinued creates functionality, consumer-protection, and e-waste risk. Design for resilience and communicate dependencies.

### No Secure Decommissioning

Devices that retain user data and credentials after disposal create privacy and security risk. Build secure reset and credential revocation into the device.

## Self-Check

- Did I design security for the device's full operational life, including vulnerability management, secure updates, a defined support life, and end-of-support planning?
- For devices that control physical systems, did I conduct a safety-security risk assessment with failsafe behavior and isolation of safety-critical functions?
- Did I apply data minimization at the device level, identify sensitive and non-user data, provide meaningful notice and choice, and govern retention and sharing?
- Did I manage supply-chain and component risk through provenance, software-bill-of-materials tracking, and manufacturing vetting?
- Did I design for resilience against cloud, app, and platform dependency withdrawal, and communicate the dependencies and risks to purchasers?
- Did I build secure decommissioning with factory reset, data deletion, and credential revocation for returned, resold, or discarded devices?
- Did I confirm the applicable IoT cybersecurity, safety, and data-protection standards with specialist counsel for safety-critical or regulated device classes?
