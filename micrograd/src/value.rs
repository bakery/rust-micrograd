use serde::{Deserialize, Serialize};
use std::f32::consts::E;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{ops, vec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueOperation {
    Add,
    Multiply,
    Tanh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    id: usize,
    label: String,
    data: f32,
    grad: f32,
    children: Vec<Box<Value>>,
    op: Option<ValueOperation>,
}

fn tanh(x: f32) -> f32 {
    (f32::powf(E, 2.0 * x) - 1.0) / (f32::powf(E, 2.0 * x) + 1.0)
}

impl Value {
    pub fn new(data: f32, label: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Value {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            label: String::from(label),
            data: data,
            grad: 0.0,
            children: vec![],
            op: None,
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = String::from(label);
    }

    pub fn set_grad(&mut self, grad: f32) {
        self.grad = grad;
    }

    pub fn tanh(self) -> Value {
        let mut result = Value::new(tanh(self.data), &format!("tanh({})", self.label));
        result.op = Some(ValueOperation::Tanh);
        result.children = vec![Box::new(self)];

        result
    }

    pub fn backward(&mut self) -> &Value {
        self.grad = 1.0;
        self._backward();

        for child in &mut self.children {
            child._backward();
        }

        self
    }

    pub fn _backward(&mut self) {
        if let Some(op) = &self.op {
            match op {
                ValueOperation::Add => {
                    // for addition, let them grads flow
                    for child in &mut self.children {
                        child.set_grad(self.grad * 1.0)
                    }
                }
                ValueOperation::Multiply => {
                    assert!(self.children.len() == 2);

                    // for multiplications, it's grad * the_other_guy

                    let d0 = self.children[0].data;
                    let d1 = self.children[1].data;

                    self.children[0].set_grad(self.grad * d1);
                    self.children[1].set_grad(self.grad * d0);
                }
                ValueOperation::Tanh => {
                    assert!(self.children.len() == 1);

                    let val = self.children[0].data;

                    // derivative of tanh: (1-tanh ^ 2)
                    self.children[0].set_grad(self.grad * (1.0 - tanh(val) * tanh(val)));
                }
            }

            for child in &mut self.children {
                child._backward();
            }
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        let mut r = Value::new(self.data + rhs.data, "a");
        r.children = vec![Box::new(self), Box::new(rhs)];
        r.op = Some(ValueOperation::Add);
        r
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        let mut r = Value::new(self.data * rhs.data, "b");
        r.children = vec![Box::new(self), Box::new(rhs)];
        r.op = Some(ValueOperation::Multiply);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        let value = Value::new(1.0, "a");
        let another_value = Value::new(4.0, "b");
        assert_ne!(value.id, another_value.id);
    }

    #[test]
    fn test_addition() {
        let result = Value::new(1.0, "a") + Value::new(2.0, "b");
        assert_eq!(result.data, 3.0);
        assert_eq!(result.children.len(), 2);
        assert_eq!(result.children[0].data, 1.0);
        assert_eq!(result.children[1].data, 2.0);
    }

    #[test]
    fn test_multiplication() {
        let result = Value::new(2.0, "a") * Value::new(3.0, "b");
        assert_eq!(result.data, 6.0);
        assert_eq!(result.children.len(), 2);
    }

    #[test]
    fn test_tanh() {
        let result = Value::new(2.0, "a").tanh();
        assert_eq!(result.data, 0.9640276);
        assert_eq!(result.label, "tanh(a)");
    }

    #[test]
    fn test_labels() {
        let mut l = Value::new(1.0, "L");
        assert_eq!(l.label, "L");
        l.set_label("LL");
        assert_eq!(l.label, "LL");
    }

    #[test]
    fn test_backward() {
        // let n = Value::new(0.8814, "n");
        // let mut o = Value::tanh(n);

        // o.set_grad(1.0);
        // o.backward();

        // assert_eq!(o.children[0].grad, 0.4999814);

        // let mut z = Value::new(-6.0, "x") + Value::new(6.8814, "y");

        // z.set_grad(0.5);
        // z.backward();

        // assert_eq!(z.children[0].grad, 0.5);
        // assert_eq!(z.children[1].grad, 0.5);

        // let mut x2_w2 = Value::new(0.0, "x2") * Value::new(1.0, "w2");

        // x2_w2.set_grad(0.5);
        // x2_w2.backward();

        // assert_eq!(x2_w2.children[0].grad, 0.5);
        // assert_eq!(x2_w2.children[1].grad, 0.0);

        // ========================================

        // let x1 = Value::new(2.0, "x1");
        // let x2 = Value::new(0.0, "x2");
        // let w1 = Value::new(-3.0, "w1");
        // let w2 = Value::new(1.0, "w2");
        // let b = Value::new(6.8813735870195432, "b");

        // let mut x1_w1 = x1 * w1;
        // x1_w1.set_label("x1*w1");

        // let mut x2_w2 = x2 * w2;
        // x2_w2.set_label("x2*w2");

        // let mut x1_w1_x2_w2 = x1_w1 + x2_w2;
        // x1_w1_x2_w2.set_label("x1*w1 + x2*w2");

        // let mut n = x1_w1_x2_w2 + b;
        // n.set_label("n");
    }
}
