use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::CumulativeSumControlChart;

#[pyclass]
pub struct CumulativeSumControlChartOperator {
    inner: CumulativeSumControlChart,
    output: Vec<f64>,
}

#[pymethods]
impl CumulativeSumControlChartOperator {
    #[new]
    #[pyo3(signature = (threshold=1.0))]
    fn new(threshold: f64) -> PyResult<Self> {
        Ok(Self {
            inner: CumulativeSumControlChart::new(threshold)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, change: f64) -> f64 {
        let value = self.inner.append(change);
        self.output.push(value);
        value
    }

    fn extend(&mut self, py: Python<'_>, change: PyReadonlyArray1<f64>) -> PyResult<()> {
        let change = change.as_slice()?;
        py.allow_threads(|| {
            for &change in change {
                self.append(change);
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
}
