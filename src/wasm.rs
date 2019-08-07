//! This file defines the extern "C" API, which is compatible with the
//! [Wasm C API](https://github.com/WebAssembly/wasm-c-api).

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use super::{
    Callable, Engine, ExportType, Extern, Func, FuncType, ImportType, Instance, Module, Store,
    Trap, Val, ValType,
};
use std::boxed::Box;
use std::cell::RefCell;
use std::mem;
use std::ptr;
use std::rc::Rc;
use std::slice;

pub type byte_t = ::std::os::raw::c_char;
pub type float32_t = f32;
pub type float64_t = f64;
pub type wasm_byte_t = byte_t;
#[repr(C)]
#[derive(Clone)]
pub struct wasm_byte_vec_t {
    pub size: usize,
    pub data: *mut wasm_byte_t,
}
pub type wasm_name_t = wasm_byte_vec_t;
#[repr(C)]
#[derive(Clone)]
pub struct wasm_config_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_engine_t {
    engine: Rc<RefCell<Engine>>,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_store_t {
    store: Rc<RefCell<Store>>,
}
#[doc = ""]
pub type wasm_mutability_t = u8;
#[repr(C)]
#[derive(Clone)]
pub struct wasm_limits_t {
    pub min: u32,
    pub max: u32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_valtype_t {
    ty: ValType,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_valtype_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_valtype_t,
}
pub type wasm_valkind_t = u8;
#[repr(C)]
#[derive(Clone)]
pub struct wasm_functype_t {
    functype: FuncType,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_functype_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_functype_t,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_globaltype_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_globaltype_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_globaltype_t,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_tabletype_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_tabletype_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_tabletype_t,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_memorytype_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_memorytype_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_memorytype_t,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_externtype_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_externtype_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_externtype_t,
}
pub type wasm_externkind_t = u8;
#[repr(C)]
#[derive(Clone)]
pub struct wasm_importtype_t {
    ty: ImportType,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_importtype_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_importtype_t,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_exporttype_t {
    ty: ExportType,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_exporttype_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_exporttype_t,
}
#[doc = ""]
#[repr(C)]
#[derive(Clone)]
pub struct wasm_ref_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wasm_val_t {
    pub kind: wasm_valkind_t,
    pub of: wasm_val_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wasm_val_t__bindgen_ty_1 {
    pub i32: i32,
    pub i64: i64,
    pub f32: float32_t,
    pub f64: float64_t,
    pub ref_: *mut wasm_ref_t,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_val_vec_t {
    pub size: usize,
    pub data: *mut wasm_val_t,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_frame_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_frame_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_frame_t,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_instance_t {
    instance: Rc<RefCell<Instance>>,
}
pub type wasm_message_t = wasm_name_t;
#[repr(C)]
#[derive(Clone)]
pub struct wasm_trap_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_foreign_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_module_t {
    module: Rc<RefCell<Module>>,
    imports: Vec<wasm_importtype_t>,
    exports: Vec<wasm_exporttype_t>,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_shared_module_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_func_t {
    func: Rc<RefCell<Func>>,
}
pub type wasm_func_callback_t = ::std::option::Option<
    unsafe extern "C" fn(args: *const wasm_val_t, results: *mut wasm_val_t) -> *mut wasm_trap_t,
>;
pub type wasm_func_callback_with_env_t = ::std::option::Option<
    unsafe extern "C" fn(
        env: *mut ::std::os::raw::c_void,
        args: *const wasm_val_t,
        results: *mut wasm_val_t,
    ) -> *mut wasm_trap_t,
>;
#[repr(C)]
#[derive(Clone)]
pub struct wasm_global_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_table_t {
    _unused: [u8; 0],
}
pub type wasm_table_size_t = u32;
#[repr(C)]
#[derive(Clone)]
pub struct wasm_memory_t {
    _unused: [u8; 0],
}
pub type wasm_memory_pages_t = u32;
#[repr(C)]
#[derive(Clone)]
pub struct wasm_extern_t {
    ext: Rc<RefCell<Extern>>,
}
#[repr(C)]
#[derive(Clone)]
pub struct wasm_extern_vec_t {
    pub size: usize,
    pub data: *mut *mut wasm_extern_t,
}

#[no_mangle]
pub unsafe extern "C" fn wasm_byte_vec_delete(v: *mut wasm_byte_vec_t) {
    let _ = Vec::from_raw_parts((*v).data, 0, (*v).size);
}

#[no_mangle]
pub unsafe extern "C" fn wasm_byte_vec_new_uninitialized(out: *mut wasm_byte_vec_t, size: usize) {
    let mut buffer = Vec::<byte_t>::with_capacity(size);
    let result = out.as_mut().unwrap();
    result.size = buffer.capacity();
    result.data = buffer.as_mut_ptr();
    mem::forget(buffer);
}

#[no_mangle]
pub unsafe extern "C" fn wasm_engine_delete(engine: *mut wasm_engine_t) {
    let _ = Box::from_raw(engine);
}

#[no_mangle]
pub unsafe extern "C" fn wasm_engine_new() -> *mut wasm_engine_t {
    let engine = Box::new(wasm_engine_t {
        engine: Rc::new(RefCell::new(Engine::default())),
    });
    Box::into_raw(engine)
}

#[no_mangle]
pub unsafe extern "C" fn wasm_extern_as_func(e: *mut wasm_extern_t) -> *mut wasm_func_t {
    let func = (*e).ext.borrow().func().clone();
    let func = Box::new(wasm_func_t { func });
    Box::into_raw(func)
}

#[no_mangle]
pub unsafe extern "C" fn wasm_extern_vec_delete(v: *mut wasm_extern_vec_t) {
    let buffer = Vec::from_raw_parts((*v).data, (*v).size, (*v).size);
    for p in buffer {
        // TODO wasm_extern_delete
        let _ = Box::from_raw(p);
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_func_as_extern(f: *mut wasm_func_t) -> *mut wasm_extern_t {
    let ext = Extern::Func((*f).func.clone());
    let ext = Box::new(wasm_extern_t {
        ext: Rc::new(RefCell::new(ext)),
    });
    Box::into_raw(ext)
}

#[no_mangle]
pub unsafe extern "C" fn wasm_func_call(
    func: *const wasm_func_t,
    args: *const wasm_val_t,
    results: *mut wasm_val_t,
) -> *mut wasm_trap_t {
    let func = (*func).func.borrow();
    let mut params = Vec::with_capacity(func.param_arity());
    for i in 0..params.len() {
        let val = &(*args.offset(i as isize));
        params.push(val.val());
    }
    let out = func.call(&params).expect("good call");
    for i in 0..func.result_arity() {
        let val = &mut (*results.offset(i as isize));
        *val = wasm_val_t::from_val(&out[i]);
    }
    ptr::null_mut()
}

impl wasm_val_t {
    fn from_val(_val: &Val) -> wasm_val_t {
        unimplemented!("wasm_val_t::from_val")
    }

    fn val(&self) -> Val {
        unimplemented!("wasm_val_t::val")
    }
}

impl Callable for wasm_func_callback_t {
    fn call(&self, params: &[Val], results: &mut [Val]) -> Result<(), Trap> {
        let params = params
            .iter()
            .map(|p| wasm_val_t::from_val(p))
            .collect::<Vec<_>>();
        let mut out_results = Vec::with_capacity(results.len());
        let func = self.expect("wasm_func_callback_t fn");
        let out = unsafe { func(params.as_ptr(), out_results.as_mut_ptr()) };
        if out != ptr::null_mut() {
            panic!("wasm_func_callback_t trap");
        }
        for i in 0..results.len() {
            results[i] = out_results[i].val();
        }
        Ok(())
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_func_new(
    store: *mut wasm_store_t,
    ty: *const wasm_functype_t,
    callback: wasm_func_callback_t,
) -> *mut wasm_func_t {
    let store = (*store).store.clone();
    let ty = (*ty).functype.clone();
    let callback = Rc::new(callback);
    let func = Box::new(wasm_func_t {
        func: Rc::new(RefCell::new(Func::new(store, ty, callback))),
    });
    Box::into_raw(func)
}

#[no_mangle]
pub unsafe extern "C" fn wasm_func_delete(f: *mut wasm_func_t) {
    let _ = Box::from_raw(f);
}

#[no_mangle]
pub unsafe extern "C" fn wasm_functype_new(
    params: *mut wasm_valtype_vec_t,
    results: *mut wasm_valtype_vec_t,
) -> *mut wasm_functype_t {
    let params = Vec::from_raw_parts((*params).data, (*params).size, (*params).size)
        .into_iter()
        .map(|vt| (*vt).ty.clone())
        .collect::<Vec<_>>();
    let results = Vec::from_raw_parts((*results).data, (*results).size, (*results).size)
        .into_iter()
        .map(|vt| (*vt).ty.clone())
        .collect::<Vec<_>>();
    let functype = FuncType::new(params.into_boxed_slice(), results.into_boxed_slice());
    let functype = Box::new(wasm_functype_t { functype });
    Box::into_raw(functype)
}

#[no_mangle]
pub unsafe extern "C" fn wasm_functype_delete(ft: *mut wasm_functype_t) {
    let _ = Box::from_raw(ft);
}

#[no_mangle]
pub unsafe extern "C" fn wasm_instance_delete(instance: *mut wasm_instance_t) {
    let _ = Box::from_raw(instance);
}

#[no_mangle]
pub unsafe extern "C" fn wasm_instance_new(
    store: *mut wasm_store_t,
    module: *const wasm_module_t,
    imports: *const *const wasm_extern_t,
    _result: *mut *mut wasm_trap_t,
) -> *mut wasm_instance_t {
    let store = (*store).store.clone();
    let mut externs: Vec<Rc<RefCell<Extern>>> = Vec::with_capacity((*module).imports.len());
    for i in 0..(*module).imports.len() {
        let import = *imports.offset(i as isize);
        externs.push((*import).ext.clone());
    }
    let module = (*module).module.clone();
    match Instance::new(store, module, &externs) {
        Ok(instance) => {
            let instance = Box::new(wasm_instance_t {
                instance: Rc::new(RefCell::new(instance)),
            });
            Box::into_raw(instance)
        }
        _ => unimplemented!(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_instance_exports(
    instance: *const wasm_instance_t,
    out: *mut wasm_extern_vec_t,
) {
    let instance = &(*instance).instance.borrow();
    let exports = instance.exports();
    let mut buffer = Vec::with_capacity(exports.len());
    for e in exports.iter() {
        let ext = Box::new(wasm_extern_t { ext: e.clone() });
        buffer.push(Box::into_raw(ext));
    }
    (*out).size = buffer.capacity();
    (*out).data = buffer.as_mut_ptr();
    mem::forget(buffer);
}

#[no_mangle]
pub unsafe extern "C" fn wasm_module_delete(module: *mut wasm_module_t) {
    let _ = Box::from_raw(module);
}

#[no_mangle]
pub unsafe extern "C" fn wasm_module_new(
    store: *mut wasm_store_t,
    binary: *const wasm_byte_vec_t,
) -> *mut wasm_module_t {
    let binary = slice::from_raw_parts((*binary).data as *const u8, (*binary).size);
    let store = (*store).store.clone();
    let module = Module::new(store, binary).expect("module");
    let imports = module
        .imports()
        .iter()
        .map(|i| wasm_importtype_t { ty: i.clone() })
        .collect::<Vec<_>>();
    let exports = module
        .exports()
        .iter()
        .map(|e| wasm_exporttype_t { ty: e.clone() })
        .collect::<Vec<_>>();
    let module = Box::new(wasm_module_t {
        module: Rc::new(RefCell::new(module)),
        imports,
        exports,
    });
    Box::into_raw(module)
}

#[no_mangle]
pub unsafe extern "C" fn wasm_store_delete(store: *mut wasm_store_t) {
    let _ = Box::from_raw(store);
}

#[no_mangle]
pub unsafe extern "C" fn wasm_store_new(engine: *mut wasm_engine_t) -> *mut wasm_store_t {
    let engine = (*engine).engine.clone();
    let store = Box::new(wasm_store_t {
        store: Rc::new(RefCell::new(Store::new(engine))),
    });
    Box::into_raw(store)
}

#[no_mangle]
pub unsafe extern "C" fn wasm_valtype_vec_new_empty(out: *mut wasm_valtype_vec_t) {
    (*out).data = ptr::null_mut();
    (*out).size = 0;
}
