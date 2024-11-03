#![allow(missing_debug_implementations)]
#![allow(missing_copy_implementations)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::new_without_default)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

uniffi::setup_scaffolding!();

mod utils;

pub use utils::*;