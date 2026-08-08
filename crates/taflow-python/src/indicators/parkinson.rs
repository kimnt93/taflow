use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::Parkinson;

#[pyclass]
pub struct ParkinsonOperator {
    inner: Parkinson,
    output: Vec<f64>,
}

#[pymethods]
impl ParkinsonOperator {
    #[new]
    #[pyo3(signature = (timeperiod=20))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Parkinson::new(timeperiod)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let value = self.inner.append(high, low);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low) = (high.as_slice()?, low.as_slice()?);
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (&high, &low) in high.iter().zip(low) {
                self.append(high, low);
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
}
