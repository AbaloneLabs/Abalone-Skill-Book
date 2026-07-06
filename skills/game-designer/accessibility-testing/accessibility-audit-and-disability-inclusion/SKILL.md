---
name: accessibility-audit-and-disability-inclusion.md
description: Use when the agent is conducting an accessibility audit for disability inclusion, identifying barriers for motor visual auditory and cognitive disabilities, or evaluating whether a game is playable by players with disabilities or excludes them through barriers that the team never noticed because no disabled playtester was ever in the room, and the barriers persist into release.
---

# Accessibility Audit and Disability Inclusion

Accessibility — making a game playable by people with disabilities — is a matter of who gets to play, and it is also where most teams fail silently, because the barriers that exclude disabled players are invisible to able-bodied teams who never encounter them, and without disabled playtesters and deliberate audit the barriers persist into release and exclude players the team never realized they were excluding. The judgment problem is that accessibility requires deliberate audit (the barriers are invisible without it), must address the full range of disabilities (motor, visual, auditory, cognitive), and must involve disabled players (whose lived experience reveals barriers the team cannot see). Agents tend to miss this because the team's able-bodied experience hides the barriers, and because accessibility is treated as optional polish rather than inclusion that determines who can play. The harm is games that exclude disabled players through barriers that were never found because no one looked, and no disabled player was asked. This skill covers how to conduct accessibility audit and inclusion that finds and removes barriers, and avoid the invisible-barrier, narrow-scope, and no-disabled-voice traps. The agent has latitude in the solutions, but the obligation to audit and include is not optional.

## Core Rules

### Audit Deliberately Across All Disability Categories

Accessibility barriers must be audited deliberately across all disability categories — motor (input, timing), visual (contrast, text, color-dependence), auditory (subtitles, cues), cognitive (complexity, memory, reading) — because the barriers are invisible without deliberate audit, and each category has distinct barriers that a general "accessibility pass" will miss. The decision rule: audit deliberately across all disability categories, use category-specific checklists, and avoid general passes. General passes miss category barriers, because the audit was not category-specific.

### Involve Disabled Playtesters With Lived Experience

Disabled playtesters — people with the actual disabilities the design must accommodate — must be involved, because their lived experience reveals barriers that able-bodied teams cannot see, and accessibility designed without disabled input is guesswork that misses the real barriers. The decision rule: involve disabled playtesters throughout, center their feedback, and avoid able-bodied-only design. Able-bodied-only design misses barriers, because the lived experience was absent.

### Provide Remappable and Alternative Input Options

Motor accessibility requires remappable input (so players can adapt controls to their needs) and alternative input options (so players who cannot use standard controls can play), because fixed or standard-only input excludes players whose motor needs differ, and remapping and alternatives are the baseline of motor inclusion. The decision rule: provide remappable input and alternative options, and avoid fixed or standard-only input. Fixed input excludes motor-disabled players, because the input could not be adapted.

### Ensure Visual Information Is Not Conveyed by Color Alone

Visual information must not be conveyed by color alone — patterns, icons, labels, and text must supplement color — because colorblind players (a large population) cannot perceive color-dependent information, and color-only encoding excludes them from game-critical information. The decision rule: supplement color with patterns, icons, or labels, and avoid color-only information encoding. Color-only encoding excludes colorblind players, because the information was not perceivable.

### Support Adjustable Text, Contrast, and Audio Cues

Visual and auditory accessibility require adjustable text (size, font), adjustable contrast (so low-vision players can read), and audio cues (so visual information has auditory alternatives), because fixed text, contrast, and cue design excludes low-vision and deaf players, and adjustability lets each player configure to their needs. The decision rule: support adjustable text, contrast, and audio cues, and avoid fixed visual and auditory design. Fixed design excludes low-vision and deaf players, because the configuration was not adjustable.

### Avoid Time Pressure and Provide Difficulty and Assist Options

Cognitive and motor accessibility requires avoiding unnecessary time pressure (which excludes players who need more time) and providing difficulty and assist options (which let players tailor the challenge to their ability), because fixed time pressure and fixed difficulty exclude players whose cognitive or motor needs require accommodation. The decision rule: avoid unnecessary time pressure, provide difficulty and assist options, and avoid fixed-pressure fixed-difficulty design. Fixed-pressure design excludes players, because the time and difficulty could not be accommodated.

## Common Traps

### General Accessibility Passes Missing Category Barriers

The team does a general accessibility pass, and the category barriers are missed. The trap is that the pass addresses accessibility. The false signal is that the audit is done. The harm is that the general pass addresses surface accessibility without category depth, the motor, visual, auditory, and cognitive barriers that require category-specific audit persist, the disabled players in each category encounter the missed barriers, the exclusion continues behind a checked box, and the audit fails to find the barriers it was meant to find, because the audit was not category-specific.

### Able-Bodied-Only Design Missing Barriers

The team designs accessibility without disabled playtesters, and the barriers are missed. The trap is that the team considered accessibility. The false signal is that the options exist. The harm is that the able-bodied-only design is guesswork, the real barriers (which only lived experience reveals) are not found, the accessibility options do not address the actual exclusion, the disabled players encounter barriers the team could not see, and the inclusion fails because the disabled voice was absent, because the playtesters were not disabled.

### Fixed Input Excluding Motor-Disabled Players

The team ships fixed or standard-only input, and motor-disabled players are excluded. The trap is that the input is standard. The false signal is that the controls work. The harm is that the fixed input cannot be adapted to motor needs, the players who cannot use standard controls are excluded, the remapping and alternatives that would include them are absent, the game is unplayable for motor-disabled players, and the exclusion persists because the input was not remappable, because the input was fixed.

### Color-Only Encoding Excluding Colorblind Players

The team conveys game-critical information by color alone, and colorblind players are excluded. The trap is that the color is clear. The false signal is that the information is visible. The harm is that the color-only encoding cannot be perceived by colorblind players, the game-critical information (friend or foe, hazard or safe, tier or type) is invisible to them, the players who cannot distinguish the colors are excluded or harmed, the game is unplayable or unfair for colorblind players, and the exclusion persists because the color was not supplemented, because the encoding was color-only.

### Fixed Visual and Auditory Design Excluding Players

The team ships fixed text, contrast, and cue design, and low-vision and deaf players are excluded. The trap is that the design is readable. The false signal is that the text and cues work. The harm is that the fixed text is too small for low-vision players, the fixed contrast is unreadable, the visual cues have no auditory alternatives for deaf players, the players whose visual or auditory needs differ are excluded, the game is unplayable for low-vision and deaf players, and the exclusion persists because the design was not adjustable, because the configuration was fixed.

### Fixed-Pressure Fixed-Difficulty Design Excluding Players

The team ships fixed time pressure and fixed difficulty, and players who need accommodation are excluded. The trap is that the pressure and difficulty are the design. The false signal is that the challenge is intended. The harm is that the fixed time pressure excludes players who need more time (cognitive, motor), the fixed difficulty excludes players whose ability differs, the players who need accommodation cannot play, the assist and difficulty options that would include them are absent, and the exclusion persists because the pressure and difficulty were not adjustable, because the design was fixed.

## Self-Check

- Is accessibility audited deliberately across all disability categories (motor, visual, auditory, cognitive)?
- Are disabled playtesters with lived experience involved throughout, centering their feedback?
- Is input remappable with alternative options for motor-disabled players?
- Is visual information supplemented with patterns, icons, or labels, never conveyed by color alone?
- Are text, contrast, and audio cues adjustable for low-vision and deaf players?
- Is unnecessary time pressure avoided, with difficulty and assist options for cognitive and motor accommodation?
- Did I confirm the game is playable by disabled players rather than excluding them through invisible barriers?
