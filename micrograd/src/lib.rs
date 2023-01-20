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
    values: JsValue,
}

#[wasm_bindgen]
impl Playground {
    pub fn new() -> Playground {
        // XX: initial example
        // let a = Value::new(2.0, "a");
        // let b = Value::new(-3.0, "b");
        // let c = Value::new(10.0, "c");
        // let mut e = a * b;
        // e.set_label("e");
        // let mut d = e + c;
        // d.set_label("d");
        // let f = Value::new(-2.0, "f");
        // let mut l = d * f;
        // l.set_label("L");
        // Playground { values: vec![l] }

        // XX: our first neuron
        let x1 = Value::new(2.0, "x1");
        let x2 = Value::new(0.0, "x2");
        let w1 = Value::new(-3.0, "w1");
        let w2 = Value::new(1.0, "w2");
        let b = Value::new(6.8813735870195432, "b");

        let mut x1_w1 = x1 * w1;
        x1_w1.set_label("x1*w1");

        let mut x2_w2 = x2 * w2;
        x2_w2.set_label("x2*w2");

        let mut x1_w1_x2_w2 = x1_w1 + x2_w2;
        x1_w1_x2_w2.set_label("x1*w1 + x2*w2");

        let mut n = x1_w1_x2_w2 + b;
        n.set_label("n");

        Playground {
            values: serde_wasm_bindgen::to_value::<Vec<&Value>>(&vec![n.tanh().backward()])
                .unwrap(),
        }
    }

    pub fn get_state(&self) -> JsValue {
        self.values.clone()
    }
}
