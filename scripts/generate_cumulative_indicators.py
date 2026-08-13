"""Generate canonical cumulative state implementations and adapters."""

from __future__ import annotations

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CORE = ROOT / "crates" / "taflow-core" / "src" / "stream"
PYROOT = ROOT / "python" / "taflow"
TESTS = ROOT / "tests"
NATIVE = ROOT / "crates" / "taflow-python" / "src" / "indicators"

# class, module, state field, initial value, update, Polars expression
INDICATORS = (
    ("CumulativeSum", "cumulative_sum", "total", "0.0", "self.total += input", "cum_sum"),
    ("CumulativeProduct", "cumulative_product", "total", "1.0", "self.total *= input", "cum_prod"),
    ("CumulativeMaximum", "cumulative_maximum", "extreme", "f64::NEG_INFINITY", "self.extreme = self.extreme.max(input)", "cum_max"),
    ("CumulativeMinimum", "cumulative_minimum", "extreme", "f64::INFINITY", "self.extreme = self.extreme.min(input)", "cum_min"),
)


RUST = '''//! Persistent {label} state.

use super::StreamingIndicator;
use crate::error::TaResult;

/// Compute the {label} of chronological scalar observations.
#[derive(Debug, Clone)]
pub struct {class_name} {{
    {field}: f64,
    value: Option<f64>,
}}

impl {class_name} {{
    /// Create a fresh cumulative state.
    pub fn new() -> TaResult<Self> {{
        Ok(Self::default())
    }}

    /// Append one value and return the current cumulative result.
    pub fn append(&mut self, input: f64) -> f64 {{
        {update};
        self.value = Some(self.{field});
        self.{field}
    }}

    /// Append a slice into `output` in scalar replay order.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {{
        output.reserve(input.len());
        output.extend(input.iter().map(|&input| self.append(input)));
    }}

    /// Return the latest result, or `None` before the first observation.
    pub fn value(&self) -> Option<f64> {{
        self.value
    }}

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {{
        self.{field} = {initial};
        self.value = None;
    }}
}}

impl Default for {class_name} {{
    fn default() -> Self {{
        Self {{
            {field}: {initial},
            value: None,
        }}
    }}
}}

impl StreamingIndicator for {class_name} {{
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<Self::Output> {{
        Some(Self::append(self, input))
    }}

    fn value(&self) -> Option<Self::Output> {{
        Self::value(self)
    }}

    fn reset(&mut self) {{
        Self::reset(self);
    }}
}}
'''


RUST_TEST = '''use super::{module}::{class_name};

#[test]
fn scalar_bulk_and_reset_are_bitwise_identical() {{
    let input = [2.0_f64, 4.0, 1.0, 8.0, 2.0];
    let mut scalar_state = {class_name}::new().unwrap();
    assert!(scalar_state.value().is_none());
    let scalar: Vec<_> = input
        .iter()
        .map(|&input| scalar_state.append(input))
        .collect();
    let final_value = scalar_state.value();

    scalar_state.reset();
    assert!(scalar_state.value().is_none());
    let replay: Vec<_> = input
        .iter()
        .map(|&input| scalar_state.append(input))
        .collect();
    assert_eq!(scalar, replay);

    let mut bulk_state = {class_name}::new().unwrap();
    let mut bulk = Vec::new();
    bulk_state.extend_slice_into(&input[..2], &mut bulk);
    bulk_state.extend_slice_into(&input[2..], &mut bulk);
    assert_eq!(bulk, scalar);
    assert_eq!(bulk_state.value(), final_value);
}}
'''


PYTHON = '''"""Persistent {label} indicator."""

from typing import Any

import numpy as np

from ._native import {class_name} as _Native{class_name}
from ._series import as_float64_series


class {class_name}:
    """Compute the {label} in persistent Rust state.

    Construct with no arguments, then supply the chronological numeric series
    through ``extend`` or ``append``. There is no warm-up. Correctness maps to
    the Polars ``Series.{polars_method}`` expression.
    """

    def __init__(self) -> None:
        self._state = _Native{class_name}()

    def append(self, _input: float) -> "{class_name}":
        """Append one observation and return this indicator."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "{class_name}":
        """Append chronological observations and return this indicator."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned ``float64`` history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest result, or ``None`` before the first value."""
        return self._state.value

    def reset(self) -> "{class_name}":
        """Restore fresh native state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return len(self._state)
'''


PYTHON_TEST = '''import numpy as np
import polars as pl

from taflow import {class_name}


def test_{module}_matches_polars_and_lifecycle() -> None:
    values = np.array([2.0, 4.0, 1.0, 8.0, 2.0, -3.0, 5.0], dtype=np.float64)
    expected = pl.Series(values).{polars_method}().to_numpy()
    np.testing.assert_array_equal({class_name}(values).compute(), expected)

    state = {class_name}([])
    assert state.value is None
    assert state.extend(values[:3]) is state
    assert state.extend(values[3:]) is state
    np.testing.assert_array_equal(state.compute(), expected)
    assert state.reset() is state
    for value in values:
        assert state.append(float(value)) is state
    np.testing.assert_array_equal(state.compute(), expected)
'''


NATIVE_ADAPTER = '''use numpy::{{PyArray1, PyReadonlyArray1}};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{class_name} as {class_name}State;

#[pyclass]
pub struct {class_name} {{
    inner: {class_name}State,
    outputs: Vec<f64>,
}}

#[pymethods]
impl {class_name} {{
    #[new]
    fn new() -> PyResult<Self> {{
        Ok(Self {{
            inner: {class_name}State::new()
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            outputs: Vec::new(),
        }})
    }}

    fn append(&mut self, input: f64) -> f64 {{
        let value = self.inner.append(input);
        self.outputs.push(value);
        value
    }}

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {{
        let input = input.as_slice()?;
        py.allow_threads(|| self.inner.extend_slice_into(input, &mut self.outputs));
        Ok(())
    }}

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {{
        PyArray1::from_vec(py, self.outputs.clone())
    }}

    #[getter]
    fn value(&self) -> Option<f64> {{
        self.inner.value()
    }}

    fn reset(&mut self) {{
        self.inner.reset();
        self.outputs.clear();
    }}

    fn __len__(&self) -> usize {{
        self.outputs.len()
    }}
}}
'''


def main() -> None:
    for class_name, module, field, initial, update, polars_method in INDICATORS:
        label = module.replace("_", " ")
        (CORE / f"{module}.rs").write_text(RUST.format(
            label=label, class_name=class_name, field=field, initial=initial,
            update=update,
        ))
        (CORE / f"{module}_test.rs").write_text(RUST_TEST.format(
            module=module, class_name=class_name,
        ))
        (PYROOT / f"{module}.py").write_text(PYTHON.format(
            label=label, class_name=class_name, polars_method=polars_method,
        ))
        (TESTS / f"{module}_test.py").write_text(PYTHON_TEST.format(
            module=module, class_name=class_name, polars_method=polars_method,
        ))
        (NATIVE / f"{module}.rs").write_text(NATIVE_ADAPTER.format(
            class_name=class_name,
        ))


if __name__ == "__main__":
    main()
