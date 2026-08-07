pub mod common;
pub mod error;
pub mod ma_type;
pub mod simd;
pub mod sliding_window;
pub mod stream;
pub mod traits;

#[allow(unused_mut, unused_variables, unused_assignments, dead_code)]
pub use error::{TaError, TaResult};
pub use ma_type::MaType;
