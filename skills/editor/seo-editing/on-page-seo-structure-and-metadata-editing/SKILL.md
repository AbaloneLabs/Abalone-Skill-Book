---
name: on_page_seo_structure_and_metadata_editing.md
description: Use when the agent is editing heading hierarchy, title tags, meta descriptions, internal linking and anchor text, structured data and schema markup, URL slugs, or content architecture for both search engine indexability and human usability in web content.
---

# On-Page SEO Structure And Metadata Editing

On-page SEO structure is the skeleton that lets both crawlers and readers make sense of a page. Editors miss the importance of this layer because it is largely invisible: a reader never sees a title tag in the body, a crawler never reads a heading the way a human does, and metadata lives in the document head rather than the prose. Yet these structural elements decide whether a page is understood, indexed, and presented in search results, and whether a human scanning the page can navigate it. Poor structural editing causes concrete harms: a brilliant article never ranks because its headings are decorative, a page earns clicks but loses them because the meta description misrepresents the content, and internal authority fails to flow because links use generic anchor text. The editor's task is to make structure serve two audiences at once, the machine that indexes and the human who reads, without sacrificing either.

## Core Rules

### Enforce A Logical Heading Hierarchy Without Gaps

Headings are not styling tools; they are a semantic outline of the page. The editor must verify that the page has exactly one H1 that describes the whole content, and that subsequent headings descend in order without skipping levels. An H2 may be followed by H3s, but an H2 followed directly by an H4 breaks the outline for both crawlers and assistive technology. Decorative subheads styled to look bigger but tagged as paragraphs or bold text are invisible to the structure. The editor maps the heading tree of every page and confirms it reads as a coherent table of contents: each heading summarizes its section, sibling headings are parallel, and nesting reflects genuine subordination. A clean hierarchy improves search understanding, screen-reader navigation, and the reader's ability to scan.

### Write Title Tags For Both Ranking And The Result Page

The title tag is the most prominent on-page SEO element and the headline readers see in search results. It must contain the primary keyword near the front, accurately describe the page, and stay within the pixel width search engines display before truncation, roughly sixty characters as a practical limit. The editor checks that titles are unique across the site, because duplicate titles confuse indexing and dilute relevance. A title must also compel a click without deceiving; a title that overpromises trains readers to distrust the brand and increases pogo-sticking, which harms rankings. The editor balances keyword presence, accuracy, uniqueness, and appeal, treating the title as the page's single most important advertisement.

### Craft Meta Descriptions That Earn The Right Click

Meta descriptions do not directly influence rankings, but they heavily influence click-through rate, which affects traffic and indirect ranking signals. The editor writes or revises descriptions to summarize the page's value in roughly 150 to 160 characters, include the primary keyword where natural, and end with an implicit reason to click. Descriptions must match the page content; a description that promises something the page does not deliver increases bounce and damages trust. Where the page targets a featured snippet or answers a direct question, the description can preview the answer. The editor treats each description as ad copy constrained by honesty, verifying that it reflects what the reader will actually find.

### Govern Internal Linking And Anchor Text Strategically

Internal links distribute authority across the site and help search engines understand relationships between pages. The editor evaluates whether important pages receive enough internal links from relevant context, whether orphan pages exist without inbound links, and whether anchor text is descriptive rather than generic. Anchor text like "click here" or "learn more" tells neither crawlers nor screen-reader users what the destination contains. Descriptive anchor text, such as "our guide to espresso extraction," signals topical relevance and improves accessibility simultaneously. The editor also checks that links are not over-optimized with exact-match keyword anchors on every reference, which can look manipulative, and that links point to live, relevant destinations.

### Apply Structured Data And Schema Accurately

Structured data, expressed as schema markup, gives search engines explicit information about what a page contains: an article, a product, a recipe, an FAQ, a review, an event. The editor's role is not necessarily to write the markup but to verify that the schema chosen matches the actual content and that the required properties are present and correct. Marking up reviews that do not exist, applying Product schema to a category page, or omitting required fields triggers errors and can suppress rich results. The editor cross-checks the declared schema against the visible page content, because markup that contradicts the page is treated as spam. Where schema is added, the editor confirms it describes content a human can actually see.

### Edit URLs And Content Architecture For Clarity And Stability

URLs are part of the content's identity and a ranking and usability signal. The editor advocates for short, readable, keyword-relevant slugs separated by hyphens, free of unnecessary parameters, dates, or category cruft that bloats the path. URLs should be stable; changing a URL without a redirect breaks links and loses accumulated authority. At the architecture level, the editor considers whether the site's topic grouping makes sense, whether related content is clustered and cross-linked, and whether the depth of clicks from the home page reflects the importance of each page. A flat, logical architecture helps crawlers find content and helps readers understand the site's scope.

### Serve Indexability And Usability Together

Every structural decision has both a crawler consequence and a human consequence. The editor must not optimize one at the expense of the other. A heading stuffed with keywords may help a crawler but repel a reader; a clever title may charm a human but omit the keyword crawlers use to match. The editor's discipline is to find choices that serve both, and where a true conflict exists, to favor the reader, because search engines increasingly reward human-satisfying content. The editor documents structural conventions, such as heading depth limits and title length targets, so that consistency holds across contributors and over time.

## Common Traps

### Using Headings For Visual Styling

Tagging text as an H3 because it looks the right size, or skipping from H2 to H4, corrupts the semantic outline. This trap occurs because authors think visually rather than structurally. The editor must enforce hierarchy by meaning, not appearance, and route styling through CSS.

### Duplicate Or Missing Title Tags

Pages that share a title, or that use a default CMS title, confuse indexing and waste the most important on-page element. The trap is leaving titles to templates. The editor verifies each page has a unique, descriptive, keyword-relevant title.

### Meta Descriptions That Misrepresent The Page

A description written to maximize clicks but disconnected from content increases bounce and erodes trust. The trap is treating the description as bait. The editor ensures every description accurately previews the page and delivers on its promise.

### Generic Anchor Text

"Click here" and "read more" tell neither crawlers nor users what the link leads to. The trap is convenience. The editor rewrites anchor text to describe the destination, improving both SEO and accessibility.

### Schema Markup That Contradicts Visible Content

Marking up content the page does not actually show, such as fake reviews or ratings, is treated as spam and can trigger penalties. The trap is adding schema for rich results without honest grounding. The editor cross-checks markup against what a reader sees.

### Ignoring Orphan Pages And Broken Internal Links

Pages with no inbound internal links are hard for crawlers and readers to find; broken links waste authority and frustrate users. The trap is editing pages in isolation rather than as a network. The editor maps internal links across the site and fixes orphans and dead ends.

### Unstable Or Bloated URLs

Changing URLs without redirects, or using long parameter-heavy slugs, breaks link equity and confuses users. The trap is treating URLs as disposable. The editor defends stable, clean URLs and requires redirects for any change.

## Self-Check

- Does the page have exactly one H1, and does the full heading tree descend in order with no skipped levels or decorative misuse?
- Is the title tag unique across the site, keyword-relevant, accurate, and within display limits before truncation?
- Does the meta description accurately summarize the page, include the keyword where natural, and fit within roughly 160 characters?
- Are internal links placed in relevant context, pointing to live destinations, with descriptive rather than generic anchor text?
- Have orphan pages been identified and connected, and have broken links been removed or redirected?
- Does any structured data or schema markup accurately reflect visible page content, with required properties present and no misleading claims?
- Is the URL short, readable, keyword-relevant, hyphen-separated, and stable, with redirects in place for any prior version?
- Does the content architecture cluster related topics and keep important pages within a reasonable click depth of the home page?
- Where crawler and human needs conflict, has the editor found a solution serving both, or documented the decision to favor the reader?
- Have structural conventions been recorded so consistency holds across contributors and future edits?
