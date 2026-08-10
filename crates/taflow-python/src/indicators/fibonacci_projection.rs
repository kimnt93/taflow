use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{FibonacciProjection as State, FibonacciProjectionValue};

#[pyclass]
pub struct FibonacciProjection {
    inner: State,
    projection_618: Vec<f64>,
    projection_1000: Vec<f64>,
    projection_1618: Vec<f64>,
    projection_2618: Vec<f64>,
}

#[pymethods]
impl FibonacciProjection {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            projection_618: Vec::new(),
            projection_1000: Vec::new(),
            projection_1618: Vec::new(),
            projection_2618: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(FibonacciProjectionValue {
            projection_618: f64::NAN,
            projection_1000: f64::NAN,
            projection_1618: f64::NAN,
            projection_2618: f64::NAN,
        });
        self.projection_618.push(value.projection_618);
        self.projection_1000.push(value.projection_1000);
        self.projection_1618.push(value.projection_1618);
        self.projection_2618.push(value.projection_2618);
        result.map(|value| {
            (
                value.projection_618,
                value.projection_1000,
                value.projection_1618,
                value.projection_2618,
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
            PyArray1::from_vec(py, self.projection_618.clone()),
            PyArray1::from_vec(py, self.projection_1000.clone()),
            PyArray1::from_vec(py, self.projection_1618.clone()),
            PyArray1::from_vec(py, self.projection_2618.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.projection_618,
                value.projection_1000,
                value.projection_1618,
                value.projection_2618,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.projection_618.clear();
        self.projection_1000.clear();
        self.projection_1618.clear();
        self.projection_2618.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
