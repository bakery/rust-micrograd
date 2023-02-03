use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{ops, vec};

#[derive(Debug)]
pub enum ValueOperation {
    Add,
    Multiply,
    Tanh,
}

#[derive(Debug)]
pub struct AValue<'a> {
    pub id: usize,
    pub label: String,
    pub data: f32,
    pub grad: f32,
    pub children: Vec<&'a AValue<'a>>,
    pub op: Option<ValueOperation>,
}

impl<'a> AValue<'a> {
    pub fn new(data: f32, label: &str) -> Self {
        AValue {
            id: 1, // COUNTER.fetch_add(1, Ordering::Relaxed),
            label: String::from(label),
            data: data,
            grad: 0.0,
            children: vec![],
            op: None,
        }
    }
}

// impl<'a> ops::Add<&AValue<'a>> for &AValue<'a> {
//     type Output = AValue<'a>;

//     fn add(self, rhs: &AValue) -> Self::Output {
//         let mut r = AValue::<'a>::new(self.data + rhs.data, "a");
//         r.children = vec![self, rhs];
//         r.op = Some(ValueOperation::Add);
//         r
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        // let v1 = AValue::new(1.0, "v1");
        // let v2: AValue = AValue::new(2.0, "v2");
        // let v3 = &v1 + &v2;

        // assert_eq!(v3.data, 3.0);
    }
}
