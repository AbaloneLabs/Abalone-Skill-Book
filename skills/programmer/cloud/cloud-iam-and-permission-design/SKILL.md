---
name: cloud_iam_and_permission_design.md
description: Use when the agent is designing cloud identity and access management, writing least-privilege IAM policies, choosing between roles and direct user grants, setting up service accounts and workload identity, granting cross-account or cross-service access, reasoning about allow/deny precedence in policy evaluation, or auditing existing permissions for over-broad wildcard access. Also covers IAM as the primary security boundary, resource-level versus account-level scope, the trap of *:* wildcard policies that persist because they "just work," permission drift after launch, break-glass and emergency access, and the tension between least privilege and operational velocity. Use when provisioning a new workload, defining who or what can access a cloud resource, reviewing a policy for blast radius, or debugging an access-denied error.
---

# Cloud IAM And Permission Design

Cloud IAM is the security boundary that most reliably gets the wrong attention: it is treated as a setup task during provisioning, then ignored until an access-denied error, a failed audit, or a breach forces a review. Engineers grant `*:*` or administrator-equivalent roles to make a deployment work, intend to "tighten it later," and never do — so the wildcard persists for years on a service account that now touches production data and billing. Unlike a bug in application code, an over-broad permission produces no error and no log; it fails silently, surfacing only when an attacker exploits it.

The judgment problem is that least privilege is correct but friction-heavy (a tightly scoped role breaks the next feature), while broad access is wrong but friction-free (a wildcard works immediately and nothing complains). Agents optimize for the immediate friction and inherit the latent risk. The harm is severe: an over-permissioned service account is the most common path by which a cloud environment is compromised, because attackers do not break cryptography — they find the credential with the keys to everything.

## Core Rules

### Treat IAM As The Primary Security Boundary, Not A Deployment Prerequisite

IAM is the security boundary, not a deployment-checklist step. Permissions must be designed against threat models (what could a compromised credential do), not convenience (what makes the deploy pass); blast radius must be bounded; and the default posture is deny.

- **Model the threat: assume the credential leaks.** Design every identity as if its keys will be stolen — through committed secrets, leaked variables, compromised CI, or insider access. The question is not "is this trusted" but "if compromised, how far does the damage reach." An identity that can read all storage, write all databases, and create identities has a blast radius equal to the whole account.
- **Default-deny, then allow explicitly, scoped to the resource.** Start from no access and add specific actions on specific resources, binding to the most specific ARN (`s3:GetObject` on `arn:...:bucket/*`, not `*`). Action-level scoping with resource wildcards still grants every present and future resource of that type.

### Prefer Roles Over Direct Grants, And Scope Workload Identity To The Workload

Direct grants to users or long-lived service accounts with embedded credentials are the most common source of permission drift: each is a one-off decision never revisited. Roles — reusable, named permission bundles — make permission design an auditable artifact.

- **Grant permissions to roles; attach roles to identities.** This lets you review "what does the payments role allow" independently of "who holds it" and change it in one place. Design roles around job function, not ad-hoc requests — a `power-user` role accumulated from "just one more thing" is an unbounded escalation surface.
- **One identity per workload, scoped to its resources.** A service reading one queue and writing one table needs an identity scoped to that queue and table — not a shared "app service account" every microservice reuses. Shared accounts make blast radius unbounded and attribution impossible. Separate identities by environment: dev must not reach production data. Prefer platform-attested workload identity (federation, instance profiles, managed identities) over static keys, which are the dominant leak vector.

### Design Cross-Account And Cross-Service Trust Deliberately

Cross-boundary access is where permission design gets subtle, because trust must be established in both directions and the conditions matter as much as the trust itself.

- **Cross-account access uses trust policies, not shared credentials.** Account A's role can be assumed by Account B only if A's trust policy explicitly permits B's identity. Never share long-lived credentials across accounts; use role assumption with a tightly-scoped trust policy, and constrain it with conditions (source ARN, source account, MFA, IP, time). A policy allowing "any principal in account B" is broad; one allowing "B assuming role X, only from source IP Y, only with MFA" is bounded.
- **For calls on behalf of a user, pass the user's token.** When service A calls service B for user U, B should authorize based on U's identity (passed through a token), not grant A blanket rights in B. Granting A broad rights makes A a privilege-escalation surface and abandons least privilege end-to-end.

### Understand Policy Evaluation: Explicit Deny Wins, And All Policies Union

Cloud policy evaluation is non-obvious: it produces both accidental over-permissioning and accidental lockouts, because the union of several "reasonable" policies can be far broader than any single one.

- **Default is deny; an explicit deny always wins.** If any applicable policy explicitly denies an action, it is denied regardless of other allows. An explicit deny is the strongest tool for a hard boundary ("no identity may delete this bucket, ever").
- **All applicable policies union into the effective set.** Identity-based, resource-based, permission boundaries, organization-level, and session policies all combine. A workload may appear narrowly scoped by its direct role yet gain broad access through an inherited organization policy or a permissive resource policy. Audit the union; use permission boundaries or service control policies to set a ceiling no role can exceed, and test effective permissions with a simulator — the interaction of multiple policies is easy to get wrong by reading.

### Keep Permissions Tight Over Time: Audit, Alert, Remove

Permissions decay: a role accumulates capabilities as features are added, wildcards silently broaden as new resources match them, and migration identities are never removed. Least privilege is a posture you maintain, not a state you reach once. Review which identities can access sensitive resources (production data, billing, identity management) and challenge each grant against current need; "last accessed" data reveals permissions unused for months. Alert on privilege-escalation actions (identity creation, role assumption, policy modification, key creation) and require approval in production. Tie identity lifecycle to workload lifecycle and remove retired identities.

## Common Traps

### The `*:*` Or Administrator Role That "Just Works"

Granting a wildcard or admin role to resolve an access-denied error during setup, intending to tighten it later, and never doing so. The wildcard produces no error and no friction, so nothing forces the follow-up, and the permission persists for years. Resolve access-denied by adding the specific missing permission, not by escalating to a wildcard; treat any wildcard as a tracked defect with a narrowing plan.

### Long-Lived Static Credentials Committed Or Embedded

Static access keys in config files, environment variables, CI secrets, or instance user-data, which leak through source commits, log exfiltration, or shared artifacts. Prefer platform-attested workload identity or short-lived federated tokens; where unavoidable, store a static key in a secrets manager, rotate it, and scope it minimally.

### Shared Service Accounts Across Workloads

One account reused by many microservices so attribution is impossible and the blast radius of any single compromise is the union of all of them. Give each workload its own identity scoped to its resources; a shared account makes "which service did this" unanswerable and "how bad is this leak" catastrophic.

### Trusting Another Account Unconditionally

A cross-account trust policy allowing any principal in the partner account without conditions, turning an open trust into a path by which a compromise there reaches you. Constrain trust with specific roles, source conditions, and MFA.

### Inferring Effective Permissions From A Single Policy

Reading only the role you wrote and concluding the identity is narrowly scoped, when an inherited organization policy, a permissive resource policy, or a permission boundary broadens the effective set. The union of all applicable policies is the real permission; verify it with a simulator or actual call.

### Production And Non-Production Sharing An Identity

A dev or staging workload using an identity that can reach production data, so a mistake or compromise in a lower environment propagates to production. Use distinct identities per environment, with dev unable to touch production.

### No Break-Glass Path, Or An Unaudited One

Either no emergency access exists (so a locked-out team breaks things under pressure), or a standing administrator account exists with no logging (an unmonitored back door). Design a break-glass identity unused in normal operation, whose every use is alerted and reviewed, with sealed and rotated credentials.

## Self-Check

- [ ] IAM is treated as the primary security boundary: every identity was designed against the threat that its credential leaks, blast radius is bounded, posture is default-deny, and permissions are scoped at resource level (most specific ARNs), not just action level.
- [ ] Permissions are granted to reusable roles attached to identities (not direct user grants), roles are designed around workload or function rather than accumulated ad-hoc, and no broad "power user" role absorbs one-off requests.
- [ ] Each workload has its own identity scoped to its resources (no shared service accounts), environments use distinct identities (dev cannot reach prod), and platform-attested workload identity is preferred over static credentials.
- [ ] Cross-account and cross-service trust uses explicit trust policies constrained by conditions (role, source, MFA, IP), no credentials are shared across accounts, and calls on behalf of a user pass the user's token rather than granting the caller blanket rights.
- [ ] Policy evaluation is understood and verified: explicit deny wins, the effective permission is the union of all applicable policies, permission boundaries/SCPs cap the maximum grant, and effective permissions were tested with a simulator rather than inferred.
- [ ] There are no `*:*` or administrator-equivalent roles on production identities; any wildcard is a tracked defect with a narrowing plan, and access-denied errors were resolved by adding specific permissions rather than escalating.
- [ ] No long-lived static credentials are committed, embedded in configs/CI/user-data, or shared across workloads; where unavoidable they live in a secrets manager with rotation and minimal scope.
- [ ] Permissions are maintained over time: sensitive-resource permissions are audited with "last accessed" data, privilege-escalation actions are alerted, and identities for retired workloads or migrations are removed.
- [ ] A break-glass path exists, is unused in normal operation, its every use is alerted and reviewed, and its credentials are sealed and rotated — neither absent nor unaudited.
