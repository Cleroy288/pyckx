# Output Format Rules

Always use one of the formats below. Never add prose, greetings, or filler outside these structures.

---

## Format 1 — Implementation

> Use when writing or modifying code.

```
Demand         : <what was asked>
Files analyzed : <full/path/one.ext>, <full/path/two.ext>, ...

══════════════════════════════════════════════════════════════
  CHANGES
══════════════════════════════════════════════════════════════

  ┌─ full/path/to/file.ext
  │
  ├─ What    : <what was done, one line>
  ├─ Why     : <reason, 1–2 sentences>
  │
  └─ Diff
     ```lang
     - <removed code>
     + <added code>
     ```

  ┌─ full/path/to/another/file.ext
  │
  ├─ What    : <what was done>
  ├─ Why     : <reason>
  │
  └─ Diff
     ```lang
     + <added code>
     ```

══════════════════════════════════════════════════════════════
  SUMMARY : <one sentence — what changed and why it matters>
══════════════════════════════════════════════════════════════
```

---

## Format 2 — Plan

> Use when asked to analyze, plan, or propose before touching any code.

```
Demand   : <what was asked>
Analysis : <what was found in the codebase — 1 to 3 sentences>
Problem  : <root cause or blocker — one line>
Solution : <proposed approach — one to two lines>
Why      : <justification — 1 to 2 sentences>

  ┌─ full/path/to/file.ext  [optional — only if code clarifies the plan]
  │
  │  ```lang
  │  <illustrative snippet or pseudo-code>
  │  ```
  │
  └─────────────────────────────────────────────────────────
```

> Code in a plan is illustrative, not final. Repeat the block for each relevant file.

---

## Format 3 — Question

> Use when asked a direct question, technical or otherwise.

```
Demand  : <the question asked>

  ┌─ Sources  [only if external context was fetched]
  │  Jira      : <ticket id + title> — <url>
  │  Bitbucket : <repo/branch or PR title> — <url>
  │  File      : full/path/to/relevant/file.ext
  │
  ├─ Answer
  │  <direct answer — no preamble, no fluff>
  │
  ├─ Context  [optional — only if needed to understand the answer]
  │  <background or nuance, 2–3 sentences max>
  │
  ├─ Example  [optional — only if a snippet makes it clearer]
  │  ```lang
  │  <minimal illustrative code>
  │  ```
  │
  └─ Watch out  [optional — only if there's a real gotcha worth flagging]
     <one line warning or caveat>
```

> Optional sections are omitted entirely when not needed.
> A simple factual question gets only `Demand + Answer`.

---

## Multiple Approaches

When two valid solutions exist, present them before committing:

```
  ┌─ Option A : <label>
  │  <one line description>
  │  Trade-off : <what you gain / what you lose>
  │
  ├─ Option B : <label>
  │  <one line description>
  │  Trade-off : <what you gain / what you lose>
  │
  └─ Chosen   : Option A — <one line reason>
```

Then continue with the standard format.

---

## Rules

- Full paths always — `internal/handler/auth.go`, never just `auth.go` — this applies everywhere: diffs, answers, context, examples, source lists
- Diffs show only the relevant lines, never the entire file
- Use the correct language tag on every code block (`go`, `ts`, `svelte`, etc.)
- Explanations are short and technical — no padding, no hedging
- Plans never contain final code unless explicitly asked
- Changes never contain prose outside the structured format
- Out-of-scope changes must be flagged — see `behavior.md`
