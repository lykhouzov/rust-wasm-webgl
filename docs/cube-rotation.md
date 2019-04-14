## Cube rotation

This example I've decided to convert tutorial from [Mozilla dev docs](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Creating_3D_objects_using_WebGL).
Difference with [Draw a Rotating 3D Cube](https://www.tutorialspoint.com/webgl/webgl_cube_rotation.htm) is that Mozilla's example uses `mat4` library in order to do rotation.
Luckily, we have [mat4](https://crates.io/crates/mat4) crate witch does the same work in Rust.

### Steps to apply cube rotation

In general, concept is the same as previous [Triangle Rotation](rotation), so we need few matrices to rotate a shape.

So, we have source for vertex shader
```rust
let vsSource = r#"
    attribute vec4 aVertexPosition;
    attribute vec4 aVertexColor;

    uniform mat4 uModelViewMatrix;
    uniform mat4 uProjectionMatrix;

    varying lowp vec4 vColor;

    void main(void) {
      gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
      vColor = aVertexColor;
    }
  "#;
```
Where vertex position attribute is multipied by projection and view matrices.

I've create `ProgramInfo` struct to emulate Javascript literal object created to store program inforation.
```rust
struct ProgramInfo(
    WebGlProgram,
    (u32, u32),
    (
        Result<WebGlUniformLocation, String>,
        Result<WebGlUniformLocation, String>,
    ),
);
```

It is not absolutely the same structure as in JS, but do the same job.

Then as always:
- create shader program
```rust
let shaderProgram = initShaderProgram(&gl, vsSource, fsSource)?;
```
- initialize buffers:
```rust
let buffers:Buffers = initBuffers(&gl)?;
```
- and call `requestAnimationFrame`
```rust
let f = Rc::new(RefCell::new(None));
let g = f.clone();
*g.borrow_mut() = Some(Closure::wrap(Box::new(move |d| {
    drawScene(
        &gl.clone(),
        programmInfo.clone(),
        buffers.clone(),
        d * 0.001f32,
    )
    .unwrap();
    request_animation_frame(f.borrow().as_ref().unwrap());
}) as Box<FnMut(f32)>));

```

Full example in [examples/cube-rotation](../examples/cube-rotation) folder.
