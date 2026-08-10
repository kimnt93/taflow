use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::BullishPercentIndex as State;
#[pyclass]
pub struct BullishPercentIndex {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl BullishPercentIndex {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, on_buy_signal_count: f64, universe_size: f64) -> Option<f64> {
        let x = self.inner.append(on_buy_signal_count, universe_size);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        on_buy_signal_count: PyReadonlyArray1<f64>,
        universe_size: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (on_buy_signal_count, universe_size) =
            (on_buy_signal_count.as_slice()?, universe_size.as_slice()?);
        if on_buy_signal_count.len() != universe_size.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "breadth inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for i in 0..on_buy_signal_count.len() {
                self.append(on_buy_signal_count[i], universe_size[i]);
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
