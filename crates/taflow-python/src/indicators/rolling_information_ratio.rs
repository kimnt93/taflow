use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::RollingInformationRatio;
#[pyclass]
pub struct RollingInformationRatioOperator {
    inner: RollingInformationRatio,
    outputs: Vec<f64>,
}
#[pymethods]
impl RollingInformationRatioOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: RollingInformationRatio::new(timeperiod)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        let v = self.inner.append(input, benchmark);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
        benchmark: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (input, benchmark) = (input.as_slice()?, benchmark.as_slice()?);
        if input.len() != benchmark.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            input.iter().zip(benchmark).for_each(|(&x, &b)| {
                self.append(x, b);
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
