use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::UpDownVolumeRatio as State;
#[pyclass]
pub struct UpDownVolumeRatio {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl UpDownVolumeRatio {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new()
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, advancing_volume: f64, declining_volume: f64) -> Option<f64> {
        let x = self.inner.append(advancing_volume, declining_volume);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        advancing_volume: PyReadonlyArray1<f64>,
        declining_volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (advancing_volume, declining_volume) =
            (advancing_volume.as_slice()?, declining_volume.as_slice()?);
        if advancing_volume.len() != declining_volume.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "breadth inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for i in 0..advancing_volume.len() {
                self.append(advancing_volume[i], declining_volume[i]);
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
