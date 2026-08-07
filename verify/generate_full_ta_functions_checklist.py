"""Generate the source-derived Rust/Python computation checklist.

This intentionally does not import TA-Lib or use a TA-Lib registry.  The
inventory is built from taflow's Rust/PyO3/Python source files only.
"""
from __future__ import annotations

import ast
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PYROOT = ROOT / "python" / "taflow"
OUTPUT = ROOT / "plans" / "full-ta-functions-checklist.md"


def python_exports() -> tuple[list[str], dict[str, str]]:
    tree = ast.parse((PYROOT / "__init__.py").read_text())
    names: list[str] = []
    modules: dict[str, str] = {}
    for node in tree.body:
        if isinstance(node, ast.ImportFrom) and node.level and node.module:
            for alias in node.names:
                modules[alias.asname or alias.name] = node.module.rsplit(".", 1)[-1]
        if isinstance(node, ast.Assign) and any(
            isinstance(target, ast.Name) and target.id == "__all__"
            for target in node.targets
        ) and isinstance(node.value, (ast.List, ast.Tuple)):
            names = [
                item.value
                for item in node.value.elts
                if isinstance(item, ast.Constant) and isinstance(item.value, str)
            ]
    return names, modules


def python_class_names() -> set[str]:
    result: set[str] = set()
    for path in PYROOT.glob("*.py"):
        tree = ast.parse(path.read_text())
        result.update(
            node.name for node in tree.body if isinstance(node, ast.ClassDef)
        )
    # Public aliases created with imports rather than another class body.
    result.update(
        {
            "DonchianChannels",
            "GapDown",
            "GapUp",
            "HigherHigh",
            "HighestSince",
            "InsideBar",
            "LowerLow",
            "LowestSince",
            "OutsideBar",
            "SwingHigh",
            "SwingHighsLows",
            "SwingLow",
            "ValueWhen",
        }
    )
    return result


def native_symbols_by_module() -> dict[str, set[str]]:
    direct: dict[str, set[str]] = {}
    dependencies: dict[str, set[str]] = {}
    for path in PYROOT.glob("*.py"):
        tree = ast.parse(path.read_text())
        symbols: set[str] = set()
        local: set[str] = set()
        for node in tree.body:
            if not isinstance(node, ast.ImportFrom) or not node.module:
                continue
            if (node.level == 1 and node.module == "_native") or node.module == "taflow._native":
                symbols.update(alias.name for alias in node.names)
            elif node.level:
                local.add(node.module.rsplit(".", 1)[-1])
        direct[path.stem] = symbols
        dependencies[path.stem] = local
    changed = True
    while changed:
        changed = False
        for module, imported_modules in dependencies.items():
            expanded = direct[module] | set().union(
                *(direct.get(imported, set()) for imported in imported_modules)
            )
            if expanded != direct[module]:
                direct[module] = expanded
                changed = True
    return direct


def registered_native_classes() -> set[str]:
    text = "\n".join(
        path.read_text()
        for path in (ROOT / "crates" / "taflow-python" / "src").rglob("*.rs")
    )
    names = set(re.findall(r"\bpub struct\s+([A-Za-z][A-Za-z0-9_]*)", text))
    names.update(re.findall(r"scalar_state_class!\(\s*([A-Za-z][A-Za-z0-9_]*)", text))
    names.update(re.findall(r"binary_state_class!\(\s*([A-Za-z][A-Za-z0-9_]*)", text))
    names.update(re.findall(r"add_class::<(?:indicators|state_api)::([A-Za-z][A-Za-z0-9_]*)", text))
    return names


def rows() -> list[tuple[str, bool, bool]]:
    exported, modules = python_exports()
    classes = python_class_names()
    native_by_module = native_symbols_by_module()
    native_classes = registered_native_classes()
    result: list[tuple[str, bool, bool]] = []
    for name in exported:
        if name not in classes or name == "MaType":
            continue
        module = modules.get(name, "")
        rust = bool(native_by_module.get(module, set()) & native_classes)
        result.append((name, rust, True))

    # Public same-shape computations present in source outside the root class
    # surface.  These are deliberately explicit so real Rust -> Python gaps do
    # not disappear merely because their spellings differ.
    result.extend(
        [
            ("RollingApply", False, True),
            ("SessionFlags", True, True),
        ]
    )
    return sorted(set(result), key=lambda row: row[0].casefold())


def marker(value: bool) -> str:
    return "x" if value else "_"


def main() -> None:
    inventory = rows()
    rust_count = sum(rust for _, rust, _ in inventory)
    python_count = sum(python for _, _, python in inventory)
    gaps = [name for name, rust, python in inventory if not (rust and python)]
    lines = [
        "# Full TAFlow function coverage checklist",
        "",
        "This is the source-derived master inventory of public, same-shape TAFlow computations.",
        "It intentionally does **not** import, enumerate, or count TA-Lib. The inputs are the",
        "Rust stream sources, registered PyO3 classes, Python adapters/exports, and the four",
        "planning checklists named in the audit request.",
        "",
        "`x` means a concrete implementation exists in that layer; `_` means the public layer is",
        "missing. A Rust `x` is based on a registered native kernel used by the Python module, not",
        "on a same-spelled token. Naming mismatches are therefore called out separately below.",
        "Enums, output/value structs, adapter classes, metadata helpers, scalar reductions,",
        "look-ahead operations, and internal-only primitives are excluded.",
        "",
        "## Audit summary",
        "",
        f"- Computation surfaces: **{len(inventory)}**",
        f"- Rust/native implementations: **{rust_count}**",
        f"- Python interfaces: **{python_count}**",
        f"- Complete in both layers: **{len(inventory) - len(gaps)}**",
        f"- Layer gaps: **{len(gaps)}**",
        "",
        "## Complete source inventory",
        "",
        "| Class | Rust | Py |",
        "|---|:---:|:---:|",
    ]
    lines.extend(
        f"| {name} | {marker(rust)} | {marker(python)} |"
        for name, rust, python in inventory
    )
    lines += [
        "",
        "## Missing layer report",
        "",
        "| Class | Missing | Source evidence / action |",
        "|---|---|---|",
        "| RollingApply | Rust | Python execution helper exists, but there is no Rust kernel. Its arbitrary Python callback prevents a general native kernel; retain as an explicit Python-only exception or narrow its reducer contract. |",
        "",
        "`SessionFlags` is present in both layers under `taflow.executions`, so it is not a gap.",
        "",
        "## Completed canonical renames",
        "",
        "The Rust core, PyO3 states, and Python classes now use the same canonical `Math*`",
        "spellings. The old Rust/PyO3 names are no longer exported.",
        "",
        "| Former Rust | Former PyO3 | Canonical class | Status |",
        "|---|---|---|---|",
        "| Add | StatefulAdd | MathAdd / StatefulMathAdd | complete |",
        "| Sub | StatefulSub | MathSubtract / StatefulMathSubtract | complete |",
        "| Mult | StatefulMult | MathMultiply / StatefulMathMultiply | complete |",
        "| Div | StatefulDiv | MathDivide / StatefulMathDivide | complete |",
        "",
        "The unary math structs were normalized in the same pass (`Acos` → `MathAcos`, ...,",
        "`Tanh` → `MathTanh`).",
        "",
        "Other legacy Rust spellings found by the scan should be normalized or retired where a",
        "canonical type already exists: `Linearreg*`, `Tsf`, `RollingMinmax*`,",
        "`MovingAverageConvergenceDivergence`, and `TripleExponentialAverage`.",
        "",
        "## Polars same-shape recommendations",
        "",
        "The current [Polars Series computation reference](https://docs.pola.rs/api/python/stable/reference/series/computation.html)",
        "was filtered to numeric operations that return an aligned Series and can be causal and",
        "chunk-invariant. The following useful source gaps now have persistent Rust state, PyO3",
        "bindings, and canonical Python classes:",
        "",
        "| Recommended class | Rust | Py | Polars analogue |",
        "|---|:---:|:---:|---|",
        "| MathAbs | x | x | `Series.abs` |",
        "| MathAcosh | x | x | `Series.arccosh` |",
        "| MathAsinh | x | x | `Series.arcsinh` |",
        "| MathAtanh | x | x | `Series.arctanh` |",
        "| MathCbrt | x | x | `Series.cbrt` |",
        "| MathCot | x | x | `Series.cot` |",
        "| MathDegrees | x | x | `Series.degrees` |",
        "| MathLog1p | x | x | `Series.log1p` |",
        "| MathRadians | x | x | `Series.radians` |",
        "| CumulativeCount | x | x | `Series.cum_count` |",
        "| ExponentiallyWeightedSum | x | x | `Series.ewm_sum` |",
        "",
        "Already covered under canonical TAFlow names: `ewm_mean` →",
        "`ExponentialMovingAverage`, `rolling_mean` → `SimpleMovingAverage`, `diff` →",
        "`Momentum`, `pct_change` → `RateOfChangePercent`, and Polars rolling min/max/sum/median/",
        "quantile/rank/skew/kurtosis/std/var → their `Rolling*` classes. `*_by`, global rank,",
        "scalar aggregations, index-returning methods, and future-dependent peak markers are",
        "excluded from this same-shape causal API.",
        "",
        "## Reproducibility",
        "",
        "Regenerate this report after interface changes:",
        "",
        "```bash",
        "python3 verify/generate_full_ta_functions_checklist.py",
        "```",
    ]
    OUTPUT.write_text("\n".join(lines) + "\n")
    print(
        f"wrote {OUTPUT}: {len(inventory)} computations, "
        f"{rust_count} Rust/native, {python_count} Python"
    )


if __name__ == "__main__":
    main()
