mod utils;
pub mod value;

use std::vec;

use value::Value;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, micrograd!");
}

#[wasm_bindgen]
pub struct Playground {
    value1: Value,
    value2: Value,
}

#[wasm_bindgen]
impl Playground {
    pub fn new() -> Playground {
        Playground {
            value1: Value::new(1.0),
            value2: Value::new(2.0),
        }
    }

    pub fn getState(&self) -> Result<JsValue, JsValue> {
        let state = vec![&self.value1, &self.value2];
        Ok(serde_wasm_bindgen::to_value(&state)?)
    }
}
