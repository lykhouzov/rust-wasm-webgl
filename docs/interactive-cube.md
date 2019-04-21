## Interactive Cube

This tutorial is based on previouse [cube rotation](cube-rotation) example

### Steps to Apply
In previous example we done almost all work for this tutorial.
It remains only add listeners for mouse events.

So, let's create them.
First we need to convert our canvas element into `EventTarget` object
```rust
let event_target: EventTarget = canvas.into();
```
And then attach to this object event listeneres.

On `mousedown` event we need to set our `drag` flag to `true`
```rust
let drag = drag.clone();
let mousedown_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
    *drag.borrow_mut() = true;
}) as Box<dyn FnMut(MouseEvent)>);
event_target
    .add_event_listener_with_callback("mousedown", mousedown_cb.as_ref().unchecked_ref())
    .unwrap();
mousedown_cb.forget();
```
Then set to `false` this flag on `mouseup` and `mouseout` events
```rust
let drag = drag.clone();
let mouseup_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
    *drag.borrow_mut() = false;
}) as Box<dyn FnMut(MouseEvent)>);
event_target
    .add_event_listener_with_callback("mouseup", mouseup_cb.as_ref().unchecked_ref())
    .unwrap();
event_target
    .add_event_listener_with_callback("mouseout", mouseup_cb.as_ref().unchecked_ref())
    .unwrap();
mouseup_cb.forget();
```

On `mousemove` event we calculate what exact angles the cube was rotated
```rust
let mousemove_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
    if *drag.borrow() {
        let cw = *canvas_width.borrow();
        let ch = *canvas_height.borrow();
        *dX.borrow_mut() = (event.movement_x() as f32) * 2.0 * PI / cw;
        *dY.borrow_mut() = (event.movement_y() as f32) * 2.0 * PI / ch;
        *theta.borrow_mut() += *dX.borrow();
        *phi.borrow_mut() += *dY.borrow();
    }
}) as Box<dyn FnMut(web_sys::MouseEvent)>);
event_target
    .add_event_listener_with_callback("mousemove", mousemove_cb.as_ref().unchecked_ref())
    .unwrap();
mousemove_cb.forget();
```

We change our `drawScene` function to recevie `theta` and `phi` angles instead of `time` and pass thse angles to it
```rust
drawScene(
    &gl.clone(),
    programmInfo.clone(),
    buffers.clone(),
    *theta.borrow(),
    *phi.borrow(),
)
.unwrap();
```

Inside `drawScene` we use the to rotate the cube
```rust
let mat_to_rotate = modelViewMatrix.clone();
mat4::rotate_x(
    &mut modelViewMatrix, // destination matrix
    &mat_to_rotate,       // matrix to rotate
    &phi,
);
let mat_to_rotate = modelViewMatrix.clone();
mat4::rotate_y(
    &mut modelViewMatrix, // destination matrix
    &mat_to_rotate,       // matrix to rotate
    &theta,
);
```

This is it.
Full example in [examples/interactive-cube](../examples/interactive-cube) folder.

### PS
My chalege was how to use "global" variables in different closures.
and an example of animation request closue helps.
So, all required variables I've put into RefCell
```rust
let drag = Rc::new(RefCell::new(false));
let theta = Rc::new(RefCell::new(0.0));
let phi = Rc::new(RefCell::new(0.0));
let dX = Rc::new(RefCell::new(0.0));
let dY = Rc::new(RefCell::new(0.0));
let canvas_width = Rc::new(RefCell::new(canvas.width() as f32));
let canvas_height = Rc::new(RefCell::new(canvas.height() as f32));
```
and then before use just clone a reference
```rust
// MOUSEMOVE
{
    let theta = theta.clone();
    let phi = phi.clone();
    let canvas_width = canvas_width.clone();
    let canvas_height = canvas_height.clone();
    let dX = dX.clone();
    let dY = dY.clone();
    let drag = drag.clone();
    let mousemove_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        if *drag.borrow() {
            let cw = *canvas_width.borrow();
            let ch = *canvas_height.borrow();
            *dX.borrow_mut() = (event.movement_x() as f32) * 2.0 * PI / cw;
            *dY.borrow_mut() = (event.movement_y() as f32) * 2.0 * PI / ch;
            *theta.borrow_mut() += *dX.borrow();
            *phi.borrow_mut() += *dY.borrow();
        }
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);
    event_target
        .add_event_listener_with_callback("mousemove", mousemove_cb.as_ref().unchecked_ref())
        .unwrap();
    mousemove_cb.forget();
}
```