---
name: visual-and-audio-accessibility.md
description: Use when the agent is designing color-coded information, subtitles, audio cues, HUD readability, contrast settings, colorblind modes, screen reader support, or reviewing whether a game communicates state through more than one sensory channel.
---

# Visual and Audio Accessibility

Visual and audio accessibility is the discipline of making sure every piece of information the game communicates reaches the player through at least one channel they can actually perceive, and ideally through more than one. Designers miss this because their own senses are intact and the game communicates fluently to them on every channel at once — the red flash, the low-health heartbeat, the color of the loot beam, the subtitle that confirms the voice line. The judgment problem is that information encoded in a single sensory channel becomes a hard wall for any player missing that channel: the colorblind player who cannot distinguish the red and green beams, the deaf player who never hears the audio cue that a stealth enemy is approaching, the low-vision player who cannot read the small-font tooltip. The harm is that these players do not experience a slightly worse version of the game; they experience a game that silently withholds critical state, and they cannot tell whether they failed because of skill or because information was never delivered. The agent has wide latitude in implementation, but the governing principle is redundancy: no gameplay-critical fact should ride on a single sense.

## Core Rules

### Never Encode Critical Information in Color Alone

Color is a wonderful secondary channel but a fatal primary one. Loot rarity, enemy threat level, friendly versus hostile units, status effects, and objective markers are routinely color-coded, and roughly one in twelve men has some form of color vision deficiency. The rule is that any information conveyed by color must also be conveyed by shape, icon, pattern, label, or position. Decision criterion: if you desaturate the screen to grayscale, can a player still read every gameplay-critical state? If not, the design excludes colorblind players. Provide a colorblind mode that remaps palettes, but do not rely on it — grayscale robustness is the real test.

### Make Subtitles and Captions On by Default and Fully Featured

Subtitles are not a courtesy; they are the primary access path for deaf and hard-of-hearing players, and they benefit the majority of players in noisy or muted environments. Subtitles must be on by default, must cover all dialogue including background and incidental lines, and must include non-dialogue audio via captions (e.g., "[glass shattering]", "[footsteps approaching]"). They must be resizable, repositionable, and carry speaker names and color coding that itself passes the color-alone test. Decision criterion: a player playing with sound off should receive every narrative and gameplay-relevant audio fact through text. Background chatter that conveys story or warnings is not optional flavor if it is unsubtitled.

### Deliver Every Audio Cue With a Visual Equivalent, and Vice Versa

The reload sound, the low-health beep, the stealth-detected sting, the boss-enrage audio — each of these is critical state for someone. The rule is bilateral redundancy: every gameplay-critical audio cue needs a visual equivalent (icon, screen edge flash, HUD indicator), and every gameplay-critical visual cue that a blind or low-vision player might miss needs an audio equivalent. Decision criterion: can the game be played with the sound off, and can the core loop be navigated with significant vision loss using audio alone? Full audio-only play is a high bar, but the direction matters — every step toward bilateral redundancy expands the audience.

### Treat Contrast and Text Size as Accessibility Infrastructure

HUD text, tooltips, subtitles, and menu options are routinely shipped at sizes and contrast ratios that fail for low-vision players, players on small screens, and players in bright environments. The rule is to meet or exceed recognized contrast standards (commonly 4.5:1 for body text, higher for small text), to allow text scaling without layout breakage, and to avoid relying on thin fonts or low-weight type at small sizes. Decision criterion: can a player with moderate low vision read every piece of text the game requires them to read, at default settings, and can they scale it up if not? If text scaling breaks the UI, the UI was not built to be accessible.

### Support Screen Readers and UI Traversal for Blind and Low-Vision Players

Full screen-reader support is a major undertaking but is increasingly expected, particularly in menu-heavy, turn-based, or text-driven games. The judgment is to assess early whether the game's genre and audience warrant it, and if so, to build the UI with traversal, focus order, and semantic labels from the start rather than retrofitting. Decision criterion: are interactive elements announced with their name, state, and purpose, and can a player navigate menus and read game state without sight? Even partial support — readable menus, narrated key state — meaningfully expands access for low-vision players even when full blind play is out of scope.

### Provide Per-Channel Volume Controls and Audio Description Options

A single master volume is not accessibility. Players need independent control over dialogue, music, sound effects, and UI audio so they can prioritize the channel carrying information (usually dialogue) and reduce the channel that masks it (usually music or ambient effects). Where cinematics convey spatial or action information visually, audio description tracks provide access for blind players. Decision criterion: can a player rebalance the mix so that subtitles-plus-boosted-dialogue is intelligible over the score, and are cutscenes that convey plot through visuals described for non-sighted players?

## Common Traps

### The Grayscale Test Failure

A designer builds a beautiful color-coded loot and threat system and never checks it without color. The trap is that color feels universal to a designer with typical vision, so the single-channel encoding goes unnoticed. The false signal is that the colors "look distinct" on a calibrated screen to typical vision; the harm is that colorblind players see identical items as the same rarity or misread enemy threat, making the game confusing or unplayable, and the team never learns why retention drops.

### Subtitles That Cover Only Main Dialogue

Subtitles are added for the protagonist and named NPCs, but ambient NPC chatter, radio broadcasts, enemy callouts, and environmental announcements are left uncaptioned. The trap is that the team captions what they consider "real" dialogue and treats the rest as flavor, when the rest often carries warnings, story, or objective hints. The false signal is a subtitle option that appears complete; the harm is that deaf players receive a degraded, confusing narrative and miss gameplay cues that hearing players get for free.

### Relying on a Colorblind Mode Instead of Robust Design

A colorblind palette swap is added and treated as the fix, but the underlying design still encodes state in color, so the mode merely shifts which colors are used. The trap is that the mode helps some deficiencies and not others, and that it does nothing for grayscale scenarios or for players who do not know which mode to select. The false signal is the presence of the toggle; the harm is complacency — the team stops at the toggle instead of passing the grayscale test, and players with uncommon deficiencies remain excluded.

### Audio Cues With No Visual Backup

A stealth game communicates detection through audio alone — the heartbeat, the musical sting, the guard's callout — and a deaf player has no idea they have been spotted until the visual consequence (a guard running at them) appears, by which point the failure is already locked in. The trap is that the audio cue feels essential to the tension and the designer does not want to "spoil" it with a visual indicator. The false signal is that the tension works for hearing players; the harm is that deaf players play a strictly harder, more arbitrary game and may conclude they are bad at it rather than underserved.

### Tiny, Low-Contrast Stylized Text

The art director chooses a thin, low-weight font at small size for aesthetic reasons, and the team ships it because it looks good in screenshots. The trap is that aesthetic readability for typical vision in ideal conditions is treated as universal readability. The false signal is the clean look in a dark demo room; the harm is that low-vision players, players on phones in sunlight, and players on small displays cannot read critical text, and scaling the text breaks the carefully composed layout.

## Self-Check

- If I desaturate the game to grayscale, can a player still read every gameplay-critical state, or is any fact encoded in color alone?
- Are subtitles on by default, covering all dialogue including ambient and incidental lines, with non-dialogue audio captioned, resizable, and repositionable?
- Does every gameplay-critical audio cue have a visual equivalent, and does every critical visual cue have an audio equivalent where feasible?
- Do HUD, tooltip, subtitle, and menu text meet recognized contrast ratios and scale without breaking layout?
- If the genre warrants it, does the UI support screen-reader traversal with semantic labels, focus order, and announced state?
- Are there independent volume controls for dialogue, music, SFX, and UI, and do cinematics offer audio description where plot is conveyed visually?
- Did I verify redundancy with players who actually lack the channel, rather than assuming the second channel is "nice to have"?
