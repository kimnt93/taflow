#!/usr/bin/env python3
"""Audit the class-only, one-indicator-per-file TAFlow implementation style."""

from __future__ import annotations

import inspect
import json
import re
from collections import defaultdict
from pathlib import Path

import taflow

from all_interfaces import ARRAYS, SKIP


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
PLACEHOLDERS = (
    "Parameters are documented by the constructor signature",
    "Input values or the aligned result container",
    "Updated state, converted values, or aligned output",
    "updated adapter or output",
)


def normalized_annotation(annotation: object) -> object:
    if isinstance(annotation, str):
        return annotation.strip("'\"")
    return annotation


def main() -> int:
    issues: list[dict[str, str]] = []
    classes_by_file: dict[str, list[str]] = defaultdict(list)
    checked = 0
    for name in dict.fromkeys(taflow.__all__):
        cls = getattr(taflow, name, None)
        if not isinstance(cls, type) or name in SKIP or not hasattr(cls, "append"):
            continue
        checked += 1
        source = inspect.getsourcefile(cls) or "<unknown>"
        relative = str(Path(source).resolve().relative_to(ROOT))
        classes_by_file[relative].append(name)
        signature = inspect.signature(cls)
        for parameter in signature.parameters.values():
            if parameter.name in ARRAYS and parameter.default is not inspect.Parameter.empty:
                issues.append({"class": name, "kind": "optional-series", "detail": parameter.name})
            elif (
                parameter.name not in ARRAYS
                and parameter.default is inspect.Parameter.empty
                and parameter.kind not in (parameter.VAR_POSITIONAL, parameter.VAR_KEYWORD)
            ):
                issues.append({"class": name, "kind": "required-config", "detail": parameter.name})
        for method_name in ("append", "extend", "reset"):
            annotation = normalized_annotation(
                inspect.signature(getattr(cls, method_name)).return_annotation
            )
            if annotation not in (name, cls):
                issues.append({
                    "class": name,
                    "kind": "return-type",
                    "detail": f"{method_name} -> {annotation!r}; expected {name!r}",
                })
        docs = [inspect.getdoc(cls) or ""] + [
            inspect.getdoc(getattr(cls, method, None)) or ""
            for method in ("__init__", "append", "extend", "compute", "value", "reset")
        ]
        hits = sorted({placeholder for doc in docs for placeholder in PLACEHOLDERS if placeholder in doc})
        if hits:
            issues.append({"class": name, "kind": "placeholder-doc", "detail": "; ".join(hits)})

    for source, names in classes_by_file.items():
        unique = sorted(set(names))
        if len(unique) > 1:
            issues.append({
                "class": ", ".join(unique),
                "kind": "python-aggregation-file",
                "detail": source,
            })

    stream = ROOT / "crates/taflow-core/src/stream"
    rust_helpers = {"ActiveZoneList", "SortedRing", "Window", "WindowIter"}
    for path in stream.glob("*.rs"):
        text = path.read_text()
        states = [name for name in re.findall(r"^pub struct ([A-Z][A-Za-z0-9_]*)", text, re.M)
                  if not name.endswith("Value") and name not in rust_helpers]
        if len(states) > 1:
            issues.append({"class": ", ".join(states), "kind": "rust-aggregation-file",
                           "detail": str(path.relative_to(ROOT))})
        for name in states:
            impl_start = re.search(
                rf"^impl(?:\s+[A-Za-z0-9_:<>]+\s+for)?\s+{name}\s*\{{", text, re.M)
            methods = {method for method in ("new", "append", "value", "reset")
                       if impl_start and re.search(rf"(?:pub\s+)?fn {method}\b",
                                                   text[impl_start.start():])}
            missing = sorted({"new", "append", "value", "reset"} - methods)
            if missing:
                issues.append({"class": name, "kind": "rust-lifecycle",
                               "detail": f"{path.relative_to(ROOT)} missing {', '.join(missing)}"})

    mod_text = (stream / "mod.rs").read_text()
    for name in re.findall(r"^pub use [A-Za-z0-9_]+::([a-z_][A-Za-z0-9_]*);", mod_text, re.M):
        if name != "session_flags":
            issues.append({"class": name, "kind": "public-rust-batch-export",
                           "detail": "stream/mod.rs"})

    counts: dict[str, int] = defaultdict(int)
    for issue in issues:
        counts[issue["kind"]] += 1
    payload = {"classes_checked": checked, "counts": dict(sorted(counts.items())), "issues": issues}
    (HERE / "STYLE_AUDIT.json").write_text(json.dumps(payload, indent=2) + "\n")
    lines = [
        "# TAFlow class-style audit",
        "",
        f"Public stateful classes checked: **{checked}** | Issues: **{len(issues)}**",
        "",
    ]
    lines.extend(f"- `{kind}`: **{count}**" for kind, count in sorted(counts.items()))
    lines += ["", "| Class | Issue | Detail |", "|---|---|---|"]
    lines.extend(
        f"| `{issue['class']}` | `{issue['kind']}` | {issue['detail']} |" for issue in issues
    )
    (HERE / "STYLE_AUDIT.md").write_text("\n".join(lines) + "\n")
    print(f"style audit: {checked} classes, {len(issues)} issues")
    for kind, count in sorted(counts.items()):
        print(f"- {kind}: {count}")
    return int(bool(issues))


if __name__ == "__main__":
    raise SystemExit(main())
