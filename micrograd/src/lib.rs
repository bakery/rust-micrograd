pub mod neuron;
mod utils;
pub mod value;

use std::vec;

// use neuron::MLP;
// use value::Value;
use js_sys;
use wasm_bindgen::prelude::*;

use neuron::{Layer, Neuron, MLP};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub enum PlaygroundPresets {
    BasicExpression,
    Neuron,
    BasicMLP,
}

#[wasm_bindgen]
pub struct Playground {}

#[wasm_bindgen]
impl Playground {
    pub fn new() -> Playground {
        Playground {}
    }

    pub fn load_preset(&mut self, preset: PlaygroundPresets) -> JsValue {
        match preset {
            PlaygroundPresets::BasicExpression => {
                // https://www.youtube.com/watch?v=VMj-3S1tku0&t=1615s
                let mut expression =
                    (value!(2.0, "a") * value!(-3.0, "b") + value!(10.0, "c")) * value!(-2.0, "f");
                expression.set_label("L");
                expression.backward();
                serde_wasm_bindgen::to_value(&expression.flatten()).unwrap()
            }
            PlaygroundPresets::Neuron => {
                // https://www.youtube.com/watch?v=VMj-3S1tku0&t=1797s
                let mut n = value!(2.0, "x1") * value!(-3.0, "w1")
                    + value!(0.0, "x2") * value!(1.0, "w2")
                    + value!(6.881374, "b");
                n.set_label("n");
                let mut expression = n.tanh();
                expression.set_label("o");
                expression.backward();

                serde_wasm_bindgen::to_value(&expression.flatten()).unwrap()
            }
            PlaygroundPresets::BasicMLP => {
                let mut net = MLP {
                    layers: vec![Layer::new(1, 1), Layer::new(1, 1)],
                    result: None,
                };
                net.forward(vec![value!(2.0, "x1")]);

                let mut loss = net.loss(vec![0.5]);
                loss.backward();

                serde_wasm_bindgen::to_value(&loss.flatten()).unwrap()
            }
        }
    }
}

// let mut net = MLP::default();
// let result = net.forward(vec![
//     ValueRef::new(1.0),
//     ValueRef::new(2.0),
//     ValueRef::new(3.0),
// ]);

// let loss = net.loss(vec![1.0, -1.0]);

// if let Some(mut l) = loss {
//     l.backward();
//     net.adjust(0.05);

//     Playground {
//         values: serde_wasm_bindgen::to_value(&l.flatten()).unwrap()
//     }
// } else {
//     Playground {
//         values: JsValue::NULL
//     }
// }
