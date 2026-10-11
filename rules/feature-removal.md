# FEATURE REMOVAL RULES — Remove completely, or not at all

> Status: ACTIVE (v1, 2026-10-11). Source: vision doc (`04_rules_system.md`). Removing a feature
> must NOT leave half of that feature scattered around the project.

## RM1 — Before removal (decision gate)

1. Confirm the removal intent and scope with the user when it originates from an agent-side
   proposal (global rule G10); user-requested removals proceed but scope gets confirmed in the
   task summary.
2. Read FEATURE_REGISTRY + feature doc + architecture docs: map every consumer, dependency, and
   document-data footprint. Removals of DEPRECATED features check what replaced them and that
   migration exists.
3. **`.gip` data impact is the critical question:** does any document data belong to this feature?
   Data removal/migration requires an ADR + versioned migration (canvas rules CAN-SER-3) — old
   documents must still load (documented behavior: drop-with-warning / migrate / refuse), never
   silently corrupt.
4. If the feature is entangled with a subsystem others need, extract-and-keep the subsystem,
   remove only the feature surface (refactoring rules apply to the extraction).

## RM2 — The removal sweep (ALL of these, checked — the vision doc's list, binding)

- [ ] Feature code paths and entry points (commands, tools, menu items, toolbar buttons, palette
      entries, shortcut-registry bindings, context-menu items)
- [ ] Dead imports left by the above
- [ ] Dead styles / DS component variants used ONLY by this feature (if a DS component loses its
      last consumer: remove it from the DS reference + docs too — the DS stays honest)
- [ ] Dead tests (tests of removed behavior — NOT tests of shared subsystems the feature used;
      shared coverage stays)
- [ ] Dead documentation: feature doc marked `removed` (kept for history with removal date/reason,
      per registry statuses) — docs of still-existing neighbors updated to drop references
- [ ] Unused dependencies (if this feature was a dependency's only consumer → remove the
      dependency; CQ1.4 works both ways)
- [ ] Obsolete feature flags / experiment toggles guarding the removed path
- [ ] Old compatibility code that existed only for this feature
- [ ] Old state: store slices, UI prefs, transient-state handling belonging to the feature
      (state-management domains)
- [ ] Old event listeners / subscriptions / observers registered for it
- [ ] Unreachable code revealed by the removal (re-run dead-code detection after, not just before)
- [ ] Icons in the icon set whose only consumer was this feature (icons rules IC6 prune)
- [ ] Golden fixtures/benchmarks that only exercised the removed feature (keep any that also
      guard shared subsystems)
- [ ] Serialization schema entries (via the ADR'd migration from RM1.3 — schema cleanup is
      migration-gated, not sweep-immediate)

## RM3 — Verification

- Full test suite green (removals break consumers you didn't know about — that's the point of
  running everything); neighbors from the RM1 map exercised per bug-fixing step 8 discipline.
- Search evidence in the task summary: queries run for the feature's names/IDs/tokens showing
  zero live references (docs/history mentions are fine; code references are not).
- Performance check when the removed feature touched hot paths (it should improve or stay flat;
  a regression means something else was load-bearing — investigate, don't revert blindly).
- Documents containing the feature's data load correctly per the RM1.3 decision (golden-file test
  with a fixture containing legacy data).

## RM4 — Documentation completion

- FEATURE_REGISTRY: status → `removed` with date + one-line reason + pointer to the task summary.
- Feature doc: header marked REMOVED (kept; never deleted — debugging history and intent memory
  depend on it); known-consumers list updated with where functionality moved, if it moved.
- Architecture docs/ADRs: mark superseded decisions (ADR status), update subsystem maps if a
  boundary moved. Rules: if a rule section existed solely for this feature, mark it removed with
  date (rules are edited, not silently hollowed). Memory CHANGELOG + task summary as always.
