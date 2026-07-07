---
name: io_nio_and_networking.md
description: Use when the agent is doing file or network I/O in Java — reading/writing files (InputStream/OutputStream, Reader/Writer, Files, NIO Channels/Buffers/Path), performing blocking socket I/O (Socket, ServerSocket) or non-blocking I/O (Selector, SocketChannel, SelectionKey), using HTTP clients (HttpClient), handling character encoding (Charset, UTF-8), parsing serialized formats, or diagnosing resource leaks (unclosed streams/sockets), character encoding bugs (mojibake), performance problems (byte-by-byte reads, missing buffering), NIO selector spin, or blocking calls that never return. Covers buffering, charset handling, NIO vs classic I/O, resource management, and blocking-vs-async I/O tradeoffs.
---

# IO, NIO, And Networking

Java has two parallel I/O APIs: the classic `java.io` stream hierarchy (`InputStream`, `OutputStream`, `Reader`, `Writer`), added in JDK 1.0, and the newer `java.nio` channel/buffer/selector API (`Channel`, `ByteBuffer`, `Selector`), added in JDK 1.4. Layered on top are the convenience methods in `java.nio.file.Files` (JDK 7+) and the modern `java.net.http.HttpClient` (JDK 11+). Each layer has different performance characteristics, error semantics, and resource-management obligations, and the most common bugs — resource leaks, character-encoding corruption, and pathological performance — come from mixing layers carelessly or ignoring the buffering and charset rules that each layer silently depends on.

The judgment problem is not "how do I read a file" but "which I/O API matches my access pattern, am I buffering at the right granularity, am I handling character encoding explicitly rather than relying on the platform default, and am I closing every resource on every exit path including exceptions." Agents tend to read byte-by-byte without buffering, rely on the default charset (which varies by OS and locale, producing mojibake in production), forget to close streams on exception paths, use blocking I/O where async would prevent thread exhaustion, and reach for raw NIO selectors when a higher-level abstraction would be correct and simpler. The cost is slow systems, corrupted text, leaked file descriptors, and threads that block forever.

## Core Rules

### Always Buffer Byte-By-Byte Or Char-By-Char I/O

Reading or writing a file one byte (or one character) at a time through an unbuffered stream is catastrophically slow: each `read()` call is a syscall (or at least a JVM-to-native transition). A file read byte-by-byte can be 100x slower than the same read through a `BufferedInputStream` with an 8KB buffer. The rule: never call `read()`/`write()` on an unbuffered stream in a loop; always wrap in `BufferedInputStream`/`BufferedOutputStream` (for bytes) or `BufferedReader`/`BufferedWriter` (for characters), or use the bulk `read(byte[])`/`write(byte[])` methods with a manually managed buffer.

The modern alternative is `java.nio.file.Files`, which provides bulk operations (`Files.readAllBytes`, `Files.readString`, `Files.lines`, `Files.write`) that handle buffering internally. For most file reads, `Files.readString(path)` (Java 11+) or `Files.readAllLines(path)` is the right choice — they are buffered, charset-aware, and close the resource automatically. Reserve manual `InputStream`/`BufferedInputStream` for cases where you need streaming control (large files, partial reads, binary formats) that the convenience methods do not support.

For network I/O, the same rule applies: wrap `Socket.getInputStream()` in `BufferedInputStream` before reading in a loop. Network round-trips per byte are even more expensive than file syscalls.

### Specify The Charset Explicitly, Never Rely On The Default

The single most common text bug is relying on the platform default charset. `new String(bytes)`, `String.getBytes()`, `new InputStreamReader(in)`, `new FileReader(path)`, and `Files.readAllBytes` without a charset argument all use `Charset.defaultCharset()`, which is determined by the JVM's locale and OS settings — it is UTF-8 on most modern Linux, but it can be windows-1252 on Windows or ISO-8859-1 elsewhere. Code that works on the developer's machine produces mojibake (corrupted characters, `?` replacements, `Ã©` for `é`) in production because the production charset differs.

The rule: always pass the charset explicitly. Use `StandardCharsets.UTF_8` for all text unless you have a specific reason otherwise (a legacy protocol, a fixed-width mainframe format). `new String(bytes, UTF_8)`, `String.getBytes(UTF_8)`, `new InputStreamReader(in, UTF_8)`, `Files.readString(path, UTF_8)`. Java 18+ defaults to UTF-8 (JEP 400), which reduces but does not eliminate the problem — explicit is still safer because it documents intent and survives a JVM flag override.

When you see `new String(bytes)` or `getBytes()` without a charset, treat it as a latent bug. The bytes were encoded with some charset; if it was not UTF-8 and the decoding charset differs, the text is corrupted silently.

### Close Every Resource On Every Path, Including Exceptions

Every `InputStream`, `OutputStream`, `Reader`, `Writer`, `Channel`, `Socket`, `ServerSocket`, `Connection`, and `Selector` holds a finite OS resource (a file descriptor, a socket). File descriptors are limited (typically 1024 to 65536 per process); leaking them eventually throws `TooManyFilesException` (`EMFILE`) and the process cannot open new files or sockets. The only correct way to manage these resources is `try-with-resources` (see the exception-handling skill), which guarantees close on every exit path including exceptions and return statements.

The classic leak pattern is opening a resource, doing work that might throw, and closing in a `finally` — but if the open itself is outside the `try`, an exception before the `try` leaks. Another is returning from inside the `try` without closing — `try-with-resources` handles this; manual `finally` often does not. Every resource opened must be in a `try-with-resources` header; nesting is acceptable when resources have different lifetimes.

Network resources (`Socket`, `ServerSocket`) have an additional obligation: closing the socket does not immediately free the port on some OSes (TIME_WAIT), and lingering connections can exhaust ephemeral ports under high churn. For servers, set `SO_REUSEADDR` and tune `SO_LINGER` where appropriate. For clients, connection pooling (HTTP keep-alive, JDBC pools) reuses sockets rather than opening and closing per request.

### Choose Classic I/O, NIO, Or Async Based On The Concurrency Model

Classic `java.io` streams are blocking: a `read()` blocks the calling thread until data is available. This is simple and correct for low-to-moderate concurrency, but it requires one thread per concurrent connection — which scales poorly past a few thousand connections because of thread memory overhead and context-switching cost. NIO selectors were designed to solve this: a single thread can manage many channels via a `Selector`, waking when any channel is ready, enabling tens of thousands of concurrent connections on one thread.

The decision: use classic blocking I/O (or the modern `HttpClient`/`ExecutorService` abstractions) for most application code. It is simpler, easier to reason about, and performs well up to hundreds of concurrent connections. Reach for raw NIO selectors only when you have a proven need for tens of thousands of concurrent connections and the expertise to handle selector spin, partial writes, and the complexity of non-blocking protocols — and even then, prefer Netty or a similar framework that has solved these problems. Raw selector code is notoriously hard to get right; the number of production bugs from hand-rolled NIO is large.

For HTTP, use `java.net.http.HttpClient` (Java 11+), which supports both synchronous and asynchronous (`CompletableFuture`-returning) requests, HTTP/2, and connection pooling. Do not hand-roll HTTP over `Socket` unless you are implementing a protocol from scratch.

### Handle Partial Reads And Writes Explicitly In Network I/O

A network `read()` is not guaranteed to fill the buffer — it returns as many bytes as are currently available, which may be fewer than requested (a "short read"). Similarly, a network `write()` may write fewer bytes than provided (a "short write"). Code that assumes `read(buf)` fills `buf` completely, or that `write(buf)` sends all bytes, is incorrect for network I/O — it works in testing (where data often arrives in one packet) and fails intermittently in production (where TCP fragmentation produces partial reads).

For classic blocking I/O, use `DataInputStream.readFully(buf)` (which loops until the buffer is full or EOF) when you need a fixed number of bytes, or loop manually on `read()` accumulating into a `ByteArrayOutputStream` until you have a complete message. For NIO, the channel's `read(ByteBuffer)` returns the count read; you must loop, compacting the buffer, until you have a complete frame. Failing to handle partial reads produces truncated messages and protocol corruption that surfaces as intermittent, hard-to-reproduce bugs.

The same applies to writes: `write()` may not write everything; loop on `write()` (or use `Channels.newChannel(out).write(buf)` which loops) until the buffer is fully drained. For non-blocking NIO, partial writes are the norm, and you must buffer the remainder and retry on the next `OP_WRITE`.

## Common Traps

### Unbuffered Byte-By-Byte Reads

`FileInputStream.read()` in a loop without `BufferedInputStream` is 100x slower than buffered. Always buffer, or use `Files.readAllBytes`/`Files.readString`.

### Relying On The Default Charset

`new String(bytes)`, `getBytes()`, `new FileReader(path)` use the platform default, which varies by OS/locale and causes mojibake in production. Always pass `StandardCharsets.UTF_8` explicitly.

### Leaking Resources By Closing In finally Without try-with-resources

Manual `finally` masks exceptions if `close()` throws and misses the resource if the open is outside the `try`. Use `try-with-resources` for every `AutoCloseable`.

### Assuming read() Fills The Buffer On Network I/O

TCP delivers partial reads; `read(buf)` may return fewer bytes than requested. Use `readFully` or loop-and-accumulate for fixed-length reads; frame messages for variable-length protocols.

### Hand-Rolling NIO Selectors For Moderate Concurrency

Raw selector code is error-prone (selector spin, partial writes, incomplete frames). Use blocking I/O, `HttpClient`, or Netty unless you have a proven need for >10k concurrent connections.

### Blocking Forever Without Timeouts

`Socket.read()` with no `SO_TIMEOUT` blocks forever if the peer stops responding. Always set read/connect timeouts (`setSoTimeout`, `connect(addr, timeout)`, `HttpClient` timeouts).

### Ignoring Character Encoding In Legacy APIs

`ObjectInputStream`, `DataInputStream.readUTF()`, and some serializers use fixed or implicit encodings. Verify the encoding of every byte-to-char boundary, especially at system boundaries.

## Self-Check

- [ ] All byte/char I/O is buffered (`BufferedInputStream`/`BufferedReader` or bulk `read(byte[])`/`write(byte[])`), or the convenience methods in `Files` are used; no unbuffered single-byte/single-char reads in loops.
- [ ] Every byte-to-char and char-to-byte conversion specifies `StandardCharsets.UTF_8` (or a documented alternative) explicitly; no reliance on `Charset.defaultCharset()` via `new String(bytes)`, `getBytes()`, or no-charset `InputStreamReader`/`FileReader`.
- [ ] Every `AutoCloseable` resource (`InputStream`, `OutputStream`, `Socket`, `Channel`, `Connection`) is in a `try-with-resources` header, closed on every exit path including exceptions and early returns; no resource is opened outside a try block.
- [ ] The I/O model matches the concurrency requirement: classic blocking I/O or `HttpClient` for normal load; raw NIO selectors or Netty only when a proven need for very high connection counts exists, and never hand-rolled where a framework would do.
- [ ] Network reads handle partial data: `readFully` or loop-and-accumulate for fixed-length reads, explicit framing for variable-length messages; writes loop until the buffer is fully drained.
- [ ] All blocking network operations have timeouts (`SO_TIMEOUT`, connect timeout, `HttpClient` timeout); no `read()`/`accept()`/`connect()` can block indefinitely.
- [ ] File descriptor usage is bounded: server sockets set `SO_REUSEADDR`, connection pooling is used for high-churn clients (HTTP keep-alive, JDBC pools), and there is monitoring for descriptor leaks.
- [ ] Large files are streamed (`Files.lines`, `InputStream` + buffered loop) rather than loaded entirely into memory via `Files.readAllBytes` when the size could exceed available heap.
