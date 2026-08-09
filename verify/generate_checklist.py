"""Generate a reproducible public-interface coverage inventory.

This script intentionally uses only source introspection and installed reference
registries.  It is safe to run from the independent ``verify`` environment.
"""
from __future__ import annotations

import ast
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYROOT = ROOT / "python" / "taflow"
PYINDICATORS = PYROOT / "indicators"
OUT_JSON = Path(__file__).parent / "function_inventory.json"
OUT_MD = Path(__file__).parent / "FUNCTION_CHECKLIST.md"
CORE = ROOT / "crates" / "taflow-core" / "src" / "indicators"
TESTS = ROOT / "tests"


def public_defs(path: Path) -> list[dict[str, str]]:
    tree = ast.parse(path.read_text())
    native_names: list[str] = []
    for node in tree.body:
        if isinstance(node, ast.ImportFrom) and (
            (node.level >= 1 and node.module == "_native")
            or node.module == "taflow._native"
        ):
            for alias in node.names:
                native_names.append(alias.name)
                if alias.asname:
                    native_names.append(alias.asname)
    out = []
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)) and not node.name.startswith("_"):
            out.append({"name": node.name, "kind": "class" if isinstance(node, ast.ClassDef) else "function", "module": path.stem, "native_symbols": native_names})
    return out


def module_native_symbols(path: Path) -> list[str]:
    """Return all native imports, including dynamically-created public adapters."""
    tree = ast.parse(path.read_text())
    result: list[str] = []
    for node in tree.body:
        if isinstance(node, ast.ImportFrom) and (
            (node.level >= 1 and node.module == "_native")
            or node.module == "taflow._native"
        ):
            result.extend(alias.name for alias in node.names)
    return result


def local_import_modules(path: Path) -> list[str]:
    tree = ast.parse(path.read_text())
    return [node.module.rsplit(".", 1)[-1] for node in tree.body if isinstance(node, ast.ImportFrom) and node.level > 0 and node.module and node.module != "_native"]


def exported_names() -> list[str]:
    tree = ast.parse((PYROOT / "__init__.py").read_text())
    for node in tree.body:
        if isinstance(node, ast.Assign):
            for target in node.targets:
                if isinstance(target, ast.Name) and target.id == "__all__" and isinstance(node.value, (ast.List, ast.Tuple)):
                    return [e.value for e in node.value.elts if isinstance(e, ast.Constant) and isinstance(e.value, str)]
    return []


def export_modules() -> dict[str, str]:
    tree = ast.parse((PYROOT / "__init__.py").read_text())
    result: dict[str, str] = {}
    for node in tree.body:
        if isinstance(node, ast.ImportFrom) and node.level > 0 and node.module:
            module = node.module.rsplit(".", 1)[-1]
            for alias in node.names:
                result[alias.asname or alias.name] = module
    indicator_tree = ast.parse((PYINDICATORS / "__init__.py").read_text())
    indicator_modules: dict[str, str] = {}
    for node in indicator_tree.body:
        if isinstance(node, ast.ImportFrom) and node.level > 0 and node.module:
            for alias in node.names:
                indicator_modules[alias.asname or alias.name] = node.module.rsplit(".", 1)[-1]
    return {
        name: indicator_modules.get(name, module) if module == "indicators" else module
        for name, module in result.items()
    }


def native_functions() -> set[str]:
    """Return public one-shot PyO3 functions.

    TAFlow is continuous-only; the former ``func_api.rs`` batch bindings are
    intentionally not registered by the extension anymore.  Keep this
    inventory explicit so a dead source file cannot make them look public.
    """
    return set()


def native_classes() -> set[str]:
    text = (ROOT / "crates" / "taflow-python" / "src" / "state_api.rs").read_text()
    text += (ROOT / "crates" / "taflow-python" / "src" / "indicators" / "mod.rs").read_text()
    text += (ROOT / "crates" / "taflow-python" / "src" / "lib.rs").read_text()
    names = set(re.findall(r"\b(?:pub struct|pub use [^;]+::)\s*([A-Za-z][A-Za-z0-9_]*)", text))
    names.update(re.findall(r"scalar_state_class!\(\s*([A-Za-z][A-Za-z0-9_]*)", text))
    names.update(re.findall(r"add_class::<(?:indicators|state_api)::([A-Za-z][A-Za-z0-9_]*)", text))
    return names


def rust_exports() -> set[str]:
    text = (CORE / "mod.rs").read_text()
    names = set()
    for line in text.splitlines():
        if line.strip().startswith("pub use "):
            names.update(re.findall(r"\b[A-Za-z][A-Za-z0-9_]*\b", line.split("pub use ", 1)[1]))
    return names


def references() -> tuple[list[str], list[str], list[str]]:
    try:
        import talib
        ta = list(talib.get_functions())
    except Exception:
        ta = []
    try:
        import pandas_ta_classic as pta
        pandas_ta = sorted(name for name in dir(pta) if not name.startswith("_"))
    except Exception:
        pandas_ta = []
    try:
        from smartmoneyconcepts import smc
        smc_functions = sorted(name for name in dir(smc) if not name.startswith("_"))
    except Exception:
        smc_functions = []
    return ta, pandas_ta, smc_functions


def camel_to_snake(name: str) -> str:
    """Derive the required module name from a canonical class name."""
    words = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", words).lower()


def load_correctness() -> dict[str, str]:
    """Summarize independent-oracle verdicts by canonical class."""
    path = Path(__file__).parent / "SOURCE_COMPARISON.json"
    if not path.exists():
        return {}
    grouped: dict[str, list[dict[str, object]]] = {}
    for row in json.loads(path.read_text()):
        grouped.setdefault(str(row["class"]), []).append(row)
    result: dict[str, str] = {}
    for name, rows in grouped.items():
        verdicts = {str(row["verdict"]) for row in rows}
        independent = any(str(row.get("source", "")).lower() != "self" for row in rows)
        if "FAIL" in verdicts:
            result[name] = "FAIL"
        elif "VARIANT" in verdicts:
            result[name] = "VARIANT"
        elif verdicts == {"MATCH"} and independent:
            result[name] = "MATCH"
        else:
            result[name] = "NO ORACLE"
    return result


def load_benchmarks() -> set[str]:
    """Return classes with a correctness-gated, full-size benchmark report."""
    result: set[str] = set()
    for path in (Path(__file__).parent / "benchmark_reports").glob("*.json"):
        try:
            report = json.loads(path.read_text())
            correctness = report["correctness"]
            sizes = set(report["protocol"]["sizes"])
            if (
                correctness["batch_vs_oracle"]["passed"]
                and correctness["continue_vs_batch_bitwise"]
                and {1_000, 10_000, 100_000, 1_000_000}.issubset(sizes)
            ):
                result.add(str(report["canonical_class"]))
        except (KeyError, TypeError, ValueError, json.JSONDecodeError):
            continue
    return result


def implementation_checklist(rows: list[dict[str, object]]) -> tuple[list[str], dict[str, int]]:
    """Build the strict all-indicator TODO checklist required by AGENTS.md."""
    correctness = load_correctness()
    benchmarks = load_benchmarks()
    indicators = [row for row in rows if row["kind"] == "class"]
    lines = [
        "## Canonical implementation TODO",
        "",
        "A row is checked only when the canonical full-name Rust and Python files exist,",
        "both implementation files are test-free and expose no same-named free function,",
        "separate same-named Rust and Python tests exist, an independent source reports",
        "`MATCH`, and a correctness-gated 1K/10K/100K/1M benchmark report exists.",
        "",
        "| Done | Canonical class | Module | Rust | Python | Rust test | Python test | Correctness | Benchmark |",
        "|---|---|---|---|---|---|---|---|---|",
    ]
    counts = {"indicators": len(indicators), "complete": 0, "structure": 0, "correctness": 0, "benchmark": 0}
    for row in indicators:
        name = str(row["name"])
        module = camel_to_snake(name)
        rust_path = CORE / f"{module}.rs"
        python_path = PYINDICATORS / f"{module}.py"
        rust_text = rust_path.read_text() if rust_path.exists() else ""
        python_defs = public_defs(python_path) if python_path.exists() else []
        python_classes = [item["name"] for item in python_defs if item["kind"] == "class"]
        python_functions = [item["name"] for item in python_defs if item["kind"] == "function"]
        rust_ok = bool(
            rust_text
            and re.search(rf"\b(?:pub\s+)?struct\s+{re.escape(name)}\b", rust_text)
            and "#[cfg(test)]" not in rust_text
            and not re.search(rf"(?m)^pub(?:\(crate\))?\s+fn\s+{re.escape(module)}\b", rust_text)
        )
        python_ok = python_classes == [name] and module not in python_functions
        rust_test = (CORE / f"{module}_test.rs").exists()
        python_test = (TESTS / f"{module}_test.py").exists()
        structure = rust_ok and python_ok and rust_test and python_test
        verdict = correctness.get(name, "MISSING")
        benchmark = name in benchmarks
        done = structure and verdict == "MATCH" and benchmark
        counts["structure"] += int(structure)
        counts["correctness"] += int(verdict == "MATCH")
        counts["benchmark"] += int(benchmark)
        counts["complete"] += int(done)
        mark = "x" if done else " "
        yes = "yes"
        no = "TODO"
        lines.append(
            f"| [{mark}] | `{name}` | `{module}` | {yes if rust_ok else no} | "
            f"{yes if python_ok else no} | {yes if rust_test else no} | "
            f"{yes if python_test else no} | {verdict} | {yes if benchmark else no} |"
        )
    return lines, counts


def main() -> None:
    exported = exported_names()
    export_module_map = export_modules()
    adapter_paths = [
        path for path in sorted(PYROOT.rglob("*.py"))
        if path.name != "__init__.py"
    ]
    defs = {d["name"]: d for path in adapter_paths for d in public_defs(path)}
    module_symbols = {path.stem: module_native_symbols(path) for path in adapter_paths}
    module_paths = {path.stem: path for path in adapter_paths}
    for _ in range(len(module_paths)):
        for module, path in module_paths.items():
            for imported in local_import_modules(path):
                module_symbols[module].extend(module_symbols.get(imported, []))
            module_symbols[module] = sorted(set(module_symbols[module]))
    native = native_functions()
    native_state = native_classes()
    rust = rust_exports()
    ta, pandas_ta, smc_functions = references()
    # Canonical names are deliberately not forced to match by spelling.
    rows = []
    for name in exported:
        d = defs.get(name, {"name": name, "kind": "export", "module": export_module_map.get(name, "__init__")})
        d = {**d, "native_symbols": d.get("native_symbols") or module_symbols.get(d.get("module", ""), [])}
        alias = name.upper()
        rows.append({
            **d,
            "exported": True,
            "native_binding": bool(set(d.get("native_symbols", [])) & native_state) or name in native or alias in native,
            "native_symbols": d.get("native_symbols", []),
            "rust_export": name in rust,
            "talib_alias": alias if alias in ta else "",
            "pandas_ta_reference": name.lower() in {x.lower() for x in pandas_ta},
            "smc_reference": name.lower() in {x.lower() for x in smc_functions},
        })
    todo_lines, todo_counts = implementation_checklist(rows)
    inventory = {"counts": {"python_exports": len(rows), "native_functions": len(native), "native_state_classes": len(native_state), "rust_exports": len(rust), "talib_functions": len(ta), "pandas_ta_symbols": len(pandas_ta), "smc_functions": len(smc_functions), **todo_counts}, "rows": rows, "talib_functions": ta, "smc_functions": smc_functions}
    OUT_JSON.write_text(json.dumps(inventory, indent=2, sort_keys=True) + "\n")
    lines = ["# taflow public function checklist", "", "Generated by `python generate_checklist.py`; rerun after API changes.", "", f"- Python exports: **{len(rows)}**", f"- Indicator classes: **{todo_counts['indicators']}**", f"- Fully complete indicators: **{todo_counts['complete']}**", f"- Canonical structure plus separate tests: **{todo_counts['structure']}**", f"- Independent correctness matches: **{todo_counts['correctness']}**", f"- Full-protocol benchmark reports: **{todo_counts['benchmark']}**", f"- Native PyO3 functions: **{len(native)}**", f"- Native state/indicator classes: **{len(native_state)}**", f"- Rust stream exports: **{len(rust)}**", f"- TA-Lib registry: **{len(ta)}**", f"- pandas-ta-classic symbols: **{len(pandas_ta)}**", f"- SmartMoneyConcepts functions: **{len(smc_functions)}**", ""]
    lines.extend(todo_lines)
    lines.extend(["", "## Python exports", "", "| Status | Python export | Kind | Module | Native symbol(s) | Rust/native | TA-Lib alias | pandas-ta reference | SMC reference |", "|---|---|---|---|---|---|---|---|---|"])
    for r in rows:
        status = "implemented" if (r["native_binding"] or r["rust_export"]) else "python-only"
        refs = "yes" if r["pandas_ta_reference"] else "—"
        symbols = ", ".join(f"`{x}`" for x in r.get("native_symbols", [])) or "—"
        lines.append(f"| {status} | `{r['name']}` | {r['kind']} | `{r['module']}` | {symbols} | {'yes' if (r['native_binding'] or r['rust_export']) else '—'} | `{r['talib_alias']}` | {refs} | {'yes' if r['smc_reference'] else '—'} |")
    lines += ["", "## TA compatibility", "", "TA-Lib is an external oracle only. TAFlow exports native-backed CamelCase classes directly from `taflow`; no TA-Lib compatibility package is shipped."]
    OUT_MD.write_text("\n".join(lines) + "\n")
    print(f"wrote {OUT_JSON} and {OUT_MD}: {len(rows)} exports")


if __name__ == "__main__":
    main()
