# Feature Documentation

Every major feature gets its own folder/document here (e.g. `vector-editing/`, `auto-layout/`,
`components/`, `groups/`, `frames/`, `text/`, `responsive-vectors/`).

Each feature doc must explain (as applicable): what the feature is · why it exists · user behavior ·
UI behavior · interaction states · keyboard behavior · mouse/pointer behavior · selection behavior ·
expected canvas behavior · dependencies · related systems · implementation notes · important rules ·
edge cases · known bugs · limitations · test cases · regression risks.

Standard: another agent must be able to understand the feature **without reconstructing it from code**.

The master index of all features lives in `FEATURE_REGISTRY.md` (update it on every feature
addition/removal/status-change/major-behavior-change).

Full requirements: `../memory/03_documentation_system.md` §1–2.
