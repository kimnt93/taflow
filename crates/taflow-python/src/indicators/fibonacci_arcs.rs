use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{FibonacciArcs as State, FibonacciArcsValue};

#[pyclass]
pub struct FibonacciArcs {
    inner: State,
    radius_382: Vec<f64>,
    radius_500: Vec<f64>,
    radius_618: Vec<f64>,
}

#[pymethods]
impl FibonacciArcs {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            radius_382: Vec::new(),
            radius_500: Vec::new(),
            radius_618: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(FibonacciArcsValue {
            arc_382: f64::NAN,
            arc_500: f64::NAN,
            arc_618: f64::NAN,
        });
        self.radius_382.push(value.arc_382);
        self.radius_500.push(value.arc_500);
        self.radius_618.push(value.arc_618);
        result.map(|value| (value.arc_382, value.arc_500, value.arc_618))
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
    ) {
        (
            PyArray1::from_vec(py, self.radius_382.clone()),
            PyArray1::from_vec(py, self.radius_500.clone()),
            PyArray1::from_vec(py, self.radius_618.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.arc_382, value.arc_500, value.arc_618))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.radius_382.clear();
        self.radius_500.clear();
        self.radius_618.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
