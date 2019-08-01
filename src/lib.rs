mod callable;
mod context;
mod externals;
mod instance;
mod module;
mod runtime;
mod trap;
mod types;
mod values;

#[macro_use]
extern crate failure_derive;

pub use crate::instance::Instance;
pub use crate::module::Module;
pub use crate::runtime::{Config, Engine, Store};
pub use crate::values::Val;
