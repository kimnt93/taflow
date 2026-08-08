use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::McGinleyDynamic;
#[pyclass]
pub struct McGinleyDynamicOperator {
    inner: McGinleyDynamic,
    values: Vec<f64>,
}
#[pymethods]
impl McGinleyDynamicOperator {
    #[new]
    #[pyo3(signature = (length=10, c=1.0))]
    fn new(length: usize, c: f64) -> PyResult<Self> {
        Ok(Self {
            inner: McGinleyDynamic::new(length, c)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            values: Vec::new(),
        })
    }
    fn append(&mut self, close: f64) -> f64 {
        let v = self.inner.append(close).unwrap();
        self.values.push(v);
        v
    }
    fn extend(&mut self, py: Python<'_>, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        let close = close.as_slice()?;
        py.allow_threads(|| {
            for &v in close {
                self.append(v);
            }
        });
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
