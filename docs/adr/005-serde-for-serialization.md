# ADR-005: Serde for Serialization and Deserialization

## Date
2026-03-30

## Status
Accepted

## Context
The blog generator needs to parse a JSON configuration file into Rust types. There are multiple ways to handle this, from manual parsing to using a serialization framework.

## Decision
I will use serde (with serde_json) for all serialization and deserialization. It is the de facto standard in the Rust ecosystem, well-documented, and widely adopted.

## Consequences
- **Idiomatic Rust**: serde is what most Rust developers expect to see for this kind of work.
- **Derive macros reduce boilerplate**: `#[derive(Serialize, Deserialize)]` handles most cases without manual parsing code.
- **Rich ecosystem**: serde integrates with many formats and libraries beyond JSON if needed in the future.
- **Additional dependencies**: serde and serde_json are added to the project, but they are lightweight and well-maintained.