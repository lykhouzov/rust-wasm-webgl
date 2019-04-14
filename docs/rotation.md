## Rotation

This example is based on previous [Colors](colors) example.

### Steps to apply rotation
In order to rotate our triangle we need to represent few matrices.
Let's do it in shader programm.
```rust
   // vertex shader source code
    let vertCode = r#"attribute vec3 position;
uniform mat4 Pmatrix;
uniform mat4 Vmatrix;
uniform mat4 Mmatrix;
attribute vec3 color;
varying vec3 vColor;
void main(void) {
   gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);
   vColor = color;
}
"#;
```
And get uniform location
```rust
let Pmatrix: WebGlUniformLocation = gl
        .get_uniform_location(&shaderProgram, "Pmatrix")
        .ok_or_else(|| String::from("cannot get Pmatrix"))
        .unwrap();
    let Vmatrix = gl
        .get_uniform_location(&shaderProgram, "Vmatrix")
        .ok_or_else(|| String::from("cannot get Vmatrix"))
        .unwrap();
    let Mmatrix = gl
        .get_uniform_location(&shaderProgram, "Mmatrix")
        .ok_or_else(|| String::from("cannot get Mmatrix"))
        .unwrap();
```

After that we do magic with `request_animation_frame` function
```rust
let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let matrices = (proj_matrix, view_matrix, mov_matrix);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |dt| {
        animate(&gl.clone(), matrices, u_locations.clone(), dt).unwrap();
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut(f32)>));
    request_animation_frame(g.borrow().as_ref().unwrap());
```
Which is still not clear for me how it works.

And last one is to create `animate` function
```rust
fn animate(
    gl: &WebGlRenderingContext,
    matrices: ([f32; 16], [f32; 16], [f32; 16]),
    locations: (
        WebGlUniformLocation,
        WebGlUniformLocation,
        WebGlUniformLocation,
    ),
    dt: f32,
) -> Result<(), JsValue> {
    let (proj_matrix, view_matrix, mov_matrix) = matrices;
    let (Pmatrix, Vmatrix, Mmatrix) = locations;
    let mov_matrix = rotateZ(mov_matrix, dt * 0.001f32);

    // Clear the canvas
    gl.clear_color(0.5, 0.5, 0.5, 0.9);

    // Enable the depth test
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    gl.depth_func(WebGlRenderingContext::LEQUAL);

    // Clear the color buffer bit
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // Set the view port
    // gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    gl.uniform_matrix4fv_with_f32_array(Some(&Pmatrix), false, &proj_matrix);
    gl.uniform_matrix4fv_with_f32_array(Some(&Vmatrix), false, &view_matrix);
    gl.uniform_matrix4fv_with_f32_array(Some(&Mmatrix), false, &mov_matrix);
    // Draw the triangle
    gl.draw_elements_with_i32(
        WebGlRenderingContext::TRIANGLES,
        3 as i32,
        WebGlRenderingContext::UNSIGNED_SHORT,
        0,
    );
    Ok(())
}
```

Where we use converted functions from the tutorial.
```rust
#[allow(non_snake_case)]
fn get_projection(angle: f32, a: f32, zMin: f32, zMax: f32) -> [f32; 16] {
    use std::f32::consts::PI;
    let ang = ((angle * 0.5) * PI / 180.0).tan(); //angle*0.5
    [
        0.5 / ang,
        0.0,
        0.0,
        0.0,
        0.0,
        0.5 * a / ang,
        0.0,
        0.0,
        0.0,
        0.0,
        -(zMax + zMin) / (zMax - zMin),
        -1.0,
        0.0,
        0.0,
        (-2.0 * zMax * zMin) / (zMax - zMin),
        0.0,
    ]
}

fn rotateZ(mut m: [f32; 16], angle: f32) -> [f32; 16] {
    let c: f32 = angle.cos();
    let s: f32 = angle.sin();
    let mv0: f32 = m[0];
    let mv4: f32 = m[4];
    let mv8: f32 = m[8];

    m[0] = c * m[0] - s * m[1];
    m[4] = c * m[4] - s * m[5];
    m[8] = c * m[8] - s * m[9];
    m[1] = c * m[1] + s * mv0;
    m[5] = c * m[5] + s * mv4;
    m[9] = c * m[9] + s * mv8;

    m
}
```

Full example in [examples/rotation](../examples/rotation) folder.

### PS
I still get an error in browser console, even the code is working perfect and i see rotation
```
WebGL: INVALID_OPERATION: uniformMatrix4fv: location is not from current program
```
And i could get how to fix it.

Also, for the first attempts I was getting black screen, because all my matrices were without expicit type anotation.
