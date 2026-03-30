# ADR-011: Validation Boundary Between Core and Shell

## Date
2026-03-30

## Status
Accepted

## Context
ADR-001 (make invalid states unrepresentable) and ADR-006 (functional core, imperative shell) can conflict. For example, a `Series` can be constructed with an `inputFolder` that does not exist on disk. Validating path existence requires file system access, which belongs in the shell, not the core.

## Decision
Validation is split across the two layers, each responsible for its own domain:

- **Core** enforces structural validity: correct types, formats, non-empty values, and relationships between fields. Anything that can be checked without I/O is rejected at construction time.
- **Shell** enforces environmental validity: file existence, permissions, and readable content. These checks happen at the I/O boundary before data is passed into the core.

ADR-001 applies within each layer's domain. A structurally valid value in the core (e.g., a syntactically correct path) is considered valid even if the path does not point to a real file. The shell is responsible for ensuring that the environment matches what the core expects.

## Consequences
- **Core stays pure**: no I/O sneaks in under the guise of validation.
- **ADR-001 is preserved in spirit**: each layer makes its own category of invalid states unrepresentable.
- **Clear error reporting**: structural errors come from the core, environmental errors come from the shell, making it easy to distinguish between a bad config and a missing file.
- **Requires discipline**: it can be tempting to add an I/O check inside the core for convenience. This ADR makes the boundary explicit.