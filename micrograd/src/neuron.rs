use crate::value::Value;
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    nin: usize,
    weights: Vec<Box<Value>>,
    bias: Box<Value>,
}

impl Neuron {
    pub fn new(nin: usize) -> Self {
        let generator = Uniform::from(-1.0..1.0);
        let mut rng = rand::thread_rng();

        Neuron {
            nin,
            weights: (0..nin)
                .map(|i| Box::new(Value::new(generator.sample(&mut rng), &format!("w{}", i))))
                .collect::<Vec<Box<Value>>>(),
            bias: Box::new(Value::new(generator.sample(&mut rng), "bias")),
        }
    }

    pub fn forward(&mut self, inputs: Vec<Box<Value>>) -> Value {
        assert!(self.nin == inputs.len());

        inputs
            .iter()
            .zip(self.weights.iter())
            .fold(*(self.bias.clone()), |acc, x| {
                let w = x.1.clone();
                let lbl = &(w.label);
                let mut prod = w.clone() * x.0.clone();
                prod.set_label(&format!("{} x {}", "input", lbl));
                acc + prod
            })
            .tanh()
    }

    pub fn parameters(&self) -> Vec<f32> {
        let mut r = vec![self.bias.data];

        for w in &self.weights {
            r.push(w.data);
        }

        return r;
    }

    pub fn adjust(&mut self, lr: f32) {
        self.bias.adjust(lr);
        self.weights.iter_mut().for_each(|w| (*w).adjust(lr));
    }
}

impl Default for Neuron {
    fn default() -> Self {
        Self {
            nin: 3,
            weights: vec![
                Box::new(Value::new(0.0, "w1")),
                Box::new(Value::new(0.1, "w2")),
                Box::new(Value::new(-0.5, "w3")),
            ],
            bias: Box::new(Value::new(3.0, "b")),
        }
    }
}

pub struct Layer {
    neurons: Vec<Box<Neuron>>,
}

impl Layer {
    pub fn new(nin: usize, nout: usize) -> Self {
        Layer {
            neurons: (0..nout)
                .map(|_| Box::new(Neuron::new(nin)))
                .collect::<Vec<Box<Neuron>>>(),
        }
    }

    pub fn forward(&self, inputs: Vec<Box<Value>>) -> Vec<Box<Value>> {
        self.neurons
            .iter()
            .map(|n| Box::new(n.clone().forward(inputs.clone())))
            .collect::<Vec<Box<Value>>>()
    }

    pub fn adjust(&mut self, lr: f32) {
        self.neurons.iter_mut().for_each(|n| (*n).adjust(lr))
    }

    // pub fn parameters() -> Vec<Vec<f32>> {

    // }
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            neurons: vec![Box::new(Default::default()), Box::new(Default::default())],
        }
    }
}

pub struct MLP {
    layers: Vec<Box<Layer>>,
}

impl MLP {
    pub fn new(nin: usize, nouts: Vec<usize>) -> Self {
        let mut sizes = vec![nin];
        sizes.extend(nouts);

        MLP {
            layers: (0..sizes.len() - 1)
                .map(|i| Box::new(Layer::new(sizes[i], sizes[i + 1])))
                .collect::<Vec<Box<Layer>>>(),
        }
    }

    pub fn forward(&self, inputs: Vec<Box<Value>>) -> Vec<Box<Value>> {
        let mut data = inputs.clone();
        for n in &self.layers {
            data = n.clone().forward(data);
        }
        data
    }

    pub fn loss(target: Vec<f32>, prediction: Vec<Box<Value>>) -> Value {
        assert_eq!(target.len(), prediction.len());

        target
            .iter()
            .zip(prediction.iter())
            .fold(Value::new(0.0, "loss"), |acc, x| {
                acc + (x.1.clone() - *x.0).pow(2)
            })
    }

    pub fn adjust(&mut self, lr: f32) {
        self.layers.iter_mut().for_each(|l| (*l).adjust(lr))
    }
}

impl Default for MLP {
    fn default() -> Self {
        // 3 input neurons, 2 layers of 4, 1 output
        MLP::new(3, vec![4, 4, 1])
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::value::tanh;

    #[test]
    fn test_basics() {
        let mut n = Neuron::default();

        assert_eq!(n.weights.len(), 3);

        let r = n.forward(vec![
            Box::new(Value::new(1.0, "x1")),
            Box::new(Value::new(4.0, "x2")),
            Box::new(Value::new(2.0, "x3")),
        ]);

        // tanh(x1 * w1 + x2 * w2 + x3 * w3 + b)
        let target = tanh(1.0 * 0.0 + 4.0 * 0.1 + 2.0 * (-0.5) + 3.0);

        assert_eq!(r.data, target);

        let layer = Layer::default();
        let lr = layer.forward(vec![
            Box::new(Value::new(1.0, "x1")),
            Box::new(Value::new(4.0, "x2")),
            Box::new(Value::new(2.0, "x3")),
        ]);

        assert_eq!(lr.len(), 2);
        assert_eq!(lr[0].clone().data, target);
        assert_eq!(lr[1].clone().data, target);

        let network = MLP::default();
        let nr = network.forward(vec![
            Box::new(Value::new(1.0, "x1")),
            Box::new(Value::new(4.0, "x2")),
            Box::new(Value::new(2.0, "x3")),
        ]);

        assert_eq!(nr.len(), 1);

        let mut loss = MLP::loss(vec![1.0], nr);

        loss.backward();

        loss.adjust(0.01);
    }

    #[test]
    fn test_forward_loss_adjust() {
        let mut network = MLP::default();
        let inputs = vec![
            vec![2.0, 3.0, -1.0],
            vec![3.0, -1.0, 0.5],
            vec![0.5, 1.0, 1.0],
            vec![1.0, 1.0, -1.0],
        ];
        let targets = vec![1.0, -1.0, -1.0, 1.0];

        let mut losses = vec![];

        for _ in 0..1 {
            let mut predictions = vec![];

            for i in 0..inputs.len() {
                network
                    .forward(
                        inputs[i]
                            .iter()
                            .map(|v| Box::new(Value::new(*v, "in")))
                            .collect(),
                    )
                    .iter()
                    .for_each(|p| predictions.push(p.clone()))
            }

            println!(
                "[OUT]: {:?}",
                predictions.iter().map(|p| p.data).collect::<Vec<f32>>()
            );

            let mut loss = MLP::loss(targets.clone(), predictions);

            losses.push(loss.data);

            println!(">>>>>>>> loss before {:?}", loss);

            loss.backward();

            println!(">>>>>>>> loss after {:?}", loss);

            network.adjust(0.05);
        }

        assert_eq!(losses, vec![1.0])
    }
}
