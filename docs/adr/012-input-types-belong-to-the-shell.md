# ADR-012: Input Types Belong to the Shell

## Date
2026-03-30

## Status
Accepted

## Supersedes
This ADR revises the assumptions behind ADR-004 and ADR-007 regarding where input-related types live in the codebase.

## Context
The input JSON configuration file (`InputConfiguration`, `Blog`, `Series`, `Article`) exists because the user needs to instruct the program on how to build the blog. This is inherently an I/O concern: the shell reads a file, parses it, resolves paths, reads templates and sources, and then feeds the core with what it needs.

The core does not care about how the input is structured. It only needs template strings, substitution values, and content strings. It returns rendered output. The JSON schema, folder conventions, path resolution, and file reading are all shell responsibilities.

Placing the input types in `src/core/` was a mistake — it coupled the core to the shape of the input file, which is an I/O artifact.

### Key insight

Just because a type does not perform I/O, it does not necessarily mean it belongs to the core. The input types are pure data structures with no side effects, but they exist solely to describe the shape of an external file. Their reason for existing is I/O, so they belong in the shell.

## Decision
The input-related types (`InputConfiguration`, `Blog`, `Series`, `Article`) and their deserialization logic belong in `src/io/`, not `src/core/`.

The core exposes only the minimal interface it needs to do its job: accept template strings and substitution values, return rendered strings. It has no knowledge of JSON, file paths, or the input file structure.

The shell is responsible for:
- Reading and parsing the JSON configuration file.
- Resolving paths and reading templates and article sources from disk.
- Mapping the input types into whatever the core needs.
- Writing the core's output to disk.

## Consequences
- **Purer core**: the core becomes a string transformation engine with no knowledge of the input format.
- **True single-sandwich FCIS**: the shell gathers all data, calls the core once, and writes the results. No multiple handoffs.
- **Shell owns the input schema**: changes to the JSON format only affect the shell, never the core.
- **Core is reusable**: the core could theoretically work with any input format, not just this specific JSON schema.
- **Requires refactoring**: the existing input types need to move from `src/core/` to `src/io/`.
