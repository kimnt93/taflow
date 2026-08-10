use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{FibonacciChannel as State, FibonacciChannelValue};

#[pyclass]
pub struct FibonacciChannel {
    inner: State,
    lower: Vec<f64>,
    retracement_382: Vec<f64>,
    retracement_618: Vec<f64>,
    upper: Vec<f64>,
}

#[pymethods]
impl FibonacciChannel {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            lower: Vec::new(),
            retracement_382: Vec::new(),
            retracement_618: Vec::new(),
            upper: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(FibonacciChannelValue {
            base: f64::NAN,
            level_618: f64::NAN,
            level_1000: f64::NAN,
            level_1618: f64::NAN,
        });
        self.lower.push(value.base);
        self.retracement_382.push(value.level_618);
        self.retracement_618.push(value.level_1000);
        self.upper.push(value.level_1618);
        result.map(|value| {
            (
                value.base,
                value.level_618,
                value.level_1000,
                value.level_1618,
            )
        })
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        if high.len() != low.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "high and low inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for index in 0..high.len() {
                self.append(high[index], low[index]);
            }
        });
        Ok(())
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.lower.clone()),
            PyArray1::from_vec(py, self.retracement_382.clone()),
            PyArray1::from_vec(py, self.retracement_618.clone()),
            PyArray1::from_vec(py, self.upper.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.base,
                value.level_618,
                value.level_1000,
                value.level_1618,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.lower.clear();
        self.retracement_382.clear();
        self.retracement_618.clear();
        self.upper.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
