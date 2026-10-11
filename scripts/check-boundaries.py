#!/usr/bin/env python3
"""Gipulo dependency-law checker (docs/architecture/dependencies.md).

Enforces the ADR-001 / engine-core.md crate boundaries:

  * the synthetic dependency graph of `core/crates/*` must match the allowed edges;
  * external dependencies are allowed ONLY where the registry says so
    (least-privilege: layer-1 crates have zero external deps);
  * all nine decided crates must exist (ADR-001 §Consequences).

Run from anywhere:   python3 scripts/check-boundaries.py [repo_root]
Exit codes: 0 = PASS, 1 = violations (printed), 2 = usage/setup error.
"""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path

# --- The law (keep in sync with docs/architecture/dependencies.md) -------------------------

ALL_CRATES = [
    "gip-document",
    "gip-coords",
    "gip-geometry",
    "gip-layout",
    "gip-text",
    "gip-history",
    "gip-scene",
    "gip-render",
    "gip-session",
]

ALLOWED_INTERNAL: dict[str, set[str]] = {
    "gip-document": set(),
    "gip-coords": set(),
    "gip-geometry": {"gip-document", "gip-coords"},
    "gip-layout": {"gip-document", "gip-geometry", "gip-coords"},
    "gip-text": {"gip-document", "gip-geometry", "gip-coords"},
    "gip-history": {"gip-document"},
    "gip-scene": {"gip-document", "gip-geometry", "gip-coords"},
    "gip-render": {"gip-document", "gip-geometry", "gip-coords", "gip-scene"},
    "gip-session": {c for c in ALL_CRATES if c != "gip-session"},
}

# External deps: crate -> allowed names with justification refs (registry doc §3).
ALLOWED_EXTERNAL: dict[str, set[str]] = {
    "gip-render": {"wgpu"},          # R1 / ADR-001: the graphics abstraction itself.
    "gip-session": {"wasm-bindgen"}, # engine-core.md §3: the boundary transport.
    "gip-document": {"serde", "serde_json", "sha2", "unicode-normalization"},  # ADR-003 + ADR-009: schema serialization, documentHash, NFC —
    # compile-time derive + pure functions only; the document phase imports these. This is the
    # ONE deliberate exception to layer-1 zero-external-deps; any further exception needs its
    # own registered decision (registry §5).
}

# Crate names that are core crates (prefix rule catches typos like `gip-doc`).
CORE_PREFIX = "gip-"


def deps_of(manifest: dict) -> list[str]:
    """All dependency names from dependencies / dev-dependencies / target.* sections."""
    names: list[str] = []

    def collect(table: dict | None) -> None:
        if isinstance(table, dict):
            names.extend(table.keys())

    collect(manifest.get("dependencies"))
    collect(manifest.get("dev-dependencies"))
    collect(manifest.get("build-dependencies"))
    for target in manifest.get("target", {}).values():
        if isinstance(target, dict):
            collect(target.get("dependencies"))
            collect(target.get("dev-dependencies"))
    return names


# WRITE DOOR (ADR-004 A6): the op vocabulary and `apply` live in gip-document::edit. Only these
# crates may name that module; everything else must mutate documents through gip-history::transact.
WRITE_DOOR_USERS = {"gip-document", "gip-history"}


def write_door_violations(crates_dir: Path) -> list[str]:
    problems: list[str] = []
    for crate_dir in sorted(p for p in crates_dir.iterdir() if p.is_dir()):
        if crate_dir.name in WRITE_DOOR_USERS:
            continue
        for source in sorted(crate_dir.rglob("*.rs")):
            text = source.read_text(encoding="utf-8")
            if "gip_document::edit" in text or "edit::" in text and "gip_document" in text:
                problems.append(
                    f"{source.relative_to(crates_dir.parent.parent)}: uses the write door "
                    f"(gip_document::edit); only gip-history may mutate documents (ADR-004 A6)"
                )
    return problems


def check(repo_root: Path) -> list[str]:
    problems: list[str] = []
    crates_dir = repo_root / "core" / "crates"

    if not crates_dir.is_dir():
        return [f"missing directory: {crates_dir}"]

    present = sorted(p.name for p in crates_dir.iterdir() if p.is_dir())
    for required in ALL_CRATES:
        if required not in present:
            problems.append(f"required crate missing: core/crates/{required}")

    for crate in present:
        manifest_path = crates_dir / crate / "Cargo.toml"
        if not manifest_path.is_file():
            problems.append(f"{crate}: no Cargo.toml")
            continue
        with manifest_path.open("rb") as fh:
            manifest = tomllib.load(fh)

        for dep in deps_of(manifest):
            if dep == crate:
                problems.append(f"{crate}: depends on itself")
                continue
            if dep.startswith(CORE_PREFIX) and dep not in ALL_CRATES:
                problems.append(f"{crate}: unknown core crate dependency `{dep}` (typo?)")
                continue
            if dep in ALL_CRATES:
                if dep not in ALLOWED_INTERNAL.get(crate, set()):
                    problems.append(
                        f"{crate}: illegal internal dependency `{dep}` "
                        f"(allowed: {sorted(ALLOWED_INTERNAL.get(crate, set())) or 'none'})"
                    )
                continue
            if dep not in ALLOWED_EXTERNAL.get(crate, set()):
                problems.append(
                    f"{crate}: external dependency `{dep}` not in the dependency registry "
                    f"(docs/architecture/dependencies.md §3; CQ1.4)"
                )

    problems.extend(write_door_violations(crates_dir))
    return problems


def main() -> int:
    if len(sys.argv) > 2:
        print(__doc__)
        return 2
    repo_root = Path(sys.argv[1]).resolve() if len(sys.argv) == 2 else Path(__file__).resolve().parents[1]

    problems = check(repo_root)
    if problems:
        print("BOUNDARY CHECK FAILED — the dependency law is violated:")
        for problem in problems:
            print(f"  ✗ {problem}")
        return 1

    print("BOUNDARY CHECK PASSED")
    print(f"  repo root: {repo_root}")
    print(f"  crates:    {len(ALL_CRATES)} (internal edges restricted; external deps least-privilege)")
    for crate in ALL_CRATES:
        internal = sorted(ALLOWED_INTERNAL[crate])
        external = sorted(ALLOWED_EXTERNAL.get(crate, set()))
        print(f"    {crate}: internal={internal or '[]'} external={external or '[]'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
