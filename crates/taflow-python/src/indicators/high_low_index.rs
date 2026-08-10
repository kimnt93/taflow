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
    fn append(&mut self, new_highs: f64, new_lows: f64) -> Option<f64> {
        let x = self.inner.append(new_highs, new_lows);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        new_highs: PyReadonlyArray1<f64>,
        new_lows: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (new_highs, new_lows) = (new_highs.as_slice()?, new_lows.as_slice()?);
        if new_highs.len() != new_lows.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "breadth inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for i in 0..new_highs.len() {
                self.append(new_highs[i], new_lows[i]);
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
