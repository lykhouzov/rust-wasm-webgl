## Drawing a Quad
This example is very similar to [Drawing Triangle](drawing-triangle).

Here you just need to add one more vertex and 3 indeces to draw 2 triangles which become a quade

```rust
let vertices: [f32; 12] = [
    -0.5, 0.5, 0.0, //
    -0.5, -0.5, 0.0, //
    0.5, -0.5, 0.0, //
    0.5, 0.5, 0.0, //
];
// ... //
let indices: [u16; 6] = [3, 2, 1, 3, 1, 0];
```

This is it.