# ADR-010: Test-Driven Development

## Date
2026-03-30

## Status
Accepted

## Context
Choosing a development methodology affects code quality, design decisions, and confidence in refactoring. The project is both a real tool and a learning exercise (ADR-002), so a disciplined approach helps reinforce good habits.

## Decision
All code is developed using strict Test-Driven Development following the Red-Green-Refactor cycle:

1. **Red** — Write a single failing test.
2. **Green** — Write the minimal code to make that test pass.
3. **Refactor** — Clean up only if needed, ensuring all tests still pass.

No implementation code is written before a failing test exists. Only one test is written at a time.

## Consequences
- **High test coverage by construction**: every piece of logic has at least one test from the moment it is written.
- **Better design**: writing tests first forces thinking about the API before the implementation, leading to cleaner interfaces.
- **Safe refactoring**: a comprehensive test suite makes it safe to restructure code without fear of silent regressions.
- **Slower initial pace**: writing the test before the code adds a step, but this is offset by fewer bugs and less debugging later.
- **Pairs well with functional core (ADR-006)**: pure functions are naturally easy to test, reinforcing both decisions.