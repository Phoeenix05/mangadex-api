pub mod athome;
pub use athome::*;

pub mod chapter;
pub use chapter::*;

pub mod cover;
pub use cover::*;

pub mod manga;
pub use manga::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiError;
