---
name: cloud_and_saas_data_governance_compliance.md
description: Use when the agent is migrating data to cloud services, managing SaaS data governance, evaluating cloud provider shared responsibility, ensuring multi-cloud compliance, or addressing data residency, encryption key management, and regulatory compliance for cloud-hosted personal data under GDPR, HIPAA, PCI DSS, FedRAMP, and sector-specific cloud requirements.
---

# Cloud And SaaS Data Governance Compliance

Cloud and SaaS data governance compliance governs how organizations manage personal and regulated data when it is hosted by third-party cloud and SaaS providers. The defining feature is the shared responsibility model—the cloud provider secures the infrastructure while the customer remains responsible for data, configuration, and access—and that this line shifts depending on the service model (IaaS, PaaS, SaaS). The central difficulty is that organizations often misunderstand where their responsibility begins, that multi-cloud and SaaS sprawl create fragmented data governance, and that cloud introduces data residency, key management, and sub-processor complexity that on-premises systems did not have.

Use this skill before advising on cloud migration, SaaS data governance, shared responsibility mapping, or multi-cloud compliance. The goal is to make the agent identify the service model, the responsibility boundary, the data residency requirements, and the key management approach before concluding that cloud-hosted data is compliant.

## Core Rules

### Map The Shared Responsibility Model For Each Service

Responsibility allocation depends on the service model.

Map:

- IaaS: provider secures infrastructure (physical, host, network); customer secures everything above (OS, applications, data);
- PaaS: provider secures infrastructure and platform (OS, runtime); customer secures applications and data;
- SaaS: provider secures infrastructure, platform, and application; customer secures data and configuration (access, identity);
- the specific responsibilities for encryption, patching, access management, backup, and monitoring;
- the documentation of the boundary for each cloud service used;
- the gap between the provider's responsibility and the customer's capability in SaaS.

The shared responsibility model shifts with the service model. In IaaS, the customer has the most responsibility. In SaaS, the provider takes more but the customer retains data, identity, and configuration responsibility. The boundary must be documented for each service. A common failure is assuming the provider handles security that is actually the customer's responsibility (e.g., S3 bucket configuration, database encryption settings).

### Determine Data Residency And Sovereignty Requirements

Data location affects regulatory compliance.

Address:

- the regulatory requirements for where data can be stored and processed;
- data localization mandates (certain data must stay in-country);
- the cloud provider's region and availability zone configuration;
- data replication and backup across regions (may violate residency);
- the provider's sub-processor locations and data flows;
- the effect of government access powers in the storage region;
- the documentation of data location for regulatory demonstration;
- the use of sovereign cloud offerings where available.

Data residency requirements dictate where data can be stored. Some data must stay in-country (data localization). Cloud regions must be selected to meet residency. Replication across regions can violate residency. Sub-processor locations extend the data flow. Government access powers in the storage region affect surveillance risk. Data location must be documented. Sovereign cloud offerings (government clouds, EU sovereign clouds) address specific residency needs.

### Manage Encryption And Key Management

Encryption and key control are critical in cloud environments.

Implement:

- encryption at rest for all sensitive data;
- encryption in transit for all data flows;
- the choice of key management: provider-managed, customer-managed (BYOK), or hold-your-own-key (HYOK);
- the effect of key management on government access (provider-managed keys may be accessible to the provider's government);
- the use of customer-managed keys for sensitive data requiring enhanced protection;
- key rotation and lifecycle management;
- the integration with hardware security modules (HSMs) for high-security requirements;
- the documentation of the encryption and key management approach.

Encryption at rest and in transit is a baseline. Key management determines who can access the data: provider-managed keys are convenient but the provider (and its government) may access them. Customer-managed keys (BYOK) give the customer control. Hold-your-own-key (HYOK) keeps keys entirely outside the provider. For sensitive data, customer-managed or HYOK provides stronger protection against provider and government access. Key rotation and lifecycle management must be operational.

### Manage Cloud Identity And Access Management

Cloud IAM is the primary security control.

Implement:

- the integration of cloud IAM with enterprise identity providers (federation, SSO);
- role-based access control mapped to job functions;
- the principle of least privilege for cloud resources;
- multi-factor authentication for all cloud access;
- privileged access management for cloud administrator accounts;
- the monitoring of cloud access logs and anomalous activity;
- the management of service accounts and machine identities;
- the handling of cross-account and cross-cloud access.

Cloud IAM controls who can access what in the cloud. Federation with enterprise identity providers centralizes management. RBAC must be mapped to job functions with least privilege. MFA is essential. Privileged cloud accounts (root, global administrator) need PAM. Cloud access logs (CloudTrail, Azure Activity Log, Cloud Audit Logs) must be monitored. Service accounts and machine identities need management. Cross-account and cross-cloud access adds complexity.

### Address Multi-Cloud And SaaS Sprawl

Organizations increasingly use multiple clouds and SaaS services.

Manage:

- the inventory of all cloud and SaaS services in use (including shadow IT);
- the data classification for each service (what data is in each);
- the responsibility mapping for each service;
- the consistency of controls across services;
- the fragmentation of governance across providers;
- the integration of logging and monitoring across services;
- the risk of inconsistent configuration across similar services;
- the use of cloud security posture management (CSPM) tools.

Multi-cloud and SaaS sprawl create fragmented governance. An inventory of all services (including shadow IT discovered through CASB or network analysis) is the foundation. Each service must be classified by data sensitivity. Responsibility mapping varies by service. Controls must be consistent. Logging and monitoring must integrate across services. CSPM tools detect misconfigurations and drift across providers.

### Manage Cloud Sub-Processors And Data Flow

Cloud providers use sub-processors that affect data governance.

Map:

- the cloud provider's sub-processors (support, analytics, telemetry);
- the data flows to sub-processors and their locations;
- the contractual flow-down of data protection obligations;
- the transparency about sub-processor changes;
- the right to object to sub-processors;
- the effect of sub-processors on transfer mechanisms (SCCs, DPF);
- the documentation of the full data flow chain.

Cloud providers use sub-processors for support, analytics, telemetry, and other functions. Each sub-processor receives data and introduces transfer and governance considerations. Data flows must be mapped. Data protection obligations must flow down contractually. Sub-processor changes must be transparent. The right to object protects the customer. Sub-processors in non-adequate countries affect transfer mechanisms. The full data flow chain must be documented.

### Ensure Regulatory Compliance For Cloud-Hosted Data

Specific regulations apply to cloud-hosted data.

Comply with:

- GDPR Article 28 processor requirements and DPAs for cloud services;
- HIPAA business associate agreements (BAAs) for cloud-hosted PHI;
- PCI DSS for cloud-hosted cardholder data (cloud provider's PCI attestation);
- FedRAMP authorization for federal cloud services;
- sector-specific cloud requirements (financial services, government, defense);
- data residency and localization requirements;
- the cloud provider's compliance attestations (SOC 2, ISO 27001, sector-specific);
- the gap between provider attestation and customer responsibility.

GDPR requires a DPA with cloud processors. HIPAA requires a BAA for PHI in the cloud. PCI DSS requires the provider's PCI attestation and the customer's responsibility matrix. FedRAMP authorizes cloud services for federal use. Sector-specific requirements add layers. The provider's attestations (SOC 2, ISO 27001) cover the provider's responsibilities but not the customer's. The gap between attestation and customer responsibility must be addressed.

### Plan For Cloud Exit And Data Portability

Cloud exit planning prevents lock-in and ensures continuity.

Plan:

- data portability and the ability to extract data in usable formats;
- the documentation of data location and structure;
- the transition timeline and provider cooperation obligations;
- the feasibility of migrating to another provider or on-premises;
- the cost and technical complexity of exit;
- the effect of proprietary formats and APIs on portability;
- the handling of data deletion upon exit;
- the testing of exit plans for critical services.

Cloud lock-in creates operational and compliance risk. Exit planning must address data extraction in usable formats, transition support, and the feasibility of alternatives. Proprietary formats and APIs can impede portability. The cost and complexity of exit should be assessed before commitment. Data deletion upon exit must be verified. Exit plans for critical services should be tested periodically.

## Common Traps

### Shared Responsibility Misunderstood

Assuming the cloud provider secures elements (data, configuration, access) that are the customer's responsibility.

### Data Residency Violated By Cross-Region Replication

Configuring backups or replication across regions without verifying residency compliance.

### Provider-Managed Keys Used For Sensitive Data

Using provider-managed encryption keys for sensitive data, exposing it to provider and government access.

### Shadow IT Cloud Services Not Inventoried

Cloud and SaaS services adopted by business units without IT or compliance oversight.

### Sub-Processor Data Flows Not Mapped

Failing to map where the cloud provider's sub-processors send data.

### Provider Attestation Treated As Covering Customer Responsibilities

Assuming SOC 2 or ISO 27001 certification covers configuration and access that are the customer's job.

### No Cloud Exit Plan

Committing to a cloud service without planning for data extraction and migration.

## Self-Check

- Is the shared responsibility model mapped for each service (IaaS, PaaS, SaaS) with specific responsibilities for encryption, patching, access, backup, monitoring, documented boundaries, and SaaS capability gaps?
- Are data residency and sovereignty requirements addressed with regulatory requirements, localization mandates, region/zone configuration, cross-region replication, sub-processor locations, government access effects, location documentation, and sovereign cloud offerings?
- Is encryption and key management implemented with at-rest and in-transit encryption, key management choice (provider-managed, BYOK, HYOK), government access effects, customer-managed keys for sensitive data, rotation/lifecycle, HSM integration, and documented approach?
- Is cloud IAM managed with enterprise IDP integration, RBAC, least privilege, MFA, privileged access management, log monitoring, service account/machine identity management, and cross-account/cross-cloud handling?
- Is multi-cloud and SaaS sprawl managed with complete inventory (including shadow IT), data classification, responsibility mapping, control consistency, governance integration, logging/monitoring integration, configuration consistency, and CSPM tools?
- Are cloud sub-processors and data flows mapped with sub-processor identification, data flow/location mapping, contractual flow-down, change transparency, objection rights, transfer mechanism effects, and full chain documentation?
- Is regulatory compliance ensured for cloud-hosted data including GDPR Article 28/DPA, HIPAA BAA, PCI DSS, FedRAMP, sector-specific, residency/localization, provider attestations, and attestation-responsibility gap?
- Is cloud exit and data portability planned with usable format extraction, location/structure documentation, transition timeline, alternative feasibility, cost/complexity, proprietary format effects, deletion verification, and critical service testing?
- Is the cloud data governance program reviewed at least annually and when services or regulations change?
- Are cloud configurations continuously monitored for drift from compliance baselines?