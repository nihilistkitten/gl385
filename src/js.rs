//! Safe wrappers over JavaScript and WASM functions.

use crate::{Error, Result};

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
