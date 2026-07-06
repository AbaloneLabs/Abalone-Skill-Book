---
name: interactive_and_reproducible_figures.md
description: Use when the agent is building interactive visualizations, creating dashboards, embedding widgets in notebooks or web pages, making figures reproducible with code, using plotting libraries programmatically, or ensuring dynamic figures remain accessible and stable.
---

# Interactive And Reproducible Figures

Interactive and code-generated figures expand what visualization can do, but they introduce failure modes that static figures do not have. An interactive widget can break when data changes, depend on libraries that vanish, or exclude readers whose devices cannot render it. A reproducible figure pipeline can silently produce different output across environments if dependencies are not pinned. Agents often treat interactivity as a feature to add without considering whether it aids comprehension, and treat reproducibility as assumed rather than engineered, producing figures that look impressive but are fragile, inaccessible, or unverifiable.

The harm this skill prevents is rotting visualizations that stop working, dynamic figures that mislead through interaction, and "reproducible" pipelines that fail to reproduce. The agent has freedom in tool choice and interaction design, but the constraints around stability, accessibility, and true reproducibility are binding.

## Core Rules

### Justify Interactivity Against Its Costs

Interactivity is not automatically better than a static figure. It adds cognitive load, accessibility burden, and maintenance risk. Use it only when it serves comprehension.

Interactivity earns its place when:

- the dataset is too large or layered for one static view;
- the reader needs to explore subsets, filter, or zoom;
- comparison across many conditions benefits from toggling;
- a dashboard supports ongoing monitoring rather than one-time reading.

When a static figure conveys the message, prefer it. Interactivity for its own sake obscures the takeaway.

### Make Interactive Figures Accessible

Dynamic figures often fail accessibility standards. Readers using screen readers, keyboard navigation, or assistive devices can be entirely excluded.

Ensure accessibility:

- provide a static fallback or text description of the key insight;
- support keyboard navigation through controls;
- ensure color choices meet contrast standards;
- do not rely on hover-only information that is invisible without a mouse;
- test with screen readers where possible;
- include alt text or a long description summarizing the figure's message.

### Pin Dependencies For True Reproducibility

A figure script that "works on my machine" is not reproducible. Library updates change rendering, defaults, and output silently.

Engineer reproducibility:

- pin exact library versions in a lockfile or environment file;
- record the language version and operating system;
- use environment managers (conda, renv, virtualenv) to isolate;
- containerize complex pipelines where feasible;
- document the build steps in a README or Makefile;
- test that the pipeline runs from a clean environment.

### Separate Data, Code, And Presentation

Figures become unmaintainable when data, transformation logic, and styling are tangled. A clean separation lets each layer change independently.

Structure the pipeline:

- raw data stored untouched;
- a cleaning or transformation script producing analysis-ready data;
- a plotting script that reads analysis data and applies styling;
- a shared theme or style file for consistent appearance;
- output written to a known directory with versioned filenames.

### Provide A Static Capture For Publication And Archive

Interactive figures may not survive in the published record. Journals, repositories, and archives often require static versions. Even web-based interactives can rot as libraries change.

Always provide:

- a static PNG or PDF export of the key view;
- a caption that communicates the main finding without interaction;
- an archived snapshot if the interactive is web-hosted;
- the underlying data so the figure can be regenerated statically.

### Encode Interaction State And Defaults Deliberately

The default view of an interactive figure is what most readers see. If the default hides the key pattern behind a filter or zoom, the message is lost.

Design the default state:

- show the most important comparison or trend by default;
- pre-select the most relevant filters;
- set axis ranges to reveal, not clip, the data;
- label controls clearly so users understand what they change;
- ensure the initial view communicates the headline finding.

### Document The Figure's Provenance

A reader or future collaborator should be able to trace a figure back to its data and code. Undocumented figures become untraceable over time.

Document provenance:

- which script and commit produced the figure;
- which data file and version was the input;
- what parameters or settings were used;
- where the output is stored;
- any manual post-processing steps, however minor.

### Test Figures Across Environments And Over Time

A figure that renders today may break tomorrow. Periodic regeneration catches silent failures from dependency drift, data schema changes, or encoding issues.

Test by:

- regenerating all figures in a clean environment periodically;
- comparing outputs to detect unintended changes;
- running continuous integration on the figure pipeline where possible;
- checking that interactive widgets still function after library updates.

## Common Traps

### Adding Interactivity Because It Seems Modern

Sliders and tooltips that do not aid comprehension add clutter and exclude readers. Default to static unless interaction earns its place.

### Assuming Code-Generated Means Reproducible

Unpinned dependencies and implicit environments break reproducibility. Reproducibility must be engineered and tested, not assumed.

### Hover-Only Information

Data visible only on hover is invisible to readers without a mouse and to screen readers. Surface key information in the default view.

### Fragile Web-Only Figures

Interactive figures hosted on a specific library version can rot when the library changes or the host disappears. Always archive a static capture.

### Tangled Data, Logic, And Styling

When transformation and plotting are mixed, changes to one break the other. Separate layers for maintainability.

### Unlabeled Or Confusing Controls

If readers cannot tell what a slider or toggle does, the interaction frustrates rather than enlightens. Label controls and explain their effect.

### Silent Output Changes From Dependency Drift

A figure that looks different after a library update may be wrong, not just restyled. Pin versions and verify outputs after updates.

## Self-Check

- [ ] Does interactivity serve comprehension, or would a static figure convey the message better?
- [ ] Does the interactive figure have a static fallback and text description for accessibility?
- [ ] Are all dependencies pinned in a lockfile or environment for true reproducibility?
- [ ] Are data, transformation logic, and styling separated into distinct pipeline layers?
- [ ] Is a static capture (PNG or PDF) provided for publication and archiving?
- [ ] Does the default view of the interactive figure communicate the headline finding?
- [ ] Is the figure's provenance documented (script, commit, data version, parameters)?
- [ ] Can the figure be regenerated from a clean environment without manual intervention?
- [ ] Are controls clearly labeled so users understand what they change?
- [ ] Has the figure been tested for silent changes after dependency or data updates?
