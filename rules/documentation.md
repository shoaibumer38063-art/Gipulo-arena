# DOCUMENTATION RULES — The documentation system is engineering infrastructure

> Status: ACTIVE (v1, 2026-10-11). Source: vision doc (`03_documentation_system.md` is the full
> spec). Documentation is part of the architecture; a task without its documentation updates is
> an incomplete task.

## D1 — Structure (canonical locations)

```
docs/source/     verbatim user documents — READ-ONLY, ultimate truth
docs/memory/     distilled agent memory — kept in sync, CHANGELOG-disciplined
docs/features/   per-feature docs + FEATURE_REGISTRY.md (master index of ALL features)
docs/architecture/  core-system docs + ADRs (ADR-NNN-title.md)
docs/bugs/       BUG-NNN records (mandatory template, see D4)
docs/prompts/    user prompt archive (index header, see D5)
docs/tasks/      task/decision summaries (template, see D6)
rules/           the rules system (this file's family)
```

## D2 — The Update Rule (runs at the end of EVERY meaningful task)

Answer all ten; update every "yes" in the same task:
feature changed? · behavior changed? · architecture changed? · bug fixed? · regression test
added? · rule changed? · file structure changed? · registry needs update? · task summary due? ·
prompt worth archiving?
Update ONLY affected documents — no ceremonial rewrites, no stale-and-rewrite-everything either.

## D3 — Feature docs & registry

- Every major feature gets a doc detailed enough that another agent understands the feature
  WITHOUT reconstructing it from code: what/why · user & UI behavior · interaction states ·
  keyboard/pointer behavior · selection behavior · canvas behavior · dependencies · related
  systems · implementation notes · rules that bind it · edge cases · known bugs · limitations ·
  test cases · regression risks.
- FEATURE_REGISTRY.md is the single master index (completed/partial/planned/experimental/
  deprecated/removed + doc pointer + code location). Updated whenever a feature is added,
  removed, changes status, or changes major behavior. Agents read it BEFORE building anything
  (feature-addition rules step 1).

## D4 — Bug records (mandatory template)

```text
Bug: / Symptoms: / Affected Feature: / Root Cause: / Fix: / Files Changed: / Tests Added:
/ Regression Risks: / Verification: / Related Bugs:
```
Plus: reproduction fixture committed, regression test linked (testing rules T3). The debugging
history is searchable project memory — write it so the next agent never starts from zero.

## D5 — Prompt archive

Important user prompts archived verbatim with index header: Date · Task ID · Feature · Subsystem ·
Related Bug · Keywords · Related Documentation. The archive preserves original INTENT (a prompt
may contain details the implementation doesn't fully show). It supplements — never replaces —
proper docs.

## D6 — Task summaries

After every important task, save the structured summary: problem · what was inspected · approach
selected · why · alternatives rejected · files changed · behavior changed · verification ·
remaining risks · next steps if the problem returns. Reasoning context YES; private raw
chain-of-thought NO.

## D7 — ADRs

Any important architecture decision (including every [PROPOSED] value in the rules becoming
confirmed/changed) gets an ADR: Decision · Date · Status · Context · Why · Alternatives ·
Consequences. ADRs prevent future agents from replacing systems they don't understand. Superseded
ADRs are kept, marked, and linked — never deleted.

## D8 — Writing standards

- Concrete over generic: docs state exact behaviors, values, file paths, rule IDs (CAN-*, UI*,
  CQ*, …). "Handles edge cases appropriately" is banned prose.
- Tagged epistemics in research-derived content: [PUBLIC]/[OBS]/[REC]/[PROPOSED]/[INF]
  (canvas rules §0). Assumptions never masquerade as facts.
- One topic per doc; cross-reference by rule ID/file path, not by copying content (copying =
  duplication = drift; CQ1 applies to docs too).
- Docs are updated in the SAME task as the change — "documentation later" is how documentation
  dies. Stale doc discovered incidentally ⇒ fix it or file it, never ignore silently.

## D9 — Memory maintenance

- Distilled memory (`docs/memory/`) changes ⇒ append to `docs/memory/CHANGELOG.md` (date, what,
  why) and verify the master beacon (`GIPULO_AGENT_MEMORY.md`) still matches reality.
- Sources-win rule: any conflict between distillation and `docs/source/` is resolved in favor of
  the source, in the same task, with a changelog entry.
- New user documents arriving later: add verbatim to `docs/source/`, distill into memory, note
  supersession relationships — the ingestion pattern established 2026-10-11 (task summary 001).
