use serde::Serialize;
use std::cell::RefCell;
use std::f32::consts::E;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{ops, vec};

pub static COUNTER: AtomicUsize = AtomicUsize::new(1);

#[macro_export]
macro_rules! value {
    ($data:expr) => {{
        use crate::value::COUNTER;
        use std::sync::atomic::Ordering;
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        value!(id, $data, &format!("node-{}", id))
    }};
    ($data:expr, $label:expr) => {{
        use crate::value::COUNTER;
        use std::sync::atomic::Ordering;
        value!(COUNTER.fetch_add(1, Ordering::Relaxed), $data, $label)
    }};
    ($id:expr, $data:expr, $label:expr) => {{
        use crate::value::ValueRef;

        ValueRef::new($id, $data, $label)
    }};
}

pub fn tanh(x: f32) -> f32 {
    (f32::powf(E, 2.0 * x) - 1.0) / (f32::powf(E, 2.0 * x) + 1.0)
}

#[derive(Debug, Clone, Serialize)]
pub enum ValueOperation {
    Add,
    Multiply,
    Tanh,
    Pow(u8),
}

#[derive(Debug)]
pub struct RawValue {
    pub id: usize,
    pub label: String,
    pub data: f32,
    pub grad: f32,
    pub op: Option<ValueOperation>,
    pub children: Vec<ValueRef>,
}

#[derive(Debug, Serialize)]
pub struct FlatRawValue {
    pub id: usize,
    pub label: String,
    pub data: f32,
    pub grad: f32,
    pub op: Option<ValueOperation>,
    pub children: Vec<usize>,
}

impl From<&ValueRef> for FlatRawValue {
    fn from(value: &ValueRef) -> Self {
        FlatRawValue {
            id: value.id(),
            label: value.label(),
            data: value.data(),
            grad: value.grad(),
            op: value.op(),
            children: (*(value.0))
                .borrow()
                .children
                .iter()
                .map(|c| c.id())
                .collect::<Vec<usize>>(),
        }
    }
}

impl Drop for RawValue {
    fn drop(&mut self) {
        // println!(">>>>>>>>>>>>> value dropped")
    }
}

impl RawValue {
    pub fn new(data: f32, label: String, grad: f32) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);

        // println!(">>>>>>>>>>>>> value created");

        let id = COUNTER.fetch_add(1, Ordering::Relaxed);

        RawValue {
            id,
            label,
            data,
            grad,
            children: vec![],
            op: None,
        }
    }
}

#[derive(Debug)]
pub struct ValueRef(Rc<RefCell<RawValue>>);

impl ValueRef {
    pub fn new(id: usize, data: f32, label: &str) -> Self {
        ValueRef(Rc::new(RefCell::new(RawValue {
            id,
            data,
            label: String::from(label),
            grad: 0.0,
            children: vec![],
            op: None,
        })))
    }

    pub fn id(&self) -> usize {
        (*(self.0)).borrow().id
    }

    pub fn data(&self) -> f32 {
        (*(self.0)).borrow().data
    }

    pub fn grad(&self) -> f32 {
        (*(self.0)).borrow().grad
    }

    pub fn label(&self) -> String {
        (*(self.0)).borrow().label.clone()
    }

    pub fn op(&self) -> Option<ValueOperation> {
        (*(self.0)).borrow().op.clone()
    }

    pub fn clone(&self) -> ValueRef {
        ValueRef(Rc::clone(&self.0))
    }

    pub fn set_label(&mut self, label: &str) {
        (*(self.0)).borrow_mut().label = String::from(label);
    }
    pub fn set_data(&mut self, data: f32) {
        (*(self.0)).borrow_mut().data = data;
    }

    pub fn set_op(&mut self, op: Option<ValueOperation>) {
        (*(self.0)).borrow_mut().op = op;
    }

    pub fn set_children(&mut self, children: Vec<ValueRef>) {
        (*(self.0)).borrow_mut().children = children;
    }

    pub fn tanh(self) -> Self {
        let mut result = value!(tanh(self.data()), &format!("tanh({})", self.label()));
        result.set_op(Some(ValueOperation::Tanh));
        result.set_children(vec![ValueRef::clone(&self)]);

        result
    }

    pub fn pow(self, n: u8) -> Self {
        let mut result = value!(
            f32::powi(self.data(), n.into()),
            &format!("{}^{}", self.label(), n)
        );
        result.set_op(Some(ValueOperation::Pow(n)));
        result.set_children(vec![ValueRef::clone(&self)]);

        result
    }

    pub fn adjust(&mut self, lr: f32) {
        let mut _grad = 0.0;
        {
            _grad = self.grad()
        }
        self.set_data(self.data() - (lr * _grad));
        (*(self.0)).borrow_mut().grad = 0.0;
    }

    pub fn backward(&mut self) {
        (*(self.0)).borrow_mut().grad = 1.0;
        self._backward();
    }

    pub fn set_grad(&mut self, grad: f32) {
        (*(self.0)).borrow_mut().grad = self.grad() + grad;
    }

    fn _backward(&mut self) {
        let mut _self = (*(self.0)).borrow_mut();
        let grad = _self.grad;
        if let Some(op) = &_self.op {
            match op {
                ValueOperation::Add => {
                    // for addition, let them grads flow
                    for child in &mut _self.children {
                        child.set_grad(grad);
                    }
                }
                ValueOperation::Multiply => {
                    assert!(_self.children.len() == 2);

                    // for multiplications, it's grad * the_other_guy

                    let grad = _self.grad;
                    let d0 = _self.children[0].data();
                    let d1 = _self.children[1].data();

                    _self.children[0].set_grad(grad * d1);
                    _self.children[1].set_grad(grad * d0);
                }
                ValueOperation::Tanh => {
                    assert!(_self.children.len() == 1);

                    let grad = _self.grad;
                    let d = _self.children[0].data();

                    // derivative of tanh: (1-tanh ^ 2)
                    _self.children[0].set_grad(grad * (1.0 - tanh(d) * tanh(d)));
                }
                ValueOperation::Pow(n) => {
                    assert!(_self.children.len() == 1);

                    // derivate of x^n = n * x ^ (n-1)

                    let grad = _self.grad;
                    let d = _self.children[0].data();
                    let power_n = *n;

                    _self.children[0]
                        .set_grad(grad * (f32::from(power_n) * f32::powi(d, (power_n - 1).into())));
                }
            }

            for child in &mut _self.children {
                (*child)._backward();
            }
        }
    }

    fn _flatten(&self, children: &Vec<ValueRef>) -> Vec<FlatRawValue> {
        if children.len() == 0 {
            vec![]
        } else {
            let mut start = vec![];

            for c in children {
                start.push(FlatRawValue::from(c));
                start.append(&mut self._flatten(&(*(c.0)).borrow().children));
            }

            start
        }
    }

    pub fn flatten(&self) -> Vec<FlatRawValue> {
        let mut start = vec![FlatRawValue::from(self)];

        start.append(&mut self._flatten(&(*(self.0)).borrow().children));

        start
    }
}

impl ops::Add<ValueRef> for ValueRef {
    type Output = ValueRef;

    fn add(self, rhs: ValueRef) -> Self::Output {
        let mut r = value!(
            self.data() + rhs.data(),
            &format!("{} + {}", self.label(), rhs.label())
        );
        r.set_children(vec![ValueRef::clone(&self), ValueRef::clone(&rhs)]);
        r.set_op(Some(ValueOperation::Add));
        r
    }
}

impl ops::Mul<ValueRef> for ValueRef {
    type Output = ValueRef;

    fn mul(self, rhs: ValueRef) -> Self::Output {
        let mut r = value!(
            self.data() * rhs.data(),
            &format!("{} * {}", self.label(), rhs.label())
        );
        r.set_children(vec![ValueRef::clone(&self), ValueRef::clone(&rhs)]);
        r.set_op(Some(ValueOperation::Multiply));
        r
    }
}

impl ops::Mul<f32> for ValueRef {
    type Output = ValueRef;

    fn mul(self, rhs: f32) -> Self::Output {
        self * value!(rhs)
    }
}

impl ops::Sub<ValueRef> for ValueRef {
    type Output = ValueRef;

    fn sub(self, rhs: ValueRef) -> Self::Output {
        self + rhs * (-1.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_id_generation() {
        let value = value!(1.0);
        let another_value = value!(4.0);
        assert_ne!(value.id(), another_value.id());
    }

    #[test]
    fn test_addition() {
        let result = value!(1.0) + value!(2.0);
        assert_eq!(result.data(), 3.0);
        assert_eq!((*(result.0)).borrow().children.len(), 2);
    }

    #[test]
    fn test_subtraction() {
        let result = value!(1.0) - value!(3.0);
        assert_eq!(result.data(), -2.0);
    }

    #[test]
    fn test_multiplication() {
        let result = value!(2.0) * value!(3.0);
        assert_eq!(result.data(), 6.0);
        assert_eq!((*(result.0)).borrow().children.len(), 2);
    }

    #[test]
    fn test_tanh() {
        let result = value!(2.0, "x".into()).tanh();
        assert_eq!(result.data(), 0.9640276);
        assert_eq!(result.label(), "tanh(x)");
    }

    #[test]
    fn test_pow() {
        let result = value!(2.0, "z".into()).pow(2);
        assert_eq!(result.data(), 4.0);
        assert_eq!(result.label(), "z^2");
    }

    #[test]
    fn test_flatten() {
        let r = value!(2.0) + value!(1.5);
        let rr = r.flatten();
        assert_eq!(rr.len(), 3);
    }

    #[test]
    fn test_forward_and_backward() {
        let x1 = value!(2.0); // "x1"
        let x2 = value!(0.0); // "x2"
        let w1 = value!(-3.0); //  "w1"
        let w2 = value!(1.0); // "w2"
        let b = value!(6.8813735870195432); // "b"

        let mut x1_w1 = x1 * w1;
        // x1_w1.set_label("x1*w1");

        let mut x2_w2 = x2 * w2;
        // x2_w2.set_label("x2*w2");

        let mut x1_w1_x2_w2 = x1_w1 + x2_w2;
        // x1_w1_x2_w2.set_label("x1*w1 + x2*w2");

        let mut n = x1_w1_x2_w2 + b;
        // n.set_label("n");

        let mut result = n.tanh();

        result.backward();

        // assert_eq!(result.data(), 0.7071067);
        // assert_eq!(result.grad(), 1.0);

        // assert_eq!(result.children[0].data, 0.8813734);
        // assert_eq!(result.children[0].grad, 0.5000001);

        // assert_eq!(result.children[0].children[0].data, -6.0);
        // assert_eq!(result.children[0].children[0].grad, 0.5000001);

        // assert_eq!(result.children[0].children[1].data, 6.8813734);
        // assert_eq!(result.children[0].children[1].grad, 0.5000001);

        // assert_eq!(result.children[0].children[0].children[0].data, -6.0);
        // assert_eq!(result.children[0].children[0].children[0].grad, 0.5000001);

        // assert_eq!(result.children[0].children[0].children[1].data, 0.0);
        // assert_eq!(result.children[0].children[0].children[1].grad, 0.5000001);

        // assert_eq!(
        //     result.children[0].children[0].children[0].children[0].data,
        //     2.0
        // );
        // assert_eq!(
        //     result.children[0].children[0].children[0].children[0].grad,
        //     -1.5000004
        // );

        // assert_eq!(
        //     result.children[0].children[0].children[0].children[1].data,
        //     -3.0
        // );
        // assert_eq!(
        //     result.children[0].children[0].children[0].children[1].grad,
        //     1.0000002
        // );

        // assert_eq!(
        //     result.children[0].children[0].children[1].children[0].data,
        //     0.0
        // );
        // assert_eq!(
        //     result.children[0].children[0].children[1].children[0].grad,
        //     0.5000001
        // );

        // assert_eq!(
        //     result.children[0].children[0].children[1].children[1].data,
        //     1.0
        // );
        // assert_eq!(
        //     result.children[0].children[0].children[1].children[1].grad,
        //     0.0
        // );
    }
}
