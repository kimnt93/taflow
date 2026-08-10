use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::VolumeByTimeProfile as State;
#[pyclass]
pub struct VolumeByTimeProfile {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl VolumeByTimeProfile {
    #[new]
    #[pyo3(signature=(buckets=24,utc_offset_minutes=0))]
    fn new(buckets: usize, utc_offset_minutes: i32) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(buckets, utc_offset_minutes)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, o: f64, h: f64, l: f64, c: f64, v: f64, t: i64) -> Option<f64> {
        let x = self.inner.append(o, h, l, c, v, t);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        o: PyReadonlyArray1<f64>,
        h: PyReadonlyArray1<f64>,
        l: PyReadonlyArray1<f64>,
        c: PyReadonlyArray1<f64>,
        v: PyReadonlyArray1<f64>,
        t: PyReadonlyArray1<i64>,
    ) -> PyResult<()> {
        let (o, h, l, c, v, t) = (
            o.as_slice()?,
            h.as_slice()?,
            l.as_slice()?,
            c.as_slice()?,
            v.as_slice()?,
            t.as_slice()?,
        );
        if [o.len(), h.len(), l.len(), c.len(), v.len(), t.len()]
            .windows(2)
            .any(|x| x[0] != x[1])
        {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "OHLCV and timestamp must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for i in 0..o.len() {
                self.append(o[i], h[i], l[i], c[i], v[i], t[i]);
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
