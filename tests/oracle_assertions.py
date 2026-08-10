"""Shared deterministic data matrices for external-oracle tests."""

import numpy as np

from scripts.verification.correctness import verdict, verify_function
from scripts.verification.registry import build_registry, make_data


def assert_registered_oracle_match(class_name: str, bars: int = 512) -> None:
    """Assert oracle and lifecycle parity on random and adversarial histories."""
    specs = {
        spec.cls.__name__: spec
        for spec in build_registry().values()
        if spec.cls is not None
    }
    cases = [("random", make_data(bars), bars)]
    for case_name in ("constant", "monotonic", "repeated_extrema", "minimum"):
        size = 64
        data = make_data(size, seed=117)
        if case_name == "constant":
            close = np.full(size, 100.0)
        elif case_name == "monotonic":
            close = np.linspace(80.0, 120.0, size)
        elif case_name == "repeated_extrema":
            close = np.resize(np.array([99.0, 101.0, 101.0, 99.0]), size)
        else:
            close = data["close"]
        if case_name != "minimum":
            data.update(
                open=close.copy(),
                high=close + 1.0,
                low=close - 1.0,
                close=close,
                close2=close * 1.01,
                volume=np.full(size, 1_000.0),
            )
        cases.append((case_name, data, size))

    for case_name, data, size in cases:
        row = verify_function(specs[class_name], data, size, max(1, size - 17))
        assert verdict(row) == "MATCH", {"case": case_name, **row}
