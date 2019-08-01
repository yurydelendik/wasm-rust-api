mod context;
pub mod externals;
pub mod instance;
pub mod module;
pub mod runtime;
pub mod types;
pub mod values;

#[macro_use]
extern crate failure_derive;

pub use crate::instance::Instance;
pub use crate::module::Module;
pub use crate::runtime::{Config, Engine, Store};
pub use crate::values::Val;
