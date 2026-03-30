# ADR-007: Folder Structure

## Date
2026-03-30

## Status
Accepted

## Context
ADR-006 established the functional core / imperative shell architecture. The codebase needs a folder structure that makes this separation visible and enforces it by convention.

I also prefer small, focused files — each struct or enum in its own file — over large files that bundle multiple types together.

## Decision
The source code is organized into two top-level modules:

- `src/core/` — the functional core. Pure functions and types. No I/O, no side effects.
- `src/io/` — the imperative shell. All file system access, reading inputs, and writing outputs.

Each struct, enum, or significant type gets its own file.

## Consequences
- **Visible architecture**: the folder names immediately communicate what belongs where.
- **Small, navigable files**: one type per file makes it easy to find things and keeps diffs focused.
- **Convention-based enforcement**: placing I/O code in `src/io/` and pure code in `src/core/` makes violations easy to spot in review.
- **More files to manage**: the trade-off of many small files is more `mod` declarations, but this is minor.