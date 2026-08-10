use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::BatPattern as State;

#[pyclass]
pub struct BatPattern {
    inner: State,
    output: Vec<f64>,
}

#[pymethods]
impl BatPattern {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let value = self.inner.append(open, high, low, close);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let open = open.as_slice()?;
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        let length = open.len();
        if high.len() != length || low.len() != length || close.len() != length {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "OHLC inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for index in 0..length {
                self.append(open[index], high[index], low[index], close[index]);
            }
        });
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
