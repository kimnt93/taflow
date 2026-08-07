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
OUT_JSON = Path(__file__).parent / "function_inventory.json"
OUT_MD = Path(__file__).parent / "FUNCTION_CHECKLIST.md"


def public_defs(path: Path) -> list[dict[str, str]]:
    tree = ast.parse(path.read_text())
    native_names: list[str] = []
    for node in tree.body:
        if isinstance(node, ast.ImportFrom) and ((node.level == 1 and node.module == "_native") or node.module == "taflow._native"):
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
        if isinstance(node, ast.ImportFrom) and ((node.level == 1 and node.module == "_native") or node.module == "taflow._native"):
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
    return result


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
    text = (ROOT / "crates" / "taflow-core" / "src" / "stream" / "mod.rs").read_text()
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


def main() -> None:
    exported = exported_names()
    export_module_map = export_modules()
    defs = {d["name"]: d for p in sorted(PYROOT.glob("*.py")) if p.name != "__init__.py" for d in public_defs(p)}
    module_symbols = {p.stem: module_native_symbols(p) for p in PYROOT.glob("*.py")}
    module_paths = {p.stem: p for p in PYROOT.glob("*.py")}
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
    inventory = {"counts": {"python_exports": len(rows), "native_functions": len(native), "native_state_classes": len(native_state), "rust_exports": len(rust), "talib_functions": len(ta), "pandas_ta_symbols": len(pandas_ta), "smc_functions": len(smc_functions)}, "rows": rows, "talib_functions": ta, "smc_functions": smc_functions}
    OUT_JSON.write_text(json.dumps(inventory, indent=2, sort_keys=True) + "\n")
    lines = ["# taflow public function checklist", "", "Generated by `python generate_checklist.py`; rerun after API changes.", "", f"- Python exports: **{len(rows)}**", f"- Native PyO3 functions: **{len(native)}**", f"- Native state/indicator classes: **{len(native_state)}**", f"- Rust stream exports: **{len(rust)}**", f"- TA-Lib registry: **{len(ta)}**", f"- pandas-ta-classic symbols: **{len(pandas_ta)}**", f"- SmartMoneyConcepts functions: **{len(smc_functions)}**", "", "## Python exports", "", "| Status | Python export | Kind | Module | Native symbol(s) | Rust/native | TA-Lib alias | pandas-ta reference | SMC reference |", "|---|---|---|---|---|---|---|---|---|"]
    for r in rows:
        status = "implemented" if (r["native_binding"] or r["rust_export"]) else "python-only"
        refs = "yes" if r["pandas_ta_reference"] else "—"
        symbols = ", ".join(f"`{x}`" for x in r.get("native_symbols", [])) or "—"
        lines.append(f"| {status} | `{r['name']}` | {r['kind']} | `{r['module']}` | {symbols} | {'yes' if (r['native_binding'] or r['rust_export']) else '—'} | `{r['talib_alias']}` | {refs} | {'yes' if r['smc_reference'] else '—'} |")
    lines += ["", "## TA-Lib compatibility", "", "TA-Lib is an external oracle only. One-shot uppercase functions are intentionally not exported by `taflow.talib`; use persistent CamelCase classes from `taflow` (or optional state aliases from `taflow.talib.state`)."]
    OUT_MD.write_text("\n".join(lines) + "\n")
    print(f"wrote {OUT_JSON} and {OUT_MD}: {len(rows)} exports")


if __name__ == "__main__":
    main()
