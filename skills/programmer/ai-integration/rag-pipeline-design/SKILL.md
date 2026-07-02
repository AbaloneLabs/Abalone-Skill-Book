---
name: rag_pipeline_design.md
description: Use when the agent is designing or debugging a retrieval-augmented generation (RAG) system — choosing a chunking strategy for documents, selecting an embedding model, building the retrieval step (vector search, hybrid search), evaluating retrieval quality, adding a reranker, mitigating hallucination, or wiring source/citation tracking into the generated answer. Also covers the failure modes of naive fixed-size chunking that splits semantic units, embeddings that do not match the query distribution, retrieval that returns top-k by cosine similarity regardless of relevance, hallucinated answers that look authoritative, untraceable claims with no citations, and the recurring mistake of treating RAG as "embed everything and ask the model" when each stage has its own quality bar.
---

# RAG Pipeline Design

RAG couples two systems that each have their own failure modes: a retriever that finds passages, and a generator that answers from them. The judgment problem is that the overall quality is bounded by the weakest stage, and the stages interact in ways that are easy to miss. Bad chunking fragments a coherent answer across pieces the retriever cannot rank together. An embedding model chosen for benchmark scores retrieves poorly on the actual query distribution. A top-k cosine retrieval returns the most similar passages regardless of whether they answer the question. The generator, given marginally-relevant context, hallucinates a confident-sounding wrong answer with real citations attached. RAG is not "embed everything and ask the model"; it is a pipeline where chunking, embedding, retrieval, reranking, generation, and citation each have a quality bar, and shipping any stage on vibes produces a system that confidently misleads users.

Agents tend to miss these problems because the demo works: embed ten documents, ask a question about one of them, and the answer is correct and cited. The harm appears on real corpora and real queries. A 100,000-document corpus dilutes retrieval so the right passage ranks below similar-but-wrong ones. A chunk boundary splits a definition from its term, so neither chunk retrieves well. The model, asked to answer "from the context," fills gaps from its parametric memory and presents them as if sourced. A citation points to a chunk that does not actually support the claim, because the link between generated text and source was never enforced. The judgment problem is to treat RAG as a measurable pipeline with per-stage evaluation, conservative generation, and enforced provenance — not a single magical embed-and-ask call.

This skill covers chunking, embedding selection, retrieval quality, reranking, hallucination mitigation, and citation tracking. It complements the search-index-design and relevance-ranking skills (retrieval mechanics), the prompt-engineering skill (the generation prompt), and the ai-guardrails skill (output validation).

## Core Rules

### Chunk To Preserve Semantic Units, Not To Fit A Token Budget

Chunking determines what the retriever can find and what the generator sees as context. Naive fixed-size chunking (every 500 tokens) is the default and a frequent source of poor retrieval, because it splits semantic units — a definition separated from its term, a step separated from its procedure, a table separated from its caption:

- **Chunk by structure first, size second.** Prefer natural boundaries: headings, paragraphs, list items, table-plus-caption, code-plus-comment. Structural chunking keeps related content together.
- **Overlap adjacent chunks.** An overlap (50–100 tokens) prevents a relevant span from being split exactly at a boundary, so at least one chunk contains it whole.
- **Match chunk size to the content and the model.** Too-small chunks lose context the generator needs to interpret them; too-large chunks dilute relevance signal and waste the context window. Tune chunk size against retrieval quality on real documents, not a default.
- **Preserve metadata per chunk.** Each chunk should carry its source document, section path, and position, so retrieval can filter (by source, date, permissions) and citations can point to a precise location.

The weak pattern is fixed-size chunking with no overlap and no metadata, which fragments semantics and makes citations imprecise.

### Choose The Embedding Model Against Your Query Distribution, Not A Leaderboard

Embedding models differ enormously, and benchmark scores (MTEB) do not guarantee performance on your data. A model that tops general benchmarks can underperform on domain-specific, multilingual, or short-query-vs-long-document workloads:

- **Evaluate on your own query-document pairs.** Build a small set of real queries with known-relevant documents and measure recall@k for candidate embedding models. The model that wins on your data is the right one, regardless of leaderboard rank.
- **Match the embedding to the retrieval pattern.** Symmetric (query and document similar length) vs asymmetric (short query, long document) workloads favor different models; choose one designed for your pattern.
- **Consider multilingual needs.** If content or queries span languages, use a multilingual embedding model; a monolingual model will fail on the other languages silently.
- **Pin the model version.** Embeddings are only comparable within a model; changing the model requires re-embedding the entire corpus. Version and plan re-embedding as a first-class operation.

### Measure Retrieval Quality Before Trusting The Generator

The generator cannot answer correctly from passages the retriever never surfaced. Retrieval quality is the foundation, and it must be measured:

- **Build a retrieval evaluation set.** Real queries paired with the documents that should be retrieved for each. Measure recall@k (did the relevant doc appear in the top k?) and, where graded judgments exist, nDCG.
- **Tune k against the context budget and precision.** Larger k improves recall but dilutes the context with irrelevant passages (which can trigger hallucination) and costs tokens. Find the k where recall plateaus.
- **Consider hybrid retrieval.** Pure vector (dense) retrieval captures semantic similarity but can miss exact keyword/term matches; combining dense with sparse (BM25) retrieval — hybrid search — often beats either alone, especially for queries with specific terms, codes, or names.
- **Add a reranker for precision.** Retrieval (especially hybrid) may return many candidates; a cross-encoder reranker, scored on query+passage pairs, reorders the top candidates by true relevance far better than cosine similarity. Reranking is expensive per pair, so apply it only to the top-N retrieved candidates, not the whole corpus.

A RAG system with unmeasured retrieval is a system whose foundation is assumed, not known.

### Mitigate Hallucination By Constraining The Generator To The Context

Hallucination — confident claims not supported by the retrieved context — is the central RAG risk. It cannot be eliminated, but it is reduced by how the generation stage is designed:

- **Instruct the model to answer only from the provided context and to say when it cannot.** A clear "if the context does not contain the answer, say you don't know" instruction, reinforced by the prompt structure, reduces confabulation. Measure the refusal rate; a model that never refuses is probably hallucinating on some inputs.
- **Lower temperature for factual answering.** Factual RAG benefits from low temperature; high temperature encourages the model to generate plausible-sounding content beyond the context.
- **Prefer grounded generation patterns where available.** Some models/APIs support citation-grounded or constrained generation that ties output spans to source spans; use these when the stakes warrant.
- **Detect ungrounded claims post-hoc.** Where feasible, verify generated claims against the retrieved context (a second model or a citation-check pass) and flag or strip claims not supported by any source. This is expensive but valuable for high-stakes domains.
- **Do not over-retrieve irrelevant context.** Stuffing the context with marginally-relevant passages encourages the model to use them, increasing hallucination. Precision of context matters as much as recall.

### Enforce Citation And Provenance — Do Not Hope For It

A RAG answer without traceable provenance is an unverifiable claim. Citations must be enforced by design, not requested and hoped for:

- **Require every factual claim to cite a source chunk.** Structure the output so claims carry citations (inline references, footnotes) pointing to specific retrieved chunks.
- **Validate that cited chunks support the claims.** A citation to a chunk that does not actually contain the supporting evidence is worse than no citation (it lends false authority). Where stakes warrant, verify claim-citation alignment.
- **Carry source metadata end-to-end.** From chunk → retrieval → generation → displayed answer, the source document, section, and exact location must be preserved so a user can click through to the origin.
- **Handle the "no relevant context" case explicitly.** When retrieval returns nothing relevant, the system should say so and cite nothing, rather than answering from parametric memory with fabricated citations.

### Plan Index Freshness And Re-Embedding

A RAG corpus is not static; documents are added, updated, and deleted. The pipeline must keep the index current:

- **Define the freshness window.** How stale can the corpus be for the use case? A product support RAG may tolerate hours; a news RAG needs near-real-time. Match the indexing cadence to the requirement.
- **Incremental vs full re-embedding.** Re-embedding the whole corpus on every change is expensive; prefer incremental indexing (embed and add new/changed chunks, remove deleted). But plan periodic full rebuilds to catch drift and apply embedding-model upgrades.
- **Version documents and chunks.** When a source document changes, its chunks change; re-embed and replace, do not accumulate stale chunks alongside new ones.

## Common Traps

### Fixed-Size Chunking That Splits Semantic Units

Chunking every N tokens with no regard for structure, splitting definitions from terms or steps from procedures, so neither half retrieves well. Chunk by structure with overlap and metadata.

### Choosing An Embedding Model By Leaderboard Rank

Picking the top-MTEB model without evaluating on your query distribution, then finding it underperforms on domain-specific or asymmetric workloads. Evaluate candidates on your own query-document pairs.

### Top-k Cosine Retrieval With No Quality Measurement

Returning the k most cosine-similar chunks and assuming relevance, with no recall@k measurement, so the right passage ranks below similar-but-wrong ones unnoticed. Measure retrieval quality and consider hybrid search plus reranking.

### Pure Dense Retrieval Missing Exact Matches

Using only vector search, which misses exact term/code/name matches that BM25 handles well. Combine dense and sparse retrieval (hybrid) for queries with specific terms.

### Over-Stuffing Context And Increasing Hallucination

Padding the context with many marginally-relevant passages to maximize recall, which encourages the model to use irrelevant content and hallucinate. Tune k for the precision-recall balance.

### Confident Hallucination With Real Citations

The model generating a claim not in the context but attaching a real citation, lending false authority. Instruct grounding, lower temperature, and verify claim-citation alignment for high-stakes domains.

### Citations That Do Not Support The Claim

A citation pointing to a chunk that does not actually contain the evidence, because claim-citation alignment was never validated. Validate citations against source chunks.

### Answering From Parametric Memory When Context Is Empty

When retrieval returns nothing relevant, the model answering from its training memory and fabricating citations, instead of refusing. Handle the empty-context case explicitly.

### Unversioned Corpus And Stale Chunks

Accumulating outdated chunks when source documents change, because re-embedding and replacement is not defined. Version documents and chunks; re-embed on change.

## Self-Check

- [ ] Chunking preserves semantic units (structural boundaries, overlap, metadata per chunk) rather than naive fixed-size splits, and chunk size was tuned against retrieval quality on real documents.
- [ ] The embedding model was chosen by evaluating recall@k on your own query-document pairs (and multilingual capability where needed), not by leaderboard rank, and is versioned with a re-embedding plan for upgrades.
- [ ] Retrieval quality is measured (recall@k, nDCG on a real evaluation set), k is tuned for the precision-recall-context balance, and hybrid (dense+sparse) retrieval with a reranker is used where it improves precision.
- [ ] The generator is constrained to the context: explicit "answer only from context, say if you cannot" instruction, low temperature for factual tasks, and grounded/constrained generation where available.
- [ ] Hallucination is actively mitigated: refusal rate is measured (a model that never refuses is suspect), context precision is maintained (not over-stuffed), and ungrounded claims are detected/flagged for high-stakes domains.
- [ ] Citations are enforced by design: every factual claim cites a source chunk, claim-citation alignment is validated where stakes warrant, and source metadata (document, section, location) is preserved end-to-end for click-through.
- [ ] The empty/no-relevant-context case is handled explicitly (refuse and cite nothing) rather than answered from parametric memory with fabricated citations.
- [ ] Index freshness matches the use case: incremental indexing for changes, a defined freshness window, periodic full rebuilds, and versioned documents/chunks so stale chunks do not accumulate.
- [ ] The highest-risk cases were verified — split semantic units, leaderboard-chosen embeddings, unmeasured retrieval, over-stuffed context, hallucination with real citations, unsupported citations, and parametric-memory answers on empty retrieval — not only the ten-document demo that worked.
