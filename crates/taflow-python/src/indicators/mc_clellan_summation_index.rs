use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::McClellanSummationIndex as State;
#[pyclass]
pub struct McClellanSummationIndex {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl McClellanSummationIndex {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, advancers: f64, decliners: f64) -> Option<f64> {
        let x = self.inner.append(advancers, decliners);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        advancers: PyReadonlyArray1<f64>,
        decliners: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (advancers, decliners) = (advancers.as_slice()?, decliners.as_slice()?);
        if advancers.len() != decliners.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for i in 0..advancers.len() {
                self.append(advancers[i], decliners[i]);
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
