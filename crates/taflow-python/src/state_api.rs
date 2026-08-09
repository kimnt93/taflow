//! PyO3 adapters for the incremental core API.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

pub(crate) fn py_value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}

/// Appends `Option<f64>` results straight into a Rust-side output cache,
/// NaN-filling warm-up. One pass, no temporary `Vec`.
pub(crate) fn extend_from_options<I>(cache: &mut Vec<f64>, values: I)
where
    I: IntoIterator<Item = Option<f64>>,
{
    let values = values.into_iter();
    cache.reserve(values.size_hint().0);
    cache.extend(values.map(|value| value.unwrap_or(f64::NAN)));
}

pub(crate) fn push_option(cache: &mut Vec<f64>, value: Option<f64>) -> Option<f64> {
    cache.push(value.unwrap_or(f64::NAN));
    value
}
