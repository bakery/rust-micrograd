//     // pub fn forward(&mut self, inputs: Vec<f32>) -> Value {
//     //     assert!(self.nin == inputs.len());

//     //     let mut result = Value::build(0.0, "acc");

//     //     for i in 0..self.nin {
//     //         result = result + (inputs[i] * self.weights[i]);
//     //     }

//     //     result
//     //     // inputs
//     //     //     .iter()
//     //     //     .zip(self.weights.iter())
//     //     //     .fold(*(self.bias), |acc, x| {
//     //     //         let lbl = &(w.label);

//     //     //         let mut prod = *(x.1) * (*(x.0));

//     //     //         prod.set_label(&format!("{} x {}", "input", lbl));

//     //     //         acc + prod
//     //     //     })
//     //     //     .tanh()
//     // }

//     // pub fn parameters(&self) -> Vec<f32> {
//     //     let mut r = vec![self.bias.data];

//     //     for w in &self.weights {
//     //         r.push(w.data);
//     //     }

//     //     return r;
//     // }

//     // pub fn adjust(&mut self, lr: f32) {
//     //     self.bias.adjust(lr);
//     //     self.weights.iter_mut().for_each(|w| (*w).adjust(lr));
//     // }
// }

// impl Default for Neuron {
//     fn default() -> Self {
//         Self {
//             nin: 3,
//             value: Neuron::build_value(
//                 Box::new((0..3).map(|i| match i {
//                     0 => Value::build(0.0, "w1"),
//                     1 => Value::build(0.1, "w2"),
//                     2 => Value::build(-0.5, "w3"),
//                     _ => Value::build(-0.5, "wn"),
//                 })),
//                 Box::new((0..3).map(|i| Value::build(0.0, &format!("x{i}")))),
//                 3.0,
//             ),
//         }
//     }
// }

// // ------------------------//////////////////////////////////////////////////////////////////////////////

// // pub struct MLP {
// //     layers: Vec<Box<Layer>>,
// // }

// // impl MLP {
// //     pub fn new(nin: usize, nouts: Vec<usize>) -> Self {
// //         let mut sizes = vec![nin];
// //         sizes.extend(nouts);

// //         MLP {
// //             layers: (0..sizes.len() - 1)
// //                 .map(|i| Box::new(Layer::new(sizes[i], sizes[i + 1])))
// //                 .collect::<Vec<Box<Layer>>>(),
// //         }
// //     }

// //     pub fn forward(&self, inputs: Vec<Box<Value>>) -> Vec<Box<Value>> {
// //         let mut data = inputs.clone();
// //         for n in &self.layers {
// //             data = n.clone().forward(data);
// //         }
// //         data
// //     }

// //     pub fn loss(target: Vec<f32>, prediction: Vec<Box<Value>>) -> Value {
// //         assert_eq!(target.len(), prediction.len());

// //         target
// //             .iter()
// //             .zip(prediction.iter())
// //             .fold(ValueRef::clone(0.0, "loss"), |acc, x| {
// //                 acc + (x.1.clone() - *x.0).pow(2)
// //             })
// //     }

// //     pub fn adjust(&mut self, lr: f32) {
// //         self.layers.iter_mut().for_each(|l| (*l).adjust(lr))
// //     }
// // }

// // impl Default for MLP {
// //     fn default() -> Self {
// //         // 3 input neurons, 2 layers of 4, 1 output
// //         MLP::new(3, vec![4, 4, 1])
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use std::vec;

//     use super::*;

//     #[test]
//     fn test_basics() {
//         let mut n = Neuron::default();

//         // assert_eq!(n.weights.len(), 3);

//         // let r = n.forward(vec![
//         //     Box::new(ValueRef::clone(1.0, "x1")),
//         //     Box::new(ValueRef::clone(4.0, "x2")),
//         //     Box::new(ValueRef::clone(2.0, "x3")),
//         // ]);

//         // // tanh(x1 * w1 + x2 * w2 + x3 * w3 + b)
//         // let target = tanh(1.0 * 0.0 + 4.0 * 0.1 + 2.0 * (-0.5) + 3.0);

//         // assert_eq!(r.data, target);

//         // let layer = Layer::default();
//         // let lr = layer.forward(vec![
//         //     Box::new(ValueRef::clone(1.0, "x1")),
//         //     Box::new(ValueRef::clone(4.0, "x2")),
//         //     Box::new(ValueRef::clone(2.0, "x3")),
//         // ]);

//         // assert_eq!(lr.len(), 2);
//         // assert_eq!(lr[0].clone().data, target);
//         // assert_eq!(lr[1].clone().data, target);

//         // let network = MLP::default();
//         // let nr = network.forward(vec![
//         //     Box::new(ValueRef::clone(1.0, "x1")),
//         //     Box::new(ValueRef::clone(4.0, "x2")),
//         //     Box::new(ValueRef::clone(2.0, "x3")),
//         // ]);

//         // assert_eq!(nr.len(), 1);

//         // let mut loss = MLP::loss(vec![1.0], nr);

//         // loss.backward();

//         // loss.adjust(0.01);
//     }

//     // #[test]
//     // fn test_forward_loss_adjust() {
//     //     let mut network = MLP::default();
//     //     let inputs = vec![
//     //         vec![2.0, 3.0, -1.0],
//     //         vec![3.0, -1.0, 0.5],
//     //         vec![0.5, 1.0, 1.0],
//     //         vec![1.0, 1.0, -1.0],
//     //     ];
//     //     let targets = vec![1.0, -1.0, -1.0, 1.0];

//     //     let mut losses = vec![];

//     //     for _ in 0..1 {
//     //         let mut predictions = vec![];

//     //         for i in 0..inputs.len() {
//     //             network
//     //                 .forward(
//     //                     inputs[i]
//     //                         .iter()
//     //                         .map(|v| Box::new(ValueRef::clone(*v, "in")))
//     //                         .collect(),
//     //                 )
//     //                 .iter()
//     //                 .for_each(|p| predictions.push(p.clone()))
//     //         }

//     //         println!(
//     //             "[OUT]: {:?}",
//     //             predictions.iter().map(|p| p.data).collect::<Vec<f32>>()
//     //         );

//     //         let mut loss = MLP::loss(targets.clone(), predictions);

//     //         losses.push(loss.data);

//     //         println!(">>>>>>>> loss before {:?}", loss);

//     //         loss.backward();

//     //         println!(">>>>>>>> loss after {:?}", loss);

//     //         network.adjust(0.05);
//     //     }

//     //     assert_eq!(losses, vec![1.0])
//     // }
// }

use crate::value;
use crate::value::ValueRef;
use rand::distributions::{Distribution, Uniform};
use std::vec;

#[derive(Debug)]
pub struct Neuron {
    nin: usize,
    weights: Vec<ValueRef>,
    bias: ValueRef,
}

impl Neuron {
    fn new(nin: usize) -> Self {
        let mut rng = rand::thread_rng();
        let generator = Uniform::from(-1.0..1.0);

        Neuron {
            nin,
            weights: (0..nin)
                .map(|_| value!(generator.sample(&mut rng)))
                .collect::<Vec<ValueRef>>(),
            bias: value!(generator.sample(&mut rng)),
        }
    }
    fn params(&self) -> Vec<f32> {
        let mut data = vec![];

        for w in &self.weights {
            data.push(w.data());
        }

        data.push(self.bias.data());

        data
    }
}

impl Neuron {
    fn forward(&self, inputs: &Vec<ValueRef>) -> ValueRef {
        assert_eq!(self.weights.len(), inputs.len());
        self.weights
            .iter()
            .zip(inputs.iter())
            .fold(ValueRef::clone(&self.bias), |acc, d| {
                acc + ValueRef::clone(&d.0) * ValueRef::clone(&d.1)
            })
            .tanh()
    }

    pub fn adjust(&mut self, lr: f32) {
        self.bias.adjust(lr);
        self.weights.iter_mut().for_each(|w| w.adjust(lr));
    }
}

#[derive(Debug)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(nin: usize, nout: usize) -> Self {
        Layer {
            neurons: (0..nout).map(|_| Neuron::new(nin)).collect::<Vec<Neuron>>(),
        }
    }

    fn forward(&self, inputs: Vec<ValueRef>) -> Vec<ValueRef> {
        self.neurons
            .iter()
            .map(|n| n.forward(&inputs))
            .collect::<Vec<ValueRef>>()
    }

    pub fn adjust(&mut self, lr: f32) {
        self.neurons.iter_mut().for_each(|n| (*n).adjust(lr))
    }

    pub fn params(&self) -> Vec<Vec<f32>> {
        self.neurons
            .iter()
            .map(|n| n.params())
            .collect::<Vec<Vec<f32>>>()
    }
}

#[derive(Debug)]
pub struct MLP {
    pub layers: Vec<Layer>,
    pub result: Option<Vec<ValueRef>>,
}

impl MLP {
    pub fn adjust(&mut self, lr: f32) {
        self.layers.iter_mut().for_each(|l| (*l).adjust(lr))
    }

    pub fn forward(&mut self, inputs: Vec<ValueRef>) {
        self.result = Some(
            self.layers
                .iter()
                .fold(inputs, |acc, layer| layer.forward(acc)),
        );
    }

    pub fn loss(&self, target: Vec<f32>) -> ValueRef {
        if let Some(predictions) = &self.result {
            assert_eq!(target.len(), predictions.len());
            target
                .iter()
                .map(|t| value!(*t))
                .zip(predictions.iter())
                .fold(value!(0.0), |acc, d| {
                    acc + (d.0 - ValueRef::clone(&d.1)).pow(2)
                })
        } else {
            value!(0.0)
        }
    }

    pub fn params(&self) -> Vec<Vec<Vec<f32>>> {
        self.layers
            .iter()
            .map(|layer| layer.params())
            .collect::<Vec<Vec<Vec<f32>>>>()
    }
}

impl Default for MLP {
    fn default() -> Self {
        MLP {
            layers: vec![
                Layer {
                    neurons: vec![Neuron::new(3), Neuron::new(3)],
                },
                Layer {
                    neurons: vec![Neuron::new(2), Neuron::new(2)],
                },
            ],
            result: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_forward() {
        let n = Neuron::new(2);
        let r = n.forward(&vec![value!(1.0), value!(2.0)]);
        let params = n.params();

        assert_eq!(n.weights.len(), 2);
        assert_eq!(params.len(), 3);
        assert_eq!(r.data(), params[0] * 1.0 + params[1] * 2.0 + params[2]);
    }

    #[test]
    fn test_basics() {
        let mut net = MLP::default();

        for _ in 0..10 {
            let result = net.forward(vec![value!(1.0), value!(2.0), value!(3.0)]);

            let mut loss = net.loss(vec![1.0, -1.0]);
            loss.backward();
            net.adjust(0.05);
        }
    }

    #[test]
    fn test_training_loop() {
        let mut net = MLP {
            layers: vec![Layer::new(3, 4), Layer::new(4, 4), Layer::new(4, 1)],
            result: None,
        };
        let inputs = vec![
            vec![2.0, 3.0, -1.0],
            vec![3.0, -1.0, 0.5],
            vec![0.5, 1.0, 1.0],
            vec![1.0, 1.0, -1.0],
        ];
        let targets = vec![vec![1.0], vec![-1.0], vec![-1.0], vec![1.0]];

        for _ in 0..10 {
            let mut loss = value!(0.0);

            for i in 0..inputs.len() {
                net.forward(
                    inputs[i]
                        .iter()
                        .map(|i| value!(*i))
                        .collect::<Vec<ValueRef>>(),
                );

                if let Some(r) = &net.result {
                    println!(
                        "result is {:?}",
                        r.iter().map(|rr| rr.data()).collect::<Vec<f32>>()
                    );
                }

                loss = value!(0.0) + net.loss(targets[i].clone());
            }

            println!("PARAMS: {:?}", net.params());

            loss.backward();
            net.adjust(0.05);

            println!("LOSS is: {}", loss.data());
        }
    }
}
