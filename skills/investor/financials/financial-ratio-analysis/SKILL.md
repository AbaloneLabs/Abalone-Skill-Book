---
name: financial_ratio_analysis.md
description: Use when the agent is computing and interpreting financial ratios (profitability, liquidity, leverage, efficiency, valuation), comparing ratios across companies and over time, judging whether ratio improvements reflect real performance or accounting distortions, or reviewing the limitations of ratio analysis including industry differences, accounting policy effects, and the risk of reading a single ratio in isolation.
---

# Financial Ratio Analysis

Financial ratio analysis distills the three statements into standardized metrics that compare companies of different sizes, track performance over time, and benchmark against peers. Ratios are the common language of financial analysis: profitability, liquidity, leverage, efficiency, and return. The judgment problem is that ratios are deceptively easy to compute and dangerously easy to misread. A ratio is a ratio of two numbers, each of which carries accounting distortions, and the comparison context (industry, accounting policy, business model, cycle position) determines whether a given value is good, bad, or meaningless. Agents tend to compute a table of ratios, compare to a generic benchmark, and declare a company strong or weak, missing that the same ratio means opposite things in different industries, that ratio improvement can signal deterioration, and that ratios are lagging outputs that say little about the future. The skill is using ratios as a structured starting point for questions, not as answers.

This skill is for applying ratio analysis with awareness of context and distortion.

## Core Rules

### Group Ratios By The Question They Answer

Ratios are not a flat list. They answer distinct questions, and grouping them prevents scattered, meaningless comparison.

Organize by purpose:

- profitability (gross, operating, net margin, return on equity, return on assets, return on invested capital): how much profit per dollar of revenue or capital?
- liquidity (current, quick, cash ratio): can the firm pay near-term obligations?
- solvency and leverage (debt-to-equity, debt-to-capital, interest coverage, debt-to-EBITDA): is the capital structure sustainable?
- efficiency (asset turnover, inventory turnover, receivables days, payable days): how well are assets and working capital managed?
- valuation (price-to-earnings, price-to-book, EV-to-EBITDA, dividend yield): how is the market pricing the business?

A ratio is only useful if you know which question it answers. Mixing valuation ratios with liquidity ratios in one table obscures both.

### Always Compare Against The Right Benchmark

A ratio in isolation is meaningless. A 5% net margin is excellent for a grocer and disastrous for a software firm. The comparison context is the analysis.

Use multiple benchmarks:

- the company's own history, to detect trend and cyclicality;
- direct peers with similar business models and accounting policies;
- the industry median and range, not a single average that outliers distort;
- the company's own prior guidance and targets.

Choose peers carefully. Comparing a capital-light licensor to a capital-heavy manufacturer on return-on-assets produces nonsense. The benchmark must match the business model and capital structure.

### Trace Ratios To Their Underlying Drivers

A ratio is an output; the analysis is in the drivers. Return on equity decomposed into margin, asset turnover, and leverage (the DuPont framework) reveals why ROE changed, not just that it did.

Decompose:

- ROE into net margin, asset turnover, and financial leverage, to see whether improvement came from operations or from taking more debt;
- ROIC into operating margin and invested capital turnover, the cleanest measure of operating performance;
- the cash conversion cycle (receivable days plus inventory days minus payable days) into its components;
- gross margin into price, volume, mix, and input cost effects.

A rising ROE driven purely by adding leverage is not operational improvement; it is increased risk. Decomposition separates genuine performance from financial engineering.

### Adjust For Accounting Distortions Before Comparing

Two companies with identical economics can show very different ratios because of accounting choices. Naive comparison across them is misleading.

Adjust for:

- depreciation method and useful lives (accelerated versus straight-line distorts margins and returns);
- capitalization policies (capitalizing costs inflates profit and assets);
- lease accounting (operating versus finance leases change leverage and returns);
- goodwill and intangibles from acquisitions (inflate the asset base and depress ROA);
- one-time items, restructuring, and write-downs that distort a single year.

Where possible, restate to a comparable basis, or compare ratios that are less distorted (EV-to-EBITDA is less affected by capital structure than price-to-earnings). Always note which accounting choices drive the differences.

### Distinguish Cyclical From Structural Ratio Movement

Many ratios move with the business cycle, and a ratio that looks terrible at the cycle trough may be normal for the industry. Confusing cyclical lows with structural decline leads to wrong conclusions.

Assess:

- where the company and industry are in the cycle (peak margins often mean-revert);
- normalized versus current ratios, averaging over a full cycle to remove cyclical distortion;
- whether margin or return change reflects a structural shift (new competition, disruption) or a cyclical swing;
- the sustainability of current ratios given capacity, pricing power, and cost structure.

A commodity producer showing record margins at the cycle peak is not a structural improvement. Use normalized, through-cycle ratios for valuation and structural judgment.

### Read Ratios As A Coherent System, Not In Isolation

Ratios interrelate, and a single ratio can mislead. A strong current ratio might hide slow inventory turning to obsolescence; high ROE might hide excessive leverage; rising margins might hide falling volume.

Cross-check:

- liquidity with efficiency (high current ratio plus falling inventory turnover signals stale stock);
- profitability with leverage (high ROE plus rising debt-to-equity signals risk, not strength);
- margins with turnover (low-margin businesses can be attractive if asset turnover is high);
- growth with cash conversion (growing revenue plus deteriorating cash flow signals low-quality growth).

Build a coherent story across the ratio groups. Contradictions between ratios are where the real insight and the real risks live.

### Recognize What Ratios Cannot Tell You

Ratios are backward-looking, accounting-based summaries. They do not capture strategy, management quality, competitive position, disruption risk, or the future. Treating a strong ratio set as proof of a good investment is a category error.

Remember:

- ratios describe the past; valuation and investment are about the future;
- accounting numbers reflect choices and estimates, not pure economics;
- ratios say nothing about moat, disruption, regulation, or management;
- a company can have excellent historical ratios and a deteriorating future.

Use ratios to structure questions and identify where to dig deeper, not to deliver a verdict. The strongest analysis uses ratios as the entry point to qualitative investigation.

## Common Traps

### Comparing Ratios Across Unlike Business Models

A ratio means different things in different industries. Comparing a bank's leverage to a manufacturer's, or a licensor's asset turnover to a retailer's, is meaningless.

### Reading A Single Ratio In Isolation

One ratio can mislead. Strong liquidity can hide obsolete inventory; high ROE can hide excessive leverage. Read ratios as a system.

### Ignoring Accounting Policy Differences

Depreciation, capitalization, leases, and goodwill create differences unrelated to economics. Compare on a comparable basis or note the distortion.

### Confusing Cyclical Peaks With Structural Improvement

Record margins at a cycle peak often mean-revert. Use through-cycle, normalized ratios for structural judgment.

### Treating Ratio Improvement As Always Good

ROE rising from added leverage is more risk, not better operations. Margin rising from deferred maintenance is future cost, not efficiency. Trace to drivers.

### Over-Optimizing On A Ratio

Management can game a single ratio (cut R&D to boost current margin, buy back stock with debt to boost EPS) at the expense of the business. Watch for ratio management.

### Using Outdated Or Averaged Benchmarks

Industry averages distorted by outliers, or stale peer data, produce wrong conclusions. Use current, median-based, like-for-like peers.

### Forgetting Ratios Are Lagging

Ratios describe the past. A strong ratio set does not protect against disruption, poor strategy, or a deteriorating competitive position.

## Self-Check

- [ ] Ratios are grouped by the question they answer (profitability, liquidity, leverage, efficiency, valuation), not presented as a flat undifferentiated list.
- [ ] Each ratio is compared against the company's history, carefully chosen like-for-like peers, and industry medians, with benchmarks matched to the business model.
- [ ] Key ratios (ROE, ROIC, cash conversion cycle, gross margin) are decomposed into their drivers to distinguish operational performance from financial engineering.
- [ ] Accounting distortions (depreciation, capitalization, leases, goodwill, one-time items) are adjusted for or noted before comparison.
- [ ] Cyclical versus structural movement is distinguished, with through-cycle normalized ratios used where the business is cyclical.
- [ ] Ratios are read as a coherent system, with contradictions between ratio groups investigated rather than ignored.
- [ ] The analysis acknowledges that ratios are backward-looking, accounting-based, and silent on strategy, moat, disruption, and management, and uses them to structure questions rather than deliver a verdict.
- [ ] The recommendation states that ratio analysis has inherent limitations, that accounting choices and estimates affect all inputs, that ratios do not predict future performance, and that this is not investment advice and professional financial expertise may be warranted for complex situations.