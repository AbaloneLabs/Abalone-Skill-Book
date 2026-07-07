---
name: cultural_and_regional_adaptation.md
description: Use when the agent is designing for audiences in different countries or regions, adapting color, imagery, symbols, names, dates, units, payment methods, etiquette, hierarchy, or content to local cultural expectations, or when reviewing whether a design assumes one culture's norms as universal.
---

# Cultural And Regional Adaptation

Cultural adaptation is deeper than translation. Two users who read the same words can interpret color, layout, imagery, names, numbers, and hierarchy in opposite ways because of the cultural context they bring. A color that signals celebration in one region signals mourning in another; a gesture that is friendly in one country is offensive in a second; a name format that is universal to one culture is impossible in a third. Designing as if one culture's conventions are universal produces interfaces that feel foreign, wrong, or insulting to everyone outside the origin culture.

Use this skill before designing or reviewing products intended for more than one country or region, selecting color and imagery for international audiences, designing forms and identity systems, choosing symbols and icons, or deciding what counts as polite, clear, or trustworthy. The goal is to prevent the agent from exporting one culture's assumptions as if they were neutral, and from treating adaptation as a translation layer rather than a design decision.

## Core Rules

### Question Every Assumption Of Universality

Most cultural mistakes come from treating a local convention as a law of nature. Name structure, address format, date order, the start of the week, the first day of the year, the meaning of colors, the reading of symbols, the appropriate level of formality, and the expected hierarchy of information all vary. Before treating any of these as fixed, ask which culture defined them and whether they hold elsewhere.

Maintain a habit of asking, for each convention: is this universal, or is this the convention of the origin market?

### Adapt Color Meaning, Not Just Color Values

Color carries strong cultural meaning that is rarely universal. White is purity and weddings in some regions and mourning in others. Red is danger or loss in some financial contexts and prosperity or gain in others. Green has religious significance in some markets and environmental meaning in others. A status palette that reads correctly in one region can confuse or offend in another.

When designing for multiple regions:

- do not assume red means error and green means success everywhere;
- check financial color conventions, which invert between some markets;
- verify that brand and accent colors carry no unintended religious or political meaning;
- reinforce status with text and icon so meaning never depends on color alone;
- allow regional palette variation where a single global choice causes confusion.

### Design Identity And Data Inputs For Global Variation

Names, addresses, phone numbers, and family structure vary far more than most form designs allow. Some people have one name; some have many. Some cultures put the family name first; some have no family name. Addresses may rely on landmarks rather than numbered streets. Phone numbers vary in length and format. Gender categories that are binary in one culture are not in another.

Build inputs that accept:

- single names, multiple names, and names in any script;
- varied address structures, including those without street numbers;
- international phone formats and country codes;
- flexible date entry matching the local convention;
- identity categories that do not force an inappropriate structure.

Where a structure is required by the system, explain why and choose the most inclusive option.

### Adapt Imagery, Symbols, And Gestures

Visual elements carry cultural load that translation cannot fix. Hand gestures, body language, clothing, religious symbols, animals, and even numbers can be benign in one culture and offensive in another. The number four is associated with death in some East Asian cultures; certain hand gestures are deeply offensive in parts of the Americas, Europe, and the Middle East; an image of a shoe sole is insulting in some Arab cultures.

When imagery will travel across markets:

- avoid hand gestures unless their meaning is verified in each market;
- check animals, numbers, and symbols for negative associations;
- verify that clothing and body exposure suit the target audience;
- prefer locally appropriate photography over universal reuse;
- involve reviewers familiar with the target culture where feasible.

### Respect Local Conventions For Dates, Time, Calendars, And Units

The order of date components, the first day of the week, the calendar system, the clock format, the measurement system, and the decimal separator all vary. A date written as 03/04/2026 means different months in different regions. Some markets use non-Gregorian calendars for civil or religious purposes. Mixing metric and imperial units causes real errors.

Use locale-aware formatting, label ambiguous formats explicitly, and never assume the source market's convention is the default. Where a single global format is forced, make the format unambiguous, such as spelling out the month.

### Adapt Tone, Formality, And Hierarchy

The appropriate tone and level of formality vary by culture. Some audiences expect direct, informal communication; others expect formality and honorifics. The expected hierarchy of information, the role of authority and trust signals, and the acceptable directness of calls to action all shift. A playful, first-name tone that works in one market can feel disrespectful in another.

Consider:

- formality of address, including pronouns and honorifics;
- directness of calls to action and error messages;
- prominence of trust signals such as certifications, endorsements, and local institutions;
- the role of community and social proof, which varies in weight by culture.

### Localize Payment, Contact, And Commerce Expectations

Commerce conventions differ widely. Payment methods, delivery expectations, tipping, taxation display, return policies, and customer service channels that are standard in one market may be absent or inappropriate in another. Forcing a single payment or contact model excludes users who rely on local alternatives.

Adapt to local payment methods, display tax in the locally expected way, and offer contact and support channels that match how users in each market actually communicate.

## Common Traps

### Treating Origin-Culture Norms As Universal

Assuming the source market's conventions for names, dates, colors, and tone are neutral exports them as defaults that exclude or confuse other audiences.

### Color Meaning Assumed Constant

A status or brand palette that reads correctly in one region can signal the opposite, such as red for gain versus loss in finance, elsewhere.

### Forms That Reject Real Names And Addresses

Inputs built for one name and address structure fail for users whose identity or location does not fit that mold.

### Universal Reuse Of Imagery And Gestures

An image or gesture that is friendly in one culture can be offensive in another, and translation does nothing to fix it.

### Ambiguous Date And Number Formats

A format like 03/04 means different things in different regions, and silent misreading causes real errors.

### One-Size Payment And Contact Model

Forcing a single payment method or support channel excludes users who depend on local alternatives.

### Tone Calibrated To One Market

Playful, informal, or highly direct copy that suits one culture can read as disrespectful or untrustworthy in another.

## Self-Check

- [ ] Each convention used in the design was checked for whether it is universal or specific to the origin culture.
- [ ] Color meaning, especially for status and finance, was verified for each target region rather than assumed constant.
- [ ] Name, address, phone, date, and identity inputs accept global variation rather than one structure.
- [ ] Imagery, symbols, gestures, animals, and numbers were checked for negative cultural associations in each market.
- [ ] Dates, times, calendars, units, and decimal separators use locale-aware formatting, and ambiguous formats are labeled explicitly.
- [ ] Tone, formality, and hierarchy of information were adapted to each audience rather than copied from the source market.
- [ ] Payment methods, tax display, and support channels match local commerce expectations.
- [ ] Trust signals and social proof reflect what each audience actually relies on.
- [ ] Where a single global choice was forced, it was made deliberately and explained, not defaulted by accident.
- [ ] Reviewers familiar with the target cultures were involved where feasible, rather than relying on assumptions.