use crate::types::ValType;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct AnyRef;
impl AnyRef {
    pub fn null() -> AnyRef {
        AnyRef
    }
}

impl ::std::string::ToString for AnyRef {
    fn to_string(&self) -> String {
        "anyref".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct FuncRef;
impl ::std::string::ToString for FuncRef {
    fn to_string(&self) -> String {
        "funcref".to_owned()
    }
}

#[derive(Debug, Clone)]
pub enum Val {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
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
        Val::F32(val)
    }
}

impl From<f64> for Val {
    fn from(val: f64) -> Val {
        Val::F64(val)
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
