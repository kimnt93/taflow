use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::HighLowIndex as State;
#[pyclass]
pub struct HighLowIndex {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl HighLowIndex {
    #[new]
    fn new(period: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, a: f64, b: f64, c: f64, d: f64) -> Option<f64> {
        let x = self.inner.append(a, b, c, d);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        a: PyReadonlyArray1<f64>,
        b: PyReadonlyArray1<f64>,
        c: PyReadonlyArray1<f64>,
        d: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (a, b, c, d) = (a.as_slice()?, b.as_slice()?, c.as_slice()?, d.as_slice()?);
        if [a.len(), b.len(), c.len(), d.len()]
            .windows(2)
            .any(|x| x[0] != x[1])
        {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "breadth inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for i in 0..a.len() {
                self.append(a[i], b[i], c[i], d[i]);
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
