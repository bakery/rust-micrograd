use serde::{Deserialize, Serialize};
use std::f32::consts::E;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{ops, vec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueOperation {
    Add,
    Multiply,
    Tanh,
    Pow(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub id: usize,
    pub label: String,
    pub data: f32,
    pub grad: f32,
    pub children: Vec<Box<Value>>,
    pub op: Option<ValueOperation>,
}

impl Drop for Value {
    fn drop(&mut self) {
        print!(">>>>>>>>>>>>> vLUE dropped")
    }
}

pub fn tanh(x: f32) -> f32 {
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

    pub fn adjust(&mut self, lr: f32) {
        println!(
            ">>>>>>> adjusting {} by {} with grad {}",
            self.data,
            -lr * self.grad,
            self.grad
        );
        self.data += -lr * self.grad;
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

    pub fn pow(self, n: u8) -> Value {
        let mut result = Value::new(
            f32::powi(self.data, n.into()),
            &format!("{} ^ {}", self.label, n),
        );
        result.op = Some(ValueOperation::Pow(n));
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
                ValueOperation::Pow(n) => {
                    assert!(self.children.len() == 1);

                    let val = self.children[0].data;

                    // derivate of x^n = n * x ^ (n-1)
                    self.children[0]
                        .set_grad(self.grad * (f32::from(*n) * f32::powi(val, (n - 1).into())))
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

impl ops::Add<Box<Value>> for Value {
    type Output = Value;

    fn add(self, rhs: Box<Value>) -> Self::Output {
        let mut r = Value::new(self.data + rhs.data, "a");
        r.children = vec![Box::new(self), rhs.clone()];
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

impl ops::Mul<f32> for Value {
    type Output = Value;

    fn mul(self, rhs: f32) -> Self::Output {
        let rhs_val = Value::new(rhs, "rhs_b");
        let mut r = Value::new(self.data * rhs_val.data, "b");

        r.children = vec![Box::new(self), Box::new(rhs_val)];
        r.op = Some(ValueOperation::Multiply);
        r
    }
}

impl ops::Mul<Value> for Box<Value> {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        let mut r = Value::new(self.data * rhs.data, "b");
        r.children = vec![self.clone(), Box::new(rhs)];
        r.op = Some(ValueOperation::Multiply);
        r
    }
}

impl ops::Mul<Box<Value>> for Box<Value> {
    type Output = Value;

    fn mul(self, rhs: Box<Value>) -> Self::Output {
        let mut r = Value::new(self.data * rhs.data, "b");
        r.children = vec![self.clone(), rhs.clone()];
        r.op = Some(ValueOperation::Multiply);
        r
    }
}

impl ops::Sub<f32> for Value {
    type Output = Value;

    fn sub(self, rhs: f32) -> Self::Output {
        Value::new(-rhs, "sub") + self
    }
}

impl ops::Sub<f32> for Box<Value> {
    type Output = Value;

    fn sub(self, rhs: f32) -> Self::Output {
        Value::new(-rhs, "sub") + self.clone()
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
    fn test_adjust() {
        let mut value = Value::new(1.0, "a");
        value.grad = -0.3;
        value.adjust(0.05);
        assert_eq!(value.data, 1.0 + (-0.05 * value.grad));
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
    fn test_subtraction() {
        let result = Value::new(5.0, "a") - 6.0;
        assert_eq!(result.data, -1.0);
    }

    #[test]
    fn test_multiplication() {
        let result = Value::new(2.0, "a") * Value::new(3.0, "b");
        assert_eq!(result.data, 6.0);
        assert_eq!(result.children.len(), 2);

        let result = result * 2.0;
        assert_eq!(result.data, 12.0);
        assert_eq!(result.children.len(), 2);
    }

    #[test]
    fn test_tanh() {
        let result = Value::new(2.0, "a").tanh();
        assert_eq!(result.data, 0.9640276);
        assert_eq!(result.label, "tanh(a)");
    }

    #[test]
    fn test_pow() {
        let result = Value::new(2.0, "a").pow(2);
        assert_eq!(result.data, 4.0);
        assert_eq!(result.label, "a ^ 2");
    }

    #[test]
    fn test_labels() {
        let mut l = Value::new(1.0, "L");
        assert_eq!(l.label, "L");
        l.set_label("LL");
        assert_eq!(l.label, "LL");
    }

    #[test]
    fn test_forward_and_backward() {
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

        let mut result = n.tanh();

        result.backward();

        assert_eq!(result.data, 0.7071067);
        assert_eq!(result.grad, 1.0);

        assert_eq!(result.children[0].data, 0.8813734);
        assert_eq!(result.children[0].grad, 0.5000001);

        assert_eq!(result.children[0].children[0].data, -6.0);
        assert_eq!(result.children[0].children[0].grad, 0.5000001);

        assert_eq!(result.children[0].children[1].data, 6.8813734);
        assert_eq!(result.children[0].children[1].grad, 0.5000001);

        assert_eq!(result.children[0].children[0].children[0].data, -6.0);
        assert_eq!(result.children[0].children[0].children[0].grad, 0.5000001);

        assert_eq!(result.children[0].children[0].children[1].data, 0.0);
        assert_eq!(result.children[0].children[0].children[1].grad, 0.5000001);

        assert_eq!(
            result.children[0].children[0].children[0].children[0].data,
            2.0
        );
        assert_eq!(
            result.children[0].children[0].children[0].children[0].grad,
            -1.5000004
        );

        assert_eq!(
            result.children[0].children[0].children[0].children[1].data,
            -3.0
        );
        assert_eq!(
            result.children[0].children[0].children[0].children[1].grad,
            1.0000002
        );

        assert_eq!(
            result.children[0].children[0].children[1].children[0].data,
            0.0
        );
        assert_eq!(
            result.children[0].children[0].children[1].children[0].grad,
            0.5000001
        );

        assert_eq!(
            result.children[0].children[0].children[1].children[1].data,
            1.0
        );
        assert_eq!(
            result.children[0].children[0].children[1].children[1].grad,
            0.0
        );
    }

    // XX: cannot do this with current implementation
    // #[test]
    // fn test_value_reuse() {
    //     let a = Value::new(3.0, "a");
    //     let mut b = a + a;

    //     b.backward();

    //     assert_eq!(b.data, 6.0);
    //     assert_eq!(b.grad, 1.0);
    //     assert_eq!(b.children.len(), 2);
    //     assert_eq!(b.children[0].grad, 2.0);
    //     assert_eq!(b.children[1].grad, 2.0);
    // }
}
