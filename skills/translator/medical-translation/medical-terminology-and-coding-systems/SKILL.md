---
name: medical_terminology_and_coding_systems.md
description: Use when the agent is translating medical content that relies on standardized coding systems and terminologies such as ICD SNOMED CT MedDRA LOINC RxNorm and ATC, preserving coded concept identity across languages, handling the relationship between verbatim terms and their coded categories, or ensuring translated medical terminology remains traceable to the correct code without semantic drift.
---

# Medical Terminology And Coding Systems

Modern medical translation does not deal only in words; it deals in coded concepts. Healthcare, clinical research, pharmacovigilance, and health information exchange rely on standardized terminologies and coding systems, ICD for diagnoses, SNOMED CT for clinical concepts, MedDRA for adverse events, LOINC for lab tests, RxNorm for medications, and ATC for drug classification, among others. These systems assign a code to a concept, and the code is meant to be language-independent: the same concept has the same code whether it is expressed in English, Japanese, or Spanish. This creates a translation problem that does not exist in general domains. When a translator renders a medical term, they are not just choosing a target-language word; they are responsible for ensuring that the target word denotes the same coded concept as the source, so that the code, the meaning, and the clinical reality stay aligned. The failure modes are specific and dangerous. A translator may render a term fluently but map it to a neighboring concept with a different code, so the translation reads correctly while the coded data point now refers to a different diagnosis, a different procedure, or a different adverse event. A translator may break the relationship between a verbatim reported term and its coded category, so that pharmacovigilance analysis can no longer group the case correctly. A translator may use a target-language term that belongs to an older version of a coding system, or to a different hierarchy branch, introducing semantic drift that propagates through databases and analyses. The discipline this skill covers is translating within coded medical terminologies while preserving concept identity, code traceability, and the structural relationships the coding systems impose. Agents often treat medical codes as background metadata, when they are in fact the load-bearing structure that determines whether translated medical data remains valid.

Use this skill when translating medical content that uses standardized coding systems, preserving coded concept identity, handling verbatim-versus-coded term relationships, or ensuring translated medical terminology stays traceable to the correct code. The goal is to produce target terminology that denotes exactly the same coded concept as the source, with no semantic drift across the code boundary.

## Core Rules

### Translate To The Concept Behind The Code, Not Just The Word

Every coded medical term denotes a specific concept with defined boundaries, and the code exists precisely because words alone are ambiguous. Translation must target the concept the code represents, not a word that sounds equivalent.

Before rendering a coded term, identify the code and read its concept definition in the terminology system. The definition tells you exactly what the concept includes and excludes, which often differs from the everyday meaning of the word. A term like "myocardial infarction" denotes a specific concept in ICD and SNOMED CT with defined clinical criteria, and the target-language rendering must denote that same concept, not a broader or narrower everyday notion of heart attack. A term like "nausea" in MedDRA is a specific preferred term within a hierarchy, and the target-language equivalent must map to that preferred term, not to a related but distinct term like "vomiting." Translating by surface word without consulting the concept definition is the primary source of semantic drift across the code boundary, because fluency does not guarantee conceptual identity. Always anchor the rendering to the code's defined concept.

### Preserve The Relationship Between Verbatim Terms And Coded Categories

In clinical research, pharmacovigilance, and electronic health records, medical content often carries both a verbatim term, the investigator's or patient's original words, and a coded term, the standardized category it was mapped to. This relationship is analytically essential and must be preserved exactly in translation.

The verbatim term is the raw description, such as a patient's reported "felt sick to stomach," and the coded term is the standardized category, such as the MedDRA preferred term "nausea." The relationship between them is how analysts verify coding decisions, detect miscoding, and aggregate cases. If the translator renders the verbatim term in a way that no longer supports the coded category, or alters it to match the code, the relationship is broken. Preserve the verbatim term as a faithful rendering of the original words, even if it is less precise than the code, because the verbatim is evidence of what was reported. Preserve the coded term as the exact standardized designation for that code, using the official target-language version of the terminology. Do not normalize the verbatim toward the code or vice versa, because the gap between them is meaningful. Flag cases where the verbatim and the code seem mismatched, because that may reflect a source coding error that needs resolution, not translation.

### Use The Official Target-Language Version Of Each Coding System

Major medical coding systems publish official translations and localized versions, and the translator must use these rather than rendering terms freely. Using unofficial renderings breaks code traceability.

ICD has official translations and clinical modifications for many languages and regions, such as ICD-10-CM for the US and ICD-10-GM for Germany, and these are not interchangeable. SNOMED CT has official language editions and extensions. MedDRA is officially translated and maintained by the MSSO across many languages, with standardized preferred terms and lowest-level terms for each. LOINC, RxNorm, and ATC have their own official multilingual support to varying degrees. For any coded term, use the official target-language designation from the relevant system and version, and record the code alongside it. Do not substitute a different target-language term that seems better, because the official term is what the code maps to and what downstream systems expect. Where a system lacks an official translation for the target language, follow the system's documented guidance for interim handling, which may involve retaining the source term with the code or using a validated translation, and flag the gap. Unofficial renderings make the translated data non-conformant and untraceable.

### Maintain Version And Hierarchy Consistency

Medical coding systems are versioned and hierarchical, and translation must respect both. A code means a specific concept in a specific version, and it sits in a specific place in a hierarchy that drives aggregation and analysis.

Record the version of each coding system used, because the same code or term may change meaning across versions, and a translation tied to the wrong version refers to a different concept. Within a project, use one version consistently, because mixing versions produces data that cannot be aggregated. Respect the hierarchy: a MedDRA preferred term belongs to a specific high-level term and system organ class, and translating the preferred term must not imply a different location in the hierarchy, because safety analysis groups terms by hierarchy. A SNOMED CT concept has a position defined by its "is-a" relationships, and the target-language rendering must denote a concept with the same relationships, not a sibling or a parent. Version and hierarchy errors are invisible at the word level but corrupt the coded data, and they are among the hardest defects to detect in review because the translation reads fluently while the code refers to the wrong place in the structure.

### Handle Coding Gaps And Unmapped Concepts Deliberately

Not every medical concept in a source text has a corresponding code in the target system, and not every target-language term has an official coding equivalent. Recognize these gaps and handle them deliberately rather than forcing a false mapping.

When a source term maps to a code but no official target-language designation exists for that code, options include retaining the official source-language designation with the code, using a validated interim translation with a documented gap flag, or describing the concept while preserving the code. When a source term does not map to any code, do not invent a mapping, because a fabricated code relationship is data corruption. Record the unmapped concept and escalate it, because coding systems evolve and the gap may be fillable through the system's update process. The principle is that the absence of a mapping is information, and papering over it with a plausible-sounding term destroys traceability. A translated dataset full of forced mappings is less accurate, not more, than one that honestly flags its gaps.

### Reconcile Terminology Across Documents And Systems

Medical content often spans multiple documents and systems that must use terminology consistently, and translation must reconcile across them rather than rendering each in isolation.

A clinical trial protocol, its case report forms, its statistical analysis plan, and its study report all refer to the same adverse events, the same endpoints, and the same procedures, and their terminology must align. A patient record, a claim, and a registry that refer to the same diagnosis must use the same code. Build a project termbase anchored to codes before drafting, and enforce it across all documents. When a term appears in multiple documents, verify that the rendering and the code are identical everywhere, because inconsistency between documents is a defect even when each rendering is individually correct. Cross-document reconciliation is especially critical in regulatory submissions, where reviewers compare documents and where terminology inconsistency signals poor quality control.

### Verify Numeric And Coded Content Through Separate Checks

Medical codes, doses, lab values, and identifiers are high-stakes numeric and alphanumeric content that must be verified through a dedicated check, not only while reading. A transcription error in a code is a different concept.

Implement a separate verification pass for all codes, values, units, and identifiers, comparing target against source element by element. A code transposed by one digit refers to a different concept; a lab value with a misplaced decimal is a different result; a unit error in a dose is a safety failure. These errors are easy to make during translation and hard to catch by ordinary reading, because the erroneous value often looks plausible. A dedicated check, ideally by a second person or a systematic tool, catches what reading misses. Treat coded and numeric medical content the way safety-critical engineering treats specifications: verify, do not assume.

### Coordinate Clinical And Terminology Review

Medical translation involving coding systems requires review by clinicians or coding specialists who can verify that the target terminology denotes the correct coded concept. Coordinate this review and treat it as essential.

Linguistic review confirms that the translation reads well and is grammatically correct. Clinical and terminology review confirms that each coded term maps to the correct concept, that verbatim-coded relationships are intact, that the version and hierarchy are consistent, and that no semantic drift has occurred across the code boundary. This review requires domain and coding expertise that a translator may not have. Provide reviewers with the source, translation, termbase, code references, and the coding system version. Incorporate review feedback carefully, because a clinically necessary correction may differ from the linguistically preferred rendering. For content that feeds databases, registries, or submissions, clinical and terminology review is part of the deliverable, not optional.

## Common Traps

### Translating The Word Instead Of The Coded Concept

A fluent target word may denote a neighboring concept with a different code. Always anchor the rendering to the code's defined concept, not to surface equivalence.

### Breaking The Verbatim-To-Coded Relationship

Normalizing a verbatim term toward its code, or altering it so it no longer supports the coded category, destroys the analytically essential relationship. Preserve both faithfully.

### Using Unofficial Renderings Instead Of Official Terminology

Coding systems publish official target-language designations. Substituting a better-sounding term breaks code traceability and makes data non-conformant.

### Ignoring Version And Hierarchy

A code means a specific concept in a specific version at a specific hierarchical position. Wrong-version or wrong-hierarchy renderings corrupt aggregation and analysis invisibly.

### Forcing A Mapping Where None Exists

Inventing a code relationship to cover a gap is data corruption. Record unmapped concepts and escalate them rather than papering over the gap.

### Inconsistent Terminology Across Documents

The same coded term rendered differently across a protocol, CRF, and report is a defect. Reconcile across all documents using a code-anchored termbase.

### Trusting Reading To Catch Code And Value Errors

Transposed codes and misplaced decimals look plausible while changing the concept or result. Verify coded and numeric content through a dedicated check.

### Skipping Clinical And Terminology Review

Linguistic review cannot verify coded concept identity. Database and submission content requires clinical and terminology review as part of the deliverable.

## Self-Check

Before approving medical translation that involves coding systems, verify:

- Each coded term was rendered to the concept defined by its code, not to a surface-equivalent word, with the concept definition consulted before rendering.
- The relationship between verbatim reported terms and their coded categories is preserved exactly, with neither normalized toward the other and mismatches flagged.
- Official target-language designations from the relevant coding system and version were used, with codes recorded alongside terms, and no unofficial renderings substituted.
- Version and hierarchy consistency are maintained, with one coding system version per project and no renderings that imply a different hierarchical position.
- Coding gaps and unmapped concepts are recognized and handled by retention, validated interim translation, or escalation, never by forced or fabricated mapping.
- Terminology is reconciled across all documents and systems using a code-anchored termbase, with identical renderings and codes for the same concept everywhere.
- All codes, values, units, and identifiers were verified through a dedicated check against the source, not only by reading.
- Clinical and terminology review by qualified specialists is coordinated and treated as essential for database, registry, and submission content.
- No coded term has drifted to a neighboring concept, and no verbatim-coded relationship has been broken by normalization.
- The translated medical data remains traceable to the correct codes and denotes exactly the same clinical concepts as the source, so downstream analysis is not corrupted.
