use failure::Error;
use std::cell::RefCell;
use std::fs::read;
use std::rc::Rc;
use wasm_rust_api::*;

fn main() -> Result<(), Error> {
    let wasm = read("gcd.wasm")?;
    let engine = Rc::new(RefCell::new(Engine::default()));
    let store = Rc::new(RefCell::new(Store::new(engine)));
    let module = Rc::new(RefCell::new(Module::new(store.clone(), &wasm)?));
    let gcd_index = module
        .borrow()
        .exports()
        .iter()
        .enumerate()
        .find(|(_, export)| export.name().to_string() == "gcd")
        .unwrap()
        .0;
    let instance = Rc::new(RefCell::new(Instance::new(store.clone(), module, &[])?));
    let gcd = instance.borrow().exports()[gcd_index]
        .borrow()
        .func()
        .clone();
    let result = gcd.borrow().call(&[Val::from(6i32), Val::from(27i32)])?;
    println!("{:?}", result);
    Ok(())
}
