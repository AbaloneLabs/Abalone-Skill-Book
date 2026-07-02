---
name: rendering_strategies.md
description: Use when the agent is choosing how a page should render, deciding between client-side rendering, server-side rendering, static generation, incremental static regeneration, streaming, or edge runtimes, planning a hybrid app where different pages use different modes, reasoning about hydration cost and hydration mismatches, evaluating SEO versus interactivity versus data freshness tradeoffs, or selecting a data-fetching timing that matches the chosen render mode. Also covers first-contentful-paint, time-to-interactive, and interaction-to-next-paint implications, the server-infrastructure cost of SSR, island and selective hydration, and React server components. This skill is the render-mode architecture decision and is distinct from component-level concerns and from HTTP caching mechanics.
---

# Rendering Strategies

The rendering strategy is the decision about *when and where the HTML and its data are produced* — in the browser after the JavaScript loads, on the server on every request, at build time once, on a schedule, progressively as a stream, or at the edge near the user. It is one of the highest-leverage architectural choices in a frontend, because it determines three things that are otherwise very hard to fix later: how fast the first useful pixels appear, whether search engines and link-preview crawlers can see the content, and how fresh the data is when the page loads. Get it wrong and the symptoms are concrete and expensive — a marketing site that ranks poorly because crawlers see an empty shell, a dashboard whose users stare at a spinner on every navigation because it re-fetches after render, a content site that is fast but shows yesterday's prices, or an app that is "modern" but needs a server farm to render every request.

Agents tend to pick a single strategy for the whole app based on the framework's default or on fashion, then live with the mismatch on every page that does not fit. The better mental model is that rendering strategy is a per-page (or per-route, or per-component) decision, governed by a tradeoff matrix among three axes — SEO/crawlability, interactivity/time-to-interactive, and data freshness — plus the cost of the infrastructure each mode requires. A landing page, a logged-in dashboard, and a product page with live inventory are not the same problem and should rarely use the same mode.

This skill is about the architecture-level mode decision and the data-fetching timing it implies. It is distinct from component-level rendering concerns (covered in component-architecture-and-state) and from the HTTP caching mechanics that govern how responses are stored (a separate concern about cache headers and CDN behavior). It assumes those are handled elsewhere and focuses on choosing the render mode and living with its consequences.

## Core Rules

### Know Each Strategy's Data-Fetching Timing And Tradeoff

Each rendering mode is defined by *when the data is fetched and the HTML produced*, and that timing dictates the tradeoffs. Know the full set before choosing.

- **CSR (client-side rendering).** The server sends a near-empty shell; the browser downloads JavaScript, runs it, then fetches data and renders. Fast once loaded and trivial to host as static files, but the first useful paint waits for JS plus the data fetch, and crawlers that do not execute JS see an empty page. Good for logged-in, highly interactive apps behind auth; poor for content that must be indexed or must paint fast on slow devices.
- **SSR (server-side rendering, per request).** The server fetches data and produces HTML on every request. Crawlers see full content, the first paint is real content, and data is always fresh — but every request pays server compute and a time-to-first-byte tied to the slowest data fetch, and the server must run your app at scale. Good for dynamic, personalized, or frequently-changing content; expensive for high-traffic static-ish pages.
- **SSG (static site generation, at build time).** HTML is generated once at build time and served as static files from a CDN. Fastest to serve, cheap to host, fully crawlable — but data is frozen at build time and updates require a rebuild. Good for marketing, docs, and content that changes rarely; wrong for anything that must reflect live data.
- **ISR (incremental static regeneration).** Static pages that are regenerated in the background on a schedule or on-demand, serving the stale version until the new one is ready. A middle ground between SSG's speed and SSR's freshness. Good for large content sites where pages change occasionally but not on every view.
- **Streaming SSR / server components.** The server sends HTML in chunks as each piece becomes ready, so the browser paints the shell and fast parts immediately while slower data streams in. Reduces the "wait for the slowest query" cost of classic SSR and lets you mix static and dynamic regions on one page. Good when a page has both fast and slow data dependencies.
- **Edge rendering.** Rendering at CDN edge locations close to the user, lowering latency. Compatible with SSR/SSG/streaming; constrains what runtime APIs you can use. Good for latency-sensitive global apps; requires code that runs in the restricted edge runtime.

The decisive question for each page: *when must the data be fresh, who needs to read the HTML, and how much server cost is acceptable?* Match the mode to those answers, not to a framework default.

### Treat Rendering Strategy As A Per-Page Decision, Not An App-Wide One

The most common architectural error is choosing one mode for the entire application. Real products have pages with wildly different needs, and a good architecture lets each route pick its mode.

- **Marketing and landing pages → SSG.** Content is known at build time, must paint fast, and must be crawlable. Staleness is fine.
- **Logged-in dashboards and app shells → CSR (or a thin SSR shell).** Content is personalized, behind auth, and highly interactive; crawlers are irrelevant and the data is fetched live anyway. A static shell plus client data-fetching is often the right shape.
- **Dynamic, personalized, or frequently-updated content → SSR or ISR.** Content must be fresh and crawlable. Use SSR when it changes per request or per user; use ISR when it changes occasionally and stale-while-revalidate is acceptable.
- **Hybrid shells → streaming / partial pre-rendering.** A page that is mostly static with one live region (a product page with real-time stock) can pre-render the static parts and stream only the dynamic part.

A mature app commonly mixes three or four of these across its routes. Choose the framework and architecture that permit per-route modes, and document each route's chosen mode so the decision is visible.

### Account For The Cost Of Hydration

SSR and SSG are not "free fast pages." The server sends HTML, but to become interactive the browser must still download the JavaScript, parse it, and "hydrate" the server HTML into an interactive tree. This is double work — the page is rendered once on the server and effectively again on the client — and it has real costs.

- **Hydration is expensive on low-powered devices.** A page that paints fast from the server can still be non-interactive for seconds while hydration runs, especially on mobile. Time-to-interactive can lag first-contentful-paint badly.
- **Hydration mismatches break the page.** If the HTML the server produces differs from what the client would produce (different theme, random ID, time-dependent value, client-only data), the framework warns, discards the server HTML, and re-renders — defeating the purpose and causing flicker. Server and client must produce identical markup for the hydrated tree.
- **Selective and island hydration reduce the cost.** Instead of hydrating the whole page, hydrate only the interactive islands (the framework's term) and leave static regions as plain HTML. This is the core idea behind islands architecture and partial hydration: ship interactivity only where it is needed.
- **Server components shift work back to the server.** Components that render only on the server ship zero hydration JavaScript for their own rendering; only the interactive client components pay hydration cost. This lets a page be dynamic on the server without forcing the whole tree through hydration.

Plan hydration deliberately: know which regions must be interactive, prefer selective/island hydration or server components for the rest, and eliminate sources of mismatch.

### Map The SEO-Interactivity-Freshness Tradeoff Explicitly

The three axes pull against each other, and pretending you can have all three on every page leads to bad choices. Make the tradeoff explicit per page.

- **SEO / crawlability** wants pre-rendered HTML at request or build time, so crawlers and link previews see content. CSR hurts this.
- **Interactivity / time-to-interactive** wants minimal JavaScript and hydration, which favors SSG and static shells. Heavy SSR with full hydration can hurt TTI even as it helps first paint.
- **Data freshness** wants rendering as close to the request as possible (SSR or on-demand ISR), which fights the static speed of SSG and adds server cost.

A page rarely wins on all three. A logged-in dashboard trades SEO away entirely (it is behind auth) to win on interactivity and freshness. A marketing page trades freshness away to win on SEO and speed. A product page tries to win SEO and freshness and accepts some hydration cost. State the priority order for each page before choosing the mode.

### Tie Data-Fetching Timing To The Render Mode

The data-fetching strategy is not independent of the render mode — it is determined by it, and mixing them up produces the worst outcomes.

- **SSG → fetch at build time.** Data is baked in; updates need a rebuild or ISR.
- **SSR → fetch on the server before rendering.** The render waits for the data; parallelize independent fetches and avoid serial waterfalls that inflate TTFB.
- **CSR → fetch in the browser after the JS loads.** Use a server-state library for caching, loading states, and deduplication; fetch at route boundaries, not in deep leaf components.
- **Streaming → fetch per region.** Each streamed region can fetch independently and flush when ready, so a slow query does not block the fast ones.
- **Server components → fetch on the server, colocated with the component that needs the data.** Avoid the client-side waterfall by fetching where the data is consumed on the server.

The classic failure is choosing SSR for SEO but then fetching the data on the client after hydration, so the server renders an empty shell that crawlers see anyway — you pay the SSR cost and get none of the SEO benefit. If you chose SSR for crawlability, the data must be fetched on the server.

### Reason About Performance With The Right Metrics

Different modes optimize different metrics, and optimizing the wrong one wastes effort. Know which metric each mode moves.

- **TTFB (time to first byte)** is dominated by server work in SSR (the server must fetch and render before responding) and is near-zero for SSG from a CDN. SSR's freshness costs TTFB.
- **FCP / LCP (first contentful / largest contentful paint)** benefits from server-rendered or static HTML; CSR hurts it because content waits for JS plus data.
- **TTI (time to interactive)** is hurt by large JS bundles and full hydration, even when FCP is good. A page can paint fast and still feel sluggish.
- **INP (interaction to next paint)** reflects the responsiveness of interactions after load; it is driven by main-thread blocking from hydration and from heavy client logic.

Choose the mode that improves the metric that matters for the page's goal. A content page cares about LCP and crawlability; an interactive app cares about TTI and INP. Do not optimize FCP on a page whose users are blocked by hydration.

### Price In The Server-Infrastructure Cost

SSR, ISR, streaming, and server components all require a server that runs your app — and that server is a real, scaling cost and a real operational burden. SSG and CSR can be served as static files from a CDN for nearly nothing.

- **SSR scales with traffic and with data-fetch latency.** Every request runs server compute and waits on data; a slow upstream API multiplies your server cost and your TTFB.
- **ISR amortizes the cost** by regenerating rarely, but still needs a server for regeneration.
- **Edge runtimes reduce latency but constrain APIs** and can complicate dependencies that assume a Node environment.
- **Static is cheapest and most resilient.** When a page can be static, the operational argument for making it static is strong.

Do not choose SSR by default "because it is more powerful." Choose it when the freshness or SEO requirement justifies the server cost, and prefer SSG or ISR where the content allows. A surprising amount of "dynamic" content is actually static-with-occasional-updates and is far cheaper as ISR.

## Common Traps

### Picking One Render Mode For The Whole App

Defaulting the entire application to CSR or SSR based on the framework's boilerplate, then fighting the mismatch on every page that does not fit. Treat the mode as a per-route decision driven by the tradeoff matrix.

### SSR For SEO But Fetching Data On The Client

Rendering the shell on the server for crawlability, then doing the actual data fetch in the browser after hydration — so crawlers see an empty shell and you paid the SSR cost for nothing. If SSR was chosen for SEO, fetch the data on the server.

### Ignoring Hydration Cost On Low-Power Devices

Shipping a fully-hydrated SSR page and assuming fast first paint means a fast experience. On mobile, hydration can block interactivity for seconds after paint. Prefer selective/island hydration or server components for non-interactive regions.

### Hydration Mismatch From Client-Only Or Time-Dependent Values

Rendering a theme, a locale, a random value, or a timestamp differently on the server and client, triggering a hydration mismatch that discards the server HTML. Ensure the hydrated tree is byte-identical, or gate the differing value behind a client-mounted check.

### Optimizing FCP While Users Wait On TTI

Celebrating a fast first-contentful-paint on a page whose users cannot interact for seconds because hydration is blocking the main thread. Track TTI and INP, not just paint metrics.

### Choosing SSR By Default And Paying For Static Content

Running a server farm to render pages whose content changes once a week, when SSG plus ISR would be faster, cheaper, and more resilient. Make content static-by-default and dynamic only where freshness requires it.

### Serial Data Fetches Inflating TTFB

In SSR, fetching data sequentially (parent waits for its fetch, then child waits for its own) when the fetches are independent and could run in parallel, turning a 200ms page into an 800ms one. Parallelize independent server-side fetches.

### Assuming Edge Fixes Everything

Moving rendering to the edge to reduce latency without checking that dependencies run in the edge runtime, then discovering Node-only libraries break at deploy. Validate the runtime constraints before committing to edge.

## Self-Check

- [ ] Each page's render mode (CSR, SSR, SSG, ISR, streaming, edge) was chosen explicitly against the SEO-interactivity-freshness tradeoff and the server-cost budget, not inherited from a framework default — and the chosen mode is documented per route.
- [ ] Marketing/landing pages are SSG (or ISR), logged-in dashboards/app shells are CSR or a thin shell, and dynamic/personalized content is SSR or ISR, with a hybrid/streaming approach used where a page mixes static and live regions.
- [ ] The data-fetching timing matches the render mode — SSG fetches at build, SSR fetches on the server before render with independent fetches parallelized, CSR fetches at route boundaries via a server-state library, and streaming fetches per region — with no SSR-for-SEO page that fetches its data on the client.
- [ ] Hydration cost was planned deliberately: non-interactive regions use selective/island hydration or server components to avoid shipping and running hydration JavaScript for static content.
- [ ] The hydrated tree is byte-identical between server and client — no client-only values, random IDs, time-dependent output, or persisted theme/locale rendered into the initial markup without a mounted guard — so there are no hydration mismatches.
- [ ] The performance metric being optimized is the one that matters for each page's goal (LCP and crawlability for content, TTI and INP for interactive apps), and FCP is not being celebrated on a page whose users are blocked by hydration.
- [ ] The server-infrastructure cost of SSR/ISR/streaming was priced in and justified by a freshness or SEO requirement, and content that could be static is SSG/ISR rather than paying per-request server compute.
- [ ] If edge rendering is used, all dependencies were verified to run in the edge runtime, and the latency gain is not offset by runtime-constraint breakage.
