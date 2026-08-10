"""Contract tests for the external correctness and benchmark registry."""

from __future__ import annotations

import taflow

from scripts.verification.registry import (
    NUMPY_BINDINGS,
    SMC_BINDINGS,
    WICKRA_BINDINGS,
    build_registry,
)


def test_registry_contains_every_public_lifecycle_class() -> None:
    """Every public indicator lifecycle must receive a verification status."""
    expected = {
        candidate
        for name in taflow.__all__
        if isinstance((candidate := getattr(taflow, name, None)), type)
        and all(
            hasattr(candidate, method)
            for method in ("append", "extend", "compute", "reset")
        )
    }
    registry = build_registry()
    actual = {spec.cls for spec in registry.values() if spec.cls is not None}

    assert actual == expected
    assert all(spec.error is None for spec in registry.values())


def test_talib_has_priority_and_wickra_bindings_are_explicit() -> None:
    """Wickra is only selected for named classes without a TA-Lib mapping."""
    registry = build_registry()
    by_class = {spec.cls.__name__: spec for spec in registry.values() if spec.cls}

    assert sum(spec.talib_name is not None for spec in registry.values()) == 161
    for class_name, binding in WICKRA_BINDINGS.items():
        spec = by_class[class_name]
        assert spec.talib_name is None
        assert spec.wickra == binding

    for class_name, binding in {**NUMPY_BINDINGS, **SMC_BINDINGS}.items():
        spec = by_class[class_name]
        assert spec.talib_name is None
        assert spec.wickra is None
        assert spec.numpy == binding or spec.smc == binding
