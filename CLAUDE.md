# Claude Code Rules

## Workflow: TDD Partnership

We follow strict **Red-Green-Refactor** (TDD), one step at a time.

### The Cycle
1. **Red** — Write a single failing test. Stop. Show the user the failing output.
2. **Green** — Write the minimal code to make that test pass. Nothing more.We
3. **Refactor** — Clean up only if needed. All tests must still pass.
4. Repeat for the next small increment.

### Rules
- Never write implementation code before a failing test exists.
- Never write more than one test at a time.
- Never skip steps or jump ahead, even if the next step seems obvious.
- Always run the tests and show the output at each step.
- Ask the user before moving to the next step — they drive the pace.
- Make invalid states unrepresentable
- To increase the clarity of the codebase, prefer using long, descriptive names instead of short, concise ones
- Write doc comments for everything

## Git

- Never commit on behalf of the user. The user handles all git commits themselves.