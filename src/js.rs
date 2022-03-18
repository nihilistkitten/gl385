//! Safe wrappers over JavaScript and WASM functions.

use crate::{Error, Result};

use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

/// Create and bind a buffer.
///
/// # Errors
/// Errors if buffer creation failed.
///
pub fn bind_buffer(context: &WebGl2RenderingContext) -> Result<()> {
    let buffer = context.create_buffer().ok_or(Error::BufferCreation)?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    Ok(())
}

/// Fill the bound webgl buffer with data.
pub fn fill_buffer(context: &WebGl2RenderingContext, data: &[f32]) {
    // safety: our view into the array is removed at the end of the scope, and we perform no
    // allocations within the scope, so the data doesn't get moved while we still hold a view to it
    unsafe {
        let array_buf_view = js_sys::Float32Array::view(data);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

/// Resize the webgl canvas to fill the screen in a square. Returns the new size.
pub fn resize_canvas() -> u32 {
    let window = web_sys::window().expect("the current html has a window");

    let canvas = window
        .document()
        .expect("the current window has a document")
        .get_element_by_id("glcanvas")
        .expect("the current document has an element with id glcanvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;

    let size = width.min(height);

    canvas.set_width(size);
    canvas.set_height(size);

    size
}
