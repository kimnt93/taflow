use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{StochasticRelativeStrengthIndex as State, StreamingIndicator};
use taflow::MaType;

#[pyclass]
pub struct StochasticRelativeStrengthIndex {
    inner: State,
    fastk: Vec<f64>,
    fastd: Vec<f64>,
}

#[pymethods]
impl StochasticRelativeStrengthIndex {
    #[new]
    #[pyo3(signature = (timeperiod=14, fastk_period=5, fastd_period=3, fastd_matype=0))]
    fn new(
        timeperiod: usize,
        fastk_period: usize,
        fastd_period: usize,
        fastd_matype: i32,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(
                timeperiod,
                fastk_period,
                fastd_period,
                MaType::try_from(fastd_matype).map_err(|e| PyValueError::new_err(e.to_string()))?,
            )
            .map_err(|e| PyValueError::new_err(e.to_string()))?,
            fastk: Vec::new(),
            fastd: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.fastk, value.fastd));
        let output = value.unwrap_or((f64::NAN, f64::NAN));
        self.fastk.push(output.0);
        self.fastd.push(output.1);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(input, &mut self.fastk, &mut self.fastd)
        });
        Ok(())
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.fastk.clone()),
            PyArray1::from_vec(py, self.fastd.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.fastk, value.fastd))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.fastk.clear();
        self.fastd.clear();
    }
    fn __len__(&self) -> usize {
        self.fastk.len()
    }
}
