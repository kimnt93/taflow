use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{GoldenPocket as State, GoldenPocketValue};

#[pyclass]
pub struct GoldenPocket {
    inner: State,
    lower: Vec<f64>,
    midpoint: Vec<f64>,
    upper: Vec<f64>,
}

#[pymethods]
impl GoldenPocket {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            lower: Vec::new(),
            midpoint: Vec::new(),
            upper: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64)> {
        let result = self.inner.append(high, low);
        let value = result.unwrap_or(GoldenPocketValue {
            low: f64::NAN,
            mid: f64::NAN,
            high: f64::NAN,
        });
        self.lower.push(value.low);
        self.midpoint.push(value.mid);
        self.upper.push(value.high);
        result.map(|value| (value.low, value.mid, value.high))
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
            PyArray1::from_vec(py, self.lower.clone()),
            PyArray1::from_vec(py, self.midpoint.clone()),
            PyArray1::from_vec(py, self.upper.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.low, value.mid, value.high))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.lower.clear();
        self.midpoint.clear();
        self.upper.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
