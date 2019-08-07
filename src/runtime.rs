use std::cell::RefCell;
use std::rc::Rc;

use crate::context::Context;

use cranelift_codegen::settings;
use wasmtime_jit::Features;

// Runtime Environment

// Configuration

fn default_flags() -> settings::Flags {
    let flag_builder = settings::builder();
    settings::Flags::new(flag_builder)
}

pub struct Config {
    flags: settings::Flags,
    features: Features,
    debug_info: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            debug_info: false,
            features: Default::default(),
            flags: default_flags(),
        }
    }

    pub fn new(flags: settings::Flags, features: Features, debug_info: bool) -> Config {
        Config {
            flags,
            features,
            debug_info,
        }
    }

    pub(crate) fn debug_info(&self) -> bool {
        self.debug_info
    }

    pub(crate) fn flags(&self) -> &settings::Flags {
        &self.flags
    }

    pub(crate) fn features(&self) -> &Features {
        &self.features
    }
}

// Engine

pub struct Engine {
    config: Config,
}

impl Engine {
    pub fn new(config: Config) -> Engine {
        Engine { config }
    }

    pub fn default() -> Engine {
        Engine::new(Config::default())
    }

    pub(crate) fn config(&self) -> &Config {
        &self.config
    }
}

// Store

pub struct Store {
    _engine: Rc<RefCell<Engine>>,
    context: Context,
}

impl Store {
    pub fn new(engine: Rc<RefCell<Engine>>) -> Store {
        let flags = engine.borrow().config().flags().clone();
        let features = engine.borrow().config().features().clone();
        let debug_info = engine.borrow().config().debug_info();
        Store {
            _engine: engine,
            context: Context::create(flags, features, debug_info),
        }
    }

    pub(crate) fn context(&mut self) -> &mut Context {
        &mut self.context
    }
}
