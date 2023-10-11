use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use std::panic;
extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn set_hook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn do_panic() {
    panic!("do_panic");
}

#[wasm_bindgen]
pub struct Foo {
    name: String,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            name: "-".to_string(),
        }
    }

    #[wasm_bindgen]
    pub async fn start(&mut self, name: String) {
        if true {
            panic!("oh no");
        }
        self.name = name;
    }

    #[wasm_bindgen]
    pub fn hello(&self) {
        log(&format!("Hello, I'm {}", self.name));
    }
}
