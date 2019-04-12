## Applying Colors

This example is based on [Drawing a Quad](drawing-quad) example.
To simplify view, I've created few marcos to create `Float32Array` and `Uint16Array`:

```rust
macro_rules! float_32_array
//    ...
macro_rules! uint_16_array
```

So now to create Float32Array in Rust we need only do following
```rust
let vertices_array = float_32_array!(vertices);
```

### Steps to Colors

We need to add colors array for each vertex. So copy-paste added in original tutorial example an array and convert it to Rust.
```rust
let colors[i32;12] = [ 0,0,1, 1,0,0, 0,1,0, 1,0,1,];
```

Then we need to add color buffer and bind color data to it
```rust
// Create an empty buffer object to store the vertex buffer
let colors_buffer = gl.create_buffer().ok_or("failed to create buffer")?;

//Bind appropriate array buffer to it
gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&colors_buffer));

// Pass the vertex data to the buffer
gl.buffer_data_with_array_buffer_view(
    WebGlRenderingContext::ARRAY_BUFFER,
    &colors_array,
    WebGlRenderingContext::STATIC_DRAW,
);

```

Next step is to update shader program
```rust
// vertex shader source code
    let vertCode = r#"attribute vec3 coordinates;
attribute vec3 color;
varying vec3 vColor;
void main(void) {
   gl_Position = vec4(coordinates, 1.0);
   vColor = color;
}
"#;

// fragment shader source code
    let fragCode = r#"precision mediump float;
varying vec3 vColor;
void main(void) {
    gl_FragColor = vec4(vColor, 1.);
}"#;
```
and make use of color attribute
```rust
// bind the color buffer
gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&colors_buffer));

// get the attribute location
let color = gl.get_attrib_location(&shaderProgram, "color") as u32;

// point attribute to the volor buffer object
gl.vertex_attrib_pointer_with_i32(color, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

// enable the color attribute
gl.enable_vertex_attrib_array(color);
```

And ` npm start`...and my quad is black.
As you may noticed, color array is `Float32Array`, where mine is `i32`,
so let's convert it to `f32` array:
```rust
let colors: [f32; 12] = [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
```

Now my quad is colorized.

Hura! Hura!

Full example in [examples/colors](../examples/colors) directory.