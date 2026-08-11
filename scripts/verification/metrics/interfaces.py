"""Audit the public, class-only lifecycle of :mod:`taflow.metrics`."""

from __future__ import annotations

import argparse
import ast
import inspect
import re
from pathlib import Path
from typing import Any, get_type_hints

import numpy as np

try:
    from .registry import MetricSpec, resolve_specs
except ImportError:
    from registry import MetricSpec, resolve_specs  # type: ignore[no-redef]


def _annotation_text(value: Any) -> str:
    return value if isinstance(value, str) else str(value)


def _public_functions(tree: ast.Module) -> list[str]:
    return [
        node.name
        for node in tree.body
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
        and not node.name.startswith("_")
    ]


def audit(spec: MetricSpec) -> list[str]:
    """Return contract failures for one available metric class."""
    failures: list[str] = []
    cls = spec.load_class()
    source_path = Path(inspect.getsourcefile(cls) or "")
    expected_name = f"{spec.module}.py"
    if source_path.name != expected_name:
        failures.append(f"class source is {source_path.name}, expected {expected_name}")
    source = source_path.read_text(encoding="utf-8")
    tree = ast.parse(source)
    classes = [
        node.name
        for node in tree.body
        if isinstance(node, ast.ClassDef) and not node.name.startswith("_")
    ]
    if classes != [spec.class_name]:
        failures.append(
            f"public classes are {classes!r}, expected only {spec.class_name}"
        )
    forbidden_functions = _public_functions(tree)
    if forbidden_functions:
        failures.append(f"module-level public functions found: {forbidden_functions}")
    if re.search(rf"^\s*def\s+{re.escape(spec.module)}\s*\(", source, re.MULTILINE):
        failures.append("same-named free metric function found")
    doc = inspect.getdoc(cls) or ""
    for required in ("Rust", "warm", "oracle"):
        if required.casefold() not in doc.casefold():
            failures.append(f"class docstring does not describe {required}")

    factory_name = spec.factories[0]
    factory = getattr(cls, factory_name, None)
    if factory is None:
        failures.append(f"{factory_name} factory is missing")
        return failures
    parameters = list(inspect.signature(factory).parameters.values())
    required_series = 2 if spec.paired else 1
    if len(parameters) < required_series or any(
        parameter.default is not inspect.Parameter.empty
        for parameter in parameters[:required_series]
    ):
        failures.append(f"{factory_name} must require its series")
    try:
        empty = np.array([], dtype=np.float64)
        state = factory(empty, empty) if spec.paired else factory(empty)
    except Exception as error:  # noqa: BLE001 - audit reports public failures.
        failures.append(f"empty {factory_name} construction failed: {error}")
        return failures
    for method_name in ("append", "extend", "compute", "reset", "__len__"):
        if not callable(getattr(state, method_name, None)):
            failures.append(f"{method_name} is missing")
    if not hasattr(type(state), "value"):
        failures.append("value property is missing")
    if failures:
        return failures
    if len(state) != 0:
        failures.append("empty state length is not zero")
    if state.compute() is not None or state.value is not None:
        failures.append("empty state does not expose None")
    append_result = state.append(0.01, 0.005) if spec.paired else state.append(0.01)
    if append_result is not state:
        failures.append("append is not fluent")
    extend_result = (
        state.extend(np.array([-0.02, 0.015]), np.array([-0.01, 0.005]))
        if spec.paired
        else state.extend(np.array([-0.02, 0.015]))
    )
    if extend_result is not state:
        failures.append("extend is not fluent")
    if len(state) != 3:
        failures.append("native delegated length is not three")
    if state.reset() is not state or len(state) != 0:
        failures.append("reset is not fluent or did not clear length")

    for method_name in ("append", "extend", "reset"):
        annotation = inspect.signature(getattr(cls, method_name)).return_annotation
        if spec.class_name not in _annotation_text(annotation):
            failures.append(f"{method_name} return annotation is not {spec.class_name}")
    compute_annotation = inspect.signature(cls.compute).return_annotation
    if compute_annotation is inspect.Signature.empty:
        failures.append("compute has no concrete return annotation")
    try:
        hints = get_type_hints(cls.compute)
        if "return" not in hints:
            failures.append("compute return annotation cannot be resolved")
    except (NameError, TypeError):
        failures.append("compute return annotation cannot be resolved")
    return failures


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("metrics", nargs="*", help="canonical metric class names")
    parser.add_argument(
        "--metric",
        action="append",
        default=[],
        help="canonical metric class name (repeatable)",
    )
    parser.add_argument("--list", action="store_true", help="list registered contracts")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    requested = args.metrics + args.metric
    if args.list:
        for spec in resolve_specs(requested or None):
            print(f"{spec.class_name}\ttaflow.metrics.{spec.module}")
        return 0
    specs = resolve_specs(requested or None, available_only=not requested)
    if not specs:
        raise RuntimeError("no implemented registered metrics found")
    failed = False
    for spec in specs:
        failures = audit(spec)
        status = "PASS" if not failures else "FAIL"
        print(f"{status} {spec.class_name}")
        for failure in failures:
            print(f"  - {failure}")
        failed |= bool(failures)
    return int(failed)


if __name__ == "__main__":
    raise SystemExit(main())
