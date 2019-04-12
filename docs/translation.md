## Translation

This example is based on previous [Colors](colors) example.

### Steps to apply translation

We need to update vertex shader and add translation attribute to it.
```rust
// vertex shader source code
    let vertCode = r#"attribute vec3 coordinates;
attribute vec3 color;
varying vec3 vColor;
uniform vec4 translation;
void main(void) {
   gl_Position = vec4(coordinates, 1.0) + translation;
   vColor = color;
}
"#;
```

Then make use of new `translation` attribute
```rust
/* ==========translation======================================*/
let Tx = 0.5;
let Ty = 0.5;
let Tz = 0.0;
let translation = gl.get_uniform_location(&shaderProgram, "translation");
gl.uniform4f(translation.as_ref(), Tx, Ty, Tz, 0.0);
```

This is it. No we see our color quad in top-right corner of the screen.

Full example in [examples/translation](../examples/translation) folder.