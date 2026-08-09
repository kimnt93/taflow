use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::RollingVolumeWeightedAveragePrice;

#[pyclass]
pub struct RollingVolumeWeightedAveragePriceOperator {
    inner: RollingVolumeWeightedAveragePrice,
    outputs: Vec<f64>,
}

#[pymethods]
impl RollingVolumeWeightedAveragePriceOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: RollingVolumeWeightedAveragePrice::new(timeperiod)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        let value = self.inner.append(high, low, close, volume);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (h, l, c, v) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            volume.as_slice()?,
        );
        if h.len() != l.len() || h.len() != c.len() || h.len() != v.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (((&h, &l), &c), &v) in h.iter().zip(l).zip(c).zip(v) {
                self.append(h, l, c, v);
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
