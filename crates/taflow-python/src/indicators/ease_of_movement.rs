use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::EaseOfMovement;

#[pyclass]
pub struct EaseOfMovementOperator {
    inner: EaseOfMovement,
    outputs: Vec<f64>,
}

#[pymethods]
impl EaseOfMovementOperator {
    #[new]
    fn new(period: usize, divisor: f64) -> PyResult<Self> {
        Ok(Self {
            inner: EaseOfMovement::new(period, divisor)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, volume: f64) -> Option<f64> {
        let value = self.inner.append(high, low, volume);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (h, l, v) = (high.as_slice()?, low.as_slice()?, volume.as_slice()?);
        if h.len() != l.len() || h.len() != v.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for ((&h, &l), &v) in h.iter().zip(l).zip(v) {
                self.append(h, l, v);
            }
        });
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
