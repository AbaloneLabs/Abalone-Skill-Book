---
name: javascript_dates_timezones_and_duration.md
description: Use when the agent is handling dates, times, timezones, or durations in JavaScript, choosing between the built-in Date and libraries (Temporal, date-fns, dayjs, luxon), storing and transmitting timestamps (ISO 8601, UTC, epoch), converting between timezones, computing date arithmetic and ranges, parsing user date input, formatting dates for display by locale, handling DST transitions, or debugging "the date is off by one day" or "timezone shifted by N hours" bugs. Covers UTC vs local semantics, ISO 8601 formats, timezone conversion, DST pitfalls, duration vs timestamp distinction, and the tradeoff between the built-in Date and modern date libraries.
---

# Dates, Timezones, And Duration

Date and time handling is one of the most bug-prone areas of programming because it combines several independent dimensions: an instant in time, a wall-clock reading in a timezone, a calendar date, and a duration. JavaScript's built-in `Date` conflates these and offers weak timezone support, which makes it easy to write code that is off by hours, off by a day, or wrong across DST transitions. The judgment problem is keeping these dimensions distinct, storing and transmitting instants in UTC, and converting to local time only at the display boundary.

Agents tend to store local times, parse ambiguous date strings, do date arithmetic with milliseconds that ignores DST, or use `Date` for calendar operations it was not designed for. The harm appears as events scheduled at the wrong hour across timezones, reports that shift by a day for users in different zones, recurring meetings that drift at DST boundaries, and durations that miscount because a day was 23 or 25 hours. The real work is treating an instant as a UTC moment, a wall-clock reading as timezone-dependent, a calendar date as a separate concept, and a duration as yet another, and never confusing them.

## Core Rules

### Distinguish Instants, Wall-Clock Times, Calendar Dates, And Durations

These are different concepts that `Date` conflates. Keeping them straight prevents most date bugs.

- **Instant**: a unique moment in time (a UTC timestamp, an epoch millisecond). "When did the event happen?" Two observers anywhere agree on the instant.
- **Wall-clock time**: what a clock reads in a specific timezone (`2026-07-07 14:00 America/New_York`). The same instant reads differently in different zones.
- **Calendar date**: a day on a calendar (`2026-07-07`), with no time or zone. Birthdays, due dates, and "daily" schedules are calendar dates.
- **Duration**: an amount of time (`PT2H`, two hours), not anchored to an instant. "How long did it take?"

The root of most bugs is treating a wall-clock time as if it were an instant, or a calendar date as if it had a time. Ask which dimension you actually have, and use the right representation.

### Store And Transmit Instants In UTC (ISO 8601 With Offset)

For any moment that crosses systems or timezones, store and transmit it as a UTC instant. The robust formats are:

- **ISO 8601 with a `Z` or explicit offset**: `2026-07-07T14:00:00Z` (UTC) or `2026-07-07T14:00:00+00:00`. The offset/Z makes the instant unambiguous.
- **Epoch milliseconds** (`Date.now()`): a single integer, timezone-proof, easy to sort and compare. Good for storage and logs.

Avoid storing a local wall-clock time without a timezone (`2026-07-07T14:00:00`), which is ambiguous — two readers in different zones interpret it differently. If you must store a wall-clock time, store the timezone alongside it.

### Convert To Local Time Only At The Display Boundary

Keep the instant in UTC through all storage, computation, and transmission. Convert to a local wall-clock reading only when showing it to a user, using their timezone.

- `new Date(epoch).toLocaleString(locale, { timeZone: "America/New_York" })` converts a UTC instant to a local reading.
- Libraries (`Temporal`, `luxon`, `dayjs` with timezone plugin) make explicit timezone conversion clearer than the built-in `Date`.
- Never call `getHours()`/`getDate()` on a `Date` for anything other than the user's local zone, because those methods use the runtime's local zone and produce wrong results on a server in a different zone.

The classic "off by N hours" bug comes from calling local-zone methods on a server whose zone differs from the user's. Convert explicitly with the user's timezone.

### Do Not Do Calendar Arithmetic With Milliseconds

Adding `24 * 60 * 60 * 1000` milliseconds to compute "tomorrow" is wrong across DST transitions, where a day is 23 or 25 hours, and across leap seconds in some contexts.

- For "same time tomorrow", use a date library that operates in calendar units (`addDays(date, 1)`) and respects the calendar, not raw milliseconds.
- For "add one month", calendar arithmetic is required (months have different lengths); millisecond arithmetic cannot do it.
- Durations in milliseconds are correct only for elapsed real time between two instants, not for calendar-unit scheduling.

### Handle DST Transitions Explicitly

Daylight Saving Time transitions create 23-hour days (spring forward) and 25-hour days (fall back). Code that assumes every day has 24 hours breaks twice a year in DST zones.

- "Every day at 09:00" scheduled by adding 24 hours drifts by an hour at each transition. Schedule by wall-clock time and let the library handle the gap/overlap.
- During a fall-back overlap, a local time like `01:30` occurs twice; an instant is ambiguous without knowing which side. During spring-forward, a local time like `02:30` does not occur at all.
- For scheduling, prefer a library that models wall-clock recurrence and resolves DST explicitly.

### Parse User Input Strictly And Validate

User date input is ambiguous (`07/08/26` — is that July 8 or August 7?). Parse strictly with an explicit format or a date library, and validate the result.

- Prefer ISO 8601 (`YYYY-MM-DD`) for input where you control the format; it is unambiguous.
- For free-form input, use a date parsing library with an explicit format string and reject unparseable input rather than guessing.
- Validate ranges (no month 13, no day 32, handle February and leap years). The built-in `Date` rolls over (`new Date(2026, 12, 32)` becomes a valid date in the next year), masking invalid input.

### Format For Display By Locale

Display formatting depends on the user's locale (month/day order, separators, 12 vs 24 hour, month names). Use `Intl.DateTimeFormat` or a locale-aware library.

- `Intl.DateTimeFormat('en-US').format(date)` and `Intl.DateTimeFormat('de-DE').format(date)` produce locale-correct output.
- Include the timezone in the format when the instant's zone matters to the reader.
- Avoid hand-rolled string formatting, which produces wrong ordering and names for other locales.

### Prefer A Modern Date Library For Non-Trivial Work

The built-in `Date` is mutable, has poor timezone support, and conflates instants with wall-clock times. For anything beyond "get current epoch" or simple formatting, prefer:

- **Temporal** (the upcoming built-in): cleanly separates `Instant`, `ZonedDateTime`, `PlainDate`, `Duration`.
- **luxon / dayjs / date-fns**: mature libraries with explicit timezone and calendar support.

Reach for these whenever you do timezone conversion, calendar arithmetic, recurrence, or duration computation. The built-in `Date` will cost more in bugs than it saves in dependencies.

## Common Traps

### Storing Local Time Without Timezone

`2026-07-07T14:00:00` is ambiguous. Store UTC (`...Z`) or epoch, or store the timezone alongside the wall-clock time.

### Local-Zone Methods On A Server

`date.getHours()` uses the server's local zone, producing wrong times for users in other zones. Convert explicitly with the user's timezone.

### Millisecond Arithmetic Across DST

Adding `86400000` ms for "tomorrow" drifts at DST transitions where days are 23 or 25 hours. Use calendar-unit arithmetic.

### Treating A Calendar Date As An Instant

A birthday or due date is a calendar date, not a moment. Storing it as a midnight instant in some zone shifts it for users in other zones. Store calendar dates as `YYYY-MM-DD` strings.

### Ambiguous User Input Parsing

`07/08/26` parsed by the built-in `Date` guesses a format, often wrongly. Parse with an explicit format and validate.

### `Date` Rollover Masking Invalid Input

`new Date(2026, 12, 32)` silently rolls over to a valid date. Validate ranges explicitly rather than trusting construction.

### Assuming 24-Hour Days In Scheduling

Recurring schedules built on 24-hour increments drift at DST. Schedule by wall-clock time with a DST-aware library.

### Hand-Rolled Locale Formatting

Building date strings by hand produces wrong ordering, separators, and names for other locales. Use `Intl.DateTimeFormat`.

## Self-Check

- [ ] Instants, wall-clock times, calendar dates, and durations are treated as distinct concepts, not conflated through a single `Date`.
- [ ] Instants are stored and transmitted in UTC (ISO 8601 with `Z`/offset, or epoch milliseconds); local wall-clock times are never stored without a timezone.
- [ ] Conversion to local time happens only at the display boundary, using the user's explicit timezone, not the server's local zone.
- [ ] Calendar arithmetic (add a day/month, recurrence) uses calendar-unit operations, not raw millisecond addition that breaks across DST.
- [ ] DST transitions (23- and 25-hour days, gaps and overlaps) are handled explicitly for scheduling and duration.
- [ ] User date input is parsed with an explicit format and validated for range; the built-in `Date`'s silent rollover is not trusted to validate input.
- [ ] Calendar dates (birthdays, due dates) are stored as `YYYY-MM-DD` strings, not as midnight instants in an arbitrary zone.
- [ ] Display formatting uses `Intl.DateTimeFormat` or a locale-aware library, not hand-rolled string building.
- [ ] Non-trivial date work (timezone conversion, calendar arithmetic, recurrence, duration) uses Temporal or a mature library rather than the built-in `Date`.
