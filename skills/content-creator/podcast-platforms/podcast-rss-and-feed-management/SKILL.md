---
name: podcast_rss_and_feed_management.md
description: Use when the agent is managing a podcast RSS feed, writing and validating feed tags, handling feed redirects and migration, troubleshooting distribution errors, or deciding how feed structure affects directory updates and listener continuity.
---

# Podcast RSS And Feed Management

The RSS feed is the single pipe through which every directory receives a podcast, and it is also the most fragile and least visible part of the whole operation. The judgment problem is that creators treat the feed as background infrastructure they never touch, when in fact a single malformed tag, a broken enclosure URL, or a botched migration can silently drop the show from every directory at once. Worse, feed mistakes during migration can sever existing subscribers from the show permanently, because listeners are subscribed to a feed URL, not to the show's identity.

The harm is silent, total distribution failure and irreversible subscriber loss. A feed error breaks every directory simultaneously and often invisibly, since listeners do not report missing episodes. A migration done without a proper redirect strands subscribers on the old feed. Inconsistent or missing tags cause directories to misread or reject the show. This skill helps the agent manage the RSS feed as the critical asset it is, validate and maintain feed integrity, handle redirects and migration without losing subscribers, and troubleshoot the distribution errors that otherwise stay hidden until they cause real damage.

## Core Rules

### Treat The Feed As The Single Critical Asset

Every directory depends on the RSS feed. A feed problem is not a minor technical issue, it is a total distribution failure.

Treat the feed as critical by:

- understanding that all directories read the same feed;
- monitoring feed health as part of routine operations;
- validating the feed after any change;
- keeping feed uptime and reliability a top priority;
- not assuming the host handles feed health automatically.

The feed is the show's lifeline to every directory. Treating it as background infrastructure invites silent failure.

### Validate The Feed After Every Change

Feed changes, new tags, artwork updates, hosting modifications, can introduce errors that break directory updates.

Validate by:

- running a feed validator after any change;
- checking that required tags remain complete and correct;
- verifying enclosure URLs resolve to valid audio files;
- confirming new episodes appear in the feed promptly;
- not making unvalidated manual feed edits.

Validation catches errors before they reach directories. Skipping it risks silent breakage across every platform.

### Keep Required And Recommended Tags Complete

Directories rely on specific RSS tags to list and display the show correctly. Missing or malformed tags cause misreading or rejection.

Manage tags by:

- keeping required tags like title, link, description, and enclosure complete;
- using recommended tags for artwork, categories, and episode details;
- following the tag formats each directory expects;
- not leaving optional-but-important fields empty;
- updating tags when directory requirements change.

Complete, correct tags ensure directories display and rank the show properly. Missing tags cause silent under-display or rejection.

### Ensure Enclosure URLs Are Stable And Valid

The enclosure tag points to the actual audio file. Broken, changed, or expiring enclosure URLs break episode playback everywhere.

Ensure enclosure integrity by:

- using stable, permanent URLs for audio files;
- not changing file URLs after publication;
- verifying enclosure URLs resolve before and after release;
- avoiding hosting that uses expiring or rotating URLs;
- monitoring for broken audio links.

A broken enclosure URL makes episodes unplayable across all directories. Stable URLs protect playback reliability.

### Handle Migration With A Proper Redirect

When moving hosts, subscribers stay tied to the old feed URL unless a proper redirect is set up. Without it, the show loses its audience.

Handle migration by:

- setting up a 301 redirect from the old feed to the new one;
- verifying the new host supports the redirect;
- keeping the old feed alive long enough for listeners to migrate;
- not deleting the old feed prematurely;
- testing that subscribers follow the redirect.

A proper redirect carries subscribers to the new feed. A botched migration strands them on a dead feed permanently.

### Preserve The Feed URL As The Show's Identity

Listeners subscribe to a feed URL. Changing that URL without a redirect severs the relationship, because there is no automatic re-subscription.

Preserve identity by:

- keeping the feed URL stable across the show's life;
- using a custom domain for the feed where possible for portability;
- planning any URL change with a redirect in advance;
- never silently changing the feed URL;
- treating the URL as a permanent subscriber relationship.

The feed URL is the subscriber contract. Protecting it protects the audience the show has built.

### Troubleshoot Distribution Errors Systematically

When episodes fail to appear or update, the cause usually lies in the feed, the host, or a specific directory's processing.

Troubleshoot by:

- checking the feed first for validity and new episodes;
- verifying enclosure URLs and audio file availability;
- checking whether the issue affects all directories or one;
- reviewing host status for outages or processing delays;
- distinguishing feed errors from directory-specific delays.

Systematic troubleshooting isolates whether the problem is the feed, the host, or a directory, and prevents misdiagnosis.

### Plan For Feed And Directory Changes Over Time

Directory requirements, feed standards, and hosting features evolve. A feed that worked at launch can silently fall out of compliance.

Plan for change by:

- reviewing feed compliance periodically against current standards;
- updating tags when directories revise requirements;
- monitoring hosting and feed feature changes;
- re-validating after platform-side changes;
- not assuming launch-time compliance lasts forever.

Periodic review keeps the feed compliant and reliable as standards and platforms shift over the show's life.

## Common Traps

### Treating The Feed As Background Infrastructure

Ignoring feed health until something breaks allows silent total distribution failure across every directory.

### Skipping Validation After Changes

Unvalidated feed edits introduce errors that break directory updates and episode appearance.

### Changing Or Breaking Enclosure URLs

Altering audio file URLs after publication breaks episode playback across all directories silently.

### Migration Without A Proper Redirect

Moving hosts without a 301 redirect strands existing subscribers on the dead old feed permanently.

### Silently Changing The Feed URL

Changing the feed URL without a redirect severs the subscriber relationship because listeners are tied to the URL.

### Incomplete Or Malformed Tags

Missing required or recommended tags cause directories to misread, under-display, or reject the show.

### Assuming Launch-Time Compliance Lasts

Failing to review feed compliance as directory requirements evolve lets a working feed silently fall out of standards.

## Self-Check

Before approving podcast RSS and feed management, verify:

- The RSS feed is treated as the single critical asset connecting the show to every directory.
- The feed is validated after every change, with required tags complete and correctly formatted.
- Enclosure URLs are stable, permanent, and resolve to valid audio files.
- Any host migration includes a proper 301 redirect to carry subscribers to the new feed.
- The feed URL is preserved as the show's subscriber identity, with no silent URL changes.
- Required and recommended tags are complete and follow each directory's expected formats.
- Distribution errors are troubleshot systematically to isolate feed, host, or directory causes.
- The old feed is kept alive long enough during migration for listeners to follow the redirect.
- Feed compliance is reviewed periodically against current directory and RSS standards.
- Feed health is monitored as routine operations, not only when a problem appears.
