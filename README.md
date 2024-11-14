<div align="center">

![rust micrograd screensho](https://github.com/bakery/rust-micrograd/blob/master/.github/preview.jpeg)

# Micrograd in Rust

Reimplementing [Micrograd](https://github.com/karpathy/micrograd) in Rust. Based on [Andrej Karpathy's youtube series](https://www.youtube.com/watch?v=VMj-3S1tku0&t=1626s)
</div>

This is an attempt to implement [Micrograd](https://github.com/karpathy/micrograd) in Rust while preserving a friendly looking API. This ships with WebAssemlby bindings and a web based playground allowing to step through forward and back prop passes.

## Rust API preview

```rust
// define an expression: a * b + c * f
let mut expression = (value!(2.0, "a") * value!(-3.0, "b") + value!(10.0, "c")) * value!(-2.0, "f");
// backprop, compute gradients
expression.backward();
```

```rust
// define a basic 2-layer MLP
let mut net = MLP {
    layers: vec![Layer::new(1, 1), Layer::new(1, 1)],
    result: None,
};

// forward pass on the MLP
net.forward(vec![value!(2.0, "x1")]);

// compute loss based on the expected output value
let mut loss = net.loss(vec![0.5]);

// backprop
loss.backward();
```
