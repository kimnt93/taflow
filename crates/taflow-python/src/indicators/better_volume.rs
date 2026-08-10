use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::BetterVolume as State;
#[pyclass]
pub struct BetterVolume {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl BetterVolume {
    #[new]
    #[pyo3(signature = (period=20))]
    fn new(period: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period)
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, h: f64, l: f64, c: f64, v: f64) -> Option<f64> {
        let x = self.inner.append(h, l, c, v);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        h: PyReadonlyArray1<f64>,
        l: PyReadonlyArray1<f64>,
        c: PyReadonlyArray1<f64>,
        v: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (h, l, c, v) = (h.as_slice()?, l.as_slice()?, c.as_slice()?, v.as_slice()?);
        if [h.len(), l.len(), c.len(), v.len()]
            .windows(2)
            .any(|x| x[0] != x[1])
        {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "OHLCV inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for (((a, b), d), e) in h.iter().zip(l).zip(c).zip(v) {
                self.append(*a, *b, *d, *e);
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
