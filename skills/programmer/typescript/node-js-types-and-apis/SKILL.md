---
name: typescript_node_js_types_and_apis.md
description: Use when the agent is writing TypeScript for Node.js, typing the Node standard library (fs, path, stream, http, Buffer, process, events, child_process), using @types/node, configuring tsconfig for Node (module/moduleResolution NodeNext, lib without DOM), typing Readable/Writable/Duplex streams and async iteration, typing http request/response, Buffer vs string vs Uint8Array, process and env typing, Error.code narrowing (ENOENT, EADDRINUSE), or is diagnosing "Buffer is not assignable to Uint8Array", "Cannot find module 'fs'", DOM lib conflicts with Node types, stream type mismatches, or callback-vs-promise API confusion. Covers Node platform typing, the lib/@types/node configuration, stream and Buffer typing, and the pitfalls of mixing DOM and Node type worlds.
---

# Node.js Types And APIs In TypeScript

Node.js is a distinct runtime from the browser, and its TypeScript configuration and typing pitfalls are entirely different even though the language is the same. The browser ships a `DOM` lib with `window`, `document`, `fetch`, `XMLHttpRequest`, `File`, `Blob`. Node ships none of those; instead it has `fs`, `path`, `http`, `Buffer`, `process`, `stream`, `child_process`, and a module system that evolved from CommonJS to native ESM. A TypeScript project that includes the `DOM` lib in a Node server silently makes browser globals available as types — `fetch` and `document` type-check even though they do not exist at runtime in older Node — and a project that omits `@types/node` cannot see `process` or `Buffer`. Layered on top are the genuinely hard type problems of Node: streams have a complex generic hierarchy (Readable/Writable/Duplex/Transform, readable async iteration, object mode vs byte mode), `Buffer` is a subclass of `Uint8Array` with subtle assignability rules, the `fs` API has callback, Promise, and synchronous variants whose types differ, and error handling narrows on `Error.code` strings (`ENOENT`, `EADDRINUSE`) rather than subclasses. The judgment problem is to configure the project for the Node platform (lib, module resolution, `@types/node`), to use the right API variant (Promise `fs` over callback, async iteration over event handlers), and to narrow Node errors by their `code` field.

Agents frequently start from a browser-oriented tsconfig, hit "Cannot find name 'process'", add `DOM` to lib to make red squiggles go away, and then ship server code that calls `fetch` assuming it exists, or that mishandles streams because the types were permissive. The remedy is to set `lib` to the Node feature set, install `@types/node` matching the runtime version, choose `NodeNext`/`Node16` module resolution for modern Node, prefer the Promise/async-iterable forms of Node APIs, and narrow errors by `code`.

## Core Rules

### Configure lib And @types/node For The Node Platform, Not The Browser

A Node TypeScript project should not include the `DOM` lib. Set `lib` to the ECMAScript feature set of the target Node version (e.g., `ES2022`), omit `DOM`, and install `@types/node` at a version matching the runtime. Including `DOM` makes browser globals (`document`, `window`, `fetch` on older Node, `File`, `Blob` with browser semantics) type-check without existing, producing runtime `ReferenceError`s that the compiler did not catch.

- `lib: ["ES2022"]` (no `DOM`) for a Node server; add `@types/node` for `fs`, `path`, `process`, `Buffer`, etc.
- Match `@types/node` to the runtime: a Node 20 runtime with `@types/node@14` hides newer APIs (`fetch`, `structuredClone`, `crypto.randomUUID`); a Node 14 runtime with `@types/node@20` exposes APIs that do not exist.
- If you need a few browser-shaped types (`File`, `FormData`) that Node also provides, they come from `@types/node` or undici, not from `DOM`.

### Use NodeNext/Node16 Module Resolution For Modern Node

Node's module resolution distinguishes ESM (`.mjs`/`type: module`/`.js` in ESM packages) from CJS (`.cjs`/default), and the `NodeNext`/`Node16` `moduleResolution` setting makes TypeScript mirror that: ESM files must use `import`/`export`, CJS files use `require`, extensions matter in ESM, and the `type` field in `package.json` governs interpretation. Using the older `node` (classic) resolution hides these distinctions and produces code that type-checks but fails at runtime under native ESM.

- Set `module: NodeNext` and `moduleResolution: NodeNext` for Node projects; let the `type` field drive ESM vs CJS.
- In ESM, import with extensions (`import './foo.js'` even for `.ts` source) per Node's resolution rules.
- Use `import` for ESM dependencies and `require`/`createRequire` for CJS interop; do not assume `require` exists in an ESM file.

### Prefer Promise And Async-Iterable APIs Over Callbacks And Event Handlers

Node's older APIs are callback-based (`fs.readFile(path, cb)`) and event-based (`stream.on('data', ...)`); the modern equivalents are Promise-based (`fs.promises.readFile`) and async-iterable (`for await (const chunk of stream)`). The Promise/async forms compose with `async`/`await`, return typed values, and avoid the error-first callback pyramid. Prefer them unless you have a specific reason (interop with a callback-only API, backpressure-sensitive event handling).

- Use `fs/promises` (`fsPromises`) over callback `fs` for file work; type returns as `Buffer` or `string`.
- Iterate streams with `for await (const chunk of stream)` over `.on('data')`; it respects backpressure and is typed.
- `stream.pipeline` (Promise form) over manual `.pipe()` chains, which do not propagate errors or cleanup reliably.

### Type Streams By Their Mode And Direction

Node streams are generic over their chunk type and have a class hierarchy: `Readable`, `Writable`, `Duplex`, `Transform`. In object mode, chunks are arbitrary objects; in byte mode (the default for `fs` streams), chunks are `Buffer`. Type a stream parameter by what it carries (`Readable<Buffer>` conceptually, or use the web streams `ReadableStream<Uint8Array>` for the standard type where interop matters). Async iteration yields the chunk type. Mixing a `string`-emitting stream with a `Buffer`-expecting consumer type-checks loosely and corrupts at runtime.

- Distinguish Node streams (`stream.Readable`) from web streams (`ReadableStream`); they are different types with conversion helpers (`Readable.toWeb`/`fromWeb`).
- Object-mode streams need a typed chunk; do not assume `Buffer`.

### Handle Buffer, Uint8Array, And String Deliberately

`Buffer` is a Node-specific subclass of `Uint8Array` with extra methods (`toString`, `concat`, allocation). A `Buffer` is assignable to `Uint8Array` but a generic `Uint8Array` is not a `Buffer`; APIs that return `Uint8Array` (web crypto, `TextEncoder`) cannot be passed to APIs typed to require `Buffer` without conversion (`Buffer.from`). Encoding matters: a `Buffer` is bytes; converting to/from a string requires an explicit encoding (`utf8`, `hex`, `base64`), and the default for `buf.toString()` is `utf8`. Do not assume byte length equals string length (multi-byte characters).

- Convert at boundaries: `Buffer.from(uint8)` to get a `Buffer`, `new Uint8Array(buf)` (or the buffer itself, since `Buffer extends Uint8Array`) to pass to web APIs.
- Always pass encoding explicitly when converting; never rely on the default for non-UTF-8 data.

### Narrow Node Errors By code, Not By Class

Node errors carry a `code` string property (`ENOENT`, `EACCES`, `EADDRINUSE`, `ECONNRESET`) rather than distinct subclasses. Narrow by checking `err.code` after confirming the value is an error with a `code` property. This is different from custom-application error handling (where subclasses are common) and from the browser; it is the standard Node pattern and must be typed (`err` is `unknown` in `catch`, narrow to `NodeJS.ErrnoException` or check `'code' in err`).

- `catch (err) { if (err instanceof Error && err.code === 'ENOENT') ... }` — note `code` is not on the base `Error` type, so narrow to `ErrnoException` or use a `'code' in err` guard.
- Handle the specific codes that matter (`ENOENT` for missing file, `EADDRINUSE` for port in use) and rethrow the rest.

## Common Traps

### DOM Lib In A Node Project

Including `DOM` makes `document`/`window`/`fetch`(old Node) type-check without existing. Omit `DOM`, use `@types/node`.

### @types/node Version Mismatch

`@types/node@14` on Node 20 hides `fetch`/`crypto.randomUUID`; `@types/node@20` on Node 14 exposes them. Match the versions.

### Wrong Module Resolution For Native ESM

Classic `node` resolution lets you write CJS-style imports that fail under native ESM. Use `NodeNext` and respect extensions and `type: module`.

### Callback API Where Promise Was Intended

`fs.readFile(path, cb)` mixed with `await` produces a `void` result. Use `fs/promises`.

### Buffer Not Assignable To Uint8Array (Or Vice Versa)

A web API returning `Uint8Array` passed to a Node API requiring `Buffer` fails the type. Convert with `Buffer.from`.

### Event-Based Stream Without Error Handling

`.on('data')` without `.on('error')` leaves errors unhandled and the stream leaking. Use async iteration or `pipeline`.

### Assuming Error Has A code Property

`err.code` on a base `Error` is a type error; narrow to `NodeJS.ErrnoException` or guard with `'code' in err`.

### Default Encoding Assumptions

`buf.toString()` defaults to `utf8`; `Buffer.from(str)` defaults to `utf8`. For binary data this corrupts; pass the encoding explicitly.

## Self-Check

- [ ] `lib` omits `DOM` and targets the Node feature set, `@types/node` is installed at a version matching the runtime, and no browser globals type-check without a Node equivalent.
- [ ] Module resolution is `NodeNext`/`Node16`, the `type` field drives ESM/CJS, ESM imports use extensions, and `require`/`createRequire` is used only for deliberate CJS interop.
- [ ] File and I/O work uses `fs/promises` and async-iterable streams (`for await ... of stream`), with `stream.pipeline` (Promise) for multi-stage pipelines rather than `.pipe()` chains.
- [ ] Streams are typed by direction and chunk type, Node streams and web streams are distinguished and converted at boundaries, and object-mode vs byte-mode is explicit.
- [ ] `Buffer`, `Uint8Array`, and `string` are converted at boundaries with explicit encoding, and byte length is not confused with string length.
- [ ] Node errors are narrowed by `code` (with `NodeJS.ErrnoException` or a `'code' in err` guard), specific codes (`ENOENT`, `EADDRINUSE`) are handled, and others are rethrown.
- [ ] No DOM-typed global (`document`, `window`, browser `fetch`/`File`) is used in server code without a Node/undici equivalent, and the runtime version actually provides any modern API used.
- [ ] The Node typing has been considered under the runtime version, module format, stream modes, and error shapes, and remains correct end to end.
