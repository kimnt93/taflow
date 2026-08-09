//! PyO3 adapters for the incremental core API.

use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{
    self, FastStochasticOscillator, StochasticOscillator, StochasticRelativeStrengthIndex,
    StreamingIndicator,
};

use crate::conversion::to_py_array;

pub(crate) fn py_value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}

/// Appends `Option<f64>` results straight into a Rust-side output cache,
/// NaN-filling warm-up. One pass, no temporary `Vec`.
pub(crate) fn extend_from_options<I>(cache: &mut Vec<f64>, values: I)
where
    I: IntoIterator<Item = Option<f64>>,
{
    let values = values.into_iter();
    cache.reserve(values.size_hint().0);
    cache.extend(values.map(|value| value.unwrap_or(f64::NAN)));
}

pub(crate) fn push_option(cache: &mut Vec<f64>, value: Option<f64>) -> Option<f64> {
    cache.push(value.unwrap_or(f64::NAN));
    value
}

macro_rules! scalar_state_class {
    ($class:ident, $inner:ty, $default_period:literal) => {
        #[pyclass]
        pub struct $class {
            inner: $inner,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (timeperiod=$default_period))]
            fn new(timeperiod: usize) -> PyResult<Self> {
                Ok(Self {
                    inner: <$inner>::new(timeperiod).map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, input: f64) -> Option<f64> {
                push_option(&mut self.outputs, self.inner.append(input))
            }

            fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                let input = input.as_slice()?;
                let outputs = &mut self.outputs;
                py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
                Ok(())
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

scalar_state_class!(StatefulMax, stream::RollingMax, 30);
scalar_state_class!(StatefulMaxindex, stream::RollingArgmax, 30);
scalar_state_class!(StatefulMin, stream::RollingMin, 30);
scalar_state_class!(StatefulMinindex, stream::RollingArgmin, 30);
scalar_state_class!(StatefulLinearreg, stream::RollingLinearRegression, 14);
scalar_state_class!(
    StatefulLinearregSlope,
    stream::RollingLinearRegressionSlope,
    14
);
scalar_state_class!(
    StatefulLinearregIntercept,
    stream::RollingLinearRegressionIntercept,
    14
);
scalar_state_class!(
    StatefulLinearregAngle,
    stream::RollingLinearRegressionAngle,
    14
);
scalar_state_class!(StatefulTsf, stream::RollingTimeSeriesForecast, 14);

macro_rules! deviation_state_class {
    ($class:ident, $inner:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$inner,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (timeperiod=5, nbdev=1.0))]
            fn new(timeperiod: usize, nbdev: f64) -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$inner::new(timeperiod, nbdev).map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, input: f64) -> Option<f64> {
                push_option(&mut self.outputs, self.inner.append(input))
            }

            fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                let input = input.as_slice()?;
                let outputs = &mut self.outputs;
                py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
                Ok(())
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

macro_rules! bivariate_statistic_class {
    ($class:ident, $inner:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$inner,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            #[pyo3(signature = (timeperiod=5))]
            fn new(timeperiod: usize) -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$inner::new(timeperiod).map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, input0: f64, input1: f64) -> Option<f64> {
                push_option(&mut self.outputs, self.inner.append(input0, input1))
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                input0: PyReadonlyArray1<f64>,
                input1: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let input0 = input0.as_slice()?;
                let input1 = input1.as_slice()?;
                if input0.len() != input1.len() {
                    return Err(PyValueError::new_err("inputs must have equal lengths"));
                }
                let outputs = &mut self.outputs;
                let inner = &mut self.inner;
                py.allow_threads(|| inner.extend_slices_into(input0, input1, outputs))
                    .map_err(py_value_error)
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

bivariate_statistic_class!(StatefulBeta, RollingBeta);
bivariate_statistic_class!(StatefulCorrel, RollingCorrelation);

macro_rules! unary_state_class {
    ($class:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$class,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$class::new().map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, input: f64) -> f64 {
                let value = self
                    .inner
                    .append(input)
                    .expect("stateless transform is warm");
                self.outputs.push(value);
                value
            }

            fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                let input = input.as_slice()?;
                let outputs = &mut self.outputs;
                py.allow_threads(|| self.inner.extend_slice_into(input, outputs));
                Ok(())
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

unary_state_class!(MathAbs);
unary_state_class!(MathAcos);
unary_state_class!(MathAcosh);
unary_state_class!(MathAsin);
unary_state_class!(MathAsinh);
unary_state_class!(MathAtan);
unary_state_class!(MathAtanh);
unary_state_class!(MathCbrt);
unary_state_class!(MathCeil);
unary_state_class!(MathCos);
unary_state_class!(MathCosh);
unary_state_class!(MathCot);
unary_state_class!(MathDegrees);
unary_state_class!(MathExp);
unary_state_class!(MathFloor);
unary_state_class!(MathLn);
unary_state_class!(MathLog10);
unary_state_class!(MathLog1p);
unary_state_class!(MathRadians);
unary_state_class!(MathSin);
unary_state_class!(MathSinh);
unary_state_class!(MathSqrt);
unary_state_class!(MathTan);
unary_state_class!(MathTanh);

macro_rules! binary_math_state_class {
    ($class:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$class,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$class::new().map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, left: f64, right: f64) -> f64 {
                let value = self.inner.append(left, right);
                self.outputs.push(value);
                value
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                left: PyReadonlyArray1<f64>,
                right: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let left = left.as_slice()?;
                let right = right.as_slice()?;
                let inner = &mut self.inner;
                let outputs = &mut self.outputs;
                py.allow_threads(|| inner.extend_slices_into(left, right, outputs))
                    .map_err(py_value_error)
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

binary_math_state_class!(MathAdd);
binary_math_state_class!(MathSubtract);
binary_math_state_class!(MathMultiply);
binary_math_state_class!(MathDivide);

macro_rules! binary_state_class {
    ($class:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$class,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$class::new().map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, left: f64, right: f64) -> f64 {
                let value = self.inner.append(left, right);
                self.outputs.push(value);
                value
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                left: PyReadonlyArray1<f64>,
                right: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let left = left.as_slice()?;
                let right = right.as_slice()?;
                py.allow_threads(|| {
                    self.inner
                        .extend_slices_into(left, right, &mut self.outputs)
                })
                .map_err(py_value_error)
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

binary_state_class!(MedianPrice);

macro_rules! price3_state_class {
    ($class:ident) => {
        #[pyclass]
        pub struct $class {
            inner: stream::$class,
            outputs: Vec<f64>,
        }

        #[pymethods]
        impl $class {
            #[new]
            fn new() -> PyResult<Self> {
                Ok(Self {
                    inner: stream::$class::new().map_err(py_value_error)?,
                    outputs: Vec::new(),
                })
            }

            fn append(&mut self, high: f64, low: f64, close: f64) -> f64 {
                let value = self.inner.append(high, low, close);
                self.outputs.push(value);
                value
            }

            fn extend(
                &mut self,
                py: Python<'_>,
                high: PyReadonlyArray1<f64>,
                low: PyReadonlyArray1<f64>,
                close: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let high = high.as_slice()?;
                let low = low.as_slice()?;
                let close = close.as_slice()?;
                py.allow_threads(|| {
                    self.inner
                        .extend_slices_into(high, low, close, &mut self.outputs)
                })
                .map_err(py_value_error)
            }

            #[getter]
            fn value(&self) -> Option<f64> {
                self.inner.value()
            }

            fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
                to_py_array(py, self.outputs.clone())
            }

            fn __len__(&self) -> usize {
                self.outputs.len()
            }

            fn reset(&mut self) {
                self.inner.reset();
                self.outputs.clear();
            }
        }
    };
}

price3_state_class!(TypicalPrice);
price3_state_class!(WeightedClose);
