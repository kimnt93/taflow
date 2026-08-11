use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{metrics::UlcerIndex as State, MetricInputKind, NanPolicy};

fn value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}

fn input_kind(name: &str, initial_equity: Option<f64>) -> PyResult<MetricInputKind> {
    match (name, initial_equity) {
        ("returns", None) => Ok(MetricInputKind::Returns),
        ("log_returns", None) => Ok(MetricInputKind::LogReturns),
        ("equity", None) => Ok(MetricInputKind::Equity),
        ("pnl", Some(initial_equity)) => Ok(MetricInputKind::PeriodPnl { initial_equity }),
        ("pnl", None) => Err(PyValueError::new_err(
            "initial_equity is required for period-P&L conversion",
        )),
        (_, Some(_)) => Err(PyValueError::new_err(
            "initial_equity is accepted only for period-P&L input",
        )),
        _ => Err(PyValueError::new_err("unsupported UlcerIndex input mode")),
    }
}

/// Native state backing `taflow.metrics.UlcerIndex`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct UlcerIndex {
    inner: State,
}

#[pymethods]
impl UlcerIndex {
    #[new]
    #[pyo3(signature = (input_mode, initial_equity=None, nan_policy="omit"))]
    fn new(input_mode: &str, initial_equity: Option<f64>, nan_policy: &str) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(
                input_kind(input_mode, initial_equity)?,
                NanPolicy::try_from(nan_policy).map_err(value_error)?,
            )
            .map_err(value_error)?,
        })
    }

    fn append(&mut self, value: f64) -> PyResult<Option<f64>> {
        self.inner.append(value).map_err(value_error)
    }

    fn extend(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.extend(values))
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
