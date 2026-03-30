# ADR-009: Chrono Wrapped in a Custom Date Type

## Date
2026-03-30

## Status
Accepted

## Context
The blog generator needs date handling for article publication and update dates. The `chrono` crate is the most widely used date/time library in the Rust ecosystem and integrates well with serde for deserialization from JSON.

However, per ADR-008, third-party types must not leak into public APIs.

## Decision
I will use `chrono` internally, wrapped inside a custom `Date` struct that lives in `src/core/date.rs`. The `Date` type exposes only the functionality the blog generator needs:

- Construction from year, month, and day components.
- Display as an ISO 8601 date string.
- Ordering for sorting articles by date.
- Deserialization from JSON via serde.

The `chrono::NaiveDate` type is never exposed outside of the `Date` module.

## Consequences
- **Chrono is an implementation detail**: it can be replaced without affecting any code outside `date.rs`.
- **Focused API**: `Date` only supports what the blog generator actually needs, not the full surface area of chrono.
- **Easy to test**: the custom type has its own unit tests independent of the rest of the codebase.