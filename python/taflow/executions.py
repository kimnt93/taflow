"""Canonical execution and data-adapter namespace.

Indicator classes live in :mod:`taflow`; execution graph helpers are kept in
this explicit namespace so the root package remains focused on computation.
"""

from .execution import (
    AdapterGateway as TAAdapterGateway,
    ArrowAdapter as TAArrowAdapter,
    Expr as TAExpr,
    NumpyAdapter as TANumpyAdapter,
    Pipeline as TAPipeline,
    PolarsAdapter as TAPolarsAdapter,
    PythonListAdapter as TAPythonListAdapter,
    adapt_input as AdaptInput,
    adapt_output as AdaptOutput,
)
from .rolling_apply import rolling_apply as _rolling_apply
from .session import session_flags as _session_flags

# Both singular and plural spellings are provided for discoverability; they
# refer to the same graph implementation.
TAPipelines = TAPipeline


def RollingApply(_input: object, timeperiod: int, function: object) -> object:
    """Apply an incremental-compatible reducer over a rolling window."""
    return _rolling_apply(_input, timeperiod, function)


def SessionFlags(session_id: object) -> object:
    """Convert session identifiers to native session-boundary flags."""
    return _session_flags(session_id)


def ToNumpy(values: object) -> object:
    """Convert computed values to a contiguous NumPy representation."""
    return AdaptOutput(values, adapter="numpy")


def ToList(values: object) -> object:
    """Convert computed values to a Python list."""
    return AdaptOutput(values, adapter="list")


def ToArrow(values: object) -> object:
    """Convert computed values to an Arrow array using the optional adapter."""
    return AdaptOutput(values, adapter="arrow")


def ToPolars(values: object, *, name: object = "value") -> object:
    """Convert computed values to a Polars series using the optional adapter."""
    return AdaptOutput(values, adapter="polars", name=name)


def ToPandas(values: object, *, name: object = "value") -> object:
    """Convert computed values to a pandas Series (optional dependency)."""
    import pandas as pd

    return pd.Series(values, name=name)

__all__ = [
    "TAPipeline",
    "TAPipelines",
    "RollingApply",
    "SessionFlags",
    "TAExpr",
    "TANumpyAdapter",
    "TAPythonListAdapter",
    "TAArrowAdapter",
    "TAPolarsAdapter",
    "TAAdapterGateway",
    "AdaptInput",
    "AdaptOutput",
    "ToNumpy",
    "ToList",
    "ToArrow",
    "ToPolars",
    "ToPandas",
]
