use crate::callable::{Callable, WasmtimeFn};
use crate::runtime::Store;
use crate::trap::Trap;
use crate::types::{ExternType, FuncType, GlobalType, MemoryType};
use crate::values::Val;
use std::cell::RefCell;
use std::rc::Rc;
use std::result::Result;

// Externals

pub enum Extern {
    Func(Rc<RefCell<Func>>),
    Global(Rc<RefCell<Global>>),
    Table(Rc<RefCell<Table>>),
    Memory(Rc<RefCell<Memory>>),
}

impl Extern {
    pub fn func(&self) -> &Rc<RefCell<Func>> {
        match self {
            Extern::Func(func) => func,
            _ => panic!("Extern::Func expected"),
        }
    }
    pub fn global(&self) -> &Rc<RefCell<Global>> {
        match self {
            Extern::Global(global) => global,
            _ => panic!("Extern::Global expected"),
        }
    }
    pub fn table(&self) -> &Rc<RefCell<Table>> {
        match self {
            Extern::Table(table) => table,
            _ => panic!("Extern::Table expected"),
        }
    }
    pub fn memory(&self) -> &Rc<RefCell<Memory>> {
        match self {
            Extern::Memory(memory) => memory,
            _ => panic!("Extern::Memory expected"),
        }
    }

    pub fn r#type(&self) -> ExternType {
        match self {
            Extern::Func(ft) => ExternType::ExternFunc(ft.borrow().r#type().clone()),
            Extern::Memory(ft) => ExternType::ExternMemory(ft.borrow().r#type().clone()),
            _ => unimplemented!("ExternType::type"),
        }
    }

    pub(crate) fn get_wasmtime_export(&mut self) -> wasmtime_runtime::Export {
        unimplemented!("get_wasmtime_export")
    }

    pub(crate) fn from_wasmtime_export(
        store: Rc<RefCell<Store>>,
        export: wasmtime_runtime::Export,
    ) -> Extern {
        use cranelift_wasm::GlobalInit;
        match export {
            wasmtime_runtime::Export::Function {
                address,
                vmctx,
                signature,
            } => {
                let ty = FuncType::from_cranelift_signature(signature.clone());
                let callable = WasmtimeFn::new(store.clone(), signature, address, vmctx);
                let f = Func::new(store, ty, Box::new(callable));
                Extern::Func(Rc::new(RefCell::new(f)))
            }
            wasmtime_runtime::Export::Memory {
                definition,
                vmctx,
                memory,
            } => {
                let ty = MemoryType::from_cranelift_memory(memory.memory.clone());
                let m = Memory::new(store, ty);
                Extern::Memory(Rc::new(RefCell::new(m)))
            }
            wasmtime_runtime::Export::Global {
                definition,
                vmctx,
                global,
            } => {
                let ty = GlobalType::from_cranelift_global(global.clone());
                let val = match global.initializer {
                    GlobalInit::I32Const(i) => Val::from(i),
                    GlobalInit::I64Const(i) => Val::from(i),
                    _ => unimplemented!("from_wasmtime_export initializer"),
                };
                Extern::Global(Rc::new(RefCell::new(Global::new(store, ty, val))))
            }
            _ => unimplemented!("from_wasmtime_export other"),
        }
    }
}

pub struct Func {
    store: Rc<RefCell<Store>>,
    callable: Box<dyn Callable + 'static>,
    r#type: FuncType,
}

impl Func {
    pub fn new(store: Rc<RefCell<Store>>, r#type: FuncType, callable: Box<Callable>) -> Func {
        Func {
            store,
            callable,
            r#type,
        }
    }

    pub fn r#type(&self) -> &FuncType {
        &self.r#type
    }

    pub fn param_arity(&self) -> usize {
        self.r#type.params().len()
    }

    pub fn result_arity(&self) -> usize {
        self.r#type.results().len()
    }

    pub fn callable(&self) -> &(dyn Callable + 'static) {
        self.callable.as_ref()
    }

    pub fn call(&self, params: &[Val]) -> Result<Box<[Val]>, Trap> {
        let mut results = vec![Val::default(); self.result_arity()];
        self.callable.call(params, &mut results)?;
        Ok(results.into_boxed_slice())
    }
}

pub struct Global {
    store: Rc<RefCell<Store>>,
    r#type: GlobalType,
    val: Val,
}

impl Global {
    pub fn new(store: Rc<RefCell<Store>>, r#type: GlobalType, val: Val) -> Global {
        Global { store, r#type, val }
    }

    pub fn r#type(&self) -> &GlobalType {
        &self.r#type
    }

    pub fn get(&self) -> &Val {
        &self.val
    }

    pub fn set(&mut self, val: Val) {
        self.val = val;
    }
}

pub struct Table;

pub struct Memory {
    store: Rc<RefCell<Store>>,
    r#type: MemoryType,
}

impl Memory {
    pub fn new(store: Rc<RefCell<Store>>, r#type: MemoryType) -> Memory {
        Memory { store, r#type }
    }

    pub fn r#type(&self) -> &MemoryType {
        &self.r#type
    }

    pub unsafe fn data(&self) -> *const u8 {
        unimplemented!("Memory::data")
    }

    pub fn data_size(&self) -> usize {
        unimplemented!("Memory::data_size")
    }

    pub fn size(&self) -> u32 {
        unimplemented!("Memory::size")
    }

    pub fn grow(&mut self, delta: u32) -> bool {
        unimplemented!("Memory::grow")
    }
}
