---
name: tauri-permissions
description: Tauri v2 permissions and capability configuration rules. Use when configuring plugin access, defining capabilities, setting up ACL/permissions for commands, or debugging "command not allowed" / permission denied errors in Tauri apps.
---

# Tauri Permissions (v2)

Tauri v2 enforces a capability-based permission system. Every plugin command and core API must be explicitly allowed. Forgetting a permission = silent or confusing runtime denial.

## Core Model

- **Capability**: a set of permissions granted to a specific window/webview.
- **Permission**: allows specific commands (e.g., `fs:allow-read-text-file`).
- Defined in `capabilities/*.json`.

## Common Traps

### Missing permission for a plugin command
Using a plugin API (fs, dialog, shell, http...) without adding its permission to the capability:
```json
{
  "permissions": ["fs:allow-read-text-file", "dialog:allow-open"]
}
```
Symptom: command works in dev but fails, or throws "not allowed". Check `src-tauri/capabilities/`.

### Scoping too broadly
`core:default` or `fs:allow-*` grants everything. In production, scope to the minimum:
```json
{ "identifier": "fs:scope", "allow": [{"path": "$APPDATA/*"}] }
```
Over-scoping defeats the security model Tauri provides.

### Window-specific capabilities
Capabilities target specific windows via the `windows` array. A command works in the main window but not in a second window because the capability doesn't list it.

### Plugin permissions must be in the capability, not just Cargo.toml
Adding a plugin to `Cargo.toml` enables the Rust code. It does NOT grant the frontend permission to call it. Both are required.

## Self-Check

- [ ] Does every plugin/core API used by the frontend have an explicit permission?
- [ ] Are permissions scoped to the minimum needed (not wildcard)?
- [ ] Do capabilities list all windows that need them?
- [ ] After adding a plugin, is its permission added to capabilities too?
