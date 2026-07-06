---
name: visual-ethics-and-accessibility-in-graphics.md
description: Use when the agent is making ethical and accessibility decisions in data visualization and visual journalism, handling sensitive or identifying data, designing for colorblind and screen-reader users, labeling honestly, or deciding what should and should not be visualized for an audience.
---

# Visual Ethics and Accessibility in Graphics

A visualization makes choices that prose can hedge but a graphic cannot easily undo: whom to map, what to color red, how granularly to break down a population, whether to name neighborhoods or individuals, whether a chart of deaths emphasizes the number or the people. These choices carry ethical weight because graphics feel objective and are absorbed in a glance, and because they can identify, stigmatize, or dehumanize the people behind the data. At the same time, a graphic that a portion of the audience literally cannot perceive, due to colorblindness, low vision, or reliance on screen readers, fails in its duty to inform. This skill covers the judgment involved in the ethics and accessibility of visual journalism: what to visualize and what to withhold, how to represent people behind the data, and how to ensure graphics reach and respect the whole audience.

## Core Rules

### Decide What Should and Should Not Be Visualized

Not every true fact should be visualized. A map of crime by block can stigmatize neighborhoods and invite vigilantism even if the data is accurate; a chart breaking down a small population by a sensitive characteristic can identify individuals; a visualization of victims can reduce people to data points. Before visualizing, ask whether the graphic could identify, harm, or stigmatize the people behind the data, and whether the public interest justifies that risk. Sometimes the right decision is to aggregate, to suppress small cells, to use broader geography, or not to visualize at all. The fact that data can be mapped or charted does not mean it should be.

### Protect Individual Identification in Granular Data

Granular data visualizations, dot maps, small-area charts, detailed demographic breakdowns, can identify individuals even when names are absent, especially for small populations or rare characteristics. Assess the re-identification risk before publishing. Suppress or aggregate small cells, blur geography, combine categories, or add noise where identification is possible. Be especially careful with data about vulnerable populations, victims, health conditions, immigration status, or other sensitive attributes. The duty to minimize harm applies to data points as much as to named subjects.

### Represent People Behind the Data, Not Just Numbers

Data about people can dehumanize when it reduces them to points on a chart. Consider whether the visualization acknowledges the humanity behind the numbers, especially for data about deaths, victims, or suffering. This does not mean every chart must be emotional, but it means being thoughtful about framing, context, and the risk of reducing human experience to an aesthetic. Avoid visual treatments that trivialize suffering (e.g., playful icons in a chart of deaths). Provide context that reminds the reader these are people, not just data.

### Design for Colorblind and Low-Vision Readers

A significant portion of the audience, roughly one in twelve men and a smaller share of women, has some form of color vision deficiency, and many readers have low vision. Design graphics to be legible to them. Avoid red-green encoding as the sole means of distinguishing data; use colorblind-safe palettes; ensure sufficient contrast; use shape, pattern, or direct labeling in addition to color. Test graphics with colorblind simulators before publishing. A graphic that excludes colorblind readers fails a substantial part of the audience and is simply incomplete journalism.

### Provide Text Alternatives and Screen-Reader Access

Readers who use screen readers, including people who are blind or have low vision, cannot perceive a standard image chart. Provide alternatives: descriptive alt text that conveys the chart's key finding, a data table underlying the chart where feasible, and structured markup that allows screen readers to navigate the data. Do not treat accessibility as an afterthought; a chart that cannot be accessed by screen-reader users is not reaching the whole audience. Build accessible alternatives into the production process for significant graphics.

### Label Honestly and Avoid Misleading Visual Rhetoric

Visual choices carry rhetorical force that readers absorb without examining. A 3D chart distorts perception; an area-encoded chart where radius doubles the visual area quadruples; an icon-based chart where icons overlap creates false impressions. Choose visual encodings that represent the data accurately and avoid techniques that distort perception, however visually appealing. Label directly, annotate honestly, and ensure the visual treatment supports rather than overrides the numbers. The graphic's visual rhetoric should serve the truth, not dramatize it.

### Be Cautious With Sensitive and Traumatic Visualizations

Visualizations of mass death, violence, disease, or disaster require particular care. Consider the impact on readers, on victims and their families, and on affected communities. Avoid gratuitous detail or visualization choices that exploit suffering for engagement. Provide content notes where the visualization depicts trauma. Weigh the public interest against the harm, and where the visualization is necessary, present it with gravity and context rather than spectacle. The same harm-minimization standards that apply to images apply to data graphics.

### Maintain Consistent Ethical Standards Across Formats

Ethical standards do not relax because a graphic is interactive, animated, or on social media. Interactive features should not allow users to drill down to identifying data you would not publish directly; animations should not dramatize or distort; social-format graphics should meet the same honesty standards as site graphics. Maintain consistent ethical and accessibility standards across every format, and review each format for its specific risks. The medium changes the risks but not the standards.

## Common Traps

### Visualizing Identifiable or Stigmatizing Data Because It Is Available

Mapping or charting granular data that identifies or stigmatizes individuals or neighborhoods because the data exists. This is a trap because accuracy does not justify harm. Assess re-identification and stigma risk before visualizing.

### Using Red-Green Encoding That Excludes Colorblind Readers

Relying on red and green to distinguish data, which is invisible to many readers. This is a trap because it excludes a substantial audience. Use colorblind-safe palettes and redundant encoding.

### Providing No Text Alternative for Screen-Reader Users

Publishing image charts with no alt text or data table. This is a trap because it excludes blind and low-vision readers entirely. Build accessible alternatives into production.

### Reducing People to Data Points Without Context

Visualizing deaths, victims, or suffering as pure numbers with no acknowledgment of humanity. This is a trap because it dehumanizes the subjects. Provide context and treat suffering with gravity.

### Using Visually Appealing but Distorting Techniques

3D charts, misleading area encodings, or overlapping icons that distort perception. This is a trap because the visual rhetoric overrides the numbers. Choose encodings that represent data accurately.

### Exploiting Traumatic Data for Engagement

Using dramatic visualization of death or suffering to drive engagement without regard for victims or readers. This is a trap because it exploits harm. Weigh public interest and present with gravity.

### Lowering Standards for Interactive or Social Formats

Allowing interactive drill-down to identifying data, or relaxing honesty standards for social graphics. This is a trap because the format changes risks, not standards. Maintain consistent ethics across formats.

## Self-Check

- Has the visualization been assessed for whether it could identify, stigmatize, or harm the people behind the data, with aggregation or suppression applied where needed?
- For granular data, has re-identification risk been evaluated, especially for small populations or sensitive attributes, and mitigated where present?
- Does the graphic use colorblind-safe palettes and redundant encoding (shape, pattern, labels) so colorblind readers are not excluded?
- Are there text alternatives, alt text, and underlying data tables so screen-reader users can access the information?
- Do visual encodings represent the data accurately, avoiding 3D, misleading area scales, and other techniques that distort perception?
- For visualizations of trauma, death, or suffering, has the public interest been weighed against harm, with content notes and gravity where publication is warranted?
- Does the graphic acknowledge the humanity behind the numbers rather than reducing people to data points?
- For graphics involving sensitive data, legally protected information, or potential identification, has an editor reviewed the ethical and accessibility decisions, and where legal or privacy risk exists, has counsel or appropriate expertise been consulted before publication?
