---
name: haskell_parsing_and_parser_combinators.md
description: Use when the agent is writing a parser in Haskell (parser combinators with Megaparsec/Attoparsec/Parsec, parsing structured text like JSON/CSV/config/DSLs, writing recursive grammars, handling whitespace and indentation, reporting parse errors with source positions, choosing between parser combinators and parser generators like Happy/Megaparsec), dealing with left recursion, backtracking and try, performance of parsers, or is diagnosing "parser infinite loops / left recursion", "backtracking is slow", "parse error messages are unhelpful / no source position", "whitespace handling is inconsistent", or parser-design issues. Covers parser combinators (Megaparsec/Attoparsec), backtracking and `try`, left recursion, whitespace/indentation handling, error reporting with positions, performance, and combinator-vs-generator choice.
---

# Parsing And Parser Combinators In Haskell

Haskell's parser-combinator libraries (Megaparsec, Attoparsec, the older Parsec) let you build parsers by composing small parsing functions, which is elegant but has sharp edges. Agents write left-recursive grammars that infinite-loop (combinators do not handle left recursion automatically), overuse `try` (backtracking is slow and masks errors), handle whitespace inconsistently (lexing-style `symbol`/`lexeme` combinators avoid this), produce error messages without source positions (useless for debugging), or choose Attoparsec (fast, binary/line-oriented, weak errors) where Megaparsec (good errors, feature-rich) was needed. The judgment problem is to structure the grammar to avoid left recursion, to use backtracking (`try`) sparingly and deliberately, to handle whitespace via lexeme combinators, to report errors with positions and context, and to choose the right library by performance/error needs.

Agents hit infinite loops from left recursion, slow parsers from `try`, confusing errors without positions, or pick the wrong library. The remedy is iterative (not left-recursive) grammars, minimal backtracking, lexeme-based whitespace, positioned errors, and the right library.

## Core Rules

### Structure The Grammar To Avoid Left Recursion

Parser combinators are recursive-descent: a parser `p = do { x <- p; rest }` calls itself before consuming input, infinite-looping. Left-recursive grammars (`expr = expr '+' term`) must be rewritten to iterative/right-recursive form (`expr = term (many (char '+' *> term))`, then fold with `foldl`). Use combinators like `many`/`some`/`sepBy`/`chainl`/`chainr` for repetition and left-associative operators (`chainl` folds left, avoiding explicit left recursion). For operator precedence, use `makeExprParser` (Megaparsec/Expr) which handles precedence/associativity without left recursion. Recognize the symptom: a parser that hangs on valid input is likely left-recursive.

- Rewrite left-recursive rules to iterative form (`many`/`sepBy`/`chainl`/`chainr`).
- `makeExprParser` for operator precedence/associativity (no left recursion).
- Symptom: hang on valid input → left recursion.

### Use Backtracking (try) Sparingly And Deliberately

By default, a combinator that consumes input then fails does not backtrack (the consumed input is lost), so an alternative (`p <|> q`) after `p` partially consumed will fail rather than try `q`. `try p` makes `p` backtrack on failure (no input consumed), enabling `try p <|> q`. But `try` is slow (it may re-parse) and masks errors (a `try` that fails reports no useful error). Use it only where genuinely ambiguous (a prefix shared between alternatives). Structure the grammar to minimize ambiguity (factor common prefixes, use `notFollowedBy` to peek without consuming). In Megaparsec, prefer the explicit `try`-only-where-needed style; in Attoparsec, backtracking is the default (fast but less precise errors).

- `try p` backtracks on failure; use only for genuinely ambiguous alternatives.
- Factor common prefixes and use `notFollowedBy` to reduce the need for `try`.
- `try` is slow and masks errors; minimize it. Attoparsec backtracks by default (fast, weaker errors).

### Handle Whitespace And Indentation Via Lexeme Combinators

Inconsistent whitespace handling (some parsers consume trailing space, others don't) causes subtle bugs. Use the "lexeme" style: `lexeme p = p <* spaces` (a parser followed by its trailing whitespace), and `symbol s = lexeme (string s)`, so every token consumes its trailing whitespace uniformly. Megaparsec's `Text.Megaparsec.Char.Lexer` provides `lexeme`, `symbol`, `space`, `skipLineComment`/`skipBlockComment`, and indentation combinators (`indentLevel`, `indentGuard`) for indentation-sensitive grammars (Python-like). Centralize whitespace in a lexer module; do not scatter raw `spaces` calls. For indentation-sensitive parsing, use `indentLevel`/`indentGuard`/`nonIndented`/`indentBlock` deliberately.

- Lexeme style: `lexeme p = p <* spaces`; `symbol` for tokens; uniform trailing-whitespace consumption.
- Centralize whitespace in a lexer module (Megaparsec `Char.Lexer`); avoid scattered raw `spaces`.
- Indentation-sensitive: `indentLevel`/`indentGuard`/`nonIndented`/`indentBlock`.

### Report Errors With Source Positions And Context

A parse error "expected 'x'" without a position is useless. Megaparsec tracks source positions (line/column) and supports custom error components and error context (`label`/`hidden`/`withSource`/`region`). Use `parseTest`/`runParser` with a source name; enrich errors with `<?>` (a label for the expected thing) and `registerError`/`fancy` for custom errors. For user-facing parsers (a config/DSL), invest in good error messages (position, what was expected, what was found, context). Attoparsec's errors are weaker (less positional) — choose Megaparsec when error quality matters. Do not swallow errors silently; surface them with context.

- Megaparsec tracks positions; use `<?>` labels, custom errors, and `region` for context.
- For user-facing parsers, invest in positioned, contextual error messages.
- Attoparsec errors are weaker; choose Megaparsec when error quality matters.

### Choose The Library By Performance And Error Needs

- **Attoparsec**: fast, designed for binary and line-oriented text (HTTP, JSON, CSV); backtracks by default; weaker error messages. Use for high-throughput parsing where errors are diagnosed elsewhere.
- **Megaparsec**: feature-rich, excellent error messages with positions, good for user-facing parsers (config, DSLs, source languages); `try`-explicit (faster where applicable). Slightly slower than Attoparsec.
- **Parsec**: the predecessor; prefer Megaparsec for new code.
- **Happy** (parser generator, Yacc-like): for complex grammars with precedence/ambiguity where combinators get unwieldy; generates tables, fast, but less Haskell-idiomatic.

Match the library to the workload: Attoparsec for speed/binary, Megaparsec for errors/UX, Happy for complex grammars.

- Attoparsec: fast, binary/line-oriented, weak errors.
- Megaparsec: good errors, feature-rich, for user-facing parsers.
- Happy: parser generator for complex grammars where combinators are unwieldy.

## Common Traps

### Left Recursion Infinite-Looping

`p = p *> q` hangs. Rewrite iteratively (`chainl`/`many`/`makeExprParser`).

### Overusing try (Slow, Masks Errors)

`try` everywhere re-parses and hides errors. Use only for genuine ambiguity; factor prefixes.

### Inconsistent Whitespace Handling

Some tokens consume trailing space, others don't. Use lexeme style uniformly.

### Errors Without Source Positions

"expected x" with no line/column is useless. Use Megaparsec positions and `<?>` labels.

### Attoparsec Where Megaparsec Was Needed

Attoparsec's weak errors hurt user-facing parsers. Choose by error-quality needs.

### Backtracking Default Mismatch

Megaparsec requires explicit `try`; Attoparsec backtracks by default. Know the library's default.

### Slow Parser From Excessive Backtracking

Ambiguous grammar + `try` re-parses exponentially. Disambiguate; use `notFollowedBy`.

### Swallowed Parse Errors

`maybe`/`either` discarding the error. Surface errors with context.

## Self-Check

- [ ] The grammar avoids left recursion (iterative `many`/`sepBy`/`chainl`/`chainr`, or `makeExprParser` for operators); no parser hangs on valid input.
- [ ] `try` is used only for genuinely ambiguous alternatives; common prefixes are factored and `notFollowedBy` reduces backtracking.
- [ ] Whitespace is handled uniformly via lexeme combinators (`lexeme`/`symbol`, centralized in a lexer module); indentation-sensitive parsing uses `indentLevel`/`indentGuard`.
- [ ] Parse errors include source positions (line/column), expected/found, and context (`<?>` labels, custom errors); errors are surfaced, not swallowed.
- [ ] The library matches the workload (Attoparsec for speed/binary, Megaparsec for errors/UX, Happy for complex grammars).
- [ ] The parser's backtracking default (Megaparsec explicit `try` vs Attoparsec default) is understood and applied correctly.
- [ ] Performance is considered (no exponential backtracking; Attoparsec for high-throughput where errors are secondary).
- [ ] The parser has been considered under left recursion, ambiguity, whitespace, error quality, and library choice, and remains correct and maintainable.
