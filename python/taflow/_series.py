"""Shared one-dimensional input normalization for indicator objects."""

from __future__ import annotations

from typing import Any

import numpy as np


def as_float64_series(values: Any, *, column: str | None = None) -> np.ndarray:
    """Return a contiguous one-dimensional float64 array.

    Dataframes require an explicit column unless they contain exactly one
    column. Series-like objects are converted through ``to_numpy`` when
    available, which supports pandas and Polars without mandatory dependencies.
    """

    if hasattr(values, "columns"):
        columns = list(values.columns)
        if column is None:
            if len(columns) != 1:
                raise ValueError(
                    "column is required when a dataframe has multiple columns"
                )
            column = columns[0]
        if column not in columns:
            raise ValueError(f"column {column!r} was not found")
        values = values[column]
    elif column is not None:
        raise ValueError("column is only valid for dataframe input")

    if hasattr(values, "to_numpy"):
        values = values.to_numpy()

    try:
        array = np.asarray(values, dtype=np.float64)
    except (TypeError, ValueError) as error:
        raise ValueError("input must be a numeric one-dimensional series") from error

    if array.ndim != 1:
        raise ValueError("input must be one-dimensional")
    return np.ascontiguousarray(array)


def as_bool_series(values: Any, *, column: str | None = None) -> np.ndarray:
    """Return a contiguous one-dimensional boolean array.

    Dataframes follow the same explicit-column rules as numeric series. This
    adapter performs shape normalization only; indicator semantics stay in
    the native Rust state.
    """

    if hasattr(values, "columns"):
        columns = list(values.columns)
        if column is None:
            if len(columns) != 1:
                raise ValueError(
                    "column is required when a dataframe has multiple columns"
                )
            column = columns[0]
        if column not in columns:
            raise ValueError(f"column {column!r} was not found")
        values = values[column]
    elif column is not None:
        raise ValueError("column is only valid for dataframe input")

    if hasattr(values, "to_numpy"):
        values = values.to_numpy()

    try:
        array = np.asarray(values, dtype=np.bool_)
    except (TypeError, ValueError) as error:
        raise ValueError("input must be a boolean one-dimensional series") from error

    if array.ndim != 1:
        raise ValueError("input must be one-dimensional")
    return np.ascontiguousarray(array)


def as_int64_series(values: Any, *, column: str | None = None) -> np.ndarray:
    """Return a contiguous one-dimensional signed 64-bit integer array."""
    if hasattr(values, "columns"):
        columns = list(values.columns)
        if column is None:
            if len(columns) != 1:
                raise ValueError(
                    "column is required when a dataframe has multiple columns"
                )
            column = columns[0]
        if column not in columns:
            raise ValueError(f"column {column!r} was not found")
        values = values[column]
    elif column is not None:
        raise ValueError("column is only valid for dataframe input")
    if hasattr(values, "to_numpy"):
        values = values.to_numpy()
    try:
        array = np.asarray(values, dtype=np.int64)
    except (TypeError, ValueError) as error:
        raise ValueError("input must be an integer one-dimensional series") from error
    if array.ndim != 1:
        raise ValueError("input must be one-dimensional")
    return np.ascontiguousarray(array)
