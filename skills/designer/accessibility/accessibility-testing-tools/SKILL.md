---
name: accessibility_testing_tools.md
description: Use when the agent is selecting or using accessibility testing tools, interpreting automated scan results, combining automated and manual testing, choosing between browser extensions, linters, screen readers, and contrast analyzers, understanding what tools can and cannot detect, or ensuring testing tools supplement rather than replace human judgment and real user testing.
---

# Accessibility Testing Tools

Accessibility testing tools are valuable precisely because they catch a class of errors quickly and consistently, but they are dangerous when treated as a complete solution. Automated tools can detect a meaningful fraction of accessibility issues, but they cannot detect a large fraction of the issues that actually affect users, because many accessibility problems are contextual, semantic, and experiential in ways no scanner can judge. The judgment problem is knowing what each category of tool can and cannot find, combining automated scanning with manual and assistive-technology testing, and refusing to let a clean automated report substitute for testing with real users and real assistive technology. Agents tend to fail by running a scanner, declaring the result accessible, and stopping, by misinterpreting tool output as binary verdicts, and by choosing tools that do not match the testing need.

Use this skill when selecting, running, or interpreting accessibility testing tools for any product: automated scanners, browser extensions, linters, contrast checkers, or screen readers. The goal is a testing approach that uses tools for what they do well without mistaking them for the whole job.

## Core Rules

### Understand What Automated Tools Can And Cannot Detect

Automated accessibility tools are excellent at finding a specific set of issues and blind to most of the rest. Treating a clean scan as proof of accessibility is the most common and most damaging misuse of these tools.

Automated tools can typically detect:

- missing or incorrect HTML semantics, such as missing alt text, empty labels, or wrong roles;
- basic contrast failures against standard ratios;
- missing document language, title, or landmarks;
- duplicate or missing IDs and label associations;
- some structural issues like skipped heading levels.

Automated tools typically cannot detect:

- whether alt text is meaningful or merely present;
- whether a flow is operable or understandable;
- whether focus order is logical in context;
- whether content is readable or cognitively accessible;
- whether the experience works with a real screen reader, keyboard, or switch;
- whether the design is usable under real conditions.

Estimates vary, but automated tools commonly detect on the order of a third to half of accessibility issues. A clean automated report is a starting point, never a conclusion.

### Combine Automated, Manual, And Assistive Technology Testing

Because no single method finds everything, a credible accessibility test combines layers, each catching what the others miss.

Layer the testing:

- automated scanning, for fast, repeatable detection of structural and contrast issues;
- manual keyboard testing, to confirm operability without a mouse;
- screen reader testing, with real assistive technology, to confirm semantics and reading order;
- manual review against guidelines, for issues that require judgment;
- testing with real users who use assistive technology, for issues that require experience.

Each layer has a cost and a payoff. Skipping the manual and user layers because the automated layer passed leaves the majority of real issues undetected.

### Choose The Right Tool For The Question

Different tools answer different questions, and using the wrong tool produces irrelevant or misleading results. Match the tool to the testing need.

Match tools to needs:

- automated scanners, such as Axe or WAVE, for bulk structural and contrast issues across pages;
- browser extensions, for quick checks during development;
- linters and CI integrations, to catch issues before they ship;
- contrast analyzers, for precise color and text checks;
- screen readers, such as VoiceOver, NVDA, or TalkBack, for the real assisted experience;
- keyboard-only navigation, for operability without pointing devices.

Running a contrast checker tells you nothing about operability. Running a scanner tells you nothing about the lived experience of a screen reader user. Pick the tool that answers the actual question.

### Interpret Tool Output As Signals, Not Verdicts

Automated tools report issues, but those reports require interpretation. Some flagged issues are real and critical; some are false positives; some are technically flagged but low-impact. Treating every report item as equally important, or dismissing them all, both lead to poor outcomes.

Interpret carefully by:

- reviewing each flagged issue in context to confirm it is real;
- prioritizing by impact on users, not just by count;
- recognizing false positives, where the tool flags something that is actually correct;
- understanding that the absence of a flag is not the absence of a problem.

A long list of issues is not necessarily a disaster, and a short list is not necessarily a pass. Read the output with judgment.

### Test With Real Assistive Technology, Not Simulators Only

Simulators and emulators are useful for quick checks, but they do not reproduce the real experience of using assistive technology. A design that passes a simulator can fail badly with a real screen reader, because real usage involves navigation patterns, speed, and expectations that simulators do not capture.

Test for real by:

- using actual screen readers, VoiceOver, NVDA, JAWS, TalkBack, as real users do;
- navigating by keyboard only, including tab order and visible focus;
- testing with switches and voice control where relevant;
- observing or working with users who depend on these technologies daily.

The gap between simulated and real assistive technology use is where many accessibility failures hide. Close it with real testing.

### Integrate Tools Into The Workflow, Not Just Final QA

Accessibility tools are most effective when they run early and often, catching issues before they become expensive to fix. Running tools only at the end forces late, costly remediation.

Integrate by:

- adding linters and scanners to the development workflow and CI;
- checking contrast and semantics during design, before code is written;
- training designers and engineers to use tools as they work, not only testers at the end;
- treating accessibility checks like any other quality gate, continuous and expected.

Late, audit-driven accessibility is more expensive and less effective than accessibility built in and checked continuously.

### Do Not Let Tools Replace Judgment And User Testing

The deepest failure is treating tools as a substitute for the judgment and user involvement that accessibility ultimately requires. Tools find issues; they do not determine whether the experience is usable, understandable, or respectful.

Keep tools in their place by:

- using them to find candidates for review, not to make final decisions;
- always pairing automated results with manual and user testing;
- recognizing that usability and dignity are judged by people, not scanners;
- involving users with disabilities in testing, not only as subjects of compliance.

A product can pass every automated check and still be inaccessible in practice. The tools are assistants, not authorities.

## Common Traps

### Treating A Clean Scan As Proof Of Accessibility

Automated tools detect only a fraction of real issues. A pass is a starting point, not a conclusion.

### Using Only One Tool Or Method

No single method finds everything. Layer automated, manual, and assistive technology testing.

### Wrong Tool For The Question

A contrast checker cannot assess operability. Match the tool to the testing need.

### Reading Output As Binary Verdicts

Tool reports need interpretation and prioritization. Some flags are false positives; some real issues are unflagged.

### Simulators Instead Of Real Assistive Technology

Simulators do not reproduce real screen reader or keyboard use. Test with actual tools and users.

### Tools Only At Final QA

Late, audit-driven testing is costly and ineffective. Integrate tools into the workflow.

### Tools Replacing Judgment And Users

Scanners find issues but cannot judge usability or dignity. Always involve real users.

## Self-Check

- [ ] The team understands which issues automated tools can and cannot detect, and does not treat a clean scan as complete.
- [ ] Testing layers automated scanning, manual keyboard and review, screen reader testing, and real user testing.
- [ ] Tools are matched to the specific question being asked, not used generically.
- [ ] Tool output is interpreted as signals requiring judgment, not as binary verdicts.
- [ ] Real assistive technology, screen readers, keyboards, switches, is used, not only simulators.
- [ ] Accessibility tools are integrated into design and development workflows, not only final QA.
- [ ] False positives are recognized and real issues are prioritized by user impact.
- [ ] Users who depend on assistive technology are involved in testing, not only compliance scans.
- [ ] No claim of accessibility rests on automated results alone.
- [ ] Tools supplement human judgment rather than replacing it.
