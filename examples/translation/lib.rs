extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
use js_sys::{Float32Array, Uint16Array, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

#[allow(dead_code)]
mod utils;
use utils::{compile_shader, link_program, set_panic_hook};

#[allow(non_snake_case)]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    /*============ Creating a canvas =================*/
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    /*==========Defining and storing the geometry=======*/

    let vertices: [f32; 12] = [
        -0.5, 0.5, 0.0, //
        -0.5, -0.5, 0.0, //
        0.5, -0.5, 0.0, //
        0.5, 0.5, 0.0, //
    ];
    let vertices_array = float_32_array!(vertices);

    let indices: [u16; 6] = [3, 2, 1, 3, 1, 0];
    let indices_array = uint_16_array!(indices);

    let colors: [f32; 12] = [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
    let colors_array = float_32_array!(colors);

    // Create an empty buffer object to store the vertex buffer
    let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer")?;

    //Bind appropriate array buffer to it
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    // Pass the vertex data to the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    // Unbind the buffer
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

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

    // Unbind the buffer
    gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);

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
    /*=========================Shaders========================*/

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
    // Create a vertex shader object
    let vertShader = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vertCode)?;

    // fragment shader source code
    let fragCode = r#"precision mediump float;
varying vec3 vColor;
void main(void) {
    gl_FragColor = vec4(vColor, 1.);
}"#;
    // Create fragment shader object
    let fragShader = compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, fragCode)?;
    // Link both programs
    let shaderProgram = link_program(&gl, &vertShader, &fragShader)?;
    // Use the combined shader program object
    gl.use_program(Some(&shaderProgram));

    /*======== Associating shaders to buffer objects ========*/

    // Bind vertex buffer object
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    // Bind appropriate array buffer to it
    gl.bind_buffer(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&Index_Buffer),
    );

    // Get the attribute location
    let coord = gl.get_attrib_location(&shaderProgram, "coordinates") as u32;

    // Point an attribute to the currently bound VBO
    gl.vertex_attrib_pointer_with_i32(coord, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

    // Enable the attribute
    gl.enable_vertex_attrib_array(coord);

    // bind the color buffer
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&colors_buffer));

    // get the attribute location
    let color = gl.get_attrib_location(&shaderProgram, "color") as u32;

    // point attribute to the volor buffer object
    gl.vertex_attrib_pointer_with_i32(color, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

    // enable the color attribute
    gl.enable_vertex_attrib_array(color);

    /* ==========translation======================================*/
    let Tx = 0.5;
    let Ty = 0.5;
    let Tz = 0.0;
    let translation = gl.get_uniform_location(&shaderProgram, "translation");
    gl.uniform4f(translation.as_ref(), Tx, Ty, Tz, 0.0);

    /*============= Drawing the primitive ===============*/

    // Clear the canvas
    gl.clear_color(0.5, 0.5, 0.5, 0.9);

    // Enable the depth test
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    // Clear the color buffer bit
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // Set the view port
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    // Draw the triangle
    gl.draw_elements_with_i32(
        WebGlRenderingContext::TRIANGLES,
        indices.len() as i32,
        WebGlRenderingContext::UNSIGNED_SHORT,
        0,
    );
    Ok(())
}
