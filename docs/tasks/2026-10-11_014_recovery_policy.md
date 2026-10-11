# Task 014 — Trap recovery, autosave & document integrity (P0-class policy)

**Date:** 2026-10-11 · **Status:** COMPLETE as a policy decision (docs only; no code by design) · **Governs:** `docs/architecture/ADR-008-recovery-and-integrity.md` (PROPOSED) · `engine-core.md` §8, §11.2 · `file-format.md` §4 · `ADR-001` accepted cost 4

## Goal

Close the P0-class gap that ADR-001 and engine-core §8 require before any risky subsystem ships:
how the app recovers from a wasm trap or panic without losing committed work, without trusting a
dying instance, and without telemetry.

## Decisions (see ADR-008 for the full table, sub-decisions 1–16)

- Recovery unit = committed, validated whole-file snapshot (`gip-document::file::to_json`).
- Snapshots only at commit boundaries (never mid-transaction).
- Core serializes and validates; shell stores in local storage (one atomic write per generation).
- Restore validates every generation; an invalid one falls back to the older one; never fabricated.
- 3 generations; autosave 2 s debounce, 30 s max while dirty [owner review].
- Diagnostics: panic hook → local crash record, user-visible, never transmitted.
- Fault injection: cargo feature `fault-injection`, excluded from release by a CI check.
- Honest loss statement in the recovery copy (undo history and the last seconds of edits).

## Why no code yet

Recovery needs the wasm boundary (`gip-session` panic hook and fault injection) and the shell
storage module. Neither exists yet, because the shell does not import the bindings (engine-core §10).
The policy is recorded now so the first boundary task implements it rather than inventing it.

## Owner review (non-blocking)

1. Autosave cadence (2 s / 30 s). 2. Generations kept (3). 3. Clean-close behaviour when dirty.
4. Recovery copy wording (content designer review).

## Deferred

Storage-quota policy · snapshot compression (ADR-003 defers it) · multi-tab conflicts (own ADR) ·
SHA-256 `documentHash` on generations (waits for the dependency decision).

## Measured

Not applicable (policy). Workspace unchanged: 131 tests green from task 013; no code was modified in this task.
