"""Generate the explicit one-class-per-file price-transform family."""

from __future__ import annotations

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CORE = ROOT / "crates" / "taflow-core" / "src" / "stream"
PYTHON = ROOT / "python" / "taflow"
TESTS = ROOT / "tests"

# class, module, ordered inputs, Rust expression, Python base, TA-Lib oracle
INDICATORS = (
    ("AveragePrice", "average_price", ("open", "high", "low", "close"),
     "(open + high + low + close) * 0.25", "OhlcPriceState", "AVGPRICE"),
    ("MedianPrice", "median_price", ("high", "low"),
     "(high + low) * 0.5", "HlPriceState", "MEDPRICE"),
    ("TypicalPrice", "typical_price", ("high", "low", "close"),
     "(high + low + close) * (1.0 / 3.0)", "HlcPriceState", "TYPPRICE"),
    ("WeightedClose", "weighted_close", ("high", "low", "close"),
     "(high + low + close + close) * 0.25", "HlcPriceState", "WCLPRICE"),
)


def rust_implementation(class_name: str, module: str, inputs: tuple[str, ...],
                        expression: str) -> str:
    append_args = ", ".join(f"{name}: f64" for name in inputs)
    slices = ",\n        ".join(f"{name}: &[f64]" for name in inputs)
    validation = "\n".join(
        f"        if {name}.len() != len {{\n"
        f"            return Err(TaError::LengthMismatch {{\n"
        f"                expected: len,\n                got: {name}.len(),\n"
        f"            }});\n        }}"
        for name in inputs[1:]
    )
    indexed = ", ".join(f"{name}[index]" for name in inputs)
    label = module.replace("_", " ")
    return f'''//! Persistent pointwise `{label}` transform.

use crate::error::{{TaError, TaResult}};

/// Compute {label} for aligned chronological prices without warm-up.
#[derive(Debug, Clone, Default)]
pub struct {class_name} {{
    value: Option<f64>,
}}

impl {class_name} {{
    /// Create a fresh price-transform state.
    pub fn new() -> TaResult<Self> {{
        Ok(Self::default())
    }}

    /// Transform one chronological price tuple.
    pub fn append(&mut self, {append_args}) -> f64 {{
        let value = {expression};
        self.value = Some(value);
        value
    }}

    /// Transform aligned slices after validating every length before mutation.
    pub fn extend_slices_into(
        &mut self,
        {slices},
        output: &mut Vec<f64>,
    ) -> TaResult<()> {{
        let len = {inputs[0]}.len();
{validation}
        output.reserve(len);
        for index in 0..len {{
            output.push(self.append({indexed}));
        }}
        Ok(())
    }}

    /// Return the latest result, or `None` before the first price tuple.
    pub fn value(&self) -> Option<f64> {{
        self.value
    }}

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {{
        self.value = None;
    }}
}}
'''


def rust_test(class_name: str, module: str, inputs: tuple[str, ...],
              expression: str) -> str:
    arrays = {
        "open": "[10.0_f64, 11.0, 12.0, 13.0]",
        "high": "[12.0_f64, 13.0, 14.0, 15.0]",
        "low": "[8.0_f64, 9.0, 10.0, 11.0]",
        "close": "[11.0_f64, 12.0, 13.0, 14.0]",
    }
    declarations = "\n".join(f"    let {name} = {arrays[name]};" for name in inputs)
    indexed = ", ".join(f"{name}[index]" for name in inputs)
    refs = ", ".join(f"&{name}" for name in inputs)
    bad_refs = ", ".join(
        f"&{name}[..3]" if index == len(inputs) - 1 else f"&{name}"
        for index, name in enumerate(inputs)
    )
    return f'''use super::{module}::{class_name};

#[test]
fn lifecycle_bulk_and_validation_are_consistent() {{
{declarations}
    let expected: Vec<_> = (0..{inputs[0]}.len())
        .map(|index| {{
            {"; ".join(f"let {name} = {name}[index]" for name in inputs)};
            {expression}
        }})
        .collect();
    let mut state = {class_name}::new().unwrap();
    assert!(state.value().is_none());
    let scalar: Vec<_> = (0..{inputs[0]}.len())
        .map(|index| state.append({indexed}))
        .collect();
    assert_eq!(scalar, expected);
    assert_eq!(state.value(), expected.last().copied());
    state.reset();
    assert!(state.value().is_none());
    let mut bulk = Vec::new();
    state.extend_slices_into({refs}, &mut bulk).unwrap();
    assert_eq!(bulk, expected);
    let before = bulk.clone();
    assert!(state.extend_slices_into({bad_refs}, &mut bulk).is_err());
    assert_eq!(bulk, before);
}}
'''


def python_adapter(class_name: str, module: str, inputs: tuple[str, ...],
                   base: str, oracle: str) -> str:
    signature = ", ".join(f"{name}: float" for name in inputs)
    bulk_signature = ", ".join(f"{name}: Any" for name in inputs)
    args = ", ".join(inputs)
    formula = {
        "AveragePrice": "(open + high + low + close) / 4",
        "MedianPrice": "(high + low) / 2",
        "TypicalPrice": "(high + low + close) / 3",
        "WeightedClose": "(high + low + 2 * close) / 4",
    }[class_name]
    return f'''"""Persistent {module.replace("_", "-")} transform."""

from typing import Any

from ._native import {class_name} as _Native{class_name}
from ._price_state import {base}


class {class_name}({base}):
    """Compute ``{formula}`` in persistent Rust state.

    The constructor requires the aligned chronological {", ".join(inputs)}
    series. Pass empty aligned arrays for a fresh streaming state. Output has no
    rolling warm-up and maps to TA-Lib ``{oracle}``.
    """

    _native_cls = _Native{class_name}

    def append(self, {signature}) -> "{class_name}":
        """Append one aligned price tuple and return this indicator."""
        super().append({args})
        return self

    def extend(self, {bulk_signature}) -> "{class_name}":
        """Append aligned price histories and return this indicator."""
        super().extend({args})
        return self

    def reset(self) -> "{class_name}":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
'''


def python_test(class_name: str, module: str, inputs: tuple[str, ...],
                oracle: str) -> str:
    construction = ", ".join(inputs)
    empties = ", ".join("[]" for _ in inputs)
    first = ", ".join(f"{name}[:43]" for name in inputs)
    rest = ", ".join(f"{name}[43:]" for name in inputs)
    scalar = ", ".join(f"float({name}[index])" for name in inputs)
    oracle_args = ", ".join(inputs)
    return f'''import numpy as np
import pytest
import talib

from taflow import {class_name}


def test_{module}_matches_talib_and_lifecycle() -> None:
    rng = np.random.default_rng(16127)
    close = 100.0 + np.cumsum(rng.normal(0.0, 0.5, 128))
    high = close + rng.uniform(0.1, 2.0, 128)
    low = close - rng.uniform(0.1, 2.0, 128)
    open = low + rng.random(128) * (high - low)
    expected = talib.{oracle}({oracle_args})
    actual = {class_name}({construction})
    np.testing.assert_allclose(actual.compute(), expected, rtol=1e-12, atol=1e-12)

    state = {class_name}({empties})
    assert state.extend({first}) is state
    assert state.extend({rest}) is state
    np.testing.assert_allclose(state.compute(), expected, rtol=1e-12, atol=1e-12)
    assert state.reset() is state
    for index in range(len(close)):
        assert state.append({scalar}) is state
    np.testing.assert_allclose(state.compute(), expected, rtol=1e-12, atol=1e-12)

    fresh = {class_name}({empties})
    assert len(fresh) == 0
    assert fresh.value is None
    with pytest.raises(ValueError):
        fresh.extend({", ".join("[1.0, 2.0]" if i == 0 else "[1.0]" for i in range(len(inputs)))})
    assert len(fresh) == 0
'''


def main() -> None:
    for class_name, module, inputs, expression, base, oracle in INDICATORS:
        (CORE / f"{module}.rs").write_text(
            rust_implementation(class_name, module, inputs, expression)
        )
        (CORE / f"{module}_test.rs").write_text(
            rust_test(class_name, module, inputs, expression)
        )
        (PYTHON / f"{module}.py").write_text(
            python_adapter(class_name, module, inputs, base, oracle)
        )
        (TESTS / f"{module}_test.py").write_text(
            python_test(class_name, module, inputs, oracle)
        )


if __name__ == "__main__":
    main()
