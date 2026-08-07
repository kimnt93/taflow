use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::AverageDailyDollarValue;

#[pyclass]
pub struct AverageDailyDollarValueOperator {
    inner: AverageDailyDollarValue,
    output: Vec<f64>,
}

#[pymethods]
impl AverageDailyDollarValueOperator {
    #[new]
    #[pyo3(signature = (timeperiod=20))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: AverageDailyDollarValue::new(timeperiod)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let value = self.inner.append(close, volume);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(
        &mut self,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (close, volume) = (close.as_slice()?, volume.as_slice()?);
        if close.len() != volume.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (&close, &volume) in close.iter().zip(volume) {
            self.append(close, volume);
        }
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
}
