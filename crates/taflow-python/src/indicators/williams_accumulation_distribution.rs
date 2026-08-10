use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::WilliamsAccumulationDistribution as State;
#[pyclass]
pub struct WilliamsAccumulationDistribution {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl WilliamsAccumulationDistribution {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().unwrap(),
            output: Vec::new(),
        })
    }
    fn append(&mut self, h: f64, l: f64, c: f64) -> Option<f64> {
        let x = self.inner.append(h, l, c);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        h: PyReadonlyArray1<f64>,
        l: PyReadonlyArray1<f64>,
        c: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (h, l, c) = (h.as_slice()?, l.as_slice()?, c.as_slice()?);
        if [h.len(), l.len(), c.len()].windows(2).any(|x| x[0] != x[1]) {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for ((&a, &b), &d) in h.iter().zip(l).zip(c) {
                self.append(a, b, d);
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
