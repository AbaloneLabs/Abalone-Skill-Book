---
name: web_scraping_and_digital_data_collection.md
description: Use when the agent is collecting data from digital sources such as web scraping, public APIs, social media platforms, digital trace data, or app logs, and must judge technical feasibility, terms of service compliance, ethical scraping behavior, data representativeness, and the gap between what is digitally available and what the research question actually requires.
---

# Web Scraping And Digital Data Collection

Digital trace data tempts researchers with scale and convenience. Millions of posts, transactions, clicks, and records appear to be sitting there, ready to be scraped, and the cost of collection looks negligible compared to a survey or experiment. This appearance is misleading, and acting on it produces research that is technically successful, legally precarious, ethically questionable, and substantively wrong. Digital data are not a census of human behavior; they are the exhaust of platforms whose designs, algorithms, access rules, and user bases determine what is visible. A researcher who scrapes what is easy to reach and analyzes it as if it represented a population has studied the platform's architecture as much as the phenomenon of interest.

Use this skill when planning to collect data from websites, APIs, social media, app logs, or other digital traces. The goal is to keep the agent from treating digital availability as research validity, from violating terms of service or ethical norms in pursuit of data, from assuming digital traces are representative, and from ignoring the gap between the data the platform exposes and the construct the study needs. The agent has latitude in source and technique, but the technical, legal, ethical, and inferential boundaries must each be made explicit and defensible.

## Core Rules

### Judge Technical Feasibility Against Reality, Not Against A Tutorial

Digital sources change constantly. A scraping approach that worked last year, or that worked in a blog post, may be broken, rate-limited, authenticated, or prohibited now. Feasibility must be verified against the live target before committing the study to it.

Assess feasibility concretely:

- whether the data is exposed through a stable documented API, an unstable undocumented endpoint, or HTML parsing;
- pagination, rate limits, authentication, and anti-bot defenses such as captchas or behavioral fingerprinting;
- whether the full target population is reachable or only a recent, filtered, or algorithmically ranked subset;
- the volume, velocity, and structure of the data and whether it can be stored and processed as assumed;
- the durability of access over the data collection window and the risk of mid-study cutoff.

A study design that depends on scraping a source the researcher has not actually tested is a design built on hope. Pilot the collection on the live system before staking the research on it.

### Treat Terms Of Service And Law As Real Constraints

Platform terms of service are not bureaucratic noise; they are enforceable contracts, and in many jurisdictions they intersect with computer-fraud, data-protection, and privacy law. Ignoring them is a risk to the researcher, the institution, and the participants.

Check:

- the platform's terms of service and any specific scraping or API use policy;
- whether an API with an acceptable-use policy exists and whether the planned use fits it;
- whether the data involves personal data protected by law such as GDPR or similar regimes;
- whether authentication or circumvention of access controls is involved, which can cross legal lines;
- institutional review or legal sign-off for novel or large-scale collection.

The existence of a technical path to data does not imply permission to take it. When terms prohibit the intended use, the correct response is to seek an agreement, use the sanctioned API, change the source, or abandon the approach, not to scrape covertly.

### Scrape Ethically Even When Permitted

Permission to collect does not exhaust ethical obligation. Digital collection can burden platforms, expose individuals, and create datasets whose later misuse is hard to control.

Practice ethical collection:

- respect robots.txt and explicit no-scrape signals as a baseline, not a ceiling;
- rate-limit requests to avoid degrading service for the platform and its users;
- collect the minimum necessary rather than hoarding data that may never be analyzed;
- avoid collecting identifying information that the research does not need;
- consider whether users would be surprised or harmed to learn their data was studied;
- plan for secure storage, controlled access, and a defensible retention and deletion policy.

Public does not mean non-personal, and accessible does not mean consensual. The ethical question is not only whether collection is allowed but whether the people whose traces are collected would recognize the research as fair.

### Question Whether Digital Traces Represent The Target Population

This is the central inferential trap. Digital traces represent the users of a platform, filtered by who participates, who is active, who is visible, and what the algorithm chooses to surface. They do not represent a general population unless that is specifically demonstrated.

Interrogate representativeness:

- who uses the platform, who does not, and how that skews the reachable population by age, region, language, and socioeconomic position;
- whether activity volume concentrates among a small subset, so that scraped posts over-represent power users;
- whether algorithmic ranking, recommendation, and moderation shape what is visible independent of true behavior;
- whether bots, spam, influencers, and coordinated campaigns contaminate the trace;
- whether the platform's user base has changed over the collection period, breaking temporal comparability.

A finding derived from a platform's visible traces is, at minimum, a finding about that platform's visible population. Generalizing beyond it requires evidence of representativeness, which digital convenience samples rarely provide.

### Understand What APIs And Scraping Actually Return

The data returned by an API or scraper is not the platform's full data; it is a filtered, paginated, often ranked view defined by the platform's design and business interests. Treating the returned slice as the population is a category error.

Determine:

- what the API or page exposes versus what exists in the platform's backend;
- whether results are chronological, relevance-ranked, personalized, or sampled;
- whether pagination caps, rate limits, or recall limits truncate the accessible set;
- whether deleted, hidden, private, or moderated content is absent and how that biases the record;
- whether the schema and semantics of returned fields are documented and stable.

The gap between digital-available data and research-needed data is often large. A variable that looks like engagement may be a platform-defined composite that changes meaning across releases. Confirm what each field actually measures before building constructs on it.

### Bridge The Gap Between Available Data And The Research Construct

Digital traces rarely measure the construct of interest directly. They measure clicks, posts, follows, and timestamps, which the researcher must map to constructs like trust, polarization, influence, or wellbeing. This mapping is a measurement decision with validity consequences.

Bridge carefully:

- define the construct and then identify which traces operationalize it;
- state explicitly what the trace captures and what it misses;
- avoid relabeling a convenient metric as the construct without validity evidence;
- check whether the same construct is measured comparably across platforms, languages, or time;
- acknowledge the gap between behavior and inference and avoid overclaiming from traces.

A retweet is not endorsement, a click is not preference, and a follow is not trust. The construct validity of digital measures must be argued, not assumed.

### Plan For Volatility, Reproducibility, And Provenance

Digital data are ephemeral. Pages change, APIs deprecate, accounts vanish, and platforms re-rank. A dataset scraped today may be impossible to recreate tomorrow, which threatens reproducibility and auditability.

Plan for volatility:

- capture raw responses with timestamps, request parameters, and source versions;
- document the collection script, seeds, software versions, and any sampling decisions;
- store both raw and parsed data so re-parsing is possible without re-collecting;
- record what could not be collected and why, so the reachable sample is characterized;
- anticipate that platform changes may break replication and document the access conditions that held.

A digital collection that cannot be described in enough detail for another researcher to understand its scope is not a reliable evidence base, however large its volume.

## Common Traps

### Equating Availability With Representativeness

Scraping what is easy to reach and analyzing it as a population studies the platform's visible, active, algorithmically surfaced users rather than the intended population.

### Ignoring Terms Of Service And Law

Treating platform rules as obstacles to route around exposes the researcher and institution to legal, ethical, and reputational risk and can invalidate the data's usable status.

### Aggressive Scraping That Harms Platforms

Hammering endpoints without rate limiting degrades service, can trigger bans, and is ethically indistinguishable from denial of service.

### Treating Public Data As Non-Personal

Publicly accessible posts, profiles, and traces can still identify individuals, reveal sensitive attributes, and cause harm when aggregated or re-identified.

### Assuming API Output Equals Platform Reality

APIs and pages return filtered, ranked, sampled views; treating the returned slice as the full population overstates coverage and misrepresents behavior.

### Relabeling Traces As Constructs

Calling a like engagement or a follow influence without validity evidence turns a convenient metric into a false measure of the phenomenon.

### Unreproducible Snapshots

Collecting data without scripts, timestamps, parameter logs, or raw storage makes the dataset impossible to audit or replicate once the source changes.

### Ignoring Bots And Coordinated Activity

Treating all accounts and posts as genuine human behavior contaminates the trace with spam, automation, and manipulation that distort every downstream measure.

## Self-Check

- Has the live source been piloted so that feasibility, rate limits, authentication, and recall are verified rather than assumed?
- Are platform terms of service, API use policies, and applicable data-protection and computer-fraud law checked, and is the planned use within them or formally agreed?
- Is collection rate-limited, minimal, robots-aware, and free of unnecessary identifying data, with secure storage and a retention plan?
- Has the reachable population been compared to the target population for skew in users, activity, visibility, and bot contamination?
- Is it clear what the API or page actually exposes versus what exists, including ranking, pagination caps, and absent moderated or deleted content?
- Is each research construct explicitly mapped to specific traces, with the gap between trace and construct acknowledged rather than hidden?
- Are field semantics documented and checked for stability across platforms, languages, and time?
- Are raw responses, scripts, seeds, versions, parameters, and sampling decisions recorded so the collection is reproducible and auditable?
- Is the scope of what could not be collected characterized, so the sample's limits are transparent?
- Are conclusions bounded to the platform's visible population unless representativeness is specifically demonstrated?
