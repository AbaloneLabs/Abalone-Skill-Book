---
name: accessibility-testing-methodology-and-validation.md
description: Use when the agent is designing an accessibility testing methodology, planning disabled playtester recruitment and validation, or evaluating whether accessibility testing actually finds real barriers or degenerates into checklist compliance and automated scans that pass while real disabled players still cannot play the game at release.
---

# Accessibility Testing Methodology and Validation

Accessibility testing is how a team verifies that its accessibility work actually removes barriers, and it is also where accessibility efforts most commonly fail to deliver, because testing easily degenerates into checklist compliance (the boxes are checked) and automated scans (the tools pass) while real disabled players still cannot play the game, because the checklists and scans do not capture the lived barriers. The judgment problem is that accessibility testing must involve real disabled players (whose experience reveals real barriers), must test the full experience (not just compliance items), and must validate that barriers are actually removed (not just addressed on paper). Agents tend to miss this because checklist compliance and automated scans feel like testing (and produce passing results) while missing the barriers that only disabled play reveals, and because recruiting disabled playtesters requires effort that checklist-only testing avoids. The harm is games that pass accessibility checks but exclude disabled players at release, because the testing never validated real play. This skill covers how to design accessibility testing that finds and validates real barriers, and avoid the checklist, scan-only, and paper-compliance traps. The agent has latitude in the methodology, but the obligation to validate with real disabled players is not optional.

## Core Rules

### Test With Real Disabled Players, Not Just Checklists

Accessibility testing must involve real disabled players — people with the actual disabilities playing the actual game — because checklists and automated tools cannot reveal the lived barriers that disabled players encounter, and only real play validates real accessibility. The decision rule: test with real disabled players, center their experience, and avoid checklist-only testing. Checklist-only testing misses barriers, because the lived experience was absent.

### Test the Full Experience, Not Just Compliance Items

Accessibility testing must cover the full game experience — every mode, every path, every interaction — not just the compliance items (subtitles, remapping) that checklists enumerate, because barriers exist throughout the experience (a critical cue with no visual alternative, a mode without subtitle support), and compliance-item testing misses the barriers outside the items. The decision rule: test the full experience across modes and paths, and avoid compliance-item-only testing. Compliance-item testing misses experience barriers, because the testing was not full.

### Use Automated Tools as a Supplement, Not a Replacement

Automated accessibility tools (contrast checkers, scan tools) are useful supplements that catch some barriers efficiently, but they are not a replacement for disabled playtesting, because they cannot capture the lived barriers (can the player actually navigate this? is this usable with this disability?), and scan-only testing passes while real barriers persist. The decision rule: use automated tools as a supplement to disabled playtesting, and avoid scan-only validation. Scan-only validation passes falsely, because the tools could not capture lived barriers.

### Validate That Barriers Are Actually Removed, Not Just Addressed

When barriers are found and addressed, the fix must be validated — does the change actually remove the barrier for disabled players? — because addressing a barrier on paper (adding an option) does not guarantee it removes the barrier (the option may be insufficient, buried, or buggy), and validation confirms the real removal. The decision rule: validate barrier removal with disabled players, and avoid paper-only addressing. Paper-only addressing does not remove barriers, because the fix was not validated.

### Recruit Diverse Disabled Playtesters Across Disability Categories

Disabled playtesters must be diverse across disability categories — motor, visual, auditory, cognitive — because each category has distinct barriers, and testing with one category (or with a single disabled playtester) misses the barriers of the others, and diverse recruitment covers the range. The decision rule: recruit diverse disabled playtesters across categories, and avoid single-category or token testing. Single-category testing misses other barriers, because the diversity was insufficient.

### Integrate Accessibility Testing Throughout, Not Just at the End

Accessibility testing must be integrated throughout development — early concept, vertical slice, late beta — not just at the end, because late-only testing finds barriers too late to fix (when the design is locked), and early testing finds barriers when they are cheap to address, and throughout-integration builds accessibility in rather than bolting it on. The decision rule: integrate accessibility testing throughout development, and avoid end-only testing. End-only testing finds barriers too late, because the design was locked.

## Common Traps

### Checklist-Only Testing Missing Barriers

The team tests with checklists only, and the barriers are missed. The trap is that the checklists are comprehensive. The false signal is that the items pass. The harm is that the checklist-only testing addresses compliance items, the lived barriers that disabled players encounter (which checklists cannot enumerate) are not found, the disabled players hit the missed barriers at release, the accessibility passes on paper while failing in practice, and the exclusion persists behind checked boxes, because the testing was checklist-only.

### Compliance-Item Testing Missing Experience Barriers

The team tests only the compliance items, and the experience barriers are missed. The trap is that the items are the standard. The false signal is that the accessibility features work. The harm is that the compliance-item testing validates the enumerated features (subtitles, remapping), the barriers outside the items (a mode without subtitle support, a critical cue with no alternative) are not tested, the disabled players encounter the untested barriers in the broader experience, and the exclusion persists in the untested experience, because the testing was not full.

### Scan-Only Validation Passing Falsely

The team validates with automated scans only, and the validation passes falsely. The trap is that the scans are objective. The false signal is that the tools pass. The harm is that the scan-only validation catches the barriers tools can detect (contrast ratios), the lived barriers (can the player navigate this? is this usable?) are not captured, the scans pass while the real barriers persist, the disabled players encounter the barriers the scans could not see, and the exclusion persists behind passing scans, because the validation was scan-only.

### Paper-Only Addressing Not Removing Barriers

The team addresses barriers on paper without validation, and the barriers are not removed. The trap is that the feature is added. The false signal is that the option exists. The harm is that the paper-only addressing adds an option, the option may be insufficient (does not actually remove the barrier), buried (hard to find), or buggy (does not work), the disabled players still encounter the barrier despite the addressed feature, the exclusion persists behind the added option, and the barrier is not removed because the fix was not validated, because the addressing was paper-only.

### Single-Category or Token Testing Missing Other Barriers

The team tests with one disability category or a token disabled playtester, and the other barriers are missed. The trap is that disabled players were involved. The false signal is that the testing is inclusive. The harm is that the single-category testing finds the barriers of that category, the barriers of the other categories (motor, visual, auditory, cognitive) are not found, the token playtester cannot represent all disabilities, the disabled players in the untested categories encounter the missed barriers, and the exclusion persists for the untested categories, because the diversity was insufficient.

### End-Only Testing Finding Barriers Too Late

The team tests accessibility only at the end, and the barriers are found too late. The trap is that the end testing is thorough. The false signal is that the testing is done. The harm is that the end-only testing finds barriers when the design is locked, the barriers are too expensive or impossible to fix at that stage, the accessibility is bolted on imperfectly or not at all, the disabled players encounter the unfixable barriers at release, and the exclusion persists because the barriers were found too late, because the testing was not integrated throughout.

## Self-Check

- Does accessibility testing involve real disabled players, not just checklists and compliance items?
- Is the full game experience tested across modes and paths, not just enumerated compliance items?
- Are automated tools used as a supplement to disabled playtesting, not a replacement for it?
- Are barrier fixes validated with disabled players to confirm actual removal, not just paper addressing?
- Are disabled playtesters diverse across all disability categories, avoiding single-category or token testing?
- Is accessibility testing integrated throughout development, not just at the end when design is locked?
- Did I confirm accessibility testing finds and validates real barriers rather than passing checks while excluding players?
