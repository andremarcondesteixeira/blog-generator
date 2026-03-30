# ADR-008: No Leaking Third-Party Types in Public APIs

## Date
2026-03-30

## Status
Accepted

## Context
When public structs and function signatures directly use types from third-party crates, the project's API becomes coupled to those dependencies. Upgrading, replacing, or removing a dependency then forces changes across every consumer of the affected types.

## Decision
Public types must never expose types from third-party dependencies. Instead, third-party types are wrapped in custom project-owned types that expose only the functionality the project needs.

## Consequences
- **Dependency changes are localized**: swapping or upgrading a dependency only requires changes inside the wrapper, not across the codebase.
- **Minimal surface area**: wrappers expose only what the project actually uses, keeping the API focused.
- **Additional boilerplate**: each wrapped dependency requires a custom type with forwarded functionality.
- **Clearer ownership**: the project controls its own type signatures and can evolve them independently of upstream changes.