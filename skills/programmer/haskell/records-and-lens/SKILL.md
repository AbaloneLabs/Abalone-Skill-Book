---
name: haskell_records_and_lens.md
description: Use when the agent is working with Haskell records (record syntax, record wildcards, the record field accessor/update, DuplicateRecordFields/OverloadedRecordFields extensions, the name-clash problem), using the lens library (Lens'/Prism'/Traversal'/Iso', view/over/set, (.~)/(?~), composing lenses with (.), makeLenses template haskell), designing records for evolution, choosing between lens and generic-sop/raw records, dealing with nested record updates, or is diagnosing "record field name clashes across types", "nested record update is verbose", "lens is overkill / too magical", or record-design issues. Covers the record name-clash problem and extensions, lens fundamentals and when to use them, makeLenses and TemplateHaskell, nested updates, record wildcards, and the trade-offs of lens vs raw records.
---

# Records And Lens In Haskell

Haskell records have a notorious limitation: field accessor functions share the global namespace, so two record types with a field named `name` clash (`Person { name :: String }` and `Company { name :: String }` both define `name :: a -> String`, an ambiguity). Extensions (`DuplicateRecordFields`, `OverloadedRecordField`) mitigate this, and the `lens` library solves it (each field becomes a type-indexed lens `name :: Lens' a b`), but `lens` is powerful and abstract — easy to over-apply where a plain record update suffices, or to make code unreadable for those unfamiliar with the operators. The judgment problem is to handle the name-clash problem (extensions or lens), to reach for lens when nested updates/abstraction justify it (not for trivial access), to understand `makeLenses`/TemplateHaskell generation, and to keep record design evolvable.

Agents hit field-name clashes, write verbose nested updates (`a { b = a.b { c = ... } }`), or over-apply lens where a plain record is clearer. The remedy is the right extension or lens for clashes, lens for nested/composable updates, and plain records for simple cases.

## Core Rules

### Handle The Record Name-Clause Problem With Extensions Or Lens

The classic record limitation: field accessors are top-level functions, so two records sharing a field name (`name`) clash. Mitigations:

- `DuplicateRecordFields`: allows the same field name in different record types; access is disambiguated by type (`person.name`, or type-directed resolution). The simplest fix for the clash; enable it project-wide.
- `OverloadedRecordField`/`OverloadedRecordDot` (newer): a more systematic solution with dot-syntax (`r.field`) and overloaded accessors.
- `NoFieldSelectors` (GHC 9.2+): disables generating accessor functions (use lens or dot-syntax instead), avoiding clashes entirely.
- `lens` with `makeLenses`: generates type-indexed lenses (`name :: Lens' Person String`), so field access is `view name person`/`person ^. name`, no clash.

Choose by project: `DuplicateRecordFields` for minimal change; `OverloadedRecordDot` for modern dot-syntax; `lens` for full power. Do not live with the clash silently.

- `DuplicateRecordFields` for same-name fields across types (minimal change).
- `OverloadedRecordDot`/`NoFieldSelectors` for modern dot-syntax/no accessors.
- `lens` (`makeLenses`) for type-indexed accessors (no clash).

### Reach For Lens When Nested Updates Or Abstraction Justify It

Lens shines for nested updates and composable access. A plain nested update `cfg { db = cfg.db { pool = cfg.db.pool { size = 10 } } }` is verbose and error-prone; with lens it's `cfg & db . pool . size .~ 10`. Lens also enables traversals (`each`, `mapped`), prisms (sum-type branches), and isos (isomorphisms). But lens is abstract and operator-heavy (`^.`, `.~`, `%~`, `?~`), which can hurt readability for those unfamiliar. Rule: use lens when (1) updates are deeply nested, (2) you need to abstract over field access (a function parameterized by a lens), or (3) you traverse/modify collections of fields. For trivial single-field access/update on a flat record, a plain record (`r { x = ... }`) is clearer. Do not lens-ify everything.

- Lens for nested updates (`& db . pool . size .~ 10`), traversals, prisms, parameterized access.
- Plain records for trivial flat access/update (clearer than lens operators).
- Do not lens-ify trivially; the abstraction must earn its keep.

### Use makeLenses (TemplateHaskell) To Generate Lenses

`makeLenses ''Config` (TemplateHaskell) generates a lens per field (`field :: Lens' Config FieldType`, with the leading underscore dropped: `_field` → `field`). `makeLensesFor` maps fields to custom lens names; `makeClassy` generates a class for subrecords (useful for config/reader patterns). TemplateHaskell has costs (compile time, less portable, some tooling friction) but is the standard way to generate lenses. Keep `makeLenses` in the module defining the record (so the lenses are exported alongside). For libraries avoiding TemplateHaskell, hand-write lenses (`field :: Lens' a b; field f s = (\x -> s { _field = x }) <$> f (_field s)`) or use `generic-lens`/`optics` for derivation without TH.

- `makeLenses ''T` generates a lens per field; `makeLensesFor` for custom names; `makeClassy` for subrecords.
- Keep `makeLenses` in the record's module; be aware of TemplateHaskell costs (compile time, portability).
- Alternatives without TH: hand-written lenses, `generic-lens`, `optics`.

### Design Records For Evolution (Strict Fields, Newtypes, Sum Types)

Record design choices affect evolution:

- Strict fields (`field :: !Int`) avoid space leaks (thunks accumulating) and force evaluation; use for fields that should always be WHNF. Lazy fields for intentional laziness (a stream, a memo).
- `newtype` wrappers (`newtype UserId = UserId Text`) give type safety (can't confuse `UserId` with `OrderId`) at no runtime cost; prefer over bare `Int`/`Text` for identifiers.
- Sum types (`data Status = Pending | Active | Closed`) over "stringly-typed" fields (`status :: String`); the compiler checks exhaustivity.
- Avoid records with too many fields (consider grouping into subrecords) or optional fields modeled as bottom/`undefined` (use `Maybe`).

Design records to make invalid states unrepresentable and evolution (adding fields) non-breaking where possible.

- Strict fields (`!`) to avoid space leaks; lazy for intentional laziness.
- `newtype` wrappers for type-safe identifiers (no cost); sum types over stringly-typed fields.
- Avoid too-many-fields and bottom/`undefined` optionals (use `Maybe`); make invalid states unrepresentable.

### Consider optics/generic-lens As Lighter Alternatives

The `lens` library is large and abstract; `optics` (a newer, more type-safe, more guided library) and `generic-lens` (derives lenses via Generics, no TemplateHaskell) are lighter alternatives. `optics` has better error messages and a more constrained composition (fewer surprising types); `generic-lens` derives lenses for any record without TH. For a new project wanting field access without `lens`'s heft, consider `optics` or `generic-lens`. For an existing `lens` codebase, stay with `lens` (mixing is confusing). The core judgment — lens for nested/composable, plain records for trivial — applies to all.

- `optics` (newer, more type-safe, better errors) and `generic-lens` (Generics-derived, no TH) as lighter alternatives.
- New projects may prefer `optics`/`generic-lens`; existing `lens` codebases stay with `lens`.
- The core judgment (lens for nested/composable, plain for trivial) applies to all.

## Common Traps

### Record Field Name Clashes

Two records with `name` clash. Use `DuplicateRecordFields`/`OverloadedRecordDot`/`lens`.

### Verbose Nested Record Updates

`a { b = a.b { c = ... } }` is error-prone. Use lens (`& b . c .~ ...`).

### Over-Applying Lens To Trivial Records

Lens operators on flat records hurt readability. Use plain records for trivial cases.

### Space Leaks From Lazy Record Fields

Unforced thunks accumulate. Use strict fields (`!`) where appropriate.

### Stringly-Typed Fields

`status :: String` allows invalid values. Use sum types.

### Bare Int/Text For Identifiers

Confusing `UserId` with `OrderId`. Use `newtype` wrappers.

### makeLenses In The Wrong Module

Lenses not exported with the record. Generate in the record's module and export.

### Mixing lens And optics In One Codebase

Confusing for readers. Pick one library per codebase.

## Self-Check

- [ ] Record field-name clashes are handled (`DuplicateRecordFields`/`OverloadedRecordDot`/`NoFieldSelectors`/`lens`), not lived with silently.
- [ ] Lens is used for nested updates, traversals, prisms, and parameterized access; plain records are used for trivial flat access/update (lens not over-applied).
- [ ] `makeLenses` (TemplateHaskell) generates lenses in the record's module and exports them; alternatives (`generic-lens`/`optics`) are considered where TH is undesirable.
- [ ] Records are designed for evolution: strict fields (`!`) to avoid space leaks, `newtype` wrappers for type-safe identifiers, sum types over stringly-typed fields, `Maybe` for optionals.
- [ ] The lens/optics library is chosen consistently per codebase (not mixed); the choice is justified by the project's complexity.
- [ ] Invalid states are made unrepresentable (sum types, newtypes) where feasible; records are not over-sized (grouped into subrecords if many fields).
- [ ] Field access/update is readable (lens operators used where they aid, plain syntax where lens adds no value).
- [ ] The record/lens design has been considered under name clashes, nested updates, space leaks, evolution, and readability, and remains clear and maintainable.
