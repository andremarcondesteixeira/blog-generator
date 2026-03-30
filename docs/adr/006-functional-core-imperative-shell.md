# ADR-006: Functional Core, Imperative Shell

## Date
2026-03-30

## Status
Accepted

## Context
A blog generator's workflow is naturally a pipeline: read inputs, transform data, write outputs. Choosing where I/O lives in the architecture affects testability and code clarity.

## Decision
I will follow the "Functional Core, Imperative Shell" architecture:

- **Functional core**: pure functions that receive data and return data. No file system access, no side effects. This includes parsing the configuration, ordering articles, resolving paths, and rendering HTML.
- **Imperative shell**: a thin layer that performs all I/O — reading the JSON config, reading template and article files, feeding data into the core, and writing the generated HTML to disk.

## Consequences
- **Highly testable core**: the bulk of the logic can be tested with plain inputs and outputs, no filesystem mocking or setup required.
- **Pairs well with TDD**: pure functions are straightforward to test in a red-green-refactor cycle.
- **Clear separation of concerns**: I/O code is isolated and minimal, making it easy to trace where side effects happen.
- **Rigid boundary**: data must be fully loaded before the core can process it, which is fine for a blog generator where inputs are small.