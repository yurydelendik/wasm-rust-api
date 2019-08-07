use std::cell::{RefCell, RefMut};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use wasmtime_jit::Compiler;

#[derive(Clone)]
pub struct Context {
    compiler: Rc<RefCell<Compiler>>,
    debug_info: bool,
}

impl Context {
    pub fn new(compiler: Compiler, debug_info: bool) -> Context {
        Context {
            compiler: Rc::new(RefCell::new(compiler)),
            debug_info,
        }
    }

    pub fn create(debug_info: bool) -> Context {
        Context::new(create_compiler(), debug_info)
    }

    pub(crate) fn debug_info(&self) -> bool {
        self.debug_info
    }

    pub(crate) fn compiler(&mut self) -> RefMut<Compiler> {
        self.compiler.borrow_mut()
    }
}

impl Hash for Context {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        unsafe {
            let ptr = Rc::into_raw(self.compiler.clone());
            let _ = Rc::from_raw(ptr);
            ptr
        }
        .hash(state)
    }
}

impl Eq for Context {}

impl PartialEq for Context {
    fn eq(&self, other: &Context) -> bool {
        Rc::ptr_eq(&self.compiler, &other.compiler)
    }
}

pub(crate) fn create_compiler() -> Compiler {
    let isa = {
        let isa_builder =
            cranelift_native::builder().expect("host machine is not a supported target");
        let flag_builder = cranelift_codegen::settings::builder();
        isa_builder.finish(cranelift_codegen::settings::Flags::new(flag_builder))
    };

    Compiler::new(isa)
}
