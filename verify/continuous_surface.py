"""Audit the public continuous-update surface without running batch oracles."""
from __future__ import annotations

import inspect
import json
from pathlib import Path

import taflow


def main() -> None:
    root_rows = []
    for name in taflow.__all__:
        obj = getattr(taflow, name, None)
        if isinstance(obj, type) and name not in {"MaType"}:
            root_rows.append({
                "name": name,
                "append": callable(getattr(obj, "append", None)),
                "extend": callable(getattr(obj, "extend", None)),
                "reset": callable(getattr(obj, "reset", None)),
                "compute_or_value": callable(getattr(obj, "compute", None)) or hasattr(obj, "value"),
            })
    import taflow._native as native
    talib_one_shot = []
    try:
        import taflow.talib as tt
        talib_one_shot = sorted(name for name in dir(tt) if name.isupper() and callable(getattr(tt, name, None)))
    except ImportError:
        pass
    report = {
        "root_exports": len(taflow.__all__),
        "root_indicators": len(root_rows),
        "root_indicators_with_lifecycle": sum(all(row[k] for k in ("append", "extend", "reset", "compute_or_value")) for row in root_rows),
        "native_one_shot_functions": sorted(name for name in dir(native) if name.isupper() and callable(getattr(native, name, None))),
        "taflow_talib_one_shot_exports": talib_one_shot,
        "rows": root_rows,
    }
    (Path(__file__).parent / "CONTINUOUS_SURFACE.json").write_text(json.dumps(report, indent=2) + "\n")
    lines = [
        "# Continuous-update surface audit", "",
        f"- Root exports: **{report['root_exports']}**",
        f"- Root indicator classes: **{report['root_indicators']}**",
        f"- Root classes with append/extend/reset/compute-or-value: **{report['root_indicators_with_lifecycle']}**",
        f"- Native one-shot functions still compiled: **{len(report['native_one_shot_functions'])}**",
        f"- One-shot functions visible at `taflow.talib`: **{len(talib_one_shot)}**",
        "", "| Layer | Name | Lifecycle |", "|---|---|---|"]
    lines.extend(f"| taflow | `{row['name']}` | {'PASS' if all(row[k] for k in ('append', 'extend', 'reset', 'compute_or_value')) else 'REVIEW'} |" for row in root_rows)
    if talib_one_shot:
        lines += ["", "## One-shot compatibility leakage", "", ", ".join(f"`{name}`" for name in talib_one_shot)]
    (Path(__file__).parent / "CONTINUOUS_SURFACE.md").write_text("\n".join(lines) + "\n")
    print(f"root indicators: {report['root_indicators_with_lifecycle']}/{report['root_indicators']} lifecycle-complete")
    print(f"native one-shot functions: {len(report['native_one_shot_functions'])}; taflow.talib visible: {len(talib_one_shot)}")


if __name__ == "__main__":
    main()
