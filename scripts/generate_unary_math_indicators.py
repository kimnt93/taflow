"""Generate the explicit one-class-per-file unary math indicator family."""

from __future__ import annotations

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CORE = ROOT / "crates" / "taflow-core" / "src" / "stream"
PYTHON = ROOT / "python" / "taflow"
TESTS = ROOT / "tests"

# class, module, Rust expression, public oracle, Python domain expression
INDICATORS = (
    ("MathAbs", "math_abs", "input.abs()", "np.abs", "np.linspace(-8.0, 8.0, 127)"),
    ("MathAcos", "math_acos", "input.acos()", "talib.ACOS", "np.linspace(-0.95, 0.95, 127)"),
    ("MathAcosh", "math_acosh", "input.acosh()", "np.arccosh", "np.linspace(1.0, 12.0, 127)"),
    ("MathAsin", "math_asin", "input.asin()", "talib.ASIN", "np.linspace(-0.95, 0.95, 127)"),
    ("MathAsinh", "math_asinh", "input.asinh()", "np.arcsinh", "np.linspace(-8.0, 8.0, 127)"),
    ("MathAtan", "math_atan", "input.atan()", "talib.ATAN", "np.linspace(-8.0, 8.0, 127)"),
    ("MathAtanh", "math_atanh", "input.atanh()", "np.arctanh", "np.linspace(-0.95, 0.95, 127)"),
    ("MathCbrt", "math_cbrt", "input.cbrt()", "np.cbrt", "np.linspace(-64.0, 64.0, 127)"),
    ("MathCeil", "math_ceil", "input.ceil()", "talib.CEIL", "np.linspace(-8.25, 8.25, 127)"),
    ("MathCos", "math_cos", "input.cos()", "talib.COS", "np.linspace(-6.0, 6.0, 127)"),
    ("MathCosh", "math_cosh", "input.cosh()", "talib.COSH", "np.linspace(-3.0, 3.0, 127)"),
    ("MathCot", "math_cot", "input.tan().recip()", "lambda values: 1.0 / np.tan(values)", "np.linspace(0.1, 3.0, 127)"),
    ("MathDegrees", "math_degrees", "input.to_degrees()", "np.degrees", "np.linspace(-6.0, 6.0, 127)"),
    ("MathExp", "math_exp", "input.exp()", "talib.EXP", "np.linspace(-3.0, 3.0, 127)"),
    ("MathFloor", "math_floor", "input.floor()", "talib.FLOOR", "np.linspace(-8.25, 8.25, 127)"),
    ("MathLn", "math_ln", "input.ln()", "talib.LN", "np.linspace(0.1, 12.0, 127)"),
    ("MathLog10", "math_log10", "input.log10()", "talib.LOG10", "np.linspace(0.1, 12.0, 127)"),
    ("MathLog1p", "math_log1p", "input.ln_1p()", "np.log1p", "np.linspace(-0.9, 12.0, 127)"),
    ("MathRadians", "math_radians", "input.to_radians()", "np.radians", "np.linspace(-720.0, 720.0, 127)"),
    ("MathSin", "math_sin", "input.sin()", "talib.SIN", "np.linspace(-6.0, 6.0, 127)"),
    ("MathSinh", "math_sinh", "input.sinh()", "talib.SINH", "np.linspace(-3.0, 3.0, 127)"),
    ("MathSqrt", "math_sqrt", "input.sqrt()", "talib.SQRT", "np.linspace(0.0, 64.0, 127)"),
    ("MathTan", "math_tan", "input.tan()", "talib.TAN", "np.linspace(-1.0, 1.0, 127)"),
    ("MathTanh", "math_tanh", "input.tanh()", "talib.TANH", "np.linspace(-6.0, 6.0, 127)"),
)


RUST_IMPLEMENTATION = """//! Persistent pointwise `{operation_name}` transform.

use super::StreamingIndicator;
use crate::error::TaResult;

/// Apply `{operation_name}` to each value without warm-up.
#[derive(Debug, Clone, Default)]
pub struct {class_name} {{
    value: Option<f64>,
}}

impl {class_name} {{
    /// Create a fresh pointwise transform state.
    pub fn new() -> TaResult<Self> {{
        Ok(Self::default())
    }}

    /// Transform one chronological value.
    pub fn append(&mut self, input: f64) -> Option<f64> {{
        self.value = Some({rust_expression});
        self.value
    }}

    /// Transform a slice into `output` while preserving scalar replay order.
    pub fn extend_slice_into(&mut self, input: &[f64], output: &mut Vec<f64>) {{
        output.reserve(input.len());
        output.extend(input.iter().map(|&input| {{
            self.append(input)
                .expect("pointwise transforms have no warm-up")
        }}));
    }}

    /// Return the latest result, or `None` before the first value.
    pub fn value(&self) -> Option<f64> {{
        self.value
    }}

    /// Restore fresh-state behavior without reallocating.
    pub fn reset(&mut self) {{
        self.value = None;
    }}
}}

impl StreamingIndicator for {class_name} {{
    type Output = f64;

    fn append(&mut self, input: f64) -> Option<Self::Output> {{
        Self::append(self, input)
    }}

    fn value(&self) -> Option<Self::Output> {{
        Self::value(self)
    }}

    fn reset(&mut self) {{
        Self::reset(self);
    }}
}}
"""


RUST_TEST = """use super::{module_name}::{class_name};

#[test]
fn lifecycle_and_bulk_are_consistent() {{
    let input = [{rust_values}];
    let expected: Vec<_> = input.iter().map(|&input| {rust_expression}).collect();
    let mut state = {class_name}::new().unwrap();
    assert!(state.value().is_none());
    let scalar: Vec<_> = input
        .iter()
        .map(|&input| state.append(input).unwrap())
        .collect();
    for (actual, expected) in scalar.iter().zip(&expected) {{
        assert_eq!(actual.to_bits(), expected.to_bits());
    }}
    assert_eq!(state.value().unwrap().to_bits(), expected.last().unwrap().to_bits());
    state.reset();
    assert!(state.value().is_none());
    let mut bulk = Vec::new();
    state.extend_slice_into(&input, &mut bulk);
    for (actual, expected) in bulk.iter().zip(&expected) {{
        assert_eq!(actual.to_bits(), expected.to_bits());
    }}
}}
"""


PYTHON_ADAPTER = '''"""Persistent pointwise {operation_name} transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import {class_name} as _Native{class_name}


class {class_name}(MathUnaryState):
    """Apply pointwise {operation_name} in persistent Rust state.

    Construct with no arguments, then supply chronological values through
    ``extend`` or ``append``.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``{oracle_name}``.
    """

    _native_cls = _Native{class_name}

    def append(self, _input: float) -> "{class_name}":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "{class_name}":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "{class_name}":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
'''


PYTHON_TEST = """import numpy as np
{talib_import}
from taflow import {class_name}


def test_{module_name}_matches_{oracle_slug}_and_lifecycle() -> None:
    rng = np.random.default_rng(104729)
    primary = {domain_expression}
    datasets = (
        primary[:1],
        primary,
        np.full(17, primary[len(primary) // 2]),
        rng.choice(primary, size=211, replace=True),
    )
    for values in datasets:
        expected = oracle(values)
        actual = {class_name}(values)
        np.testing.assert_allclose(
            actual.compute(), expected, rtol=1e-12, atol=1e-12, equal_nan=True
        )

        state = {class_name}([])
        split = len(values) // 3
        assert state.extend(values[:split]) is state
        assert state.extend(values[split:]) is state
        np.testing.assert_allclose(
            state.compute(), expected, rtol=1e-12, atol=1e-12, equal_nan=True
        )
        assert state.value == state.compute()[-1]
        assert state.reset() is state
        for value in values:
            assert state.append(float(value)) is state
        np.testing.assert_allclose(
            state.compute(), expected, rtol=1e-12, atol=1e-12, equal_nan=True
        )

    fresh = {class_name}([])
    assert len(fresh) == 0
    assert fresh.value is None
"""


def main() -> None:
    rust_values = "-0.75_f64, -0.25, 0.25, 0.75"
    for class_name, module_name, rust_expression, oracle, domain in INDICATORS:
        operation_name = module_name.removeprefix("math_").replace("_", " ")
        (CORE / f"{module_name}.rs").write_text(
            RUST_IMPLEMENTATION.format(
                operation_name=operation_name,
                class_name=class_name,
                rust_expression=rust_expression,
            )
        )
        (CORE / f"{module_name}_test.rs").write_text(
            RUST_TEST.format(
                module_name=module_name,
                class_name=class_name,
                rust_expression=rust_expression,
                rust_values=rust_values,
            )
        )
        (PYTHON / f"{module_name}.py").write_text(
            PYTHON_ADAPTER.format(
                operation_name=operation_name,
                class_name=class_name,
                oracle_name=oracle,
            )
        )
        uses_talib = oracle.startswith("talib.")
        oracle_expression = oracle if uses_talib else oracle
        (TESTS / f"{module_name}_test.py").write_text(
            PYTHON_TEST.format(
                talib_import="import talib\n" if uses_talib else "",
                class_name=class_name,
                module_name=module_name,
                oracle_slug="talib" if uses_talib else "numpy",
                domain_expression=domain,
            ).replace("expected = oracle(values)", f"expected = ({oracle_expression})(values)")
        )


if __name__ == "__main__":
    main()
