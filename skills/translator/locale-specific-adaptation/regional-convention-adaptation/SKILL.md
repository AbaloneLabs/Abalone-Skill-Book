---
name: regional_convention_adaptation.md
description: Use when the agent is adapting locale-dependent conventions in a translation, converting and localizing date time number currency measurement unit address telephone name and calendar formats, choosing first-day-of-week and week numbering, or ensuring locale conventions feel native rather than left in source form.
---

# Regional Convention Adaptation

Dates, times, numbers, currency, units of measurement, addresses, telephone numbers, names, and calendars look like formatting, but they are content, and getting them wrong causes confusion, errors, and sometimes harm. A date written as 03/04/2026 means March 4 to a US reader and April 3 to most of the rest of the world, and the difference can cause a missed deadline or a legal misstep. A number written as 1.234 means one point two three four in some locales and one thousand two hundred thirty-four in others. A price left in the source currency means nothing to a reader who cannot convert it. An address in source format cannot be used by a local postal system. A name written given-name-first may invert the reader's identity. These conventions are locale-specific, and a translation that carries them over unchanged is not neutral; it is wrong for the reader. Adaptation is the work of converting these conventions to the target locale's forms so that the text functions for the reader. The judgment problem is that conventions interact with each other, that some conversions carry legal or financial consequences, and that the right adaptation depends on context, not on a mechanical rule. This skill covers how to adapt each convention, where the risks lie, and how to ensure the localized conventions feel native rather than merely converted.

Use this skill when adapting dates, times, numbers, currency, units, addresses, phone numbers, names, or calendars for a target locale, when deciding whether and how to convert, or when ensuring locale conventions are correct and consistent. The goal is to make the conventions function for the target reader, not to leave them in a form only the source reader understands.

## Core Rules

### Convert Dates And Times To The Locale Format Deliberately

Date and time formats are the most error-prone convention because the same digits mean different things in different locales. Convert deliberately, never by assumption.

Determine the locale's date order: month-day-year in the United States, day-month-year in most of Europe and Latin America, year-month-day in many East Asian locales. Determine the separator, slash, period, or hyphen, and whether leading zeros and two-digit years are conventional. Determine whether the locale uses 12-hour or 24-hour time, and how time zones are expressed. For ambiguous numeric dates, never guess; confirm the intended date from context before converting, because a wrong conversion moves the event to the wrong day. Spell out months where ambiguity could cause harm, especially in legal, medical, and scheduling content. Apply the chosen format consistently across the document, because mixing formats within a deliverable signals carelessness and invites misreading.

### Localize Numbers According To Decimal And Grouping Rules

Number formatting differs in decimal separators, grouping separators, and grouping size, and the differences change the value if misread.

Determine whether the locale uses a period or comma for the decimal separator, and the opposite for the thousands grouping, and whether grouping is in thousands or in lakhs as in South Asian conventions. A number like 1,234.56 in US English becomes 1.234,56 in many European locales and must not be copied verbatim. Handle percentages, where the symbol and its spacing differ, and negative numbers, where the notation differs. For large or financial numbers, verify the magnitude survives conversion, because a misplaced separator changes thousands into millions. Where precision matters, in scientific, medical, or engineering content, preserve significant figures through formatting and never round during adaptation.

### Handle Currency With Conversion Awareness

Currency adaptation raises the question of whether to convert the value or only the format, and the choice has financial and legal consequences.

Format-only adaptation changes the symbol and placement to the locale convention but leaves the value in the source currency; this is correct when the source currency is the point, such as a price in euros for a European product. Conversion changes both the currency and the value to the target currency; this is appropriate for reader comprehension but introduces exchange-rate, rounding, and legal-accuracy issues. Never convert currency silently, because a converted price may be wrong by the time it is read, may violate pricing regulations, or may misrepresent an offer. If conversion is required, use a documented rate, state the rate and date, and flag that converted prices are indicative. For contracts, invoices, and regulated pricing, do not convert without explicit authorization, because the source currency amount may be legally binding.

### Convert Measurement Units To The Locale System

Measurement systems differ between metric and imperial, and the wrong system confuses or endangers the reader. Convert deliberately, with attention to precision and to safety.

Determine the locale's system: metric for most of the world, imperial or US customary in the United States, and mixed in the United Kingdom and a few others. Convert distances, weights, volumes, temperatures, and areas to the locale's system. For technical, medical, and safety content, preserve precision and verify the conversion, because a wrong unit conversion in a dose, a specification, or an instruction can cause harm. In some regulated fields, such as pharmaceuticals and aviation, the source unit may be legally required even in a metric locale; confirm before converting. Where both systems are useful, provide both with the source unit primary, but do this deliberately rather than leaving the reader to convert.

### Adapt Address Telephone And Name Formats

Addresses, telephone numbers, and names have locale-specific structures, and leaving them in source form makes them unusable or disrespectful.

For addresses, adapt the field order and format to the locale, recognizing that address components and their order differ, and that some locales use postal codes, prefectures, or building identifiers that others do not. For telephone numbers, format with the correct country code, area code structure, and grouping, and clarify whether a number is domestic or international. For names, respect the locale's given-surname order and naming traditions, including patronymics, middle names, and single names, and do not force a foreign name order onto a locale that uses another. Where a name or address must remain in its original script for legal or delivery reasons, retain it but explain the retention, because forcing a transliteration can break mail delivery or legal identity.

### Respect Calendar And Week Conventions

Calendars and weeks are locale-specific, and assuming one calendar or week structure alienates readers or causes scheduling errors.

Most of the world uses the Gregorian calendar, but some locales and contexts use other calendars, such as Hijri, Hebrew, Persian, or Japanese era calendars, and religious or cultural contexts may require a specific calendar. The first day of the week differs, Sunday in some locales, Monday or Saturday in others, and week numbering schemes differ, with ISO week numbers common in Europe. Adapt calendar references, week starts, and week numbers to the locale, and where a non-Gregorian calendar is relevant, provide it correctly rather than converting mechanically. For scheduling and legal content, confirm the calendar in use, because a date in one calendar maps to a different day in another.

### Apply Conventions Consistently And Document The Rules

Inconsistent convention handling within a deliverable is a defect. Decide the rule for each convention, apply it consistently, and document it.

Maintain a conventions log that records the chosen date format, number format, currency approach, unit system, and address and name conventions for the locale, so every segment follows the same rules. Where the source itself shifts conventions, mirror the shift deliberately rather than letting inconsistency creep in. Coordinate conventions with formatting and engineering teams, because the localized conventions must also be supported by the system that displays them, and a correctly translated date that the software re-formats back to the source convention is still wrong for the reader.

## Common Traps

### Copying Source Dates And Numbers Verbatim

Unconverted dates and numbers confuse or mislead; 03/04 means different months in different locales.

### Misreading Numeric Formats

Swapping decimal and grouping separators changes thousands into fractions and vice versa; verify magnitude through conversion.

### Converting Currency Silently

Silent currency conversion introduces exchange-rate, rounding, and legal-accuracy problems; convert only with authorization and a documented rate.

### Converting Units Imprecisely In Safety Content

Wrong unit conversion in medical, engineering, or safety content can cause harm; preserve precision and confirm regulated requirements.

### Forcing Source Name Or Address Order

Foreign name or address structures can break delivery, legal identity, or respect; adapt to the locale's conventions.

### Assuming One Calendar Or Week Structure

Calendars, first-day-of-week, and week numbering differ; assuming one alienates readers or causes scheduling errors.

### Inconsistent Convention Handling

Mixed formats within a deliverable signal carelessness; decide rules and apply them consistently with a documented log.

## Self-Check

Before approving locale convention adaptation, verify:

- Dates and times were converted to the locale format deliberately, with ambiguous numeric dates confirmed from context and spelled out where harm is possible.
- Numbers follow the locale's decimal and grouping rules, with magnitude and precision preserved through conversion.
- Currency was handled with awareness of format-only versus value conversion, and any conversion used a documented rate and authorization.
- Measurement units were converted to the locale system with precision preserved, and regulated fields requiring source units were confirmed.
- Addresses, telephone numbers, and names were adapted to locale structures without forcing source forms that break usability or respect.
- Calendar, first-day-of-week, and week numbering conventions match the locale, with non-Gregorian calendars handled correctly where relevant.
- Conventions are applied consistently across the deliverable, guided by a documented conventions log.
- Convention choices are coordinated with formatting and engineering so the system displays the localized conventions correctly.
- No date, number, currency, unit, or name was left in source form where the target reader needs the locale convention to understand or use it.
