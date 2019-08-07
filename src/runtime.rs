use std::cell::RefCell;
use std::rc::Rc;

use crate::context::Context;

// Runtime Environment

// Configuration

pub struct Config {
    debug_info: bool,
}

impl Config {
    pub fn default() -> Config {
        Config { debug_info: false }
    }

    pub(crate) fn debug_info(&self) -> bool {
        self.debug_info
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
        let debug_info = engine.borrow().config().debug_info();
        Store {
            _engine: engine,
            context: Context::create(debug_info),
        }
    }

    pub(crate) fn context(&mut self) -> &mut Context {
        &mut self.context
    }
}
