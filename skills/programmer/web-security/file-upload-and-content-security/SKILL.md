---
name: file_upload_and_content_security.md
description: Use when the agent is building or reviewing a feature that accepts file uploads from users — avatars, documents, images, attachments, imports, archives, media — or that processes, transforms, stores, or serves user-uploaded content; deciding where to store uploads, how to validate type and size, how to prevent malicious file execution, path traversal, zip bombs, or stored XSS via uploaded content; serving user content safely (same-origin vs separate domain, Content-Disposition, Content-Type sniffing); or handling archive extraction, image processing, document parsing, or media transcoding of untrusted input. Covers upload validation, storage isolation, content sniffing, archive and parsing safety, and serving untrusted content without enabling stored XSS or RCE.
---

# File Upload And Content Security

File upload is a feature that feels simple — accept a file, store it, maybe show it back — and is one of the richest sources of severe vulnerabilities in web applications. The reason is that an uploaded file is attacker-controlled content that the application stores, often transforms, and frequently serves back to users or to itself. Each of those steps is an opportunity for the content to do something the developer did not intend: a file named `avatar.png` that is actually HTML and renders as stored XSS when served; an image that exploits a parser bug to achieve remote code execution during thumbnailing; an archive that contains a path-traversal entry to overwrite a config file on extraction; a "PDF" whose declared type is a lie and whose bytes are something else entirely. The uploaded file crosses a trust boundary, and almost every component that touches it — the validator, the storage layer, the parser, the serving path — has historically been a vulnerability class.

Agents tend to treat upload validation as "check the extension and the Content-Type header," which is exactly the validation an attacker controls and lies about. The judgment problem is recognizing that an uploaded file's true nature is determined by its bytes and by how each downstream component interprets them, not by the name or header the client supplied, and that defense requires validation at the right layer, isolation of storage and serving, and careful handling of every transformation. This skill covers the end-to-end discipline of accepting, processing, storing, and serving user-uploaded content without enabling stored XSS, path traversal, parser exploitation, or remote code execution.

## Core Rules

### Do Not Trust Client-Supplied Type Information

The filename extension, the `Content-Type` header, and any client-supplied metadata are all attacker-controlled and routinely falsified. Validation that relies on them validates nothing.

- **Validate type by inspecting the file's bytes (magic bytes / file signature), not the extension or header.** Read the leading bytes and compare against known signatures (e.g., PNG begins with `\x89PNG`, JPEG with `\xFF\xD8\xFF`). Use a library that does content sniffing (`file`, `mimetypes`/`python-magic`, `imageinfo`) rather than trusting the name.
- **Even byte-signature validation is not a guarantee of safety.** A file can have a valid image header and still contain embedded exploits or polyglot content valid as multiple types. Byte validation raises the bar but does not eliminate parser risk; combine it with the isolation and serving rules below.
- **Enforce an allow-list of accepted types, not a block-list.** Decide which types you will accept (e.g., PNG, JPEG, PDF) and reject everything else. A block-list of "dangerous" types will miss the next dangerous type; an allow-list cannot.

### Store Uploads Isolated From Executable Contexts

Where and how an uploaded file is stored determines whether a malicious file can be executed or served as active content. The safe default is isolation: store user content where it cannot be executed as code and where it is served from a context that cannot act as the application.

- **Store uploads on a separate origin or domain, not under the application's origin.** Serving `user-content.example.com` from a dedicated domain means a stored-XSS payload in an upload cannot read the application's cookies or DOM at `app.example.com`. This is the single most effective defense against stored XSS via uploads.
- **Never store uploads in a location the web server will execute or serve as the application.** A PHP file in the web root, a template in a templates directory, or a static file served with the application's origin becomes executable or active content. Store outside the document root and serve via a controlled handler, or on a separate static domain.
- **Generate new, random filenames; do not preserve user-supplied names.** The user's filename can contain path traversal (`../../config`), executable extensions, or encoding tricks. Generate a random name (UUID, content hash) for storage; keep the original name only as metadata if needed for download, and sanitize it when used.
- **Store on object storage or a dedicated file service, not the application server's local disk** where feasible, so uploads do not consume application disk, are not in the executable path, and can be served by a CDN.

### Serve User Content With Safe Headers And Disposition

How content is served back determines whether the browser treats it as active content. The headers on the response are the application's statement of what the content is, and they must be set deliberately, not inferred by the browser.

- **Set `Content-Type` to the verified type, and set `X-Content-Type-Options: nosniff`.** Without `nosniff`, browsers may sniff and execute content as a type the application did not intend (e.g., rendering a text file as HTML). `nosniff` forces the browser to honor the declared type.
- **Set `Content-Disposition: attachment` for content that should be downloaded, not rendered.** For documents and files that should not render inline (especially HTML, SVG, and types that can carry script), force download so the browser does not execute them in an application context.
- **Serve from a separate origin with a restrictive cookie and CORS policy.** The upload domain should not receive the application's cookies, should set permissive CORS only where genuinely needed, and should not share origin with authenticated functionality. This contains the blast radius of any active content that slips through.
- **Set `Content-Security-Policy` on the upload origin.** A CSP that disallows inline script and external resources prevents even an HTML/SVG upload from executing script in that origin.

### Bound Size, Count, And Resource Use

Uploads are a denial-of-service vector as much as a security one. An unbounded upload endpoint lets an attacker exhaust disk, memory, or CPU.

- **Enforce a maximum size before or during upload, not after.** Reject uploads exceeding a size limit early — at the reverse proxy or during streaming — so a 10GB upload does not fill memory or disk before validation. Set limits at the load balancer, web server, and application.
- **Limit the number and rate of uploads per user.** Prevent an attacker from filling storage with many small files or overwhelming processing.
- **Quarantine or scan uploads before making them available.** Process (virus-scan, re-encode, validate) in a sandboxed step before the file is reachable by other users or by the serving path.

### Handle Archives And Parsing As Untrusted Input

Extracting archives (zip, tar, gzip) and parsing complex formats (images, documents, media) are among the most exploit-prone operations. Treat the decompressor and parser as running on hostile input.

- **Defend against zip bombs and decompression bombs.** A small archive can decompress to enormous size (a 42KB zip expanding to petabytes). Enforce a maximum decompressed size and count, and use streaming extraction that aborts when limits are exceeded, rather than extracting fully into memory.
- **Defend against path traversal in archive entries.** Archive entries named `../../etc/passwd` or with absolute paths can write outside the extraction directory if the extractor joins paths naively. Resolve each entry's target and verify it stays within the intended directory; reject entries that escape.
- **Keep parsers and image-processing libraries updated and sandboxed.** Image and document parsers have a long history of memory-corruption exploits (buffer overflows in libpng, libjpeg, ImageMagick, PDF parsers). Run parsing in an isolated process or container with limited privileges, keep libraries patched, and consider disabling dangerous delegates (ImageMagick's policy.xml).
- **Limit parsing time and resources.** A maliciously crafted file can cause a parser to consume excessive CPU or memory (the "billion laughs" XML attack, decompression loops). Enforce timeouts and memory limits on parsing.

## Common Traps

### Validating Type By Extension Or Content-Type Header

Trusting the filename extension or the `Content-Type` header, both of which the client controls and falsifies. Validate by inspecting the file's bytes; use an allow-list of accepted types.

### Serving Uploads From The Application Origin

Serving user content from `app.example.com/uploads/`, so an HTML or SVG upload executes as stored XSS with access to the application's cookies and DOM. Serve from a separate origin with restrictive headers.

### Preserving User-Supplied Filenames In Storage

Storing files under the user-supplied name, enabling path traversal, executable extensions, or encoding tricks. Generate random storage names; sanitize any original name used for download.

### Missing `nosniff` Or Wrong Content-Disposition

Omitting `X-Content-Type-Options: nosniff` or serving active content types (HTML, SVG) inline without `Content-Disposition: attachment`, letting the browser sniff or render them as executable content. Set both headers deliberately.

### No Size Or Rate Limits

Accepting uploads of unbounded size or count, allowing an attacker to exhaust disk, memory, or CPU. Enforce size limits at the edge and rate limits per user.

### Naive Archive Extraction

Extracting archives without bounding decompressed size (zip bombs) or validating entry paths (path traversal), allowing storage exhaustion or arbitrary file write. Stream-extract with limits and path validation.

### Unpatched Or Unsandboxed Parsers

Running image or document parsers on untrusted input without updates or isolation, exposing memory-corruption exploits. Keep parsers patched, disable dangerous delegates, and isolate parsing in a sandboxed process.

### Processing Uploads Synchronously In The Request

Performing expensive parsing or transformation inline in the request handler, so a malicious file blocks the worker and becomes a DoS vector. Quarantine and process asynchronously with resource limits.

## Self-Check

- [ ] File type is validated by inspecting the file's bytes (magic bytes / signature), not by the client-supplied extension or Content-Type header, and an allow-list of accepted types is enforced (not a block-list of dangerous ones).
- [ ] Uploads are stored on a separate origin or dedicated domain (not under the application origin), outside the web document root and any executable path, under randomly generated storage names rather than user-supplied filenames.
- [ ] User content is served with a verified `Content-Type`, `X-Content-Type-Options: nosniff`, `Content-Disposition: attachment` for content that should download rather than render, a restrictive cookie/CORS policy on the upload origin, and a CSP that blocks inline script.
- [ ] Maximum upload size is enforced at the edge (load balancer/web server) and during streaming, upload count and rate are limited per user, and uploads are quarantined/scanned before being made available to other users.
- [ ] Archive extraction bounds decompressed size and entry count (streaming, aborting on limit), validates each entry's resolved path stays within the extraction directory (rejecting path traversal), and rejects absolute or escaping paths.
- [ ] Image, document, and media parsers are kept updated, run in an isolated/sandboxed process with limited privileges and resource (CPU/memory/time) limits, and dangerous delegates (e.g., ImageMagick policy.xml) are disabled.
- [ ] Expensive processing (parsing, transcoding, virus scanning) is performed asynchronously with resource limits, not synchronously in the request handler where it can be abused for DoS.
- [ ] The upload feature has been reviewed end-to-end for: type spoofing, stored XSS via active content, path traversal in storage or extraction, parser exploitation, decompression/resource exhaustion, and serving-context confusion — each addressed at the layer where it occurs.
