# Debugging & Fix Documentation (project debugging memory)

Every significant bug/fix/regression gets a record here as `BUG-NNN-short-title.md`, so that when a
similar bug appears months later the agent does NOT start from zero — it can read: symptoms, cause,
what was attempted, what actually fixed it, files involved, what must not change, and which
regression test was added.

## Mandatory record template

```text
Bug:
Symptoms:
Affected Feature:
Root Cause:
Fix:
Files Changed:
Tests Added:
Regression Risks:
Verification:
Related Bugs:
```

## Rules

- No symptom-only patches; find the real root cause (bug-fixing workflow:
  `../memory/04_rules_system.md`).
- When a bug is fixed, add a regression test wherever practical — "the same bug should not be
  allowed to silently return later."
- Canvas bugs: also check `../memory/06_canvas_rules_requirements.md` testing section.

No bugs recorded yet (no code exists — FOUNDATION phase as of 2026-10-11).
