#!/usr/bin/env python3
"""Normalize public indicator constructor and fluent return annotations.

This is intentionally a narrow source codemod.  It does not alter method
bodies or numerical behavior.  Run it after adding an adapter, then run the
interface audit to verify the resulting runtime signatures.
"""

from __future__ import annotations

import argparse
import ast
import inspect
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[1]
PACKAGE = ROOT / "python" / "taflow"
SERIES_NAMES = {
    "_input", "input", "values", "price", "change", "value", "left",
    "right", "x", "y", "benchmark", "close", "high", "low", "h", "l",
    "_open", "open", "volume", "periods", "condition", "new_session",
    "anchor", "entry", "_exit", "input0", "input1", "_input0", "_input1",
}
SHARED_CLASSES = {
    "UnaryStateAdapter", "OhlcStateAdapter", "OhlcvStateAdapter",
    "CloseVolumeStateAdapter", "OhlcPriceState", "HlcPriceState",
    "HlPriceState", "_MathUnary", "_MathBinary", "_RollingPair", "_Bivariate",
}
DEFAULTS = {
    ("Lag", "timeperiod"): "1",
    ("LogReturn", "timeperiod"): "1",
    ("HullMovingAverage", "timeperiod"): "10",
    ("VolumeWeightedMovingAverage", "timeperiod"): "10",
    ("ZeroLagExponentialMovingAverage", "timeperiod"): "10",
    ("ArnaudLegouxMovingAverage", "timeperiod"): "10",
    ("Rising", "timeperiod"): "1",
    ("Falling", "timeperiod"): "1",
    ("SignalDelay", "timeperiod"): "1",
    ("SignedPower", "exponent"): "2.0",
    ("RollingPercentile", "percentile"): "50.0",
    ("RollingQuantile", "quantile"): "0.5",
}


def default_for(class_name: str, parameter: str) -> str:
    if (class_name, parameter) in DEFAULTS:
        return DEFAULTS[class_name, parameter]
    if parameter == "timeperiod":
        return "14"
    raise KeyError(f"no safe default for {class_name}.{parameter}")


def annotation_text(annotation: ast.expr | None, *, required_series: bool) -> str:
    if annotation is None:
        return "object"
    if required_series and isinstance(annotation, ast.BinOp) and isinstance(annotation.op, ast.BitOr):
        if isinstance(annotation.right, ast.Constant) and annotation.right.value is None:
            annotation = annotation.left
        elif isinstance(annotation.left, ast.Constant) and annotation.left.value is None:
            annotation = annotation.right
    if (
        required_series
        and isinstance(annotation, ast.Subscript)
        and isinstance(annotation.value, ast.Name)
        and annotation.value.id == "Optional"
    ):
        annotation = annotation.slice
    return ast.unparse(annotation)


def public_classes() -> dict[Path, set[str]]:
    import taflow

    result: dict[Path, set[str]] = {}
    for name in taflow.__all__:
        cls = getattr(taflow, name, None)
        if not isinstance(cls, type) or not hasattr(cls, "append"):
            continue
        source = inspect.getsourcefile(cls)
        if source:
            result.setdefault(Path(source).resolve(), set()).add(name)
    return result


def positional_defaults(node: ast.FunctionDef) -> dict[str, ast.expr | None]:
    args = [*node.args.posonlyargs, *node.args.args]
    result: dict[str, ast.expr | None] = {arg.arg: None for arg in args}
    for arg, default in zip(args[-len(node.args.defaults):], node.args.defaults):
        result[arg.arg] = default
    return result


def constructor_header(node: ast.FunctionDef, class_name: str) -> str:
    positional = [*node.args.posonlyargs, *node.args.args]
    defaults = positional_defaults(node)
    self_args = [arg for arg in positional if arg.arg in {"self", "cls"}]
    values = [arg for arg in positional if arg.arg not in {"self", "cls"}]
    values.sort(key=lambda arg: arg.arg not in SERIES_NAMES)
    # Replacement begins at the `def` token, after the line's existing class
    # indentation, so only continuation lines include their full indentation.
    lines = ["def __init__("]
    for arg in [*self_args, *values]:
        if arg.arg in {"self", "cls"}:
            lines.append(f"        {arg.arg},")
            continue
        is_series = arg.arg in SERIES_NAMES
        annotation = annotation_text(arg.annotation, required_series=is_series)
        default = None if is_series else defaults[arg.arg]
        if default is None and not is_series:
            default_text = default_for(class_name, arg.arg)
        elif default is not None:
            default_text = ast.unparse(default)
        else:
            default_text = None
        suffix = f" = {default_text}" if default_text is not None else ""
        lines.append(f"        {arg.arg}: {annotation}{suffix},")
    if node.args.vararg:
        lines.append(f"        *{node.args.vararg.arg},")
    elif node.args.kwonlyargs:
        lines.append("        *,")
    for arg, default in zip(node.args.kwonlyargs, node.args.kw_defaults):
        is_series = arg.arg in SERIES_NAMES
        annotation = annotation_text(arg.annotation, required_series=is_series)
        if is_series:
            default_text = None
        elif default is None:
            default_text = default_for(class_name, arg.arg)
        else:
            default_text = ast.unparse(default)
        suffix = f" = {default_text}" if default_text is not None else ""
        lines.append(f"        {arg.arg}: {annotation}{suffix},")
    if node.args.kwarg:
        lines.append(f"        **{node.args.kwarg.arg},")
    lines.append("    ) -> None:")
    return "\n".join(lines)


def offsets(text: str) -> list[int]:
    starts = [0]
    for line in text.splitlines(keepends=True):
        starts.append(starts[-1] + len(line))
    return starts


def absolute(starts: list[int], line: int, column: int) -> int:
    return starts[line - 1] + column


def header_end(text: str, start: int, body_start: int) -> int:
    # The final signature colon is the last colon before the first body node.
    return text.rfind(":", start, body_start) + 1


def changes_for(path: Path, names: set[str]) -> list[tuple[int, int, str]]:
    text = path.read_text()
    starts = offsets(text)
    tree = ast.parse(text)
    edits: list[tuple[int, int, str]] = []
    for cls in (node for node in tree.body if isinstance(node, ast.ClassDef)):
        if cls.name not in names and cls.name not in SHARED_CLASSES:
            continue
        return_name = "Self" if cls.name in SHARED_CLASSES else cls.name
        for method in (node for node in cls.body if isinstance(node, ast.FunctionDef)):
            if method.name == "__init__" and method.body:
                start = absolute(starts, method.lineno, method.col_offset)
                body = absolute(starts, method.body[0].lineno, method.body[0].col_offset)
                end = header_end(text, start, body)
                edits.append((start, end, constructor_header(method, cls.name)))
            elif method.name in {"append", "extend", "reset"}:
                if method.returns is not None:
                    start = absolute(starts, method.returns.lineno, method.returns.col_offset)
                    end = absolute(starts, method.returns.end_lineno, method.returns.end_col_offset)
                    edits.append((start, end, f'"{return_name}"'))
    return edits


def apply_edits(text: str, edits: Iterable[tuple[int, int, str]]) -> str:
    for start, end, replacement in sorted(edits, reverse=True):
        text = text[:start] + replacement + text[end:]
    return text


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    by_file = public_classes()
    # Shared bases may not be the source location inspect reports for a public
    # subclass, so include every package module in the narrow AST scan.
    for path in PACKAGE.glob("*.py"):
        by_file.setdefault(path.resolve(), set())
    changed: list[Path] = []
    for path, names in sorted(by_file.items()):
        before = path.read_text()
        after = apply_edits(before, changes_for(path, names))
        if after != before:
            changed.append(path)
            if not args.check:
                path.write_text(after)
    print(f"{len(changed)} files need normalization" if args.check else
          f"normalized {len(changed)} files")
    return int(args.check and bool(changed))


if __name__ == "__main__":
    raise SystemExit(main())
