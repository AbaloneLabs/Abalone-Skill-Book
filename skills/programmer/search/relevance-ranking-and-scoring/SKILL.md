---
name: relevance_ranking_and_scoring.md
description: Use when the agent is tuning search relevance, designing a scoring function, choosing or configuring BM25/TF-IDF, applying field or document boosts, blending full-text relevance with business signals (recency, popularity, price, margin, personalization), building learning-to-rank models, debugging "the wrong results rank first," or resolving conflicts between relevance and revenue/conversion. Also covers function_score and script_score patterns, the dangers of hand-tuning boosts by feel, the gap between offline relevance metrics (nDCG, MRR, precision@k) and online behavior, and the recurring mistake of optimizing a ranking that looks right on a few queries but degrades the long tail.
---

# Relevance Ranking And Scoring

Ranking is the part of search that users actually feel: given a query, which ten results come first. A search engine's default relevance (usually BM25) is a strong, well-understood baseline for *textual* relevance — it rewards documents that contain the query terms frequently and rarely elsewhere. But almost no production search ranks on text alone. Real rankings blend textual relevance with business signals: how fresh the document is, how popular it is, its price, its margin, the seller's trust score, the user's personalization vector. The judgment problem is how to combine these signals without destroying the user's trust that results actually match what they typed. Boost the business signals too hard and the top result for `red shoes` becomes a popular blue shirt; boost them too little and the ranking ignores everything the business knows about quality.

Agents tend to miss the subtleties because tuning feels intuitive — "make popular items rank higher" — and because the top results on the three queries the developer tested look right. The harm is in the long tail and in the conflicts. A relevance metric (nDCG) improves while revenue per search falls, because the new ranking surfaces informative-but-free content over purchasable products. A popularity boost helps head queries and destroys rare queries, where popularity is zero and the boost becomes noise. A field boost set by feel to "make titles matter more" over-weights keyword-stuffed titles and pushes genuine matches down. The judgment problem is to treat ranking as a measurable, instrumented system with explicit goals, not a set of magic numbers tuned until the demo queries look acceptable.

This skill covers scoring functions, boosting, business-signal blending, relevance measurement, and the relevance-versus-business-objective conflict. It complements the search-index-design skill (which covers what gets indexed) and the experiment-metrics skill (which covers measuring online impact).

## Core Rules

### Start From BM25 And Justify Every Departure

BM25 is the default textual relevance function in most engines for good reason: it is term-frequency-based with saturation (a term's score contribution flattens as it repeats, so keyword stuffing is self-limiting) and length normalization (it does not punish long documents as harshly as raw TF). Treat it as the baseline and depart from it only with a reason:

- **BM25 parameters (`k1`, `b`).** `k1` controls term-frequency saturation; `b` controls length normalization (0 = ignore length, 1 = full normalization). Defaults (`k1≈1.2`, `b≈0.75`) are sensible for general text; tune only with offline metrics on a query set, never by eye.
- **Know when BM25 is insufficient.** It measures textual match only. It cannot know that a result is outdated, unpopular, out of stock, or a poor business fit. Every business signal you need is a justified departure from pure BM25.
- **Do not reinvent textual relevance.** A hand-rolled TF-IDF or count-based score is almost always worse than the engine's tuned BM25. Spend effort on blending business signals, not replacing the text score.

Write down, for your ranking: what is the base text score, and what does each additional signal add? If you cannot articulate why a signal is there, it is probably noise.

### Blend Business Signals Explicitly, Not By Hidden Weight

When you add recency, popularity, or personalization, make the blend explicit and bounded. The two main patterns:

- **Linear blend (multiplicative or additive on top of the text score).** `final = text_score * (1 + α * popularity_norm) + β * recency_norm`. Simple, interpretable, easy to tune, but sensitive to the scales of the inputs — normalize every signal to a comparable range (0–1) before blending.
- **Multiplicative gating (function_score).** Multiply the text score by a function of the business signal, so a result that does not match the text cannot rank high no matter how popular. This preserves the "results must match the query" guarantee, which additive blends can violate.

Strong practice: normalize every signal, start with small weights, and gate so that a non-matching document cannot outrank a matching one. Weak practice: dumping raw numeric fields (raw price, raw view count) into a script score where their unbounded magnitude dominates the text score and one viral document drowns everything else.

### Tune Boosts Against A Query Set With Offline Metrics, Not By Eye

Boost tuning by feel is the most common ranking failure. "Titles should matter more" becomes `title^10`, which over-weights any document with the query word in the title regardless of body match. Tune against evidence:

- **Build a judgment set (relevance judgments / qrels).** A set of representative queries, each with a list of documents rated for relevance (e.g., 0–4). Include head queries, tail queries, ambiguous queries, and queries with known failure modes — not just the three you remember.
- **Measure with rank-aware metrics.** nDCG (normalized discounted cumulative gain) rewards relevant results near the top and tolerates graded relevance; MRR (mean reciprocal rank) rewards the first relevant result; precision@k measures the share of relevant results in the top k. Use nDCG as the primary metric for web-style ranking; use precision@k or recall@k when the user scans a fixed window.
- **Tune one parameter at a time against the metric.** Change a boost, recompute nDCG across the whole judgment set, and keep the change only if the metric improves overall — not just on the query you were staring at. A change that helps one query and hurts a hundred is a regression.
- **Watch the long tail.** Head queries are easy; the long tail of rare queries is where ranking quietly breaks. Ensure the judgment set includes tail queries, or a boost that helps head queries will silently degrade them.

### Resolve The Relevance-Versus-Business Conflict Deliberately

This is the central tension of production search. Pure relevance serves the user's stated intent; business signals serve the platform's goals (revenue, conversion, margin, retention). They conflict constantly:

- **Sponsored vs organic.** Paid placements must be visually distinct and bounded; blurring them erodes trust and may violate regulation. Decide the slot allocation (top N organic, then sponsored, or interleaved) and label clearly.
- **Relevance vs revenue.** Boosting high-margin items can push the most relevant results down. Decide the tradeoff explicitly: is this a relevance-first search with light revenue boost, or a revenue-optimized feed with a relevance floor? Both are legitimate; hiding the choice is not.
- **Relevance vs conversion.** A popular item converts well but may not match a specific query. A pure conversion-optimized ranking returns bestsellers for every query and feels broken to users searching for something specific. Gate conversion signals behind a minimum relevance threshold.
- **Personalization vs consistency.** Personalized ranking helps repeat users and confuses users who expect stable results. Decide what is personalized (ranking within a relevant set) vs what is invariant (whether a result appears at all).

State the ranking's objective explicitly: "maximize relevance, subject to a revenue floor," or "maximize conversion, subject to a relevance threshold." An unstated objective becomes an implicit one that drifts toward whatever the last tuner optimized.

### Use Learning-To-Rank When The Signal Combination Is Complex

When the blend involves many signals with nonlinear interactions (text score, recency, popularity, personalization, freshness, quality), hand-tuned weights plateau. Learning-to-rank (LTR) trains a model to rank from features:

- **Requires labeled data.** Either explicit judgments (the judgment set) or implicit signals (clicks, conversions, add-to-cart) mined from logs. Implicit signals are abundant but biased (position bias: users click what ranks high, reinforcing the current ranking).
- **Handles nonlinearity the linear blend cannot.** An LTR model can learn that recency matters more for news queries and less for evergreen reference content; a fixed weight cannot.
- **Is harder to debug and govern.** A model that learned a shortcut (e.g., "always rank the highest-priced item first") optimizes the training signal while harming users. Inspect feature importances and slice performance, not just the aggregate metric.
- **Needs online validation.** Offline nDCG improvement does not guarantee online improvement; A/B test the model against the incumbent on real traffic measuring the business outcome.

Reach for LTR when hand-tuning has plateaued and you have the labeled data and infrastructure to maintain it. Do not reach for it as a first step — a well-tuned BM25-plus-boostes baseline beats a poorly-trained LTR model and is far easier to operate.

### Make Ranking Debuggable And Observable

Ranking bugs ("why does this rank first?") are unanswerable without per-document score breakdowns. Build observability in from the start:

- **Expose the score explanation.** Every result should carry a breakdown of which signals contributed how much (text score, each boost, each function). This is the engine's `explain` feature; surface it in internal tooling.
- **Log queries and their top results.** A query log with the top-k results and their scores lets you reproduce and diagnose ranking regressions after the fact.
- **Monitor ranking metrics over time.** Track nDCG on a fixed judgment set, click-through rate, zero-result rate, and reformulation rate (users re-querying immediately, a signal of poor results). A ranking change that improves nDCG but spikes reformulation rate has traded a metric for user frustration.

## Common Traps

### Hand-Tuning Boosts Until The Demo Queries Look Right

Setting `title^10` and `description^2` because the three queries you checked look acceptable, then shipping a ranking that destroys the long tail. Tune against a judgment set with offline metrics across head and tail queries.

### Blending Unnormalized Business Signals

Adding raw `view_count` or raw `price` to a script score, so one viral document or one expensive item dominates every query. Normalize every signal to a comparable range before blending.

### Letting Business Boosts Override Textual Match

Boosting popularity so hard that a popular-but-irrelevant item outranks a precise textual match, making search feel broken. Gate business signals multiplicatively so a non-matching document cannot outrank a matching one.

### Optimizing nDCG While Revenue Or Conversion Falls

Improving the offline relevance metric while the business outcome degrades, because relevance and revenue genuinely conflict and the metric does not capture revenue. Define the ranking objective explicitly and validate business metrics online.

### Ignoring Position Bias In Implicit Feedback

Training an LTR model on raw clicks, which are biased toward already-high-ranked results, so the model learns to reproduce the current ranking. Debias clicks (position-bias correction, randomized exploration) or use explicit judgments.

### Overfitting A Learning-To-Rank Model To The Judgment Set

A model that achieves excellent nDCG on its training judgments but generalizes poorly, because it learned a labeling artifact. Hold out judgments, inspect feature importances, and validate online.

### Treating Sponsored And Organic As The Same Ranking

Blurring paid placements into organic results to maximize clicks, eroding user trust and risking regulatory issues. Separate, label, and bound sponsored slots explicitly.

### Reporting Only Aggregate Metrics, Hiding Slice Regressions and changing The Ranking Without A Reversible Rollout

An nDCG improvement that comes from helping head queries while quietly degrading rare queries, invisible in the aggregate. Slice metrics by query type (head/tail, navigational/informational) and inspect the worst-affected queries.

Shipping a new ranking to 100% of traffic with no rollback path, then discovering a regression days later when metrics settle. Roll out ranking changes via A/B test with guardrail metrics and a rollback plan.

## Self-Check

- [ ] The base text score is the engine's BM25 (or an explicitly justified alternative), and every additional signal (recency, popularity, personalization, margin) has a stated reason for being included.
- [ ] Business signals are normalized to a comparable range before blending and gated so that a document that does not match the query text cannot outrank one that does.
- [ ] Boosts and weights were tuned against a judgment set spanning head and tail queries, measured with rank-aware offline metrics (nDCG, MRR, precision@k), not by eye on a few demo queries.
- [ ] The relevance-versus-business conflict is resolved explicitly: the ranking objective is stated (relevance-first with a revenue floor, or conversion-optimized with a relevance threshold), and sponsored placements are separated, labeled, and bounded.
- [ ] If learning-to-rank is used, it was reached for after hand-tuning plateaued, trained on debiased or explicit data, validated on held-out judgments, inspected for shortcut-learning, and A/B tested online.
- [ ] Ranking is debuggable and observable: per-document score explanations are available in internal tooling, queries and top-k results are logged, and ranking metrics (nDCG, zero-result rate, reformulation rate) are monitored over time.
- [ ] Ranking changes roll out via A/B test with guardrail metrics and a rollback path, not as an irreversible full-traffic cutover.
- [ ] The highest-risk cases were verified — a popularity boost overriding textual match, unnormalized signals dominating, nDCG improving while revenue falls, long-tail regressions hidden by aggregate metrics, and position-biased implicit training data — not only the few demo queries that look right.
