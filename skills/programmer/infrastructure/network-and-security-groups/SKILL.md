---
name: network_and_security_groups.md
description: Use when the agent is designing cloud network topology or security group and firewall rules, planning VPC or VNet CIDR ranges and subnet tiers, writing least-privilege ingress and egress rules, setting up NAT gateways and internet egress, configuring VPC peering Transit Gateway or private connectivity, building defense-in-depth network layers, segmenting networks for blast-radius containment, or reviewing flow logs and accumulated firewall rules. Covers network topology and security-group and firewall rule design, distinct from general IaC delivery and from identity and authentication design.
---

# Network And Security Groups

Cloud network design has two halves that agents often treat as one: the topology (the VPC or VNet, its subnets, its route tables, its connectivity to the internet and to other networks) and the policy (the security groups or firewall rules that decide which packets may flow). Both must be right, and they fail in opposite ways. Topology fails silently and permanently — a CIDR range chosen too small cannot be grown without recreating the network, and an overlapping range chosen today blocks the peering you need in two years. Policy fails noisily but accumulates — every rule added to fix an outage stays forever, and the "least privilege" posture erodes into "everything can reach everything" one emergency at a time.

Agents tend to reach for the wizard defaults: a VPC with a single large CIDR, a couple of public subnets, security groups that allow 0.0.0.0/0 on 443 and "we'll tighten it later," and a NAT gateway because the docs said to. Each is defensible in a tutorial and wrong in production. A single tier of subnets forces stateful workloads into public space; a security group referencing CIDRs instead of other groups means every new service widens the blast radius; a peering chosen over a Transit Gateway works until the third account joins and the mesh becomes unmanageable. The discipline is to design the network for growth and segmentation from the start, because the cost of redoing it later is paid in outages and re-IP'ing running systems.

This skill is about the network and the rules on it. It is distinct from how the network is delivered as code (see `infrastructure-as-code-design`) and from identity and authentication (a network rule that allows a flow does not authenticate the caller; an IAM policy that permits an action does not open a port). Both layers are needed; neither substitutes for the other.

## Core Rules

### Plan CIDR Ranges For Growth And For Future Connectivity, Not Just For Today

CIDR planning is the one network decision that is hardest to undo. A VPC's CIDR cannot be enlarged in place on most clouds, and overlapping CIDRs between networks you intend to connect will block peering and require painful renumbering. Plan generously and plan for the connections you do not yet know you need.

- **Reserve a large, non-overlapping CIDR block per VPC** from a documented range, and keep a registry of which VPC uses which range. Overlaps are discovered only when you try to peer or connect two networks, at which point renumbering a live system is the only fix. A registry prevents this.
- **Size the VPC for years of growth, not current need.** A `/16` gives 65k addresses and headroom for many subnets; a `/24` runs out sooner than expected once you account for reserved addresses, multiple availability zones, and future services. It is far easier to leave address space unused than to grow a VPC later.
- **Avoid the default ranges everyone uses.** `10.0.0.0/16`, `172.16.0.0/12`, and `192.168.0.0/16` collide constantly with on-prem networks, VPN clients, and other clouds. Choosing a less-common subrange (for example, a distinct chunk of `10.x`) reduces the chance of an overlap that blocks a future merger, acquisition, or peering.
- **Plan subnet sizing per tier and per zone, with headroom.** Subnets cannot be resized on most clouds once created. Size each subnet to fit the maximum number of instances you can plausibly run in that tier in that zone, plus reserved addresses (cloud providers reserve several per subnet).

### Use Distinct Subnet Tiers To Force Topological Separation

Subnet tiers are how the network expresses the trust boundary. A flat network where everything is in one subnet makes every workload a neighbor of the internet; a tiered network makes the path from internet to data explicit and controllable.

- **Public (DMZ) subnets for things with a route to the internet gateway**, private subnets for the application tier with no direct internet ingress, and isolated (or data) subnets for databases and sensitive stores with no route to the internet at all. The tier a workload lives in should match its trust level: load balancers and bastions in public, app services in private, databases in isolated.
- **Route tables enforce the tier, not just the subnet label.** A subnet is "public" because its route table points at an internet gateway; a subnet is "private" because it does not. Do not trust the name — verify the route table. The common error is a subnet labeled private that accidentally has an internet route.
- **Put the data tier in a subnet with no egress to the internet.** A database that can reach the internet is a database that can exfiltrate. The isolated tier's route table should have no default route to a NAT gateway or internet gateway; egress, if any, should be to a strictly controlled inspection path.

### Write Security Group And Firewall Rules Least-Privilege, Default-Deny, And By Reference

Security groups are where least privilege lives or dies. The strong posture is default-deny with explicit, narrow allows, and referencing other security groups rather than CIDRs wherever possible.

- **Start from default-deny.** Every security group should begin with no inbound rules and only the outbound the workload actually needs. The weak posture is a permissive default "to make it work" that never gets tightened.
- **Reference other security groups instead of CIDR ranges.** A rule that allows ingress from "the security group of the app tier" follows the app tier wherever it scales and says exactly what is intended. A rule that allows ingress from `10.0.5.0/24` is brittle (it breaks if the app moves subnets) and over-broad (it allows anything in that range, not just the app). Prefer group-to-group references; use CIDRs only for sources outside your control (a corporate egress IP, a partner range).
- **Be specific about ports and protocols.** Allow the exact port the service listens on, not "all ports." A rule allowing all TCP from a broad source is functionally an open door regardless of intent.
- **Constrain egress as well as ingress.** Most breaches that exfiltrate do so over egress that was left wide open (`0.0.0.0/0` outbound). Limit egress to the destinations the workload actually needs (a secrets manager, an API, a package mirror), and treat unrestricted egress as a defect, not a default.
- **Understand stateful versus stateless.** Cloud security groups are typically stateful (return traffic for an allowed flow is automatically allowed); traditional firewalls and some network ACLs are stateless (you must allow both directions). Know which you are writing, because a stateless rule set that only allows ingress will silently drop the replies.

### Design Internet Egress Deliberately, And Know What A NAT Gateway Costs And Hides

How workloads reach the internet is a security and cost decision, not just a connectivity one.

- **Private subnets reach the internet through a NAT gateway (or NAT instance), not directly.** This lets the app tier fetch patches and call external APIs without being reachable from the internet. But NAT is not free — on AWS, NAT gateway per-GB processing charges are a common surprise bill, and a chatty service can cost more in NAT fees than in compute.
- **Constrain what can egress.** A NAT gateway that allows all destinations lets any compromised workload call any internet host. For higher-security tiers, route egress through a controlled egress proxy or firewall that allowlists destinations, so exfiltration has a chokepoint.
- **Decide whether the data tier may egress at all.** The strongest pattern is no egress for the data tier — no NAT route, no internet gateway. If a database must call out (for backups, for licensing), route that one flow through an inspected path rather than opening general egress.

### Choose Connectivity Between Networks Knowing The Limits Of Each Option

Connecting VPCs to each other or to on-prem is where topology decisions compound. Each option has hard limits that determine whether it scales.

- **VPC peering is point-to-point and non-transitive.** Peering A-to-B and B-to-C does not let A reach C; each pair needs its own peering. A handful of peerings is fine; a full mesh of many VPCs becomes O(n²) and unmanageable. Peering also fails outright if the CIDRs overlap, which is why the CIDR registry matters.
- **Transit Gateway (or the equivalent hub) scales the many-to-many case.** It centralizes connectivity so each VPC attaches once instead of peering to every other VPC, and it supports transitive routing. Use it once the number of connected networks grows past a few, or when you need centralized inspection and routing policy.
- **Private connectivity (PrivateLink, Private Service Connect, service endpoints) keeps traffic off the public internet.** Use it to reach managed services (databases, object storage, APIs) without traversing the internet, which is both more secure and often more reliable. Understand that some private endpoints still need careful security group and DNS configuration, or they can be reached more broadly than intended.
- **VPN and Direct Connect for on-prem.** Choose based on bandwidth, latency, and redundancy needs, and always plan for the link being a single point of failure unless you provision redundant paths. Overlapping CIDRs between cloud and on-prem is the classic blocker here; resolve it in the CIDR plan, not at integration time.

### Build Defense-In-Depth Layers, And Segment For Blast-Radius Containment

A single security layer is a single point of failure for an attacker. Defense in depth means an attacker who defeats one control still faces the next.

- **Layer the controls: internet, DMZ, application, data.** Each layer should assume the previous is compromised. The DMZ assumes the internet is hostile; the application tier assumes the DMZ may be breached; the data tier assumes the application tier may be compromised and limits what the app can do to the data (read but not drop, specific tables, no broad egress).
- **Segment laterally, not just vertically.** Vertical segmentation (internet to app to data) is the common case; lateral segmentation (this microservice cannot reach that microservice's database) contains a compromise to one service instead of the whole environment. Network policies and per-service security groups make lateral segmentation real.
- **Make each segment's blast radius explicit.** The question is not "is this allowed" but "if this segment is compromised, what can it reach?" A segment that can reach every database in the environment has a large blast radius; one that can reach only its own database has a small one. Design to minimize the reachable set from any single compromise.

### Audit, Log, And Periodically Review The Accumulated Rules

Network policy decays. Rules added for an emergency stay; rules for decommissioned services stay; broad rules stay because no one is sure what depends on them. Without periodic review, the rule set drifts toward permissive.

- **Enable flow logs (VPC flow logs, equivalent) and retain them.** Flow logs are the evidence of what actually traversed the network — essential for incident response, for proving compliance, and for finding rules that match no traffic (candidates for removal).
- **Review the rule set on a cadence, not only during audits.** Periodically ask of each rule: what depends on this, when was it last needed, can it be narrowed or removed. Unused and overly broad rules accumulate silently; a review cadence is the only thing that reverses it.
- **Treat rule additions as reviewed changes, not quick fixes.** A security group change made in an incident should be tracked like any other change — reviewed, narrowed, and either kept deliberately or removed during the post-incident cleanup. Rules added under pressure and never revisited are how default-deny erodes.

## Common Traps

### A Too-Small Or Overlapping CIDR That Cannot Be Grown Or Peered Later

A `/24` VPC that runs out of addresses, or a `10.0.0.0/16` that overlaps with the network you need to peer with next year. CIDRs are nearly immutable once workloads run on them. Plan large, plan non-overlapping, and keep a registry.

### A "Private" Subnet That Accidentally Has An Internet Route

The subnet is labeled private, but its route table points at an internet gateway, so it is effectively public. The route table defines the tier, not the name. Verify routes, especially after refactors.

### Security Group Rules Using Broad CIDRs Instead Of Group References

Allowing ingress from `10.0.0.0/16` "because that is the VPC" lets every workload in the VPC reach the service, follows nothing as it scales, and widens the blast radius. Reference the specific source security group instead.

### `0.0.0.0/0` Egress Left As The Default

Unrestricted egress is the path most exfiltration takes, and it is the default on most security groups. Constrain egress to the destinations the workload needs; treat wide-open egress as a defect to be narrowed, not a convenience to keep.

### Confusing Stateful Security Groups With Stateless ACLs

Cloud security groups are stateful (return traffic is allowed automatically); network ACLs are usually stateless (both directions must be allowed). Writing stateless rules as if they were stateful silently drops replies; the symptom is "the rule is there but traffic does not work."

### Reaching For VPC Peering Past The Point Where It Scales

Peering is fine for two or three VPCs, but it is non-transitive and O(n²) in a mesh. Past a handful of networks, the peering graph becomes unmanageable and overlapping-CIDR conflicts multiply. Move to a Transit Gateway once the count grows.

### Private Endpoints Assumed Secure Without Checking Their Security Groups

A private endpoint keeps traffic off the internet, but if its security group is broad, the endpoint is reachable from more of the network than intended. Private connectivity is not a substitute for least-privilege rules on the endpoint itself.

### Rules Added In An Emergency And Never Removed Or Narrowed

An outage is fixed by opening a broad rule, the fix works, and the rule stays forever. Over years the rule set becomes permissive by accumulation. Track emergency rule changes as debt and review them in post-incident cleanup.

### Trusting The Tier Name Instead Of The Route Table And Rules

A subnet called "isolated" that has a NAT route, or a security group called "restricted" that allows broad ingress, gives false assurance. Verify the actual routes and rules; the name is documentation, not enforcement.

### No Flow Logs, So Incidents And Stale Rules Are Invisible

Without flow logs there is no evidence of what traversed the network, so incident response is blind and stale-rule cleanup is guesswork. Enable and retain flow logs as a baseline, not an afterthought.

## Self-Check

- [ ] VPC and VNet CIDR ranges are sized for years of growth (not just current need), drawn from non-default ranges that reduce collision risk, recorded in a registry of which network uses which range, and confirmed non-overlapping with every network they may need to connect to (on-prem, peer clouds, VPN clients).
- [ ] Subnets are organized into tiers (public/DMZ, private/application, isolated/data) where the tier matches the workload's trust level, the route tables actually enforce the tier (a "private" subnet has no internet route, an "isolated" subnet has no NAT or internet route), and the data tier has no path to the internet.
- [ ] Security groups and firewall rules start from default-deny, allow only the exact ports and protocols needed, reference other security groups rather than broad CIDRs wherever the source is internal, and constrain egress (not just ingress) to the destinations the workload actually needs.
- [ ] The stateful-versus-stateless nature of each rule layer is known and handled (stateless ACLs allow both directions, stateful security groups allow return traffic automatically), so no rule silently drops replies.
- [ ] Internet egress is deliberate — private subnets reach the internet through a NAT gateway or controlled egress proxy, the data tier has no egress or only an inspected path, and the cost and allowlist implications of NAT were considered rather than accepted as a default.
- [ ] Inter-network connectivity (peering, Transit Gateway, private endpoints, VPN/Direct Connect) was chosen knowing each option's limits — peering is non-transitive and fails on overlapping CIDRs, Transit Gateway scales the many-to-many case, private endpoints still need least-privilege security groups, and on-prem links have redundancy planned.
- [ ] Defense in depth is layered (internet to DMZ to app to data, each assuming the prior is compromised) and segmented laterally (a compromise of one service cannot reach every database), with each segment's reachable set — its blast radius — minimized by design.
- [ ] The network design was produced as code (Terraform, CloudFormation, or equivalent) with peer review and version history, not hand-edited in a console, so changes are reproducible, auditable, and reversible — a topology decision this hard to undo should never be made through an untracked click.
- [ ] Flow logs are enabled and retained for incident response and compliance, the rule set is reviewed on a cadence to remove or narrow unused and over-broad rules, and emergency rule changes are tracked as debt that is reconciled in post-incident cleanup rather than left to accumulate.
