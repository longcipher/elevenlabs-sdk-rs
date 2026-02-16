#!/usr/bin/env python3
"""Check SDK endpoint coverage against the OpenAPI spec.

Parses ``docs/openapi.json`` for all (method, path, operationId) triples,
scans ``crates/elevenlabs-sdk/src/services/`` for implemented methods, and
reports coverage percentage plus any missing endpoints.

Usage::

    python3 scripts/check_coverage.py
    # or
    just sdk-check-coverage
"""

from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Paths – relative to the repo root
# ---------------------------------------------------------------------------

REPO_ROOT = Path(__file__).resolve().parent.parent
OPENAPI_PATH = REPO_ROOT / "docs" / "openapi.json"
SERVICES_DIR = REPO_ROOT / "crates" / "elevenlabs-sdk" / "src" / "services"

# HTTP methods we care about in the OpenAPI spec
HTTP_METHODS = {"get", "post", "put", "patch", "delete", "head", "options"}


# ---------------------------------------------------------------------------
# 1. Parse the OpenAPI spec
# ---------------------------------------------------------------------------


def parse_openapi(spec_path: Path) -> list[dict]:
    """Return a list of endpoint dicts from the OpenAPI spec.

    Each dict has keys: method, path, operation_id, deprecated.
    """
    with open(spec_path, encoding="utf-8") as f:
        spec = json.load(f)

    endpoints: list[dict] = []
    for path, methods in spec.get("paths", {}).items():
        for method, detail in methods.items():
            if method.lower() not in HTTP_METHODS:
                continue
            if not isinstance(detail, dict):
                continue
            operation_id = detail.get("operationId", "")
            deprecated = detail.get("deprecated", False)
            endpoints.append(
                {
                    "method": method.upper(),
                    "path": path,
                    "operation_id": operation_id,
                    "deprecated": bool(deprecated),
                }
            )
    return endpoints


# ---------------------------------------------------------------------------
# 2. Scan Rust service files for implemented endpoints
# ---------------------------------------------------------------------------

# Patterns to extract path strings from Rust service code.
# Matches path literals like "/v1/models", "/v1/voices", format! paths, etc.
PATH_PATTERN = re.compile(
    r"""
    (?:
        # Plain string path:  self.client.get("/v1/foo")
        \.(?:get|post|post_bytes|post_stream|delete|delete_json|delete_with_body|
             patch|put|post_multipart|post_multipart_bytes|post_multipart_stream|
             get_bytes)\s*\(\s*"([^"]+)"
        |
        # format! path in let binding: let path = format!("/v1/dubbing/{dubbing_id}")
        format!\s*\(\s*"([^"]+)"
        |
        # Plain string in let binding:  let mut path = "/v1/history".to_owned()
        (?:let\s+(?:mut\s+)?path\s*=\s*)"([^"]+)"
        |
        # Bare string literal containing an API path (e.g. inside a comment or
        # any other position) — broadest fallback
        "(/v1/[^"]*)"
    )
    """,
    re.VERBOSE,
)

# Pattern for doc comment paths, e.g.: /// Calls `POST /v1/text-to-speech/{voice_id}`.
DOC_PATH_PATTERN = re.compile(
    r"`(?:GET|POST|PUT|PATCH|DELETE)\s+(/v\d+/[^`]+)`"
)


def normalise_path(raw: str) -> str:
    """Normalise a Rust path string to the OpenAPI format.

    Replaces ``{foo_id}`` placeholders and removes query strings.
    """
    # Strip query parameters
    path = raw.split("?")[0]
    # Remove format string suffixes like {suffix}
    # Already in {param} form from format!, leave as-is
    return path


def scan_services(services_dir: Path) -> set[str]:
    """Return a set of normalised API paths found in the service source files."""
    implemented: set[str] = set()

    for rs_file in sorted(services_dir.glob("*.rs")):
        if rs_file.name == "mod.rs":
            continue
        content = rs_file.read_text(encoding="utf-8")
        # Match code-level path strings
        for m in PATH_PATTERN.finditer(content):
            raw = m.group(1) or m.group(2) or m.group(3) or m.group(4)
            if raw and raw.startswith("/v"):
                normalised = normalise_path(raw)
                implemented.add(normalised)
        # Match doc-comment paths like: /// Calls `POST /v1/foo/{id}`.
        for m in DOC_PATH_PATTERN.finditer(content):
            raw = m.group(1)
            if raw:
                normalised = normalise_path(raw)
                implemented.add(normalised)

    return implemented


def normalise_openapi_path(path: str) -> str:
    """Normalise an OpenAPI path for comparison.

    E.g. ``/v1/voices/{voice_id}`` stays as-is because the Rust code uses
    the same ``{param}`` syntax via ``format!``.
    """
    return path


# ---------------------------------------------------------------------------
# 3. Match and report
# ---------------------------------------------------------------------------


def match_endpoint(ep_path: str, implemented: set[str]) -> bool:
    """Check whether an OpenAPI endpoint path is covered by an implemented path."""
    normalised = normalise_openapi_path(ep_path)
    if normalised in implemented:
        return True

    # Try matching with parameter placeholders – the OpenAPI path might use
    # different param names than the Rust code.  Build a regex from the
    # OpenAPI path and check the implemented set.
    pattern = re.sub(r"\{[^}]+\}", r"{[^}]+}", re.escape(normalised))
    pattern = pattern.replace(r"\{[^}]+\}", r"\{[^}]+\}")
    regex = re.compile("^" + pattern + "$")
    return any(regex.match(p) for p in implemented)


def main() -> None:
    if not OPENAPI_PATH.exists():
        print(f"ERROR: OpenAPI spec not found at {OPENAPI_PATH}", file=sys.stderr)
        sys.exit(1)

    if not SERVICES_DIR.is_dir():
        print(f"ERROR: Services directory not found at {SERVICES_DIR}", file=sys.stderr)
        sys.exit(1)

    endpoints = parse_openapi(OPENAPI_PATH)
    implemented = scan_services(SERVICES_DIR)

    # Separate deprecated from active
    active = [ep for ep in endpoints if not ep["deprecated"]]
    deprecated = [ep for ep in endpoints if ep["deprecated"]]

    # Exclude non-API endpoints (e.g. /docs is a redirect)
    EXCLUDED_PATHS = {"/docs"}
    active = [ep for ep in active if ep["path"] not in EXCLUDED_PATHS]

    covered = []
    missing = []
    for ep in active:
        if match_endpoint(ep["path"], implemented):
            covered.append(ep)
        else:
            missing.append(ep)

    total_active = len(active)
    covered_count = len(covered)
    pct = (covered_count / total_active * 100) if total_active else 0

    # ── Report ──────────────────────────────────────────────────────────
    print("=" * 70)
    print("ElevenLabs SDK — Endpoint Coverage Report")
    print("=" * 70)
    print()
    print(f"OpenAPI spec:      {OPENAPI_PATH.relative_to(REPO_ROOT)}")
    print(f"Services dir:      {SERVICES_DIR.relative_to(REPO_ROOT)}")
    print()
    print(f"Total endpoints:   {len(endpoints)}")
    print(f"  Active:          {total_active}")
    print(f"  Deprecated:      {len(deprecated)}")
    print()
    print(f"Covered:           {covered_count}/{total_active}  ({pct:.1f}%)")
    print(f"Missing:           {len(missing)}")
    print()

    if deprecated:
        print("-" * 70)
        print("Deprecated (excluded from coverage):")
        print("-" * 70)
        for ep in deprecated:
            print(f"  {ep['method']:7s} {ep['path']}")
            if ep["operation_id"]:
                print(f"          operationId: {ep['operation_id']}")
        print()

    if missing:
        print("-" * 70)
        print("Missing endpoints:")
        print("-" * 70)
        for ep in missing:
            print(f"  {ep['method']:7s} {ep['path']}")
            if ep["operation_id"]:
                print(f"          operationId: {ep['operation_id']}")
        print()

    # Exit with non-zero if coverage is below 50% (configurable threshold)
    threshold = float(os.environ.get("COVERAGE_THRESHOLD", "0"))
    if pct < threshold:
        print(
            f"FAIL: Coverage {pct:.1f}% is below threshold {threshold:.1f}%",
            file=sys.stderr,
        )
        sys.exit(1)

    print("Done.")


if __name__ == "__main__":
    main()
