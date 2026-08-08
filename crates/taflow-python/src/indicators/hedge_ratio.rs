use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::HedgeRatio;

#[pyclass]
pub struct HedgeRatioOperator {
    inner: HedgeRatio,
    outputs: Vec<f64>,
}

#[pymethods]
impl HedgeRatioOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: HedgeRatio::new(timeperiod)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, x: f64, y: f64) -> Option<f64> {
        let value = self.inner.append(x, y);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        x: PyReadonlyArray1<f64>,
        y: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (x, y) = (x.as_slice()?, y.as_slice()?);
        if x.len() != y.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (&x, &y) in x.iter().zip(y) {
                self.append(x, y);
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
}
