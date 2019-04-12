## Drawing Triangle
This tutorial is veri similar to [previos one](drawing-points).
But there are few difference:
- the triablge is drawn by `drawElements`(`draw_elements_with_i32`) function
- it uses indeces array in order to use `drawElements` function

## Changes

Add indeces array
```rust
let indices:[u16;3] = [0,1,2];
let indices_array = {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()?
        .buffer();
    let location: u32 = indices.as_ptr() as u32 / 2;
    Uint16Array::new(&memory_buffer).subarray(location, location + indices.len() as u32)
};
```
`indices_array` is represatation of `Uint16Array` in Rust
Create buffer for indeces
```rust
// Create an empty buffer object to store Index buffer
let Index_Buffer = gl.create_buffer().ok_or("failed to create buffer")?;
// Bind appropriate array buffer to it
gl.bind_buffer(
    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
    Some(&Index_Buffer),
);
// Pass the vertex data to the buffer
gl.buffer_data_with_array_buffer_view(
    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
    &indices_array,
    WebGlRenderingContext::STATIC_DRAW,
);
```

Change drawing function from `draw_array` into `draw_elements_with_i32`
```rust
// Draw the triangle
gl.draw_elements_with_i32(
    WebGlRenderingContext::TRIANGLES,
    indices.len() as i32,
    WebGlRenderingContext::UNSIGNED_SHORT,
    0,
);
```

### Chalenges faced
This example took a day or two to understand why I do not see triangle on a screen. I even opened a [question ticket](https://github.com/rustwasm/wasm-bindgen/issues/1438) in wasm-bindgen repository and thanks to [@nstoddard](https://github.com/nstoddard) I finnaly made it works.

So my path was following. Because drawing function is `draw_elements_with_i32` I thought that `indeces` array should be `[i32;3]` type.
Also, copying example to build Float32Array, I was deviding location pointer by 4. But correct example is that indices are `u16` elemets array. They takes 2 bits so I have to device by 2 and also change `indices: [i32;3]` into `indices:[u16;3]`. After that modification I could see my triange.

Hura! Hura! Hura!

### PS
In [official example](https://github.com/rustwasm/wasm-bindgen/blob/master/examples/webgl/src/lib.rs) `draw_arrays` method is used to draw a triangle.
```rust
gl.draw_arrays(
    WebGlRenderingContext::TRIANGLES,
    0,
    (vertices.len() / 3) as i32,
);
```
It much easy to go from previous Draw Points section to currect, you just need to change render type from `POINTS` to `TRIANGLES`