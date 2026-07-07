---
name: go_serialization_json_and_io.md
description: Use when the agent is writing or reviewing Go code that serializes or deserializes data with encoding/json, encoding/gob, encoding/xml, encoding/csv, or third-party libraries (msgpack, protobuf), implements json.Marshaler/Unmarshaler, uses io.Reader/io.Writer and io.Copy, streams large data, handles JSON tag customization, number/string ambiguity, time formats, or diagnoses performance issues, precision loss from float/number handling, streaming-vs-buffer tradeoffs, or Reader/Writer composition errors.
---

# Go Serialization, JSON, And IO

Go's `encoding/*` packages and the `io` package are the foundation of data interchange and streaming. `encoding/json` is the default for web APIs; `io.Reader`/`io.Writer` are the universal streaming interfaces. The judgment problem is that serialization and IO have many correctness and performance edges: JSON's number type is ambiguous (parsed as `float64` by default, losing precision for large integers), streaming vs buffering trades memory for throughput, custom `Marshaler`/`Unmarshaler` implementations must be correct and efficient, and composing `Reader`/`Writer` correctly (closing, flushing, error handling) is easy to get wrong. Choosing the wrong format or buffering strategy produces precision bugs, memory blowups on large payloads, and silent data corruption.

Agents tend to use `json.Unmarshal` into `interface{}` (losing type safety and precision), to buffer entire large payloads in memory, to forget `Flush`/`Close` on writers, or to ignore IO errors. The judgment problem is to unmarshal into typed structs, to stream large data through `Reader`/`Writer` rather than buffering, to implement custom marshalers correctly, and to choose the format by the use case (JSON for web, protobuf/gob for efficiency, CSV/XML where appropriate). This skill is about treating serialization and IO as precision, performance, and correctness decisions.

## Core Rules

### Unmarshal Into Typed Structs, Not interface{}

`json.Unmarshal(data, &v)` where `v` is `interface{}` parses JSON numbers as `float64` (losing precision for integers beyond 2^53), booleans, strings, arrays, and objects into `map[string]interface{}`. This loses type safety, loses integer precision, and produces reflection-heavy, slow code. Always unmarshal into a typed struct (or map of concrete types):

```
type Order struct {
    ID    int64   `json:"id"`
    Total float64 `json:"total"`
}
var o Order
json.Unmarshal(data, &o)
```

For fields where JSON numbers may exceed float64 precision (IDs, timestamps, large counts), use `json.Number` (decode with `Decoder.UseNumber()`) or a `string` field with a custom type, to preserve the exact representation. Never trust a `float64`-parsed large integer ID to round-trip exactly.

### Stream Large Data Through Reader/Writer, Do Not Buffer Entirely

For large payloads (files, large API responses), use streaming (`json.NewDecoder(r).Decode(&v)`, `json.NewEncoder(w).Encode(v)`, `io.Copy(dst, src)`) rather than reading the whole thing into memory (`io.ReadAll` then `Unmarshal`). Streaming caps memory at the buffer size, not the payload size, which is essential for multi-megabyte or unbounded payloads.

`io.Copy(dst, src)` copies from a reader to a writer in chunks; it is the idiomatic way to stream. `json.NewDecoder` reads and decodes incrementally and can decode a stream of multiple JSON values (e.g., newline-delimited JSON). Use `Decoder`/`Encoder` for streams, `Marshal`/`Unmarshal` for in-memory values.

For very large JSON, consider a streaming JSON parser (jsoniter, or the `json.Decoder` `Token` API) that does not build the whole object tree. For structured large data, a row-oriented format (CSV, newline-delimited JSON, protobuf) streams more naturally than a single huge JSON object.

### Implement Marshaler/Unmarshaler Correctly And Efficiently

Custom `MarshalJSON`/`UnmarshalJSON` lets a type control its serialization. Implement them when the default struct serialization is wrong (a custom format, a computed field, an enum represented as a string). The implementation must be correct (round-trip: `Unmarshal(Marshal(x))` recovers `x`) and efficient (avoid re-allocating buffers; use `json.Marshal` on a helper struct rather than hand-building JSON strings, which is error-prone).

Common patterns: marshal a different struct (the "external representation") and unmarshal back, converting to/from the internal type; use `json.RawMessage` to defer decoding of a sub-field until you know its type (for a discriminated union); use `omitempty` to skip zero values. Validate in `UnmarshalJSON` (reject invalid input) rather than after, so an invalid value never materializes.

### Handle JSON Tags And Edge Cases Deliberately

Struct tags (`` `json:"fieldName,omitempty"` ``) control the JSON field name and behavior. Know the edge cases:

- `omitempty` omits a field when it is the zero value — but the zero value may be valid (a `0` quantity, an empty string that means "default"), in which case `omitempty` hides data. Use a pointer (`*int`) to distinguish "absent" (nil) from "zero" (0).
- `string` tag (`json:",string"`) encodes a number as a JSON string — useful for large integer IDs that exceed JavaScript's safe integer range, but changes the wire format.
- Unexported fields are not serialized; this is sometimes surprising.
- Anonymous (embedded) structs have their fields promoted, which affects the JSON layout.

Time values serialize as RFC3339 strings by default; ensure both sides agree on the format and timezone (prefer UTC). For custom time formats, implement `MarshalJSON`/`UnmarshalJSON`.

### Compose Reader/Writer Correctly: Close, Flush, And Check Errors

`io.Reader`/`io.Writer` compose (a `bufio.Writer` wraps a `Writer`, a `gzip.Writer` wraps that), but the composition has correctness rules:

- **Close/Flush**: a `bufio.Writer`, `gzip.Writer`, or `json.Encoder` buffers data; you must `Flush` (or `Close`) to write the final buffer. Forgetting `Flush` truncates the output silently. Use `defer w.Close()` (checking the error) for writers that need closing.
- **Error checking**: `Writer.Write` can return an error that must be checked; ignoring it loses data silently. After a series of writes, check the error (a common pattern is to check once at the end, since most writers latch the error).
- **io.Copy error**: `io.Copy` returns an error from either the read or the write; check it.
- **Reader EOF**: `Read` returns `io.EOF` to signal end-of-stream, which is not an error; distinguish it from a real read error.

Wrap multi-write operations so a write error aborts the rest. For file IO, close in a `defer` and check the close error (especially for writes, where close may flush and fail).

### Choose The Format By Use Case

- **JSON**: human-readable, ubiquitous for web APIs, but verbose and slow. Use for public APIs and configuration.
- **Protocol Buffers / msgpack / CBOR**: compact, fast, schema-driven. Use for internal high-throughput RPC, storage, or when payload size matters.
- **gob**: Go-to-Go binary encoding, type-aware. Use only between Go services (it is not cross-language).
- **CSV / XML**: when the interchange format is dictated externally (spreadsheets, legacy systems).
- **newline-delimited JSON (NDJSON)**: streams naturally (one JSON object per line), good for logs and event streams.

Do not default to JSON for high-throughput internal communication; protobuf or msgpack can be 5-10x smaller and faster.

## Common Traps

### Unmarshaling Into interface{} Loses Integer Precision

`json.Unmarshal` into `interface{}` parses numbers as `float64`, losing precision for large integers. Unmarshal into typed structs or use `json.Number`.

### Buffering A Huge Payload In Memory

`io.ReadAll(r)` then `Unmarshal` loads the whole payload into memory. Stream with `json.NewDecoder` and `io.Copy`.

### Forgetting Flush/Close On A Buffered Writer

A `bufio.Writer` or `gzip.Writer` not flushed truncates output. `defer w.Close()` and check the error.

### omitempty Hiding A Valid Zero Value

`omitempty` on `Quantity int` drops a legitimate `0`. Use a pointer (`*int`) to distinguish absent from zero.

### Ignoring Write Errors

`w.Write(data)` errors ignored lose data silently. Check errors, especially for a series of writes.

### JSON For High-Throughput Internal RPC

JSON is verbose and slow; protobuf/msgpack is much smaller and faster for internal communication.

### Hand-Building JSON Strings

`fmt.Sprintf(`{"id":%d}`, id)` is error-prone (escaping, injection). Use `json.Marshal` on a struct.

### Time Format/Timezone Mismatch

JSON time defaults to RFC3339; both sides must agree on format and timezone. Prefer UTC and ISO8601/RFC3339.

## Self-Check

- [ ] JSON is unmarshaled into typed structs (or maps of concrete types), not `interface{}`; large integer fields use `json.Number`, `string` tags, or custom types to preserve precision beyond float64.
- [ ] Large payloads stream through `Reader`/`Writer` (`json.NewDecoder`, `io.Copy`) rather than being buffered entirely in memory; `io.ReadAll` is reserved for bounded payloads.
- [ ] Custom `MarshalJSON`/`UnmarshalJSON` round-trip correctly, are validated on input, and use `json.Marshal` on a helper struct rather than hand-built strings; `json.RawMessage` defers decoding of polymorphic fields.
- [ ] Struct tags are used deliberately: `omitempty` does not hide valid zero values (pointers distinguish absent from zero), `string` tags handle large-integer wire formats, and embedded-struct promotion is understood.
- [ ] Composed writers (`bufio`, `gzip`, `json.Encoder`) are flushed/closed (with error checking) so output is not truncated, and write/read errors are checked rather than ignored.
- [ ] The serialization format matches the use case: JSON for public APIs and config, protobuf/msgpack for high-throughput internal RPC, gob only Go-to-Go, NDJSON for streaming events.
- [ ] Time values agree on format and timezone (UTC, RFC3339) across producer and consumer.
- [ ] No JSON is hand-built by string concatenation; `json.Marshal` is used to handle escaping and structure correctly.
