use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::OutsideBar;
#[pyclass]
pub struct OutsideBarOperator {
    inner: OutsideBar,
    outputs: Vec<f64>,
}
#[pymethods]
impl OutsideBarOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: OutsideBar::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let v = self.inner.append(high, low);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low) = (high.as_slice()?, low.as_slice()?);
        if high.len() != low.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            high.iter().zip(low).for_each(|(&h, &l)| {
                self.append(h, l);
            })
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
        self.outputs.len()
    }
}
