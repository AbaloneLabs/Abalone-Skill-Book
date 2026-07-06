---
name: accessible-ui-and-control-options.md
description: Use when the agent is designing accessible UI and control options, building remapping and assist features, planning colorblind and subtitle systems, or evaluating whether the game's interface and controls accommodate the full range of player needs or lock players out through fixed layouts, unreadable text, and controls that cannot be adapted to motor, visual, or cognitive needs.
---

# Accessible UI and Control Options

The UI and controls are where most accessibility barriers concretely manifest — the interface the player must read and the controls the player must use — and accessible UI and control design is the practical work of removing those barriers through remapping, assist features, adjustable presentation, and flexible interaction. The judgment problem is that accessible UI and controls must be flexible (adaptable to diverse needs), must be readable and perceivable (across visual and auditory differences), and must not lock players out through fixed design that serves only the able-bodied default. Agents tend to miss this because the default UI and controls work for the able-bodied team, and the flexibility that disabled players need is invisible until the default fails for someone. The harm is UI that cannot be read, controls that cannot be used, and games that lock out players whose needs the fixed design does not serve. This skill covers how to design accessible UI and control options that accommodate the full range of needs, and avoid the fixed, unreadable, and non-adaptable traps. The agent has latitude in the design, but the obligation to accommodate diverse needs is not optional.

## Core Rules

### Provide Full Input Remapping and Custom Bindings

Input must be fully remappable — every action bindable to any input, custom bindings saveable — so players can adapt controls to their motor needs (one-handed, alternative devices, comfort), because fixed controls exclude players whose needs differ from the default, and full remapping is the baseline of motor-accessible control. The decision rule: provide full input remapping and custom bindings, and avoid fixed or partially-remappable controls. Fixed controls exclude motor-diverse players, because the input could not be adapted.

### Support Adjustable UI Scale, Text Size, and Contrast

UI presentation must be adjustable — UI scale, text size, contrast — so players can configure the interface to their visual needs, because fixed UI scale and text exclude low-vision players, and adjustability lets each player read the interface. The decision rule: support adjustable UI scale, text size, and contrast, and avoid fixed-size UI. Fixed-size UI excludes low-vision players, because the text could not be read.

### Implement Colorblind Modes and Non-Color Information Encoding

Colorblind modes (which adjust palettes for color vision deficiencies) and non-color information encoding (patterns, icons, labels supplementing color) must be implemented, because color-dependent UI excludes colorblind players, and the modes and encoding let them perceive the information. The decision rule: implement colorblind modes and non-color encoding, and avoid color-dependent UI. Color-dependent UI excludes colorblind players, because the color could not be perceived.

### Provide Comprehensive Subtitle and Audio Caption Systems

Auditory accessibility requires comprehensive subtitles (for all dialogue and narrative audio) and audio captions (for all sound cues and effects), because deaf and hard-of-hearing players cannot perceive audio-only information, and subtitles and captions provide the visual alternative. The decision rule: provide comprehensive subtitles and audio captions, and avoid audio-only information. Audio-only information excludes deaf players, because the audio could not be perceived.

### Offer Assist Options for Motor and Cognitive Needs

Assist options — aim assist, input buffering, timing adjustments, simplified controls — must be offered for motor and cognitive needs, because players whose motor or cognitive abilities differ from the default may need assistance to play, and assist options provide that accommodation without removing the game for those who do not. The decision rule: offer assist options for motor and cognitive needs, make them optional, and avoid fixed-difficulty fixed-control design. Fixed design excludes players, because the assistance was absent.

### Ensure UI Navigation Works Across Input Methods

UI navigation must work across input methods — gamepad, keyboard, mouse, alternative inputs — so players using any input can navigate the interface, because UI that only navigates with one input (mouse-only menus) excludes players using others (gamepad, alternative), and cross-input navigation includes all players. The decision rule: ensure UI navigation works across all input methods, and avoid single-input UI. Single-input UI excludes players, because the navigation did not work for their input.

## Common Traps

### Fixed Controls Excluding Motor-Diverse Players

The team ships fixed or partially-remappable controls, and motor-diverse players are excluded. The trap is that the default controls are standard. The false signal is that the controls work. The harm is that the fixed controls cannot be adapted to motor needs, the one-handed players, the alternative-device users, and the comfort-need players are excluded, the remapping that would include them is absent or partial, the game is unplayable for motor-diverse players, and the exclusion persists because the controls were not fully remappable, because the input was fixed.

### Fixed-Size UI Excluding Low-Vision Players

The team ships fixed-size UI and text, and low-vision players are excluded. The trap is that the UI is readable. The false signal is that the text is clear. The harm is that the fixed-size UI cannot be read by low-vision players, the text is too small, the contrast is insufficient, the players whose visual needs require larger or higher-contrast text are excluded, the game is unplayable or strainful for low-vision players, and the exclusion persists because the UI was not adjustable, because the size was fixed.

### Color-Dependent UI Excluding Colorblind Players

The team designs color-dependent UI without colorblind modes or non-color encoding, and colorblind players are excluded. The trap is that the color is informative. The false signal is that the UI is clear. The harm is that the color-dependent UI cannot be perceived by colorblind players, the information encoded in color is invisible, the modes and encoding that would include them are absent, the game is confusing or unplayable for colorblind players, and the exclusion persists because the color was not supplemented, because the UI was color-dependent.

### Audio-Only Information Excluding Deaf Players

The team conveys critical information through audio only, and deaf players are excluded. The trap is that the audio is clear. The false signal is that the sound works. The harm is that the audio-only information cannot be perceived by deaf players, the dialogue and cues are inaudible, the subtitles and captions that would include them are absent or incomplete, the game is confusing or unplayable for deaf players, and the exclusion persists because the audio was not supplemented, because the information was audio-only.

### Fixed-Difficulty Fixed-Control Design Excluding Players

The team ships fixed difficulty and controls without assist options, and players who need assistance are excluded. The trap is that the challenge is the design. The false signal is that the game is fair. The harm is that the fixed-difficulty fixed-control design excludes players whose motor or cognitive abilities require assistance, the aim assist, timing adjustments, and simplified controls that would include them are absent, the players who need accommodation cannot play, and the exclusion persists because the assistance was not offered, because the design was fixed.

### Single-Input UI Excluding Players

The team designs UI that navigates with only one input, and players using other inputs are excluded. The trap is that the input is standard. The false signal is that the UI works. The harm is that the single-input UI (mouse-only menus) cannot be navigated by gamepad or alternative-input players, the navigation does not work for their input, the players using non-default inputs are excluded from the interface, the game is unplayable for players whose input the UI did not support, and the exclusion persists because the navigation was single-input, because the UI did not work across inputs.

## Self-Check

- Is input fully remappable with custom bindings for motor-diverse players?
- Are UI scale, text size, and contrast adjustable for low-vision players?
- Are colorblind modes and non-color information encoding implemented for colorblind players?
- Are comprehensive subtitles and audio captions provided for deaf and hard-of-hearing players?
- Are assist options (aim assist, timing, simplified controls) offered for motor and cognitive needs?
- Does UI navigation work across all input methods (gamepad, keyboard, mouse, alternatives)?
- Did I confirm the UI and controls accommodate the full range of needs rather than locking players out?
