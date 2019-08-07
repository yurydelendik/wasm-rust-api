use crate::callable::Callable;
use crate::types::ValType;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use cranelift_codegen::ir;
use wasmtime_jit::RuntimeValue;

#[derive(Clone)]
pub struct AnyRef;
impl AnyRef {
    pub fn null() -> AnyRef {
        AnyRef
    }
}

impl fmt::Debug for AnyRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "anyref")
    }
}

pub struct FuncRef {
    pub callable: Box<dyn Callable + 'static>,
}

impl fmt::Debug for FuncRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "funcref")
    }
}

#[derive(Debug, Clone)]
pub enum Val {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
    AnyRef(Rc<RefCell<AnyRef>>),
    FuncRef(Rc<RefCell<FuncRef>>),
}

impl Val {
    pub fn default() -> Val {
        Val::AnyRef(Rc::new(RefCell::new(AnyRef::null())))
    }

    pub fn r#type(&self) -> ValType {
        match self {
            Val::I32(_) => ValType::I32,
            Val::I64(_) => ValType::I64,
            Val::F32(_) => ValType::F32,
            Val::F64(_) => ValType::F64,
            Val::AnyRef(_) => ValType::AnyRef,
            Val::FuncRef(_) => ValType::FuncRef,
        }
    }

    pub(crate) unsafe fn write_value_to(&self, _ptr: *const i64) {
        unimplemented!("Val::write_value");
    }

    pub(crate) unsafe fn read_value_from(_ptr: *mut i64, _ty: ir::Type) -> Val {
        unimplemented!("Val::read_value");
    }
}

impl From<i32> for Val {
    fn from(val: i32) -> Val {
        Val::I32(val)
    }
}

impl From<i64> for Val {
    fn from(val: i64) -> Val {
        Val::I64(val)
    }
}

impl From<f32> for Val {
    fn from(val: f32) -> Val {
        Val::F32(val.to_bits())
    }
}

impl From<f64> for Val {
    fn from(val: f64) -> Val {
        Val::F64(val.to_bits())
    }
}

impl Into<i32> for Val {
    fn into(self) -> i32 {
        if let Val::I32(i) = self {
            i
        } else {
            panic!("Invalid conversion of {:?} to i32.", self);
        }
    }
}

impl Into<i64> for Val {
    fn into(self) -> i64 {
        if let Val::I64(i) = self {
            i
        } else {
            panic!("Invalid conversion of {:?} to i64.", self);
        }
    }
}

impl Into<f32> for Val {
    fn into(self) -> f32 {
        if let Val::F32(i) = self {
            RuntimeValue::F32(i).unwrap_f32()
        } else {
            panic!("Invalid conversion of {:?} to f32.", self);
        }
    }
}

impl Into<f64> for Val {
    fn into(self) -> f64 {
        if let Val::F64(i) = self {
            RuntimeValue::F64(i).unwrap_f64()
        } else {
            panic!("Invalid conversion of {:?} to f64.", self);
        }
    }
}

impl From<Rc<RefCell<AnyRef>>> for Val {
    fn from(val: Rc<RefCell<AnyRef>>) -> Val {
        Val::AnyRef(val)
    }
}

impl From<Rc<RefCell<FuncRef>>> for Val {
    fn from(val: Rc<RefCell<FuncRef>>) -> Val {
        Val::FuncRef(val)
    }
}
