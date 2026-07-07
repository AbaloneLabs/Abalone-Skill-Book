---
name: vector_and_semantic_search.md
description: Use when the agent is designing or building vector search, semantic search, embedding-based retrieval, or hybrid (vector + lexical) search; choosing an embedding model, generating and indexing embeddings, selecting an approximate nearest neighbor (ANN) index and its parameters (HNSW, IVF, product quantization), deciding on hybrid retrieval and fusion, handling embedding lifecycle and re-indexing when models change, or evaluating semantic retrieval quality. Also covers the failure modes of embeddings that encode the wrong similarity (model mismatch), ANN recall/precision tradeoffs that silently drop relevant results, stale embeddings after a model change, hybrid fusion that mishandles score scales, and the recurring mistake of treating vector search as a drop-in replacement for lexical search when its recall, cost, and interpretability characteristics differ fundamentally.
---

# Vector And Semantic Search

Vector search retrieves items by semantic similarity — items whose meaning is close to the query, even if they share no words — by embedding queries and items into a vector space and finding nearest neighbors. This solves a real problem lexical search cannot (synonymy, paraphrase, cross-lingual, conceptual match), and it has become a core retrieval method for text, images, and multimodal content. The judgment problem is that vector search is easy to demo and hard to get right in production, because its behavior differs from lexical search in ways that matter. The similarity an embedding encodes depends entirely on the model and its training, and a model that encodes the wrong notion of similarity (topical vs semantic vs stylistic) returns plausible-but-wrong results. Approximate nearest neighbor (ANN) indexes trade recall for speed, and mis-set parameters silently drop the relevant results. Embeddings go stale when the model changes, and a re-index is a large, planned operation. Hybrid retrieval (combining vector and lexical) requires fusing scores from different scales, and naive fusion mishandles it. The discipline is to choose the embedding model for the similarity you need, tune the ANN for the recall you can accept, manage the embedding lifecycle, fuse hybrid signals correctly, and evaluate semantic retrieval on its own terms rather than assuming it is a better lexical search.

Agents tend to under-invest here because an off-the-shelf embedding model and a vector database produce relevant-looking results immediately, and the demo query "find similar items" works. The harm appears on real queries and at scale. The embedding model encodes topical similarity when the use case needs exact-match-ish similarity, and precise matches are missed. The ANN's recall is set too aggressively for speed, and the truly relevant result is not in the candidate set the ranker sees. The embedding model is upgraded and the old embeddings are now in a different space, producing nonsense results until a full re-index completes. A hybrid system fuses vector and lexical scores by simple weighted sum, but the scores are on different scales, so one dominates. The system is evaluated with lexical-search assumptions (exact-match precision) and appears to underperform, when the relevant metric is semantic recall. The judgment problem is to match the embedding to the needed similarity, tune and monitor ANN recall, manage the embedding lifecycle and re-indexing, fuse hybrid signals correctly, and evaluate semantic retrieval appropriately.

This skill covers embedding model selection, ANN indexing and recall, hybrid retrieval and fusion, the embedding lifecycle, and evaluation. It complements the relevance-ranking skill (ranking candidates), the search-index-design skill (the index), and the rag-pipeline-design skill (retrieval for generation). Here the focus is embedding-based semantic retrieval and its specific characteristics.

## Core Rules

### Choose The Embedding Model For The Similarity You Need

An embedding encodes whatever notion of similarity the model was trained to encode, and "similarity" is not one thing. The model must match the use case's needed similarity:

- **Match the similarity type to the task.** Topical similarity (same subject), semantic equivalence (same meaning, different words), stylistic similarity (same tone), or visual similarity (same appearance) are different, and a model trained for one encodes a different space. A model encoding topical similarity will not find exact paraphrases; one encoding semantic equivalence may not group by topic. Know which similarity your task needs and choose a model that encodes it.
- **Match the domain.** A general-purpose text embedding may underperform on specialized domains (medical, legal, code, product catalogs); a domain-specific or fine-tuned model often encodes the domain's relevant similarity better. Evaluate on domain data, not general benchmarks.
- **Match the modality.** Text, image, audio, and multimodal embeddings each have their own models and spaces; cross-modal retrieval (image query to text items) requires a model trained to align the modalities (e.g., CLIP-style), not separate unrelated embeddings.
- **Account for language and multilingual needs.** A monolingual model will not retrieve across languages; a multilingual model aligns cross-lingual semantics but may underperform within a single language. Match the model's language coverage to the query and corpus languages (see the multilingual-search skill).

### Tune The ANN Index For The Recall You Can Accept

Exact nearest-neighbor search is too slow at scale, so vector search uses approximate nearest neighbor (ANN) indexes (HNSW, IVF, product quantization) that trade recall for speed. The tradeoff must be tuned and monitored:

- **Understand the recall/speed/cost tradeoff.** ANN parameters (HNSW's efSearch, IVF's nprobe, quantization) control how many candidates are examined: more examination yields higher recall (fewer true neighbors missed) at higher latency and cost. The default is rarely optimal for a specific dataset and SLO.
- **Tune for the recall your application can accept.** If the ranker only sees ANN's candidate set, a low-recall ANN silently drops the relevant item before ranking. Measure recall (what fraction of true nearest neighbors the ANN returns) at your latency budget, and tune parameters to meet a recall floor.
- **Account for memory and storage.** Vectors are large (hundreds to thousands of dimensions per item); product quantization reduces memory but costs recall. Balance memory cost against the recall loss quantization introduces, and size the index for the corpus.
- **Re-tune as the corpus grows and changes.** ANN performance depends on the data distribution; parameters tuned on a small or different corpus may degrade as the corpus grows or shifts. Periodically re-evaluate recall.

### Fuse Hybrid Retrieval (Vector + Lexical) Correctly

Many systems combine vector and lexical retrieval for the strengths of each (semantic match + exact match), and fusion is where hybrid systems often go wrong:

- **Recognize that vector and lexical scores are on different scales.** A vector cosine score and a lexical BM25 score are not comparable; a naive weighted sum lets the higher-variance score dominate. Use a fusion method that handles the scales (reciprocal rank fusion, or score normalization) rather than raw weighted addition.
- **Fusion method matters.** Reciprocal rank fusion (RRF) combines ranks (robust to scale, simple, effective); learned fusion (a model combining normalized scores) can outperform when enough data exists. Choose the method for the data and the tuning effort justified.
- **Decide what each retriever contributes.** Lexical retrieval excels at exact matches, rare terms, names, and codes; vector retrieval excels at semantic and conceptual match. Design the hybrid so each does what it is good at, rather than running both and hoping.
- **Evaluate hybrid as a whole, not each retriever.** The point of hybrid is that the combination beats either alone; measure end-to-end quality, and confirm the hybrid actually improves over the better single retriever (a poorly fused hybrid can be worse than lexical alone).

### Manage The Embedding Lifecycle And Re-Indexing

Embeddings are derived artifacts tied to a specific model, and they have a lifecycle that must be managed:

- **Embeddings are tied to the model that produced them.** Changing the embedding model (or fine-tuning it) changes the vector space; old and new embeddings are not comparable. A model change requires re-embedding the entire corpus (a large, planned operation), not a gradual mix.
- **Plan re-indexing as a first-class operation.** Re-embedding millions of items is expensive and time-consuming; design the re-index to be parallelizable, resumable, and runnable without downtime, and to produce a clean cutover (old index and new index coexist until the switch).
- **Handle query and corpus embedding consistency.** The query embedder and the corpus embedder must be the same model; a mismatch produces nonsense results. Version the embedding model and ensure both paths use the same version.
- **Re-embed when content changes.** An item whose content changed but whose embedding was not updated is retrieved by its old meaning; keep embeddings in sync with content, or schedule re-embedding for changing content.

### Evaluate Semantic Retrieval On Its Own Terms

Semantic retrieval is not "better lexical search," and evaluating it with lexical assumptions misleads. Evaluation must reflect semantic retrieval's purpose:

- **Measure recall and relevance for the similarity you intended.** Use judged relevance sets that reflect semantic relevance (paraphrases, conceptual matches), not only exact-match relevance. A vector system evaluated only on exact-match precision will appear to underperform, even when it is succeeding at semantic recall.
- **Measure ANN recall separately from ranking quality.** If the ranker only sees the ANN's candidates, ANN recall is a ceiling on quality; measure both (did the relevant item enter the candidate set? did the ranker rank it well?) to localize failures.
- **Evaluate at the corpus's scale and distribution.** Recall and latency on a small sample differ from production; evaluate on a representative corpus, including the tail (rare items, long queries) where semantic retrieval often matters most.
- **Use online evaluation alongside offline.** Offline judged sets catch regressions; online metrics (engagement, reformulation, success rate) confirm real-world value. Semantic relevance is hard to judge offline exhaustively; online signals are essential.

## Common Traps

### An Embedding Model That Encodes The Wrong Similarity

Using a general-purpose or topically-trained embedding when the task needs a different similarity (semantic equivalence, stylistic, domain-specific), returning plausible-but-wrong results. Match the model's encoded similarity to the task.

### ANN Recall Too Aggressive, Dropping Relevant Results

ANN parameters tuned for speed with recall too low, so the relevant item never enters the candidate set the ranker sees. Measure recall at the latency budget and tune to a recall floor.

### Stale Embeddings After A Model Change

Upgrading the embedding model without re-embedding the corpus, so old and new embeddings are in incomparable spaces and results are nonsense. Treat a model change as a full re-index operation.

### Hybrid Fusion Mishandling Score Scales

Combining vector and lexical scores by naive weighted sum when they are on different scales, letting one dominate. Use rank-based fusion (RRF) or score normalization.

### Treating Vector Search As A Drop-In For Lexical Search

Assuming vector search is simply better lexical search, when its recall, cost, exact-match weakness, and interpretability differ fundamentally. Use vector for semantic strength, keep lexical for exact match, and evaluate each appropriately.

### Embedding Model Mismatch Between Query And Corpus

The query embedder and corpus embedder using different models or versions, producing incomparable vectors. Version the embedding model and ensure both paths use the same version.

### Evaluating Semantic Retrieval With Lexical Assumptions

Measuring only exact-match precision and concluding vector search underperforms, when the relevant metric is semantic recall. Evaluate on judged semantic relevance and use online engagement signals.

## Self-Check

- [ ] The embedding model encodes the similarity the task needs (topical, semantic-equivalence, stylistic, visual), is matched to the domain (general vs specialized/fine-tuned) and modality, and covers the query and corpus languages.
- [ ] The ANN index is tuned for a measured recall floor at the latency budget (parameters like efSearch/nprobe/quantization set with recall measured, not defaulted), memory/storage is sized for the corpus, and recall is re-evaluated as the corpus grows or shifts.
- [ ] Hybrid retrieval fuses vector and lexical signals correctly (rank-based fusion like RRF, or learned fusion with normalized scores, not naive weighted sum of incomparable scores), each retriever contributes what it is good at, and the hybrid is confirmed to beat the better single retriever end-to-end.
- [ ] The embedding lifecycle is managed: embeddings are tied to a versioned model, query and corpus embedders use the same model/version, a model change triggers a planned full re-index (parallelizable, resumable, no-downtime cutover), and content changes trigger re-embedding.
- [ ] Evaluation reflects semantic retrieval's purpose: judged semantic relevance sets (not only exact-match), ANN recall measured separately from ranking quality, evaluation at corpus scale and distribution including the tail, and online engagement/reformulation signals alongside offline.
- [ ] The highest-risk cases were verified — a model encoding the wrong similarity, an ANN dropping the relevant item from the candidate set, a model change with stale embeddings, a hybrid fusion dominated by one score scale, and an evaluation that unfairly penalized semantic recall — not only the clean "find similar" demo.
