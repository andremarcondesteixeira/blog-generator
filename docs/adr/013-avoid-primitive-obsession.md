# ADR-013: Avoid Primitive Obsession

## Date
2026-04-02

## Status
Accepted

## Context
ADR-001 establishes the principle of making invalid states unrepresentable. One common way invalid states sneak in is through **primitive obsession** — using raw types like `String`, `i32`, or `Vec<String>` to represent domain concepts that have their own rules and constraints.

For example, a URL path like `"posts/my-article.html"` is not just any string. It has structure (segments separated by `/`), character restrictions (only alphanumeric, hyphens, underscores, and periods in the last segment), and semantic meaning. Passing it around as a plain `String` means every function that receives it must either trust that the caller validated it, or validate it again.

Wrapping it in a dedicated type like `UrlPath` that validates on construction means the value is guaranteed correct everywhere it appears. The type system enforces the invariant — no re-checking, no trusting callers, no accidental misuse.

## Decision
Avoid primitive obsession. When a value has domain meaning and constraints beyond what its underlying type provides, wrap it in a dedicated type that validates on construction.

Guidelines:
- If a `String` has format rules (e.g., URL paths, slugs, email addresses), create a newtype.
- If a number has range constraints (e.g., must be positive, must be 1–12), create a newtype.
- If two values have the same underlying type but different meanings (e.g., an article title vs. a series name), consider whether a newtype prevents mix-ups.
- The newtype's constructor is the single validation point. Once constructed, the value is guaranteed valid.

## Consequences
- **Stronger compile-time guarantees**: the type system prevents passing the wrong kind of string to the wrong function.
- **Single validation point**: validation happens once at construction. All downstream code can trust the value.
- **Pairs with ADR-001**: newtypes are one of the primary tools for making invalid states unrepresentable.
- **More types to maintain**: each newtype adds a file and a small amount of boilerplate. This is a worthwhile trade-off for the safety gained.
- **Clearer APIs**: function signatures document what kind of value is expected, not just `String`.