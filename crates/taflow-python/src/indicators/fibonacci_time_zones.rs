use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{FibonacciTimeZones as State, FibonacciTimeZonesValue};

#[pyclass]
pub struct FibonacciTimeZones {
    inner: State,
    current_zone: Vec<f64>,
    next_zone: Vec<f64>,
}

#[pymethods]
impl FibonacciTimeZones {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            current_zone: Vec::new(),
            next_zone: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(FibonacciTimeZonesValue {
            on_zone: f64::NAN,
            bars_to_next: f64::NAN,
        });
        self.current_zone.push(value.on_zone);
        self.next_zone.push(value.bars_to_next);
        result.map(|value| (value.on_zone, value.bars_to_next))
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
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.current_zone.clone()),
            PyArray1::from_vec(py, self.next_zone.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.on_zone, value.bars_to_next))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.current_zone.clear();
        self.next_zone.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
