---
name: rust_serde_and_serialization.md
description: Use when the agent is implementing serialization or deserialization in Rust with serde, deriving Serialize and Deserialize, designing data types that round-trip through JSON/TOML/bincode, customizing field names with serde attributes (rename, skip, skip_serializing_if, default), handling versioned or evolving formats, writing custom Serialize/Deserialize impls, bridging untyped JSON (serde_json::Value) and typed structs, choosing a serialization format, or debugging deserialization errors like "missing field" or "invalid type". Covers serde derive attributes, the typed-vs-untyped boundary, format selection, forward/backward compatibility, and the tradeoff between strict typed deserialization and flexible schema handling.
---

# Serde And Serialization

Serialization converts Rust types to and from external formats (JSON, TOML, bincode, YAML, protobuf). Serde is the ecosystem standard, and its derive macros make most types serializable with a single attribute. The judgment problem is not "add `#[derive(Serialize, Deserialize)]`" but designing types that round-trip correctly, survive format evolution, reject invalid input at the right boundary, and do not leak Rust-specific quirks into a portable format.

Agents tend to derive blindly, expose field names that mirror Rust conventions into wire formats, deserialize everything into `serde_json::Value` and then hand-validate, or break deserialization on every format change because they did not plan for versioning. The harm appears as config files that reject valid input, APIs whose serialized shape changes when a field is renamed in source, deserialization that accepts malformed data, and formats that cannot evolve without breaking every consumer. The real work is treating the serialized shape as a contract, customizing the mapping deliberately, validating at the boundary, and planning for evolution.

## Core Rules

### Treat The Serialized Shape As A Public Contract

Once a type is serialized to a format consumed by others (a config file, an API response, a file on disk), its field names, types, and structure become a contract. Renaming a Rust field changes the wire format and breaks every reader.

- Decide field names for the format deliberately, independent of Rust naming. Rust idiomatically names fields in `snake_case`; JSON and config formats often expect `camelCase` or `kebab-case`. Use `#[serde(rename_all = "camelCase")]` at the struct level or `#[serde(rename = "createdAt")]` per field.
- The serialized shape is part of your API. Changing it is a breaking change for consumers, even if the Rust type is internal.
- Document the expected format with examples, especially for config and file formats humans edit.

### Use Serde Attributes To Control The Mapping Deliberately

Serde's attribute system lets you control exactly how each field maps, which is essential for robust formats.

- **`#[serde(default)]`**: the field is optional on deserialization; if absent, the type's `Default` is used. Essential for adding new fields to an evolving format without breaking older readers.
- **`#[serde(skip)]`**: the field is not serialized or deserialized at all (useful for derived/cached state).
- **`#[serde(skip_serializing_if = "Option::is_none")]`**: omit the field from output when it is `None`, keeping output clean and stable.
- **`#[serde(flatten)]`**: inline a nested struct's fields into the parent in the format, useful for grouping without an extra nesting level.
- **`#[serde(with = "...")]`**: use a custom serializer/deserializer module for a field, e.g., for serializing a `DateTime` in a specific format.

Reach for these attributes whenever the natural Rust representation does not match the desired format. Do not contort the Rust type to fit the format; map it at the serde boundary instead.

### Plan For Format Evolution: Backward And Forward Compatibility

Formats evolve. A field added today must not break a reader written yesterday, and a reader written today should tolerate fields added tomorrow.

- Make new fields optional with `#[serde(default)]` so older data without them still deserializes.
- Use `#[serde(deny_unknown_fields)]` only when you specifically want to reject unknown fields; by default serde ignores unknown fields, which gives forward compatibility (new fields in data are ignored by old readers). For strict config validation, deny unknown fields to catch typos; for extensible formats, allow them.
- For breaking changes (renaming or removing a field), provide a migration path: an alias (`#[serde(alias = "oldName")]`), a version field, or a custom deserializer that accepts both shapes.
- Consider a schema version field (`version: u32`) for formats that may change substantially, so readers can branch on it.

### Deserialize Into Typed Structs At The Boundary, Not Into Value

The strongest pattern is to deserialize external data directly into a typed struct at the system boundary, so the rest of the program works with validated, typed values.

- `let config: Config = serde_json::from_str(&data)?;` validates structure and types in one step; downstream code receives a `Config`, not a bag of `Value`.
- Errors surface immediately at the boundary with precise messages ("missing field `timeout`"), rather than as panics deep in business logic.
- Avoid passing `serde_json::Value` around internally; it is untyped and pushes validation to every consumer.

`serde_json::Value` (or `toml::Value`) is appropriate when the schema is genuinely dynamic or unknown (e.g., a generic JSON API proxy), or for a two-phase parse where you inspect a discriminator then deserialize into the right type. But for known shapes, deserialize to a struct.

### Handle Enums And Tagged Variants Explicitly

Enums serialize in different shapes depending on the representation. For externally-tagged enums (the default), the variant is a key: `{"Circle": {...}}`. For internally-tagged (common for JSON APIs), use `#[serde(tag = "type")]` so the discriminator is a field inside the object: `{"type": "circle", "radius": 5}`. Choose the representation that matches your format's conventions and document it, because it is part of the contract and changing it breaks readers.

Exhaustiveness in deserialization matters: an unknown variant is an error by default, which is usually correct (reject data you do not understand) but can be a compatibility problem if new variants appear. Decide deliberately whether unknown variants should error or be tolerated.

### Choose The Format For The Use Case

Different formats have different strengths.

- **JSON**: ubiquitous, human-readable, no comments, no binary efficiency. Right for web APIs and interop.
- **TOML/YAML**: human-edited config. TOML is flatter and less error-prone; YAML is powerful but has surprising edge cases (the "Norway problem", implicit typing).
- **bincode**: compact binary, Rust-to-Rust only. Right for internal persistence and IPC where both ends are Rust.
- **protobuf / flatbuffers**: schema-driven, language-agnostic, efficient. Right for cross-language systems with a schema contract.

Match the format to the consumers and the editing model. Do not use JSON for hand-edited config (no comments), and do not use YAML for machine-to-machine where its implicit typing causes surprises.

### Validate Semantics After Structural Deserialization

Serde validates structure (fields present, types match) but not semantics (the value is in range, the reference exists, the combination is consistent). After deserializing into a struct, run semantic validation and return a typed error for invalid combinations. Do not assume a value that deserialized is a value that is valid.

## Common Traps

### Field Rename Breaks The Wire Format

Renaming a Rust field without a serde attribute changes the serialized key and breaks all readers. Use `rename`/`alias` to preserve the wire name across internal refactors.

### Adding A Required Field Breaks Old Data

A new field without `#[serde(default)]` causes deserialization of old data (without that field) to fail. Make new fields optional or provide defaults.

### Deserializing Into Value And Hand-Validating Everywhere

Passing `serde_json::Value` through the program pushes type checking to every consumer and produces poor error messages. Deserialize to a struct at the boundary.

### `deny_unknown_fields` Breaking Forward Compatibility

Denying unknown fields rejects data from newer producers that added fields. Use it only for strict, closed schemas (like config where typos should fail), not for extensible formats.

### Exposing Rust Naming In The Format

`snake_case` field names leaking into a JSON API that consumers expect in `camelCase`. Set `rename_all` at the struct level to match format conventions.

### Treating Deserialized Data As Semantically Valid

Structural deserialization success does not mean the values are in range or consistent. Add a semantic validation pass and return typed errors.

### Custom Serialize/Deserialize That Does Not Round-Trip

A hand-written serializer/deserializer pair that does not perfectly invert each other produces data that cannot be read back. Always test round-tripping (serialize then deserialize yields the original) for custom impls.

## Self-Check

- [ ] The serialized field names and structure are treated as a public contract, customized with `rename`/`rename_all` to match format conventions rather than mirroring Rust naming.
- [ ] New or evolving fields use `#[serde(default)]` or are optional so older data still deserializes.
- [ ] External data is deserialized into typed structs at the boundary, not passed around as `serde_json::Value`.
- [ ] `deny_unknown_fields` is used only where a closed schema is intended, not where forward compatibility matters.
- [ ] Enum representation (tagged/untagged/adjacent) is chosen deliberately to match the format and documented as part of the contract.
- [ ] The format was chosen for its consumers and editing model (JSON for APIs, TOML/YAML for config, bincode for Rust-to-Rust).
- [ ] Semantic validation runs after structural deserialization, returning typed errors for out-of-range or inconsistent values.
- [ ] Custom Serialize/Deserialize implementations are tested for round-trip correctness.
- [ ] Breaking format changes provide a migration path (alias, version field, or accepting both shapes).
