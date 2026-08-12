use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{metrics::UpMarketCaptureRatio as State, NanPolicy};

fn value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}
/// Native state backing `taflow.metrics.UpMarketCaptureRatio`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct UpMarketCaptureRatio {
    inner: State,
}

#[pymethods]
impl UpMarketCaptureRatio {
    #[new]
    #[pyo3(signature = (periods_per_year=252.0, nan_policy="omit"))]
    fn new(periods_per_year: f64, nan_policy: &str) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(
                periods_per_year,
                NanPolicy::try_from(nan_policy).map_err(value_error)?,
            )
            .map_err(value_error)?,
        })
    }

    fn from_returns(
        &mut self,
        py: Python<'_>,
        primary: PyReadonlyArray1<'_, f64>,
        benchmark: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<()> {
        let primary = primary.as_slice()?;
        let benchmark = benchmark.as_slice()?;
        py.allow_threads(|| self.inner.from_returns(primary, benchmark))
            .map(|_| ())
            .map_err(value_error)
    }

    fn from_log_returns(
        &mut self,
        py: Python<'_>,
        primary: PyReadonlyArray1<'_, f64>,
        benchmark: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<()> {
        let primary = primary.as_slice()?;
        let benchmark = benchmark.as_slice()?;
        py.allow_threads(|| self.inner.from_log_returns(primary, benchmark))
            .map(|_| ())
            .map_err(value_error)
    }

    fn from_equity(
        &mut self,
        py: Python<'_>,
        primary: PyReadonlyArray1<'_, f64>,
        benchmark: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<()> {
        let primary = primary.as_slice()?;
        let benchmark = benchmark.as_slice()?;
        py.allow_threads(|| self.inner.from_equity(primary, benchmark))
            .map(|_| ())
            .map_err(value_error)
    }

    fn from_pnl(
        &mut self,
        py: Python<'_>,
        primary: PyReadonlyArray1<'_, f64>,
        benchmark: PyReadonlyArray1<'_, f64>,
        initial_capital: f64,
        benchmark_initial_capital: f64,
    ) -> PyResult<()> {
        let primary = primary.as_slice()?;
        let benchmark = benchmark.as_slice()?;
        py.allow_threads(|| {
            self.inner.from_pnl(
                primary,
                benchmark,
                initial_capital,
                benchmark_initial_capital,
            )
        })
        .map(|_| ())
        .map_err(value_error)
    }

    fn append(&mut self, primary: f64, benchmark: f64) -> PyResult<Option<f64>> {
        self.inner.append(primary, benchmark).map_err(value_error)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        primary: PyReadonlyArray1<'_, f64>,
        benchmark: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<()> {
        let primary = primary.as_slice()?;
        let benchmark = benchmark.as_slice()?;
        py.allow_threads(|| self.inner.extend(primary, benchmark))
            .map(|_| ())
            .map_err(value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self) -> Option<f64> {
        self.inner.compute()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
