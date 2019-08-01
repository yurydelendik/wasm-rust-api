use crate::runtime::Store;
use crate::types::{ExternType, FuncType, GlobalType, MemoryType};
use crate::values::Val;
use core::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::result::Result;

use cranelift_codegen::ir;
use wasmtime_runtime::{VMContext, VMFunctionBody};

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
        use wasmtime_runtime::Export::*;
        match export {
            wasmtime_runtime::Export::Function {
                address,
                vmctx,
                signature,
            } => {
                let ty = FuncType::from_cranelift_signature(signature.clone());
                let callable = WasmtimeFn(store.clone(), signature, address, vmctx);
                let f = Func::new(store, ty, Box::new(callable));
                Extern::Func(Rc::new(RefCell::new(f)))
            }
            wasmtime_runtime::Export::Memory {
                definition,
                vmctx,
                memory,
            } => {
                let ty = MemoryType::from_cranelift_memory(memory.memory.clone());
                let m = self::Memory::new(store, ty);
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
                Extern::Global(Rc::new(RefCell::new(self::Global::new(store, ty, val))))
            }
            _ => unimplemented!("from_wasmtime_export other"),
        }
    }
}

#[derive(Fail, Debug)]
#[fail(display = "Wasm trap")]
pub struct Trap;

pub trait Callable: Any {
    fn call(&self, params: &[Val], results: &mut [Val]) -> Result<(), Trap>;
}

struct WasmtimeFn(
    Rc<RefCell<Store>>,
    ir::Signature,
    *const VMFunctionBody,
    *mut VMContext,
);

impl Callable for WasmtimeFn {
    fn call(&self, params: &[Val], results: &mut [Val]) -> Result<(), Trap> {
        use core::cmp::max;
        use core::{mem, ptr};

        let mut store = self.0.borrow_mut();
        let signature = &self.1;
        let address = self.2;
        let callee_vmctx = self.3;

        let context = store.context();
        let value_size = mem::size_of::<u64>();
        let mut values_vec: Vec<u64> = vec![0; max(params.len(), results.len())];

        // Store the argument values into `values_vec`.
        for (index, arg) in params.iter().enumerate() {
            unsafe {
                let ptr = values_vec.as_mut_ptr().add(index);

                match arg {
                    Val::I32(x) => ptr::write(ptr as *mut i32, *x),
                    Val::I64(x) => ptr::write(ptr as *mut i64, *x),
                    // Val::F32(x) => ptr::write(ptr as *mut u32, *x),
                    // Val::F64(x) => ptr::write(ptr as *mut u64, *x),
                    _ => unimplemented!("WasmtimeFn arg"),
                }
            }
        }

        // Get the trampoline to call for this function.
        let exec_code_buf = context
            .compiler()
            .get_trampoline(address, signature, value_size)
            .map_err(|_| Trap)?; //was ActionError::Setup)?;

        // Make all JIT code produced thus far executable.
        context.compiler().publish_compiled_code();

        // Call the trampoline.
        if let Err(message) = unsafe {
            wasmtime_runtime::wasmtime_call_trampoline(
                callee_vmctx,
                exec_code_buf,
                values_vec.as_mut_ptr() as *mut u8,
            )
        } {
            return Err(Trap); //Ok(ActionOutcome::Trapped { message });
        }

        // Load the return values out of `values_vec`.
        for (index, abi_param) in signature.returns.iter().enumerate() {
            unsafe {
                let ptr = values_vec.as_ptr().add(index);

                results[index] = match abi_param.value_type {
                    ir::types::I32 => Val::from(ptr::read(ptr as *const i32)),
                    ir::types::I64 => Val::from(ptr::read(ptr as *const i64)),
                    //ir::types::F32 => RuntimeValue::F32(ptr::read(ptr as *const u32)),
                    //ir::types::F64 => RuntimeValue::F64(ptr::read(ptr as *const u64)),
                    other => panic!("unsupported value type {:?}", other),
                }
            }
        }

        Ok(())
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
        let mut results = Vec::new();
        results.resize(self.result_arity(), Val::default());
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
