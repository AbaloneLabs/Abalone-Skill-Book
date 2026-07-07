---
name: php_arrays_iteration_and_references.md
description: Use when the agent is working with PHP arrays (indexed, associative, nested), iterating with foreach, using references in foreach (& $value) and noticing the classic dangling-reference bug, passing arrays by reference vs by value, using array_map/array_filter/array_reduce and the index/keys semantics, building large arrays in loops, using list/array destructuring, using the splat operator (...), using generators (yield), handling array mutation and immutability, or is diagnosing "foreach changes the wrong element", "array_map preserves keys and reindexes oddly", "the last foreach value is corrupted", "passing an array did not modify the original", or large-memory array issues. Covers PHP array semantics (ordered maps), the foreach-by-reference dangling reference trap, pass-by-value by default, functional array functions and their key behavior, generators for memory, and the traps of key preservation and reindexing.
---

# Arrays, Iteration, And References In PHP

PHP arrays are the central data structure: they are ordered maps, simultaneously usable as lists, dictionaries, stacks, queues, and sets. This versatility conceals subtle semantics that cause recurring bugs. PHP arrays are passed by *value* by default (a copy is made on assignment and on function call), unlike most languages, which surprises agents expecting reference semantics. The `foreach` loop has a notorious dangling-reference bug when used with `& $value` (the reference survives the loop and a subsequent loop over the same variable corrupts the array). The functional array functions (`array_map`, `array_filter`, `array_walk`) differ in their callback signatures and key-preservation behavior, so using the wrong one or ignoring keys produces silently wrong results. And building large arrays in loops can exhaust memory where a generator would stream. The judgment problem is to choose the right iteration form, understand value-vs-reference semantics, know which array function preserves keys, and reach for generators when memory matters.

Agents default to `foreach` with mutation, hit the dangling-reference bug, misuse `array_map`'s key-preservation (it keeps keys, so "mapping a list" can produce a list with gaps), or build giant arrays where a generator streams. The remedy is to know the semantics precisely and choose deliberately.

## Core Rules

### Know PHP Arrays Are Ordered Maps, Passed By Value

A PHP array is an ordered map (insertion-ordered key→value), usable as a list (sequential integer keys), a dictionary (string keys), or mixed. Two surprises: (1) arrays are passed by *value* — assignment and function calls copy (copy-on-write, so cheap until mutated), so a function that modifies its array parameter does not affect the caller's array unless passed by reference (`&`); (2) integer-like string keys are cast to integers (`"1"` becomes `1`), and `true`/`false`/`null` keys cast too. When you need the caller's array modified, pass by reference explicitly and deliberately, not by accident.

- Arrays are passed by value; modification inside a function does not affect the caller unless passed by reference (`&`).
- Keys are insertion-ordered; integer-like string keys cast to int.
- Use pass-by-reference (`&`) deliberately when the caller's array must change; otherwise return a new array.

### Beware The foreach-By-Reference Dangling Reference

`foreach ($arr as & $value)` iterates by reference, letting you mutate elements in place. But the reference variable (`$value`) *survives the loop* and remains aliased to the last element of the array. A *second* `foreach` over the same array using the same variable name (`foreach ($arr as $value)`) without `unset($value)` after the first loop will, on the last iteration, overwrite the last element with the second-to-last value — a classic silent corruption. Always `unset($value)` immediately after a by-reference foreach, or use a different variable name in the next loop. Better, avoid by-reference foreach entirely and build a new array with `array_map`/a plain foreach.

- After `foreach ($arr as & $value)`, `$value` is still a reference to the last element.
- A subsequent `foreach ($arr as $value)` (same name) corrupts the last element.
- Always `unset($value)` after a by-reference foreach; prefer building a new array over in-place mutation.

### Understand Key-Preservation And Reindexing Across Array Functions

The functional array functions differ in key handling, and ignoring this produces wrong results:

- `array_map(fn, $arr)` preserves keys (a list `[a,b,c]` mapped to `[0=>A, 2=>C]` after filtering elsewhere keeps its keys, producing gaps). To reindex a list, append `array_values(...)`.
- `array_filter($arr, fn)` preserves keys; filtered lists have gaps — wrap in `array_values(...)` to reindex as a sequential list.
- `array_walk($arr, fn)` operates in place (the callback receives `& $value`); it returns bool, not the array, and is for side effects, not transformation.
- `array_map` with multiple arrays uses the longest, and the callback receives one element from each; keys are preserved from the first array.

When you want a clean sequential list after filter/map, always `array_values(...)` to reindex.

- `array_map`/`array_filter` preserve keys → filtered/mapped lists may have gaps; `array_values(...)` to reindex.
- `array_walk` is in-place (side effects), returns bool; do not use it where you want a transformed array.
- `array_map` over multiple arrays aligns by position and preserves the first array's keys.

### Use list/array Destructuring And The Splat Operator Deliberately

PHP supports array destructuring (`[$a, $b] = $arr`, `['x' => $x] = $arr`) and the splat operator (`...$args` to spread an array into function arguments, and `function f(...$args)` to collect variadic arguments). Use destructuring to name array elements and reduce indexing errors; use splat to forward arguments or accept variadic args. Note splat in function *calls* (`f(...$arr)`) requires integer-keyed, sequentially-indexed arrays (string keys are allowed in named-args PHP 8+ but the array must be valid as args).

- Destructure (`[$a,$b]` / `['k'=>$v]`) to name elements and reduce indexing bugs.
- Splat (`...`) spreads an array to args or collects variadic args; the array must be valid as args (sequential int keys, or named args in 8+).

### Use Generators (yield) When Memory Or Laziness Matters

A function using `yield` returns a `Generator`, producing values lazily (one at a time) rather than building the whole array in memory. For large or infinite sequences, file/stream processing, or pipelines, a generator avoids materializing the whole collection. Trade-off: generators are single-pass (you can iterate once) and not indexable; convert to array (`iterator_to_array`) if you need random access or multiple passes. Prefer generators for streaming/large data; arrays for small, reused, or randomly-accessed data.

- `yield` for large/infinite/streaming data (memory-bounded); `iterator_to_array` to materialize when random access is needed.
- Generators are single-pass and not indexable; arrays for small, reused, randomly-accessed data.

## Common Traps

### foreach-By-Reference Corrupting The Last Element

The reference survives the loop; a next foreach overwrites the last element. `unset($value)` after, or build a new array.

### Expecting Function Array Modification To Affect The Caller

Arrays are by value; the caller's array is unchanged. Pass by reference (`&`) deliberately or return a new array.

### array_filter Producing A List With Gaps

Keys are preserved, so `[0,1,2]` filtered to `[0,2]` has keys `0` and `2`. `array_values(...)` to reindex as `[0,1]`.

### Using array_walk Where array_map Was Meant

`array_walk` is in-place, returns bool, callback takes `& $value`. Using it for transformation returns bool, not the array. Use `array_map`.

### array_map Over Multiple Arrays Misaligning

Keys from the first array are preserved; elements align by position. Mismatched sizes pad with nulls.

### Building A Huge Array In A Loop

Materializing millions of rows in an array exhausts memory. Use a generator (`yield`) to stream.

### Integer-Like String Keys Cast Silently

`$arr["1"]` and `$arr[1]` are the same key. Mixing string and int keys that look alike collide.

### Mutating An Array While Iterating It

Adding/removing during foreach can skip or repeat elements. Build a new array or iterate a copy.

## Self-Check

- [ ] Arrays are treated as passed by value; caller modification requires explicit pass-by-reference (`&`) or returning a new array, not assumed.
- [ ] After any `foreach (... as & $value)`, `$value` is `unset()` (or a different variable name used in the next loop) to prevent the dangling-reference corruption.
- [ ] After `array_filter`/`array_map` on a list, `array_values(...)` is applied if a sequential (0-indexed) list is required.
- [ ] `array_walk` is used only for in-place side effects (returns bool); `array_map` is used for transformation (returns array).
- [ ] The key-preservation behavior of the chosen array function is known and accounted for (gaps after filter, first-array keys for multi-array map).
- [ ] Large/streaming data uses generators (`yield`) rather than materializing the whole array; `iterator_to_array` only when random access is needed.
- [ ] Destructuring and splat are used where they reduce indexing errors or forward variadic args, with the array validated as valid args.
- [ ] Integer-like string keys are not assumed distinct from integer keys; iteration does not mutate the array being iterated (a copy or new array is used).
