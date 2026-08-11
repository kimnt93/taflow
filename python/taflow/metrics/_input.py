"""Shared container normalization for metric adapters."""

from __future__ import annotations

from typing import Any

import numpy as np

from .._series import as_float64_series


def as_metric_series(values: Any, *, column: str | None = None) -> np.ndarray:
    """Return one chronological contiguous float64 metric input series.

    This performs container and shape normalization only. Return derivation,
    P&L conversion, missing-value handling, and metric arithmetic remain in the
    native Rust state.
    """

    return as_float64_series(values, column=column)


def as_paired_metric_series(
    values: Any,
    benchmark_values: Any,
    *,
    column: str | None = None,
    benchmark_column: str | None = None,
) -> tuple[np.ndarray, np.ndarray]:
    """Normalize two ordered metric inputs and reject misalignment.

    The primary series is returned first and the benchmark second. Length is
    checked before either array reaches native state, preventing partial
    mutation in paired metric adapters.
    """

    primary = as_metric_series(values, column=column)
    benchmark = as_metric_series(benchmark_values, column=benchmark_column)
    if primary.shape[0] != benchmark.shape[0]:
        raise ValueError("metric and benchmark series must have equal length")
    return primary, benchmark
