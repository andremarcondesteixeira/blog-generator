# ADR-003: Personal Blog Only

## Date
2026-03-29

## Status
Accepted

## Context
Static site generators range from minimal scripts to highly configurable frameworks. Designing for a general audience introduces significant complexity: themes, plugins, flexible content models, configuration options, and documentation for external users.

This tool exists to generate my personal blog at andre.pro and nothing else.

## Decision
I will only implement features that my blog actually needs. The tool is not intended to support a general audience, and I will not add configurability, extensibility, or abstractions aimed at hypothetical external users.

## Consequences
- **Smaller scope**: features are driven by the concrete requirements of andre.pro, keeping the codebase focused and manageable.
- **Simpler design**: no need for plugin systems, theme engines, or flexible content schemas.
- **Hardcoded assumptions are fine**: I will try to make some stuff configurable, but I am not commited to it. Paths, layouts, and conventions specific to my blog can be baked in rather than made configurable.
- **Not reusable by others out of the box**: anyone else wanting to use this tool would need to fork and adapt it, which is an accepted trade-off.
