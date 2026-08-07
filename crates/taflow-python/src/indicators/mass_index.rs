use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::MassIndex;

#[pyclass]
pub struct MassIndexOperator {
    inner: MassIndex,
    values: Vec<f64>,
}

#[pymethods]
impl MassIndexOperator {
    #[new]
    #[pyo3(signature = (ema_period=9, sum_period=25))]
    fn new(ema_period: usize, sum_period: usize) -> PyResult<Self> {
        Ok(Self {
            inner: MassIndex::new(ema_period, sum_period)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            values: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let value = self.inner.append(high, low);
        self.values.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (high, low) = (high.as_slice()?, low.as_slice()?);
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (&high, &low) in high.iter().zip(low) {
            self.append(high, low);
        }
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.values.clone())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.values.clear();
    }
}
