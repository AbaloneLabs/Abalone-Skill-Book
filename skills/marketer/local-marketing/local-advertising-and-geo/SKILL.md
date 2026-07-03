---
name: local_advertising_and_geo.md
description: Use when the agent is planning geo-targeted advertising, running local search ads, using geofencing, doing local social advertising, setting radius targeting, measuring foot-traffic attribution, running local inventory ads, or allocating budget across multiple locations.
---

# Local Advertising And Geo

Local advertising is not national advertising with a smaller map. It operates on proximity, intent tied to place, and the physical reality that a customer must be able to reach the location. It fails when geo-targeting is set too wide and wastes spend on unreachable audiences, when foot-traffic attribution is assumed rather than measured, when budget is spread evenly across locations regardless of performance, or when local inventory ads promise products that are not actually in stock. The defining risk of local advertising is spending on impressions that cannot convert into visits, because the audience was too far, the store was closed, or the product was absent.

Use this skill before answering questions such as "how do we advertise to local customers", "should we use geofencing", "how do we measure store visits from ads", "how do we allocate budget across locations", or "how do local inventory ads work". The goal is to prevent the agent from treating geo as a simple targeting toggle and ignoring the proximity, inventory, and attribution realities that determine whether local ad spend produces actual visits and sales.

## Core Rules

### Set Geo-Targeting Around Reachable Geography

The core question in local advertising is how far a customer will realistically travel for the offering. Targeting too wide wastes spend on people who will never visit; targeting too narrow misses the real catchment area.

Set targeting by:

- understanding the realistic travel radius for the business type (a coffee shop versus a specialty clinic differ enormously);
- using drive-time and distance rather than raw radius where the platform allows;
- adjusting for natural barriers such as rivers, highways, and jurisdiction boundaries;
- layering intent signals such as "nearby" or "interest in category" onto geography;
- testing and refining the radius based on observed visit and conversion data.

A default radius applied to every business ignores that a convenience purchase draws from blocks while a destination purchase draws from miles.

### Match The Ad Format To Local Intent

Different local ad formats capture different intent states, and mismatching them wastes budget.

Match formats by:

- using local search ads to capture high intent from users actively searching for nearby options;
- using geofencing and location-based display to reach users currently in or near a relevant area;
- using local social ads to build awareness and consideration among a defined local audience;
- using local inventory ads to connect nearby searchers to products actually in stock;
- reserving each format for the intent it serves, not running all formats everywhere.

Search ads suit the ready-to-act moment; geofencing suits proximity-based relevance; social suits awareness. Using them interchangeably produces poor efficiency.

### Use Geofencing With Precision And Relevance

Geofencing targets users within a defined geographic boundary. It is powerful but easily misused through over-broad or irrelevant fences.

Use geofencing well by:

- drawing fences around genuinely relevant areas, such as competitor locations, events, or neighborhoods that match the audience;
- avoiding fences so large they include irrelevant populations;
- considering competitor and contextual geofencing where legally and ethically appropriate;
- respecting privacy and platform policies on location tracking;
- measuring whether fenced audiences actually convert at higher rates.

A fence drawn around an entire city is not geofencing; it is wasted spend with a geographic label.

### Measure Foot-Traffic Attribution Honestly

Foot-traffic attribution estimates whether ad exposure led to store visits. It is valuable but imperfect, and overclaiming its precision leads to bad decisions.

Measure attribution by:

- understanding the methodology and limitations of the platform's visit estimates;
- using store visit conversions where available, while recognizing they are modeled, not exact;
- combining attribution signals such as visits, calls, directions requests, and online-to-offline;
- establishing a baseline before the campaign to isolate the ad effect;
- avoiding the trap of counting every exposed visit as caused by the ad.

Modeled visit data is useful for direction and comparison, not for exact accounting. The agent should treat it as evidence, not proof.

### Run Local Inventory Ads Only With Accurate Inventory

Local inventory ads show nearby shoppers that a product is in stock at a specific location. They fail catastrophically when the inventory feed is wrong.

Run them by:

- maintaining a real-time, accurate product feed tied to each location's stock;
- excluding out-of-stock items to avoid sending customers to empty shelves;
- synchronizing pricing and promotions across the feed and the store;
- handling variations such as size or color availability per location;
- monitoring for feed errors that suppress or misrepresent products.

A customer who drives to a store based on a local inventory ad and finds the item absent loses trust and rarely returns. Accuracy is non-negotiable.

### Allocate Budget By Location Performance, Not Evenly

Spreading budget evenly across locations ignores that locations differ in population, competition, demand, and efficiency. Budget should follow performance and opportunity.

Allocate by:

- measuring cost per visit and cost per sale by location;
- weighting budget toward locations with strong return and headroom;
- reducing spend on locations where local factors make advertising inefficient;
- accounting for local competition intensity and seasonality;
- reviewing allocation regularly rather than setting it once.

A struggling location may need more presence, or it may have fundamental issues that advertising cannot fix. The agent must distinguish between the two.

### Coordinate Local Ads With Local Presence And Operations

Local advertising works only when the underlying presence and operations support it. Ads that drive traffic to a location with poor listings, bad reviews, or stockouts waste the spend.

Coordinate by:

- ensuring the location's profile, hours, and details are accurate before running ads;
- aligning ad messaging with what the location can actually deliver;
- timing campaigns to hours and days when the location can handle traffic;
- preparing staff for promoted offers and expected demand;
- connecting ad performance back to location management.

## Common Traps

### Default Radius Ignoring Real Travel Behavior

Applying the same geographic radius to every business type wastes spend on unreachable audiences or misses the real catchment.

### Over-Broad Geofences

Fences drawn around entire cities or regions are wasted spend labeled as geofencing.

### Overclaiming Attribution Precision

Treating modeled visit estimates as exact proof of ad effect leads to inflated confidence and bad budget decisions.

### Local Inventory Ads With Wrong Feeds

Inaccurate stock feeds send customers to empty shelves and destroy trust in the brand.

### Even Budget Across Unequal Locations

Spreading spend evenly ignores performance and opportunity differences between locations.

### Format-Intent Mismatch

Using awareness formats for ready-to-act intent, or capture formats for awareness, produces poor efficiency.

### Ads Driving Traffic To Unprepared Locations

Promoting a location with poor reviews, wrong hours, or stockouts wastes the ad spend and damages reputation.

## Self-Check

- [ ] Geo-targeting reflects realistic travel behavior for the business type, using drive-time where possible.
- [ ] Ad formats are matched to the intent state they capture, not used interchangeably.
- [ ] Geofences are drawn around genuinely relevant areas, not entire cities or regions.
- [ ] Foot-traffic attribution is understood as modeled data and combined with other signals, not treated as exact proof.
- [ ] Local inventory ads run only with accurate, real-time, location-specific stock feeds.
- [ ] Budget is allocated by location performance and opportunity, not spread evenly.
- [ ] Local ads are coordinated with accurate profiles, hours, and operational readiness.
- [ ] Attribution baselines are established before campaigns to isolate ad effect.
- [ ] Out-of-stock items are excluded from local inventory ads to avoid empty-shelf visits.
- [ ] Location budget decisions distinguish between advertising headroom and fundamental location problems.
