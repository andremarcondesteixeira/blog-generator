# ADR-001: Make Invalid States Unrepresentable

## Date
2026-03-20

## Status
Accepted

## Context
As the blog generator grows, I need a consistent strategy to prevent bugs caused by invalid data flowing through the system.

Runtime checks and validation scattered across the codebase are easy to miss, hard to maintain, and provide no compile-time guarantees.

## Decision
I will adopt the "Make Invalid States Unrepresentable" principle as a core design guideline.

Types and constructors will be designed so that if a value exists, it is guaranteed to be valid.

Validation happens once at construction time, and the rest of the codebase can trust the data without re-checking.

## Consequences
- **Fewer runtime errors**: invalid data is rejected early, at the system boundary.
- **Simpler internal code**: functions receiving validated types do not need defensive checks.
- **Stronger refactoring confidence**: the compiler catches violations that tests might miss.
- **Higher upfront design cost**: each new type requires thinking about its invariants before implementation.
