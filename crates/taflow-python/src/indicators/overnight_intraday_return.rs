use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{OvernightIntradayReturn as State, OvernightIntradayReturnValue};
#[pyclass]
pub struct OvernightIntradayReturn {
    inner: State,
    overnight: Vec<f64>,
    intraday: Vec<f64>,
}
#[pymethods]
impl OvernightIntradayReturn {
    #[new]
    #[pyo3(signature=(utc_offset_minutes=0))]
    fn new(utc_offset_minutes: i32) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(utc_offset_minutes)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            overnight: Vec::new(),
            intraday: Vec::new(),
        })
    }
    fn append(&mut self, o: f64, h: f64, l: f64, c: f64, v: f64, t: i64) -> Option<(f64, f64)> {
        let x = self.inner.append(o, h, l, c, v, t);
        let z = x.unwrap_or(OvernightIntradayReturnValue {
            overnight: f64::NAN,
            intraday: f64::NAN,
        });
        self.overnight.push(z.overnight);
        self.intraday.push(z.intraday);
        x.map(|z| (z.overnight, z.intraday))
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
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.overnight.clone()),
            PyArray1::from_vec(py, self.intraday.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|z| (z.overnight, z.intraday))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.overnight.clear();
        self.intraday.clear();
    }
    fn __len__(&self) -> usize {
        self.overnight.len()
    }
}
