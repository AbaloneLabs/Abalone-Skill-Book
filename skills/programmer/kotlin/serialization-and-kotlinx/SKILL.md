---
name: kotlin_serialization_and_kotlinx.md
description: Use when the agent is serializing Kotlin objects with kotlinx.serialization, @Serializable and @SerialName, custom serializers (KSerializer), Serializers and contextual/polymorphic serialization, Json configuration (ignoreUnknownKeys, encodeDefaults, coerceInputValues), sealed class serialization with class discriminators, versioning and optional fields, encoding/decoding errors and SerializationException, or is diagnosing "missing field crashes decode", "the default is not applied", "unknown key error", polymorphic discriminator issues, null vs absent distinction, or migrating a data model across versions. Covers kotlinx.serialization design, custom serializers, polymorphism, JSON configuration, versioning, and the pitfalls of strict decoding and defaults.
---

# Serialization And kotlinx.serialization In Kotlin

Serialization looks like a solved problem — annotate a class `@Serializable`, call `Json.encodeToString`/`decodeFromString`, done — and that ease hides a set of decisions that determine whether your data round-trips correctly across versions, trust boundaries, and polymorphic hierarchies. kotlinx.serialization is compile-time plugin-based: it generates a serializer per `@Serializable` class, which is fast and reflection-free, but it also means the serialization behavior is fixed by the class shape and the `Json` configuration at the point of (de)serialization. The recurring failures are: a missing or renamed field crashes decode instead of using a default; a `null` in the input is distinguished from an absent key in ways the type does not express; a polymorphic hierarchy's discriminator is wrong or unregistered; defaults are not emitted so a round-trip drops them; an unknown key from a newer server version is rejected. The judgment problem is to configure the `Json` instance for the contract you want (lenient vs strict, defaults, unknown keys), to model optional/versioned fields deliberately, to handle polymorphism via class discriminators with registered subclasses, and to treat decode as a trust boundary that can fail in typed ways.

Agents add `@Serializable` with no `Json` configuration, hit "missing field" or "unknown key" errors in production, then sprinkle `?` and defaults reactively without understanding the null-vs-absent semantics or the versioning implications. The remedy is to configure one `Json` instance per API contract, to use optional fields and defaults for forward/backward compatibility, to register polymorphic subclasses explicitly, and to decode defensively with typed error handling at boundaries.

## Core Rules

### Configure The Json Instance For The Contract You Want

The default `Json` is strict: it rejects unknown keys, does not encode defaults, and is not lenient. For most real-world API contracts you want a configured instance: `Json { ignoreUnknownKeys = true; encodeDefaults = true; coerceInputValues = true; explicitNulls = false }`. The choices encode the contract: `ignoreUnknownKeys` for forward compatibility (a newer server adds a field; old clients ignore it); `encodeDefaults` so defaults are written (otherwise a round-trip drops fields equal to their default); `coerceInputValues` so an invalid value for a field with a default falls back to the default rather than failing. Define one `Json` instance per contract (a named singleton) and reuse it; do not use the default `Json` implicitly.

- `ignoreUnknownKeys = true` for API responses (forward compatibility).
- `encodeDefaults = true` so defaults round-trip and consumers see explicit values.
- `coerceInputValues = true` so a bad value for a defaulted field uses the default.
- `explicitNulls = false` to omit null fields on encode when absence is preferred.

### Model Optional And Versioned Fields Deliberately

A field can be required (non-null, no default — decode fails if absent), optional-with-default (`val name: String = ""` — absent uses the default), or nullable (`val name: String?` — absent or null decodes to null). These three express different contracts, and the choice affects versioning: a required field added in a new version breaks old payloads that lack it; an optional-with-default field is backward-compatible (old payloads without it use the default); a nullable field distinguishes "absent" from "present and null" only with `explicitNulls = false` on encode. For evolving APIs, prefer optional-with-default for new fields so old payloads decode, and reserve required fields for genuinely mandatory data.

- New fields in an evolving API: optional with a default, so old payloads decode.
- `@SerialName("server_name")` to map a Kotlin property to a different JSON key (snake_case, legacy names).
- `@Serializable` data classes with `?`/defaults model the three contract kinds; choose per field.

### Handle Polymorphism With Class Discriminators And Registered Subclasses

A sealed (or open) hierarchy serializes polymorphically: kotlinx writes a discriminator (default `"type"`) identifying the concrete subclass, plus its fields. Decoding requires the subclass to be registered (`@Serializable sealed class` auto-registers direct subclasses in the same file; for open hierarchies or cross-module, use `SerializersModule` with `polymorphic`). Mismatched discriminator names (`@SerialName`), unregistered subclasses, or a discriminator the receiver does not know produces a `SerializationException`. Define the discriminator name and subclass serial names deliberately and consistently across producer and consumer; a rename is a breaking change.

- `@Serializable sealed interface Event { @SerialName("click") data class Click(...) : Event; ... }` — discriminator + names.
- For open/cross-module hierarchies, register via `SerializersModule { polymorphic(Event::class) { subclass(Click::class) } }`.
- Treat discriminator/serial-name changes as breaking; version them deliberately.

### Distinguish null From absent Explicitly

In JSON, a field can be absent or present-with-null, and these are different. A Kotlin `String?` decodes both absent and `null` to `null` (losing the distinction); to preserve it, use `@EncodeDefault`/`explicitNulls` carefully, or model the field as a custom type that distinguishes the two (an `Optional`-like wrapper). Most contracts collapse the distinction (absent and null both mean "no value"), which is fine; be deliberate where the distinction matters (a patch semantics where absent means "leave unchanged" and null means "clear").

- For patch/update semantics, model the distinction explicitly (a wrapper or `JsonElement` inspection).
- `explicitNulls = false` on encode omits null fields, making null encode as absent.

### Write Custom Serializers For Non-Conforming Types

Types kotlinx cannot auto-serialize (`LocalDate`, `Instant`, `UUID`, a third-party class, a value with a non-standard wire form) need a custom `KSerializer`. Implement `serialize`/`deserialize` to convert to/from a primitive or `JsonElement`, and register it (`@Serializable(with = ...)`, contextual serializers, or a `SerializersModule`). Centralize date/UUID formats in one custom serializer so the whole app is consistent; do not hand-format per call site.

- `object InstantSerializer : KSerializer<Instant> { ... }` for a consistent wire format.
- Register via `@Serializable(with = InstantSerializer::class)` or a `SerializersModule`.
- Centralize formats (ISO 8601, UTC) so every boundary agrees.

### Decode Defensively At Trust Boundaries

Decode of data from an API, file, or message is a trust boundary: the input may not match the type, and `decodeFromString` throws `SerializationException` on mismatch. Catch it at the boundary and convert to a typed error; do not let it propagate as an uncaught exception or silently default. For partially-trusted input, decode to a lenient model first (with `ignoreUnknownKeys`, `coerceInputValues`), validate, then map to a stricter domain type. Log the failing input/path for diagnosis.

- Wrap decode in try/catch for `SerializationException`; convert to a typed error.
- Lenient decode + validation + strict mapping for partially-trusted input.
- Capture the element path in error logging for diagnosis.

## Common Traps

### Missing Field Crashing Decode

A required field absent in the input throws. Make new fields optional-with-default for compatibility.

### Default Not Applied On Absent

A field with a default still fails if the input contains a wrong-type value, unless `coerceInputValues = true`. Configure the `Json` instance.

### Unknown Key Rejected

A newer server's extra field fails decode by default. Set `ignoreUnknownKeys = true`.

### Defaults Not Encoded

`encodeDefaults = false` (default) drops fields equal to their default, breaking round-trips and consumers expecting explicit values. Set `encodeDefaults = true`.

### Polymorphic Subclass Not Registered

An open hierarchy subclass not in a `SerializersModule` throws on decode. Register all subclasses.

### Discriminator Rename Is A Breaking Change

Changing `@SerialName("click")` to `"tap"` breaks existing payloads. Treat as a versioned change.

### null vs absent Collapsed Unintentionally

A `String?` decodes both to null; patch semantics that need the distinction break. Model explicitly.

### Per-Call Date Formatting Inconsistent

Formatting `Instant` per call site produces inconsistent wire formats. Centralize in one custom serializer.

## Self-Check

- [ ] One configured `Json` instance per API contract is defined and reused (`ignoreUnknownKeys`, `encodeDefaults`, `coerceInputValues`, `explicitNulls` set deliberately), and the default strict `Json` is not used implicitly for external data.
- [ ] Fields model their contract: required for mandatory data, optional-with-default for evolving/compatible fields, nullable where null is a valid value, with `@SerialName` mapping legacy/snake-case keys.
- [ ] Polymorphic hierarchies use a deliberate discriminator and serial names, all subclasses are registered (auto for sealed in-file, `SerializersModule` for open/cross-module), and discriminator/name changes are treated as breaking.
- [ ] The null-vs-absent distinction is handled explicitly where it matters (patch semantics), and collapsed deliberately where it does not.
- [ ] Non-conforming types (dates, UUID, third-party) use centralized custom serializers so every boundary agrees on format.
- [ ] Decode at trust boundaries is wrapped to catch `SerializationException` and convert to a typed error, with the element path logged for diagnosis.
- [ ] No required field added in a new version breaks old payloads; new fields are optional-with-default for backward compatibility.
- [ ] The serialization design has been considered under versioning, polymorphism, null/absent, and untrusted input, and round-trips and decodes correctly across them.
