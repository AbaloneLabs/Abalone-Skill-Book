---
name: multilingual_and_cjk_search.md
description: Use when the agent is building search for non-English or mixed-language content, configuring analyzers for Korean, Japanese, Chinese, or other languages without word boundaries, choosing between morphological analyzers, n-gram, and ICU tokenizers, handling CJK (Chinese-Japanese-Korean) segmentation, designing synonym and stopword lists per language, normalizing Unicode (case folding, diacritics, width, compatibility), detecting document language, or routing queries to the right analyzer. Also covers the failure modes of applying a Latin analyzer to CJK text (no tokens produced), n-gram index bloat and false matches, the difference between Korean morphological analysis and Japanese kuromoji, mixed-script queries, and the recurring mistake of treating "search" as language-agnostic when it is deeply language-specific.
---

# Multilingual And CJK Search

Search is not language-agnostic. The process that turns text into searchable tokens — splitting on whitespace, lowercasing, stemming — is built around the assumptions of space-delimited, alphabetic scripts. Those assumptions break the moment the content is Chinese, Japanese, Korean, Arabic, Thai, or any language that does not separate words with spaces, that uses combining marks, or that has rich inflection. The judgment problem is that tokenization for these languages is not a setting you flip; it is a pipeline of segmentation, morphological analysis, normalization, and dictionary curation that determines whether a search finds anything at all. A standard analyzer applied to Chinese text produces one token per sentence (no segmentation), so almost no query matches. An n-gram fallback produces matches but bloats the index and matches noise. The "search works in English" default is silent failure in every other language.

Agents tend to miss these problems because the test data is English, the demo query is English, and the engine's default analyzer returns results. The harm appears only when a Korean user searches in Korean and gets nothing, or gets everything (n-gram false matches), or when a Japanese query with a conjugated verb fails to match the dictionary form in the document. The judgment problem is to treat each language's analysis as a first-class design decision: what segments the text, what normalizes it, what expands or contracts the query, and how mixed-language documents are handled. This is not internationalization polish to add later; it is the difference between search working and not working for a large fraction of users.

This skill covers language detection and routing, CJK segmentation strategies, morphological analysis, normalization, synonyms and stopwords, and mixed-script handling. It complements the search-index-design skill (which covers analyzers generically) and the relevance-ranking skill.

## Core Rules

### Do Not Apply A Latin Analyzer To CJK Or Spaceless Scripts

The standard analyzer tokenizes on whitespace and punctuation. For Chinese, Japanese, and Thai — languages that do not separate words with spaces — this produces one giant token per sentence or clause. The inverted index then stores a handful of enormous tokens that no reasonable query will match, and search returns nothing. This is the most common multilingual search failure, and it is invisible until a non-English user tries it:

- **CJK requires a segmenter or morphological analyzer.** Chinese needs a dictionary-based segmenter (IK, jieba, Smartcn); Japanese needs morphological analysis (Kuromoji, Sudachi) that segments and lemmatizes; Korean needs a morphological analyzer (Nori, the Korean analyzer) that segments agglutinated forms.
- **Thai, Khmer, Lao, and similar need a sentence/word segmenter** because they also lack inter-word spaces.
- **Verify with the analyze API.** Feed real CJK text through the configured analyzer and inspect the tokens. If a Chinese sentence produces one token, the analyzer is wrong; if it produces sensible word tokens, it is working.

The weak pattern is "we'll add language support later," which ships a search that silently returns nothing for a large user population.

### Choose Segmentation Strategy By Language, Trading Accuracy Against Index Size

For CJK there are two broad strategies, and the choice has real tradeoffs:

- **Dictionary-based morphological segmentation.** Segments text into dictionary words and (for Japanese/Korean) lemmatizes inflected forms to a dictionary form. High precision, good relevance, modest index size. Requires a quality dictionary and a language-specific analyzer. The right default for Chinese, Japanese, and Korean when one is available.
- **N-gram (character bigram/trigram).** Splits text into fixed-length character substrings with no dictionary. Works for any CJK language without a dictionary, but produces many more tokens (index bloat, often 3–5x), matches substrings that are not words (false positives — `분실` matching inside unrelated text), and degrades relevance. Use as a fallback when no morphological analyzer exists, or for substring matching, not as a first choice.
- **ICU Unicode segmentation.** A middle ground that segments by Unicode word boundaries; better than whitespace for some scripts but weaker than a language-specific morphological analyzer for CJK.

Prefer the language-specific morphological analyzer; fall back to n-gram only with eyes open about index size and precision. Never default to whitespace tokenization for spaceless scripts.

### Detect Language And Route To The Right Analyzer

When documents or queries can be in multiple languages, a single analyzer cannot serve all of them. You need language detection and routing:

- **Per-field language mapping.** Index each language's content into a field mapped with that language's analyzer (`title_en`, `title_ko`, `title_ja`), and query the field matching the detected query language (or query all with a multi_match and let scoring pick the best).
- **Per-document language detection.** Detect the dominant language of each document at index time and route it to the appropriate analyzer. Detection is probabilistic and wrong on short or mixed text — set a confidence threshold and have a fallback analyzer.
- **Query-time language detection.** Detect the query's language and target the matching field/analyzer. Short queries are hard to detect reliably; consider querying multiple language fields when detection is uncertain.

The weak pattern is one analyzer for all content, which means whichever language the analyzer was built for works and every other language fails or degrades. The strong pattern is explicit routing with a documented fallback.

### Normalize Unicode Deliberately — Case, Diacritics, Width, Compatibility

Normalization determines whether two visually identical strings match. Without it, `café` and `café` (different Unicode forms), or `ＡＢＣ` (fullwidth) and `ABC`, or `ﬁ` (ligature) and `fi` fail to match:

- **Case folding.** Lowercase (or Unicode case fold) at both index and query time so `Apple` and `apple` match. Standard for Latin; less relevant for CJK (no case) but harmless.
- **Diacritic folding (asciifolding / accent folding).** Strip or normalize accents so `résumé` matches `resume`. This is a deliberate tradeoff: folding improves recall but loses meaning in languages where accents distinguish words (e.g., Spanish `año` vs `ano`). Decide per language whether folding is appropriate.
- **Width and compatibility normalization (ICU normalization).** Normalize fullwidth/halfwidth Latin and CJK, compatibility decompositions (ligatures, circled characters) so `１２３` matches `123`. Important for Japanese content, which routinely mixes fullwidth and halfwidth.
- **Apply normalization identically at index and query time**, or tokens will not match.

State, for each language: which normalizations apply, and which are turned off because they destroy meaning.

### Curate Synonyms And Stopwords Per Language, Not Globally

Synonyms and stopwords are language-specific. A global English stopword list applied to Korean does nothing useful; an English synonym map applied to Japanese maps the wrong tokens:

- **Synonyms per language.** Build a synonym set for each language from domain terms and common reformulations. Apply at query time (via `search_analyzer`) so the set can evolve without reindexing. Beware bidirectional vs one-way synonyms: bidirectional expands both directions (risky for common words); one-way is safer for mapping queries to canonical terms.
- **Stopwords per language.** Each language has its own high-frequency function words. A language-specific stopword list improves both index size and relevance by removing tokens that match everything. But be careful: aggressive stopword removal can break phrase queries (`to be or not to be` becomes empty).
- **CJK stopwords differ.** Korean/Japanese/Chinese function words and particles (은/는/이/가, は/の/で) are stopwords in their languages; reuse the analyzer's defaults and extend with domain-specific noise terms.

Maintain synonym and stopword lists per language, version them, and apply at query time.

### Handle Mixed-Script And Transliterated Queries Explicitly

Real queries mix scripts: a Korean user typing an English brand name, a Japanese query mixing Kanji, Katakana, and Latin, a transliterated query (Romaji, Pinyin). Decide how each is handled:

- **Mixed-script documents.** A single field may contain multiple scripts; the analyzer must handle each. Some analyzers (ICU, or a multi-analyzer chain) tokenize per script; a single-language analyzer may drop or mis-segment the non-native script.
- **Transliteration and romanization.** Decide whether Pinyin (Chinese), Romaji (Japanese), or Revised Romanization (Korean) queries should match native-script documents. This requires a transliteration filter at query time (or indexing both forms), and it expands recall at the cost of some precision.
- **Kana normalization (Japanese).** Hiragana and Katakana represent the same sounds; normalizing between them (via a kana-stemming or reading-form filter) lets a Hiragana query match Katakana text and vice versa.

Document the mixed-script behavior explicitly; do not assume the analyzer handles it.

## Common Traps

### Standard Analyzer On Chinese/Japanese/Korean Producing No Tokens

Applying the whitespace-based standard analyzer to CJK text, producing one token per sentence, so almost no query matches. Use a language-specific morphological analyzer or segmenter, verified with the analyze API.

### N-Gram As A Thoughtless Default For All CJK

Defaulting to character n-grams for CJK because "it works without a dictionary," producing a 3–5x larger index and many false substring matches. Prefer a morphological analyzer; use n-gram only as a fallback or for substring matching.

### One Analyzer For All Languages

Mapping all content fields with a single analyzer, so the language it was built for works and every other language silently fails or degrades. Detect language and route to per-language fields and analyzers.

### Diacritic Folding That Destroys Meaning

Applying aggressive accent folding to a language where accents distinguish words (Spanish `año`/`ano`), improving recall but creating embarrassing false matches. Decide diacritic handling per language.

### Mismatched Index-Time And Query-Time Normalization

Normalizing at index time but not query time (or with different rules), so `ＡＢＣ` and `ABC` never match. Apply identical normalization at both times.

### Global Synonym Or Stopword List Applied Across Languages

Using an English synonym map or stopword list for all content, so it does nothing for non-English text or maps the wrong tokens. Curate per-language lists and apply at query time.

### Ignoring Width And Compatibility Normalization For Japanese

Failing to normalize fullwidth/halfwidth and compatibility forms common in Japanese content, so visually identical strings do not match. Apply ICU normalization for scripts that mix widths.

### Trusting Language Detection On Short Queries

Routing based on language detection of a two-word query, where detection is unreliable, so the query hits the wrong analyzer and returns nothing. Set a confidence threshold and fall back to querying multiple language fields.

### Treating Transliteration As Automatic

Assuming a Pinyin or Romaji query will match native-script documents without a transliteration filter, so romanized queries return nothing. Implement transliteration explicitly or index both forms.

## Self-Check

- [ ] No spaceless script (Chinese, Japanese, Korean, Thai, etc.) is analyzed with a whitespace-based standard analyzer; each uses a language-specific morphological analyzer or segmenter, verified with the analyze API on real text.
- [ ] The segmentation strategy was chosen per language with its tradeoffs understood: morphological analysis as the default for CJK, n-gram only as a deliberate fallback with acknowledged index-size and false-match costs.
- [ ] Multi-language content uses language detection and routing to per-language fields/analyzers, with a confidence threshold and a documented fallback analyzer for uncertain or short text.
- [ ] Unicode normalization (case, diacritics, width, compatibility) is applied identically at index and query time, with diacritic folding decided per language where accents carry meaning.
- [ ] Synonym and stopword lists are curated per language, versioned, and applied at query time (not index time) so they can evolve without reindexing.
- [ ] Mixed-script and transliterated queries (Pinyin, Romaji, Korean romanization, Kana normalization) have explicit, documented handling rather than assumed behavior.
- [ ] Analyzers were tested on real non-English content and queries — not only English sample data — across head and tail queries in each supported language.
- [ ] The highest-risk cases were verified — standard analyzer on CJK, n-gram false matches, single-analyzer-for-all, diacritic folding destroying meaning, mismatched normalization, and unreliable detection on short queries — not only the English happy path.
