use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::session_flags as native_session_flags;

/// Build causal session-boundary flags from numeric session identifiers.
#[pyfunction]
pub fn session_flags_array<'py>(
    py: Python<'py>,
    session_id: PyReadonlyArray1<'py, f64>,
) -> Bound<'py, PyArray1<bool>> {
    PyArray1::from_vec(py, native_session_flags(session_id.as_slice().unwrap_or_default()))
}
