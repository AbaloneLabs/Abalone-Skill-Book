---
name: accessibility-testing-strategy.md
description: Use when the agent is planning or running accessibility testing, choosing between automated scans and manual checks, recruiting users with disabilities, prioritizing defects, deciding what to test at each stage, or building an accessibility testing workflow for a product.
---

# Accessibility Testing Strategy

Accessibility testing is not a single pass with an automated scanner at the end of a project. It is a layered, ongoing practice that combines automated detection, manual expert review, assistive-technology testing, and testing with real users who have disabilities. Each layer catches different defects, and skipping any layer leaves large classes of problems invisible. Agents often reduce accessibility testing to "run a tool and fix the red items," which produces a false sense of compliance while real barriers remain.

The judgment problem is deciding what to test, when, how, by whom, and against what standard, given finite time and the reality that accessibility defects are distributed across code, content, design, and interaction. A good strategy sequences tests so cheap automated checks run continuously, manual checks catch structural and interaction issues, assistive-technology checks catch real usage barriers, and user testing catches the problems no tool or expert can predict.

## Core Rules

### Treat accessibility testing as layered, not singular

No single method finds all defects. Automated tools catch roughly a third to half of issues, fast and cheaply, and should run in CI and pull requests. Manual expert review against a recognized standard catches structural, semantic, and interaction issues tools miss. Assistive-technology testing with screen readers, magnifiers, voice control, and switch devices catches real operability barriers. Testing with disabled users catches the human problems none of the above predict. Design your strategy to use all four layers, each where it is most efficient.

### Define the scope and conformance target explicitly before testing

Accessibility is meaningless without a stated scope and target. Decide which pages, flows, components, and states are in scope; which standard and version you test against; which assistive technologies and browser/OS combinations are supported; and what level of conformance is required. Testing "the whole app for accessibility" without these definitions produces scattered, incomparable results. Write the scope down and revisit it as the product changes.

### Sequence tests from cheap and broad to expensive and deep

Run automated scans first and continuously, because they are nearly free and catch repeated, mechanical defects across the whole surface. Run manual review on key flows and new components, because it catches the semantic and interaction issues that tools miss. Run assistive-technology testing on critical user journeys, because it is slower and more expensive but reveals real barriers. Run user testing on the highest-stakes flows, because recruiting and sessions are costly but uniquely valuable. Do not invert this order.

### Test the full interaction state, not only the initial render

Most accessibility defects hide in dynamic behavior: opened dialogs, expanded menus, form validation errors, modal focus traps, live updates, single-page-app route changes, and custom widgets after interaction. A page that passes a scan on load can be completely inaccessible once the user starts interacting. Manual and assistive-technology testing must exercise the full set of states, errors, and dynamic updates, not just the default view.

### Test with real assistive technology, not emulators or assumptions

Screen reader names, commands, and behavior differ across platforms and versions, and browser/AT combinations interact in ways no documentation fully captures. Testing with the actual intended combinations, using actual keyboard-only navigation, is the only reliable way to catch operability defects. Avoid relying on browser extensions that approximate AT behavior; they give a false sense of confidence.

### Recruit users with disabilities for high-stakes flows

Expert review and tools cannot predict the lived experience of using an interface with a disability. Recruiting users who actually rely on assistive technology, across a range of conditions and proficiency levels, surfaces problems that are invisible to every other method. Prioritize this for flows with high consequence: authentication, payments, essential transactions, and core task completion. Compensate participants fairly and treat findings as authoritative, not anecdotal.

### Prioritize defects by user impact, not by ease of fix or scan severity

A scanner's "critical" rating reflects a rule violation, not necessarily the severity of the barrier a user experiences. Prioritize by the actual effect on users: a defect that blocks task completion for a whole group outranks a cosmetic defect that affects few. Group related defects so that fixing an underlying pattern resolves many instances at once. Track both per-defect fixes and systemic patterns that need design or code changes.

### Embed testing across the lifecycle, not only at the end

Accessibility defects are cheapest to prevent at design time and most expensive to fix after release. Move checks left: review designs for color, contrast, target size, and structure before build; run automated checks in CI; review components in a library before they are adopted; and test complete flows before release. End-of-project-only testing guarantees late, expensive remediation and repeated defects.

## Common Traps

### Treating an automated scan as proof of accessibility

Automated tools find a meaningful fraction of issues but miss the majority of real barriers, especially those involving meaning, context, dynamic behavior, and operability. A clean scan is not a clean bill of health. The most damaging phrase in accessibility work is "the tool says we pass," because it halts the deeper testing that actually matters.

### Testing only the happy path and default state

Scanning the homepage and a couple of static pages, or running a screen reader only across the initial render, misses the defects that appear in forms, errors, modals, and dynamic updates. Most real barriers live in interaction. A strategy that does not exercise the full set of states is incomplete.

### Testing only one assistive technology or browser

VoiceOver, NVDA, JAWS, TalkBack, and browser-AT combinations behave differently, and a control that works in one can fail in another. Testing only the combination the team happens to own produces false confidence. Define the supported matrix and test across it for critical flows.

### Confusing conformance with usability

Passing a standard's checkpoints is not the same as being usable by disabled people. A technically conformant interface can still be confusing, slow, or exhausting. Conformance is a floor; user testing and expert judgment determine whether the experience is actually usable.

### Ignoring content and authoring defects

Accessibility is not only a code problem. Poor link text, missing or unhelpful alt text, jargon, complex language, missing captions, and poorly structured headings are content defects that tools partially detect but humans must judge. A testing strategy that only inspects markup misses the content layer.

### Over-relying on a single accessibility champion

When one person owns all accessibility testing, knowledge is fragile, coverage is inconsistent, and defects recur after they leave. Distribute the capability: train designers, developers, and QA to run their own checks, and use the specialist for strategy, hard cases, and review. Accessibility is a shared responsibility, not a role.

### Fixing symptoms instead of systemic patterns

Fixing the same alt-text or label defect in a hundred places, one at a time, wastes effort and guarantees recurrence. When a defect appears repeatedly, fix the underlying component, pattern, or documentation so it cannot return. Track systemic patterns alongside individual tickets.

## Self-Check

- Does your strategy use all four layers: automated, manual expert review, assistive-technology testing, and user testing with disabled participants?
- Have you written down the scope, standard, supported AT and browser combinations, and conformance level before testing?
- Are automated checks running continuously in CI, with manual and AT testing layered on key flows and components?
- Does testing exercise the full set of interaction states, errors, and dynamic updates, not only the default render?
- Are you testing with real assistive technology across the supported matrix, not emulators or assumptions?
- For high-stakes flows, have you recruited and fairly compensated users with disabilities?
- Are defects prioritized by real user impact and grouped into systemic patterns, not only by scan severity?
- Are accessibility checks embedded at design, component, CI, and release, rather than only at the end?
