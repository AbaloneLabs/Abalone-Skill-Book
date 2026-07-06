---
name: market_segmentation.md
description: Use when the agent is segmenting a market, choosing segmentation variables, forming and validating segments, sizing segments, or deciding how many segments are meaningful without overfitting or reifying arbitrary groups.
---

# Market Segmentation

Segmentation is the attempt to divide a heterogeneous market into groups that differ meaningfully in needs, behavior, or response. Done well, it helps target products, messages, and strategies to real differences among consumers. Done poorly, it manufactures groups that do not exist, chases statistical artifacts, or freezes dynamic markets into rigid categories. When segmentation is mishandled, three harms follow. Resources are spent targeting segments that are not real or not actionable. Strategies are built on segments that do not differ in the ways that matter for the decision. And consumers are boxed into categories that distort their actual, often fluid, behavior. Segmentation is a hypothesis about structure in the market, and it deserves the same rigor as any hypothesis.

The agent should use this skill when forming segments, choosing variables, deciding the number of segments, validating them, or translating segments into strategy. The goal is to keep the agent from treating clustering output as ground truth, when segments are model-derived constructions whose usefulness must be argued and tested, not assumed.

## Core Rules

### Start From The Purpose, Not The Algorithm

Segmentation serves a decision. Define the purpose before choosing variables or methods.

- Targeting product features to different need groups.
- Choosing communication channels for different audiences.
- Pricing differently across price-sensitive segments.
- Identifying an underserved or high-value group.
- Allocating sales or service resources.

Different purposes call for different segmentation bases. Segmentation for product design needs needs-based variables; segmentation for media planning needs behavioral or media-use variables. A segmentation with no clear purpose produces segments no one can act on.

### Choose Variables That Match The Purpose

The variables define what the segments mean. Choose them to align with the decision.

- Demographics, age, income, geography, for broad targeting.
- Psychographics and needs, for product and message design.
- Behavior, usage, loyalty, occasions, for retention and cross-sell.
- Attitudes and values, for positioning.
- Profitability or value, for resource allocation.

Mixing variables without thought produces segments that are hard to interpret. A segment defined by a grab-bag of variables tells no coherent story. Align variables to the purpose and justify each.

### Avoid Reifying Algorithmic Output

Clustering algorithms will produce groups from almost any data. The existence of clusters in output does not mean the segments are real, meaningful, or stable.

- Clusters can reflect scaling, outliers, or noise rather than true structure.
- The number of clusters is often an arbitrary choice.
- Different algorithms produce different segments from the same data.
- Segment membership can be unstable under resampling.

Treat segments as a useful simplification, not a discovered truth. Test their robustness to method choices and resampling before acting on them.

### Validate Segments Against External Criteria

Segments are more credible if they differ on variables not used to form them.

- Do segments differ in actual behavior, such as purchase or retention?
- Do they differ in response to marketing or product features?
- Do they differ in profitability or value?
- Do they replicate in a new sample or over time?

Validation against external outcomes tests whether the segments matter for the decision. Segments that differ only on the variables used to create them may be circular and useless.

### Decide The Number Of Segments Deliberately

The number of segments is a judgment, not a fact. Choose it with attention to the following.

- Statistical criteria, such as fit indices, treated as guidance not command.
- Interpretability, can each segment be named and described meaningfully.
- Actionability, can the business actually target that many segments.
- Stability, do the same segments reappear under resampling.
- Size, are segments large enough to matter.

A solution with eight segments may fit better statistically but be unmanageable and unstable. A solution with three may be actionable and robust. Balance fit against usefulness.

### Size Segments Realistically

Segment size estimates depend on the sample and how it represents the market.

- Weight the sample to the target population if needed.
- Report segment sizes with uncertainty, not false precision.
- Consider whether small but valuable segments justify attention despite size.
- Avoid presenting point estimates of size as exact.

A segment estimated at 12 percent of the market may really be 8 or 16. Report ranges and acknowledge the uncertainty, especially for high-stakes sizing decisions.

### Account For Segment Stability And Dynamics

Markets change, and so do segments. Treat segments as potentially dynamic.

- Re-test segmentation periodically rather than freezing it for years.
- Recognize that consumers may move between segments over time or context.
- Consider whether segments reflect enduring differences or transient states.
- Track segment migration in longitudinal data.

A segmentation frozen at one moment may mislead strategy as the market evolves. Build in reassessment.

### Ensure Segments Are Actionable

A segment is only useful if the organization can act on it.

- Can the segment be identified and reached?
- Is there a distinct strategy, product, or message for it?
- Does the organization have the capacity to serve multiple segments?
- Is the cost of segment-specific strategy justified by the benefit?

Beautiful segments that cannot be targeted or served add complexity without value. Filter segmentation through the lens of what the organization can actually do.

### Examine Overlap And Fuzzy Boundaries and connect Segmentation To Strategy Explicitly

Real consumers often do not fit cleanly into one segment.

- Consider overlapping or fuzzy membership rather than hard assignment.
- Recognize that boundary cases may behave differently.
- Avoid forcing every consumer into a single box.
- Use probabilistic assignment where appropriate.

Hard segment assignment can distort strategy by ignoring consumers who span categories. Fuzzy approaches often match reality better.

Segmentation earns its cost only when it changes decisions. Make the connection explicit.

- What product, message, price, or channel decision follows from each segment?
- How does the segmentation change what the organization will do?
- What is the expected benefit of segment-specific strategy versus a single strategy?

A segmentation that confirms what the organization already does adds little. The value is in differential strategy grounded in real segment differences.

## Common Traps

### Clustering Without A Purpose

Segments without a decision context cannot be acted on. Start from purpose.

### Grab-Bag Variables

Mixing unrelated variables produces uninterpretable segments. Align variables to purpose.

### Treating Clusters As Discovered Truth

Algorithmic output is not ground truth. Test robustness and validity.

### Circular Validation

Segments that differ only on clustering variables are not validated. Test against external outcomes.

### Choosing Segment Count By Fit Alone

Best statistical fit may be unmanageable. Balance fit with interpretability and actionability.

### False Precision In Sizing

Point estimates of segment size hide uncertainty. Report ranges.

### Freezing Segments Over Time

Markets change. Reassess segmentation periodically.

### Non-Actionable Segments

Segments that cannot be reached or served add complexity without value. Ensure actionability.

## Self-Check

- Is the segmentation purpose defined before variables and methods are chosen?
- Do the segmentation variables align with the decision the segmentation serves?
- Are segments treated as model-derived simplifications, tested for robustness rather than assumed true?
- Are segments validated against external criteria, such as behavior, response, or profitability?
- Is the number of segments chosen balancing fit, interpretability, actionability, and stability?
- Are segment sizes reported with uncertainty and appropriate population weighting?
- Is segment stability and dynamics over time considered, with reassessment planned?
- Are segments actionable, reachable, and worth segment-specific strategy?
- Is overlap and fuzzy membership considered rather than forcing hard assignment?
- Is the connection from segmentation to differential strategy made explicit?; for segmentation driving major investment or organizational structure, has an experienced market researcher or data scientist reviewed the method and validation before adoption?
