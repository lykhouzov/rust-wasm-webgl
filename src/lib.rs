extern crate js_sys;
extern crate mat4;
extern crate wasm_bindgen;
extern crate web_sys;
use js_sys::WebAssembly;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

#[allow(dead_code)]
mod utils;
use utils::{compile_shader, link_program, request_animation_frame, set_panic_hook};
#[derive(Debug, Clone)]
struct ProgramInfo(
    WebGlProgram,
    (u32, u32),
    (
        Result<WebGlUniformLocation, String>,
        Result<WebGlUniformLocation, String>,
    ),
);
#[derive(Debug,Clone)]
struct Buffers(WebGlBuffer, WebGlBuffer, WebGlBuffer);
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

    // Vertex shader program

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

    // Fragment shader program

    let fsSource = r#"
    varying lowp vec4 vColor;

    void main(void) {
      gl_FragColor = vColor;
    }
  "#;
    // Initialize a shader program; this is where all the lighting
    // for the vertices and so forth is established.
    let shaderProgram = initShaderProgram(&gl, vsSource, fsSource)?;

    // Collect all the info needed to use the shader program.
    // Look up which attributes our shader program is using
    // for aVertexPosition, aVevrtexColor and also
    // look up uniform locations.
    let programmInfo = {
        let vertexPosition = gl.get_attrib_location(&shaderProgram, "aVertexPosition") as u32;
        let vertexColor = gl.get_attrib_location(&shaderProgram, "aVertexColor") as u32;
        let projectionMatrix = gl
            .get_uniform_location(&shaderProgram, "uProjectionMatrix")
            .ok_or_else(|| String::from("cannot get uProjectionMatrix"));
        let modelViewMatrix = gl
            .get_uniform_location(&shaderProgram, "uModelViewMatrix")
            .ok_or_else(|| String::from("cannot get uModelViewMatrix"));
        ProgramInfo(
            shaderProgram,
            (vertexPosition, vertexColor),
            (projectionMatrix, modelViewMatrix),
        )
    };
    // Here's where we call the routine that builds all the
    // objects we'll be drawing.
    let buffers:Buffers = initBuffers(&gl)?;

    // Draw the scene repeatedly
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
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut(f32)>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
#[allow(non_snake_case)]
fn initShaderProgram(
    gl: &WebGlRenderingContext,
    vsSource: &str,
    fsSource: &str,
) -> Result<WebGlProgram, String> {
    let v_shader = compile_shader(gl, WebGlRenderingContext::VERTEX_SHADER, vsSource);
    let f_shader = compile_shader(gl, WebGlRenderingContext::FRAGMENT_SHADER, fsSource);

    link_program(gl, &v_shader?, &f_shader?)
}
#[allow(non_snake_case)]
fn initBuffers(gl: &WebGlRenderingContext) -> Result<Buffers, JsValue> {
    // Create a buffer for the cube's vertex positions.
    let positionBuffer = gl
        .create_buffer()
        .ok_or("failed to create positionBuffer buffer")?;

    // Select the positionBuffer as the one to apply buffer
    // operations to from here out.
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&positionBuffer));

    // Now create an array of positions for the cube.
    let positions: [f32; 72] = [
        // Front face
        -1.0, -1.0, 1.0, //
        1.0, -1.0, 1.0, //
        1.0, 1.0, 1.0, //
        -1.0, 1.0, 1.0, //
        // Back face
        -1.0, -1.0, -1.0, //
        -1.0, 1.0, -1.0, //
        1.0, 1.0, -1.0, //
        1.0, -1.0, -1.0, //
        // Top face
        -1.0, 1.0, -1.0, //
        -1.0, 1.0, 1.0, //
        1.0, 1.0, 1.0, //
        1.0, 1.0, -1.0, //
        // Bottom face
        -1.0, -1.0, -1.0, //
        1.0, -1.0, -1.0, //
        1.0, -1.0, 1.0, //
        -1.0, -1.0, 1.0, //
        // Right face
        1.0, -1.0, -1.0, //
        1.0, 1.0, -1.0, //
        1.0, 1.0, 1.0, //
        1.0, -1.0, 1.0, //
        // Left face
        -1.0, -1.0, -1.0, //
        -1.0, -1.0, 1.0, //
        -1.0, 1.0, 1.0, //
        -1.0, 1.0, -1.0, //
    ];
    let position_array = float_32_array!(positions);
    // Now pass the list of positions into WebGL to build the
    // shape. We do this by creating a Float32Array from the
    // Rust array, then use it to fill the current buffer.
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &position_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    // Now set up the colors for the faces. We'll use solid colors
    // for each face.

    let faceColors = [
        [1.0, 1.0, 1.0, 1.0], // Front face: white
        [1.0, 0.0, 0.0, 1.0], // Back face: red
        [0.0, 1.0, 0.0, 1.0], // Top face: green
        [0.0, 0.0, 1.0, 1.0], // Bottom face: blue
        [1.0, 1.0, 0.0, 1.0], // Right face: yellow
        [1.0, 0.0, 1.0, 1.0], // Left face: purple
    ];
    let color_array = {
        let color_vec: Vec<f32> = faceColors
            .iter()
            .map(|row| vec![row, row, row, row])
            .flatten()
            .flatten()
            .map(|x| *x)
            .collect();
        let mut color_arr: [f32; 96] = [0f32; 96];
        color_arr.copy_from_slice(color_vec.as_slice());
        float_32_array!(color_arr)
    };
    let colorBuffer = gl
        .create_buffer()
        .ok_or("failed to create colorBuffer buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&colorBuffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &color_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    // Build the element array buffer; this specifies the indices
    // into the vertex arrays for each face's vertices.
    let indexBuffer = gl
        .create_buffer()
        .ok_or("failed to create indexBuffer buffer")?;
    gl.bind_buffer(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&indexBuffer),
    );

    // This array defines each face as two triangles, using the
    // indices into the vertex array to specify each triangle's
    // position.

    let indices: [u16; 36] = [
        0, 1, 2, 0, 2, 3, // front
        4, 5, 6, 4, 6, 7, // back
        8, 9, 10, 8, 10, 11, // top
        12, 13, 14, 12, 14, 15, // bottom
        16, 17, 18, 16, 18, 19, // right
        20, 21, 22, 20, 22, 23, // left
    ];
    let index_array = uint_16_array!(indices);
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        &index_array,
        WebGlRenderingContext::STATIC_DRAW,
    );
    Ok(Buffers(positionBuffer, colorBuffer, indexBuffer))
}
#[allow(non_snake_case)]
fn drawScene(
    gl: &WebGlRenderingContext,
    programInfo: ProgramInfo,
    buffers: Buffers,
    deltaTime: f32,
) -> Result<(), JsValue> {
    use std::f32::consts::PI;
    let Buffers(positionBuffer, colorBuffer, indexBuffer) = buffers;
    let ProgramInfo(
        shaderProgram,
        (vertexPosition, vertexColor),
        (location_projectionMatrix, location_modelViewMatrix),
    ) = programInfo;
    gl.clear_color(0.0, 0.0, 0.0, 1.0); // Clear to black, fully opaque
    gl.clear_depth(1.0); // Clear everything
    gl.enable(WebGlRenderingContext::DEPTH_TEST); // Enable depth testing
    gl.depth_func(WebGlRenderingContext::LEQUAL); // Near things obscure far things

    // Clear the canvas before we start drawing on it.

    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    // Create a perspective matrix, a special matrix that is
    // used to simulate the distortion of perspective in a camera.
    // Our field of view is 45 degrees, with a width/height
    // ratio that matches the display size of the canvas
    // and we only want to see objects between 0.1 units
    // and 100 units away from the camera.

    let fieldOfView = 45.0 * PI / 180.0; // in radians
    let canvas: web_sys::HtmlCanvasElement = gl
        .canvas()
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let aspect: f32 = canvas.width() as f32 / canvas.height() as f32;
    let zNear = 0.1;
    let zFar = 100.0;
    let mut projectionMatrix = mat4::new_identity();

    mat4::perspective(&mut projectionMatrix, &fieldOfView, &aspect, &zNear, &zFar);

    // Set the drawing position to the "identity" point, which is
    // the center of the scene.
    let mut modelViewMatrix = mat4::new_identity();

    // Now move the drawing position a bit to where we want to
    // start drawing the square.
    let cubeRotation = deltaTime;
    let mat_to_translate = modelViewMatrix.clone();
    mat4::translate(
        &mut modelViewMatrix, // destination matrix
        &mat_to_translate,    // matrix to translate
        &[-0.0, 0.0, -6.0],
    ); // amount to translate

    let mat_to_rotate = modelViewMatrix.clone();
    mat4::rotate(
        &mut modelViewMatrix,  // destination matrix
        &mat_to_rotate,        // matrix to rotate
        &(0.0 * cubeRotation), // amount to rotate in radians
        &(0.0 * cubeRotation),
        &(1.0 * cubeRotation),
    ); // axis to rotate around (Z)
    let mat_to_rotate = modelViewMatrix.clone();
    mat4::rotate(
        &mut modelViewMatrix,        // destination matrix
        &mat_to_rotate,              // matrix to rotate
        &(0.0 * cubeRotation * 0.7), // amount to rotate in radians
        &(1.0 * cubeRotation * 0.7),
        &(0.0 * cubeRotation * 0.7),
    ); // axis to rotate around (X)

    // Tell WebGL how to pull out the positions from the position
    // buffer into the vertexPosition attribute
    {
        let numComponents = 3;
        let type_ = WebGlRenderingContext::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&positionBuffer));

        gl.vertex_attrib_pointer_with_i32(
            vertexPosition,
            numComponents,
            type_,
            normalize,
            stride,
            offset,
        );
        gl.enable_vertex_attrib_array(vertexPosition);
        // gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    }
    // Tell WebGL how to pull out the colors from the color buffer
    // into the vertexColor attribute.
    {
        let numComponents = 4;
        let type_ = WebGlRenderingContext::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&colorBuffer));
        gl.vertex_attrib_pointer_with_i32(
            vertexColor,
            numComponents,
            type_,
            normalize,
            stride,
            offset,
        );
        gl.enable_vertex_attrib_array(vertexColor);

        // gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    }

    // Tell WebGL which indices to use to index the vertices
    gl.bind_buffer(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&indexBuffer),
    );

    // Tell WebGL to use our program when drawing

    gl.use_program(Some(&shaderProgram));

    // Set the shader uniforms

    gl.uniform_matrix4fv_with_f32_array(
        Some(&location_projectionMatrix?),
        false,
        &projectionMatrix,
    );
    gl.uniform_matrix4fv_with_f32_array(Some(&location_modelViewMatrix?), false, &modelViewMatrix);
    {
        let vertexCount = 36;
        let type_ = WebGlRenderingContext::UNSIGNED_SHORT;
        let offset = 0;
        gl.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, vertexCount, type_, offset);
    }

    Ok(())
}
