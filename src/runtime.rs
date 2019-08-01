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
}

// Store

pub struct Store {
    engine: Rc<RefCell<Engine>>,
    context: Context,
}

impl Store {
    pub fn new(engine: Rc<RefCell<Engine>>) -> Store {
        Store {
            engine,
            context: Context::create(),
        }
    }

    pub(crate) fn context(&mut self) -> &mut Context {
        &mut self.context
    }
}
