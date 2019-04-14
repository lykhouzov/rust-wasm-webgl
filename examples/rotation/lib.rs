extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
use js_sys::WebAssembly;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlUniformLocation;

#[allow(dead_code)]
mod utils;
use utils::{compile_shader, link_program, request_animation_frame, set_panic_hook};

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

    let vertices: [f32; 9] = [-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0];
    let vertices_array = float_32_array!(vertices);

    let indices: [u16; 3] = [0, 1, 2];
    let indices_array = uint_16_array!(indices);

    let colors: [f32; 9] = [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
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

    /*=========================Shaders========================*/

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

    // Get the attribute location
    let position = gl.get_attrib_location(&shaderProgram, "position") as u32;

    // Point an attribute to the currently bound VBO
    gl.vertex_attrib_pointer_with_i32(position, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

    // Enable the attribute
    gl.enable_vertex_attrib_array(position);

    // bind the color buffer
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&colors_buffer));

    // get the attribute location
    let color = gl.get_attrib_location(&shaderProgram, "color") as u32;

    // point attribute to the volor buffer object
    gl.vertex_attrib_pointer_with_i32(color, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

    // enable the color attribute
    gl.enable_vertex_attrib_array(color);

    /*========================= MATRIX ========================= */

    let proj_matrix: [f32; 16] = get_projection(
        40.0,
        (canvas.width() as f32 / canvas.height() as f32) as f32,
        1.0,
        100.0,
    );
    let mov_matrix: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
    ];
    let view_matrix: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, -6.0, 1.0, //translating z
    ];
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
    let u_locations = (Pmatrix, Vmatrix, Mmatrix);
    /*============= Drawing the primitive ===============*/
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let matrices = (proj_matrix, view_matrix, mov_matrix);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |dt| {
        animate(&gl.clone(), matrices, u_locations.clone(), dt).unwrap();
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut(f32)>));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

#[allow(dead_code)]
#[allow(non_snake_case)]
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

/*=======================rotation========================*/
#[allow(non_snake_case)]
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
