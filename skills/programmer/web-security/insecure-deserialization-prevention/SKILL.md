---
name: insecure_deserialization_prevention.md
description: Use when the agent is deserializing data from an untrusted source — JSON, XML, YAML, pickle, Java ObjectInputStream, .NET BinaryFormatter, PHP unserialize, Ruby Marshal, MessagePack, or any binary or text format that reconstructs objects; building or reviewing a feature that receives serialized data across a trust boundary (APIs, cookies, cache, message queues, file imports, webhooks); choosing a serialization format; or preventing deserialization vulnerabilities including remote code execution via gadget chains, XXE, YAML entity instantiation, and class/property manipulation. Covers safe deserialization patterns, allow-listing types, format selection, parser hardening, and the blast radius of deserializing attacker-controlled bytes.
---

# Insecure Deserialization Prevention

Deserialization is the operation that turns bytes back into objects, and it is one of the few vulnerability classes that routinely yields direct remote code execution. The reason is structural: many serialization formats and runtimes, when reconstructing an object graph, will instantiate arbitrary classes, invoke methods, set properties, and resolve references as part of the process — all driven by the bytes being deserialized. If those bytes come from an attacker, the attacker controls which classes are instantiated and which code runs, and a chain of legitimate classes whose methods have side effects (a "gadget chain") can be assembled to achieve arbitrary behavior: opening files, making network connections, executing commands. The defect is not a bug in a line of code; it is the decision to hand attacker-controlled bytes to a deserializer that will execute code based on them.

Agents tend to treat deserialization as a routine data-parsing step — "parse the JSON, get the fields" — and to reach for the language's native object serialization because it is convenient (it round-trips complex objects without manual mapping). That convenience is exactly the danger: a format that reconstructs arbitrary objects is a format that executes attacker-chosen code. The judgment problem is recognizing which deserialization operations are safe (parsing a data structure into a fixed schema) and which are dangerous (reconstructing arbitrary objects or invoking logic driven by the input), and choosing formats, parsers, and boundaries that keep deserialization on the safe side. This skill covers the discipline of deserializing untrusted data without enabling code execution, type confusion, XXE, or state manipulation.

## Core Rules

### Never Deserialize Native Object Formats From Untrusted Input

The cardinal rule: do not feed attacker-controlled bytes to a deserializer that instantiates arbitrary classes or invokes methods. The formats and runtimes where this is the default are the ones with the worst historical record, and the safe path is to not use them across trust boundaries.

- **Avoid these formats for untrusted input:** Java `ObjectInputStream`, .NET `BinaryFormatter` (and `NetDataContractSerializer`, `SoapFormatter` without type restrictions), PHP `unserialize`, Ruby `Marshal`, Python `pickle`/`cPickle`/`shelve`, and YAML loaders that instantiate arbitrary Python objects (`yaml.load` without `SafeLoader`). Each of these will instantiate and invoke attacker-chosen types and is a direct RCE vector.
- **If a native format must cross a boundary, enforce a strict type allow-list.** Some runtimes support filtering the classes that may be deserialized (Java's `ObjectInputFilter`, .NET serialization binders). Configure an allow-list of expected types and reject everything else. Treat this as defense-in-depth, not as license to use the format freely — allow-lists have been bypassed historically.
- **Prefer data formats that deserialize into fixed schemas.** JSON, protobuf, MessagePack (with schema), and similar formats that map bytes into a predefined structure you control do not instantiate arbitrary classes. They are safe by construction for the type-instantiation risk, though they still need validation (below).

### Use Safe Parsers And Safe Defaults For Text Formats

Even formats that do not instantiate arbitrary classes have parser-level dangers, particularly XML and YAML, whose specifications include features that process external content or expand entities in ways that become attacks.

- **For XML, disable external entity processing (XXE).** XML parsers that resolve external entities, external DTDs, or XIncludes can be directed to read local files, perform SSRF, or exhaust memory via entity expansion (the "billion laughs" attack). Configure the parser with `disallow-doctype-decl`, `external-general-entities=false`, `external-parameter-entities=false`, or use a parser that is XXE-safe by default. Never feed untrusted XML to a parser with external entity resolution enabled.
- **For YAML, use the safe loader.** `yaml.safe_load` (Python) or equivalent constructs a data structure without instantiating arbitrary objects; `yaml.load` without `SafeLoader` can execute arbitrary code via tags. Always use the safe loader for untrusted YAML.
- **For JSON, the format itself is safe against code execution, but watch for parser quirks.** Some JSON parsers accept duplicate keys (last wins, enabling key confusion), trailing content, or non-JSON values (NaN, Infinity). Use a strict parser and reject malformed input.

### Deserialize Into A Fixed, Controlled Schema

Safe deserialization maps input into a structure you define, not into whatever the input claims to be. The schema is the boundary between "data" and "code," and it must be yours.

- **Deserialize into plain data objects (structs, dicts, data classes), not into rich domain objects with behavior.** A request body maps to a DTO; the DTO is then validated and used to construct or update domain objects. Do not bind input directly onto entities that have methods, lifecycle hooks, or security-relevant state — an attacker setting an `isAdmin` field via mass assignment is a deserialization-adjacent vulnerability.
- **Validate the deserialized structure against expectations before use.** Type, presence, range, format, and enum checks on the parsed fields. Deserialization succeeding only means the bytes were well-formed; it does not mean the values are acceptable.
- **Ignore or reject unexpected fields.** A payload with extra fields may be a probing attempt or a mass-assignment vector. Configure the deserializer to fail on or ignore unknown fields rather than silently populating them.

### Treat Every Trust Boundary As A Deserialization Boundary

Deserialization happens wherever bytes become objects, and untrusted bytes arrive at more places than the obvious API body. Each is a boundary that needs the same discipline.

- **API request and response bodies** are the obvious case, but also consider: **cookies and session data** (if you serialize state into a cookie), **cache entries** (if cache values are serialized objects and the cache is shared or attacker-influenced), **message queue payloads** (consumers must treat producers as untrusted), **file imports** (user-uploaded data files), **webhook payloads** (third-party-controlled), **redirect parameters and signed tokens** (verify signature, then deserialize safely), and **inter-service RPC** (treat other services as untrusted if compromised-service is in the threat model).
- **Apply the same rules regardless of transport.** Data over an internal network, from a "trusted" partner, or from your own cache is still deserialized input; if the source can be compromised or spoofed, treat it as untrusted.

### Prefer Schema-Based And Forward-Backward Compatible Formats

The choice of serialization format for inter-system communication affects not only security but also compatibility and evolution. Prefer formats designed for this purpose.

- **Protobuf, Avro, FlatBuffers, and Cap'n Proto** are schema-based, compact, and designed for evolution. They deserialize into a fixed schema and do not instantiate arbitrary classes, making them safe and evolvable for RPC and storage.
- **JSON with a documented schema (JSON Schema)** is human-readable and ubiquitous, suitable for public APIs and configuration, with the validation caveats above.
- **Avoid language-native object serialization for persistence or communication.** It couples the data to a specific language and version, is hard to evolve, and carries the instantiation risk. Use it only for same-process or fully-trusted same-runtime scenarios.

## Common Traps

### Deserializing Pickle, Marshal, ObjectInputStream, Or BinaryFormatter From Untrusted Input

Handing attacker-controlled bytes to a deserializer that instantiates arbitrary classes, achieving direct remote code execution via gadget chains. Use safe, schema-based formats; never use native object formats across trust boundaries.

### `yaml.load` Without `SafeLoader`

Loading untrusted YAML with a loader that instantiates arbitrary Python objects via tags, enabling code execution. Always use `yaml.safe_load` or `SafeLoader`.

### XML Parsing With External Entities Enabled (XXE)

Parsing untrusted XML with a parser that resolves external entities or DTDs, allowing local file read, SSRF, or entity-expansion DoS. Disable external entities and DOCTYPE declaration; use an XXE-safe parser configuration.

### Binding Input Directly Onto Rich Domain Objects

Deserializing request data directly onto entities with methods or security state, enabling mass assignment of privileged fields (`isAdmin`, `role`). Deserialize into plain DTOs, then construct domain objects explicitly.

### Accepting Unknown Fields Silently

Allowing the deserializer to populate unexpected fields, enabling probing and mass-assignment vectors. Configure strict handling that rejects or ignores unknown fields.

### Treating Internal Or Cache Sources As Trusted

Deserializing objects from cache, message queues, or partner systems without validation, on the assumption the source is trusted — which fails when the source is compromised or spoofed. Treat every boundary as untrusted if the source can be compromised.

### Relying On A Type Allow-List As Sufficient Defense

Using a deserialization type filter as the sole defense while continuing to use a dangerous native format, when allow-lists have been historically bypassed. Prefer a safe format; treat allow-lists as defense-in-depth.

## Self-Check

- [ ] No native object-serialization format (pickle, Marshal, ObjectInputStream, BinaryFormatter, PHP unserialize, unsafe YAML load) is used to deserialize untrusted input; schema-based formats (JSON with schema, protobuf, Avro, MessagePack with schema) are used instead.
- [ ] XML parsing disables external entity processing, external DTDs, and XIncludes (XXE-safe configuration), and the parser rejects entity-expansion attacks (billion laughs).
- [ ] YAML is loaded with a safe loader (`yaml.safe_load` / `SafeLoader`) that does not instantiate arbitrary objects.
- [ ] Input is deserialized into plain data objects (DTOs, structs, dicts) with a fixed schema, not directly onto rich domain objects with methods or security-relevant state, preventing mass assignment.
- [ ] Deserialized fields are validated (type, presence, range, format, enum) before use, and unexpected or unknown fields are rejected or ignored rather than silently populated.
- [ ] Every trust boundary is treated as a deserialization boundary — API bodies, cookies, cache, message queues, file imports, webhooks, signed tokens, and inter-service RPC — with the same safe-deserialization discipline applied regardless of transport or assumed trust.
- [ ] Where a native format must cross a boundary, a strict type allow-list (Java ObjectInputFilter, .NET serialization binder) is enforced as defense-in-depth, and this is not used as license to use the format freely.
- [ ] The serialization format chosen for persistence and inter-system communication is schema-based and forward/backward compatible, not language-native object serialization that couples data to a runtime and carries instantiation risk.
