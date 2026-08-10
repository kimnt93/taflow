use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{FibonacciConfluence as State, FibonacciConfluenceValue};

#[pyclass]
pub struct FibonacciConfluence {
    inner: State,
    retracement: Vec<f64>,
    extension: Vec<f64>,
}

#[pymethods]
impl FibonacciConfluence {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            retracement: Vec::new(),
            extension: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(FibonacciConfluenceValue {
            price: f64::NAN,
            strength: f64::NAN,
        });
        self.retracement.push(value.price);
        self.extension.push(value.strength);
        result.map(|value| (value.price, value.strength))
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
            PyArray1::from_vec(py, self.retracement.clone()),
            PyArray1::from_vec(py, self.extension.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.price, value.strength))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.retracement.clear();
        self.extension.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
