use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::Swing;

#[pyclass]
pub struct SwingHighLowOperator {
    inner: Swing,
    signal: Vec<f64>,
    level: Vec<f64>,
    bars_since: Vec<f64>,
}

#[pymethods]
impl SwingHighLowOperator {
    #[new]
    fn new(swing_length: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Swing::new(swing_length).map_err(|error| PyValueError::new_err(error.to_string()))?,
            signal: Vec::new(),
            level: Vec::new(),
            bars_since: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64)> {
        let value = self.inner.append(high, low);
        let output = value.map_or((f64::NAN, f64::NAN, f64::NAN), |value| {
            (value.signal, value.level, value.bars_since)
        });
        self.signal.push(output.0);
        self.level.push(output.1);
        self.bars_since.push(output.2);
        value.map(|_| output)
    }

    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (high, low) = (high.as_slice()?, low.as_slice()?);
        if high.len() != low.len() {
            return Err(PyValueError::new_err("high and low must have equal lengths"));
        }
        for (&high, &low) in high.iter().zip(low) {
            self.append(high, low);
        }
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
            PyArray1::from_vec(py, self.signal.clone()),
            PyArray1::from_vec(py, self.level.clone()),
            PyArray1::from_vec(py, self.bars_since.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner.value().map(|value| (value.signal, value.level, value.bars_since))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.signal.clear();
        self.level.clear();
        self.bars_since.clear();
    }
}
