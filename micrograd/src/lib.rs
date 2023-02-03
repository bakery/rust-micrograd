pub mod avalue;
pub mod neuron;
mod utils;
pub mod value;

use std::vec;

use neuron::MLP;
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
    values: JsValue,
}

#[wasm_bindgen]
impl Playground {
    pub fn new() -> Playground {
        let network = MLP::default();

        let prediction = network.forward(vec![
            Box::new(Value::new(1.0, "x1")),
            Box::new(Value::new(2.0, "x2")),
            Box::new(Value::new(3.0, "x3")),
        ]);
        let target = 1.0;

        let mut loss = MLP::loss(vec![target], prediction);
        loss.backward();

        Playground {
            values: serde_wasm_bindgen::to_value::<Vec<Box<Value>>>(&vec![Box::new(loss)]).unwrap(),
        }
    }

    pub fn get_state(&self) -> JsValue {
        self.values.clone()
    }
}
