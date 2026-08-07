use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::BosChoch;

#[pyclass]
pub struct BosChochOperator {
    inner: BosChoch,
    bos: Vec<f64>,
    choch: Vec<f64>,
    level: Vec<f64>,
    broken: Vec<f64>,
}

#[pymethods]
impl BosChochOperator {
    #[new]
    fn new(swing_length: usize) -> PyResult<Self> {
        Ok(Self {
            inner: BosChoch::new(swing_length)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            bos: Vec::new(),
            choch: Vec::new(),
            level: Vec::new(),
            broken: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, f64, f64, f64) {
        let value = self.inner.append(high, low, close);
        self.bos.push(value.bos);
        self.choch.push(value.choch);
        self.level.push(value.level);
        self.broken.push(value.broken);
        (value.bos, value.choch, value.level, value.broken)
    }

    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || low.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            self.append(high, low, close);
        }
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.bos.clone()),
            PyArray1::from_vec(py, self.choch.clone()),
            PyArray1::from_vec(py, self.level.clone()),
            PyArray1::from_vec(py, self.broken.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> {
        self.inner.value().map(|value| (value.bos, value.choch, value.level, value.broken))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.bos.clear();
        self.choch.clear();
        self.level.clear();
        self.broken.clear();
    }
}
