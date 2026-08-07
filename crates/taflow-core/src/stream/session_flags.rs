//! Session-boundary flags for aligned numeric session identifiers.

/// Return a causal flag for the first bar of each session.
///
/// The first input is always a new session. Each later flag is true when the
/// current identifier differs from the previous identifier. The result has
/// exactly the same length as `session_id`.
pub fn session_flags(session_id: &[f64]) -> Vec<bool> {
    let mut flags = vec![false; session_id.len()];
    if let Some((first, rest)) = flags.split_first_mut() {
        *first = true;
        for (index, flag) in rest.iter_mut().enumerate() {
            *flag = session_id[index + 1] != session_id[index];
        }
    }
    flags
}
