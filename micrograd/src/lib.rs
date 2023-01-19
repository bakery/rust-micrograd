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
    values: Vec<Value>,
}

#[wasm_bindgen]
impl Playground {
    pub fn new() -> Playground {
        Playground {
            values: vec![Value::new(2.0) * Value::new(-3.0) + Value::new(10.0)],
        }
    }

    pub fn get_state(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.values)?)
    }
}
