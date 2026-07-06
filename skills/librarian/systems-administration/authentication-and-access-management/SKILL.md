---
name: authentication_and_access_management.md
description: Use when the agent is configuring, evaluating, or troubleshooting library authentication and access management, including single sign-on via SAML or OpenAthens, Shibboleth and InCommon federation, EZproxy for remote e-resource access, role-based access control for staff systems, patron account authentication, multi-factor authentication, credential and secrets management, federated identity for consortial borrowing, or balancing seamless patron access against security and privacy.
---

# Authentication And Access Management

Authentication is the gate that determines who can use library systems and licensed resources. It is also the seam where convenience, security, privacy, and cost all collide. Patrons want seamless access to e-resources from anywhere; vendors want assurance that only authorized users enter; staff need appropriate permissions to do their jobs; and the library has a duty to protect patron privacy and not build unnecessary surveillance into the login flow. Getting authentication right means patrons reach the article they need without friction, staff can work efficiently, and attackers cannot walk in through weak credentials. Getting it wrong means locked-out patrons, broken e-resource access, support overload, and serious security exposure.

The judgment problem is that authentication spans many technologies and stakeholders that do not naturally coordinate. The identity provider, the learning management system, the ILS, EZproxy, the discovery layer, the link resolver, and dozens of vendors each have their own assumptions. Libraries often inherit legacy configurations, shared accounts, hardcoded credentials, and proxy setups that were "made to work" years ago and never revisited. Privacy is frequently an afterthought: authentication logs that could reveal what a patron read are kept indefinitely, or shared accounts make it impossible to attribute access. The agent's job is to design authentication that is seamless for legitimate users, least-privilege for staff, secure against common attacks, and privacy-respecting in what it logs and retains.

Use this skill when configuring, evaluating, or troubleshooting any authentication or access management for library systems or e-resources. The goal is to prevent the agent from creating friction that locks out patrons, granting excessive staff permissions, leaving shared or hardcoded credentials in place, ignoring privacy in logging, or treating authentication as a one-time setup rather than an ongoing service.

## Core Rules

### Design For Seamless Legitimate Access

Patrons should reach licensed resources with minimal friction. Authentication friction is a leading cause of e-resource access failures and support tickets.

Seamless access practices:

- use single sign-on so patrons authenticate once across services;
- configure on-campus IP recognition for walk-in users where appropriate;
- route off-campus access through a proxy such as EZproxy with SSO integration;
- ensure the discovery layer and link resolver pass authentication context smoothly;
- minimize repeated logins and confusing redirects;
- provide clear error messages and recovery paths when authentication fails.

Every extra login step is a point where patrons give up and the library loses a usage event. Smooth access is a service quality metric.

### Use Federated Identity And Single Sign-On

Federated identity, via SAML, Shibboleth, OpenAthens, or InCommon, is the foundation of modern library authentication.

Federated identity practices:

- integrate library systems with the institutional identity provider;
- use SSO to reduce credential sprawl and improve security;
- release only the attributes needed by each service, following data minimization;
- participate in federations such as InCommon for trusted identity exchange;
- configure attribute release deliberately, not at maximum disclosure;
- maintain entity registration and metadata freshness.

Federated identity centralizes authentication, improves security, and reduces password fatigue, but only if attribute release is configured minimally and deliberately.

### Configure EZproxy And Remote Access Carefully

EZproxy and similar proxies mediate off-campus access to licensed resources. Misconfiguration breaks access or exposes the library to license violations.

Proxy configuration:

- keep EZproxy updated to address security vulnerabilities;
- integrate EZproxy with SSO rather than local accounts where possible;
- configure database stanzas accurately and keep them current;
- use HTTPS throughout to protect patron traffic;
- avoid open proxies that allow unauthorized access;
- monitor for abuse, such as excessive downloads or scraping;
- ensure proxy logs are minimized and retained per privacy policy.

A misconfigured proxy can violate vendor licenses, expose patron reading habits, or become an attack surface. Configure and monitor it deliberately.

### Apply Least Privilege For Staff Access

Staff access to library systems should grant the minimum permissions needed for each role.

Least privilege practices:

- define roles with specific permissions, such as circulation, cataloging, acquisitions, admin;
- assign staff to roles based on job needs, not convenience;
- avoid granting broad admin access as a default;
- segregate duties so critical actions require appropriate authorization;
- review role assignments periodically and on role change;
- remove access promptly when staff leave.

Over-broad staff access increases the blast radius of a compromised or mishandled account. Grant only what each role requires.

### Require Multi-Factor Authentication For Sensitive Access

Administrative and high-privilege access should require multi-factor authentication.

MFA application:

- require MFA for all administrative and staff system access;
- prefer phishing-resistant MFA such as hardware keys or device-based methods;
- extend MFA to remote access and cloud administration;
- have recovery procedures that do not weaken MFA;
- document MFA enrollment and support processes.

MFA on administrative accounts is one of the highest-impact security controls. Do not leave admin access protected by passwords alone.

### Manage Credentials And Secrets Securely

Hardcoded, shared, or weak credentials are a major attack vector. Manage them systematically.

Credential management:

- eliminate shared accounts where possible; use individual accounts with auditability;
- store credentials and secrets in a secure vault or password manager, not in scripts or spreadsheets;
- rotate credentials and API keys periodically and on staff departure;
- use service accounts with minimal privileges for automated integrations;
- avoid embedding credentials in code, configuration files, or documentation;
- audit credential use and revoke unused access.

Shared and hardcoded credentials make breaches likely and attribution impossible. Treat secrets as protected assets.

### Minimize And Protect Authentication Logs

Authentication systems generate logs that can reveal patron activity. Handle them with privacy in mind.

Log privacy practices:

- log what is needed for security and troubleshooting, not more;
- avoid logging full patron reading behavior tied to identity;
- set short retention for authentication and access logs;
- restrict access to logs on a need-to-know basis;
- align logging with the library's privacy policy and legal obligations;
- do not use access logs to monitor what patrons read.

Authentication logs are a privacy liability. Minimize collection, restrict access, and retain only as long as needed.

### Provide Clear Recovery And Support Paths

Authentication will fail for some users. Recovery paths determine whether that becomes a minor incident or a lost patron.

Recovery practices:

- provide self-service password reset where appropriate;
- offer clear help for SSO and proxy errors with troubleshooting guidance;
- train service desk staff on common authentication issues;
- maintain escalation paths to identity and systems teams;
- communicate known issues and workarounds promptly;
- track authentication-related support tickets to find systemic problems.

A patron who cannot log in and cannot get help will abandon the resource. Make recovery visible and effective.

### Coordinate Across Systems And Stakeholders

Authentication touches many systems and teams. Coordination prevents gaps.

Coordination practices:

- align library authentication with institutional identity and IT;
- coordinate attribute release with the identity provider team;
- work with vendors on authentication methods and changes;
- document the end-to-end authentication flow across all systems;
- test changes across the full chain, not just one system;
- maintain shared understanding so no single person is a bottleneck.

Authentication failures often occur at the seams between systems. Coordinate and document the full flow.

### Treat Authentication As An Ongoing Service

Authentication is not set-and-forget. It requires continuous attention.

Ongoing service practices:

- monitor authentication success and failure rates;
- review and update attribute release and permissions periodically;
- apply security patches and updates promptly;
- reassess vendor authentication methods as they change;
- audit for orphaned accounts and excessive permissions;
- review logging and retention against privacy policy.

Authentication drifts as systems, vendors, and staff change. Maintain it as an active service.

## Common Traps

### Friction That Locks Out Patrons

Excessive logins and confusing redirects cause patrons to abandon e-resources. Design for seamless legitimate access.

### Maximum Attribute Release

Releasing all available identity attributes violates data minimization. Release only what each service needs.

### Shared And Hardcoded Credentials

Shared accounts and embedded credentials prevent attribution and invite breaches. Use individual accounts and secure vaults.

### Over-Broad Staff Permissions

Granting admin access by default increases risk. Apply least privilege based on role.

### No MFA On Administrative Access

Password-only admin access is easily compromised. Require MFA, preferably phishing-resistant.

### Privacy-Invading Logs

Authentication logs that capture reading behavior tied to identity are a privacy liability. Minimize, restrict, and shorten retention.

### Uncoordinated System Changes

Changing one system without testing the authentication chain breaks access elsewhere. Coordinate and test end to end.

### Set-And-Forget Authentication

Authentication drifts over time. Monitor, patch, audit, and update continuously.

## Self-Check

- Is legitimate patron access seamless, with SSO, IP recognition, and proxy integration minimizing logins and friction?
- Is federated identity configured to release only the attributes each service needs, with metadata kept current?
- Is EZproxy or remote access updated, HTTPS-protected, integrated with SSO, and monitored for abuse?
- Are staff permissions assigned by role on a least-privilege basis, reviewed periodically and on role change?
- Is multi-factor authentication required for administrative and high-privilege access, using phishing-resistant methods where possible?
- Are credentials and secrets stored in a secure vault, rotated periodically, and free from shared accounts and hardcoded values?
- Are authentication logs minimized, restricted, and retained only as long as needed, without capturing patron reading behavior?
- Are clear recovery and support paths available for authentication failures, with trained service desk staff?
- Is the end-to-end authentication flow documented and coordinated across identity, library systems, and vendors?
- Is authentication treated as an ongoing service with monitoring, patching, auditing, and periodic permission review?
