---
name: cloud_networking_and_vpc_design.md
description: Use when the agent is designing cloud network topology, laying out a VPC and subnets, deciding public versus private subnets, choosing between security groups and network ACLs, placing NAT gateways, connecting VPCs via peering or transit gateway, planning private connectivity to managed services or on-premises, or designing DNS resolution across VPCs. Also covers the trap of default-open networks and permissive 0.0.0.0/0 rules, east-west versus north-south segmentation, egress cost from NAT and cross-AZ traffic, overlapping CIDR blocks that block future peering, and the difference between cloud-provider network topology (this skill) and protocol-level networking. Use when provisioning a new VPC, reviewing a network for security and blast radius, debugging connectivity, or planning a multi-VPC or hybrid network.
---

# Cloud Networking And VPC Design

Cloud network design is where security, cost, and operability collide, and it is the layer most reliably built backwards: teams accept the provider's default VPC, leave default-open rules in place because "we'll lock it down later," and discover only at audit or breach time that every resource was reachable from the internet. Unlike application code, a network topology is hard to refactor once it carries production traffic — CIDR ranges cannot be renumbered, peering cannot be added to overlapping ranges, and a "default allow" posture is invisible until exploited. The meaningful unit is not the port but the topology: which subnet holds which workload, which subnets reach the internet, and how routing, addressing, and firewalls combine to make a resource reachable or isolated. Agents treat the VPC as plumbing configured once and conflate "the network passes" with "the network is secure." The harm is asymmetric: an over-open network produces a silent, exploitable blast radius that never errors, while an over-restricted network produces a steady drip of connectivity failures, NAT bills, and cross-AZ latency.

## Core Rules

### Design The VPC And Subnet Layout Against Workload Placement

The VPC and its subnets determine reachability; their layout should be driven by what runs where. The mistake is provisioning a single flat subnet, then bolting security on top — once everything is in one space, segmentation depends entirely on firewalls, which is fragile.

- **Plan CIDR ranges for the future.** A VPC's CIDR block cannot easily be changed after creation, and two VPCs with overlapping CIDRs cannot be peered. Choose a range large enough for growth, and coordinate ranges across VPCs and on-premises so peering and hybrid connectivity stay possible.
- **Use subnet tiering to encode reachability intent.** Public subnets (with a route to an internet gateway) hold only internet-facing things — load balancers, bastions, egress proxies. Private subnets (no direct internet route) hold application, database, and stateful tiers that should never be directly reachable from outside. When routing expresses the intent, a misconfigured firewall is a second line of defense.
- **Spread across availability zones for resilience.** Each tier should span multiple AZs via subnets in each, so a zonal failure does not take the tier down. Multi-AZ is the default for production; single-AZ is acceptable only for dev environments.

### Keep Stateful Tiers Private, And Treat Exposure As Routing + Addressing + Firewall

The public/private distinction is the most important reachability decision and the one most often defaulted wrong. A resource in a public subnet with a public IP and an open SG is reachable from the internet; a resource in a private subnet is not, because routing forbids it.

- **Nothing stateful or sensitive belongs in a public subnet.** Databases, caches, internal APIs, and application tiers live in private subnets with no direct internet path. The only legitimate residents of public subnets are internet-facing load balancers, bastion hosts, and egress proxies.
- **Reachability is the product of routing, addressing, and firewall.** Closing any one closes the path. The safest posture is to remove the public IP and the internet route, not merely to tighten the firewall on a publicly-addressed resource.
- **Bastions and load balancers are the only sanctioned bridges.** Inbound access to private tiers flows through a load balancer or a bastion/SSM session, both auditable choke points. Direct SSH/RDP to private instances should not exist.

### Use Security Groups As The Primary Control, NACLs As Guardrails

Cloud firewalls come in two flavors — stateful security groups on interfaces, and stateless NACLs on subnets — and conflating them produces gaps and pain. Security groups are the primary control: stateful, per-resource, allow-only, default-deny, and referencing other SGs rather than CIDRs, which makes them precise and maintainable. NACLs are stateless and subnet-level: both inbound and outbound rules must be written and return traffic explicitly allowed, so they are painful as the primary control. Use them sparingly as a subnet-wide guardrail for defense-in-depth.

### Place NAT Gateways Deliberately: They Enable Private Egress But Cost

Private subnets need a way to reach the internet for updates and outbound API calls, and the NAT gateway is the standard mechanism — also a recurring cost center and single point of egress that must be designed, not defaulted. NAT is billed per-gateway-hour and per-GB processed: a chatty private workload can generate large, hidden egress bills, and a NAT per AZ multiplies the hourly cost. Scrutinize what needs egress, route only what does, and keep data co-located. High availability requires a NAT per AZ — a single NAT is a single point of failure for a multi-AZ VPC; the resilient pattern is one NAT per AZ with per-AZ routing.

### Connect VPCs And Services By Scale And Topology — And Plan DNS Alongside

Connecting VPCs and managed services is where topology decisions compound: each model has different limits, cost shapes, and routing implications, and the wrong choice forces rework across every connected VPC. VPC peering is point-to-point and simple but does not scale past a handful of VPCs — it has no transitive routing (A peered to B and B to C does not let A reach C), so it suits a few stable connections, not meshes. Transit Gateway is the hub-and-spoke model for many VPCs: it centralizes routing so any attached VPC can reach any other, and supports transitive and hybrid connectivity. PrivateLink exposes a service privately without peering or public internet — prefer it when you do not need a full mesh. Across all of these, avoid overlapping CIDRs, which break peering, transit gateway, and hybrid connectivity.

DNS is the silent dependency that breaks connectivity in confusing ways: a resource is reachable by routing and firewall but unresolvable by name, so the application fails despite the network "passing." Enable private DNS for managed services — otherwise workloads resolve the public endpoint and traffic egresses the wrong way. Cross-VPC DNS resolution must be explicitly linked: peered or transit-gateway-connected VPCs do not automatically resolve each other's private names, so resolution must be enabled on the peering or gateway. Hybrid DNS needs forwarding — resolving names across on-premises and cloud requires conditional forwarders or resolver endpoints, or you get "route works, name does not" failures.

## Common Traps

### The Default VPC And Default-Open Rules Left In Place

Accepting the provider's default VPC with its permissive 0.0.0.0/0 inbound rules and public subnets, then provisioning production resources into it. The default is built for convenience, not security; everything inherits the open posture. Create a purpose-built VPC with private subnets and default-deny security groups.

### 0.0.0.0/0 On A Stateful Tier "Because It Works"

Opening a security group to the world on a database, cache, or internal API to resolve a connectivity issue, and leaving it because nothing complains. The open rule produces no error and no log; it fails only when exploited. Fix routing and SG references instead of widening to anywhere.

### Single Flat Subnet With No Tiering

Putting the load balancer, application, and database in one subnet and relying entirely on security groups for segmentation. Once segmentation depends only on firewalls, a single misconfiguration exposes everything. Use subnet tiers so routing is the first line of defense.

### Overlapping CIDRs Discovered At Peering Time

Provisioning VPCs with overlapping or default ranges, then discovering at peering or hybrid-connection time that they cannot connect without translation. Plan CIDR ranges globally before any connection is needed.

### NAT Misuse: Hidden Bills And Single-AZ Failure

Routing all private-subnet traffic through a NAT without scrutiny produces large per-GB-processed bills from chatty workloads, and a single NAT serving a multi-AZ VPC is a single point of failure — a zonal outage strands egress. Route only what needs egress, keep data co-located, and use one NAT per AZ with per-AZ routing where egress is critical.

### Using NACLs As The Primary Segmentation Control

Managing east-west segmentation with stateless NACLs, requiring paired inbound/outbound rules that drift and break return traffic. Use stateful security groups as the primary control; reserve NACLs for guardrails and hard denies.

### Connectivity Tested But DNS Not Tested

Confirming a route exists and a firewall allows traffic, but never verifying name resolution — so the application fails because the private endpoint is unresolvable. Test resolution, not just routing.

## Self-Check

- [ ] The VPC and subnet layout is designed against workload placement and reachability intent (public subnets for internet-facing tiers only, private subnets for stateful tiers), spans multiple AZs for production resilience, and CIDR ranges are planned for growth and coordinated to avoid overlaps that block future peering.
- [ ] Nothing stateful or sensitive is in a public subnet, public IPs are removed from private resources, reachability is assessed as routing + addressing + firewall together, and inbound access to private tiers flows only through load balancers or logged bastion/SSM sessions.
- [ ] Security groups are the primary control (stateful, per-resource, allow-only, default-deny, referencing other SGs rather than CIDRs), NACLs are used sparingly for guardrails, and there are no 0.0.0.0/0 inbound rules on stateful tiers.
- [ ] NAT gateways are placed deliberately: private subnets that need egress have a NAT path, per-GB and per-hour cost was weighed against actual egress needs, and high-availability designs use one NAT per AZ with per-AZ routing rather than a single shared NAT.
- [ ] Inter-VPC connectivity matches scale and topology: peering for a few point-to-point connections, transit gateway for many-VPC or hybrid hub-and-spoke, PrivateLink for private service consumption without mesh, no connected ranges overlap, and DNS resolution (private DNS, cross-VPC, hybrid forwarding) is configured and tested, not just routing.
- [ ] No production workload runs in the provider default VPC or inherits default-open rules; a purpose-built VPC with private subnets and default-deny security groups is used.
- [ ] East-west segmentation is encoded in topology (subnet tiers and SG-to-SG references) so a single firewall misconfiguration does not expose an entire tier, and the blast radius was reviewed assuming a compromised workload inside the VPC.