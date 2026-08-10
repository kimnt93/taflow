use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{FibonacciProjection as State, FibonacciProjectionValue};

#[pyclass]
pub struct FibonacciProjection {
    inner: State,
    projection_100: Vec<f64>,
    projection_1272: Vec<f64>,
    projection_1618: Vec<f64>,
    projection_200: Vec<f64>,
}

#[pymethods]
impl FibonacciProjection {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            projection_100: Vec::new(),
            projection_1272: Vec::new(),
            projection_1618: Vec::new(),
            projection_200: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(FibonacciProjectionValue {
            projection_100: f64::NAN,
            projection_1272: f64::NAN,
            projection_1618: f64::NAN,
            projection_200: f64::NAN,
        });
        self.projection_100.push(value.projection_100);
        self.projection_1272.push(value.projection_1272);
        self.projection_1618.push(value.projection_1618);
        self.projection_200.push(value.projection_200);
        result.map(|value| {
            (
                value.projection_100,
                value.projection_1272,
                value.projection_1618,
                value.projection_200,
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
            PyArray1::from_vec(py, self.projection_100.clone()),
            PyArray1::from_vec(py, self.projection_1272.clone()),
            PyArray1::from_vec(py, self.projection_1618.clone()),
            PyArray1::from_vec(py, self.projection_200.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.projection_100,
                value.projection_1272,
                value.projection_1618,
                value.projection_200,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.projection_100.clear();
        self.projection_1272.clear();
        self.projection_1618.clear();
        self.projection_200.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
