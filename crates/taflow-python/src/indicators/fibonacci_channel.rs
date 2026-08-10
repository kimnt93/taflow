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
            lower: f64::NAN,
            retracement_382: f64::NAN,
            retracement_618: f64::NAN,
            upper: f64::NAN,
        });
        self.lower.push(value.lower);
        self.retracement_382.push(value.retracement_382);
        self.retracement_618.push(value.retracement_618);
        self.upper.push(value.upper);
        result.map(|value| {
            (
                value.lower,
                value.retracement_382,
                value.retracement_618,
                value.upper,
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
                value.lower,
                value.retracement_382,
                value.retracement_618,
                value.upper,
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
