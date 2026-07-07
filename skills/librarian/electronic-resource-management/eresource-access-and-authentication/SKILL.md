---
name: eresource_access_and_authentication.md
description: Use when the agent is configuring or troubleshooting electronic resource access in a library, managing authentication systems (IP, proxy, SSO, OpenAthens, Shibboleth, SAML), handling off-campus and remote access, resolving access problems, managing link resolvers and knowledge bases, or ensuring that authorized users can reach licensed content reliably.
---

# E-Resource Access And Authentication

Licensing an electronic resource secures the right to use it, but access, the technical infrastructure that connects authorized users to the licensed content, is what actually delivers that right in practice, and it is where most e-resource service failures occur. The defining reality of e-resource access is that it sits at the intersection of multiple complex systems, the library's discovery and link resolver, the institution's identity and authentication infrastructure, the vendor's platform, and the network between them, any of which can break and sever users from content they are entitled to. A faculty member who cannot reach a database from off-campus, a student whose article link dead-ends at a paywall, or a researcher whose search returns no full text because the link resolver is misconfigured are all experiencing access failures that, to the user, feel like the library does not have the resource, even when it is fully licensed. The risk is treating access as a one-time setup rather than an ongoing service requiring monitoring, troubleshooting, and coordination across library, IT, and vendor systems. The discipline is to design access for reliability and breadth (off-campus, multi-device, distance learners), to monitor for failures, and to resolve problems quickly, because an access failure is, to the user, a collection failure.

Use this skill when configuring, troubleshooting, or managing electronic resource access and authentication. The goal is to prevent the agent from treating access as a static setup, and from neglecting the monitoring, troubleshooting, and cross-system coordination that reliable e-resource access requires.

## Core Rules

### Choose And Configure Authentication For The User Population

Authentication determines who can reach licensed content, and it must match the user population:

- IP authentication: simple and reliable for on-campus users, but useless for off-campus and distance users without additional infrastructure.
- Proxy servers (EZproxy): extend IP authentication to off-campus users by proxying their connection through the library's IP; configure and maintain the proxy, including stanzas for each resource.
- Single sign-on (SSO, SAML, Shibboleth, OpenAthens): federated identity that authenticates users through the institution's identity provider; more robust for complex user populations but requires coordination with IT.
- Walk-in users: ensure on-site walk-in users can access resources, often through IP authentication within the library's network.

Match the authentication method to the user population. A library with significant distance or remote users cannot rely on IP alone; it needs proxy or SSO to deliver access where users actually are.

### Ensure Off-Campus And Remote Access Works Reliably

Off-campus access is where most users now work, and it must be reliable:

- Proxy configuration: maintain accurate proxy stanzas and configurations for each resource; misconfigured stanzas cause access failures that look like the library lacks the resource.
- SSO integration: ensure SSO works for all licensed resources, including those that require attribute release or specific federation membership.
- Multi-device access: users access from phones, tablets, and home networks; test access across devices and network conditions.
- Credential and account management: ensure users can get or reset the credentials they need for remote access, with clear instructions and support.

Off-campus access is not a nice-to-have; it is the primary access mode for most modern users. Test it, maintain it, and support the users who depend on it.

### Manage Link Resolvers And Knowledge Bases

Link resolvers and knowledge bases connect users from citations and discovery to full text:

- Knowledge base accuracy: the knowledge base must accurately reflect the library's holdings and entitlements; stale or incorrect entries send users to paywalls or wrong links.
- Link resolver configuration: configure the link resolver to appear in databases, discovery, and citations, so users can reach full text from wherever they find a citation.
- Coverage dates and embargoes: the knowledge base must reflect coverage dates and embargoes, so users are not directed to content they cannot access.
- Interlibrary loan integration: where full text is not available, the link resolver should route seamlessly to ILL, so the user has a path forward.

A misconfigured knowledge base or link resolver defeats access even when the content is licensed. Maintain the knowledge base and test the resolver across resources.

### Monitor For Access Failures Systematically

Access failures are inevitable across complex systems, and systematic monitoring catches them before users do:

- Proactive monitoring: regularly test access to major resources from on-campus and off-campus, to catch failures early.
- Reporting channels: make it easy for users to report access problems, and triage reports promptly.
- Pattern recognition: clusters of reports about a single resource or platform often indicate a systemic issue rather than individual user error.
- Vendor status awareness: track vendor-reported outages and platform changes that affect access.

Do not wait for users to discover access failures. Monitor proactively and respond to reports quickly, because each unresolved failure is a user denied their entitlement.

### Troubleshoot Across Library, IT, And Vendor Systems

Access failures can originate in multiple systems, and troubleshooting requires cross-system coordination:

- Isolate the failure: determine whether the problem is authentication, the link resolver, the vendor platform, the network, or user error; test systematically.
- Library systems: check proxy stanzas, knowledge base entries, and link resolver configuration.
- IT and identity: coordinate with IT on SSO, network, and credential issues that originate in institutional infrastructure.
- Vendor support: engage vendor support for platform-specific issues, with specific error information and timestamps.

Access troubleshooting is rarely simple. Approach it systematically, isolate the failing system, and coordinate across library, IT, and vendor to resolve.

### Document Access Configuration And Known Issues

Access infrastructure is complex and staff-dependent; documentation sustains it:

- Configuration documentation: document proxy stanzas, SSO configuration, link resolver setup, and knowledge base management so that staff can maintain and troubleshoot.
- Known issues and workarounds: maintain a record of known access issues and their workarounds, so recurring problems are handled consistently.
- Onboarding: use documentation to onboard new e-resource staff, so access knowledge is not held by a single person.
- Vendor contacts: maintain current vendor technical support contacts for efficient escalation.

Access infrastructure that depends on one person's undocumented knowledge is fragile. Document configurations, issues, and contacts so the service is sustainable.

## Common Traps

### Treating Access As One-Time Setup

Configuring authentication once and assuming it works. The trap is that setup feels complete, but access requires ongoing monitoring and maintenance.

### Neglecting Off-Campus Access

Relying on IP authentication when most users are remote. The trap is that on-campus access feels sufficient, but distance users are cut off.

### Stale Knowledge Base And Link Resolver

Holdings and entitlements not updated, sending users to paywalls. The trap is that the resolver exists, but wrong data defeats it.

### Waiting For Users To Report Failures

Reactive rather than proactive monitoring. The trap is that users report some failures, but many simply leave, assuming the library lacks the resource.

### Troubleshooting In One System Only

Blaming the vendor or the user without isolating the failure. The trap is that each system seems fine, but the failure spans multiple systems.

### Undocumented Access Infrastructure

Configuration held by one person. The trap is that the setup works, but staff turnover breaks undocumented access.

## Self-Check

- Is the authentication method (IP, proxy, SSO) matched to the user population, including off-campus, distance, and walk-in users?
- Is off-campus and remote access tested and reliable across devices, with credential support for users?
- Are the link resolver and knowledge base accurate for holdings, coverage dates, and embargoes, with ILL integration for unavailable content?
- Is access monitored proactively, with easy user reporting and prompt triage of failures?
- Are access failures troubleshot systematically across library, IT, and vendor systems, isolating the failing component?
- Is access infrastructure documented, including configurations, known issues, workarounds, and vendor contacts, so the service is sustainable across staff?
- Are proxy stanzas, SSO configuration, and knowledge base entries maintained as resources and platforms change?
- Has the access infrastructure been reviewed for reliability and breadth, given that an access failure is experienced by the user as a collection failure?
