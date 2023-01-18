use serde::{Deserialize, Serialize};
use std::{ops, vec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueOperation {
    Add,
    Multiply,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    data: f32,
    children: Vec<Value>,
    op: Option<ValueOperation>,
}

impl Value {
    pub fn new(data: f32) -> Self {
        Value {
            data: data,
            children: vec![],
            op: None,
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        let mut r = Value::new(self.data + rhs.data);
        r.children = vec![self, rhs];
        r.op = Some(ValueOperation::Add);
        r
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        let mut r = Value::new(self.data * rhs.data);
        r.children = vec![self, rhs];
        r.op = Some(ValueOperation::Multiply);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = Value::new(1.0) + Value::new(2.0);
        assert_eq!(result.data, 3.0);
        assert_eq!(result.children.len(), 2);
    }

    #[test]
    fn test_multiplication() {
        let result = Value::new(2.0) * Value::new(3.0);
        assert_eq!(result.data, 6.0);
        assert_eq!(result.children.len(), 2);
    }
}
