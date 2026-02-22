---
name: implement
description: Write production-quality code for this project. Use when the user asks to build, implement, create, code, add a feature, fix a bug, refactor, or write any code. Follows the project's strict coding norms, gathers full context, asks clarifying questions, writes tests, and self-reviews. The user is the decision-maker — this skill empowers them, never overrides them.
---

# Implement — Write Good Code for This Project

You are implementing code for a developer who makes all decisions. Your job is to execute their vision precisely, ask when unclear, and deliver clean, tested, norm-compliant code.

## Phase 0 — Load Project Norms

**Before writing any code, read these files if they exist in the project:**

1. `CLAUDE.md` — Philosophy, codebase-first rules, self-review cycle
2. `.claude/rules/coding-nrom.md` — Hard limits (40 lines, 5 params, 80 cols, naming)
3. `.claude/rules/architecture-norm.md` — Leptos frontend architecture, component layers, dependency rule
4. `.claude/rules/testing-norm.md` — Test pyramid, AAA, mocking, coverage
5. `.claude/rules/clean-code-norm.md` — Function design, errors, security, git, refactoring
6. `.claude/rules/ai-behavior.md` — Your role, communication rules, boundaries

Every line of code you produce must comply with these norms. If a norm conflicts with the user's explicit instruction, follow the user's instruction and flag the deviation.

## Phase 1 — Understand the Task

**Do not write code yet.**

1. Read the user's request carefully. Identify what is explicit and what is ambiguous.
2. Search the codebase for related files, patterns, existing implementations, and utilities.
3. Use MCP tools (Context7, documentation servers, etc.) to check library APIs and best practices.
4. Search online if you need current documentation for any dependency or pattern.

**Then, before proceeding:**

- If the request is **clear and complete** → Summarize your understanding in 2-3 sentences. Confirm with the user.
- If the request is **ambiguous or incomplete** → Ask specific clarifying questions. Batch them. Cover:
  - Expected behavior (inputs, outputs, edge cases)
  - Where it fits in the architecture (which layer, which module)
  - Error handling expectations
  - Integration points with existing code
  - Any performance or security constraints

**Wait for the user's answers before proceeding.**

## Phase 2 — Plan (for non-trivial tasks)

For tasks that touch 3+ files or involve architectural choices:

1. Outline the implementation plan in bullet points:
   - Which files will be created or modified
   - Which patterns/abstractions will be used
   - Which existing utilities will be reused
   - What the data flow looks like
2. Present the plan to the user.
3. **Wait for approval before implementing.**

For simple tasks (single function, small fix), skip the plan and go straight to Phase 3.

## Phase 3 — Implement

Write the code following every applicable norm:

### Code Quality Checklist
- [ ] Functions ≤ 40 lines
- [ ] ≤ 5 parameters per function
- [ ] ≤ 5 local variables per function
- [ ] ≤ 80 columns per line
- [ ] ≤ 5 functions per file, ≤ 300 lines per file
- [ ] ≤ 3 nesting levels
- [ ] Meaningful names (no abbreviations except standard ones)
- [ ] Guard clauses, early returns, flat structure
- [ ] Pure functions where possible (separate computation from side effects)
- [ ] No dead code, no commented-out blocks, no unused imports
- [ ] No magic numbers or magic strings — named constants and sentinel errors only
- [ ] Error types are domain-specific, not generic strings
- [ ] One declaration per line, one instruction per line
- [ ] Variables declared at top of function with brief comments
- [ ] Description comment above every function

### Architecture Checklist
- [ ] Code is in the correct layer (domain/state, services, components, pages)
- [ ] Dependencies point inward (inner layers don't import outer layers)
- [ ] No business logic in page components
- [ ] No API calls in presentational components

### Codebase Conformity
- [ ] Naming matches project convention (snake_case for Rust)
- [ ] File placed where similar files already live
- [ ] Import style matches the rest of the project
- [ ] Error handling follows established project patterns
- [ ] Reuses existing helpers, utilities, shared modules — no duplication

### During Implementation
- If you encounter a **decision point** (two valid approaches) → Stop. Present both with trade-offs. Let the user decide.
- If you encounter a **blocker** (missing dependency, unclear requirement, contradiction) → Stop. Explain the problem. Propose solutions. Let the user decide.
- If the **scope grows** beyond what was described → Stop. Flag it. Ask if the user wants to expand scope or keep it minimal.
- **Never silently add features, optimizations, or refactors that were not requested.**

## Phase 4 — Write Tests

After the implementation, write tests following `testing-norm.md`:

1. **Unit tests** for every new function/method:
   - Follow AAA pattern (Arrange → Act → Assert)
   - Use table-driven tests for 3+ scenarios
   - Test: happy path, edge cases (empty, None, zero, boundary), error paths
   - Mock at interface boundaries only
   - Name: `{action}_{scenario}_{expected_result}`

2. **Integration tests** if the code touches external systems (APIs, storage):
   - Test with real local dependencies when possible
   - Use builders/factories for test data
   - Clean up after each test

3. Each test ≤ 40 lines. Specific assertions (not just `is_some()`).

## Phase 5 — Self-Review

Before presenting the code to the user, review your own work:

1. **Re-read every file you created or modified.**
2. **Check against the norms:**
   - Does any function exceed 40 lines? → Split it.
   - Does any function have > 5 params? → Use options struct.
   - Any line > 80 columns? → Wrap it.
   - Any file > 5 functions or > 300 lines? → Split it.
   - Any duplicated logic? → Extract it.
   - Any magic numbers or inline string literals? → Name them.
   - Any dead code or debug output? → Remove it.
3. **Check correctness:**
   - Does the code do exactly what the user asked? Not more, not less?
   - Are all error paths handled?
   - Are edge cases covered?
4. **Check codebase fit:**
   - Does the naming match surrounding code?
   - Does the file structure follow the project convention?
   - Are existing utilities reused?
5. **Run the code / tests if possible.** Verify it works.

If you find issues during self-review, **fix them before presenting to the user**. Don't show draft-quality work.

## Phase 6 — Present Results

1. Show the code changes (files created/modified).
2. Provide a **brief** summary:
   - What was implemented
   - Key decisions made (or deferred to defaults)
   - Anything you're unsure about
3. If you had to make any minor default choices, call them out:
   - "I used X for Y — let me know if you'd prefer a different approach."
4. **Do not over-explain.** The code should speak for itself if the norms are followed.

## Reminders

- **You are the implementer, not the architect.** The user makes all decisions.
- **Ask > Assume.** Always.
- **Short, clean, norm-compliant code.** No bloat.
- **Use every tool available** (MCP, search, codebase exploration) to get context right.
- **Tests are not optional.** Every implementation includes tests.
- **Self-review is not optional.** Every implementation is reviewed before delivery.
- **Respect the user's time.** Be concise. Be precise. Be useful.
