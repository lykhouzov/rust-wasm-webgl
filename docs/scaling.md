## Scaling

This example is based on previous [Colors](colors) example.

### Steps to apply scale

We need to add uniform matrix to vertex shader in order to be able to scale our shape.
```rust
// vertex shader source code
    let vertCode = r#"attribute vec3 coordinates;
attribute vec3 color;
varying vec3 vColor;
uniform mat4 u_xformMatrix;
void main(void) {
   gl_Position = u_xformMatrix * vec4(coordinates, 1.0);
   vColor = color;
}
"#;
```

Then bind data to this uniform
```rust
/*===================scaling==========================*/

let Sx = 1.0; let Sy = 1.5;let Sz = 1.0;
let xformMatrix = [
Sx,   0.0,  0.0,  0.0,
0.0,  Sy,   0.0,  0.0,
0.0,  0.0,  Sz,   0.0,
0.0,  0.0,  0.0,  1.0  
];

let u_xformMatrix = gl.get_uniform_location(&shaderProgram, "u_xformMatrix");
gl.uniform_matrix4fv_with_f32_array(u_xformMatrix.as_ref(), false, &xformMatrix);
```

and after `npm start` our colorized quad becomes not less colorized rectangle.

Full example in [examples/scaling](../examples/scaling) folder.