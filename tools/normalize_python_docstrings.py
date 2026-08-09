#!/usr/bin/env python3
"""Replace generator placeholders with signature-aware adapter documentation."""

from __future__ import annotations

import argparse
import ast
import inspect
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PACKAGE = ROOT / "python" / "taflow"
PLACEHOLDERS = (
    "Parameters are documented by the constructor signature",
    "Input values or the aligned result container",
    "Updated state, converted values, or aligned output",
    "updated adapter or output",
)
SHARED = {
    "UnaryStateAdapter", "OhlcStateAdapter", "OhlcvStateAdapter",
    "CloseVolumeStateAdapter", "OhlcPriceState", "HlcPriceState",
    "HlPriceState", "BarRelationAdapter", "ConditionValueAdapter",
    "_MathUnary", "_MathBinary", "_RollingPair", "_Bivariate",
}
SERIES = {
    "_input", "input", "values", "price", "change", "value", "left",
    "right", "x", "y", "benchmark", "close", "high", "low", "h", "l",
    "_open", "open", "volume", "periods", "condition", "new_session",
    "anchor", "entry", "_exit", "input0", "input1", "_input0", "_input1",
}


def public_by_file() -> dict[Path, set[str]]:
    import taflow

    result: dict[Path, set[str]] = {}
    for name in taflow.__all__:
        cls = getattr(taflow, name, None)
        if isinstance(cls, type) and hasattr(cls, "append"):
            source = inspect.getsourcefile(cls)
            if source:
                result.setdefault(Path(source).resolve(), set()).add(name)
    for path in PACKAGE.glob("*.py"):
        result.setdefault(path.resolve(), set())
    return result


def parameters(node: ast.FunctionDef) -> list[ast.arg]:
    return [
        arg for arg in [*node.args.posonlyargs, *node.args.args, *node.args.kwonlyargs]
        if arg.arg not in {"self", "cls"}
    ]


def role(name: str, scalar: bool) -> str:
    clean = name.lstrip("_").replace("input0", "first input").replace("input1", "second input")
    descriptions = {
        "open": "open price", "high": "high price", "low": "low price",
        "close": "close price", "volume": "volume", "condition": "boolean condition",
        "new_session": "session-boundary flag", "anchor": "anchor-reset flag",
        "entry": "entry condition", "exit": "exit condition", "benchmark": "benchmark value",
        "left": "left operand", "right": "right operand", "periods": "per-bar periods",
    }
    label = descriptions.get(clean, clean.replace("_", " "))
    return f"Current {label}." if scalar else f"Chronological {label} series."


def method_doc(node: ast.FunctionDef, owner: str) -> str:
    params = parameters(node)
    if node.name == "__init__":
        summary = "Initialize the native Rust state and process the required histories."
        scalar = False
        returns = ("None", "Constructors initialize state and do not return a value.")
    elif node.name == "append":
        summary = "Append one chronological observation to the native Rust state."
        scalar = True
        returns = (owner, "This indicator, for fluent chaining; read `value` for the result.")
    elif node.name == "extend":
        summary = "Append aligned chronological histories to the native Rust state."
        scalar = False
        returns = (owner, "This indicator, for fluent chaining.")
    elif node.name == "compute":
        return "Return the complete aligned history produced by Rust.\n\nReturns\n-------\nnumpy.ndarray or tuple of numpy.ndarray\n    One output per processed bar, including NaN warm-up positions."
    elif node.name == "value":
        return "Return the latest Rust result.\n\nReturns\n-------\nfloat, tuple, or None\n    Latest output, or None while scalar warm-up is incomplete."
    elif node.name == "reset":
        return f"Restore fresh-state behavior and clear output history.\n\nReturns\n-------\n{owner}\n    This indicator, for fluent chaining."
    else:
        return ast.get_docstring(node, clean=False) or ""

    lines = [summary]
    if params:
        lines += ["", "Parameters", "----------"]
        for parameter in params:
            annotation = ast.unparse(parameter.annotation) if parameter.annotation else "object"
            lines += [f"{parameter.arg} : {annotation}", f"    {role(parameter.arg, scalar)}"]
    lines += ["", "Returns", "-------", returns[0], f"    {returns[1]}"]
    return "\n".join(lines)


def class_doc(node: ast.ClassDef, module_summary: str) -> str:
    init = next((item for item in node.body if isinstance(item, ast.FunctionDef) and item.name == "__init__"), None)
    series = [arg.arg for arg in parameters(init)] if init else []
    inputs = [name for name in series if name in SERIES]
    suffix = f" Required input histories: {', '.join(f'`{name}`' for name in inputs)}." if inputs else ""
    return (
        f"{module_summary}\n\n"
        "This public class owns a persistent native Rust state; Python performs "
        "container conversion only. `append`, `extend`, and `reset` are fluent, "
        "`value` exposes the latest result, and `compute` returns aligned history."
        f"{suffix} Warm-up positions are represented by `NaN` in history."
    )


def offsets(text: str) -> list[int]:
    result = [0]
    for line in text.splitlines(keepends=True):
        result.append(result[-1] + len(line))
    return result


def absolute(starts: list[int], node: ast.AST, end: bool = False) -> int:
    line = node.end_lineno if end else node.lineno
    column = node.end_col_offset if end else node.col_offset
    return starts[line - 1] + column


def literal(doc: str, indent: int) -> str:
    escaped = doc.replace('"""', '\\"\\"\\"')
    padding = " " * indent
    return '"""' + escaped.replace("\n", "\n" + padding) + '"""'


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    changed = 0
    for path, names in sorted(public_by_file().items()):
        text = path.read_text()
        tree = ast.parse(text)
        starts = offsets(text)
        summary = (ast.get_docstring(tree) or path.stem.replace("_", " ").title()).splitlines()[0]
        edits: list[tuple[int, int, str]] = []
        for cls in (item for item in tree.body if isinstance(item, ast.ClassDef)):
            if cls.name not in names and cls.name not in SHARED:
                continue
            owner = "Self" if cls.name in SHARED else cls.name
            class_expr = cls.body[0] if cls.body and isinstance(cls.body[0], ast.Expr) else None
            if class_expr and isinstance(class_expr.value, ast.Constant) and isinstance(class_expr.value.value, str):
                if any(token in class_expr.value.value for token in PLACEHOLDERS):
                    edits.append((absolute(starts, class_expr), absolute(starts, class_expr, True),
                                  literal(class_doc(cls, summary), class_expr.col_offset)))
            for method in (item for item in cls.body if isinstance(item, ast.FunctionDef)):
                expr = method.body[0] if method.body and isinstance(method.body[0], ast.Expr) else None
                if not expr or not isinstance(expr.value, ast.Constant) or not isinstance(expr.value.value, str):
                    continue
                if any(token in expr.value.value for token in PLACEHOLDERS):
                    edits.append((absolute(starts, expr), absolute(starts, expr, True),
                                  literal(method_doc(method, owner), expr.col_offset)))
        updated = text
        for start, end, replacement in sorted(edits, reverse=True):
            updated = updated[:start] + replacement + updated[end:]
        if updated != text:
            changed += 1
            if not args.check:
                path.write_text(updated)
    print(f"{changed} files need doc normalization" if args.check else f"normalized docs in {changed} files")
    return int(args.check and bool(changed))


if __name__ == "__main__":
    raise SystemExit(main())
