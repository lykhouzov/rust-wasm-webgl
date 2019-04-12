## Drawing points.

No need to repeat the text from [tutorial](https://www.tutorialspoint.com/webgl/webgl_drawing_points.htm).
I will only describe what steps i've been doing and issues i have faced with.

### Creating a canvas
In this section everything looks pretty similar to official example and very simillar to JS example.
```rust
let document = web_sys::window().unwrap().document().unwrap();
let canvas = document.get_element_by_id("canvas").unwrap();
let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
```
The only difference that Rust uses snake_case naming style and functions in most cases return `Option` or `Result`,
so that we have to handle this output.

### Defining and storing the geometry

Because the tutorial is pretty old, it still use `var` keywork in order to define local variable.
Renaming `var` into `let` can be done in both sources and this section would looks equal.

### Shaders
Luckili, in official example of wasm-bindgen, there is an example of 2 functions: `compile_shader` and `link_program`.
I've moved them into `utils` mod.
This part for me looks different
```rust
...
// Create a vertex shader object
let vertShader = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vertCode)?;
...
// Create fragment shader object
let fragShader = compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, fragCode)?;
// Link both programs
let shaderProgram = link_program(&gl, &vertShader, &fragShader)?;
// Use the combined shader program object
gl.use_program(Some(&shaderProgram));
```

### Associating shaders to buffer objects
This section requires just rename of function from CamelCase style into snake_case and little tunning of parameters,
for example, second parameter of `bind_buffer` is `Option<&WebGlBuffer>` and not buffer itself

### Drawing the primitive
This section is the same as previous, plus casting of `canvas.width() as i32`

### Issue I faced with
So, convertion was done. And rust compiles sources, no issues. I run `npm start`.... but no points on a screen.
I spent few hours to understand, that issue goes from here
```rust
let vertices = [
    ...
```
*The type of `vertices` has to be specified explicitly.*
So, I've change it to
```rust
let vertices: [f32; 9] = [
    -0.5, 0.5, 0.0, //
    0.0, 0.5, 0.0, //
    -0.25, 0.25, 0.0, //
];
```

Now everything works as expected and I see three dots on a screen. Hura!

Workable source is palced in [examples/drawing-points/](../examples/drawing-points/) folder