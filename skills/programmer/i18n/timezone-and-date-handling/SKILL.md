---
name: timezone_and_date_handling.md
description: Use when the agent is storing, converting, or displaying dates and times, handling timezones and UTC conversion, dealing with daylight saving time transitions, computing date ranges and durations, scheduling recurring events, handling historical and future timezone changes, or formatting dates and times for users across regions.
---

# Timezone and Date Handling

Datetime handling is one of the most bug-prone areas in software, and the bugs are uniquely painful because they are silent until they matter: a scheduled event fires an hour early or late, a record's timestamp is off, a recurring meeting drifts, a "daily" report covers 23 or 25 hours twice a year. These bugs come from a small set of conceptual errors that are easy to make and hard to notice in testing, because the system clock and the developer's local timezone often conspire to hide them. Agents who have not been burned by datetime bugs tend to treat dates as simple values and discover, usually in production around a daylight saving transition, that time is governed by a tangle of human conventions that no amount of arithmetic can make clean.

The judgment problem is that "a time" is ambiguous until you know what kind of time it is, and the kind determines how it must be stored, converted, and compared. There are instants (a specific moment on the global timeline, unambiguous), wall-clock times (what a clock in a specific timezone shows, which is ambiguous without the timezone), and calendar dates (a day, with no time of day). There are recurring schedules (every Tuesday at 9am in some timezone) that must survive DST transitions and timezone changes. There are historical and future times, where the timezone rules themselves were or will be different. The agent must know which kind of time they are handling at each point, store each kind correctly, and never silently convert between kinds. Most datetime bugs are silent kind-conversion errors.

## Core Rules

### Know which kind of time you are handling, and keep them distinct

The root of most datetime bugs is conflating different kinds of time. An instant is a moment on the global timeline (best stored as UTC with an offset or epoch). A wall-clock time is what a local clock shows (meaningful only with its timezone, and ambiguous during DST fall-back transitions). A calendar date is a day with no time (a birthday, a holiday). A recurring schedule is a rule (every Tuesday at 9am in America/New_York) that must be re-resolved as dates approach. Know which kind you have at every point in the code, store each appropriately, and never convert between kinds without an explicit decision. Document the kind in field names or types (`created_at` as UTC instant vs. `meeting_local_time` as wall-clock with tz) so the distinction survives code review.

### Store and transmit instants in UTC, convert to a timezone only for display

For any timestamp that represents a specific moment (event time, log time, created_at, transaction time), store and transmit it in UTC (or as an unambiguous instant with offset). UTC has no DST and no ambiguity, so stored instants are comparable, sortable, and arithmetic-safe. Convert to a user's local timezone only at the display layer, when rendering the instant for a human. Never store local wall-clock times as if they were instants; a stored "2024-03-10 02:30:00" with no timezone is uninterpretable (in America/New_York, that time did not exist in 2024 due to the spring-forward gap). The rule is: UTC internally, local for display.

### Store the timezone identifier (IANA name), not just the offset

A UTC offset (like -05:00) tells you the offset at one moment but not the timezone's rules, so it cannot be used to compute future times or handle DST. America/New_York is -05:00 in winter and -04:00 in summer; storing only -05:00 loses that. Store the IANA timezone identifier (e.g., `America/New_York`, `Europe/Paris`) whenever you need to interpret or re-resolve a wall-clock time, schedule a recurring event, or display a future time correctly. The offset is derived from the identifier plus the instant; it is a computed value, not a stored one. Storing offsets instead of identifiers is a common cause of scheduling bugs across DST boundaries.

### Handle DST gaps and overlaps explicitly

Twice a year, most DST-observing timezones have a discontinuity. In spring, the clock jumps forward (a gap: times like 02:00-03:00 do not occur). In fall, the clock jumps back (an overlap: times like 01:00-02:00 occur twice). These break naive arithmetic and create ambiguity. For a time in the spring gap, decide a policy (does it resolve to the post-gap instant, or is it rejected as invalid?). For a time in the fall overlap, the wall-clock time alone is ambiguous; you need the offset or a disambiguation policy (earlier or later occurrence) to know which instant it maps to. Use a mature datetime library that handles these; never hand-roll datetime arithmetic. Test scheduling and conversions across the DST transitions of every timezone you support.

### Store recurring schedules as rules in their origin timezone, and re-resolve as needed

A recurring event ("every Tuesday at 9am") must be stored as a rule (day of week, time of day, IANA timezone), not as a series of precomputed UTC instants, because the UTC instant changes when the timezone's offset changes (DST). Re-resolve the rule to a concrete instant close to when it fires (or in a bounded look-ahead window), so the DST transition is applied correctly. A schedule precomputed far in advance will be wrong across a DST boundary. Be aware that some timezones change their rules (a country abolishes or alters DST), so even re-resolution depends on current timezone database versions; keep the tz database updated.

### Keep the timezone database updated, and understand historical and future rules

Timezone rules are political, not physical: countries change them unilaterally and sometimes on short notice. The IANA timezone database is updated several times a year to track these changes. If your system uses a stale database, it will compute times incorrectly for the affected regions, including for future events whose correct instant depends on a rule change you have not picked up. Keep the tz database current in all environments (language runtime, database, OS), and be aware that historical times may use different rules than the present (a timestamp from 1985 in a region that has since changed its rules must be interpreted with the 1985 rules, which the database tracks). Never assume timezone rules are stable or that your bundled database is current.

### Compute durations and ranges in the right unit, and watch for DST in day-based math

Durations are ambiguous: a "day" can mean 24 hours (a fixed duration) or a calendar day (which is usually 24 hours but is 23 or 25 on DST transitions, and varies with other rule changes). Know which you mean. For scheduling and human-facing ranges ("the report for Tuesday"), use calendar-day arithmetic in the relevant timezone, which correctly handles DST. For elapsed-time measurement ("how long since the event"), use fixed-duration (seconds or hours) arithmetic on UTC instants. Mixing the two produces off-by-one-hour bugs around DST. Be explicit in APIs and storage about whether a duration is wall-clock/calendar-based or fixed/elapsed.

### Validate and parse defensively, and never trust a datetime without an offset or timezone

Inputs are a major source of datetime bugs. A datetime string without an offset or timezone is ambiguous and should be rejected or assigned a documented default, never silently assumed to be UTC or local. Parse with a strict library that requires the offset/timezone, validate that the resulting instant is real (not in a DST gap, or handle the gap per policy), and reject or flag malformed input. Treat "the user sent a time with no timezone" as an error to resolve, not a value to guess at. Logging and APIs should always include the offset so the instant is unambiguous in retrospect.

## Common Traps

### Storing wall-clock local times without a timezone

A stored "02:30" with no timezone is uninterpretable (did it exist? which occurrence in a fall overlap?). Store instants in UTC, and store IANA timezone identifiers for wall-clock times.

### Storing the offset instead of the IANA timezone identifier

An offset loses DST and rule information, so future times and recurring schedules are computed wrong. Store the IANA name and derive the offset.

### Naive datetime arithmetic across DST boundaries

Adding 24 hours to an instant does not yield the same wall-clock time tomorrow during a DST transition. Use calendar-day arithmetic in the right timezone for human schedules, fixed-duration for elapsed time.

### Conflating UTC instants with local wall-clock times

Silently converting between kinds produces comparison and scheduling bugs. Know the kind at every point and convert only with an explicit decision.

### Precomputing recurring schedules far in advance

A schedule resolved to UTC instants months early will be wrong across a DST boundary. Store the rule in its timezone and re-resolve near fire time.

### Assuming the bundled timezone database is current

Timezone rules change several times a year. A stale database computes wrong times for affected regions, including future events. Keep the database updated everywhere.

### Accepting datetime input without an offset or timezone

An offset-less datetime is ambiguous. Reject or assign a documented default; never silently guess UTC or local.

## Self-Check

- Do you know, at every point in the code, whether you are handling an instant, a wall-clock time, a calendar date, or a recurring schedule, and is the kind documented in field names or types?
- Are instants stored and transmitted in UTC (or as unambiguous instants with offset), with conversion to a local timezone done only at the display layer?
- Is the IANA timezone identifier (not just the offset) stored wherever a wall-clock time, recurring schedule, or future time must be interpreted?
- Does your datetime handling explicitly account for DST spring-forward gaps and fall-back overlaps, using a mature library rather than hand-rolled arithmetic?
- Are recurring schedules stored as rules in their origin timezone and re-resolved close to fire time, rather than precomputed as UTC instants far in advance?
- Is the IANA timezone database kept current in all environments (runtime, database, OS), and do you account for historical rule differences when interpreting past timestamps?
- Are durations and ranges computed with the correct unit (calendar-day arithmetic in the right timezone for human schedules, fixed-duration for elapsed time), with the choice explicit in APIs?
- Is datetime input parsed strictly with a required offset or timezone, rejecting or defaulting offset-less values rather than silently assuming UTC or local?
- Have you tested scheduling, conversion, and range math across the DST transitions of every timezone you support?
