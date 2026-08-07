"""Causal execution graph, expression engine, and column adapters.

The graph executes each input row once.  Indicator nodes own persistent
state, while expression nodes read already-computed values from the current
row.  Nodes are memoized by identity, so a shared sub-expression is evaluated
once even when several outputs depend on it.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Callable, Mapping, Sequence

import numpy as np

from ._series import as_float64_series


class Expr:
    """A lazy scalar expression evaluated against one input bar.

    Parameters
    ----------
    fn : callable
        Function receiving a mapping of source names to scalar values.
    deps : iterable of Expr, optional
        Expressions that must be evaluated before this expression.
    name : str, optional
        Human-readable expression name used in graph diagnostics.
    """

    def __init__(
        self,
        fn: Callable[[Mapping[str, float]], float],
        deps: object = (),
        name: object = "expr",
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        fn : object
            Values or parameters consumed by this operation.
        deps : object
            Values or parameters consumed by this operation.
        name : object
            Values or parameters consumed by this operation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._fn, self.deps, self.name = fn, tuple(deps), name

    def eval(self, row: Mapping[str, float]) -> float:
        """Evaluate the expression for one aligned input row.

        Parameters
        ----------
        row : Mapping[str, float]
            Values for all source fields referenced by this expression.

        Returns
        -------
        float
            The scalar expression result.
        """
        return self._fn(row)

    def _binary(self, other: object, op: object, symbol: object) -> object:
        """Execute the _binary operation through the native Rust implementation.

        Parameters
        ----------
        other : object
            Values or parameters consumed by this operation.
        op : object
            Values or parameters consumed by this operation.
        symbol : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        rhs = (
            other
            if isinstance(other, Expr)
            else Expr(lambda _row, value=other: value, name=repr(other))
        )
        return Expr(
            lambda row: op(self.eval(row), rhs.eval(row)),
            (self, rhs),
            f"({self.name}{symbol}{rhs.name})",
        )

    def __add__(self, other: object) -> object:
        """Execute the __add__ operation through the native Rust implementation.

        Parameters
        ----------
        other : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._binary(other, lambda a, b: a + b, "+")

    def __radd__(self, other: object) -> object:
        """Execute the __radd__ operation through the native Rust implementation.

        Parameters
        ----------
        other : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._binary(other, lambda a, b: b + a, "+")

    def __sub__(self, other: object) -> object:
        """Execute the __sub__ operation through the native Rust implementation.

        Parameters
        ----------
        other : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._binary(other, lambda a, b: a - b, "-")

    def __rsub__(self, other: object) -> object:
        """Execute the __rsub__ operation through the native Rust implementation.

        Parameters
        ----------
        other : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._binary(other, lambda a, b: b - a, "-")

    def __mul__(self, other: object) -> object:
        """Execute the __mul__ operation through the native Rust implementation.

        Parameters
        ----------
        other : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._binary(other, lambda a, b: a * b, "*")

    def __rmul__(self, other: object) -> object:
        """Execute the __rmul__ operation through the native Rust implementation.

        Parameters
        ----------
        other : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._binary(other, lambda a, b: b * a, "*")

    def __truediv__(self, other: object) -> object:
        """Execute the __truediv__ operation through the native Rust implementation.

        Parameters
        ----------
        other : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._binary(other, lambda a, b: a / b if b else np.nan, "/")

    def __rtruediv__(self, other: object) -> object:
        """Execute the __rtruediv__ operation through the native Rust implementation.

        Parameters
        ----------
        other : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._binary(other, lambda a, b: b / a if a else np.nan, "/")

    def __neg__(self) -> object:
        """Execute the __neg__ operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return Expr(lambda row: -self.eval(row), (self,), f"(-{self.name})")


@dataclass(frozen=True)
class _Source(Expr):
    field: str = ""

    def __init__(self, field: str) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        field : object
            Values or parameters consumed by this operation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        object.__setattr__(self, "field", field)
        object.__setattr__(self, "name", field)
        object.__setattr__(self, "deps", ())
        object.__setattr__(self, "_fn", lambda row: row[field])


class _Indicator(Expr):
    def __init__(self, name: str, state: Any, inputs: Sequence[Expr]) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        name : object
            Values or parameters consumed by this operation.
        state : object
            Values or parameters consumed by this operation.
        inputs : object
            Values or parameters consumed by this operation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self.state, self.inputs = state, tuple(inputs)
        super().__init__(lambda row: self._value, self.inputs, name)
        self._value = np.nan

    def step(self, row: object, cache: object) -> object:
        """Execute the step operation through the native Rust implementation.

        Parameters
        ----------
        row : object
            Values or parameters consumed by this operation.
        cache : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        args = [_evaluate(dep, row, cache) for dep in self.inputs]
        value = self.state.append(*args)
        self._value = np.nan if value is None else value
        return self._value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        if hasattr(self.state, "reset"):
            self.state.reset()
        self._value = np.nan


class _Expression(Expr):
    def __init__(self, expression: Expr) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        expression : object
            Values or parameters consumed by this operation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self.expression = expression
        super().__init__(expression._fn, expression.deps, expression.name)

    def step(self, row: object, cache: object) -> object:
        """Execute the step operation through the native Rust implementation.

        Parameters
        ----------
        row : object
            Values or parameters consumed by this operation.
        cache : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return _evaluate(self.expression, row, cache)


def _evaluate(expr: Expr, row: object, cache: object) -> object:
    """Evaluate this execution expression for one input row.

    Parameters
    ----------
    expr : object
        Values or parameters consumed by this operation.
    row : object
        Values or parameters consumed by this operation.
    cache : object
        Values or parameters consumed by this operation.

    Returns
    -------
    object
        The updated adapter, native value, aligned output array, or execution node.
    """
    key = id(expr)
    if key in cache:
        return cache[key]
    if isinstance(expr, _Source):
        value = expr.eval(row)
    elif isinstance(expr, _Indicator):
        value = expr.step(row, cache)
    elif isinstance(expr, _Expression):
        value = expr.step(row, cache)
    else:
        # Binary Expr closures call their operands directly; evaluating their
        # dependency graph here ensures shared indicator nodes are cached.
        for dep in expr.deps:
            _evaluate(dep, row, cache)
        value = expr.eval(row)
    cache[key] = value
    return value


class Pipeline:
    """A reusable one-pass causal indicator dependency graph.

    Each input row is dispatched once. Indicator state is retained between
    calls, and shared expression nodes are evaluated once per row.
    """

    def __init__(self) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._sources: dict[str, _Source] = {}
        self._nodes: list[Expr] = []
        self._outputs: dict[str, Expr] = {}

    def source(self, field: str) -> Expr:
        """Return the memoized source node for an input field.

        Parameters
        ----------
        field : str
            Name used in rows passed to :meth:`append` or :meth:`extend`.

        Returns
        -------
        Expr
            Source expression node.
        """
        if field not in self._sources:
            self._sources[field] = _Source(field)
        return self._sources[field]

    def indicator(self, name: str, state: Any, *inputs: Expr) -> Expr:
        """Add a stateful indicator node to the graph.

        Parameters
        ----------
        name : str
            Stable diagnostic name for the node.
        state : object
            Object exposing ``append(*values)`` and optionally ``reset()``.
        *inputs : Expr
            Source or derived expressions consumed by the state object.

        Returns
        -------
        Expr
            The newly created indicator expression.
        """
        node = _Indicator(name, state, inputs)
        self._nodes.append(node)
        return node

    def expression(self, name: str, expression: Expr) -> Expr:
        """Add a derived expression node and return it."""
        node = _Expression(expression)
        self._nodes.append(node)
        return node

    def output(self, name: str, node: Expr) -> Expr:
        """Expose a graph node under an output name."""
        self._outputs[name] = node
        return node

    @property
    def outputs(self) -> tuple[str, ...]:
        """Execute the outputs operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return tuple(self._outputs)

    def reset(self) -> object:
        """Reset all stateful nodes and return this pipeline."""
        for node in self._nodes:
            if isinstance(node, _Indicator):
                node.reset()
        return self

    def append(self, row: Mapping[str, float]) -> dict[str, float]:
        """Dispatch one aligned bar through the graph exactly once."""
        cache: dict[int, float] = {}
        return {
            name: _evaluate(node, row, cache) for name, node in self._outputs.items()
        }

    def extend(self, rows: Mapping[str, Sequence[float]]) -> dict[str, np.ndarray]:
        """Run aligned columns and return same-length output arrays."""
        columns = {name: as_float64_series(values) for name, values in rows.items()}
        if not columns:
            return {name: np.empty(0) for name in self._outputs}
        length = len(next(iter(columns.values())))
        if any(len(values) != length for values in columns.values()):
            raise ValueError("all pipeline inputs must have the same length")
        result = {name: np.empty(length, dtype=np.float64) for name in self._outputs}
        for i in range(length):
            row = {name: values[i] for name, values in columns.items()}
            values = self.append(row)
            for name, value in values.items():
                result[name][i] = value
        return result


class NumpyAdapter:
    """Zero-copy where possible NumPy input/output adapter."""

    @staticmethod
    def input(values: object, *, column: object = None) -> object:
        """Convert an array-like input to contiguous float64 values."""
        return as_float64_series(values, column=column)

    @staticmethod
    def output(values: object) -> object:
        """Return contiguous float64 NumPy output."""
        return np.ascontiguousarray(values, dtype=np.float64)


class PythonListAdapter:
    """Adapter for Python sequences, with explicit list conversion."""

    @staticmethod
    def input(values: object, *, column: object = None) -> object:
        """Convert a Python sequence to contiguous float64 values."""
        return as_float64_series(values, column=column)

    @staticmethod
    def output(values: object) -> object:
        """Return output as a Python list of floats."""
        return np.asarray(values, dtype=np.float64).tolist()


class ArrowAdapter:
    """Optional Apache Arrow adapter; import is deferred until use."""

    @staticmethod
    def _module() -> object:
        """Execute the _module operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        try:
            import pyarrow as pa
        except ImportError as error:
            raise ImportError(
                "ArrowAdapter requires the 'arrow' extra (pyarrow)"
            ) from error
        return pa

    @classmethod
    def input(cls, values: object, *, column: object = None) -> object:
        """Execute the input operation through the native Rust implementation.

        Parameters
        ----------
        values : object
            Input values processed in chronological order.
        column : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        pa = cls._module()
        if isinstance(values, pa.Table):
            if column is None:
                if values.num_columns != 1:
                    raise ValueError("column is required for multi-column Arrow tables")
                column = values.column_names[0]
            values = values[column]
        if isinstance(values, pa.ChunkedArray):
            values = values.combine_chunks()
        return np.ascontiguousarray(
            values.to_numpy(zero_copy_only=False), dtype=np.float64
        )

    @classmethod
    def output(cls, values: object) -> object:
        """Convert values to an Arrow array."""
        return cls._module().array(np.asarray(values, dtype=np.float64))


class PolarsAdapter:
    """Optional Polars adapter; import is deferred until use."""

    @staticmethod
    def _module() -> object:
        """Execute the _module operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        try:
            import polars as pl
        except ImportError as error:
            raise ImportError(
                "PolarsAdapter requires the 'polars' extra (polars)"
            ) from error
        return pl

    @classmethod
    def input(cls, values: object, *, column: object = None) -> object:
        """Execute the input operation through the native Rust implementation.

        Parameters
        ----------
        values : object
            Input values processed in chronological order.
        column : object
            Values or parameters consumed by this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        pl = cls._module()
        if isinstance(values, pl.DataFrame):
            if column is None:
                if values.width != 1:
                    raise ValueError(
                        "column is required for multi-column Polars frames"
                    )
                column = values.columns[0]
            values = values.get_column(column)
        return np.ascontiguousarray(values.to_numpy(), dtype=np.float64)

    @classmethod
    def output(cls, values: object, name: object = "value") -> object:
        """Convert values to a Polars Series."""
        return cls._module().Series(name, values)


def adapt_input(
    values: object, *, adapter: object = "numpy", column: object = None
) -> object:
    """Convert one input through a named adapter.

    Parameters
    ----------
    values : array-like
        Input values or a supported table/series container.
    adapter : str, optional
        ``numpy``, ``list``, ``arrow``, or ``polars``.
    column : str, optional
        Column to select from a multi-column container.

    Returns
    -------
    numpy.ndarray
        Contiguous float64 input values.
    """
    return AdapterGateway.input(values, adapter=adapter, column=column)


def adapt_output(
    values: object, *, adapter: object = "numpy", **kwargs: object
) -> object:
    """Convert computed values through a named output adapter.

    Parameters
    ----------
    values : array-like
        Aligned values produced by a pipeline or indicator.
    adapter : str, optional
        ``numpy``, ``list``, ``arrow``, or ``polars``.
    **kwargs : object
        Adapter-specific output options, such as a Polars series name.

    Returns
    -------
    object
        Values represented by the requested container adapter.
    """
    return AdapterGateway.output(values, adapter=adapter, **kwargs)


class AdapterGateway:
    """Single dispatch gateway for supported input and output containers."""

    _adapters = {
        "numpy": NumpyAdapter,
        "list": PythonListAdapter,
        "arrow": ArrowAdapter,
        "polars": PolarsAdapter,
    }

    @classmethod
    def register(cls, name: str, adapter: type) -> None:
        """Register an adapter implementing ``input`` and ``output``.

        Parameters
        ----------
        name : str
            Name used by :meth:`input` and :meth:`output`.
        adapter : type
            Adapter class exposing callable ``input`` and ``output`` methods.

        Returns
        -------
        None
            The adapter is installed in the process-local registry.
        """
        if not name or not hasattr(adapter, "input") or not hasattr(adapter, "output"):
            raise TypeError("adapter must provide input() and output()")
        cls._adapters[name] = adapter

    @classmethod
    def input(
        cls, values: object, *, adapter: object = "numpy", column: object = None
    ) -> object:
        """Convert an input container through a registered adapter.

        Parameters
        ----------
        values : object
            Array-like data or a supported table/series container.
        adapter : str, optional
            Registered adapter name.
        column : str, optional
            Column selected from a multi-column container.

        Returns
        -------
        object
            Normalized input accepted by the execution pipeline.
        """
        try:
            adapter_cls = cls._adapters[adapter]
        except KeyError as error:
            raise ValueError(f"unknown adapter: {adapter}") from error
        return adapter_cls.input(values, column=column)

    @classmethod
    def output(
        cls, values: object, *, adapter: object = "numpy", **kwargs: object
    ) -> object:
        """Convert values to a registered output container.

        Parameters
        ----------
        values : object
            Aligned values produced by an indicator or pipeline.
        adapter : str, optional
            Registered adapter name.
        **kwargs : object
            Adapter-specific output options.

        Returns
        -------
        object
            Values represented by the requested output container.
        """
        try:
            adapter_cls = cls._adapters[adapter]
        except KeyError as error:
            raise ValueError(f"unknown adapter: {adapter}") from error
        return adapter_cls.output(values, **kwargs)
