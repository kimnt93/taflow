use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::TradeVolumeIndex as State;
#[pyclass]
pub struct TradeVolumeIndex {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl TradeVolumeIndex {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().unwrap(),
            output: Vec::new(),
        })
    }
    fn append(&mut self, c: f64, v: f64) -> Option<f64> {
        let x = self.inner.append(c, v);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        c: PyReadonlyArray1<f64>,
        v: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (c, v) = (c.as_slice()?, v.as_slice()?);
        if c.len() != v.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "close and volume must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for (&a, &b) in c.iter().zip(v) {
                self.append(a, b);
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
        self.output.len()
    }
}
